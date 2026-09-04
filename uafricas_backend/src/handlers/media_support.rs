//! Fiche d'un support média : thématiques multiples et couverture territoriale
//! (feature 009-medias-programmes-episodes, US3 et US4, migration 09r).
//!
//! Routes membres (gardées par `garde_detenteur`) **et** back-office (gardées
//! par `verifier_permission!`) : les règles de validation sont strictement les
//! mêmes des deux côtés, et les écrire deux fois serait le meilleur moyen d'en
//! avoir une fausse.
//!
//!   GET|PUT /api/medias/{type_support}/{support_id}/thematiques
//!   GET|PUT /api/medias/{type_support}/{support_id}/couverture
//!   GET|PUT /api/admin/medias/{type_support}/{support_id}/thematiques
//!   GET|PUT /api/admin/medias/{type_support}/{support_id}/couverture

use std::collections::HashMap;

use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::handlers::media_detention::{exiger_utilisateur_id, garde_detenteur};
use crate::middleware::admin::AdminUtilisateur;
use crate::models::media_detention::{table_pour_support, valider_type_support};
use crate::models::media_support::{
    CouverturePublique, CouvertureRequest, TerritoireDecompte, TerritoirePublic,
    TerritoiresDisponibles, ThematiqueDecompte, ThematiquePublique, ThematiquesRequest,
};
use crate::services::audit;
use crate::verifier_permission;
use crate::ApiResponse;

// ═══════════════════════════════════════════════════════════════════════════
// Lectures groupées : sans requête N+1
// ═══════════════════════════════════════════════════════════════════════════

/// Thématiques de plusieurs supports, en **une seule requête**.
pub async fn thematiques_par_supports(
    pool: &PgPool,
    type_support: &str,
    support_ids: &[Uuid],
) -> Result<HashMap<Uuid, Vec<ThematiquePublique>>, ApiErreur> {
    let mut resultat: HashMap<Uuid, Vec<ThematiquePublique>> = HashMap::new();
    if support_ids.is_empty() {
        return Ok(resultat);
    }

    // Les genres d'abord, les lignes éditoriales ensuite : la fiche les
    // affiche dans cet ordre, et le tri SQL évite de le refaire à l'écran.
    let lignes: Vec<(Uuid, Uuid, String, bool)> = sqlx::query_as(
        "SELECT st.support_id, cat.id, cat.nom, (cat.parent_id IS NOT NULL) AS est_ligne
           FROM media_content.support_thematique st
           JOIN shared.categorie cat ON cat.id = st.categorie_id
          WHERE st.type_support = $1::media_content.type_support_media
            AND st.support_id = ANY($2)
          ORDER BY (cat.parent_id IS NOT NULL), cat.ordre ASC, cat.nom ASC",
    )
    .bind(type_support)
    .bind(support_ids)
    .fetch_all(pool)
    .await?;

    for (support_id, id, nom, est_ligne_editoriale) in lignes {
        resultat
            .entry(support_id)
            .or_default()
            .push(ThematiquePublique { id, nom, est_ligne_editoriale });
    }
    Ok(resultat)
}

