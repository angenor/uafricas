//! Routes publiques et membres des concours (feature 014, famille B).
//!
//! Chaque handler commence par `resoudre_concours` (PC1) : lire un concours
//! fait avancer son cycle. Pendant l'appel et le vote, rien de ce qui sort
//! d'ici ne dit qui a déposé quoi (FR-039).

use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::handlers::jeu::{garde_joueur, joueur_optionnel};
use crate::models::jeu_concours::{
    AuteurParticipation, ConcoursPublic, ConcoursRow, MaParticipation, MoiConcours,
    ParticipationAnonyme, Phase,
};
use crate::services::image_validation;
use crate::services::jeu_concours::{self as cycle};
use crate::ApiResponse;

/// Poids et longueur bornés d'un dépôt.
const TAILLE_MAX_LECTURE: usize = 15 * 1024 * 1024;
const LEGENDE_MAX: usize = 200;

async fn compter(pool: &PgPool, sql: &str, a: Uuid, b: Option<Uuid>) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar(sql).bind(a).bind(b).fetch_one(pool).await
}

/// Le concours tel que le public le voit, avec le bloc `moi` si un membre lit.
async fn vue_publique(
    pool: &PgPool,
    concours: ConcoursRow,
    membre: Option<Uuid>,
) -> Result<ConcoursPublic, ApiErreur> {
    let phase = concours.phase(Utc::now());
    let publiees = cycle::compter_publiees(pool, concours.id).await?;

    let moi = match membre {
        None => None,
        Some(m) => {
            let participations = sqlx::query_as::<_, MaParticipation>(
                "SELECT id, etat, media_type, media_url, legende, motif_rejet, created_at
                   FROM jeu.participation
                  WHERE concours_id = $1 AND auteur_id = $2 AND etat <> 'retiree'
                  ORDER BY created_at",
            )
            .bind(concours.id)
            .bind(m)
            .fetch_all(pool)
            .await?;
            let actives = participations.iter().filter(|p| matches!(p.etat.as_str(), "en_attente" | "publiee")).count();
            let votes = compter(
                pool,
                "SELECT COUNT(*) FROM jeu.confrontation WHERE concours_id = $1 AND votant_id = $2 AND choix_id IS NOT NULL",
                concours.id,
                Some(m),
            )
            .await?;
            Some(MoiConcours {
                peut_participer: phase == Phase::Appel && (actives as i16) < concours.participations_max,
                peut_voter: phase == Phase::Vote && concours.votes_max.is_none_or(|max| votes < i64::from(max)),
                participations,
                votes_exprimes: votes,
                votes_max: concours.votes_max,
            })
        }
    };

    Ok(ConcoursPublic {
        id: concours.id,
        format: concours.format,
        titre: concours.titre,
        theme: concours.theme,
        reglement: concours.reglement,
        rattachement: concours.rattachement,
        image_url: concours.image_url,
        phase,
        appel_debut: concours.appel_debut,
        vote_debut: concours.vote_debut,
        vote_fin: concours.vote_fin,
        participations_max: concours.participations_max,
        jury: concours.jury,
        motif_annulation: concours.motif_annulation,
        participations_publiees: publiees,
        moi,
    })
}

