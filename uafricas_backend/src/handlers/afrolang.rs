use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse};
use futures_util::StreamExt;
use sqlx::PgPool;
use std::io::Write;
use uuid::Uuid;

use crate::config::LivekitConfig;
use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::afrolang::{
    AfrolangStatsResponse, CreerMessageRequest, CreerRessourceLienRequest,
    CreerSallePriveePubliquePayload, CreerSessionRequest, DemarrerRejoindreResponse,
    GroupeEthniqueFiltres, GroupeEthniqueListeResponse, GroupeEthniqueResume,
    MessageSessionResponse, MessageSessionRow, MessagesFiltres, ModerateurResponse,
    ModifierCodeAccesRequest, ModifierMaxParticipantsRequest, ModifierSallePriveeRequest,
    ModifierSalleRequest, RejoindreRequest, RessourceSalleResponse, RessourceSalleRow,
    SalleDetailResponse, SalleFiltres, SalleListeResponse, SallePriveeAPI,
    SallePriveeDetailResponse, SallePriveeRow,
    SalleRow, SessionDetailResponse, SessionFiltres, SessionListeResponse, SessionParticipantRow,
    SessionRow, TransfererModerationRequest, VerifierCodeAccesRequest, VerifierCodeAccesResponse,
    GROUPE_ETHNIQUE_RESUME_COLONNES, MESSAGE_SESSION_COLONNES, RESSOURCE_SALLE_COLONNES,
    SALLE_COLONNES, SALLE_PRIVEE_COLONNES, SESSION_COLONNES, generer_slug,
};
use crate::models::notification;
use crate::services::{afrolang_rate_limit, audit};

/// Reponse API generique
#[derive(serde::Serialize)]
struct ApiResponse<T: serde::Serialize> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

// ══════════════════════════════════════════════════════════════════════════
// Fonctions utilitaires
// ══════════════════════════════════════════════════════════════════════════

/// Extraire l'utilisateur connecte depuis le header Authorization
fn extraire_utilisateur_id(req: &HttpRequest) -> Option<Uuid> {
    let header = req.headers().get("Authorization")?.to_str().ok()?;
    let token = header.strip_prefix("Bearer ")?;
    let secret = std::env::var("JWT_SECRET").ok()?;
    let claims = jwt::valider_token(token, &secret).ok()?;
    Uuid::parse_str(&claims.sub).ok()
}

/// Verifier si l'utilisateur a le role admin
async fn verifier_admin(pool: &PgPool, utilisateur_id: Uuid) -> Result<bool, ApiErreur> {
    let is_admin: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM iam.utilisateur_role ur
            JOIN iam.role r ON ur.role_id = r.id
            WHERE ur.utilisateur_id = $1 AND r.slug = 'admin'
        )",
    )
    .bind(utilisateur_id)
    .fetch_one(pool)
    .await?;
    Ok(is_admin)
}

/// Lire le contenu texte d'un champ multipart
async fn lire_champ_texte(field: &mut actix_multipart::Field) -> Result<String, ApiErreur> {
    let mut contenu = Vec::new();
    while let Some(chunk) = field.next().await {
        let data = chunk.map_err(|e| ApiErreur::Upload(format!("Erreur lecture champ: {}", e)))?;
        contenu.extend_from_slice(&data);
    }
    String::from_utf8(contenu)
        .map_err(|e| ApiErreur::Upload(format!("Encodage UTF-8 invalide: {}", e)))
}

/// Sauvegarder un fichier uploade sur le disque local
async fn sauvegarder_fichier(field: &mut actix_multipart::Field, chemin: &str) -> Result<(), ApiErreur> {
    if let Some(parent) = std::path::Path::new(chemin).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| ApiErreur::Upload(format!("Impossible de creer le repertoire: {}", e)))?;
    }
    let mut fichier = std::fs::File::create(chemin)
        .map_err(|e| ApiErreur::Upload(format!("Impossible de creer le fichier: {}", e)))?;
    while let Some(chunk) = field.next().await {
        let data = chunk.map_err(|e| ApiErreur::Upload(format!("Erreur lecture fichier: {}", e)))?;
        fichier
            .write_all(&data)
            .map_err(|e| ApiErreur::Upload(format!("Erreur ecriture fichier: {}", e)))?;
    }
    Ok(())
}

fn calculer_total_pages(total: i64, par_page: i64) -> i64 {
    if total == 0 { 1 } else { (total as f64 / par_page as f64).ceil() as i64 }
}

// ══════════════════════════════════════════════════════════════════════════
// 1.4 — Handlers annuaire groupes ethniques (feature 005, US1)
// ══════════════════════════════════════════════════════════════════════════