/// Couverture de plusieurs supports, en **deux requêtes**, le drapeau
/// continental vit sur la table du support, les territoires sur la liaison.
pub async fn couverture_par_supports(
    pool: &PgPool,
    type_support: &str,
    support_ids: &[Uuid],
) -> Result<HashMap<Uuid, CouverturePublique>, ApiErreur> {
    let mut resultat: HashMap<Uuid, CouverturePublique> = HashMap::new();
    if support_ids.is_empty() {
        return Ok(resultat);
    }
    let table = table_pour_support(type_support)
        .ok_or_else(|| ApiErreur::Validation("Type de support inconnu".into()))?;

    let drapeaux: Vec<(Uuid, bool, bool)> = sqlx::query_as(&format!(
        "SELECT id, couverture_continentale, est_thematique FROM {table} WHERE id = ANY($1)"
    ))
    .bind(support_ids)
    .fetch_all(pool)
    .await?;

    for (id, continentale, thematique) in drapeaux {
        resultat.insert(
            id,
            CouverturePublique {
                couverture_continentale: continentale,
                territoires: Vec::new(),
                est_thematique: thematique,
            },
        );
    }

    let lignes: Vec<(Uuid, Uuid, String)> = sqlx::query_as(
        "SELECT ste.support_id, p.id, p.nom
           FROM media_content.support_territoire ste
           JOIN shared.pays p ON p.id = ste.pays_id
          WHERE ste.type_support = $1::media_content.type_support_media
            AND ste.support_id = ANY($2)
          ORDER BY p.nom ASC",
    )
    .bind(type_support)
    .bind(support_ids)
    .fetch_all(pool)
    .await?;

    for (support_id, id, nom) in lignes {
        if let Some(couverture) = resultat.get_mut(&support_id) {
            couverture.territoires.push(TerritoirePublic { id, nom });
        }
    }

    Ok(resultat)
}

// ═══════════════════════════════════════════════════════════════════════════
// Référentiels de filtre
// ═══════════════════════════════════════════════════════════════════════════

/// Thèmes **actifs du référentiel `media`**, chacun avec le nombre de supports
/// publiés qui le déclarent : zéro compris.
///
/// La liste partait auparavant des déclarations, et ne montrait donc que les
/// thèmes déjà servis. C'est ce qui cachait l'étendue du catalogue : le
/// visiteur ne pouvait pas distinguer « ce thème n'existe pas ici » de « ce
/// thème n'a encore rien ». Le décompte à `(0)` le dit, et le référentiel
/// affiché reste stable d'une visite à l'autre.
///
/// `origine` borne le décompte à « africans » ou « territoire » (09j/09o) sans
/// retirer aucun thème du catalogue. Passé `None`, le décompte reste global.
/// Comme le filtre d'état, il vit dans la condition de jointure, pas dans le
/// `WHERE` : l'y remonter transformerait la jointure externe en interne et
/// ferait disparaître les thèmes à zéro pour cette origine.
///
/// `groupe` (09u) bascule le RÉFÉRENTIEL lui-même, pas seulement le décompte :
/// `None` interroge les 22 genres de grille (09s, `parent_id IS NULL`, celui
/// de la pastille « Africans Thématique ») ; le slug d'un parent de
/// `shared.categorie` (ex. `media-groupe-africans-tele-international`)
/// interroge ses enfants — les 44 lignes éditoriales propres à la pastille
/// « Africans Télé International », disjointes des 22 premières.
///
/// Le `LEFT JOIN` sur la table du support porte l'état dans sa condition de
/// jointure, non dans le `WHERE` : l'y remonter transformerait la jointure
/// externe en jointure interne et ramènerait au comportement précédent. Le
/// décompte porte sur `s.id` et non sur `st.support_id`, sans quoi un thème
/// déclaré uniquement par des supports non publiés compterait ses lignes de
/// liaison.
pub async fn thematiques_disponibles(
    pool: &PgPool,
    type_support: &str,
    origine: Option<&str>,
    groupe: Option<&str>,
) -> Result<Vec<ThematiqueDecompte>, ApiErreur> {
    let table = table_pour_support(type_support)
        .ok_or_else(|| ApiErreur::Validation("Type de support inconnu".into()))?;

    // Le groupe des 44 lignes éditoriales suit la séquence narrative du seed
    // (09j/09u) plutôt que l'ordre alphabétique du référentiel générique.
    let ordre_par = if groupe.is_some() { "cat.ordre ASC, cat.nom ASC" } else { "cat.nom ASC" };

    let lignes = sqlx::query_as::<_, ThematiqueDecompte>(&format!(
        "SELECT cat.id, cat.nom, cat.description, COUNT(s.id)::bigint AS nombre_supports
           FROM shared.categorie cat
           LEFT JOIN media_content.support_thematique st
                  ON st.categorie_id = cat.id
                 AND st.type_support = $1::media_content.type_support_media
           LEFT JOIN {table} s
                  ON s.id = st.support_id
                 AND s.etat = 'publie'
                 AND s.deleted_at IS NULL
                 AND ($2::text IS NULL OR s.origine_publication = $2)
          WHERE cat.contexte = 'media' AND cat.actif = TRUE
            AND (
                  ($3::text IS NULL AND cat.parent_id IS NULL)
                  OR cat.parent_id = (SELECT id FROM shared.categorie WHERE slug = $3)
                )
          GROUP BY cat.id, cat.nom, cat.description
          ORDER BY {ordre_par}"
    ))
    .bind(type_support)
    .bind(origine)
    .bind(groupe)
    .fetch_all(pool)
    .await?;

    Ok(lignes)
}

