use actix_web::{web, HttpResponse};
use chrono::DateTime;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::annonce::{
    AdminAnnonceDetailResponse, AdminAnnonceListeResponse, AdminAnnonceMedia,
    AdminAnnoncePays, AdminAnnonceQueryParams, AjouterMediaAnnonceRequest,
    AjouterPaysAnnonceRequest, ChangerEtatAnnonceRequest, CreerAnnonceRequest,
    ModifierAnnonceRequest, ReordonnerMediasRequest, ADMIN_ANNONCE_DETAIL_COLONNES,
    ADMIN_ANNONCE_LISTE_COLONNES, ANNONCE_TRI_COLONNES,
};
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::verifier_permission;
use crate::ApiResponse;

// ── Etats valides pour les enums PostgreSQL ─────────────────
const ETATS_VALIDES: &[&str] = &["brouillon", "publiee", "en_attente", "expiree", "suspendue", "supprimee"];
const TYPES_OPERATION_VALIDES: &[&str] = &["vente", "troc", "don", "association", "opportunite"];
const CONDITIONS_VALIDES: &[&str] = &["neuf", "occasion", "reconditionne", "non_applicable"];
const TYPES_CONTACT_VALIDES: &[&str] = &["email", "telephone", "messagerie_plateforme"];

/// GET /api/admin/annonces
pub async fn lister_annonces(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminAnnonceQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "marketplace", "voir");

    let pagination = PaginationParams {
        page: params.page,
        par_page: params.par_page,
        tri_par: params.tri_par.clone(),
        tri_dir: params.tri_dir.clone(),
    };

    let mut conditions = vec!["a.deleted_at IS NULL".to_string()];
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_index: u32 = 1;

    if let Some(ref recherche) = params.recherche {
        let r = recherche.trim();
        if !r.is_empty() {
            conditions.push(format!(
                "(LOWER(a.titre) LIKE ${bi} OR LOWER(a.description) LIKE ${bi} OR LOWER(COALESCE(a.ville, '')) LIKE ${bi})",
                bi = bind_index
            ));
            bind_values.push(format!("%{}%", r.to_lowercase()));
            bind_index += 1;
        }
    }

    if let Some(ref etat) = params.etat {
        let e = etat.trim();
        if !e.is_empty() {
            conditions.push(format!("a.etat::text = ${}", bind_index));
            bind_values.push(e.to_string());
            bind_index += 1;
        }
    }

    if let Some(ref type_op) = params.type_operation {
        let t = type_op.trim();
        if !t.is_empty() {
            conditions.push(format!("a.type_operation::text = ${}", bind_index));
            bind_values.push(t.to_string());
            bind_index += 1;
        }
    }

    if let Some(ref cat_id) = params.categorie_id {
        let c = cat_id.trim();
        if !c.is_empty() {
            if let Ok(_) = Uuid::parse_str(c) {
                conditions.push(format!("a.categorie_id::text = ${}", bind_index));
                bind_values.push(c.to_string());
                bind_index += 1;
            }
        }
    }

    if let Some(ref pays_id) = params.pays_id {
        let p = pays_id.trim();
        if !p.is_empty() {
            if let Ok(_) = Uuid::parse_str(p) {
                conditions.push(format!(
                    "EXISTS (SELECT 1 FROM marketplace.annonce_pays ap WHERE ap.annonce_id = a.id AND ap.pays_id::text = ${})",
                    bind_index
                ));
                bind_values.push(p.to_string());
                bind_index += 1;
            }
        }
    }

    if let Some(ref cree_par) = params.cree_par {
        let cp = cree_par.trim();
        if !cp.is_empty() {
            if let Ok(_) = Uuid::parse_str(cp) {
                conditions.push(format!("a.cree_par::text = ${}", bind_index));
                bind_values.push(cp.to_string());
                bind_index += 1;
            }
        }
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let colonne = pagination.colonne_tri(ANNONCE_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    // COUNT
    let count_sql = format!(
        "SELECT COUNT(*) FROM marketplace.annonce a
         LEFT JOIN shared.categorie c ON c.id = a.categorie_id
         JOIN iam.utilisateur u ON u.id = a.cree_par
         WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    for v in &bind_values {
        count_q = count_q.bind(v);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // SELECT
    let select_sql = format!(
        "SELECT {} FROM marketplace.annonce a
         LEFT JOIN shared.categorie c ON c.id = a.categorie_id
         JOIN iam.utilisateur u ON u.id = a.cree_par
         WHERE {} ORDER BY a.{} {} LIMIT {} OFFSET {}",
        ADMIN_ANNONCE_LISTE_COLONNES, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminAnnonceListeResponse>(&select_sql);
    for v in &bind_values {
        select_q = select_q.bind(v);
    }
    let items = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/annonces/:id
pub async fn obtenir_annonce(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "marketplace", "voir");
    let id = path.into_inner();

    let sql = format!(
        "SELECT {} FROM marketplace.annonce a
         LEFT JOIN shared.categorie c ON c.id = a.categorie_id
         JOIN iam.utilisateur u ON u.id = a.cree_par
         WHERE a.id = $1 AND a.deleted_at IS NULL",
        ADMIN_ANNONCE_DETAIL_COLONNES
    );
    let row = sqlx::query_as::<_, AdminAnnonceDetailResponse>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Annonce non trouvee".into()))?;

    // Charger les pays cibles
    let pays = sqlx::query_as::<_, AdminAnnoncePays>(
        "SELECT ap.pays_id, p.nom AS pays_nom
         FROM marketplace.annonce_pays ap
         JOIN shared.pays p ON p.id = ap.pays_id
         WHERE ap.annonce_id = $1
         ORDER BY p.nom",
    )
    .bind(id)
    .fetch_all(pool.get_ref())
    .await?;

    // Charger les medias
    let medias = sqlx::query_as::<_, AdminAnnonceMedia>(
        "SELECT id, media_url, type_mime, est_principale, ordre, created_at
         FROM marketplace.annonce_media
         WHERE annonce_id = $1
         ORDER BY est_principale DESC NULLS LAST, ordre ASC",
    )
    .bind(id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "id": row.id,
            "titre": row.titre,
            "slug": row.slug,
            "description": row.description,
            "type_operation": row.type_operation,
            "etat": row.etat,
            "condition_article": row.condition_article,
            "prix": row.prix,
            "devise": row.devise,
            "prix_negociable": row.prix_negociable,
            "ville": row.ville,
            "adresse": row.adresse,
            "longitude": row.longitude,
            "latitude": row.latitude,
            "type_contact": row.type_contact,
            "contact_info": row.contact_info,
            "quantite": row.quantite,
            "nombre_vues": row.nombre_vues,
            "cree_par": row.cree_par,
            "expire_at": row.expire_at,
            "created_at": row.created_at,
            "updated_at": row.updated_at,
            "categorie_id": row.categorie_id,
            "categorie_nom": row.categorie_nom,
            "auteur_nom": row.auteur_nom,
            "auteur_prenom": row.auteur_prenom,
            "auteur_email": row.auteur_email,
            "pays": pays,
            "medias": medias,
        })),
        error: None,
    }))
}