#[derive(Debug, Deserialize)]
pub struct ConcoursPublicsQuery {
    /// `appel`, `vote`, `resultats`, `en_cours` (appel ou vote), `termines`.
    pub phase: Option<String>,
    pub rattachement: Option<String>,
    pub page: Option<i64>,
    pub taille: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct PageConcours {
    pub elements: Vec<ConcoursPublic>,
    pub total: usize,
}

/// GET /api/jeu/concours — public. Les concours à venir ne sont pas listés :
/// on n'annonce pas un appel qui n'est pas encore ouvert.
pub async fn lister_concours(
    pool: web::Data<PgPool>,
    params: web::Query<ConcoursPublicsQuery>,
) -> Result<HttpResponse, ApiErreur> {
    let rattachement = params.rattachement.as_deref().map(str::trim).filter(|r| !r.is_empty());
    let ids: Vec<Uuid> = sqlx::query_scalar(
        "SELECT id FROM jeu.concours
          WHERE ($1::text IS NULL OR rattachement = $1) AND appel_debut <= NOW()
          ORDER BY vote_fin DESC",
    )
    .bind(rattachement)
    .fetch_all(pool.get_ref())
    .await?;

    let filtre = params.phase.as_deref().unwrap_or("");
    let mut elements = Vec::new();
    for id in ids {
        let concours = cycle::resoudre_concours(pool.get_ref(), id).await?; // PC1
        let phase = concours.phase(Utc::now());
        let retenu = match filtre {
            "" => true,
            "en_cours" => matches!(phase, Phase::Appel | Phase::Vote),
            "termines" => matches!(phase, Phase::Resultats | Phase::Deliberation),
            autre => phase.code() == autre,
        };
        // Un concours annulé ne s'affiche que si on le demande.
        if retenu && (phase != Phase::Annule || filtre == "annule") {
            elements.push(vue_publique(pool.get_ref(), concours, None).await?);
        }
    }

    let taille = params.taille.unwrap_or(12).clamp(1, 50) as usize;
    let page = params.page.unwrap_or(1).max(1) as usize;
    let total = elements.len();
    let elements = elements.into_iter().skip((page - 1) * taille).take(taille).collect();
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(PageConcours { elements, total }), error: None }))
}

/// GET /api/jeu/concours/{id} — public, jeton facultatif.
pub async fn obtenir_concours(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let concours = cycle::resoudre_concours(pool.get_ref(), path.into_inner()).await?;
    if concours.phase(Utc::now()) == Phase::AVenir {
        return Err(ApiErreur::NonTrouve("Concours introuvable".into()));
    }
    let vue = vue_publique(pool.get_ref(), concours, joueur_optionnel(&req)).await?;
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(vue), error: None }))
}

/// Une participation de la galerie une fois les résultats établis : l'auteur
/// est révélé, avec le rang et la proportion de confrontations gagnées.
#[derive(Debug, Serialize, FromRow)]
pub struct ParticipationClassee {
    pub id: Uuid,
    pub media_type: String,
    pub media_url: String,
    pub legende: Option<String>,
    pub auteur_id: Uuid,
    pub auteur_nom: String,
    pub auteur_prenom: String,
    pub auteur_photo_url: Option<String>,
    pub rang: Option<i16>,
    pub taux: Option<f64>,
    pub duels: Option<i32>,
    pub sous_seuil: Option<bool>,
    pub place_jury: Option<i16>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Galerie {
    Anonyme(Vec<ParticipationAnonyme>),
    Classee(Vec<serde_json::Value>),
}

/// GET /api/jeu/concours/{id}/participations — public.
///
/// Appel et vote : les participations publiées, SANS auteur, dans un ordre
/// tiré à chaque lecture (un ordre fixe trahirait l'ordre de dépôt ou un
/// classement). Résultats : classées, auteurs révélés. Annulé : auteurs
/// révélés, sans classement.
pub async fn galerie(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let concours = cycle::resoudre_concours(pool.get_ref(), path.into_inner()).await?;
    let phase = concours.phase(Utc::now());

    let data = match phase {
        Phase::AVenir => return Err(ApiErreur::NonTrouve("Concours introuvable".into())),
        Phase::Appel | Phase::Vote | Phase::Deliberation => Galerie::Anonyme(
            sqlx::query_as::<_, ParticipationAnonyme>(
                "SELECT id, media_type, media_url, legende FROM jeu.participation
                  WHERE concours_id = $1 AND etat = 'publiee' ORDER BY random() LIMIT 300",
            )
            .bind(concours.id)
            .fetch_all(pool.get_ref())
            .await?,
        ),
        Phase::Resultats | Phase::Annule => {
            let lignes = sqlx::query_as::<_, ParticipationClassee>(
                "SELECT p.id, p.media_type, p.media_url, p.legende, u.id AS auteur_id,
                        u.nom AS auteur_nom, u.prenom AS auteur_prenom, u.photo_url AS auteur_photo_url,
                        r.rang, (r.taux * 100)::float8 AS taux, r.duels, r.sous_seuil, r.place_jury
                   FROM jeu.participation p
                   JOIN iam.utilisateur u ON u.id = p.auteur_id
                   LEFT JOIN jeu.resultat_concours r ON r.participation_id = p.id
                  WHERE p.concours_id = $1 AND p.etat = 'publiee'
                  ORDER BY r.rang NULLS LAST, p.created_at",
            )
            .bind(concours.id)
            .fetch_all(pool.get_ref())
            .await?;
            Galerie::Classee(
                lignes
                    .into_iter()
                    .map(|l| {
                        serde_json::json!({
                            "id": l.id, "media_type": l.media_type, "media_url": l.media_url,
                            "legende": l.legende,
                            "auteur": AuteurParticipation {
                                id: l.auteur_id, nom: l.auteur_nom, prenom: l.auteur_prenom,
                                photo_url: l.auteur_photo_url,
                            },
                            "rang": l.rang, "taux": l.taux.map(|t| (t * 10.0).round() / 10.0),
                            "duels": l.duels, "sous_seuil": l.sous_seuil, "place_jury": l.place_jury,
                        })
                    })
                    .collect(),
            )
        }
    };
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(data), error: None }))
}