/// Territoires réellement couverts, plus le marqueur des supports panafricains.
///
/// Ces derniers ne peuvent pas être comptés dans les lignes ci-dessus : ils
/// remontent sur **chaque** territoire (FR-036) et gonfleraient tous les
/// décomptes.
pub async fn territoires_disponibles(
    pool: &PgPool,
    type_support: &str,
) -> Result<TerritoiresDisponibles, ApiErreur> {
    let table = table_pour_support(type_support)
        .ok_or_else(|| ApiErreur::Validation("Type de support inconnu".into()))?;

    let territoires = sqlx::query_as::<_, TerritoireDecompte>(&format!(
        "SELECT p.id, p.nom, COUNT(*)::bigint AS nombre_supports
           FROM media_content.support_territoire ste
           JOIN shared.pays p ON p.id = ste.pays_id
           JOIN {table} s ON s.id = ste.support_id
          WHERE ste.type_support = $1::media_content.type_support_media
            AND s.etat = 'publie' AND s.deleted_at IS NULL
          GROUP BY p.id, p.nom
          ORDER BY p.nom ASC"
    ))
    .bind(type_support)
    .fetch_all(pool)
    .await?;

    let continentales: i64 = sqlx::query_scalar(&format!(
        "SELECT COUNT(*) FROM {table}
          WHERE couverture_continentale = TRUE AND etat = 'publie' AND deleted_at IS NULL"
    ))
    .fetch_one(pool)
    .await?;

    Ok(TerritoiresDisponibles {
        territoires,
        continentales,
    })
}

// ═══════════════════════════════════════════════════════════════════════════
// Écritures : règles communes membre et back-office
// ═══════════════════════════════════════════════════════════════════════════