/// POST /api/admin/annonces
pub async fn creer_annonce(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    body: web::Json<CreerAnnonceRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "marketplace", "modifier");

    let titre = body.titre.trim();
    if titre.is_empty() {
        return Err(ApiErreur::Validation("Le titre est requis".into()));
    }
    let description = body.description.trim();
    if description.is_empty() {
        return Err(ApiErreur::Validation("La description est requise".into()));
    }

    let type_op = body.type_operation.trim();
    if !TYPES_OPERATION_VALIDES.contains(&type_op) {
        return Err(ApiErreur::Validation(format!(
            "Type d'operation invalide. Valeurs acceptees: {}",
            TYPES_OPERATION_VALIDES.join(", ")
        )));
    }

    if let Some(ref cond) = body.condition_article {
        if !CONDITIONS_VALIDES.contains(&cond.trim()) {
            return Err(ApiErreur::Validation("Condition article invalide".into()));
        }
    }
    if let Some(ref tc) = body.type_contact {
        if !TYPES_CONTACT_VALIDES.contains(&tc.trim()) {
            return Err(ApiErreur::Validation("Type de contact invalide".into()));
        }
    }

    let etat = body.etat.as_deref().unwrap_or("brouillon").trim();
    if !ETATS_VALIDES.contains(&etat) {
        return Err(ApiErreur::Validation("Etat invalide".into()));
    }

    let slug = titre
        .to_lowercase()
        .replace(' ', "-")
        .replace(['\'', '"', '.', ','], "");
    let id = Uuid::new_v4();

    let expire_at: Option<DateTime<chrono::Utc>> = body.expire_at.as_ref().and_then(|s| {
        chrono::DateTime::parse_from_rfc3339(s)
            .ok()
            .map(|dt| dt.with_timezone(&chrono::Utc))
    });

    sqlx::query(
        "INSERT INTO marketplace.annonce
         (id, titre, slug, description, type_operation, categorie_id, condition_article,
          prix, devise, prix_negociable, ville, adresse, longitude, latitude,
          type_contact, contact_info, quantite, etat, cree_par, expire_at)
         VALUES ($1, $2, $3, $4, $5::marketplace.type_operation, $6,
                 $7::marketplace.condition_article, $8, $9, $10, $11, $12, $13, $14,
                 $15::marketplace.type_contact, $16, $17, $18::marketplace.etat_annonce, $19, $20)",
    )
    .bind(id)
    .bind(titre)
    .bind(&slug)
    .bind(description)
    .bind(type_op)
    .bind(body.categorie_id)
    .bind(body.condition_article.as_deref().unwrap_or("non_applicable"))
    .bind(body.prix)
    .bind(body.devise.as_deref().unwrap_or("XOF"))
    .bind(body.prix_negociable.unwrap_or(false))
    .bind(body.ville.as_deref().map(|s| s.trim()))
    .bind(body.adresse.as_deref().map(|s| s.trim()))
    .bind(body.longitude)
    .bind(body.latitude)
    .bind(body.type_contact.as_deref().unwrap_or("messagerie_plateforme"))
    .bind(body.contact_info.as_deref().map(|s| s.trim()))
    .bind(body.quantite.unwrap_or(1))
    .bind(etat)
    .bind(admin.id)
    .bind(expire_at)
    .execute(pool.get_ref())
    .await?;

    log::info!("Admin {} a cree l'annonce {} ({})", admin.id, titre, id);

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// PUT /api/admin/annonces/:id
pub async fn modifier_annonce(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierAnnonceRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "marketplace", "modifier");
    let id = path.into_inner();

    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM marketplace.annonce WHERE id = $1 AND deleted_at IS NULL)",
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Annonce non trouvee".into()));
    }

    let mut sets = Vec::new();
    let mut bind_strings: Vec<String> = Vec::new();
    let mut bind_uuids: Vec<Uuid> = Vec::new();
    let mut bind_index: u32 = 1;

    macro_rules! ajouter_champ_str {
        ($field:expr, $col:expr) => {
            if let Some(ref val) = $field {
                sets.push(format!("{} = ${}", $col, bind_index));
                bind_strings.push(val.trim().to_string());
                bind_index += 1;
            }
        };
    }

    ajouter_champ_str!(body.titre, "titre");
    ajouter_champ_str!(body.description, "description");
    ajouter_champ_str!(body.ville, "ville");
    ajouter_champ_str!(body.adresse, "adresse");
    ajouter_champ_str!(body.contact_info, "contact_info");
    ajouter_champ_str!(body.devise, "devise");

    // Slug auto-genere si titre change
    if let Some(ref titre) = body.titre {
        let slug = titre
            .trim()
            .to_lowercase()
            .replace(' ', "-")
            .replace(['\'', '"', '.', ','], "");
        sets.push(format!("slug = ${}", bind_index));
        bind_strings.push(slug);
        bind_index += 1;
    }

    // Champs enum (castes)
    if let Some(ref type_op) = body.type_operation {
        let t = type_op.trim();
        if !TYPES_OPERATION_VALIDES.contains(&t) {
            return Err(ApiErreur::Validation("Type d'operation invalide".into()));
        }
        sets.push(format!("type_operation = ${}::marketplace.type_operation", bind_index));
        bind_strings.push(t.to_string());
        bind_index += 1;
    }
    if let Some(ref cond) = body.condition_article {
        let c = cond.trim();
        if !CONDITIONS_VALIDES.contains(&c) {
            return Err(ApiErreur::Validation("Condition article invalide".into()));
        }
        sets.push(format!("condition_article = ${}::marketplace.condition_article", bind_index));
        bind_strings.push(c.to_string());
        bind_index += 1;
    }
    if let Some(ref tc) = body.type_contact {
        let t = tc.trim();
        if !TYPES_CONTACT_VALIDES.contains(&t) {
            return Err(ApiErreur::Validation("Type de contact invalide".into()));
        }
        sets.push(format!("type_contact = ${}::marketplace.type_contact", bind_index));
        bind_strings.push(t.to_string());
        bind_index += 1;
    }

    // UUID
    if let Some(cat_id) = body.categorie_id {
        sets.push(format!("categorie_id = ${}", bind_index));
        bind_uuids.push(cat_id);
        bind_index += 1;
    }

    // Numeriques (inline, pas de bind)
    if let Some(prix) = body.prix {
        sets.push(format!("prix = {}", prix));
    }
    if let Some(quantite) = body.quantite {
        sets.push(format!("quantite = {}", quantite));
    }
    if let Some(prix_neg) = body.prix_negociable {
        sets.push(format!("prix_negociable = {}", prix_neg));
    }
    if let Some(lon) = body.longitude {
        sets.push(format!("longitude = {}", lon));
    }
    if let Some(lat) = body.latitude {
        sets.push(format!("latitude = {}", lat));
    }

    // expire_at
    if let Some(ref exp) = body.expire_at {
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(exp) {
            sets.push(format!("expire_at = ${}", bind_index));
            bind_strings.push(dt.with_timezone(&chrono::Utc).to_rfc3339());
            bind_index += 1;
        }
    }

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }

    sets.push("updated_at = NOW()".to_string());
    let sql = format!(
        "UPDATE marketplace.annonce SET {} WHERE id = ${} AND deleted_at IS NULL",
        sets.join(", "),
        bind_index
    );

    let mut q = sqlx::query(&sql);
    for v in &bind_strings {
        q = q.bind(v);
    }
    for v in &bind_uuids {
        q = q.bind(v);
    }
    q = q.bind(id);
    q.execute(pool.get_ref()).await?;

    log::info!("Admin {} a modifie l'annonce {}", admin.id, id);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// PATCH /api/admin/annonces/:id/etat
