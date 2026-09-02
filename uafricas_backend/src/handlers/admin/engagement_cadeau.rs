//! Administration des cadeaux virtuels (permission `engagement.gerer`).
//!
//! - CRUD du catalogue
//! - Journal comptable filtrable avec totaux calculés sur le filtre
//! - Paramètres de monétisation (taux, devise, bascule paiement réel)
//! - Purge de fin de phase de test
//!
//! Toute mutation est auditée (`log_action`). Les transactions de cadeaux ont
//! leur **propre** journal métier : l'audit ne doublonne pas la comptabilité, il
//! trace qui a changé les règles du jeu.

use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::engagement_cadeau::*;
use crate::services::audit;
use crate::verifier_permission;
use crate::ApiResponse;

// ════════════════════════════════════════════════════════════════════════════
// CATALOGUE : CRUD
// ════════════════════════════════════════════════════════════════════════════

/// GET /api/admin/engagement/cadeaux : catalogue complet (actifs et inactifs).
///
/// `nombre_envois` conditionne l'affichage du bouton de suppression côté
/// interface : au-delà de zéro, seule la désactivation reste possible.
pub async fn lister_cadeaux(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");

    let cadeaux = sqlx::query_as::<_, CadeauAdmin>(
        "SELECT c.id, c.code, c.libelle, c.description, c.icone, c.couleur,
                c.prix, c.points, c.ordre, c.actif, c.created_at, c.updated_at,
                COALESCE(t.nombre, 0) AS nombre_envois,
                COALESCE(t.montant, 0) AS montant_collecte
           FROM engagement.cadeau c
           LEFT JOIN (
                SELECT cadeau_id, COUNT(*) AS nombre, SUM(montant)::bigint AS montant
                  FROM engagement.transaction_cadeau
                 WHERE etat = 'abouti'
                 GROUP BY cadeau_id
           ) t ON t.cadeau_id = c.id
          ORDER BY c.ordre, c.points DESC",
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(cadeaux), error: None }))
}

fn valider_cadeau(payload: &CadeauPayload) -> Result<(), ApiErreur> {
    if payload.prix <= 0 {
        return Err(ApiErreur::Validation("Le prix doit être strictement positif.".to_string()));
    }
    if payload.points <= 0 {
        return Err(ApiErreur::Validation(
            "Le nombre de points doit être strictement positif.".to_string(),
        ));
    }
    if payload.libelle.trim().is_empty() {
        return Err(ApiErreur::Validation("Le libellé est obligatoire.".to_string()));
    }
    Ok(())
}

/// POST /api/admin/engagement/cadeaux : créer.
/// Immédiatement visible par les membres si `actif` : aucun redémarrage.
pub async fn creer_cadeau(
    req: HttpRequest,
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    body: web::Json<CadeauPayload>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");
    valider_cadeau(&body)?;

    let code = body
        .code
        .as_deref()
        .map(str::trim)
        .filter(|c| !c.is_empty())
        .ok_or_else(|| ApiErreur::Validation("Le code est obligatoire.".to_string()))?;

    let existe: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM engagement.cadeau WHERE code = $1)")
            .bind(code)
            .fetch_one(pool.get_ref())
            .await?;
    if existe {
        return Err(ApiErreur::Conflit(format!("Le code « {code} » est déjà utilisé.")));
    }

    let cadeau = sqlx::query_as::<_, Cadeau>(&format!(
        "INSERT INTO engagement.cadeau
           (code, libelle, description, icone, couleur, prix, points, ordre, actif)
         VALUES ($1, $2, $3, $4, $5, $6, $7, COALESCE($8, 0), COALESCE($9, TRUE))
         RETURNING {COLONNES_CADEAU}"
    ))
    .bind(code)
    .bind(body.libelle.trim())
    .bind(body.description.as_deref())
    .bind(body.icone.as_deref())
    .bind(body.couleur.as_deref())
    .bind(body.prix)
    .bind(body.points)
    .bind(body.ordre)
    .bind(body.actif)
    .fetch_one(pool.get_ref())
    .await?;

    auditer(&req, pool.get_ref(), admin.id, "CREATION", cadeau.id, None).await;

    Ok(HttpResponse::Created().json(ApiResponse { success: true, data: Some(cadeau), error: None }))
}

