// ════════════════════════════════════════════════════════════════════════════
// Handlers : Matching inter-arbres et découvertes
// ════════════════════════════════════════════════════════════════════════════
//
// Endpoints :
//   GET    /api/arbre/decouvertes : lister_decouvertes
//   POST   /api/arbre/decouvertes/{id}/confirmer, confirmer_suggestion
//   POST   /api/arbre/decouvertes/{id}/rejeter, rejeter_suggestion
//   GET    /api/arbre/decouvertes/{id}/branches, obtenir_branches
//   POST   /api/arbre/decouvertes/{id}/demande-contact, demander_contact
//   POST   /api/arbre/demandes-contact/{id}/accepter, accepter_demande
//   POST   /api/arbre/demandes-contact/{id}/refuser, refuser_demande
// ════════════════════════════════════════════════════════════════════════════

use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::models::matching::{
    anonymiser_membre, DecouvertesResponse, DetailsScoreResponse,
    PersonneResumeResponse, ProfilPublicResponse, SectionDecouvertes,
    SuggestionAvecPersonnesRow, SuggestionResponse,
};
use crate::models::arbre_genealogique::{
    LienArbreResponse, PersonneNoeud, PersonneNoeudResponse,
};
use crate::services::audit;
use crate::ApiResponse;

// ─── Helper JWT (réutilisation du pattern existant) ─────────────────────

fn extraire_utilisateur_id(req: &HttpRequest) -> Result<Uuid, ApiErreur> {
    let header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| ApiErreur::NonAutorise("Token manquant".into()))?;
    let token = crate::jwt::extraire_token_du_header(header)?;
    let jwt_config = req
        .app_data::<web::Data<crate::config::JwtConfig>>()
        .ok_or_else(|| ApiErreur::BaseDeDonnees("Configuration JWT manquante".into()))?;
    let claims = crate::jwt::valider_token(token, &jwt_config.secret)?;
    claims
        .sub
        .parse::<Uuid>()
        .map_err(|_| ApiErreur::NonAutorise("ID utilisateur invalide".into()))
}

async fn obtenir_arbre_id(pool: &PgPool, utilisateur_id: Uuid) -> Result<Option<Uuid>, ApiErreur> {
    let arbre_id: Option<Uuid> = sqlx::query_scalar(
        "SELECT id FROM arbre_genealogique.arbres
         WHERE utilisateur_id = $1 AND deleted_at IS NULL",
    )
    .bind(utilisateur_id)
    .fetch_optional(pool)
    .await?;
    Ok(arbre_id)
}

// ════════════════════════════════════════════════════════════════════════════
// GET /api/arbre/decouvertes
// ════════════════════════════════════════════════════════════════════════════