pub async fn changer_etat_annonce(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ChangerEtatAnnonceRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "marketplace", "modifier");
    let id = path.into_inner();

    let etat = body.etat.trim();
    if !ETATS_VALIDES.contains(&etat) {
        return Err(ApiErreur::Validation(format!(
            "Etat invalide. Valeurs acceptees: {}",
            ETATS_VALIDES.join(", ")
        )));
    }

    let result = sqlx::query(
        "UPDATE marketplace.annonce SET etat = $1::marketplace.etat_annonce, updated_at = NOW()
         WHERE id = $2 AND deleted_at IS NULL",
    )
    .bind(etat)
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Annonce non trouvee".into()));
    }

    log::info!(
        "Admin {} a change l'etat de l'annonce {} en '{}'",
        admin.id,
        id,
        etat
    );

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id, "etat": etat })),
        error: None,
    }))
}

/// DELETE /api/admin/annonces/:id
pub async fn supprimer_annonce(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "marketplace", "supprimer");
    let id = path.into_inner();

    let result = sqlx::query(
        "UPDATE marketplace.annonce SET deleted_at = NOW(), etat = 'supprimee'::marketplace.etat_annonce, updated_at = NOW()
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Annonce non trouvee".into()));
    }

    log::info!("Admin {} a supprime l'annonce {}", admin.id, id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

/// POST /api/admin/annonces/:id/pays
pub async fn ajouter_pays_annonce(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<AjouterPaysAnnonceRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "marketplace", "modifier");
    let annonce_id = path.into_inner();

    // Verifier que l'annonce existe
    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM marketplace.annonce WHERE id = $1 AND deleted_at IS NULL)",
    )
    .bind(annonce_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Annonce non trouvee".into()));
    }

    // Verifier que le pays existe
    let pays_existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM shared.pays WHERE id = $1 AND actif = true)",
    )
    .bind(body.pays_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !pays_existe {
        return Err(ApiErreur::NonTrouve("Pays non trouve".into()));
    }

    // Inserer (ignorer si deja present)
    sqlx::query(
        "INSERT INTO marketplace.annonce_pays (annonce_id, pays_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
    )
    .bind(annonce_id)
    .bind(body.pays_id)
    .execute(pool.get_ref())
    .await?;

    log::info!(
        "Admin {} a ajoute le pays {} a l'annonce {}",
        admin.id,
        body.pays_id,
        annonce_id
    );

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "annonce_id": annonce_id, "pays_id": body.pays_id })),
        error: None,
    }))
}