/// PUT /api/admin/engagement/cadeaux/{id}, modifier.
///
/// `code` n'est jamais modifié : c'est une clé stable. Une modification de
/// `prix` ou de `points` **n'affecte aucune transaction passée**, celles-ci
/// portent leurs propres valeurs figées.
pub async fn modifier_cadeau(
    req: HttpRequest,
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<CadeauPayload>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");
    valider_cadeau(&body)?;
    let id = chemin.into_inner();

    let avant = sqlx::query_as::<_, Cadeau>(&format!(
        "SELECT {COLONNES_CADEAU} FROM engagement.cadeau WHERE id = $1"
    ))
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Cadeau introuvable".to_string()))?;

    let cadeau = sqlx::query_as::<_, Cadeau>(&format!(
        "UPDATE engagement.cadeau
            SET libelle = $2, description = $3, icone = $4, couleur = $5,
                prix = $6, points = $7, ordre = COALESCE($8, ordre),
                actif = COALESCE($9, actif), updated_at = NOW()
          WHERE id = $1
      RETURNING {COLONNES_CADEAU}"
    ))
    .bind(id)
    .bind(body.libelle.trim())
    .bind(body.description.as_deref())
    .bind(body.icone.as_deref())
    .bind(body.couleur.as_deref())
    .bind(body.prix)
    .bind(body.points)
    .bind(body.ordre)
    .bind(body.actif)
    .fetch_one(pool.get_ref())
    .await?;

    auditer(
        &req,
        pool.get_ref(),
        admin.id,
        "MODIFICATION",
        id,
        Some(serde_json::json!({
            "avant": { "libelle": avant.libelle, "prix": avant.prix,
                       "points": avant.points, "actif": avant.actif },
            "apres": { "libelle": cadeau.libelle, "prix": cadeau.prix,
                       "points": cadeau.points, "actif": cadeau.actif },
        })),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(cadeau), error: None }))
}

/// DELETE /api/admin/engagement/cadeaux/{id}
///
/// Suppression réelle uniquement si le cadeau n'a jamais été offert. Sinon
/// `409` : la contrainte `ON DELETE RESTRICT` rend l'erreur **structurelle**,
/// donc même une requête mal écrite ne peut pas casser l'historique.
pub async fn supprimer_cadeau(
    req: HttpRequest,
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");
    let id = chemin.into_inner();

    let envois: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM engagement.transaction_cadeau WHERE cadeau_id = $1",
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;

    if envois > 0 {
        return Err(ApiErreur::Conflit(format!(
            "Ce cadeau a déjà été offert {envois} fois : il ne peut plus être supprimé. \
             Désactivez-le pour le retirer du catalogue sans altérer l'historique."
        )));
    }

    let res = sqlx::query("DELETE FROM engagement.cadeau WHERE id = $1")
        .bind(id)
        .execute(pool.get_ref())
        .await?;

    if res.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Cadeau introuvable".to_string()));
    }

    auditer(&req, pool.get_ref(), admin.id, "SUPPRESSION", id, None).await;

    Ok(HttpResponse::NoContent().finish())
}