// ─── Dépôt ───────────────────────────────────────────────────────────────────

/// Ce qu'un formulaire de dépôt a envoyé.
struct Depot {
    photo: Option<Vec<u8>>,
    legende: Option<String>,
}

async fn lire_depot(mut payload: actix_multipart::Multipart) -> Result<Depot, ApiErreur> {
    use futures_util::StreamExt;
    let mut depot = Depot { photo: None, legende: None };
    while let Some(item) = payload.next().await {
        let mut champ = item.map_err(|e| ApiErreur::Upload(format!("Champ multipart invalide : {e}")))?;
        let nom = champ.content_disposition().and_then(|cd| cd.get_name()).unwrap_or("").to_string();
        let mut contenu: Vec<u8> = Vec::new();
        while let Some(morceau) = champ.next().await {
            let morceau = morceau.map_err(|e| ApiErreur::Upload(format!("Erreur de lecture : {e}")))?;
            if contenu.len() + morceau.len() > TAILLE_MAX_LECTURE {
                return Err(ApiErreur::Validation("Photo trop lourde : 15 Mo au plus".into()));
            }
            contenu.extend_from_slice(&morceau);
        }
        match nom.as_str() {
            "photo" if !contenu.is_empty() => depot.photo = Some(contenu),
            "legende" => {
                let texte = String::from_utf8_lossy(&contenu).trim().to_string();
                if texte.chars().count() > LEGENDE_MAX {
                    return Err(ApiErreur::Validation(format!("Légende : {LEGENDE_MAX} signes au plus")));
                }
                depot.legende = Some(texte).filter(|t| !t.is_empty());
            }
            _ => {}
        }
    }
    Ok(depot)
}

/// Normalise la photo (EXIF retirées) et l'écrit sous `uploads/jeu/concours/{id}/`.
fn enregistrer_photo(upload_dir: &str, concours_id: Uuid, octets: &[u8]) -> Result<String, ApiErreur> {
    let (normalisee, format, _, _) =
        image_validation::normaliser_photo(octets).map_err(|e| ApiErreur::Validation(e.message()))?;
    let nom = format!("{}.{}", Uuid::new_v4(), format.extension());
    let dossier = format!("{upload_dir}/jeu/concours/{concours_id}");
    std::fs::create_dir_all(&dossier)
        .map_err(|e| ApiErreur::Upload(format!("Impossible de créer le répertoire : {e}")))?;
    std::fs::write(format!("{dossier}/{nom}"), normalisee)
        .map_err(|e| ApiErreur::Upload(format!("Impossible d'écrire la photo : {e}")))?;
    Ok(format!("/uploads/jeu/concours/{concours_id}/{nom}"))
}

