//! Cadeaux virtuels — endpoints membre et publics (feature 008).
//!
//! - `GET  /api/engagement/cadeaux` (public) — catalogue
//! - `POST /api/engagement/cadeaux/envoyer` (JWT) — intention de paiement
//! - `POST /api/engagement/paiements/{reference}/confirmer` (JWT) — issue
//! - `GET  /api/engagement/cadeaux/{type_objet}/{objet_id}` (public) — cadeaux d'un contenu
//! - `GET  /api/engagement/mes-cadeaux` (JWT, paginé)
//! - `GET  /api/engagement/ma-cagnotte` (JWT)
//!
//! **Règle de sécurité transversale** : ni le montant, ni les points, ni le
//! taux, ni le bénéficiaire ne sont acceptés du client. L'offreur vient du JWT,
//! le prix et les points du catalogue, le bénéficiaire de
//! `services::engagement::resoudre_beneficiaire`. Un client ne peut exprimer que
//! **quel cadeau** et **sur quoi**.

use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::engagement_cadeau::*;
use crate::models::notification;
use crate::services::{engagement, paiement};
use crate::ApiResponse;

/// Extrait l'utilisateur connecté depuis le header Authorization (JWT).
fn extraire_utilisateur_id(req: &HttpRequest) -> Option<Uuid> {
    let header = req.headers().get("Authorization")?.to_str().ok()?;
    let token = header.strip_prefix("Bearer ")?;
    let secret = std::env::var("JWT_SECRET").ok()?;
    let claims = jwt::valider_token(token, &secret).ok()?;
    Uuid::parse_str(&claims.sub).ok()
}

fn exiger_utilisateur_id(req: &HttpRequest) -> Result<Uuid, ApiErreur> {
    extraire_utilisateur_id(req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))
}

/// Nom affichable d'un membre — même expression partout, pour qu'un offreur
/// s'affiche identiquement dans une modale, un journal et une notification.
const NOM_AFFICHE: &str = "TRIM(COALESCE(u.prenom, '') || ' ' || COALESCE(u.nom, ''))";

/// Lit les paramètres de monétisation. La ligne est garantie unique par le
/// schéma (`id BOOLEAN PRIMARY KEY CHECK (id)`), donc jamais absente après la
/// migration — le repli n'existe que pour ne pas planter une base à moitié
/// migrée.
async fn charger_parametres(pool: &PgPool) -> Result<ParametreMonetisation, ApiErreur> {
    let params = sqlx::query_as::<_, ParametreMonetisation>(
        "SELECT taux_commission, devise, paiement_reel_actif, updated_at
         FROM engagement.parametre_monetisation WHERE id = TRUE",
    )
    .fetch_optional(pool)
    .await?;

    Ok(params.unwrap_or(ParametreMonetisation {
        taux_commission: 10,
        devise: "XOF".to_string(),
        paiement_reel_actif: false,
        updated_at: Utc::now(),
    }))
}

async fn charger_membre_bref(pool: &PgPool, id: Uuid) -> Result<MembreBref, ApiErreur> {
    sqlx::query_as::<_, MembreBref>(&format!(
        "SELECT u.id, {NOM_AFFICHE} AS nom_affiche FROM iam.utilisateur u WHERE u.id = $1"
    ))
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Membre introuvable".to_string()))
}

/// Table et **expression** portant le libellé d'une famille, pour `cible.titre`.
///
/// Les noms de colonne diffèrent d'une famille à l'autre et ne sont pas
/// devinables : `nom_emission` pour les programmes, `nom_complet` pour une
/// personnalité, et rien du tout pour `codimoi` et `factcheck`, dont le contenu
/// EST le texte — on le tronque plutôt que de renvoyer un pavé dans une cellule
/// de tableau. `fiche_pays` n'a pas de nom propre : le nom vient du territoire
/// référencé, d'où la sous-requête.
///
/// `profil` n'y figure pas : un cadeau offert depuis un profil n'a pas de titre,
/// et le nom du bénéficiaire est déjà affiché dans sa propre colonne.
pub(crate) fn source_titre(type_objet: &str) -> Option<(&'static str, &'static str)> {
    match type_objet {
        "codimoi" => Some(("culture.codimoi", "LEFT(contenu, 80)")),
        "factcheck" => Some(("governance.factcheck", "LEFT(contenu, 80)")),
        "video" => Some(("media_content.video", "titre")),
        "fiche_pays" => Some((
            "country_profile.fiche_pays",
            "(SELECT p.nom FROM shared.pays p WHERE p.id = pays_id)",
        )),
        "chaine_tv" => Some(("media_content.chaine_tv", "nom")),
        "station_radio" => Some(("media_content.station_radio", "nom")),
        "programme_tele" => Some(("media_content.programme_tele", "nom_emission")),
        "programme_radio" => Some(("media_content.programme_radio", "nom_emission")),
        "personnalite_connue" => Some(("country_profile.personnalite_connue", "nom_complet")),
        "recette_culinaire" => Some(("country_profile.recette_culinaire", "titre")),
        _ => None,
    }
}