/// GET /api/afrolang/groupes-ethniques — Annuaire ethnique avec état de salle
pub async fn lister_groupes_ethniques(
    pool: web::Data<PgPool>,
    params: web::Query<GroupeEthniqueFiltres>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(24).clamp(1, 100);
    let offset = (page - 1) * par_page;

    let mut conditions: Vec<String> = Vec::new();
    let mut str_binds: Vec<String> = Vec::new();
    let mut uuid_binds: Vec<Uuid> = Vec::new();
    let mut param_types: Vec<&str> = Vec::new();
    let mut bind_index: u32 = 1;

    if let Some(ref q) = params.q {
        if !q.trim().is_empty() {
            conditions.push(format!(
                "lower(unaccent(ge.nom)) LIKE lower(unaccent(${}))",
                bind_index
            ));
            str_binds.push(format!("%{}%", q.trim()));
            param_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(pays_id) = params.pays_id {
        conditions.push(format!("fp.pays_id = ${}", bind_index));
        uuid_binds.push(pays_id);
        param_types.push("uuid");
        bind_index += 1;
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // Count
    let count_sql = format!(
        "SELECT COUNT(*)
         FROM country_profile.groupe_ethnique ge
         LEFT JOIN country_profile.fiche_pays fp ON fp.id = ge.fiche_pays_id
         {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    let mut str_idx = 0usize;
    let mut uuid_idx = 0usize;
    for pt in &param_types {
        match *pt {
            "str" => {
                count_q = count_q.bind(&str_binds[str_idx]);
                str_idx += 1;
            }
            "uuid" => {
                count_q = count_q.bind(uuid_binds[uuid_idx]);
                uuid_idx += 1;
            }
            _ => {}
        }
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // Data
    let data_sql = format!(
        "SELECT {}
         FROM country_profile.groupe_ethnique ge
         LEFT JOIN country_profile.fiche_pays fp ON fp.id = ge.fiche_pays_id
         LEFT JOIN shared.pays p ON p.id = fp.pays_id
         LEFT JOIN afrolang.salle s
            ON s.groupe_ethnique_id = ge.id
           AND s.actif = TRUE
           AND s.deleted_at IS NULL
         {}
         ORDER BY ge.nom ASC
         LIMIT ${} OFFSET ${}",
        GROUPE_ETHNIQUE_RESUME_COLONNES, where_clause, bind_index, bind_index + 1
    );

    let mut data_q = sqlx::query_as::<_, GroupeEthniqueResume>(&data_sql);
    str_idx = 0;
    uuid_idx = 0;
    for pt in &param_types {
        match *pt {
            "str" => {
                data_q = data_q.bind(&str_binds[str_idx]);
                str_idx += 1;
            }
            "uuid" => {
                data_q = data_q.bind(uuid_binds[uuid_idx]);
                uuid_idx += 1;
            }
            _ => {}
        }
    }
    data_q = data_q.bind(par_page).bind(offset);

    let rows = data_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(GroupeEthniqueListeResponse {
            groupes: rows.iter().map(|r| r.to_response()).collect(),
            total,
            page,
            par_page,
            total_pages: calculer_total_pages(total, par_page),
        }),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════════════
// 1.5 — Handlers salles publiques
// ══════════════════════════════════════════════════════════════════════════

/// GET /api/afrolang/salles — Liste paginee des salles publiques actives
pub async fn lister_salles(
    pool: web::Data<PgPool>,
    params: web::Query<SalleFiltres>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * par_page;

    let mut conditions: Vec<String> = vec!["s.actif = true".to_string(), "s.deleted_at IS NULL".to_string()];
    let mut bind_index = 1u32;
    let mut str_binds: Vec<String> = Vec::new();
    let mut uuid_binds: Vec<Uuid> = Vec::new();
    let mut param_types: Vec<&str> = Vec::new();

    if let Some(ref langue) = params.langue {
        if !langue.trim().is_empty() {
            conditions.push(format!("LOWER(s.langue_cible) = LOWER(${})", bind_index));
            str_binds.push(langue.trim().to_string());
            param_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(ref code) = params.langue_code {
        if !code.trim().is_empty() {
            conditions.push(format!("LOWER(s.langue_code) = LOWER(${})", bind_index));
            str_binds.push(code.trim().to_string());
            param_types.push("str");
            bind_index += 1;
        }
    }

    if let Some(groupe_id) = params.groupe_ethnique_id {
        conditions.push(format!("s.groupe_ethnique_id = ${}", bind_index));
        uuid_binds.push(groupe_id);
        param_types.push("uuid");
        bind_index += 1;
    }

    if let Some(ref recherche) = params.recherche {
        if !recherche.trim().is_empty() {
            let terme = format!("%{}%", recherche.trim().to_lowercase());
            conditions.push(format!(
                "(LOWER(s.titre) LIKE ${idx} OR LOWER(s.description) LIKE ${idx})",
                idx = bind_index
            ));
            str_binds.push(terme);
            param_types.push("str");
            bind_index += 1;
        }
    }

    let where_clause = conditions.join(" AND ");

    // Compter le total
    let count_query = format!("SELECT COUNT(*) FROM afrolang.salle s WHERE {}", where_clause);
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    let mut str_idx = 0usize;
    let mut uuid_idx = 0usize;
    for pt in &param_types {
        match *pt {
            "str" => {
                count_q = count_q.bind(&str_binds[str_idx]);
                str_idx += 1;
            }
            "uuid" => {
                count_q = count_q.bind(uuid_binds[uuid_idx]);
                uuid_idx += 1;
            }
            _ => {}
        }
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // Recuperer les salles enrichies (groupe ethnique, comptages)
    let select_query = format!(
        "SELECT {},
            ge.nom AS groupe_ethnique_nom,
            ge.fiche_pays_id AS fiche_pays_id,
            p.nom AS pays_nom,
            (SELECT COUNT(*) FROM afrolang.salle_privee sp2
             WHERE sp2.salle_id = s.id AND sp2.actif = true
               AND sp2.archivee_at IS NULL AND sp2.deleted_at IS NULL)
                AS nombre_salles_privees,
            (SELECT COUNT(*) FROM afrolang.session ses2
             JOIN afrolang.salle_privee sp3 ON sp3.id = ses2.salle_privee_id
             WHERE sp3.salle_id = s.id AND ses2.etat = 'en_cours') AS sessions_en_cours,
            (SELECT COUNT(*) FROM afrolang.salle_moderateur sm
             WHERE sm.salle_id = s.id AND sm.actif = TRUE) AS nombre_moderateurs_attitres,
            (SELECT COUNT(*) FROM afrolang.ressource_salle rs
             WHERE rs.salle_id = s.id AND rs.etat = 'publiee' AND rs.deleted_at IS NULL)
                AS ressources_count
         FROM afrolang.salle s
         LEFT JOIN country_profile.groupe_ethnique ge ON ge.id = s.groupe_ethnique_id
         LEFT JOIN country_profile.fiche_pays fp ON fp.id = ge.fiche_pays_id
         LEFT JOIN shared.pays p ON p.id = fp.pays_id
         WHERE {}
         ORDER BY s.created_at DESC
         LIMIT ${} OFFSET ${}",
        SALLE_COLONNES, where_clause, bind_index, bind_index + 1
    );

    let mut select_q = sqlx::query_as::<_, SalleRow>(&select_query);
    str_idx = 0;
    uuid_idx = 0;
    for pt in &param_types {
        match *pt {
            "str" => {
                select_q = select_q.bind(&str_binds[str_idx]);
                str_idx += 1;
            }
            "uuid" => {
                select_q = select_q.bind(uuid_binds[uuid_idx]);
                uuid_idx += 1;
            }
            _ => {}
        }
    }
    select_q = select_q.bind(par_page).bind(offset);

    let rows = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(SalleListeResponse {
            salles: rows.iter().map(|r| r.to_response()).collect(),
            total,
            page,
            par_page,
            total_pages: calculer_total_pages(total, par_page),
        }),
        error: None,
    }))
}

/// GET /api/afrolang/salles/{id} — Detail d'une salle publique (feature 005)
pub async fn obtenir_salle(
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    use crate::models::afrolang::SalleModerateurRow;

    let id = chemin.into_inner();

    let query = format!(
        "SELECT {},
            ge.nom AS groupe_ethnique_nom,
            ge.fiche_pays_id AS fiche_pays_id,
            p.nom AS pays_nom,
            (SELECT COUNT(*) FROM afrolang.salle_privee sp2
             WHERE sp2.salle_id = s.id AND sp2.actif = true
               AND sp2.archivee_at IS NULL AND sp2.deleted_at IS NULL)
                AS nombre_salles_privees,
            (SELECT COUNT(*) FROM afrolang.session ses2
             JOIN afrolang.salle_privee sp3 ON sp3.id = ses2.salle_privee_id
             WHERE sp3.salle_id = s.id AND ses2.etat = 'en_cours') AS sessions_en_cours,
            (SELECT COUNT(*) FROM afrolang.salle_moderateur sm
             WHERE sm.salle_id = s.id AND sm.actif = TRUE) AS nombre_moderateurs_attitres,
            (SELECT COUNT(*) FROM afrolang.ressource_salle rs
             WHERE rs.salle_id = s.id AND rs.etat = 'publiee' AND rs.deleted_at IS NULL)
                AS ressources_count
         FROM afrolang.salle s
         LEFT JOIN country_profile.groupe_ethnique ge ON ge.id = s.groupe_ethnique_id
         LEFT JOIN country_profile.fiche_pays fp ON fp.id = ge.fiche_pays_id
         LEFT JOIN shared.pays p ON p.id = fp.pays_id
         WHERE s.id = $1 AND s.deleted_at IS NULL",
        SALLE_COLONNES
    );

    let salle = sqlx::query_as::<_, SalleRow>(&query)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve(format!("Salle {} non trouvee", id)))?;

    // Modérateurs attitrés actifs
    let mod_query = format!(
        "SELECT {},
            u.nom AS utilisateur_nom,
            u.prenom AS utilisateur_prenom,
            u.photo_url AS utilisateur_photo,
            u.email AS utilisateur_email
         FROM afrolang.salle_moderateur sm
         LEFT JOIN iam.utilisateur u ON u.id = sm.utilisateur_id
         WHERE sm.salle_id = $1 AND sm.actif = TRUE
         ORDER BY sm.designe_at ASC",
        crate::models::afrolang::SALLE_MODERATEUR_COLONNES
    );

    let moderateurs_attitres = sqlx::query_as::<_, SalleModerateurRow>(&mod_query)
        .bind(id)
        .fetch_all(pool.get_ref())
        .await?;

    // Charger les salles privees associees (actives et non archivées)
    let sp_query = format!(
        "SELECT {},
            u.nom AS createur_nom, u.prenom AS createur_prenom,
            u.photo_url AS createur_photo,
            s2.titre AS salle_titre, s2.langue_cible AS salle_langue,
            EXISTS(SELECT 1 FROM afrolang.session ses
                   WHERE ses.salle_privee_id = sp.id AND ses.etat = 'en_cours') AS session_en_cours
         FROM afrolang.salle_privee sp
         LEFT JOIN iam.utilisateur u ON u.id = sp.cree_par
         LEFT JOIN afrolang.salle s2 ON s2.id = sp.salle_id
         WHERE sp.salle_id = $1 AND sp.actif = true
           AND sp.archivee_at IS NULL AND sp.deleted_at IS NULL
         ORDER BY sp.created_at DESC",
        SALLE_PRIVEE_COLONNES
    );

    let salles_privees = sqlx::query_as::<_, SallePriveeRow>(&sp_query)
        .bind(id)
        .fetch_all(pool.get_ref())
        .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(SalleDetailResponse {
            id: salle.id,
            titre: salle.titre.clone(),
            slug: salle.slug.clone(),
            description: salle.description.clone(),
            image_couverture_url: salle.image_couverture_url.clone(),
            langue_cible: salle.langue_cible.clone(),
            langue_code: salle.langue_code.clone(),
            alphabet: salle.alphabet.clone(),
            dictionnaire_url: salle.dictionnaire_url.clone(),
            groupe_ethnique_id: salle.groupe_ethnique_id,
            groupe_ethnique: salle.to_groupe_ethnique_light(),
            actif: salle.actif,
            moderateurs_attitres: moderateurs_attitres
                .iter()
                .map(|m| m.to_response())
                .collect(),
            nombre_salles_privees: salle.nombre_salles_privees.unwrap_or(0),
            sessions_en_cours: salle.sessions_en_cours.unwrap_or(0),
            ressources_count: salle.ressources_count.unwrap_or(0),
            salles_privees: salles_privees.iter().map(|sp| sp.to_response()).collect(),
            created_at: salle.created_at,
            updated_at: salle.updated_at,
        }),
        error: None,
    }))
}

/// POST /api/afrolang/salles — Creation multipart (image + metadonnees) [Admin]
pub async fn creer_salle(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    mut payload: Multipart,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    if !verifier_admin(pool.get_ref(), utilisateur_id).await? {
        return Err(ApiErreur::NonAutorise("Droits administrateur requis".into()));
    }

    let mut titre: Option<String> = None;
    let mut description: Option<String> = None;
    let mut langue_cible: Option<String> = None;
    let mut langue_code: Option<String> = None;
    let mut alphabet: Option<String> = None;
    let mut dictionnaire_url: Option<String> = None;
    let mut groupe_ethnique_id: Option<Uuid> = None;
    let mut image_couverture_url: Option<String> = None;

    let upload_dir = std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".to_string());

    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|e| ApiErreur::Upload(format!("Erreur multipart: {}", e)))?;
        let nom_champ = field
            .content_disposition()
            .and_then(|cd| cd.get_name().map(|s| s.to_string()))
            .unwrap_or_default();

        match nom_champ.as_str() {
            "titre" => titre = Some(lire_champ_texte(&mut field).await?),
            "description" => description = Some(lire_champ_texte(&mut field).await?),
            "langue_cible" => langue_cible = Some(lire_champ_texte(&mut field).await?),
            "langue_code" => langue_code = Some(lire_champ_texte(&mut field).await?),
            "alphabet" => alphabet = Some(lire_champ_texte(&mut field).await?),
            "dictionnaire_url" => dictionnaire_url = Some(lire_champ_texte(&mut field).await?),
            "groupe_ethnique_id" => {
                let val = lire_champ_texte(&mut field).await?;
                groupe_ethnique_id = Uuid::parse_str(val.trim()).ok();
            }
            "couverture" | "image" => {
                let nom_original = field
                    .content_disposition()
                    .and_then(|cd| cd.get_filename().map(|f| sanitize_filename::sanitize(f)))
                    .unwrap_or_else(|| format!("{}.jpg", Uuid::new_v4()));
                let nom_fichier = format!("{}_{}", Uuid::new_v4(), nom_original);
                let chemin_complet = format!("{}/couvertures/{}", upload_dir, nom_fichier);
                sauvegarder_fichier(&mut field, &chemin_complet).await?;
                image_couverture_url = Some(format!("/uploads/couvertures/{}", nom_fichier));
            }
            _ => {
                while let Some(Ok(_)) = field.next().await {}
            }
        }
    }

    let titre = titre
        .filter(|t| !t.trim().is_empty())
        .ok_or_else(|| ApiErreur::Validation("Le titre est obligatoire".into()))?;

    let groupe_ethnique_id = groupe_ethnique_id.ok_or_else(|| {
        ApiErreur::Validation("Le groupe ethnique de rattachement est obligatoire".into())
    })?;

    let slug = generer_slug(&titre);

    let row = sqlx::query_as::<_, SalleRow>(
        &format!(
            "INSERT INTO afrolang.salle
                (titre, slug, description, image_couverture_url,
                 langue_cible, langue_code, alphabet, dictionnaire_url,
                 groupe_ethnique_id, cree_par)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
             RETURNING {}",
            SALLE_COLONNES.replace("s.", "")
        ),
    )
    .bind(titre.trim())
    .bind(&slug)
    .bind(description.as_deref().map(str::trim))
    .bind(&image_couverture_url)
    .bind(langue_cible.as_deref().map(str::trim))
    .bind(langue_code.as_deref().map(str::trim))
    .bind(alphabet.as_deref().map(str::trim))
    .bind(dictionnaire_url.as_deref().map(str::trim))
    .bind(groupe_ethnique_id)
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    log::info!("Salle afrolang creee: {} ({})", row.titre, row.id);

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

/// PUT /api/afrolang/salles/{id} — Modifier une salle [Admin]
pub async fn modifier_salle(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<ModifierSalleRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    if !verifier_admin(pool.get_ref(), utilisateur_id).await? {
        return Err(ApiErreur::NonAutorise("Droits administrateur requis".into()));
    }

    let id = chemin.into_inner();

    // Construire la requete UPDATE dynamiquement
    let mut sets: Vec<String> = vec!["updated_at = NOW()".to_string()];
    let mut bind_index = 2u32; // $1 = id
    let mut bind_values: Vec<String> = Vec::new();

    if let Some(ref titre) = body.titre {
        if !titre.trim().is_empty() {
            sets.push(format!("titre = ${}", bind_index));
            bind_values.push(titre.trim().to_string());
            bind_index += 1;
            let slug = generer_slug(titre);
            sets.push(format!("slug = ${}", bind_index));
            bind_values.push(slug);
            bind_index += 1;
        }
    }
    if let Some(ref desc) = body.description {
        sets.push(format!("description = ${}", bind_index));
        bind_values.push(desc.trim().to_string());
        bind_index += 1;
    }
    if let Some(ref langue) = body.langue_cible {
        sets.push(format!("langue_cible = ${}", bind_index));
        bind_values.push(langue.trim().to_string());
        bind_index += 1;
    }
    if let Some(ref code) = body.langue_code {
        sets.push(format!("langue_code = ${}", bind_index));
        bind_values.push(code.trim().to_string());
        bind_index += 1;
    }
    if let Some(ref alpha) = body.alphabet {
        sets.push(format!("alphabet = ${}", bind_index));
        bind_values.push(alpha.trim().to_string());
        bind_index += 1;
    }
    if let Some(ref dict) = body.dictionnaire_url {
        sets.push(format!("dictionnaire_url = ${}", bind_index));
        bind_values.push(dict.trim().to_string());
        bind_index += 1;
    }
    if let Some(ref groupe_id) = body.groupe_ethnique_id {
        sets.push(format!("groupe_ethnique_id = ${}::UUID", bind_index));
        bind_values.push(groupe_id.to_string());
        bind_index += 1;
    }

    let _ = bind_index; // supprimer le warning unused

    let query = format!(
        "UPDATE afrolang.salle SET {} WHERE id = $1 RETURNING {}",
        sets.join(", "),
        SALLE_COLONNES.replace("s.", "")
    );

    let mut q = sqlx::query_as::<_, SalleRow>(&query).bind(id);
    for val in &bind_values {
        q = q.bind(val);
    }

    let row = q
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve(format!("Salle {} non trouvee", id)))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

/// DELETE /api/afrolang/salles/{id} — Soft delete [Admin]
pub async fn supprimer_salle(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    if !verifier_admin(pool.get_ref(), utilisateur_id).await? {
        return Err(ApiErreur::NonAutorise("Droits administrateur requis".into()));
    }

    let id = chemin.into_inner();

    let result = sqlx::query(
        "UPDATE afrolang.salle SET actif = false, updated_at = NOW() WHERE id = $1 AND actif = true",
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve(format!("Salle {} non trouvee", id)));
    }

    log::info!("Salle afrolang desactivee: {}", id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════════════
// 1.6 — Handlers salles privees
// ══════════════════════════════════════════════════════════════════════════

/// GET /api/afrolang/salles/{salle_id}/salles-privees — Salles privées listées
/// dans le widget d'une salle publique (contrat endpoint 2, refonte 2026-04).
///
/// Toute salle privée non archivée et non supprimée est retournée : la
/// protection repose uniquement sur le code secret vérifié côté serveur à
/// l'endpoint `verifier-code`. L'auteur courant est signalé via `est_auteur`
/// pour permettre au frontend de court-circuiter la modale (FR-014).
pub async fn lister_salles_privees_par_salle_publique(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    // Accès public : un visiteur non authentifié doit pouvoir voir la liste.
    // L'authentification sera exigée au moment d'intégrer une salle (via
    // `verifier-code` / `demarrer-ou-rejoindre`). Sans utilisateur courant,
    // `est_auteur` vaudra toujours `false` (comparaison à `Uuid::nil()`).
    let utilisateur_id = extraire_utilisateur_id(&req).unwrap_or_else(Uuid::nil);

    let salle_id = chemin.into_inner();

    let select_query = format!(
        "SELECT {},
            u.nom AS createur_nom, u.prenom AS createur_prenom,
            u.photo_url AS createur_photo,
            s.titre AS salle_titre, s.langue_cible AS salle_langue,
            EXISTS(SELECT 1 FROM afrolang.session ses
                   WHERE ses.salle_privee_id = sp.id AND ses.etat = 'en_cours') AS session_en_cours
         FROM afrolang.salle_privee sp
         LEFT JOIN iam.utilisateur u ON u.id = sp.cree_par
         LEFT JOIN afrolang.salle s ON s.id = sp.salle_id
         WHERE sp.salle_id = $1
           AND sp.actif = TRUE
           AND sp.archivee_at IS NULL
           AND sp.deleted_at IS NULL
         ORDER BY sp.created_at DESC",
        SALLE_PRIVEE_COLONNES
    );

    let rows = sqlx::query_as::<_, SallePriveeRow>(&select_query)
        .bind(salle_id)
        .fetch_all(pool.get_ref())
        .await?;

    let salles: Vec<SallePriveeAPI> =
        rows.iter().map(|r| r.to_api(utilisateur_id)).collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(salles),
        error: None,
    }))
}