/// Refuse si le membre a déjà autant de participations actives que permis.
/// À appeler SOUS le verrou de la ligne du concours.
async fn verifier_plafond(
    conn: &mut sqlx::PgConnection,
    concours: &ConcoursRow,
    membre: Uuid,
) -> Result<(), ApiErreur> {
    let actives: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM jeu.participation
          WHERE concours_id = $1 AND auteur_id = $2 AND etat IN ('en_attente', 'publiee')",
    )
    .bind(concours.id)
    .bind(membre)
    .fetch_one(&mut *conn)
    .await?;
    if actives >= i64::from(concours.participations_max) {
        return Err(ApiErreur::Conflit(if concours.participations_max == 1 {
            "Vous avez déjà une photo dans ce concours".into()
        } else {
            format!("Vous avez déjà {} photos dans ce concours, le maximum", concours.participations_max)
        }));
    }
    Ok(())
}

async fn verrouiller(conn: &mut sqlx::PgConnection, id: Uuid) -> Result<ConcoursRow, ApiErreur> {
    use crate::models::jeu_concours::CONCOURS_COLONNES;
    sqlx::query_as::<_, ConcoursRow>(&format!("SELECT {CONCOURS_COLONNES} FROM jeu.concours c WHERE c.id = $1 FOR UPDATE"))
        .bind(id)
        .fetch_optional(&mut *conn)
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Concours introuvable".into()))
}

fn exiger_appel(concours: &ConcoursRow) -> Result<(), ApiErreur> {
    if concours.phase(Utc::now()) != Phase::Appel {
        // Le fuseau du membre n'est pas connu du serveur : la date est en UTC.
        return Err(ApiErreur::Conflit(format!(
            "L'appel à participation est clos depuis le {} (UTC)",
            concours.vote_debut.format("%d/%m/%Y à %H:%M")
        )));
    }
    Ok(())
}

/// POST /api/jeu/concours/{id}/participations — membre (multipart : photo, legende).
pub async fn deposer(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    upload_dir: web::Data<String>,
    path: web::Path<Uuid>,
    payload: actix_multipart::Multipart,
) -> Result<HttpResponse, ApiErreur> {
    let membre = garde_joueur(pool.get_ref(), &req).await?;
    let id = path.into_inner();
    let concours = cycle::resoudre_concours(pool.get_ref(), id).await?;
    exiger_appel(&concours)?;

    let depot = lire_depot(payload).await?;
    let photo = depot.photo.ok_or_else(|| ApiErreur::Validation("Choisissez une photo".into()))?;

    let mut tx = pool.begin().await?;
    let concours = verrouiller(&mut tx, id).await?;
    exiger_appel(&concours)?;
    verifier_plafond(&mut tx, &concours, membre).await?;
    let media_url = enregistrer_photo(upload_dir.get_ref(), id, &photo)?;
    let creee = sqlx::query_as::<_, MaParticipation>(
        "INSERT INTO jeu.participation (concours_id, auteur_id, media_url, legende)
         VALUES ($1, $2, $3, $4)
         RETURNING id, etat, media_type, media_url, legende, motif_rejet, created_at",
    )
    .bind(id)
    .bind(membre)
    .bind(&media_url)
    .bind(&depot.legende)
    .fetch_one(&mut *tx)
    .await?;
    tx.commit().await?;

    Ok(HttpResponse::Created().json(ApiResponse { success: true, data: Some(creee), error: None }))
}

