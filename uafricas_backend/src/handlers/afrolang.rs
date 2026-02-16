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
    AfrolangStatsResponse, CreerSallePriveeRequest, CreerSessionRequest,
    ModerateurResponse, ModifierSallePriveeRequest, ModifierSalleRequest,
    RejoindreRequest, SalleDetailResponse, SalleFiltres, SalleListeResponse,
    SallePriveeDetailResponse, SallePriveeFiltres, SallePriveeListeResponse,
    SallePriveeRow, SalleRow, SessionDetailResponse, SessionFiltres,
    SessionListeResponse, SessionParticipantRow, SessionRow,
    SALLE_COLONNES, SALLE_PRIVEE_COLONNES, SESSION_COLONNES,
    generer_slug,
};

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

    let mut conditions: Vec<String> = vec!["s.actif = true".to_string()];
    let mut bind_index = 1u32;
    let mut bind_values: Vec<String> = Vec::new();

    if let Some(ref langue) = params.langue {
        if !langue.trim().is_empty() {
            conditions.push(format!("LOWER(s.langue_cible) = LOWER(${})", bind_index));
            bind_values.push(langue.trim().to_string());
            bind_index += 1;
        }
    }

    if let Some(ref recherche) = params.recherche {
        if !recherche.trim().is_empty() {
            let terme = format!("%{}%", recherche.trim().to_lowercase());
            conditions.push(format!(
                "(LOWER(s.titre) LIKE ${idx} OR LOWER(s.description) LIKE ${idx})",
                idx = bind_index
            ));
            bind_values.push(terme);
            bind_index += 1;
        }
    }

    let where_clause = conditions.join(" AND ");

    // Compter le total
    let count_query = format!("SELECT COUNT(*) FROM afrolang.salle s WHERE {}", where_clause);
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // Recuperer les salles avec sous-requetes de comptage
    let select_query = format!(
        "SELECT {},
            (SELECT COUNT(*) FROM afrolang.salle_privee sp2
             WHERE sp2.salle_id = s.id AND sp2.actif = true) AS nombre_salles_privees,
            (SELECT COUNT(*) FROM afrolang.session ses2
             JOIN afrolang.salle_privee sp3 ON sp3.id = ses2.salle_privee_id
             WHERE sp3.salle_id = s.id AND ses2.etat = 'en_cours') AS sessions_en_cours
         FROM afrolang.salle s
         WHERE {}
         ORDER BY s.created_at DESC
         LIMIT ${} OFFSET ${}",
        SALLE_COLONNES, where_clause, bind_index, bind_index + 1
    );

    let mut select_q = sqlx::query_as::<_, SalleRow>(&select_query);
    for val in &bind_values {
        select_q = select_q.bind(val);
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

/// GET /api/afrolang/salles/{id} — Detail d'une salle avec ses salles privees
pub async fn obtenir_salle(
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let id = chemin.into_inner();

    // Charger la salle avec moderateur et comptages
    let query = format!(
        "SELECT {},
            u.nom AS moderateur_nom, u.prenom AS moderateur_prenom,
            u.photo_url AS moderateur_photo,
            (SELECT COUNT(*) FROM afrolang.salle_privee sp2
             WHERE sp2.salle_id = s.id AND sp2.actif = true) AS nombre_salles_privees,
            (SELECT COUNT(*) FROM afrolang.session ses2
             JOIN afrolang.salle_privee sp3 ON sp3.id = ses2.salle_privee_id
             WHERE sp3.salle_id = s.id AND ses2.etat = 'en_cours') AS sessions_en_cours
         FROM afrolang.salle s
         LEFT JOIN iam.utilisateur u ON u.id = s.moderateur_id
         WHERE s.id = $1",
        SALLE_COLONNES
    );

    let salle = sqlx::query_as::<_, SalleRow>(&query)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve(format!("Salle {} non trouvee", id)))?;

    // Charger les salles privees associees
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
            actif: salle.actif,
            moderateur: salle.to_moderateur(),
            nombre_salles_privees: salle.nombre_salles_privees.unwrap_or(0),
            sessions_en_cours: salle.sessions_en_cours.unwrap_or(0),
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
    let mut moderateur_id: Option<Uuid> = None;
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
            "moderateur_id" => {
                let val = lire_champ_texte(&mut field).await?;
                moderateur_id = Uuid::parse_str(val.trim()).ok();
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

    let slug = generer_slug(&titre);

    let row = sqlx::query_as::<_, SalleRow>(
        &format!(
            "INSERT INTO afrolang.salle (titre, slug, description, image_couverture_url, langue_cible, moderateur_id, cree_par)
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             RETURNING {}",
            SALLE_COLONNES.replace("s.", "")
        ),
    )
    .bind(titre.trim())
    .bind(&slug)
    .bind(description.as_deref().map(str::trim))
    .bind(&image_couverture_url)
    .bind(langue_cible.as_deref().map(str::trim))
    .bind(moderateur_id)
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
    if let Some(ref mod_id) = body.moderateur_id {
        sets.push(format!("moderateur_id = ${}::UUID", bind_index));
        bind_values.push(mod_id.trim().to_string());
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

/// GET /api/afrolang/salles/{salle_id}/privees — Salles privees d'une salle publique
pub async fn lister_salles_privees(
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    params: web::Query<SallePriveeFiltres>,
) -> Result<HttpResponse, ApiErreur> {
    let salle_id = chemin.into_inner();
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * par_page;

    let mut conditions: Vec<String> = vec![
        "sp.salle_id = $1".to_string(),
        "sp.actif = true".to_string(),
    ];
    let mut bind_index = 2u32;
    let mut bind_values: Vec<String> = Vec::new();

    if let Some(ref recherche) = params.recherche {
        if !recherche.trim().is_empty() {
            let terme = format!("%{}%", recherche.trim().to_lowercase());
            conditions.push(format!(
                "(LOWER(sp.titre) LIKE ${idx} OR LOWER(sp.description) LIKE ${idx})",
                idx = bind_index
            ));
            bind_values.push(terme);
            bind_index += 1;
        }
    }

    let where_clause = conditions.join(" AND ");

    // Compter le total
    let count_query = format!(
        "SELECT COUNT(*) FROM afrolang.salle_privee sp WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query).bind(salle_id);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // Recuperer les salles privees
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
         WHERE {}
         ORDER BY sp.created_at DESC
         LIMIT ${} OFFSET ${}",
        SALLE_PRIVEE_COLONNES, where_clause, bind_index, bind_index + 1
    );

    let mut select_q = sqlx::query_as::<_, SallePriveeRow>(&select_query).bind(salle_id);
    for val in &bind_values {
        select_q = select_q.bind(val);
    }
    select_q = select_q.bind(par_page).bind(offset);

    let rows = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(SallePriveeListeResponse {
            salles_privees: rows.iter().map(|r| r.to_response()).collect(),
            total,
            page,
            par_page,
            total_pages: calculer_total_pages(total, par_page),
        }),
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
            est_protegee: resp.est_protegee,
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

/// POST /api/afrolang/salles/{salle_id}/privees — Creer une salle privee [JWT]
pub async fn creer_salle_privee(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<CreerSallePriveeRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".into()))?;

    let salle_id = chemin.into_inner();

    // Verifier que la salle parente existe et est active
    let salle_existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM afrolang.salle WHERE id = $1 AND actif = true)",
    )
    .bind(salle_id)
    .fetch_one(pool.get_ref())
    .await?;

    if !salle_existe {
        return Err(ApiErreur::NonTrouve("Salle parente non trouvee".into()));
    }

    if body.titre.trim().is_empty() {
        return Err(ApiErreur::Validation("Le titre est obligatoire".into()));
    }

    let row = sqlx::query_as::<_, SallePriveeRow>(
        &format!(
            "INSERT INTO afrolang.salle_privee (salle_id, titre, description, code_acces, max_participants, cree_par)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING {}",
            SALLE_PRIVEE_COLONNES.replace("sp.", "")
        ),
    )
    .bind(salle_id)
    .bind(body.titre.trim())
    .bind(body.description.as_deref().map(str::trim))
    .bind(body.code_acces.as_deref().map(str::trim))
    .bind(body.max_participants)
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    log::info!("Salle privee creee: {} ({})", row.titre, row.id);

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
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
    if let Some(ref code) = body.code_acces {
        sets.push(format!("code_acces = ${}", bind_index));
        bind_values.push(code.trim().to_string());
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

    // Verifier que l'utilisateur est le moderateur
    let createur_salle_privee: Uuid = sqlx::query_scalar(
        "SELECT cree_par FROM afrolang.salle_privee WHERE id = $1",
    )
    .bind(session.salle_privee_id)
    .fetch_one(pool.get_ref())
    .await?;

    if createur_salle_privee != utilisateur_id {
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

    // Verifier le moderateur
    let createur_salle_privee: Uuid = sqlx::query_scalar(
        "SELECT cree_par FROM afrolang.salle_privee WHERE id = $1",
    )
    .bind(session.salle_privee_id)
    .fetch_one(pool.get_ref())
    .await?;

    if createur_salle_privee != utilisateur_id {
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

    // Verifier le code d'acces de la salle privee
    let code_acces_salle: Option<String> = sqlx::query_scalar(
        "SELECT code_acces FROM afrolang.salle_privee WHERE id = $1",
    )
    .bind(session.salle_privee_id)
    .fetch_one(pool.get_ref())
    .await?;

    if let Some(ref code_attendu) = code_acces_salle {
        match &body.code_acces {
            Some(code_fourni) if code_fourni == code_attendu => {}
            _ => {
                return Err(ApiErreur::NonAutorise(
                    "Code d'acces invalide ou manquant".into(),
                ));
            }
        }
    }

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

    // Inserer le participant (ON CONFLICT pour gerer les re-connexions)
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

    // Mettre a jour le pic de participants
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
            "Participant non trouve dans cette session".into(),
        ));
    }

    log::info!("Utilisateur {} a quitte la session {}", utilisateur_id, id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
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

    // 3. Verifier le code d'acces de la salle privee
    let salle_privee_info: (Option<String>, Option<i32>) = sqlx::query_as(
        "SELECT code_acces, max_participants FROM afrolang.salle_privee WHERE id = $1",
    )
    .bind(session.salle_privee_id)
    .fetch_one(pool.get_ref())
    .await?;

    if let Some(ref code_attendu) = salle_privee_info.0 {
        match &body.code_acces {
            Some(code_fourni) if code_fourni == code_attendu => {}
            _ => {
                return Err(ApiErreur::NonAutorise(
                    "Code d'acces invalide ou manquant".into(),
                ));
            }
        }
    }

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
         VALUES ($1, $2, $3::afrolang.role_session)
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