/// DELETE /api/admin/annonces/:id/pays/:pays_id
pub async fn retirer_pays_annonce(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "marketplace", "modifier");
    let (annonce_id, pays_id) = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM marketplace.annonce_pays WHERE annonce_id = $1 AND pays_id = $2",
    )
    .bind(annonce_id)
    .bind(pays_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Lien pays-annonce non trouve".into()));
    }

    log::info!(
        "Admin {} a retire le pays {} de l'annonce {}",
        admin.id,
        pays_id,
        annonce_id
    );

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

/// POST /api/admin/annonces/:id/medias
pub async fn ajouter_media_annonce(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<AjouterMediaAnnonceRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "marketplace", "modifier");
    let annonce_id = path.into_inner();

    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM marketplace.annonce WHERE id = $1 AND deleted_at IS NULL)",
    )
    .bind(annonce_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Annonce non trouvee".into()));
    }

    let media_url = body.media_url.trim();
    if media_url.is_empty() {
        return Err(ApiErreur::Validation("L'URL du media est requise".into()));
    }

    let media_id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO marketplace.annonce_media (id, annonce_id, media_url, type_mime, est_principale, ordre)
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(media_id)
    .bind(annonce_id)
    .bind(media_url)
    .bind(body.type_mime.as_deref())
    .bind(body.est_principale.unwrap_or(false))
    .bind(body.ordre.unwrap_or(0))
    .execute(pool.get_ref())
    .await?;

    log::info!(
        "Admin {} a ajoute un media {} a l'annonce {}",
        admin.id,
        media_id,
        annonce_id
    );

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": media_id })),
        error: None,
    }))
}

/// DELETE /api/admin/annonces/:id/medias/:media_id
pub async fn retirer_media_annonce(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "marketplace", "modifier");
    let (annonce_id, media_id) = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM marketplace.annonce_media WHERE id = $1 AND annonce_id = $2",
    )
    .bind(media_id)
    .bind(annonce_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Media non trouve".into()));
    }

    log::info!(
        "Admin {} a retire le media {} de l'annonce {}",
        admin.id,
        media_id,
        annonce_id
    );

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

/// PUT /api/admin/annonces/:id/medias/ordre
pub async fn reordonner_medias_annonce(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ReordonnerMediasRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "marketplace", "modifier");
    let annonce_id = path.into_inner();

    for item in &body.ordres {
        sqlx::query(
            "UPDATE marketplace.annonce_media SET ordre = $1 WHERE id = $2 AND annonce_id = $3",
        )
        .bind(item.ordre)
        .bind(item.media_id)
        .bind(annonce_id)
        .execute(pool.get_ref())
        .await?;
    }

    log::info!(
        "Admin {} a reordonne les medias de l'annonce {}",
        admin.id,
        annonce_id
    );

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}