/// Remplacement intégral des thématiques d'un support, **borné aux genres de
/// grille** (09s, `parent_id IS NULL`).
///
/// Les identifiants sont confrontés au **contexte `media`** du référentiel : un
/// thème d'un autre contexte passerait sinon inaperçu, la liaison ne portant
/// aucune clé étrangère.
///
/// Le bornage n'est pas un raffinement : le formulaire d'édition ne propose que
/// les genres (`referentiels_edition` filtre `parent_id IS NULL`), et il envoie
/// la liste complète de ce qu'il connaît. Un `DELETE` non borné effacerait donc
/// **silencieusement** les lignes éditoriales d'Africans Télé International
/// (09u) dès qu'un co-détenteur toucherait à ses genres — une chaîne perdrait
/// treize déclarations sans que personne ne l'ait demandé, et sans erreur.
/// Une ligne éditoriale soumise ici est refusée pour la raison symétrique :
/// cet endpoint ne gère pas ce référentiel, il ne doit pas pouvoir y ajouter.
async fn appliquer_thematiques(
    pool: &PgPool,
    type_support: &str,
    support_id: Uuid,
    body: &ThematiquesRequest,
) -> Result<usize, ApiErreur> {
    let table = table_pour_support(type_support)
        .ok_or_else(|| ApiErreur::Validation("Type de support inconnu".into()))?;

    let (etat, est_thematique): (String, bool) = sqlx::query_as(&format!(
        "SELECT etat, est_thematique FROM {table} WHERE id = $1 AND deleted_at IS NULL"
    ))
    .bind(support_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Support introuvable".into()))?;

    body.valider(&etat)?;
    let ids = body.ids_uniques();

    // Un support thématique en déclare UNE, ni zéro ni deux (09v). Le trigger
    // SQL refuse la seconde ; il ne peut rien contre l'absence, une thématique
    // n'ayant pas encore été écrite au moment où le support naît. C'est donc
    // ici, et ici seulement, que « exactement une » est tenue.
    if est_thematique && ids.len() != 1 {
        return Err(ApiErreur::Validation(
            "Un support thématique déclare exactement une thématique".into(),
        ));
    }

    if !ids.is_empty() {
        let valides: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM shared.categorie
              WHERE id = ANY($1) AND contexte = 'media' AND parent_id IS NULL",
        )
        .bind(&ids)
        .fetch_one(pool)
        .await?;
        if valides as usize != ids.len() {
            return Err(ApiErreur::Validation(
                "Une des thématiques ne relève pas du référentiel média".into(),
            ));
        }
    }

    let mut tx = pool.begin().await?;

    sqlx::query(
        "DELETE FROM media_content.support_thematique st
          USING shared.categorie cat
          WHERE cat.id = st.categorie_id
            AND cat.parent_id IS NULL
            AND st.type_support = $1::media_content.type_support_media
            AND st.support_id = $2",
    )
    .bind(type_support)
    .bind(support_id)
    .execute(&mut *tx)
    .await?;

    for id in &ids {
        sqlx::query(
            "INSERT INTO media_content.support_thematique (type_support, support_id, categorie_id)
             VALUES ($1::media_content.type_support_media, $2, $3)
             ON CONFLICT DO NOTHING",
        )
        .bind(type_support)
        .bind(support_id)
        .bind(id)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(ids.len())
}

/// Remplacement intégral de la couverture.
///
/// Le passage à `couverture_continentale = TRUE` supprime les lignes de
/// territoire **dans la même transaction** : les laisser violerait l'invariant
/// que le trigger défend à l'écriture suivante, et la fiche afficherait deux
/// couvertures contradictoires entre-temps.
async fn appliquer_couverture(
    pool: &PgPool,
    type_support: &str,
    support_id: Uuid,
    body: &CouvertureRequest,
) -> Result<(), ApiErreur> {
    let table = table_pour_support(type_support)
        .ok_or_else(|| ApiErreur::Validation("Type de support inconnu".into()))?;

    let (etat, est_thematique): (String, bool) = sqlx::query_as(&format!(
        "SELECT etat, est_thematique FROM {table} WHERE id = $1 AND deleted_at IS NULL"
    ))
    .bind(support_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Support introuvable".into()))?;

    body.valider(&etat)?;
    let ids = body.ids_uniques();

    // Le CHECK `ck_*_thematique_continentale` rejetterait de toute façon la
    // bascule, mais avec le message de PostgreSQL. Le refus est formulé ici,
    // dans les termes de l'exigence : un support thématique n'a pas de
    // territoire à déclarer, il les couvre tous.
    if est_thematique && (!body.couverture_continentale || !ids.is_empty()) {
        return Err(ApiErreur::Validation(
            "Un support thématique couvre d'office tous les territoires : sa couverture ne se saisit pas".into(),
        ));
    }

    let mut tx = pool.begin().await?;

    // L'ordre compte : vider les territoires AVANT de poser le drapeau, sinon
    // le trigger refuserait la moindre réécriture ultérieure.
    sqlx::query(
        "DELETE FROM media_content.support_territoire
          WHERE type_support = $1::media_content.type_support_media AND support_id = $2",
    )
    .bind(type_support)
    .bind(support_id)
    .execute(&mut *tx)
    .await?;

    sqlx::query(&format!(
        "UPDATE {table} SET couverture_continentale = $2, updated_at = NOW() WHERE id = $1"
    ))
    .bind(support_id)
    .bind(body.couverture_continentale)
    .execute(&mut *tx)
    .await?;

    if !body.couverture_continentale {
        for id in &ids {
            sqlx::query(
                "INSERT INTO media_content.support_territoire (type_support, support_id, pays_id)
                 VALUES ($1::media_content.type_support_media, $2, $3)
                 ON CONFLICT DO NOTHING",
            )
            .bind(type_support)
            .bind(support_id)
            .bind(id)
            .execute(&mut *tx)
            .await?;
        }
    }

    tx.commit().await?;
    Ok(())
}

async fn journaliser(
    req: &HttpRequest,
    pool: &PgPool,
    moi: Uuid,
    type_support: &str,
    support_id: Uuid,
    table_logique: &str,
    apres: serde_json::Value,
) {
    let ip = audit::extraire_ip(req);
    let ua = audit::extraire_user_agent(req);
    audit::log_action(
        pool,
        Some(moi),
        "UPDATE",
        "media_content",
        table_logique,
        Some(support_id),
        Some(serde_json::json!({ "type_support": type_support })),
        Some(apres),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;
}

// ═══════════════════════════════════════════════════════════════════════════
// Routes membre
// ═══════════════════════════════════════════════════════════════════════════

/// Thématiques d'un support **pour son formulaire d'édition** : les genres de
/// grille seulement, comme `PUT` n'en accepte que.
///
/// La symétrie n'est pas cosmétique. Les formulaires préchargent leur
/// sélection avec `themes.map(t => t.id)` et la renvoient telle quelle : y
/// laisser les lignes éditoriales (09u) ferait échouer l'enregistrement en 400
/// sur une valeur que l'écran n'a jamais proposée ni affichée. La fiche
/// publique, elle, passe par `thematiques_par_supports` et reçoit bien les
/// deux référentiels, distingués par `est_ligne_editoriale`.
pub async fn obtenir_thematiques(
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    let (type_support, support_id) = chemin.into_inner();
    valider_type_support(&type_support)?;
    let map = thematiques_par_supports(pool.get_ref(), &type_support, &[support_id]).await?;
    let genres: Vec<ThematiquePublique> = map
        .get(&support_id)
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .filter(|t| !t.est_ligne_editoriale)
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(genres),
        error: None,
    }))
}