/// GET /api/afrolang/salles-privees/{id} — Detail d'une salle privee avec sessions
pub async fn obtenir_salle_privee(
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let id = chemin.into_inner();

    let query = format!(
        "SELECT {},
            u.nom AS createur_nom, u.prenom AS createur_prenom,
            u.photo_url AS createur_photo,
            s.titre AS salle_titre, s.langue_cible AS salle_langue,
            EXISTS(SELECT 1 FROM afrolang.session ses
                   WHERE ses.salle_privee_id = sp.id AND ses.etat = 'en_cours') AS session_en_cours
         FROM afrolang.salle_privee sp
         LEFT JOIN iam.utilisateur u ON u.id = sp.cree_par
         LEFT JOIN afrolang.salle s ON s.id = sp.salle_id
         WHERE sp.id = $1",
        SALLE_PRIVEE_COLONNES
    );

    let salle_privee = sqlx::query_as::<_, SallePriveeRow>(&query)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve(format!("Salle privee {} non trouvee", id)))?;

    // Charger les sessions associees
    let ses_query = format!(
        "SELECT {}
         FROM afrolang.session ses
         WHERE ses.salle_privee_id = $1
         ORDER BY ses.date_debut_prevue DESC NULLS LAST, ses.created_at DESC",
        SESSION_COLONNES
    );

    let sessions = sqlx::query_as::<_, SessionRow>(&ses_query)
        .bind(id)
        .fetch_all(pool.get_ref())
        .await?;

    let resp = salle_privee.to_response();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(SallePriveeDetailResponse {
            id: resp.id,
            salle_id: resp.salle_id,
            titre: resp.titre,
            description: resp.description,
            image_couverture_url: resp.image_couverture_url,
            max_participants: resp.max_participants,
            archivee_at: resp.archivee_at,
            actif: resp.actif,
            createur: resp.createur,
            salle_titre: resp.salle_titre,
            salle_langue: resp.salle_langue,
            session_en_cours: resp.session_en_cours,
            sessions: sessions.iter().map(|s| s.to_response()).collect(),
            created_at: resp.created_at,
            updated_at: resp.updated_at,
        }),
        error: None,
    }))
}

/// POST /api/afrolang/salles-privees — Création d'une salle privée par
/// l'utilisateur courant (refonte 2026-04, endpoint 1 du contrat).
///
/// Valide titre, description, code d'accès, vérifie que la salle publique
/// cible existe et est active, puis hashe le code avant l'INSERT. Retourne
/// 409 si l'utilisateur possède déjà une salle privée active pour la même
/// salle publique (FR-010).
pub async fn creer_salle_privee_publique(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    body: web::Json<CreerSallePriveePubliquePayload>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let titre = body.titre.trim().to_string();
    let titre_len = titre.chars().count();
    if !(5..=350).contains(&titre_len) {
        return Err(ApiErreur::Validation(
            "Le titre doit contenir entre 5 et 350 caractères".into(),
        ));
    }

    let description = body
        .description
        .as_ref()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    if let Some(ref d) = description {
        if d.chars().count() > 1000 {
            return Err(ApiErreur::Validation(
                "La description ne peut dépasser 1000 caractères".into(),
            ));
        }
    }

    valider_format_code_acces(body.code_acces.as_str())?;

    // Vérifier l'existence et l'activité de la salle publique.
    let salle_info: Option<(bool, Option<chrono::DateTime<chrono::Utc>>)> = sqlx::query_as(
        "SELECT actif, deleted_at FROM afrolang.salle WHERE id = $1",
    )
    .bind(body.salle_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let (salle_active, salle_deleted) = salle_info
        .ok_or_else(|| ApiErreur::Validation("Salle publique inexistante".into()))?;
    if salle_deleted.is_some() {
        return Err(ApiErreur::Validation("Salle publique supprimée".into()));
    }
    if !salle_active {
        // 422 selon le contrat — nous utilisons Validation (400) par défaut,
        // le contrat distingue « inactive » (422) de « inexistante » (400) :
        // on exprime cela via le message sans créer de variant supplémentaire.
        return Err(ApiErreur::Validation(
            "Salle publique inactive — création impossible".into(),
        ));
    }

    // Vérifier l'unicité (salle_id, utilisateur) active avant l'INSERT pour
    // fournir un 409 porteur d'information (salle_privee_existante_id).
    let existante_id: Option<Uuid> = sqlx::query_scalar(
        "SELECT id FROM afrolang.salle_privee
         WHERE salle_id = $1 AND cree_par = $2
           AND archivee_at IS NULL AND deleted_at IS NULL
         LIMIT 1",
    )
    .bind(body.salle_id)
    .bind(utilisateur_id)
    .fetch_optional(pool.get_ref())
    .await?;

    if let Some(existante) = existante_id {
        return Ok(HttpResponse::Conflict().json(ApiResponse {
            success: false,
            data: Some(serde_json::json!({
                "salle_privee_existante_id": existante,
            })),
            error: Some(
                "Vous avez déjà une salle privée pour cette salle publique".into(),
            ),
        }));
    }

    let code_hash = hasher_code_acces(body.code_acces.as_str())?;

    let insert_result = sqlx::query_as::<_, SallePriveeRow>(
        &format!(
            "INSERT INTO afrolang.salle_privee
                (salle_id, titre, description, code_acces_hash, cree_par)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING {}",
            SALLE_PRIVEE_COLONNES.replace("sp.", "")
        ),
    )
    .bind(body.salle_id)
    .bind(&titre)
    .bind(description.as_deref())
    .bind(&code_hash)
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await;

    let mut row = match insert_result {
        Ok(r) => r,
        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
            // Course critique : une salle a été créée entre notre vérification
            // et notre INSERT. Retourner le 409 de la même manière.
            let existante_id: Option<Uuid> = sqlx::query_scalar(
                "SELECT id FROM afrolang.salle_privee
                 WHERE salle_id = $1 AND cree_par = $2
                   AND archivee_at IS NULL AND deleted_at IS NULL
                 LIMIT 1",
            )
            .bind(body.salle_id)
            .bind(utilisateur_id)
            .fetch_optional(pool.get_ref())
            .await?;
            return Ok(HttpResponse::Conflict().json(ApiResponse {
                success: false,
                data: existante_id.map(|id| serde_json::json!({
                    "salle_privee_existante_id": id,
                })),
                error: Some(
                    "Vous avez déjà une salle privée pour cette salle publique".into(),
                ),
            }));
        }
        Err(e) => return Err(ApiErreur::from(e)),
    };

    // Hydrater les JOINs manquants avec une requête légère (auteur).
    let auteur: Option<(Option<String>, Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT nom, prenom, photo_url FROM iam.utilisateur WHERE id = $1",
    )
    .bind(utilisateur_id)
    .fetch_optional(pool.get_ref())
    .await?;
    if let Some((nom, prenom, photo)) = auteur {
        row.createur_nom = nom;
        row.createur_prenom = prenom;
        row.createur_photo = photo;
    }
    row.session_en_cours = Some(false);

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "creer_salle_privee",
        "afrolang",
        "salle_privee",
        Some(row.id),
        None,
        Some(serde_json::json!({
            "salle_id": body.salle_id,
            "titre": titre,
        })),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await;

    log::info!(
        "Salle privée créée : {} ({}) pour utilisateur {}",
        row.titre, row.id, utilisateur_id
    );

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(row.to_api(utilisateur_id)),
        error: None,
    }))
}

/// PUT /api/afrolang/salles-privees/{id} — Modifier sa salle privee [JWT createur]
pub async fn modifier_salle_privee(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<ModifierSallePriveeRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let id = chemin.into_inner();

    // Verifier que l'utilisateur est le createur
    let createur_id: Option<Uuid> = sqlx::query_scalar(
        "SELECT cree_par FROM afrolang.salle_privee WHERE id = $1 AND actif = true",
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?;

    match createur_id {
        None => return Err(ApiErreur::NonTrouve(format!("Salle privee {} non trouvee", id))),
        Some(cid) if cid != utilisateur_id => {
            return Err(ApiErreur::NonAutorise("Seul le createur peut modifier cette salle".into()));
        }
        _ => {}
    }

    let mut sets: Vec<String> = vec!["updated_at = NOW()".to_string()];
    let mut bind_index = 2u32;
    let mut bind_values: Vec<String> = Vec::new();

    if let Some(ref titre) = body.titre {
        if !titre.trim().is_empty() {
            sets.push(format!("titre = ${}", bind_index));
            bind_values.push(titre.trim().to_string());
            bind_index += 1;
        }
    }
    if let Some(ref desc) = body.description {
        sets.push(format!("description = ${}", bind_index));
        bind_values.push(desc.trim().to_string());
        bind_index += 1;
    }
    if let Some(max) = body.max_participants {
        sets.push(format!("max_participants = ${}", bind_index));
        bind_values.push(max.to_string());
        bind_index += 1;
    }

    let _ = bind_index;

    let query = format!(
        "UPDATE afrolang.salle_privee SET {} WHERE id = $1 RETURNING {}",
        sets.join(", "),
        SALLE_PRIVEE_COLONNES.replace("sp.", "")
    );

    let mut q = sqlx::query_as::<_, SallePriveeRow>(&query).bind(id);
    for val in &bind_values {
        q = q.bind(val);
    }

    let row = q
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve(format!("Salle privee {} non trouvee", id)))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

/// DELETE /api/afrolang/salles-privees/{id} — Soft delete [JWT createur]
pub async fn supprimer_salle_privee(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let id = chemin.into_inner();

    let createur_id: Option<Uuid> = sqlx::query_scalar(
        "SELECT cree_par FROM afrolang.salle_privee WHERE id = $1 AND actif = true",
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?;

    match createur_id {
        None => return Err(ApiErreur::NonTrouve(format!("Salle privee {} non trouvee", id))),
        Some(cid) if cid != utilisateur_id => {
            return Err(ApiErreur::NonAutorise("Seul le createur peut supprimer cette salle".into()));
        }
        _ => {}
    }

    sqlx::query("UPDATE afrolang.salle_privee SET actif = false, updated_at = NOW() WHERE id = $1")
        .bind(id)
        .execute(pool.get_ref())
        .await?;

    log::info!("Salle privee desactivee: {}", id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════════════
// 1.7 — Handlers sessions
// ══════════════════════════════════════════════════════════════════════════

/// Vérifie que l'utilisateur peut démarrer/terminer une session.
/// Règle :
///   - Salle privée : seul le créateur de la salle privée.
///   - Salle publique : créateur de la session OU modérateur attitré actif.
async fn peut_gerer_cycle_session(
    pool: &PgPool,
    session: &SessionRow,
    utilisateur_id: Uuid,
) -> Result<bool, ApiErreur> {
    if let Some(sp_id) = session.salle_privee_id {
        let createur: Uuid = sqlx::query_scalar(
            "SELECT cree_par FROM afrolang.salle_privee WHERE id = $1",
        )
        .bind(sp_id)
        .fetch_one(pool)
        .await?;
        return Ok(createur == utilisateur_id);
    }
    if let Some(salle_id) = session.salle_id {
        if session.cree_par == utilisateur_id {
            return Ok(true);
        }
        let attitre: bool = sqlx::query_scalar(
            "SELECT EXISTS(
                SELECT 1 FROM afrolang.salle_moderateur
                WHERE salle_id = $1 AND utilisateur_id = $2 AND actif = TRUE
            )",
        )
        .bind(salle_id)
        .bind(utilisateur_id)
        .fetch_one(pool)
        .await?;
        return Ok(attitre);
    }
    Ok(false)
}

/// POST /api/afrolang/salles/{salle_id}/sessions — Créer une session dans une salle publique [JWT]
///
/// Règle d'autorisation : tout utilisateur authentifié peut créer une session
/// (la salle publique n'appartient à personne). Le créateur devient modérateur
/// par défaut ; si un modérateur attitré rejoint plus tard, il prend la main
/// (FR-011 gérée par `rejoindre_session`).
pub async fn creer_session_salle_publique(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<CreerSessionRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let salle_id = chemin.into_inner();

    let salle_active: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM afrolang.salle
                       WHERE id = $1 AND actif = TRUE AND deleted_at IS NULL)",
    )
    .bind(salle_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !salle_active {
        return Err(ApiErreur::NonTrouve("Salle publique introuvable".into()));
    }

    let date_debut_prevue = body
        .date_debut_prevue
        .as_ref()
        .map(|s| {
            chrono::DateTime::parse_from_rfc3339(s)
                .or_else(|_| {
                    chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M")
                        .map(|dt| dt.and_utc().fixed_offset())
                })
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .map_err(|_| ApiErreur::Validation("Format de date invalide".into()))
        })
        .transpose()?;

    let row = sqlx::query_as::<_, SessionRow>(
        &format!(
            "INSERT INTO afrolang.session
                (salle_id, titre, moderateur_id, date_debut_prevue,
                 max_participants, tableau_blanc_actif, cree_par)
             VALUES ($1, $2, $3, $4, $5, $6, $3)
             RETURNING {}",
            SESSION_COLONNES.replace("ses.", "")
        ),
    )
    .bind(salle_id)
    .bind(body.titre.as_deref().map(str::trim))
    .bind(utilisateur_id)
    .bind(date_debut_prevue)
    .bind(body.max_participants.unwrap_or(50))
    .bind(body.tableau_blanc_actif.unwrap_or(true))
    .fetch_one(pool.get_ref())
    .await?;

    log::info!(
        "Session salle publique planifiée: {:?} ({})",
        row.titre, row.id
    );

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

/// GET /api/afrolang/salles/{salle_id}/sessions — Sessions d'une salle publique
pub async fn lister_sessions_salle_publique(
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    params: web::Query<SessionFiltres>,
) -> Result<HttpResponse, ApiErreur> {
    let salle_id = chemin.into_inner();
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * par_page;

    let mut conditions: Vec<String> = vec!["ses.salle_id = $1".to_string()];
    let mut bind_index = 2u32;
    let mut bind_values: Vec<String> = Vec::new();

    if let Some(ref etat) = params.etat {
        if !etat.trim().is_empty() {
            conditions.push(format!("ses.etat::TEXT = ${}", bind_index));
            bind_values.push(etat.trim().to_string());
            bind_index += 1;
        }
    }

    let where_clause = conditions.join(" AND ");

    let count_query = format!(
        "SELECT COUNT(*) FROM afrolang.session ses WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query).bind(salle_id);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_query = format!(
        "SELECT {}
         FROM afrolang.session ses
         WHERE {}
         ORDER BY ses.date_debut_prevue DESC NULLS LAST, ses.created_at DESC
         LIMIT ${} OFFSET ${}",
        SESSION_COLONNES, where_clause, bind_index, bind_index + 1
    );

    let mut select_q = sqlx::query_as::<_, SessionRow>(&select_query).bind(salle_id);
    for val in &bind_values {
        select_q = select_q.bind(val);
    }
    select_q = select_q.bind(par_page).bind(offset);

    let rows = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(SessionListeResponse {
            sessions: rows.iter().map(|r| r.to_response()).collect(),
            total,
            page,
            par_page,
            total_pages: calculer_total_pages(total, par_page),
        }),
        error: None,
    }))
}