async fn auditer(
    req: &HttpRequest,
    pool: &PgPool,
    admin_id: Uuid,
    action: &str,
    cible: Uuid,
    details: Option<serde_json::Value>,
) {
    let ip = audit::extraire_ip(req);
    let ua = audit::extraire_user_agent(req);
    audit::log_action(
        pool,
        Some(admin_id),
        action,
        "engagement",
        "cadeau",
        Some(cible),
        None,
        details,
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;
}

// ════════════════════════════════════════════════════════════════════════════
// JOURNAL COMPTABLE
// ════════════════════════════════════════════════════════════════════════════

/// GET /api/admin/engagement/transactions, journal filtrable et paginé.
///
/// Les `totaux` sont calculés **sur le filtre courant**, pas sur la page, et ne
/// comptent que `etat = 'abouti'`. Invariant vérifiable en recette :
/// `recettes_plateforme + cagnottes_dues = montant_total`.
pub async fn lister_transactions(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<JournalAdminQuery>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");

    let page = params.page.unwrap_or(1).max(1);
    let taille = params.taille.unwrap_or(25).clamp(1, 100);

    // Filtres neutralisés par cast paramétré (`$n IS NULL OR …`), jamais de
    // concaténation de fragments SQL.
    let filtre = "($1::uuid IS NULL
                   OR ($2::text = 'offreur'      AND t.offreur_id = $1)
                   OR ($2::text = 'beneficiaire' AND t.beneficiaire_id = $1)
                   OR ($2::text IS NULL AND (t.offreur_id = $1 OR t.beneficiaire_id = $1)))
                  AND ($3::text IS NULL OR t.etat::text = $3)
                  AND ($4::text IS NULL OR t.mode::text = $4)
                  AND ($5::boolean IS NULL OR t.simule = $5)
                  AND ($6::timestamptz IS NULL OR t.created_at >= $6)
                  AND ($7::timestamptz IS NULL OR t.created_at <= $7)";

    let total: i64 = sqlx::query_scalar(&format!(
        "SELECT COUNT(*) FROM engagement.transaction_cadeau t WHERE {filtre}"
    ))
    .bind(params.membre_id)
    .bind(params.sens.as_deref())
    .bind(params.etat.as_deref())
    .bind(params.mode.as_deref())
    .bind(params.simule)
    .bind(params.debut)
    .bind(params.fin)
    .fetch_one(pool.get_ref())
    .await?;

    let totaux = sqlx::query_as::<_, TotauxJournal>(&format!(
        "SELECT COALESCE(SUM(t.montant) FILTER (WHERE t.etat = 'abouti'), 0)::bigint AS montant_total,
                COALESCE(SUM(t.part_plateforme) FILTER (WHERE t.etat = 'abouti'), 0)::bigint AS recettes_plateforme,
                COALESCE(SUM(t.part_beneficiaire) FILTER (WHERE t.etat = 'abouti'), 0)::bigint AS cagnottes_dues,
                COUNT(*) FILTER (WHERE t.etat = 'abouti')::bigint AS nombre_abouti,
                COUNT(*) FILTER (WHERE t.etat = 'abouti' AND t.simule)::bigint AS nombre_simule
           FROM engagement.transaction_cadeau t WHERE {filtre}"
    ))
    .bind(params.membre_id)
    .bind(params.sens.as_deref())
    .bind(params.etat.as_deref())
    .bind(params.mode.as_deref())
    .bind(params.simule)
    .bind(params.debut)
    .bind(params.fin)
    .fetch_one(pool.get_ref())
    .await?;

    let lignes = sqlx::query(&format!(
        "SELECT t.id, t.created_at, t.finalise_at,
                o.id AS offreur_id, TRIM(COALESCE(o.prenom, '') || ' ' || COALESCE(o.nom, '')) AS offreur_nom,
                b.id AS beneficiaire_id, TRIM(COALESCE(b.prenom, '') || ' ' || COALESCE(b.nom, '')) AS beneficiaire_nom,
                t.type_objet, t.objet_id,
                c.code, c.libelle, c.icone, c.couleur,
                t.mode::text AS mode, t.montant, t.points, t.taux_commission,
                t.part_beneficiaire, t.part_plateforme, t.etat::text AS etat,
                t.simule, t.reference_paiement
           FROM engagement.transaction_cadeau t
           JOIN engagement.cadeau c ON c.id = t.cadeau_id
           JOIN iam.utilisateur o ON o.id = t.offreur_id
           JOIN iam.utilisateur b ON b.id = t.beneficiaire_id
          WHERE {filtre}
          ORDER BY t.created_at DESC
          LIMIT $8 OFFSET $9"
    ))
    .bind(params.membre_id)
    .bind(params.sens.as_deref())
    .bind(params.etat.as_deref())
    .bind(params.mode.as_deref())
    .bind(params.simule)
    .bind(params.debut)
    .bind(params.fin)
    .bind(taille)
    .bind((page - 1) * taille)
    .fetch_all(pool.get_ref())
    .await?;

    let mut elements = Vec::with_capacity(lignes.len());
    for l in lignes {
        let type_objet: String = l.get("type_objet");
        let objet_id: Uuid = l.get("objet_id");
        let titre = crate::handlers::engagement_cadeau::resoudre_titre(
            pool.get_ref(),
            &type_objet,
            objet_id,
        )
        .await;

        elements.push(LigneJournalAdmin {
            id: l.get("id"),
            created_at: l.get("created_at"),
            finalise_at: l.get("finalise_at"),
            offreur: MembreBref { id: l.get("offreur_id"), nom_affiche: l.get("offreur_nom") },
            beneficiaire: MembreBref {
                id: l.get("beneficiaire_id"),
                nom_affiche: l.get("beneficiaire_nom"),
            },
            cible: CibleTransaction { type_objet, objet_id, titre },
            cadeau: CadeauBref {
                code: l.get("code"),
                libelle: l.get("libelle"),
                icone: l.get("icone"),
                couleur: l.get("couleur"),
            },
            mode: l.get("mode"),
            montant: l.get("montant"),
            points: l.get("points"),
            taux_commission: l.get("taux_commission"),
            part_beneficiaire: l.get("part_beneficiaire"),
            part_plateforme: l.get("part_plateforme"),
            etat: l.get("etat"),
            simule: l.get("simule"),
            reference_paiement: l.get("reference_paiement"),
        });
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(JournalAdminPage {
            elements,
            pagination: PaginationInfo { page, taille, total },
            totaux,
        }),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════════════════
// PARAMÈTRES DE MONÉTISATION
// ════════════════════════════════════════════════════════════════════════════

/// GET /api/admin/engagement/parametres-monetisation
pub async fn obtenir_parametres(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");

    let params = sqlx::query_as::<_, ParametreMonetisation>(
        "SELECT taux_commission, devise, paiement_reel_actif, updated_at
           FROM engagement.parametre_monetisation WHERE id = TRUE",
    )
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| {
        ApiErreur::NonTrouve("Paramètres de monétisation absents : migration 35g manquante ?".into())
    })?;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(params), error: None }))
}