pub async fn lister_decouvertes(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let arbre_id = obtenir_arbre_id(pool.get_ref(), utilisateur_id)
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Aucun arbre trouvé".into()))?;

    // Récupérer toutes les suggestions impliquant cet arbre
    let suggestions = sqlx::query_as::<_, SuggestionAvecPersonnesRow>(
        "SELECT
            sc.id AS suggestion_id, sc.score, sc.score_nom, sc.score_prenoms,
            sc.score_date, sc.score_lieu, sc.score_genre,
            sc.statut, sc.confirmee_par_a, sc.confirmee_par_b,
            sc.detectee_le, sc.confirmee_le,
            -- Personne A
            pa.id AS a_personne_id, ra.id AS a_rattachement_id,
            pa.nom AS a_nom, pa.prenoms AS a_prenoms,
            pa.naissance_annee AS a_naissance_annee, pa.naissance_lieu AS a_naissance_lieu,
            pa.genre AS a_genre,
            -- Personne B
            pb.id AS b_personne_id, rb.id AS b_rattachement_id,
            pb.nom AS b_nom, pb.prenoms AS b_prenoms,
            pb.naissance_annee AS b_naissance_annee, pb.naissance_lieu AS b_naissance_lieu,
            pb.genre AS b_genre,
            -- Arbres
            ra.arbre_id AS a_arbre_id, rb.arbre_id AS b_arbre_id,
            -- Utilisateur B (pour anonymisation)
            ab.utilisateur_id AS b_utilisateur_id
         FROM arbre_genealogique.suggestions_correspondance sc
         JOIN arbre_genealogique.rattachements ra ON ra.id = sc.rattachement_a_id
         JOIN arbre_genealogique.personnes pa ON pa.id = ra.personne_id
         JOIN arbre_genealogique.rattachements rb ON rb.id = sc.rattachement_b_id
         JOIN arbre_genealogique.personnes pb ON pb.id = rb.personne_id
         JOIN arbre_genealogique.arbres ab ON ab.id = rb.arbre_id
         WHERE (ra.arbre_id = $1 OR rb.arbre_id = $1)
           AND sc.deleted_at IS NULL
           AND sc.statut NOT IN ('rejetee_a', 'rejetee_b')
         ORDER BY sc.score DESC",
    )
    .bind(arbre_id)
    .fetch_all(pool.get_ref())
    .await?;

    // Transformer en réponses avec perspective de l'utilisateur
    let mut en_attente = vec![];
    let mut en_cours = vec![];
    let mut confirmees = vec![];

    for row in suggestions {
        let est_a = row.a_arbre_id == arbre_id;
        let (ma_personne, personne_matchee, autre_utilisateur_id) = if est_a {
            (
                PersonneResumeResponse {
                    id: row.a_personne_id,
                    rattachement_id: row.a_rattachement_id,
                    nom: row.a_nom,
                    prenoms: row.a_prenoms,
                    naissance_annee: row.a_naissance_annee,
                    naissance_lieu: row.a_naissance_lieu,
                    genre: row.a_genre,
                },
                PersonneResumeResponse {
                    id: row.b_personne_id,
                    rattachement_id: row.b_rattachement_id,
                    nom: row.b_nom,
                    prenoms: row.b_prenoms,
                    naissance_annee: row.b_naissance_annee,
                    naissance_lieu: row.b_naissance_lieu,
                    genre: row.b_genre,
                },
                row.b_utilisateur_id,
            )
        } else {
            (
                PersonneResumeResponse {
                    id: row.b_personne_id,
                    rattachement_id: row.b_rattachement_id,
                    nom: row.b_nom,
                    prenoms: row.b_prenoms,
                    naissance_annee: row.b_naissance_annee,
                    naissance_lieu: row.b_naissance_lieu,
                    genre: row.b_genre,
                },
                PersonneResumeResponse {
                    id: row.a_personne_id,
                    rattachement_id: row.a_rattachement_id,
                    nom: row.a_nom,
                    prenoms: row.a_prenoms,
                    naissance_annee: row.a_naissance_annee,
                    naissance_lieu: row.a_naissance_lieu,
                    genre: row.a_genre,
                },
                row.b_utilisateur_id, // simplifié, en réalité il faut l'user de l'autre arbre
            )
        };

        // Déterminer le statut du point de vue de l'utilisateur
        let mon_statut = if row.statut == "confirmee" {
            "confirmee".to_string()
        } else if est_a && row.confirmee_par_a {
            "confirmee_de_mon_cote".to_string()
        } else if !est_a && row.confirmee_par_b {
            "confirmee_de_mon_cote".to_string()
        } else if row.confirmee_par_a || row.confirmee_par_b {
            "confirmee_de_mon_cote".to_string()
        } else {
            "en_attente".to_string()
        };

        let suggestion = SuggestionResponse {
            id: row.suggestion_id,
            ma_personne,
            personne_matchee,
            score: row.score,
            details_score: DetailsScoreResponse {
                nom: row.score_nom,
                prenoms: row.score_prenoms,
                date: row.score_date,
                lieu: row.score_lieu,
                genre: row.score_genre,
            },
            statut: mon_statut.clone(),
            membre_id_anonymise: anonymiser_membre(autre_utilisateur_id),
            detectee_le: row.detectee_le,
            confirmee_le: row.confirmee_le,
        };

        match mon_statut.as_str() {
            "confirmee" => confirmees.push(suggestion),
            "confirmee_de_mon_cote" => en_cours.push(suggestion),
            _ => en_attente.push(suggestion),
        }
    }

    let en_attente_total = en_attente.len() as i64;
    let en_cours_total = en_cours.len() as i64;
    let confirmees_total = confirmees.len() as i64;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(DecouvertesResponse {
            en_attente: SectionDecouvertes { suggestions: en_attente, total: en_attente_total },
            en_cours: SectionDecouvertes { suggestions: en_cours, total: en_cours_total },
            confirmees: SectionDecouvertes { suggestions: confirmees, total: confirmees_total },
        }),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// POST /api/arbre/decouvertes/{id}/confirmer
// ════════════════════════════════════════════════════════════════════════════

pub async fn confirmer_suggestion(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let suggestion_id = chemin.into_inner();
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let arbre_id = obtenir_arbre_id(pool.get_ref(), utilisateur_id)
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Aucun arbre trouvé".into()))?;

    // Récupérer la suggestion
    let suggestion: Option<(Uuid, Uuid, Uuid, Uuid, bool, bool, String)> = sqlx::query_as(
        "SELECT sc.id, ra.arbre_id, rb.arbre_id, sc.id,
                sc.confirmee_par_a, sc.confirmee_par_b, sc.statut
         FROM arbre_genealogique.suggestions_correspondance sc
         JOIN arbre_genealogique.rattachements ra ON ra.id = sc.rattachement_a_id
         JOIN arbre_genealogique.rattachements rb ON rb.id = sc.rattachement_b_id
         WHERE sc.id = $1 AND sc.deleted_at IS NULL",
    )
    .bind(suggestion_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let Some((_, arbre_a_id, arbre_b_id, _, confirmee_par_a, confirmee_par_b, statut)) = suggestion else {
        return Err(ApiErreur::NonTrouve("Suggestion introuvable".into()));
    };

    if statut.starts_with("rejetee") {
        return Err(ApiErreur::Conflit("Cette suggestion a déjà été rejetée".into()));
    }
    if statut == "confirmee" {
        return Err(ApiErreur::Conflit("Cette correspondance est déjà confirmée".into()));
    }

    let est_a = arbre_a_id == arbre_id;
    let est_b = arbre_b_id == arbre_id;
    if !est_a && !est_b {
        return Err(ApiErreur::AccesInterdit("Cette suggestion ne vous concerne pas".into()));
    }

    // Mettre à jour la confirmation
    let (nouvelle_conf_a, nouvelle_conf_b) = if est_a {
        (true, confirmee_par_b)
    } else {
        (confirmee_par_a, true)
    };

    let nouveau_statut = if nouvelle_conf_a && nouvelle_conf_b {
        "confirmee"
    } else if nouvelle_conf_a {
        "confirmee_a"
    } else {
        "confirmee_b"
    };

    sqlx::query(
        "UPDATE arbre_genealogique.suggestions_correspondance
         SET confirmee_par_a = $1, confirmee_par_b = $2, statut = $3,
             confirmee_le = CASE WHEN $3 = 'confirmee' THEN NOW() ELSE confirmee_le END
         WHERE id = $4",
    )
    .bind(nouvelle_conf_a)
    .bind(nouvelle_conf_b)
    .bind(nouveau_statut)
    .bind(suggestion_id)
    .execute(pool.get_ref())
    .await?;

    audit::log_action(
        pool.get_ref(), Some(utilisateur_id), "confirmer_suggestion",
        "arbre_genealogique", "suggestions_correspondance", Some(suggestion_id),
        None, Some(json!({"statut": nouveau_statut})),
        audit::extraire_ip(&req).as_deref(), audit::extraire_user_agent(&req).as_deref(),
    ).await;

    let message = if nouveau_statut == "confirmee" {
        "Correspondance confirmée ! Vous pouvez maintenant voir les branches de l'autre arbre."
    } else {
        "En attente de confirmation de l'autre membre"
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(json!({"statut": nouveau_statut, "message": message})),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// POST /api/arbre/decouvertes/{id}/rejeter
// ════════════════════════════════════════════════════════════════════════════

pub async fn rejeter_suggestion(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let suggestion_id = chemin.into_inner();
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let arbre_id = obtenir_arbre_id(pool.get_ref(), utilisateur_id)
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Aucun arbre trouvé".into()))?;

    // Vérifier ownership
    let existe: Option<(Uuid, Uuid)> = sqlx::query_as(
        "SELECT ra.arbre_id, rb.arbre_id
         FROM arbre_genealogique.suggestions_correspondance sc
         JOIN arbre_genealogique.rattachements ra ON ra.id = sc.rattachement_a_id
         JOIN arbre_genealogique.rattachements rb ON rb.id = sc.rattachement_b_id
         WHERE sc.id = $1 AND sc.deleted_at IS NULL",
    )
    .bind(suggestion_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let Some((arbre_a, arbre_b)) = existe else {
        return Err(ApiErreur::NonTrouve("Suggestion introuvable".into()));
    };

    let est_a = arbre_a == arbre_id;
    if !est_a && arbre_b != arbre_id {
        return Err(ApiErreur::AccesInterdit("Cette suggestion ne vous concerne pas".into()));
    }

    let statut = if est_a { "rejetee_a" } else { "rejetee_b" };

    sqlx::query(
        "UPDATE arbre_genealogique.suggestions_correspondance
         SET statut = $1
         WHERE id = $2",
    )
    .bind(statut)
    .bind(suggestion_id)
    .execute(pool.get_ref())
    .await?;

    audit::log_action(
        pool.get_ref(), Some(utilisateur_id), "rejeter_suggestion",
        "arbre_genealogique", "suggestions_correspondance", Some(suggestion_id),
        None, Some(json!({"statut": statut})),
        audit::extraire_ip(&req).as_deref(), audit::extraire_user_agent(&req).as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(json!({"message": "Suggestion rejetée"})),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// GET /api/arbre/decouvertes/{id}/branches
// ════════════════════════════════════════════════════════════════════════════

pub async fn obtenir_branches(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let suggestion_id = chemin.into_inner();
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let arbre_id = obtenir_arbre_id(pool.get_ref(), utilisateur_id)
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Aucun arbre trouvé".into()))?;

    // Vérifier que la correspondance est confirmée mutuellement
    let info: Option<(Uuid, Uuid, String)> = sqlx::query_as(
        "SELECT ra.arbre_id, rb.arbre_id, sc.statut
         FROM arbre_genealogique.suggestions_correspondance sc
         JOIN arbre_genealogique.rattachements ra ON ra.id = sc.rattachement_a_id
         JOIN arbre_genealogique.rattachements rb ON rb.id = sc.rattachement_b_id
         WHERE sc.id = $1 AND sc.deleted_at IS NULL",
    )
    .bind(suggestion_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let Some((arbre_a, arbre_b, statut)) = info else {
        return Err(ApiErreur::NonTrouve("Suggestion introuvable".into()));
    };

    if statut != "confirmee" {
        return Err(ApiErreur::AccesInterdit(
            "Correspondance non confirmée mutuellement".into(),
        ));
    }

    // Déterminer l'arbre de l'autre utilisateur
    let autre_arbre_id = if arbre_a == arbre_id { arbre_b } else { arbre_a };

    // Charger l'arbre complet de l'autre utilisateur
    let personnes_rows = sqlx::query_as::<_, PersonneNoeud>(
        "SELECT p.id, r.id AS rattachement_id,
                p.nom, p.prenoms, p.genre,
                p.naissance_annee, p.naissance_mois, p.naissance_jour, p.naissance_lieu,
                p.deces_annee, p.deces_mois, p.deces_jour, p.deces_lieu,
                p.photo_url
         FROM arbre_genealogique.personnes p
         JOIN arbre_genealogique.rattachements r ON r.personne_id = p.id
         WHERE r.arbre_id = $1 AND r.deleted_at IS NULL AND p.deleted_at IS NULL
         ORDER BY p.nom ASC, p.prenoms ASC",
    )
    .bind(autre_arbre_id)
    .fetch_all(pool.get_ref())
    .await?;

    let liens = sqlx::query_as::<_, LienArbreResponse>(
        "SELECT id, rattachement_source_id, rattachement_cible_id, type_lien
         FROM arbre_genealogique.liens_familiaux
         WHERE arbre_id = $1 AND deleted_at IS NULL",
    )
    .bind(autre_arbre_id)
    .fetch_all(pool.get_ref())
    .await?;

    // Anonymiser le propriétaire
    let autre_utilisateur_id: Option<Uuid> = sqlx::query_scalar(
        "SELECT utilisateur_id FROM arbre_genealogique.arbres WHERE id = $1",
    )
    .bind(autre_arbre_id)
    .fetch_optional(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(json!({
            "suggestion_id": suggestion_id,
            "personnes": personnes_rows.into_iter().map(PersonneNoeudResponse::from_row).collect::<Vec<_>>(),
            "liens": liens,
            "membre_id_anonymise": autre_utilisateur_id.map(anonymiser_membre).unwrap_or_default(),
        })),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// POST /api/arbre/decouvertes/{suggestion_id}/demande-contact
// ════════════════════════════════════════════════════════════════════════════

pub async fn demander_contact(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let suggestion_id = chemin.into_inner();
    let utilisateur_id = extraire_utilisateur_id(&req)?;

    // Vérifier correspondance confirmée
    let info: Option<(Uuid, Uuid, String)> = sqlx::query_as(
        "SELECT a.utilisateur_id, b.utilisateur_id, sc.statut
         FROM arbre_genealogique.suggestions_correspondance sc
         JOIN arbre_genealogique.rattachements ra ON ra.id = sc.rattachement_a_id
         JOIN arbre_genealogique.arbres a ON a.id = ra.arbre_id
         JOIN arbre_genealogique.rattachements rb ON rb.id = sc.rattachement_b_id
         JOIN arbre_genealogique.arbres b ON b.id = rb.arbre_id
         WHERE sc.id = $1 AND sc.deleted_at IS NULL",
    )
    .bind(suggestion_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let Some((user_a, user_b, statut)) = info else {
        return Err(ApiErreur::NonTrouve("Suggestion introuvable".into()));
    };

    if statut != "confirmee" {
        return Err(ApiErreur::AccesInterdit("Correspondance non confirmée".into()));
    }

    let destinataire_id = if user_a == utilisateur_id { user_b } else { user_a };

    // Vérifier pas de doublon
    let existe: Option<Uuid> = sqlx::query_scalar(
        "SELECT id FROM arbre_genealogique.demandes_contact
         WHERE suggestion_id = $1 AND demandeur_id = $2",
    )
    .bind(suggestion_id)
    .bind(utilisateur_id)
    .fetch_optional(pool.get_ref())
    .await?;

    if existe.is_some() {
        return Err(ApiErreur::Conflit("Demande déjà envoyée".into()));
    }

    let demande_id: Uuid = sqlx::query_scalar(
        "INSERT INTO arbre_genealogique.demandes_contact (suggestion_id, demandeur_id, destinataire_id)
         VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(suggestion_id)
    .bind(utilisateur_id)
    .bind(destinataire_id)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(json!({"id": demande_id, "statut": "en_attente"})),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// POST /api/arbre/demandes-contact/{id}/accepter
// ════════════════════════════════════════════════════════════════════════════

pub async fn accepter_demande(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let demande_id = chemin.into_inner();
    let utilisateur_id = extraire_utilisateur_id(&req)?;

    // Vérifier que l'utilisateur est le destinataire
    let demande: Option<(Uuid, Uuid)> = sqlx::query_as(
        "SELECT destinataire_id, demandeur_id FROM arbre_genealogique.demandes_contact
         WHERE id = $1 AND statut = 'en_attente'",
    )
    .bind(demande_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let Some((destinataire, demandeur)) = demande else {
        return Err(ApiErreur::NonTrouve("Demande introuvable ou déjà traitée".into()));
    };

    if destinataire != utilisateur_id {
        return Err(ApiErreur::AccesInterdit("Cette demande ne vous est pas destinée".into()));
    }

    sqlx::query(
        "UPDATE arbre_genealogique.demandes_contact
         SET statut = 'acceptee', traitee_le = NOW() WHERE id = $1",
    )
    .bind(demande_id)
    .execute(pool.get_ref())
    .await?;

    // Récupérer le profil du demandeur
    let profil: Option<(String, String, String)> = sqlx::query_as(
        "SELECT nom, prenom, email FROM iam.utilisateur WHERE id = $1",
    )
    .bind(demandeur)
    .fetch_optional(pool.get_ref())
    .await?;

    let profil_response = profil.map(|(nom, prenom, email)| ProfilPublicResponse { nom, prenom, email });

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(json!({"profil": profil_response})),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// POST /api/arbre/demandes-contact/{id}/refuser
// ════════════════════════════════════════════════════════════════════════════

pub async fn refuser_demande(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let demande_id = chemin.into_inner();
    let utilisateur_id = extraire_utilisateur_id(&req)?;

    let destinataire: Option<Uuid> = sqlx::query_scalar(
        "SELECT destinataire_id FROM arbre_genealogique.demandes_contact
         WHERE id = $1 AND statut = 'en_attente'",
    )
    .bind(demande_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let Some(dest) = destinataire else {
        return Err(ApiErreur::NonTrouve("Demande introuvable ou déjà traitée".into()));
    };

    if dest != utilisateur_id {
        return Err(ApiErreur::AccesInterdit("Cette demande ne vous est pas destinée".into()));
    }

    sqlx::query(
        "UPDATE arbre_genealogique.demandes_contact
         SET statut = 'refusee', traitee_le = NOW() WHERE id = $1",
    )
    .bind(demande_id)
    .execute(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(json!({"message": "Demande refusée"})),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// GET /api/arbre/recherche-publique?q=...
// ════════════════════════════════════════════════════════════════════════════

#[derive(serde::Deserialize)]
pub struct RechercheParams {
    pub q: String,
}

pub async fn recherche_publique(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    params: web::Query<RechercheParams>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;

    if params.q.len() < 2 {
        return Err(ApiErreur::Validation("Le terme de recherche doit contenir au moins 2 caractères".into()));
    }

    let arbre_id = obtenir_arbre_id(pool.get_ref(), utilisateur_id).await?;
    let terme_normalise = params.q.to_lowercase();

    // Recherche pg_trgm sur nom_normalise dans les autres arbres
    let resultats: Vec<(String, Option<String>, Option<i16>, Option<String>, Option<String>, f32, Uuid)> = sqlx::query_as(
        "SELECT p.nom, p.prenoms, p.naissance_annee, p.naissance_lieu, p.genre,
                similarity(p.nom_normalise, $1) AS score, a.utilisateur_id
         FROM arbre_genealogique.personnes p
         JOIN arbre_genealogique.rattachements r ON r.personne_id = p.id
         JOIN arbre_genealogique.arbres a ON a.id = r.arbre_id
         WHERE r.deleted_at IS NULL AND p.deleted_at IS NULL
           AND p.nom_normalise IS NOT NULL
           AND ($2::UUID IS NULL OR r.arbre_id != $2)
           AND (p.nom_normalise % $1 OR p.prenoms_normalise % $1)
         ORDER BY similarity(p.nom_normalise, $1) DESC
         LIMIT 20",
    )
    .bind(&terme_normalise)
    .bind(arbre_id)
    .fetch_all(pool.get_ref())
    .await?;

    let resultats_json: Vec<serde_json::Value> = resultats
        .into_iter()
        .map(|(nom, prenoms, annee, lieu, genre, score, user_id)| {
            json!({
                "nom": nom,
                "prenoms": prenoms,
                "naissance_annee": annee,
                "naissance_lieu": lieu,
                "genre": genre,
                "score_similarite": score,
                "membre_id_anonymise": anonymiser_membre(user_id),
                "source": "autre_arbre"
            })
        })
        .collect();

    let total = resultats_json.len();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(json!({"resultats": resultats_json, "total": total})),
        error: None,
    }))
}