async fn participation_du_membre(
    pool: &PgPool,
    concours_id: Uuid,
    pid: Uuid,
    membre: Uuid,
) -> Result<MaParticipation, ApiErreur> {
    sqlx::query_as::<_, MaParticipation>(
        "SELECT id, etat, media_type, media_url, legende, motif_rejet, created_at
           FROM jeu.participation WHERE id = $1 AND concours_id = $2 AND auteur_id = $3",
    )
    .bind(pid)
    .bind(concours_id)
    .bind(membre)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Participation introuvable".into()))
}

/// PUT /api/jeu/concours/{id}/participations/{pid} — membre, auteur (multipart).
///
/// Remplace la photo et/ou la légende tant que la participation n'est pas
/// publiée. Une participation refusée qu'on remplace repasse en attente.
pub async fn remplacer(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    upload_dir: web::Data<String>,
    path: web::Path<(Uuid, Uuid)>,
    payload: actix_multipart::Multipart,
) -> Result<HttpResponse, ApiErreur> {
    let membre = garde_joueur(pool.get_ref(), &req).await?;
    let (id, pid) = path.into_inner();
    let concours = cycle::resoudre_concours(pool.get_ref(), id).await?;
    exiger_appel(&concours)?;
    let avant = participation_du_membre(pool.get_ref(), id, pid, membre).await?;
    if !matches!(avant.etat.as_str(), "en_attente" | "rejetee") {
        return Err(ApiErreur::Conflit(
            "Une photo publiée ne se remplace plus : retirez-la si vous le souhaitez".into(),
        ));
    }

    let depot = lire_depot(payload).await?;
    let mut tx = pool.begin().await?;
    let concours = verrouiller(&mut tx, id).await?;
    exiger_appel(&concours)?;
    if avant.etat == "rejetee" {
        // Repasser en attente rend la participation active : le plafond compte.
        verifier_plafond(&mut tx, &concours, membre).await?;
    }
    let media_url = match &depot.photo {
        Some(octets) => enregistrer_photo(upload_dir.get_ref(), id, octets)?,
        None => avant.media_url.clone(),
    };
    let legende = if depot.photo.is_some() || depot.legende.is_some() { depot.legende } else { avant.legende };
    let apres = sqlx::query_as::<_, MaParticipation>(
        "UPDATE jeu.participation
            SET media_url = $2, legende = $3, etat = 'en_attente', motif_rejet = NULL,
                modere_par = NULL, modere_at = NULL, updated_at = NOW()
          WHERE id = $1
          RETURNING id, etat, media_type, media_url, legende, motif_rejet, created_at",
    )
    .bind(pid)
    .bind(&media_url)
    .bind(&legende)
    .fetch_one(&mut *tx)
    .await?;
    tx.commit().await?;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(apres), error: None }))
}

/// DELETE /api/jeu/concours/{id}/participations/{pid} — membre, auteur.
/// Retirée, elle sort du vote ; ses confrontations ne compteront plus.
pub async fn retirer(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    let membre = garde_joueur(pool.get_ref(), &req).await?;
    let (id, pid) = path.into_inner();
    let concours = cycle::resoudre_concours(pool.get_ref(), id).await?;
    if matches!(concours.phase(Utc::now()), Phase::Resultats | Phase::Annule) {
        return Err(ApiErreur::Conflit("Ce concours est terminé : sa galerie est figée".into()));
    }
    participation_du_membre(pool.get_ref(), id, pid, membre).await?;
    sqlx::query(
        "UPDATE jeu.participation SET etat = 'retiree', updated_at = NOW()
          WHERE id = $1 AND etat IN ('en_attente', 'publiee', 'rejetee')",
    )
    .bind(pid)
    .execute(pool.get_ref())
    .await?;
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(serde_json::json!({ "retiree": true })), error: None }))
}

/// Une participation dans « Mes activités », avec son concours et, une fois les
/// résultats établis, son rang et ce qu'elle a rapporté.
#[derive(Debug, Serialize, FromRow)]
pub struct ParticipationHistorique {
    pub id: Uuid,
    pub concours_id: Uuid,
    pub concours_titre: String,
    pub etat: String,
    pub media_url: String,
    pub legende: Option<String>,
    pub motif_rejet: Option<String>,
    pub created_at: DateTime<Utc>,
    pub rang: Option<i16>,
    pub taux: Option<f64>,
    pub gain: i64,
    #[sqlx(skip)]
    pub phase: Option<Phase>,
}