/// PUT /api/admin/engagement/parametres-monetisation
///
/// La modification du taux est **prospective** : les transactions passées
/// conservent leur taux figé. Rien à faire pour cela : c'est la conséquence du
/// gel à l'écriture, pas d'un traitement particulier ici.
pub async fn modifier_parametres(
    req: HttpRequest,
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    body: web::Json<ParametreMonetisationPayload>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");

    if !(0..=100).contains(&body.taux_commission) {
        return Err(ApiErreur::Validation(
            "Le taux de commission doit être compris entre 0 et 100.".to_string(),
        ));
    }
    let devise = body.devise.trim().to_uppercase();
    if devise.len() != 3 {
        return Err(ApiErreur::Validation(
            "La devise doit être un code ISO de 3 lettres (XOF, EUR…).".to_string(),
        ));
    }

    let avant = sqlx::query_as::<_, ParametreMonetisation>(
        "SELECT taux_commission, devise, paiement_reel_actif, updated_at
           FROM engagement.parametre_monetisation WHERE id = TRUE",
    )
    .fetch_optional(pool.get_ref())
    .await?;

    let params = sqlx::query_as::<_, ParametreMonetisation>(
        "INSERT INTO engagement.parametre_monetisation
           (id, taux_commission, devise, paiement_reel_actif, updated_at)
         VALUES (TRUE, $1, $2, $3, NOW())
         ON CONFLICT (id) DO UPDATE
            SET taux_commission = EXCLUDED.taux_commission,
                devise = EXCLUDED.devise,
                paiement_reel_actif = EXCLUDED.paiement_reel_actif,
                updated_at = NOW()
         RETURNING taux_commission, devise, paiement_reel_actif, updated_at",
    )
    .bind(body.taux_commission)
    .bind(&devise)
    .bind(body.paiement_reel_actif)
    .fetch_one(pool.get_ref())
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "MODIFICATION",
        "engagement",
        "parametre_monetisation",
        None,
        None,
        Some(serde_json::json!({
            "avant": avant.map(|a| serde_json::json!({
                "taux_commission": a.taux_commission,
                "devise": a.devise,
                "paiement_reel_actif": a.paiement_reel_actif,
            })),
            "apres": {
                "taux_commission": params.taux_commission,
                "devise": params.devise,
                "paiement_reel_actif": params.paiement_reel_actif,
            },
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(params), error: None }))
}

// ════════════════════════════════════════════════════════════════════════════
// PURGE DE FIN DE PHASE DE TEST
// ════════════════════════════════════════════════════════════════════════════