/// GET /api/afrolang/salles-privees/{sp_id}/sessions — Sessions d'une salle privee
pub async fn lister_sessions(
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    params: web::Query<SessionFiltres>,
) -> Result<HttpResponse, ApiErreur> {
    let sp_id = chemin.into_inner();
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * par_page;

    let mut conditions: Vec<String> = vec!["ses.salle_privee_id = $1".to_string()];
    let mut bind_index = 2u32;
    let mut bind_values: Vec<String> = Vec::new();

    if let Some(ref etat) = params.etat {
        if !etat.trim().is_empty() {
            conditions.push(format!("ses.etat::TEXT = ${}", bind_index));
            bind_values.push(etat.trim().to_string());
            bind_index += 1;
        }
    }

    let where_clause = conditions.join(" AND ");

    let count_query = format!(
        "SELECT COUNT(*) FROM afrolang.session ses WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query).bind(sp_id);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_query = format!(
        "SELECT {}
         FROM afrolang.session ses
         WHERE {}
         ORDER BY ses.date_debut_prevue DESC NULLS LAST, ses.created_at DESC
         LIMIT ${} OFFSET ${}",
        SESSION_COLONNES, where_clause, bind_index, bind_index + 1
    );

    let mut select_q = sqlx::query_as::<_, SessionRow>(&select_query).bind(sp_id);
    for val in &bind_values {
        select_q = select_q.bind(val);
    }
    select_q = select_q.bind(par_page).bind(offset);

    let rows = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(SessionListeResponse {
            sessions: rows.iter().map(|r| r.to_response()).collect(),
            total,
            page,
            par_page,
            total_pages: calculer_total_pages(total, par_page),
        }),
        error: None,
    }))
}

/// GET /api/afrolang/sessions/{id} — Detail d'une session avec participants
pub async fn obtenir_session(
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let id = chemin.into_inner();

    let query = format!(
        "SELECT {}
         FROM afrolang.session ses
         WHERE ses.id = $1",
        SESSION_COLONNES
    );

    let session = sqlx::query_as::<_, SessionRow>(&query)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve(format!("Session {} non trouvee", id)))?;

    // Charger les participants avec info utilisateur
    let participants = sqlx::query_as::<_, SessionParticipantRow>(
        "SELECT sp.id, sp.session_id, sp.utilisateur_id, sp.role_session,
                sp.rejoint_at, sp.quitte_at, sp.duree_secondes,
                u.nom, u.prenom, u.photo_url
         FROM afrolang.session_participant sp
         LEFT JOIN iam.utilisateur u ON u.id = sp.utilisateur_id
         WHERE sp.session_id = $1
         ORDER BY sp.rejoint_at ASC",
    )
    .bind(id)
    .fetch_all(pool.get_ref())
    .await?;

    // Charger le moderateur
    let moderateur = if let Some(mod_id) = session.moderateur_id {
        sqlx::query_as::<_, (Uuid, String, Option<String>, Option<String>)>(
            "SELECT id, nom, prenom, photo_url FROM iam.utilisateur WHERE id = $1",
        )
        .bind(mod_id)
        .fetch_optional(pool.get_ref())
        .await?
        .map(|(id, nom, prenom, photo_url)| ModerateurResponse {
            id,
            nom,
            prenom,
            photo_url,
        })
    } else {
        None
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(SessionDetailResponse {
            id: session.id,
            salle_privee_id: session.salle_privee_id,
            salle_id: session.salle_id,
            titre: session.titre.clone(),
            etat: session.etat.clone(),
            moderateur,
            date_debut_prevue: session.date_debut_prevue,
            demarre_at: session.demarre_at,
            termine_at: session.termine_at,
            duree_secondes: session.duree_secondes,
            max_participants: session.max_participants,
            nombre_participants_pic: session.nombre_participants_pic,
            tableau_blanc_actif: session.tableau_blanc_actif,
            participants: participants.iter().map(|p| p.to_response()).collect(),
            created_at: session.created_at,
            updated_at: session.updated_at,
        }),
        error: None,
    }))
}