/// Résout le titre d'une cible.
///
/// Une requête par ligne affichée : la page en compte 25 au plus, et le journal
/// comptable est consulté ponctuellement. Une jointure par famille imposerait
/// dix `LEFT JOIN` inter-schémas pour la même information — c'est le compromis
/// déjà retenu par la file de modération média.
///
/// Un échec renvoie `None` plutôt qu'une erreur : un titre manquant ne doit pas
/// faire échouer la lecture d'un journal comptable.
pub(crate) async fn resoudre_titre(pool: &PgPool, type_objet: &str, objet_id: Uuid) -> Option<String> {
    let (table, expression) = source_titre(type_objet)?;
    sqlx::query_scalar(&format!("SELECT ({expression})::text FROM {table} WHERE id = $1"))
        .bind(objet_id)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten()
}

// ════════════════════════════════════════════════════════════════════════════
// 1. CATALOGUE
// ════════════════════════════════════════════════════════════════════════════

/// GET /api/engagement/cadeaux — catalogue actif + contexte de monétisation.
pub async fn catalogue(pool: web::Data<PgPool>) -> Result<HttpResponse, ApiErreur> {
    let params = charger_parametres(pool.get_ref()).await?;

    let cadeaux = sqlx::query_as::<_, CadeauPublic>(
        "SELECT id, code, libelle, description, icone, couleur, prix, points, ordre
         FROM engagement.cadeau WHERE actif = TRUE
         ORDER BY ordre, points DESC",
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(CatalogueResponse {
            devise: params.devise,
            taux_commission: params.taux_commission,
            // C'est ce drapeau qui pilote le bandeau d'avertissement de phase de
            // test : le membre doit savoir qu'aucun argent ne circule.
            paiement_simule: !params.paiement_reel_actif,
            cadeaux,
        }),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// 2. ENVOI — création de l'intention
// ════════════════════════════════════════════════════════════════════════════

/// POST /api/engagement/cadeaux/envoyer
///
/// Aucun point n'est crédité, aucune cagnotte n'est touchée à cette étape : la
/// transaction naît en `en_attente` et n'a d'effet qu'une fois confirmée.
pub async fn envoyer_cadeau(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<EnvoyerCadeauRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let offreur_id = exiger_utilisateur_id(&req)?;

    let mode = body.mode.trim();
    if !MODES_CADEAU.contains(&mode) {
        return Err(ApiErreur::Validation(format!("Mode d'offre inconnu : « {mode} »")));
    }

    let type_objet = body.cible.type_objet.trim();
    // Distinction volontaire : une famille **connue mais sans auteur** est un
    // conflit d'état (409), pas une requête invalide (400). Le client doit
    // pouvoir dire « ce contenu n'a pas d'auteur » plutôt que « erreur de saisie ».
    if FAMILLES_SANS_AUTEUR.contains(&type_objet) {
        return Err(ApiErreur::Conflit(
            "Ce contenu n'a pas d'auteur enregistré : aucun cadeau ne peut lui être offert."
                .to_string(),
        ));
    }
    if !FAMILLES_CADEAU.contains(&type_objet) {
        return Err(ApiErreur::Validation(format!(
            "Aucun cadeau ne peut être offert sur « {type_objet} »"
        )));
    }

    // Prix et points viennent du CATALOGUE, jamais du client.
    let cadeau = sqlx::query_as::<_, CadeauPublic>(
        "SELECT id, code, libelle, description, icone, couleur, prix, points, ordre
         FROM engagement.cadeau WHERE id = $1 AND actif = TRUE",
    )
    .bind(body.cadeau_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Ce cadeau n'est plus disponible".to_string()))?;

    let beneficiaire_id =
        engagement::resoudre_beneficiaire(pool.get_ref(), type_objet, body.cible.objet_id)
            .await
            .ok_or_else(|| {
                ApiErreur::Conflit(
                    "Ce contenu n'a pas d'auteur enregistré : aucun cadeau ne peut lui être offert."
                        .to_string(),
                )
            })?;

    // Refus explicite avant l'écriture — la contrainte `ck_transaction_pas_auto_cadeau`
    // l'interdirait de toute façon, mais un 500 de contrainte ne dit rien au membre.
    if beneficiaire_id == offreur_id {
        return Err(ApiErreur::AccesInterdit(
            "Vous ne pouvez pas vous offrir un cadeau à vous-même.".to_string(),
        ));
    }

    let params = charger_parametres(pool.get_ref()).await?;
    let taux = params.taux_commission;

    // Répartition PAR DIFFÉRENCE : calculer les deux parts indépendamment
    // perdrait un franc d'arrondi. Le `CHECK` en base refuserait l'écriture.
    // Le mode « points » s'exprime dans le même schéma, sans cas particulier.
    let part_beneficiaire = if mode == "points" {
        0
    } else {
        (cadeau.prix as i64 * (100 - taux as i64) / 100) as i32
    };
    let part_plateforme = cadeau.prix - part_beneficiaire;

    let transaction_id = Uuid::new_v4();
    let intention = paiement::initier(cadeau.prix, transaction_id);

    let message = body
        .message
        .as_deref()
        .map(str::trim)
        .filter(|m| !m.is_empty())
        .map(|m| m.chars().take(280).collect::<String>());

    sqlx::query(
        "INSERT INTO engagement.transaction_cadeau
           (id, offreur_id, beneficiaire_id, cadeau_id, type_objet, objet_id, mode,
            montant, points, taux_commission, part_beneficiaire, part_plateforme,
            simule, reference_paiement, message)
         VALUES ($1, $2, $3, $4, $5, $6, $7::engagement.mode_cadeau,
                 $8, $9, $10, $11, $12, $13, $14, $15)",
    )
    .bind(transaction_id)
    .bind(offreur_id)
    .bind(beneficiaire_id)
    .bind(cadeau.id)
    .bind(type_objet)
    .bind(body.cible.objet_id)
    .bind(mode)
    .bind(cadeau.prix)
    .bind(cadeau.points)
    .bind(taux)
    .bind(part_beneficiaire)
    .bind(part_plateforme)
    .bind(intention.simule)
    .bind(&intention.reference)
    .bind(message.as_deref())
    .execute(pool.get_ref())
    .await?;

    let beneficiaire = charger_membre_bref(pool.get_ref(), beneficiaire_id).await?;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(IntentionResponse {
            transaction_id,
            reference_paiement: intention.reference,
            etat: "en_attente".to_string(),
            montant: cadeau.prix,
            points: cadeau.points,
            part_beneficiaire,
            part_plateforme,
            beneficiaire,
            simule: intention.simule,
            expire_at: Utc::now() + Duration::minutes(paiement::EXPIRATION_MINUTES),
        }),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// 3. CONFIRMATION — l'issue du paiement
// ════════════════════════════════════════════════════════════════════════════

/// POST /api/engagement/paiements/{reference}/confirmer
///
/// Séquence impérative (research R10) :
/// 1. `UPDATE … WHERE etat = 'en_attente'` — c'est le **verrou d'idempotence**
///    de la confirmation : deux requêtes concurrentes ne peuvent pas toutes deux
///    passer cette étape.
/// 2. Cagnotte créditée **dans la même transaction SQL** — une cagnotte créditée
///    sans transaction aboutie serait de l'argent inventé.
/// 3. `COMMIT`.
/// 4. **Après le commit seulement** : les points et la notification. Ils sont
///    accessoires et non bloquants ; les mettre dans la transaction comptable
///    ferait échouer un envoi payé à cause d'une erreur du moteur de points.
pub async fn confirmer_paiement(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<String>,
    body: web::Json<ConfirmerPaiementRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = exiger_utilisateur_id(&req)?;
    let reference = chemin.into_inner();

    let transaction = sqlx::query_as::<_, TransactionCadeau>(&format!(
        "SELECT {COLONNES_TRANSACTION} FROM engagement.transaction_cadeau
          WHERE reference_paiement = $1"
    ))
    .bind(&reference)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Paiement introuvable".to_string()))?;

    if transaction.offreur_id != utilisateur_id {
        return Err(ApiErreur::AccesInterdit(
            "Ce paiement ne vous appartient pas.".to_string(),
        ));
    }

    let beneficiaire = charger_membre_bref(pool.get_ref(), transaction.beneficiaire_id).await;

    // ── Rejeu d'une confirmation déjà aboutie ────────────────────────────────
    // Réponse identique, 0 point supplémentaire. C'est le comportement attendu
    // quand le membre recharge sa page de retour de paiement.
    if transaction.etat == "abouti" {
        return Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(ConfirmationResponse {
                transaction_id: transaction.id,
                etat: transaction.etat.clone(),
                points_credites: transaction.points,
                beneficiaire: beneficiaire?,
            }),
            error: None,
        }));
    }

    if transaction.etat != "en_attente" {
        return Err(ApiErreur::Conflit(format!(
            "Ce paiement est déjà « {} » et ne peut plus être confirmé.",
            transaction.etat
        )));
    }

    // ── Expiration paresseuse ────────────────────────────────────────────────
    // Résolue à la lecture, jamais par une tâche de fond — même motif que les
    // créneaux de programmation média.
    if Utc::now() - transaction.created_at > Duration::minutes(paiement::EXPIRATION_MINUTES) {
        sqlx::query(
            "UPDATE engagement.transaction_cadeau
                SET etat = 'expire', finalise_at = NOW()
              WHERE id = $1 AND etat = 'en_attente'",
        )
        .bind(transaction.id)
        .execute(pool.get_ref())
        .await?;

        return Err(ApiErreur::Conflit(
            "Ce paiement a expiré. Vous pouvez recommencer votre envoi.".to_string(),
        ));
    }

    // Le bénéficiaire a pu disparaître entre l'envoi et la confirmation.
    let Ok(beneficiaire) = beneficiaire else {
        sqlx::query(
            "UPDATE engagement.transaction_cadeau
                SET etat = 'echoue', finalise_at = NOW()
              WHERE id = $1 AND etat = 'en_attente'",
        )
        .bind(transaction.id)
        .execute(pool.get_ref())
        .await?;

        return Err(ApiErreur::Conflit(
            "Le bénéficiaire n'est plus disponible ; aucun montant n'a été prélevé.".to_string(),
        ));
    };

    // ── Issue rendue par le prestataire ──────────────────────────────────────
    let aboutit = matches!(
        paiement::confirmer(&reference, body.aboutir),
        paiement::EtatPaiement::Abouti
    );

    if !aboutit {
        sqlx::query(
            "UPDATE engagement.transaction_cadeau
                SET etat = 'echoue', finalise_at = NOW()
              WHERE id = $1 AND etat = 'en_attente'",
        )
        .bind(transaction.id)
        .execute(pool.get_ref())
        .await?;

        return Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(ConfirmationResponse {
                transaction_id: transaction.id,
                etat: "echoue".to_string(),
                points_credites: 0,
                beneficiaire,
            }),
            error: None,
        }));
    }

    // ── Comptabilité atomique ────────────────────────────────────────────────
    let mut tx = pool.begin().await?;

    let bascule = sqlx::query(
        "UPDATE engagement.transaction_cadeau
            SET etat = 'abouti', finalise_at = NOW()
          WHERE id = $1 AND etat = 'en_attente'",
    )
    .bind(transaction.id)
    .execute(&mut *tx)
    .await?;

    // Une confirmation concurrente a gagné la course : elle a déjà tout fait.
    if bascule.rows_affected() == 0 {
        tx.rollback().await?;
        return Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(ConfirmationResponse {
                transaction_id: transaction.id,
                etat: "abouti".to_string(),
                points_credites: transaction.points,
                beneficiaire,
            }),
            error: None,
        }));
    }

    // La cagnotte n'est alimentée qu'en soutien financier : en mode « points »,
    // `part_beneficiaire` vaut 0 et le `CHECK` en base l'impose déjà.
    if transaction.mode == "soutien_financier" && transaction.part_beneficiaire > 0 {
        sqlx::query(
            "INSERT INTO engagement.cagnotte (utilisateur_id, montant_cumule)
             VALUES ($1, $2)
             ON CONFLICT (utilisateur_id) DO UPDATE
                SET montant_cumule = engagement.cagnotte.montant_cumule + EXCLUDED.montant_cumule,
                    updated_at = NOW()",
        )
        .bind(transaction.beneficiaire_id)
        .bind(transaction.part_beneficiaire)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    // ── Après le COMMIT seulement ────────────────────────────────────────────
    engagement::crediter_cadeau(
        pool.get_ref(),
        transaction.beneficiaire_id,
        transaction.id,
        transaction.points,
    )
    .await;

    // `points_credites` reflète ce que le catalogue promettait. Si la règle
    // `cadeau_recu` est désactivée, aucun point n'est réellement attribué : on
    // relit donc le journal plutôt que d'annoncer un crédit qui n'a pas eu lieu.
    let points_credites: i32 = sqlx::query_scalar(
        "SELECT COALESCE(points, 0) FROM engagement.mouvement_points
          WHERE cle_idempotence = $1",
    )
    .bind(format!("cadeau:{}", transaction.id))
    .fetch_optional(pool.get_ref())
    .await
    .ok()
    .flatten()
    .unwrap_or(0);

    let offreur = charger_membre_bref(pool.get_ref(), transaction.offreur_id)
        .await
        .map(|m| m.nom_affiche)
        .unwrap_or_else(|_| "Un membre".to_string());

    let libelle_cadeau: String =
        sqlx::query_scalar("SELECT libelle FROM engagement.cadeau WHERE id = $1")
            .bind(transaction.cadeau_id)
            .fetch_optional(pool.get_ref())
            .await
            .ok()
            .flatten()
            .unwrap_or_else(|| "un cadeau".to_string());

    notification::creer_notification(
        pool.get_ref(),
        transaction.beneficiaire_id,
        notification::engagement::CADEAU_RECU,
        &format!("{offreur} vous a offert « {libelle_cadeau} » (+{points_credites} points)."),
        Some(notification::engagement::LIEN_ESPACE),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(ConfirmationResponse {
            transaction_id: transaction.id,
            etat: "abouti".to_string(),
            points_credites,
            beneficiaire,
        }),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// 4. CADEAUX REÇUS PAR UN CONTENU
// ════════════════════════════════════════════════════════════════════════════

/// GET /api/engagement/cadeaux/{type_objet}/{objet_id}
///
/// **Aucun montant en argent n'est exposé** : le public voit ce qui a été
/// offert, jamais ce que cela a coûté.
pub async fn cadeaux_contenu(
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    let (type_objet, objet_id) = chemin.into_inner();

    let resume = sqlx::query_as::<_, ResumeCadeau>(
        "SELECT c.code, c.libelle, c.icone, c.couleur, COUNT(*) AS nombre
           FROM engagement.transaction_cadeau t
           JOIN engagement.cadeau c ON c.id = t.cadeau_id
          WHERE t.type_objet = $1 AND t.objet_id = $2 AND t.etat = 'abouti'
          GROUP BY c.code, c.libelle, c.icone, c.couleur, c.ordre
          ORDER BY nombre DESC, c.ordre",
    )
    .bind(&type_objet)
    .bind(objet_id)
    .fetch_all(pool.get_ref())
    .await?;

    let total: i64 = resume.iter().map(|r| r.nombre).sum();

    let lignes = sqlx::query_as::<_, (Uuid, String, String, String, Option<String>, Option<String>, Option<String>, chrono::DateTime<Utc>)>(
        &format!(
            "SELECT u.id, {NOM_AFFICHE} AS nom_affiche, c.code, c.libelle, c.icone, c.couleur,
                    t.message, t.created_at
               FROM engagement.transaction_cadeau t
               JOIN engagement.cadeau c ON c.id = t.cadeau_id
               JOIN iam.utilisateur u ON u.id = t.offreur_id
              WHERE t.type_objet = $1 AND t.objet_id = $2 AND t.etat = 'abouti'
              ORDER BY t.created_at DESC LIMIT 10"
        ),
    )
    .bind(&type_objet)
    .bind(objet_id)
    .fetch_all(pool.get_ref())
    .await?;

    let derniers = lignes
        .into_iter()
        .map(
            |(id, nom_affiche, code, libelle, icone, couleur, message, created_at)| {
                CadeauOffertPublic {
                    offreur: MembreBref { id, nom_affiche },
                    cadeau: CadeauBref { code, libelle, icone, couleur },
                    message,
                    created_at,
                }
            },
        )
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(CadeauxContenuResponse { total, resume, derniers }),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// 5. MES CADEAUX
// ════════════════════════════════════════════════════════════════════════════

/// GET /api/engagement/mes-cadeaux?sens=recus|offerts
pub async fn mes_cadeaux(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    params: web::Query<MesCadeauxQuery>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = exiger_utilisateur_id(&req)?;

    let offerts = params.sens.as_deref() == Some("offerts");
    let page = params.page.unwrap_or(1).max(1);
    let taille = params.taille.unwrap_or(20).clamp(1, 100);

    // Colonne de filtre et colonne de contrepartie sont symétriques : le sens de
    // lecture inverse simplement les deux extrémités de la transaction.
    let (colonne_moi, colonne_autre) = if offerts {
        ("t.offreur_id", "t.beneficiaire_id")
    } else {
        ("t.beneficiaire_id", "t.offreur_id")
    };

    let total: i64 = sqlx::query_scalar(&format!(
        "SELECT COUNT(*) FROM engagement.transaction_cadeau t
          WHERE {colonne_moi} = $1 AND t.etat = 'abouti'"
    ))
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    let lignes = sqlx::query_as::<_, (
        Uuid, String, String, Option<String>, Option<String>,
        Uuid, String, String, Uuid, i32, String, i32, Option<String>, bool,
        chrono::DateTime<Utc>,
    )>(&format!(
        "SELECT t.id, c.code, c.libelle, c.icone, c.couleur,
                u.id, {NOM_AFFICHE} AS nom_affiche,
                t.type_objet, t.objet_id, t.points, t.mode::text, t.montant,
                t.message, t.simule, t.created_at
           FROM engagement.transaction_cadeau t
           JOIN engagement.cadeau c ON c.id = t.cadeau_id
           JOIN iam.utilisateur u ON u.id = {colonne_autre}
          WHERE {colonne_moi} = $1 AND t.etat = 'abouti'
          ORDER BY t.created_at DESC
          LIMIT $2 OFFSET $3"
    ))
    .bind(utilisateur_id)
    .bind(taille)
    .bind((page - 1) * taille)
    .fetch_all(pool.get_ref())
    .await?;

    let mut elements = Vec::with_capacity(lignes.len());
    for (
        id,
        code,
        libelle,
        icone,
        couleur,
        autre_id,
        nom_affiche,
        type_objet,
        objet_id,
        points,
        mode,
        montant,
        message,
        simule,
        created_at,
    ) in lignes
    {
        // Sans le titre, trois cadeaux reçus le même jour sur trois contenus
        // différents seraient indiscernables. `null` pour un cadeau au profil :
        // le nom de la contrepartie tient déjà ce rôle.
        let titre_cible = resoudre_titre(pool.get_ref(), &type_objet, objet_id).await;

        elements.push(MonCadeauResponse {
            id,
            cadeau: CadeauBref { code, libelle, icone, couleur },
            contrepartie: MembreBref { id: autre_id, nom_affiche },
            type_objet,
            objet_id,
            titre_cible,
            points,
            mode,
            // L'offreur a le droit de savoir ce qu'il a dépensé ; le
            // bénéficiaire ne voit jamais le prix ligne à ligne, seulement le
            // cumul de sa cagnotte.
            montant: offerts.then_some(montant),
            message,
            simule,
            created_at,
        });
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(MesCadeauxPage { elements, total, page, taille }),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// 6. MA CAGNOTTE
// ════════════════════════════════════════════════════════════════════════════

/// GET /api/engagement/ma-cagnotte
pub async fn ma_cagnotte(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = exiger_utilisateur_id(&req)?;
    let params = charger_parametres(pool.get_ref()).await?;

    let cagnotte = sqlx::query_as::<_, (i32, i32)>(
        "SELECT montant_cumule, montant_verse FROM engagement.cagnotte WHERE utilisateur_id = $1",
    )
    .bind(utilisateur_id)
    .fetch_optional(pool.get_ref())
    .await?
    .unwrap_or((0, 0));

    // Ce que la purge de fin de phase de test retirera. L'exposer au membre est
    // ce qui empêche la purge d'être une mauvaise surprise.
    let part_simulee: i64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(part_beneficiaire), 0)::bigint
           FROM engagement.transaction_cadeau
          WHERE beneficiaire_id = $1 AND etat = 'abouti' AND simule = TRUE",
    )
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(CagnotteResponse {
            montant_cumule: cagnotte.0,
            montant_verse: cagnotte.1,
            devise: params.devise,
            // Aucun versement dans cette itération : la mention doit être
            // explicite côté membre, pas seulement absente.
            versement_disponible: false,
            part_simulee: part_simulee as i32,
        }),
        error: None,
    }))
}