/// GET /api/jeu/concours/mes-participations — membre.
pub async fn mes_participations(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let membre = garde_joueur(pool.get_ref(), &req).await?;
    let concours_ids: Vec<Uuid> = sqlx::query_scalar(
        "SELECT DISTINCT concours_id FROM jeu.participation WHERE auteur_id = $1",
    )
    .bind(membre)
    .fetch_all(pool.get_ref())
    .await?;
    let mut phases = std::collections::HashMap::new();
    for id in concours_ids {
        let c = cycle::resoudre_concours(pool.get_ref(), id).await?; // PC1
        phases.insert(id, c.phase(Utc::now()));
    }

    let mut lignes = sqlx::query_as::<_, ParticipationHistorique>(
        "SELECT p.id, p.concours_id, c.titre AS concours_titre, p.etat, p.media_url, p.legende,
                p.motif_rejet, p.created_at, r.rang, (r.taux * 100)::float8 AS taux,
                COALESCE((SELECT SUM(g.montant) FROM jeu.gain g
                           WHERE g.annule_at IS NULL AND g.utilisateur_id = p.auteur_id
                             AND g.cle_idempotence IN ('concours:' || c.id || ':participation:' || p.id,
                                                       'concours:' || c.id || ':podium:' || p.id)), 0) AS gain
           FROM jeu.participation p
           JOIN jeu.concours c ON c.id = p.concours_id
           LEFT JOIN jeu.resultat_concours r ON r.participation_id = p.id
          WHERE p.auteur_id = $1 AND p.etat <> 'retiree'
          ORDER BY p.created_at DESC",
    )
    .bind(membre)
    .fetch_all(pool.get_ref())
    .await?;
    for l in &mut lignes {
        l.phase = phases.get(&l.concours_id).copied();
        l.taux = l.taux.map(|t| (t * 10.0).round() / 10.0);
    }
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(lignes), error: None }))
}

// ─── Vote ────────────────────────────────────────────────────────────────────

async fn concours_en_vote(pool: &PgPool, id: Uuid) -> Result<ConcoursRow, ApiErreur> {
    let concours = cycle::resoudre_concours(pool, id).await?; // PC1
    if concours.phase(Utc::now()) != Phase::Vote {
        return Err(ApiErreur::Conflit("Le vote de ce concours n'est pas ouvert".into()));
    }
    Ok(concours)
}

/// POST /api/jeu/concours/{id}/confrontations — membre.
/// Rend la paire ouverte du votant, ou en tire une nouvelle.
pub async fn confrontation_courante(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let votant = garde_joueur(pool.get_ref(), &req).await?;
    let concours = concours_en_vote(pool.get_ref(), path.into_inner()).await?;
    let tirage = cycle::tirer_confrontation(pool.get_ref(), &concours, votant).await?;
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(tirage), error: None }))
}

#[derive(Debug, Deserialize)]
pub struct VoteRequest {
    /// `gauche` ou `droite`.
    pub choix: String,
}

/// POST /api/jeu/concours/{id}/confrontations/{cid}/voter — membre.
/// Renvoie directement la paire suivante, pour enchaîner sans aller-retour.
pub async fn voter(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<VoteRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let votant = garde_joueur(pool.get_ref(), &req).await?;
    let (id, confrontation) = path.into_inner();
    let gauche = match body.choix.as_str() {
        "gauche" => true,
        "droite" => false,
        _ => return Err(ApiErreur::Validation("Choix attendu : gauche ou droite".into())),
    };
    let concours = concours_en_vote(pool.get_ref(), id).await?;
    let suivante = cycle::voter(pool.get_ref(), &concours, votant, confrontation, gauche).await?;
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(suivante), error: None }))
}