/// POST /api/afrolang/salles-privees/{sp_id}/sessions — Planifier une session [JWT moderateur]
pub async fn creer_session(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<CreerSessionRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let sp_id = chemin.into_inner();

    // Verifier que l'utilisateur est le createur (moderateur) de la salle privee
    let createur_id: Option<Uuid> = sqlx::query_scalar(
        "SELECT cree_par FROM afrolang.salle_privee WHERE id = $1 AND actif = true",
    )
    .bind(sp_id)
    .fetch_optional(pool.get_ref())
    .await?;

    match createur_id {
        None => return Err(ApiErreur::NonTrouve("Salle privee non trouvee".into())),
        Some(cid) if cid != utilisateur_id => {
            return Err(ApiErreur::NonAutorise(
                "Seul le moderateur (createur) peut planifier une session".into(),
            ));
        }
        _ => {}
    }

    // Parser la date prevue si fournie
    let date_debut_prevue = body
        .date_debut_prevue
        .as_ref()
        .map(|s| {
            chrono::DateTime::parse_from_rfc3339(s)
                .or_else(|_| {
                    chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M")
                        .map(|dt| dt.and_utc().fixed_offset())
                })
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .map_err(|_| ApiErreur::Validation("Format de date invalide".into()))
        })
        .transpose()?;

    let row = sqlx::query_as::<_, SessionRow>(
        &format!(
            "INSERT INTO afrolang.session
                (salle_privee_id, titre, moderateur_id, date_debut_prevue,
                 max_participants, tableau_blanc_actif, cree_par)
             VALUES ($1, $2, $3, $4, $5, $6, $3)
             RETURNING {}",
            SESSION_COLONNES.replace("ses.", "")
        ),
    )
    .bind(sp_id)
    .bind(body.titre.as_deref().map(str::trim))
    .bind(utilisateur_id)
    .bind(date_debut_prevue)
    .bind(body.max_participants.unwrap_or(50))
    .bind(body.tableau_blanc_actif.unwrap_or(true))
    .fetch_one(pool.get_ref())
    .await?;

    log::info!("Session planifiee: {:?} ({})", row.titre, row.id);

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

/// PUT /api/afrolang/sessions/{id}/demarrer — Demarrer une session [JWT moderateur]
pub async fn demarrer_session(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let id = chemin.into_inner();

    // Charger la session et verifier l'etat
    let session = sqlx::query_as::<_, SessionRow>(
        &format!(
            "SELECT {} FROM afrolang.session ses WHERE ses.id = $1",
            SESSION_COLONNES
        ),
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve(format!("Session {} non trouvee", id)))?;

    if !peut_gerer_cycle_session(pool.get_ref(), &session, utilisateur_id).await? {
        return Err(ApiErreur::NonAutorise(
            "Seul le moderateur peut demarrer la session".into(),
        ));
    }

    if session.etat != "planifiee" {
        return Err(ApiErreur::Validation(format!(
            "La session ne peut etre demarree (etat actuel: {})",
            session.etat
        )));
    }

    // Mettre a jour l'etat
    let row = sqlx::query_as::<_, SessionRow>(
        &format!(
            "UPDATE afrolang.session
             SET etat = 'en_cours', demarre_at = NOW(), updated_at = NOW()
             WHERE id = $1
             RETURNING {}",
            SESSION_COLONNES.replace("ses.", "")
        ),
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;

    // Ajouter le moderateur comme participant
    sqlx::query(
        "INSERT INTO afrolang.session_participant (session_id, utilisateur_id, role_session)
         VALUES ($1, $2, 'moderateur')
         ON CONFLICT (session_id, utilisateur_id) DO NOTHING",
    )
    .bind(id)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?;

    // Mettre a jour le pic de participants
    sqlx::query(
        "UPDATE afrolang.session SET nombre_participants_pic = GREATEST(nombre_participants_pic,
            (SELECT COUNT(*) FROM afrolang.session_participant
             WHERE session_id = $1 AND quitte_at IS NULL))
         WHERE id = $1",
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Session demarree: {}", id);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

/// PUT /api/afrolang/sessions/{id}/terminer — Terminer une session [JWT moderateur]
pub async fn terminer_session(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let id = chemin.into_inner();

    // Charger la session
    let session = sqlx::query_as::<_, SessionRow>(
        &format!(
            "SELECT {} FROM afrolang.session ses WHERE ses.id = $1",
            SESSION_COLONNES
        ),
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve(format!("Session {} non trouvee", id)))?;

    if !peut_gerer_cycle_session(pool.get_ref(), &session, utilisateur_id).await? {
        return Err(ApiErreur::NonAutorise(
            "Seul le moderateur peut terminer la session".into(),
        ));
    }

    if session.etat != "en_cours" {
        return Err(ApiErreur::Validation(format!(
            "La session ne peut etre terminee (etat actuel: {})",
            session.etat
        )));
    }

    // Terminer la session et calculer la duree
    let row = sqlx::query_as::<_, SessionRow>(
        &format!(
            "UPDATE afrolang.session
             SET etat = 'terminee', termine_at = NOW(),
                 duree_secondes = EXTRACT(EPOCH FROM (NOW() - demarre_at))::INT,
                 updated_at = NOW()
             WHERE id = $1
             RETURNING {}",
            SESSION_COLONNES.replace("ses.", "")
        ),
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;

    // Mettre a jour tous les participants encore actifs
    sqlx::query(
        "UPDATE afrolang.session_participant
         SET quitte_at = NOW(),
             duree_secondes = EXTRACT(EPOCH FROM (NOW() - rejoint_at))::INT
         WHERE session_id = $1 AND quitte_at IS NULL",
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Session terminee: {}", id);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

/// POST /api/afrolang/sessions/{id}/rejoindre — Rejoindre une session [JWT]
pub async fn rejoindre_session(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<RejoindreRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let id = chemin.into_inner();

    // Charger la session
    let session = sqlx::query_as::<_, SessionRow>(
        &format!(
            "SELECT {} FROM afrolang.session ses WHERE ses.id = $1",
            SESSION_COLONNES
        ),
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve(format!("Session {} non trouvee", id)))?;

    // Verifier que la session est en cours
    if session.etat != "en_cours" {
        return Err(ApiErreur::Validation(
            "La session n'est pas en cours".into(),
        ));
    }

    // Note (refonte 2026-04) : la vérification du code secret d'une salle
    // privée se fait désormais à l'endpoint dédié `verifier-code` + jeton
    // d'accès porté par `demarrer-ou-rejoindre`. Cet endpoint n'applique
    // plus de contrôle de code ici.
    let _ = &body;

    // Verifier max_participants
    let nb_actifs: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM afrolang.session_participant
         WHERE session_id = $1 AND quitte_at IS NULL",
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;

    if let Some(max) = session.max_participants {
        if nb_actifs >= max as i64 {
            return Err(ApiErreur::Validation(
                "Nombre maximum de participants atteint".into(),
            ));
        }
    }

    // Insérer le participant (ON CONFLICT pour gérer les re-connexions)
    sqlx::query(
        "INSERT INTO afrolang.session_participant (session_id, utilisateur_id, role_session)
         VALUES ($1, $2, 'participant')
         ON CONFLICT (session_id, utilisateur_id)
         DO UPDATE SET quitte_at = NULL, rejoint_at = NOW()",
    )
    .bind(id)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?;

    // Mettre à jour le pic de participants
    sqlx::query(
        "UPDATE afrolang.session SET nombre_participants_pic = GREATEST(nombre_participants_pic,
            (SELECT COUNT(*) FROM afrolang.session_participant
             WHERE session_id = $1 AND quitte_at IS NULL)),
            updated_at = NOW()
         WHERE id = $1",
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    // ── Règles de modération dynamique (feature 005 US3) ──
    // La salle publique est soit directement session.salle_id,
    // soit dérivée de session.salle_privee_id → salle_privee.salle_id
    let salle_publique_id: Option<Uuid> = if let Some(sid) = session.salle_id {
        Some(sid)
    } else if let Some(sp_id) = session.salle_privee_id {
        sqlx::query_scalar(
            "SELECT sp.salle_id FROM afrolang.salle_privee sp WHERE sp.id = $1",
        )
        .bind(sp_id)
        .fetch_optional(pool.get_ref())
        .await?
    } else {
        None
    };

    if let Some(salle_pub_id) = salle_publique_id {
        let arrivant_est_attitre: bool = sqlx::query_scalar(
            "SELECT EXISTS(
                SELECT 1 FROM afrolang.salle_moderateur
                WHERE salle_id = $1 AND utilisateur_id = $2 AND actif = TRUE
            )",
        )
        .bind(salle_pub_id)
        .bind(utilisateur_id)
        .fetch_one(pool.get_ref())
        .await?;

        let ancien_mod = session.moderateur_id;
        let mut nouveau_mod: Option<Uuid> = None;

        if ancien_mod.is_none() {
            // FR-009 : premier arrivé devient modérateur
            nouveau_mod = Some(utilisateur_id);
        } else if arrivant_est_attitre {
            // FR-011 : reprise automatique si l'actuel n'est pas attitré
            let actuel = ancien_mod.unwrap();
            if actuel != utilisateur_id {
                let actuel_est_attitre: bool = sqlx::query_scalar(
                    "SELECT EXISTS(
                        SELECT 1 FROM afrolang.salle_moderateur
                        WHERE salle_id = $1 AND utilisateur_id = $2 AND actif = TRUE
                    )",
                )
                .bind(salle_pub_id)
                .bind(actuel)
                .fetch_one(pool.get_ref())
                .await?;
                if !actuel_est_attitre {
                    nouveau_mod = Some(utilisateur_id);
                }
            }
        }

        if let Some(nouveau_id) = nouveau_mod {
            sqlx::query(
                "UPDATE afrolang.session SET moderateur_id = $2, updated_at = NOW() WHERE id = $1",
            )
            .bind(id)
            .bind(nouveau_id)
            .execute(pool.get_ref())
            .await?;

            // Mettre à jour les rôles des participants
            sqlx::query(
                "UPDATE afrolang.session_participant
                 SET role_session = CASE
                    WHEN utilisateur_id = $2 THEN 'moderateur'
                    ELSE 'participant'
                 END
                 WHERE session_id = $1
                   AND utilisateur_id IN ($2, COALESCE($3, '00000000-0000-0000-0000-000000000000'::uuid))",
            )
            .bind(id)
            .bind(nouveau_id)
            .bind(ancien_mod)
            .execute(pool.get_ref())
            .await?;

            // Notifications aux concernés
            let lien = format!("/afrolang/session/{}", id);
            if let Some(ancien) = ancien_mod {
                notification::creer_notification(
                    pool.get_ref(),
                    ancien,
                    notification::afrolang::MODERATION_REPRISE,
                    "Un modérateur attitré a rejoint la session et a pris la modération.",
                    Some(&lien),
                )
                .await;
            }
            notification::creer_notification(
                pool.get_ref(),
                nouveau_id,
                notification::afrolang::MODERATION_REPRISE,
                "Vous êtes désormais modérateur de cette session.",
                Some(&lien),
            )
            .await;
        }
    }

    log::info!("Utilisateur {} a rejoint la session {}", utilisateur_id, id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

/// POST /api/afrolang/sessions/{id}/quitter — Quitter une session [JWT]
pub async fn quitter_session(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let id = chemin.into_inner();

    // Charger la session AVANT la mise à jour pour pouvoir détecter si le partant
    // est le modérateur actif (règle FR-012)
    let session_opt = sqlx::query_as::<_, SessionRow>(&format!(
        "SELECT {} FROM afrolang.session ses WHERE ses.id = $1",
        SESSION_COLONNES
    ))
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?;
    let Some(session) = session_opt else {
        return Err(ApiErreur::NonTrouve(format!("Session {} non trouvée", id)));
    };

    let result = sqlx::query(
        "UPDATE afrolang.session_participant
         SET quitte_at = NOW(),
             duree_secondes = EXTRACT(EPOCH FROM (NOW() - rejoint_at))::INT
         WHERE session_id = $1 AND utilisateur_id = $2 AND quitte_at IS NULL",
    )
    .bind(id)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve(
            "Participant non trouvé dans cette session".into(),
        ));
    }

    log::info!("Utilisateur {} a quitté la session {}", utilisateur_id, id);

    // Vérifier s'il reste des participants actifs
    let participants_actifs: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM afrolang.session_participant
         WHERE session_id = $1 AND quitte_at IS NULL",
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;

    let session_terminee = if participants_actifs == 0 {
        // Terminer la session si elle est en cours
        let rows = sqlx::query(
            "UPDATE afrolang.session
             SET etat = 'terminee', termine_at = NOW(),
                 duree_secondes = EXTRACT(EPOCH FROM (NOW() - demarre_at))::INT,
                 updated_at = NOW()
             WHERE id = $1 AND etat = 'en_cours'",
        )
        .bind(id)
        .execute(pool.get_ref())
        .await?;

        if rows.rows_affected() > 0 {
            log::info!("Session {} terminée automatiquement (dernier participant parti)", id);
            true
        } else {
            false
        }
    } else if session.moderateur_id == Some(utilisateur_id) && session.etat == "en_cours" {
        // FR-012 : réattribuer au plus ancien participant actif, en priorisant un attitré
        // La salle publique = session.salle_id OU salle_privee.salle_id
        let salle_pub_id: Option<Uuid> = if let Some(sid) = session.salle_id {
            Some(sid)
        } else if let Some(sp_id) = session.salle_privee_id {
            sqlx::query_scalar(
                "SELECT sp.salle_id FROM afrolang.salle_privee sp WHERE sp.id = $1",
            )
            .bind(sp_id)
            .fetch_optional(pool.get_ref())
            .await?
        } else {
            None
        };

        let successeur: Option<Uuid> = if let Some(sp_id) = salle_pub_id {
            // 1. Chercher un attitré actif présent dans la session
            let attitre: Option<Uuid> = sqlx::query_scalar(
                "SELECT sp.utilisateur_id
                 FROM afrolang.session_participant sp
                 JOIN afrolang.salle_moderateur sm
                   ON sm.utilisateur_id = sp.utilisateur_id
                  AND sm.salle_id = $2
                  AND sm.actif = TRUE
                 WHERE sp.session_id = $1
                   AND sp.quitte_at IS NULL
                 ORDER BY sp.rejoint_at ASC, sm.designe_at ASC
                 LIMIT 1",
            )
            .bind(id)
            .bind(sp_id)
            .fetch_optional(pool.get_ref())
            .await?;

            if attitre.is_some() {
                attitre
            } else {
                // 2. Sinon le plus ancien participant actif
                sqlx::query_scalar(
                    "SELECT utilisateur_id FROM afrolang.session_participant
                     WHERE session_id = $1 AND quitte_at IS NULL
                     ORDER BY rejoint_at ASC LIMIT 1",
                )
                .bind(id)
                .fetch_optional(pool.get_ref())
                .await?
            }
        } else {
            sqlx::query_scalar(
                "SELECT utilisateur_id FROM afrolang.session_participant
                 WHERE session_id = $1 AND quitte_at IS NULL
                 ORDER BY rejoint_at ASC LIMIT 1",
            )
            .bind(id)
            .fetch_optional(pool.get_ref())
            .await?
        };

        if let Some(nouveau_id) = successeur {
            sqlx::query(
                "UPDATE afrolang.session SET moderateur_id = $2, updated_at = NOW() WHERE id = $1",
            )
            .bind(id)
            .bind(nouveau_id)
            .execute(pool.get_ref())
            .await?;

            sqlx::query(
                "UPDATE afrolang.session_participant
                 SET role_session = 'moderateur'
                 WHERE session_id = $1 AND utilisateur_id = $2",
            )
            .bind(id)
            .bind(nouveau_id)
            .execute(pool.get_ref())
            .await?;

            let lien = format!("/afrolang/session/{}", id);
            notification::creer_notification(
                pool.get_ref(),
                nouveau_id,
                notification::afrolang::MODERATION_REPRISE,
                "Vous êtes désormais modérateur de cette session.",
                Some(&lien),
            )
            .await;
        }

        false
    } else {
        false
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "session_terminee": session_terminee
        })),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════════════
// Phase 3 — Token LiveKit pour visioconference
// ══════════════════════════════════════════════════════════════════════════

/// POST /api/afrolang/sessions/{id}/token — Generer un token LiveKit pour rejoindre la visio
pub async fn generer_token_session(
    pool: web::Data<PgPool>,
    livekit_config: web::Data<LivekitConfig>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<RejoindreRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let session_id = chemin.into_inner();

    // 1. Charger la session
    let session = sqlx::query_as::<_, SessionRow>(
        &format!(
            "SELECT {} FROM afrolang.session ses WHERE ses.id = $1",
            SESSION_COLONNES
        ),
    )
    .bind(session_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve(format!("Session {} non trouvee", session_id)))?;

    // 2. Verifier que la session est en_cours
    if session.etat != "en_cours" {
        return Err(ApiErreur::Validation("La session n'est pas en cours".into()));
    }

    // 3. Note (refonte 2026-04) : le contrôle du code secret passe par
    //    l'endpoint dédié `verifier-code` + jeton d'accès présenté à
    //    `demarrer-ou-rejoindre`. Plus de vérification ici.
    let _ = &body;

    // 4. Verifier max_participants
    let nb_actifs: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM afrolang.session_participant
         WHERE session_id = $1 AND quitte_at IS NULL",
    )
    .bind(session_id)
    .fetch_one(pool.get_ref())
    .await?;

    let max = session.max_participants.unwrap_or(50);
    if nb_actifs >= max as i64 {
        return Err(ApiErreur::Validation("Session complete".into()));
    }

    // 5. Charger les infos utilisateur
    let (user_nom, user_prenom): (String, Option<String>) = sqlx::query_as(
        "SELECT nom, prenom FROM iam.utilisateur WHERE id = $1",
    )
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    let room_name = format!("afrolang-{}", session_id);
    let is_moderator = session.moderateur_id == Some(utilisateur_id);
    let display_name = format!(
        "{} {}",
        user_prenom.as_deref().unwrap_or(""),
        user_nom
    ).trim().to_string();

    // 6. Generer le token LiveKit
    let token = livekit_api::access_token::AccessToken::with_api_key(
        &livekit_config.api_key,
        &livekit_config.api_secret,
    )
    .with_identity(&utilisateur_id.to_string())
    .with_name(&display_name)
    .with_grants(livekit_api::access_token::VideoGrants {
        room_join: true,
        room: room_name.clone(),
        can_publish: true,
        can_subscribe: true,
        can_publish_data: true,
        ..Default::default()
    })
    .to_jwt()
    .map_err(|e| ApiErreur::Validation(format!("Erreur generation token LiveKit: {}", e)))?;

    // 7. Enregistrer le participant (ON CONFLICT pour gerer les re-connexions)
    let role = if is_moderator { "moderateur" } else { "participant" };
    sqlx::query(
        "INSERT INTO afrolang.session_participant (session_id, utilisateur_id, role_session)
         VALUES ($1, $2, $3)
         ON CONFLICT (session_id, utilisateur_id)
         DO UPDATE SET quitte_at = NULL, rejoint_at = NOW()",
    )
    .bind(session_id)
    .bind(utilisateur_id)
    .bind(role)
    .execute(pool.get_ref())
    .await?;

    // Mettre a jour le pic de participants
    sqlx::query(
        "UPDATE afrolang.session SET nombre_participants_pic = GREATEST(nombre_participants_pic,
            (SELECT COUNT(*) FROM afrolang.session_participant
             WHERE session_id = $1 AND quitte_at IS NULL)),
            updated_at = NOW()
         WHERE id = $1",
    )
    .bind(session_id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Token LiveKit genere pour utilisateur {} session {}", utilisateur_id, session_id);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "token": token,
            "room_name": room_name,
            "livekit_url": livekit_config.url,
            "is_moderator": is_moderator,
        })),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════════════
// Phase 4 — Tableau blanc collaboratif
// ══════════════════════════════════════════════════════════════════════════

/// GET /api/afrolang/sessions/{id}/tableau-blanc — Obtenir le snapshot du tableau blanc
pub async fn obtenir_tableau_blanc(
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let session_id = chemin.into_inner();

    // Verifier que la session existe
    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM afrolang.session WHERE id = $1)",
    )
    .bind(session_id)
    .fetch_one(pool.get_ref())
    .await?;

    if !existe {
        return Err(ApiErreur::NonTrouve(format!("Session {} non trouvee", session_id)));
    }

    let row = sqlx::query_as::<_, (serde_json::Value, i32)>(
        "SELECT donnees, version FROM afrolang.tableau_blanc WHERE session_id = $1",
    )
    .bind(session_id)
    .fetch_optional(pool.get_ref())
    .await?;

    match row {
        Some((donnees, version)) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(serde_json::json!({ "donnees": donnees, "version": version })),
            error: None,
        })),
        None => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(serde_json::json!({ "donnees": {}, "version": 0 })),
            error: None,
        })),
    }
}

/// PUT /api/afrolang/sessions/{id}/tableau-blanc — Sauvegarder le snapshot
pub async fn sauvegarder_tableau_blanc(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<serde_json::Value>,
) -> Result<HttpResponse, ApiErreur> {
    let session_id = chemin.into_inner();
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Token invalide ou manquant".into()))?;

    // Verifier que la session existe et que l'utilisateur est moderateur
    let session = sqlx::query_as::<_, SessionRow>(&format!(
        "SELECT {} FROM afrolang.session ses WHERE ses.id = $1",
        SESSION_COLONNES
    ))
    .bind(session_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve(format!("Session {} non trouvee", session_id)))?;

    if session.moderateur_id != Some(utilisateur_id) {
        return Err(ApiErreur::NonAutorise(
            "Seul le moderateur peut sauvegarder le tableau blanc".into(),
        ));
    }

    // UPSERT dans afrolang.tableau_blanc
    sqlx::query(
        "INSERT INTO afrolang.tableau_blanc (session_id, donnees, version)
         VALUES ($1, $2, 1)
         ON CONFLICT (session_id)
         DO UPDATE SET donnees = $2, version = afrolang.tableau_blanc.version + 1, updated_at = NOW()",
    )
    .bind(session_id)
    .bind(&body.0)
    .execute(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some("ok"),
        error: None,
    }))
}