/// POST /api/admin/engagement/purger-phase-test
///
/// Retire 100 % des points issus de cadeaux **simulés**, sans toucher aux points
/// de j'aime et de partage. La suppression est ciblée par le **motif de clé**
/// `cadeau:{id}`, jamais par une plage de dates.
///
/// Le solde est **recalculé depuis le journal restant**, jamais décrémenté :
/// soustraire les points supprimés dériverait dès qu'un plafond a écrêté un
/// mouvement.
///
/// C'est la **seule** entorse à l'immuabilité du journal de points. Elle est
/// bornée à une opération unique, tracée, et justifiée par le fait que ces
/// points n'auraient jamais dû exister hors phase de test.
pub async fn purger_phase_test(
    req: HttpRequest,
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    body: web::Json<PurgeRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");

    if body.confirmation.trim() != CONFIRMATION_PURGE {
        return Err(ApiErreur::Validation(format!(
            "Confirmation attendue : « {CONFIRMATION_PURGE} »."
        )));
    }

    // Précondition : purger tant que le paiement reste simulé rouvrirait
    // aussitôt la porte au minage : les membres pourraient regagner
    // gratuitement ce qui vient d'être retiré.
    let paiement_reel: bool = sqlx::query_scalar(
        "SELECT paiement_reel_actif FROM engagement.parametre_monetisation WHERE id = TRUE",
    )
    .fetch_optional(pool.get_ref())
    .await?
    .unwrap_or(false);

    if !paiement_reel {
        return Err(ApiErreur::Conflit(
            "La purge n'est possible qu'une fois l'encaissement réel activé : \
             purger avant rouvrirait immédiatement la porte aux points gratuits."
                .to_string(),
        ));
    }

    let mut tx = pool.begin().await?;

    // 1. Transactions concernées : simulées ET abouties, jamais les autres.
    let cibles = sqlx::query_as::<_, (Uuid, Uuid, i32)>(
        "SELECT id, beneficiaire_id, part_beneficiaire
           FROM engagement.transaction_cadeau
          WHERE simule = TRUE AND etat = 'abouti'
          FOR UPDATE",
    )
    .fetch_all(&mut *tx)
    .await?;

    if cibles.is_empty() {
        tx.rollback().await?;
        return Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(ResultatPurge {
                transactions_purgees: 0,
                mouvements_supprimes: 0,
                comptes_recalcules: 0,
                montant_cagnottes_annule: 0,
            }),
            error: None,
        }));
    }

    let ids: Vec<Uuid> = cibles.iter().map(|(id, _, _)| *id).collect();
    let cles: Vec<String> = ids.iter().map(|id| format!("cadeau:{id}")).collect();

    // 2. Suppression ciblée par MOTIF DE CLÉ. Aucun mouvement `jaime_recu` ni
    //    `partage_recu` ne peut être touché : leurs clés ne portent pas ce motif.
    let suppression = sqlx::query(
        "DELETE FROM engagement.mouvement_points WHERE cle_idempotence = ANY($1)",
    )
    .bind(&cles)
    .execute(&mut *tx)
    .await?;

    // 3. Recalcul intégral des soldes DEPUIS LE JOURNAL RESTANT, puis des statuts.
    let recalcul = sqlx::query(
        "WITH sommes AS (
             SELECT c.utilisateur_id,
                    GREATEST(COALESCE((SELECT SUM(m.points) FROM engagement.mouvement_points m
                                        WHERE m.utilisateur_id = c.utilisateur_id), 0), 0)::integer AS solde,
                    COALESCE((SELECT SUM(m.points) FROM engagement.mouvement_points m
                               WHERE m.utilisateur_id = c.utilisateur_id
                                 AND date_trunc('month', m.created_at) = date_trunc('month', NOW())), 0)::integer AS mensuel
               FROM engagement.compte c
         )
         UPDATE engagement.compte c
            SET solde_points = s.solde,
                solde_points_mensuel = s.mensuel,
                niveau_code = COALESCE(
                    (SELECT n.code FROM engagement.niveau n
                      WHERE n.seuil_min <= s.solde ORDER BY n.seuil_min DESC LIMIT 1),
                    'membre'),
                updated_at = NOW()
           FROM sommes s
          WHERE c.utilisateur_id = s.utilisateur_id
            AND (c.solde_points IS DISTINCT FROM s.solde
                 OR c.solde_points_mensuel IS DISTINCT FROM s.mensuel)",
    )
    .execute(&mut *tx)
    .await?;

    // 4. Réduction des cagnottes du montant des parts purgées.
    let montant_annule: i64 = cibles.iter().map(|(_, _, part)| *part as i64).sum();
    for (_, beneficiaire_id, part) in &cibles {
        if *part > 0 {
            sqlx::query(
                "UPDATE engagement.cagnotte
                    SET montant_cumule = GREATEST(montant_cumule - $2, 0), updated_at = NOW()
                  WHERE utilisateur_id = $1",
            )
            .bind(beneficiaire_id)
            .bind(part)
            .execute(&mut *tx)
            .await?;
        }
    }

    // 5. Marquage, JAMAIS suppression : la ligne reste lisible dans l'historique.
    sqlx::query(
        "UPDATE engagement.transaction_cadeau
            SET etat = 'purge', finalise_at = COALESCE(finalise_at, NOW())
          WHERE id = ANY($1)",
    )
    .bind(&ids)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    let resultat = ResultatPurge {
        transactions_purgees: ids.len() as i64,
        mouvements_supprimes: suppression.rows_affected() as i64,
        comptes_recalcules: recalcul.rows_affected() as i64,
        montant_cagnottes_annule: montant_annule,
    };

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "PURGE",
        "engagement",
        "transaction_cadeau",
        None,
        None,
        Some(serde_json::json!({
            "transactions_purgees": resultat.transactions_purgees,
            "mouvements_supprimes": resultat.mouvements_supprimes,
            "comptes_recalcules": resultat.comptes_recalcules,
            "montant_cagnottes_annule": resultat.montant_cagnottes_annule,
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(resultat), error: None }))
}