pub async fn definir_thematiques(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, Uuid)>,
    body: web::Json<ThematiquesRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = exiger_utilisateur_id(&req)?;
    let (type_support, support_id) = chemin.into_inner();
    garde_detenteur(pool.get_ref(), &type_support, support_id, moi, "co_detenteur").await?;

    let nombre = appliquer_thematiques(pool.get_ref(), &type_support, support_id, &body).await?;
    journaliser(
        &req,
        pool.get_ref(),
        moi,
        &type_support,
        support_id,
        "support_thematique",
        serde_json::json!({ "thematiques": nombre }),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

pub async fn obtenir_couverture(
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    let (type_support, support_id) = chemin.into_inner();
    valider_type_support(&type_support)?;
    let map = couverture_par_supports(pool.get_ref(), &type_support, &[support_id]).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: map.get(&support_id).cloned(),
        error: None,
    }))
}

pub async fn definir_couverture(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, Uuid)>,
    body: web::Json<CouvertureRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = exiger_utilisateur_id(&req)?;
    let (type_support, support_id) = chemin.into_inner();
    garde_detenteur(pool.get_ref(), &type_support, support_id, moi, "co_detenteur").await?;

    appliquer_couverture(pool.get_ref(), &type_support, support_id, &body).await?;
    journaliser(
        &req,
        pool.get_ref(),
        moi,
        &type_support,
        support_id,
        "support_territoire",
        serde_json::json!({
            "couverture_continentale": body.couverture_continentale,
            "territoires": body.ids_uniques().len(),
        }),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// Routes back-office
// ═══════════════════════════════════════════════════════════════════════════

pub async fn admin_obtenir_thematiques(
    pool: web::Data<PgPool>,
    admin: AdminUtilisateur,
    chemin: web::Path<(String, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "voir");
    obtenir_thematiques(pool, chemin).await
}

pub async fn admin_definir_thematiques(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    admin: AdminUtilisateur,
    chemin: web::Path<(String, Uuid)>,
    body: web::Json<ThematiquesRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "modifier");
    let (type_support, support_id) = chemin.into_inner();
    valider_type_support(&type_support)?;

    let nombre = appliquer_thematiques(pool.get_ref(), &type_support, support_id, &body).await?;
    journaliser(
        &req,
        pool.get_ref(),
        admin.id,
        &type_support,
        support_id,
        "support_thematique",
        serde_json::json!({ "thematiques": nombre }),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

pub async fn admin_obtenir_couverture(
    pool: web::Data<PgPool>,
    admin: AdminUtilisateur,
    chemin: web::Path<(String, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "voir");
    obtenir_couverture(pool, chemin).await
}

pub async fn admin_definir_couverture(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    admin: AdminUtilisateur,
    chemin: web::Path<(String, Uuid)>,
    body: web::Json<CouvertureRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "modifier");
    let (type_support, support_id) = chemin.into_inner();
    valider_type_support(&type_support)?;

    appliquer_couverture(pool.get_ref(), &type_support, support_id, &body).await?;
    journaliser(
        &req,
        pool.get_ref(),
        admin.id,
        &type_support,
        support_id,
        "support_territoire",
        serde_json::json!({
            "couverture_continentale": body.couverture_continentale,
            "territoires": body.ids_uniques().len(),
        }),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// Référentiels d'ÉDITION : GET /api/medias/referentiels
// ═══════════════════════════════════════════════════════════════════════════

/// Catalogue complet des thèmes et des territoires, pour les **sélecteurs**.
///
/// Distinct des référentiels de filtre ci-dessus, qui portent un décompte par
/// support et, pour les territoires, ne listent que ce qui est **déjà
/// couvert** : les réutiliser ici rendrait un territoire inédit inatteignable,
/// le premier support à vouloir le choisir ne le verrait pas dans la liste.
///
/// `parent_id IS NULL` : les 22 genres de grille (09s) seulement. Les 44
/// lignes éditoriales d'Africans Télé International (09u) sont un sous-groupe
/// dédié à la pastille de filtre du même nom, pas une déclaration que
/// n'importe quel support peut se voir proposer ici — les y inclure
/// doublerait ce sélecteur pour toutes les chaînes et stations.
pub async fn referentiels_edition(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let thematiques = sqlx::query_as::<_, (Uuid, String)>(
        "SELECT id, nom FROM shared.categorie
          WHERE contexte = 'media' AND actif = TRUE AND parent_id IS NULL
          ORDER BY nom ASC",
    )
    .fetch_all(pool.get_ref())
    .await?;

    // Les 198 lignes de `shared.pays` couvrent le monde entier : proposer
    // l'Albanie ou l'Argentine comme territoire d'une chaîne panafricaine était
    // du bruit, et noyait les 54 territoires qui ont un sens ici. Les pays
    // inactifs sont écartés au passage, comme partout ailleurs.
    let territoires = sqlx::query_as::<_, (Uuid, String)>(
        "SELECT id, nom FROM shared.pays
          WHERE actif = TRUE AND continent = 'Afrique'
          ORDER BY nom ASC",
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "thematiques": thematiques.iter()
                .map(|(id, nom)| serde_json::json!({ "id": id, "nom": nom }))
                .collect::<Vec<_>>(),
            "territoires": territoires.iter()
                .map(|(id, nom)| serde_json::json!({ "id": id, "nom": nom }))
                .collect::<Vec<_>>(),
        })),
        error: None,
    }))
}