/// DELETE /api/afrolang/sessions/{id}/tableau-blanc — Effacer le tableau blanc
pub async fn effacer_tableau_blanc(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let session_id = chemin.into_inner();
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Token invalide ou manquant".into()))?;

    // Verifier que la session existe et que l'utilisateur est moderateur
    let session = sqlx::query_as::<_, SessionRow>(&format!(
        "SELECT {} FROM afrolang.session ses WHERE ses.id = $1",
        SESSION_COLONNES
    ))
    .bind(session_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve(format!("Session {} non trouvee", session_id)))?;

    if session.moderateur_id != Some(utilisateur_id) {
        return Err(ApiErreur::NonAutorise(
            "Seul le moderateur peut effacer le tableau blanc".into(),
        ));
    }

    sqlx::query(
        "UPDATE afrolang.tableau_blanc SET donnees = '{}', version = version + 1, updated_at = NOW() WHERE session_id = $1",
    )
    .bind(session_id)
    .execute(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some("ok"),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════════════
// 1.8 — Handlers utilitaires
// ══════════════════════════════════════════════════════════════════════════

/// GET /api/afrolang/stats — Statistiques globales Afrolang
pub async fn obtenir_stats(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let total_salles: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM afrolang.salle WHERE actif = true",
    )
    .fetch_one(pool.get_ref())
    .await?;

    let total_salles_privees: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM afrolang.salle_privee WHERE actif = true",
    )
    .fetch_one(pool.get_ref())
    .await?;

    let sessions_en_cours: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM afrolang.session WHERE etat = 'en_cours'",
    )
    .fetch_one(pool.get_ref())
    .await?;

    let sessions_terminees: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM afrolang.session WHERE etat = 'terminee'",
    )
    .fetch_one(pool.get_ref())
    .await?;

    let total_participants_uniques: i64 = sqlx::query_scalar(
        "SELECT COUNT(DISTINCT utilisateur_id) FROM afrolang.session_participant",
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(AfrolangStatsResponse {
            total_salles,
            total_salles_privees,
            sessions_en_cours,
            sessions_terminees,
            total_participants_uniques,
        }),
        error: None,
    }))
}

/// GET /api/afrolang/langues — Liste des langues disponibles
pub async fn lister_langues(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let langues: Vec<String> = sqlx::query_scalar(
        "SELECT DISTINCT langue_cible FROM afrolang.salle
         WHERE actif = true AND langue_cible IS NOT NULL
         ORDER BY langue_cible ASC",
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(langues),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════════════
// Legacy supprimé par la refonte 2026-04 (feature 001-afrolang-salles-refonte)
// ══════════════════════════════════════════════════════════════════════════
// Les handlers `creer_proposition`, `lister_mes_propositions`,
// `changer_visibilite_salle_privee`, `charger_salle_privee_active`,
// `demander_adhesion`, `inviter_membre`, `decision_adhesion`,
// `lister_adhesions_salle_privee`, `retirer_abonne` ont été retirés.
// La création de salles publiques est désormais réservée aux admins et le
// contrôle d'accès aux salles privées repose uniquement sur le code secret
// (voir endpoints `verifier-code`, `sessions/demarrer-ou-rejoindre`,
//  `code-acces`, `archiver` ajoutés en fin de fichier).

// ══════════════════════════════════════════════════════════════════════════
// Feature 005 — Transfert de modération de session (US3)
// ══════════════════════════════════════════════════════════════════════════

/// PUT /api/afrolang/sessions/{id}/moderation/transferer — Transfert manuel [JWT modérateur actuel]
pub async fn transferer_moderation_session(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<TransfererModerationRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let session_id = chemin.into_inner();
    let destinataire_id = body.destinataire_id;

    if destinataire_id == utilisateur_id {
        return Err(ApiErreur::Validation(
            "Le destinataire doit être différent de l'appelant".into(),
        ));
    }

    // Charger la session
    let session = sqlx::query_as::<_, SessionRow>(&format!(
        "SELECT {} FROM afrolang.session ses WHERE ses.id = $1",
        SESSION_COLONNES
    ))
    .bind(session_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve(format!("Session {} non trouvée", session_id)))?;

    if session.etat != "en_cours" {
        return Err(ApiErreur::Validation("La session n'est pas en cours".into()));
    }

    // Vérifier que l'appelant est le modérateur actif
    if session.moderateur_id != Some(utilisateur_id) {
        return Err(ApiErreur::NonAutorise(
            "Seul le modérateur actuel peut transférer la modération".into(),
        ));
    }

    // Vérifier que le destinataire est participant actif
    let destinataire_actif: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM afrolang.session_participant
            WHERE session_id = $1 AND utilisateur_id = $2 AND quitte_at IS NULL
        )",
    )
    .bind(session_id)
    .bind(destinataire_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !destinataire_actif {
        return Err(ApiErreur::Validation(
            "Le destinataire n'est pas un participant actif de la session".into(),
        ));
    }

    // Transfert
    sqlx::query(
        "UPDATE afrolang.session
         SET moderateur_id = $2, updated_at = NOW()
         WHERE id = $1",
    )
    .bind(session_id)
    .bind(destinataire_id)
    .execute(pool.get_ref())
    .await?;

    // Mettre à jour les rôles côté participants
    sqlx::query(
        "UPDATE afrolang.session_participant
         SET role_session = CASE
            WHEN utilisateur_id = $2 THEN 'moderateur'
            WHEN utilisateur_id = $3 THEN 'participant'
            ELSE role_session
         END
         WHERE session_id = $1 AND utilisateur_id IN ($2, $3)",
    )
    .bind(session_id)
    .bind(destinataire_id)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?;

    // Notifications aux deux parties
    let salle_privee_id = session.salle_privee_id;
    let lien = format!("/afrolang/session/{}", session_id);
    notification::creer_notification(
        pool.get_ref(),
        destinataire_id,
        notification::afrolang::MODERATION_REPRISE,
        "Vous êtes désormais modérateur de cette session.",
        Some(&lien),
    )
    .await;
    notification::creer_notification(
        pool.get_ref(),
        utilisateur_id,
        notification::afrolang::MODERATION_REPRISE,
        "Vous avez transféré la modération de cette session.",
        Some(&lien),
    )
    .await;

    log::info!(
        "Session {} ({:?}) : modération transférée {} → {}",
        session_id,
        salle_privee_id,
        utilisateur_id,
        destinataire_id
    );

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "session_id": session_id,
            "moderateur_id": destinataire_id,
        })),
        error: None,
    }))
}

/// PATCH /api/afrolang/salles-privees/{id}/max-participants — Modifier la
/// limite de participants d'une salle privée (auteur uniquement).
pub async fn modifier_max_participants_salle_privee(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<ModifierMaxParticipantsRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let id = chemin.into_inner();
    let nouvelle = body.max_participants;
    if nouvelle < 1 {
        return Err(ApiErreur::Validation(
            "La limite de participants doit être supérieure ou égale à 1".into(),
        ));
    }

    let salle: Option<(Uuid, Option<i32>, Option<chrono::DateTime<chrono::Utc>>)> = sqlx::query_as(
        "SELECT cree_par, max_participants, archivee_at
         FROM afrolang.salle_privee
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?;
    let (createur, ancienne, archivee_at) = salle
        .ok_or_else(|| ApiErreur::NonTrouve(format!("Salle privée {} non trouvée", id)))?;

    if createur != utilisateur_id {
        return Err(ApiErreur::AccesInterdit(
            "Seul le créateur peut modifier la limite".into(),
        ));
    }
    if archivee_at.is_some() {
        return Err(ApiErreur::Validation("La salle privée est archivée".into()));
    }

    sqlx::query(
        "UPDATE afrolang.salle_privee
         SET max_participants = $2, updated_at = NOW()
         WHERE id = $1",
    )
    .bind(id)
    .bind(nouvelle)
    .execute(pool.get_ref())
    .await?;

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "modifier_max_participants_salle_privee",
        "afrolang",
        "salle_privee",
        Some(id),
        Some(serde_json::json!({ "max_participants": ancienne })),
        Some(serde_json::json!({ "max_participants": nouvelle })),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id, "max_participants": nouvelle })),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════════════
// Feature 005 — US6 : Messagerie de session
// ══════════════════════════════════════════════════════════════════════════

/// Vérifie que l'utilisateur est participant actif d'une session
async fn verifier_participant_actif(
    pool: &PgPool,
    session_id: Uuid,
    utilisateur_id: Uuid,
) -> Result<(), ApiErreur> {
    let actif: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM afrolang.session_participant
            WHERE session_id = $1 AND utilisateur_id = $2 AND quitte_at IS NULL
        )",
    )
    .bind(session_id)
    .bind(utilisateur_id)
    .fetch_one(pool)
    .await?;
    if !actif {
        return Err(ApiErreur::NonAutorise(
            "Vous devez être participant actif de la session".into(),
        ));
    }
    Ok(())
}

/// GET /api/afrolang/sessions/{id}/messages — Historique [JWT participant]
pub async fn lister_messages_session(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    params: web::Query<MessagesFiltres>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let session_id = chemin.into_inner();
    verifier_participant_actif(pool.get_ref(), session_id, utilisateur_id).await?;

    let limit = params.limit.unwrap_or(100).clamp(1, 500);

    let sql = format!(
        "SELECT {},
            u.nom AS auteur_nom,
            u.prenom AS auteur_prenom,
            u.photo_url AS auteur_photo
         FROM afrolang.message_session ms
         LEFT JOIN iam.utilisateur u ON u.id = ms.auteur_id
         WHERE ms.session_id = $1 AND ms.deleted_at IS NULL
           AND ($2::timestamptz IS NULL OR ms.created_at > $2)
         ORDER BY ms.created_at ASC
         LIMIT $3",
        MESSAGE_SESSION_COLONNES
    );

    let rows = sqlx::query_as::<_, MessageSessionRow>(&sql)
        .bind(session_id)
        .bind(params.since)
        .bind(limit)
        .fetch_all(pool.get_ref())
        .await?;

    let items: Vec<MessageSessionResponse> = rows.iter().map(|r| r.to_response()).collect();
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(items),
        error: None,
    }))
}

/// POST /api/afrolang/sessions/{id}/messages — Envoyer un message [JWT participant]
pub async fn envoyer_message_session(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<CreerMessageRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let session_id = chemin.into_inner();

    let contenu = body.contenu.trim();
    if contenu.is_empty() || contenu.chars().count() > 4000 {
        return Err(ApiErreur::Validation(
            "Le contenu doit faire entre 1 et 4000 caractères".into(),
        ));
    }

    verifier_participant_actif(pool.get_ref(), session_id, utilisateur_id).await?;

    let message_id: Uuid = sqlx::query_scalar(
        "INSERT INTO afrolang.message_session (session_id, auteur_id, contenu)
         VALUES ($1, $2, $3)
         RETURNING id",
    )
    .bind(session_id)
    .bind(utilisateur_id)
    .bind(contenu)
    .fetch_one(pool.get_ref())
    .await?;

    let sql = format!(
        "SELECT {},
            u.nom AS auteur_nom,
            u.prenom AS auteur_prenom,
            u.photo_url AS auteur_photo
         FROM afrolang.message_session ms
         LEFT JOIN iam.utilisateur u ON u.id = ms.auteur_id
         WHERE ms.id = $1",
        MESSAGE_SESSION_COLONNES
    );
    let row = sqlx::query_as::<_, MessageSessionRow>(&sql)
        .bind(message_id)
        .fetch_one(pool.get_ref())
        .await?;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════════════
// Feature 005 — US6 : Ressources de salle publique
// ══════════════════════════════════════════════════════════════════════════

const RESSOURCES_EXTENSIONS_AUTORISEES: &[&str] = &[
    "pdf", "png", "jpg", "jpeg", "mp3", "mp4", "webm", "ogg", "wav",
];
const RESSOURCE_TAILLE_MAX: usize = 50 * 1024 * 1024;

/// Vérifie si l'utilisateur est modérateur attitré actif d'une salle
async fn est_moderateur_attitre(
    pool: &PgPool,
    salle_id: Uuid,
    utilisateur_id: Uuid,
) -> Result<bool, ApiErreur> {
    let v: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM afrolang.salle_moderateur
            WHERE salle_id = $1 AND utilisateur_id = $2 AND actif = TRUE
        )",
    )
    .bind(salle_id)
    .bind(utilisateur_id)
    .fetch_one(pool)
    .await?;
    Ok(v)
}

/// GET /api/afrolang/salles/{salle_id}/ressources — Liste publique
pub async fn lister_ressources(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let salle_id = chemin.into_inner();
    let utilisateur_id = extraire_utilisateur_id(&req);

    let sql = format!(
        "SELECT {},
            u.nom AS auteur_nom,
            u.prenom AS auteur_prenom
         FROM afrolang.ressource_salle rs
         LEFT JOIN iam.utilisateur u ON u.id = rs.ajoute_par
         WHERE rs.salle_id = $1 AND rs.deleted_at IS NULL
           AND (
             rs.etat = 'publiee'
             OR ($2::uuid IS NOT NULL AND rs.ajoute_par = $2 AND rs.etat = 'en_attente_validation')
           )
         ORDER BY rs.created_at DESC",
        RESSOURCE_SALLE_COLONNES
    );

    let rows = sqlx::query_as::<_, RessourceSalleRow>(&sql)
        .bind(salle_id)
        .bind(utilisateur_id)
        .fetch_all(pool.get_ref())
        .await?;

    let items: Vec<RessourceSalleResponse> = rows.iter().map(|r| r.to_response()).collect();
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(items),
        error: None,
    }))
}

/// POST /api/afrolang/salles/{salle_id}/ressources/fichier — Upload fichier
/// [JWT modérateur attitré ou admin]
pub async fn uploader_ressource_fichier(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    mut payload: Multipart,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let salle_id = chemin.into_inner();

    let salle_active: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM afrolang.salle
                       WHERE id = $1 AND actif = TRUE AND deleted_at IS NULL)",
    )
    .bind(salle_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !salle_active {
        return Err(ApiErreur::NonTrouve("Salle publique introuvable".into()));
    }

    let admin = verifier_admin(pool.get_ref(), utilisateur_id).await?;
    let moderateur = est_moderateur_attitre(pool.get_ref(), salle_id, utilisateur_id).await?;
    if !admin && !moderateur {
        return Err(ApiErreur::NonAutorise(
            "Seul un modérateur attitré ou admin peut publier un fichier".into(),
        ));
    }

    let mut titre: Option<String> = None;
    let mut description: Option<String> = None;
    let mut fichier_url: Option<String> = None;

    while let Some(field_res) = payload.next().await {
        let mut field =
            field_res.map_err(|e| ApiErreur::Upload(format!("Erreur multipart : {}", e)))?;
        let nom = field
            .content_disposition()
            .and_then(|cd| cd.get_name())
            .unwrap_or("")
            .to_string();

        match nom.as_str() {
            "titre" => titre = Some(lire_champ_texte(&mut field).await?.trim().to_string()),
            "description" => {
                description = Some(lire_champ_texte(&mut field).await?.trim().to_string())
            }
            "fichier" => {
                let filename_original = field
                    .content_disposition()
                    .and_then(|cd| cd.get_filename().map(|s| s.to_string()))
                    .ok_or_else(|| ApiErreur::Upload("Nom de fichier manquant".into()))?;
                let sanitized = sanitize_filename::sanitize(&filename_original);
                let ext = std::path::Path::new(&sanitized)
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|s| s.to_lowercase())
                    .unwrap_or_default();
                if !RESSOURCES_EXTENSIONS_AUTORISEES.contains(&ext.as_str()) {
                    return Err(ApiErreur::Validation(format!(
                        "Extension '{}' non autorisée",
                        ext
                    )));
                }
                let id = Uuid::new_v4();
                let rel = format!("uploads/afrolang/ressources/{}-{}", id, sanitized);
                let abs = format!("./{}", rel);
                // Sauvegarde avec contrôle taille
                if let Some(parent) = std::path::Path::new(&abs).parent() {
                    std::fs::create_dir_all(parent).map_err(|e| {
                        ApiErreur::Upload(format!("Création répertoire : {}", e))
                    })?;
                }
                let mut out = std::fs::File::create(&abs)
                    .map_err(|e| ApiErreur::Upload(format!("Création fichier : {}", e)))?;
                let mut total: usize = 0;
                while let Some(chunk) = field.next().await {
                    let data = chunk.map_err(|e| {
                        ApiErreur::Upload(format!("Lecture fichier : {}", e))
                    })?;
                    total += data.len();
                    if total > RESSOURCE_TAILLE_MAX {
                        let _ = std::fs::remove_file(&abs);
                        return Err(ApiErreur::Validation(
                            "Fichier trop volumineux (max 50 Mo)".into(),
                        ));
                    }
                    out.write_all(&data).map_err(|e| {
                        ApiErreur::Upload(format!("Écriture fichier : {}", e))
                    })?;
                }
                fichier_url = Some(format!("/{}", rel));
            }
            _ => { let _ = lire_champ_texte(&mut field).await; }
        }
    }

    let titre = titre
        .filter(|t| !t.is_empty())
        .ok_or_else(|| ApiErreur::Validation("Le titre est obligatoire".into()))?;
    let fichier_url = fichier_url
        .ok_or_else(|| ApiErreur::Validation("Aucun fichier fourni".into()))?;

    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO afrolang.ressource_salle
            (salle_id, titre, description, type, fichier_url, etat, ajoute_par,
             valide_par, valide_at)
         VALUES ($1, $2, $3, 'fichier'::afrolang.type_ressource, $4,
                 'publiee'::afrolang.etat_ressource, $5, $5, NOW())
         RETURNING id",
    )
    .bind(salle_id)
    .bind(&titre)
    .bind(description.as_deref())
    .bind(&fichier_url)
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "afrolang.ressource.fichier_publie",
        "afrolang",
        "ressource_salle",
        Some(id),
        None,
        Some(serde_json::json!({
            "salle_id": salle_id, "titre": titre, "fichier_url": fichier_url,
        })),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "id": id,
            "fichier_url": fichier_url,
            "etat": "publiee",
        })),
        error: None,
    }))
}

/// POST /api/afrolang/salles/{salle_id}/ressources/lien — Soumettre un lien [JWT]
pub async fn soumettre_lien_externe(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<CreerRessourceLienRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let salle_id = chemin.into_inner();

    let titre = body.titre.trim();
    if titre.is_empty() {
        return Err(ApiErreur::Validation("Le titre est obligatoire".into()));
    }
    let lien = body.lien_url.trim();
    if !(lien.starts_with("http://") || lien.starts_with("https://")) {
        return Err(ApiErreur::Validation(
            "L'URL doit commencer par http:// ou https://".into(),
        ));
    }
    if lien.len() > 1000 || lien.chars().any(|c| c.is_control()) {
        return Err(ApiErreur::Validation(
            "URL invalide (longueur ou caractères non autorisés)".into(),
        ));
    }

    let salle_active: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM afrolang.salle
                       WHERE id = $1 AND actif = TRUE AND deleted_at IS NULL)",
    )
    .bind(salle_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !salle_active {
        return Err(ApiErreur::NonTrouve("Salle publique introuvable".into()));
    }

    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO afrolang.ressource_salle
            (salle_id, titre, description, type, lien_url, etat, ajoute_par)
         VALUES ($1, $2, $3, 'lien_externe'::afrolang.type_ressource, $4,
                 'en_attente_validation'::afrolang.etat_ressource, $5)
         RETURNING id",
    )
    .bind(salle_id)
    .bind(titre)
    .bind(body.description.as_deref())
    .bind(lien)
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "id": id,
            "etat": "en_attente_validation",
        })),
        error: None,
    }))
}

/// DELETE /api/afrolang/ressources/{id} — Suppression (auteur, modérateur, admin) [JWT]
pub async fn supprimer_ressource(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let id = chemin.into_inner();

    let info: Option<(Uuid, Uuid)> = sqlx::query_as(
        "SELECT salle_id, ajoute_par FROM afrolang.ressource_salle
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?;
    let (salle_id, ajoute_par) =
        info.ok_or_else(|| ApiErreur::NonTrouve("Ressource introuvable".into()))?;

    let admin = verifier_admin(pool.get_ref(), utilisateur_id).await?;
    let moderateur = est_moderateur_attitre(pool.get_ref(), salle_id, utilisateur_id).await?;
    if ajoute_par != utilisateur_id && !admin && !moderateur {
        return Err(ApiErreur::NonAutorise(
            "Vous n'êtes pas autorisé à supprimer cette ressource".into(),
        ));
    }

    sqlx::query(
        "UPDATE afrolang.ressource_salle
         SET deleted_at = NOW(), updated_at = NOW() WHERE id = $1",
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "afrolang.ressource.suppression",
        "afrolang",
        "ressource_salle",
        Some(id),
        None,
        None,
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await;

    Ok(HttpResponse::NoContent().finish())
}

// ══════════════════════════════════════════════════════════════════════════
// Refonte 2026-04 — Salles privées : code secret, rate limit, jeton d'accès
// ══════════════════════════════════════════════════════════════════════════

/// Durée de vie d'un jeton d'accès salle privée (4 heures).
const ACCES_JETON_TTL_SECONDES: i64 = 4 * 60 * 60;

/// Nom du header HTTP portant le jeton d'accès salle privée
const HEADER_ACCES_JETON: &str = "X-Afrolang-Acces-Jeton";

/// Valide le format du code d'accès saisi par un utilisateur.
///
/// Règle (R2) : 4 à 16 caractères, alphanumérique + symboles courants
/// `!@#$%&*?-`. Les espaces, unicode étendu et autres symboles sont refusés
/// pour éviter les confusions orales / saisie mobile.
pub fn valider_format_code_acces(code: &str) -> Result<(), ApiErreur> {
    let long = code.chars().count();
    if !(4..=16).contains(&long) {
        return Err(ApiErreur::Validation(
            "Le code d'accès doit contenir entre 4 et 16 caractères".into(),
        ));
    }
    let charset_ok = code.chars().all(|c| {
        c.is_ascii_alphanumeric()
            || matches!(c, '!' | '@' | '#' | '$' | '%' | '&' | '*' | '?' | '-')
    });
    if !charset_ok {
        return Err(ApiErreur::Validation(
            "Le code d'accès ne peut contenir que des caractères alphanumériques ou les symboles !@#$%&*?-"
                .into(),
        ));
    }
    Ok(())
}

/// Calcule le hash bcrypt (cost 10) d'un code d'accès en clair.
///
/// Cost 10 choisi (vs 12 pour les mots de passe) : le code est à faible
/// entropie ; la protection principale repose sur le rate limit (R4).
pub fn hasher_code_acces(code: &str) -> Result<String, ApiErreur> {
    bcrypt::hash(code, 10).map_err(|e| {
        ApiErreur::BaseDeDonnees(format!("Erreur hashage code accès : {}", e))
    })
}

/// Vérifie un code en clair contre son hash bcrypt.
pub fn verifier_code_acces_plain(code: &str, hash: &str) -> Result<bool, ApiErreur> {
    bcrypt::verify(code, hash).map_err(|e| {
        ApiErreur::BaseDeDonnees(format!("Erreur vérification code accès : {}", e))
    })
}

/// Charge la ligne complète d'une salle privée (y compris `code_acces_hash`)
/// pour vérification serveur. Filtre les salles supprimées.
async fn charger_salle_privee_interne(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<SallePriveeRow>, ApiErreur> {
    let sql = format!(
        "SELECT {},
            u.nom AS createur_nom, u.prenom AS createur_prenom,
            u.photo_url AS createur_photo,
            s.titre AS salle_titre, s.langue_cible AS salle_langue,
            EXISTS(SELECT 1 FROM afrolang.session ses
                   WHERE ses.salle_privee_id = sp.id AND ses.etat = 'en_cours') AS session_en_cours
         FROM afrolang.salle_privee sp
         LEFT JOIN iam.utilisateur u ON u.id = sp.cree_par
         LEFT JOIN afrolang.salle s ON s.id = sp.salle_id
         WHERE sp.id = $1 AND sp.deleted_at IS NULL",
        SALLE_PRIVEE_COLONNES
    );
    let row = sqlx::query_as::<_, SallePriveeRow>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(row)
}

/// POST /api/afrolang/salles-privees/{id}/verifier-code
///
/// Vérifie le code d'accès saisi. Cas particuliers :
///  1. Utilisateur == auteur → jeton remis sans vérification (FR-014).
///  2. Rate limit (5 échecs / 1 min, verrou 5 min).
///  3. Salle archivée ou supprimée → 404 (message générique, ne rien fuiter).
///
/// Audit : `verifier_code_salle_privee_echec` sur échec uniquement (les
/// succès sont loggés implicitement lors du démarrage de session).
pub async fn verifier_code_acces_salle_privee(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<VerifierCodeAccesRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let salle_privee_id = chemin.into_inner();

    // Charger la salle (sans vérifier le code) pour détecter 404/archivée.
    let salle = charger_salle_privee_interne(pool.get_ref(), salle_privee_id).await?;
    let Some(salle) = salle else {
        return Err(ApiErreur::NonTrouve("Salle privée inexistante".into()));
    };
    if salle.archivee_at.is_some() {
        // Même message générique que « inexistante » pour ne rien fuiter.
        return Err(ApiErreur::NonTrouve("Salle privée inexistante".into()));
    }

    // Auteur : court-circuit (FR-014) — pas de vérification, pas d'audit.
    if salle.cree_par == utilisateur_id {
        let (jeton, expires_at) = jwt::creer_acces_jeton_salle_privee(
            salle_privee_id,
            utilisateur_id,
            ACCES_JETON_TTL_SECONDES,
        )?;
        return Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(VerifierCodeAccesResponse {
                salle_privee_id,
                acces_jeton: jeton,
                expires_at,
            }),
            error: None,
        }));
    }

    // Rate limit avant toute vérification du hash.
    if afrolang_rate_limit::est_verrouillee(pool.get_ref(), salle_privee_id, utilisateur_id)
        .await?
    {
        return Err(ApiErreur::LimiteAtteinte(
            "Trop de tentatives, réessayez dans quelques minutes".into(),
        ));
    }

    let code = body.code_acces.as_str();
    let succes = verifier_code_acces_plain(code, &salle.code_acces_hash)?;

    // Enregistrer la tentative (succès ou échec) pour le rate limit.
    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    afrolang_rate_limit::enregistrer_tentative(
        pool.get_ref(),
        salle_privee_id,
        utilisateur_id,
        succes,
        ip.as_deref(),
        ua.as_deref(),
    )
    .await?;

    if !succes {
        // Audit échec uniquement — jamais le code en clair.
        audit::log_action(
            pool.get_ref(),
            Some(utilisateur_id),
            "verifier_code_salle_privee_echec",
            "afrolang",
            "salle_privee",
            Some(salle_privee_id),
            None,
            None,
            ip.as_deref(),
            ua.as_deref(),
        )
        .await;
        return Err(ApiErreur::AccesInterdit("Code incorrect".into()));
    }

    let (jeton, expires_at) = jwt::creer_acces_jeton_salle_privee(
        salle_privee_id,
        utilisateur_id,
        ACCES_JETON_TTL_SECONDES,
    )?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(VerifierCodeAccesResponse {
            salle_privee_id,
            acces_jeton: jeton,
            expires_at,
        }),
        error: None,
    }))
}

/// POST /api/afrolang/salles-privees/{id}/sessions/demarrer-ou-rejoindre
///
/// Démarre une nouvelle session si aucune n'est en cours, ou rejoint la
/// session `en_cours` existante. Requiert un jeton d'accès valide (obtenu
/// via `verifier-code`) dans le header `X-Afrolang-Acces-Jeton`.
pub async fn demarrer_ou_rejoindre_session_salle_privee(
    pool: web::Data<PgPool>,
    livekit_config: web::Data<LivekitConfig>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let salle_privee_id = chemin.into_inner();

    // Récupérer et valider le jeton d'accès.
    let jeton_header = req
        .headers()
        .get(HEADER_ACCES_JETON)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .ok_or_else(|| ApiErreur::NonAutorise(
            format!("Header {} manquant", HEADER_ACCES_JETON)
        ))?;
    jwt::valider_acces_jeton_salle_privee(&jeton_header, salle_privee_id, utilisateur_id)?;

    // Charger la salle pour vérifier son état (410 si archivée).
    let salle = charger_salle_privee_interne(pool.get_ref(), salle_privee_id).await?
        .ok_or_else(|| ApiErreur::NonTrouve("Salle privée inexistante".into()))?;
    if salle.archivee_at.is_some() {
        return Ok(HttpResponse::Gone().json(ApiResponse::<()> {
            success: false,
            data: None,
            error: Some("Salle privée archivée".into()),
        }));
    }

    let moderateur_id = salle.cree_par;

    // Transaction pour garantir au plus UNE session en_cours par salle privée.
    let mut tx = pool.begin().await?;

    let session_existante: Option<Uuid> = sqlx::query_scalar(
        "SELECT id FROM afrolang.session
         WHERE salle_privee_id = $1 AND etat = 'en_cours'
         FOR UPDATE",
    )
    .bind(salle_privee_id)
    .fetch_optional(&mut *tx)
    .await?;

    let session_id = match session_existante {
        Some(id) => id,
        None => {
            // Créer une nouvelle session en_cours ; moderateur = auteur de la
            // salle privée, cree_par = utilisateur courant (peut différer).
            sqlx::query_scalar(
                "INSERT INTO afrolang.session
                    (salle_privee_id, etat, moderateur_id, demarre_at,
                     max_participants, tableau_blanc_actif, cree_par)
                 VALUES ($1, 'en_cours', $2, NOW(), $3, TRUE, $4)
                 RETURNING id",
            )
            .bind(salle_privee_id)
            .bind(moderateur_id)
            .bind(salle.max_participants.unwrap_or(50))
            .bind(utilisateur_id)
            .fetch_one(&mut *tx)
            .await?
        }
    };

    // INSERT du participant (idempotent — reconnexion possible).
    let role = if utilisateur_id == moderateur_id {
        "moderateur"
    } else {
        "participant"
    };
    sqlx::query(
        "INSERT INTO afrolang.session_participant (session_id, utilisateur_id, role_session)
         VALUES ($1, $2, $3)
         ON CONFLICT (session_id, utilisateur_id)
         DO UPDATE SET quitte_at = NULL, rejoint_at = NOW()",
    )
    .bind(session_id)
    .bind(utilisateur_id)
    .bind(role)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    // Mettre à jour le pic de participants (hors transaction).
    sqlx::query(
        "UPDATE afrolang.session
         SET nombre_participants_pic = GREATEST(
                nombre_participants_pic,
                (SELECT COUNT(*) FROM afrolang.session_participant
                 WHERE session_id = $1 AND quitte_at IS NULL)
             ), updated_at = NOW()
         WHERE id = $1",
    )
    .bind(session_id)
    .execute(pool.get_ref())
    .await?;

    // Générer le token LiveKit.
    let (user_nom, user_prenom): (String, Option<String>) = sqlx::query_as(
        "SELECT nom, prenom FROM iam.utilisateur WHERE id = $1",
    )
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;
    let display_name = format!(
        "{} {}",
        user_prenom.as_deref().unwrap_or(""),
        user_nom
    )
    .trim()
    .to_string();
    let room_name = format!("afrolang-{}", session_id);

    let livekit_token = livekit_api::access_token::AccessToken::with_api_key(
        &livekit_config.api_key,
        &livekit_config.api_secret,
    )
    .with_identity(&utilisateur_id.to_string())
    .with_name(&display_name)
    .with_grants(livekit_api::access_token::VideoGrants {
        room_join: true,
        room: room_name.clone(),
        can_publish: true,
        can_subscribe: true,
        can_publish_data: true,
        ..Default::default()
    })
    .to_jwt()
    .map_err(|e| ApiErreur::Validation(format!("Erreur génération token LiveKit : {}", e)))?;

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "rejoindre_session_salle_privee",
        "afrolang",
        "session",
        Some(session_id),
        None,
        Some(serde_json::json!({
            "salle_privee_id": salle_privee_id,
            "role": role,
        })),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(DemarrerRejoindreResponse {
            session_id,
            livekit_url: livekit_config.url.clone(),
            livekit_token,
            moderateur_id: Some(moderateur_id),
        }),
        error: None,
    }))
}

/// POST /api/afrolang/salles/{salle_id}/sessions/demarrer-ou-rejoindre
///
/// US1 — Refonte 2026-04. Démarre une nouvelle session live si aucune
/// n'est en cours dans la salle publique, sinon rejoint la session
/// existante. Ouvert à n'importe quel utilisateur connecté (FR-005b) :
/// le premier arrivé devient modérateur de session ; si un modérateur
/// attitré arrive ensuite, `rejoindre_session` (endpoint compat)
/// gère la reprise automatique côté legacy — ici on se limite au
/// démarrage/jointure pour tenir SC-001 (≤ 3 s).
pub async fn demarrer_ou_rejoindre_session_salle_publique(
    pool: web::Data<PgPool>,
    livekit_config: web::Data<LivekitConfig>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let salle_id = chemin.into_inner();

    // Vérifier que la salle publique existe et est active.
    let salle_active: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM afrolang.salle
                       WHERE id = $1 AND actif = TRUE AND deleted_at IS NULL)",
    )
    .bind(salle_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !salle_active {
        return Err(ApiErreur::NonTrouve("Salle publique introuvable".into()));
    }

    // Transaction : garantir au plus UNE session en_cours par salle publique.
    let mut tx = pool.begin().await?;

    let session_existante: Option<(Uuid, Option<Uuid>)> = sqlx::query_as(
        "SELECT id, moderateur_id FROM afrolang.session
         WHERE salle_id = $1 AND etat = 'en_cours'
         FOR UPDATE",
    )
    .bind(salle_id)
    .fetch_optional(&mut *tx)
    .await?;

    let (session_id, moderateur_effectif_id, est_nouveau) = match session_existante {
        Some((id, m)) => (id, m, false),
        None => {
            // Aucun live en cours : créer et démarrer immédiatement. Le
            // créateur = modérateur initial (FR-005b).
            let nouvelle_id: Uuid = sqlx::query_scalar(
                "INSERT INTO afrolang.session
                    (salle_id, etat, moderateur_id, demarre_at,
                     max_participants, tableau_blanc_actif, cree_par)
                 VALUES ($1, 'en_cours', $2, NOW(), $3, TRUE, $2)
                 RETURNING id",
            )
            .bind(salle_id)
            .bind(utilisateur_id)
            .bind(50_i32)
            .fetch_one(&mut *tx)
            .await?;
            (nouvelle_id, Some(utilisateur_id), true)
        }
    };

    // INSERT participant (idempotent).
    let role = if moderateur_effectif_id == Some(utilisateur_id) {
        "moderateur"
    } else {
        "participant"
    };
    sqlx::query(
        "INSERT INTO afrolang.session_participant (session_id, utilisateur_id, role_session)
         VALUES ($1, $2, $3)
         ON CONFLICT (session_id, utilisateur_id)
         DO UPDATE SET quitte_at = NULL, rejoint_at = NOW()",
    )
    .bind(session_id)
    .bind(utilisateur_id)
    .bind(role)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    // Pic de participants (hors transaction).
    sqlx::query(
        "UPDATE afrolang.session
         SET nombre_participants_pic = GREATEST(
                nombre_participants_pic,
                (SELECT COUNT(*) FROM afrolang.session_participant
                 WHERE session_id = $1 AND quitte_at IS NULL)
             ), updated_at = NOW()
         WHERE id = $1",
    )
    .bind(session_id)
    .execute(pool.get_ref())
    .await?;

    // Générer le token LiveKit.
    let (user_nom, user_prenom): (String, Option<String>) = sqlx::query_as(
        "SELECT nom, prenom FROM iam.utilisateur WHERE id = $1",
    )
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;
    let display_name = format!(
        "{} {}",
        user_prenom.as_deref().unwrap_or(""),
        user_nom
    )
    .trim()
    .to_string();
    let room_name = format!("afrolang-{}", session_id);

    let livekit_token = livekit_api::access_token::AccessToken::with_api_key(
        &livekit_config.api_key,
        &livekit_config.api_secret,
    )
    .with_identity(&utilisateur_id.to_string())
    .with_name(&display_name)
    .with_grants(livekit_api::access_token::VideoGrants {
        room_join: true,
        room: room_name.clone(),
        can_publish: true,
        can_subscribe: true,
        can_publish_data: true,
        ..Default::default()
    })
    .to_jwt()
    .map_err(|e| ApiErreur::Validation(format!("Erreur génération token LiveKit : {}", e)))?;

    let action = if est_nouveau {
        "demarrer_session_salle_publique"
    } else {
        "rejoindre_session_salle_publique"
    };
    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        action,
        "afrolang",
        "session",
        Some(session_id),
        None,
        Some(serde_json::json!({
            "salle_id": salle_id,
            "role": role,
        })),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(DemarrerRejoindreResponse {
            session_id,
            livekit_url: livekit_config.url.clone(),
            livekit_token,
            moderateur_id: moderateur_effectif_id,
        }),
        error: None,
    }))
}

/// PATCH /api/afrolang/salles-privees/{id}/code-acces
///
/// Met à jour le code d'accès (auteur uniquement). Hash before/after
/// tracé dans l'audit — jamais les plaintexts.
pub async fn modifier_code_acces_salle_privee(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<ModifierCodeAccesRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let salle_privee_id = chemin.into_inner();

    valider_format_code_acces(body.nouveau_code_acces.as_str())?;

    let actuel: Option<(Uuid, String, Option<chrono::DateTime<chrono::Utc>>)> = sqlx::query_as(
        "SELECT cree_par, code_acces_hash, archivee_at
         FROM afrolang.salle_privee
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(salle_privee_id)
    .fetch_optional(pool.get_ref())
    .await?;
    let (createur, ancien_hash, archivee_at) = actuel
        .ok_or_else(|| ApiErreur::NonTrouve("Salle privée inexistante".into()))?;

    if createur != utilisateur_id {
        return Err(ApiErreur::AccesInterdit(
            "Seul le créateur peut modifier le code d'accès".into(),
        ));
    }
    if archivee_at.is_some() {
        return Err(ApiErreur::Validation("La salle privée est archivée".into()));
    }

    let nouveau_hash = hasher_code_acces(body.nouveau_code_acces.as_str())?;

    sqlx::query(
        "UPDATE afrolang.salle_privee
         SET code_acces_hash = $2, updated_at = NOW()
         WHERE id = $1",
    )
    .bind(salle_privee_id)
    .bind(&nouveau_hash)
    .execute(pool.get_ref())
    .await?;

    // Audit : on ne trace JAMAIS le plaintext, seulement les hashes
    // (pour permettre la reconstitution d'historique sans fuite).
    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "modifier_code_salle_privee",
        "afrolang",
        "salle_privee",
        Some(salle_privee_id),
        Some(serde_json::json!({ "code_acces_hash": ancien_hash })),
        Some(serde_json::json!({ "code_acces_hash": nouveau_hash })),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await;

    Ok(HttpResponse::NoContent().finish())
}

/// POST /api/afrolang/salles-privees/{id}/archiver
///
/// Archive la salle privée (auteur uniquement). Si une session est en cours,
/// elle est terminée. L'archivage libère le verrou d'unicité
/// (salle_id, cree_par) → l'utilisateur peut en recréer une.
pub async fn archiver_salle_privee_par_auteur(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let salle_privee_id = chemin.into_inner();

    let mut tx = pool.begin().await?;

    let salle: Option<(Uuid, Option<chrono::DateTime<chrono::Utc>>)> = sqlx::query_as(
        "SELECT cree_par, archivee_at
         FROM afrolang.salle_privee
         WHERE id = $1 AND deleted_at IS NULL
         FOR UPDATE",
    )
    .bind(salle_privee_id)
    .fetch_optional(&mut *tx)
    .await?;
    let (createur, archivee_at) = salle
        .ok_or_else(|| ApiErreur::NonTrouve("Salle privée inexistante".into()))?;

    if createur != utilisateur_id {
        return Err(ApiErreur::AccesInterdit(
            "Seul le créateur peut archiver cette salle".into(),
        ));
    }
    if archivee_at.is_some() {
        return Err(ApiErreur::Validation("Salle déjà archivée".into()));
    }

    // Archiver la salle.
    sqlx::query(
        "UPDATE afrolang.salle_privee
         SET archivee_at = NOW(), updated_at = NOW()
         WHERE id = $1",
    )
    .bind(salle_privee_id)
    .execute(&mut *tx)
    .await?;

    // Terminer la session en cours s'il y en a une.
    sqlx::query(
        "UPDATE afrolang.session
         SET etat = 'terminee', termine_at = NOW(),
             duree_secondes = EXTRACT(EPOCH FROM (NOW() - COALESCE(demarre_at, created_at)))::INT,
             updated_at = NOW()
         WHERE salle_privee_id = $1 AND etat = 'en_cours'",
    )
    .bind(salle_privee_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        "archiver_salle_privee",
        "afrolang",
        "salle_privee",
        Some(salle_privee_id),
        Some(serde_json::json!({ "archivee_at": null })),
        Some(serde_json::json!({ "archivee_at": "NOW()" })),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await;

    Ok(HttpResponse::NoContent().finish())
}
