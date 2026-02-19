use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::dashboard::{
    ActiviteRecenteItem, StatsRow, TendancePoint, TendancesQueryParams, TendancesResponse,
};
use crate::ApiResponse;

/// Requete CTE unique pour agreger toutes les stats du dashboard
const STATS_SQL: &str = "
WITH
  u_stats AS (
    SELECT
      COUNT(*) as total,
      COUNT(*) FILTER (WHERE etat::text = 'actif') as actifs,
      COUNT(*) FILTER (WHERE etat::text = 'en_attente') as en_attente,
      COUNT(*) FILTER (WHERE etat::text = 'suspendu') as suspendus
    FROM iam.utilisateur WHERE deleted_at IS NULL
  ),
  org_stats AS (
    SELECT COUNT(*) as total
    FROM iam.organisation WHERE deleted_at IS NULL
  ),
  ann_stats AS (
    SELECT
      COUNT(*) as total,
      COUNT(*) FILTER (WHERE etat::text = 'publiee') as publiees,
      COUNT(*) FILTER (WHERE etat::text = 'en_attente') as en_attente,
      COUNT(*) FILTER (WHERE etat::text = 'expiree') as expirees
    FROM marketplace.annonce WHERE deleted_at IS NULL
  ),
  prog_stats AS (
    SELECT
      COUNT(*) as total,
      COUNT(*) FILTER (WHERE etat::text IN ('publie', 'en_cours')) as actifs
    FROM exchange.programme WHERE deleted_at IS NULL
  ),
  cand_stats AS (
    SELECT COUNT(*) FILTER (WHERE statut::text IN ('soumise', 'en_revue')) as en_attente
    FROM exchange.candidature
  ),
  innov_stats AS (
    SELECT
      COUNT(*) as total,
      COUNT(*) FILTER (WHERE etat::text = 'publie') as publiees
    FROM innovation.innovation WHERE deleted_at IS NULL
  ),
  proj_stats AS (
    SELECT
      COUNT(*) as total,
      COUNT(*) FILTER (WHERE etat::text = 'approuve') as approuves,
      COUNT(*) FILTER (WHERE etat::text = 'en_revue') as en_revue,
      COUNT(*) FILTER (WHERE etat::text = 'soumis') as soumis
    FROM innovation.projet WHERE deleted_at IS NULL
  ),
  afri_stats AS (
    SELECT COUNT(*) as total
    FROM innovation.africantive WHERE deleted_at IS NULL
  ),
  cc_stats AS (
    SELECT COUNT(*) as total
    FROM culture.centre_culturel WHERE deleted_at IS NULL
  ),
  codi_stats AS (
    SELECT
      COUNT(*) as total,
      COUNT(*) FILTER (WHERE type::text = 'proverbe_adage') as proverbe_adage,
      COUNT(*) FILTER (WHERE type::text = 'citation') as citation,
      COUNT(*) FILTER (WHERE type::text = 'ressource_historique') as ressource_historique,
      COUNT(*) FILTER (WHERE type::text = 'bonne_pratique') as bonne_pratique
    FROM culture.codimoi WHERE deleted_at IS NULL
  ),
  sess_stats AS (
    SELECT
      COUNT(*) as total,
      COUNT(*) FILTER (WHERE etat::text = 'en_cours') as en_cours
    FROM afrolang.session
  ),
  evt_stats AS (
    SELECT
      COUNT(*) as total,
      COUNT(*) FILTER (WHERE etat::text = 'publie' AND date_debut > NOW()) as a_venir
    FROM media_content.evenement WHERE deleted_at IS NULL
  ),
  evt_insc AS (
    SELECT COUNT(*) as total
    FROM media_content.evenement_inscription
  ),
  mooc_stats AS (
    SELECT
      COUNT(*) as total,
      COUNT(*) FILTER (WHERE etat::text = 'en_cours') as en_cours
    FROM media_content.mooc WHERE deleted_at IS NULL
  ),
  mooc_insc AS (
    SELECT COUNT(*) as total
    FROM media_content.mooc_inscription
  ),
  livre_stats AS (
    SELECT COUNT(*) as total
    FROM media_content.livre WHERE deleted_at IS NULL
  ),
  radio_stats AS (
    SELECT COUNT(*) as total
    FROM media_content.station_radio WHERE deleted_at IS NULL
  ),
  tv_stats AS (
    SELECT COUNT(*) as total
    FROM media_content.chaine_tv WHERE deleted_at IS NULL
  ),
  fc_stats AS (
    SELECT
      COUNT(*) as total,
      COUNT(*) FILTER (WHERE verdict::text = 'vrai') as vrai,
      COUNT(*) FILTER (WHERE verdict::text = 'faux') as faux,
      COUNT(*) FILTER (WHERE verdict::text = 'partiellement_vrai') as partiellement_vrai,
      COUNT(*) FILTER (WHERE verdict::text = 'trompeur') as trompeur,
      COUNT(*) FILTER (WHERE verdict::text = 'non_verifie') as non_verifie
    FROM governance.factcheck WHERE deleted_at IS NULL
  ),
  bh_stats AS (
    SELECT
      COUNT(*) as total,
      COUNT(*) FILTER (WHERE gravite::text = 'faible') as faible,
      COUNT(*) FILTER (WHERE gravite::text = 'elevee') as elevee,
      COUNT(*) FILTER (WHERE gravite::text = 'critique') as critique
    FROM governance.bad_habit WHERE deleted_at IS NULL
  ),
  if_stats AS (
    SELECT COUNT(*) as total
    FROM governance.idea_force WHERE deleted_at IS NULL
  ),
  fp_stats AS (
    SELECT COUNT(*) as total
    FROM country_profile.fiche_pays
  ),
  contrib_stats AS (
    SELECT COUNT(*) FILTER (WHERE etat::text = 'en_attente') as en_attente
    FROM country_profile.contribution_fiche WHERE deleted_at IS NULL
  ),
  audit_stats AS (
    SELECT
      COUNT(*) FILTER (WHERE created_at >= CURRENT_DATE) as aujourd_hui,
      COUNT(*) FILTER (WHERE created_at >= CURRENT_DATE - INTERVAL '7 days') as cette_semaine
    FROM shared.audit_log
  )
SELECT
  u_stats.total       as utilisateurs_total,
  u_stats.actifs      as utilisateurs_actifs,
  u_stats.en_attente  as utilisateurs_en_attente,
  u_stats.suspendus   as utilisateurs_suspendus,
  org_stats.total     as organisations_total,
  ann_stats.total     as annonces_total,
  ann_stats.publiees  as annonces_publiees,
  ann_stats.en_attente as annonces_en_attente,
  ann_stats.expirees  as annonces_expirees,
  prog_stats.total    as programmes_total,
  prog_stats.actifs   as programmes_actifs,
  cand_stats.en_attente as candidatures_en_attente,
  innov_stats.total   as innovations_total,
  innov_stats.publiees as innovations_publiees,
  proj_stats.total    as projets_total,
  proj_stats.approuves as projets_approuves,
  proj_stats.en_revue as projets_en_revue,
  proj_stats.soumis   as projets_soumis,
  afri_stats.total    as africantives_total,
  cc_stats.total      as centres_culturels_total,
  codi_stats.total    as codimoi_total,
  codi_stats.proverbe_adage as codimoi_proverbe_adage,
  codi_stats.citation as codimoi_citation,
  codi_stats.ressource_historique as codimoi_ressource_historique,
  codi_stats.bonne_pratique as codimoi_bonne_pratique,
  sess_stats.total    as sessions_total,
  sess_stats.en_cours as sessions_en_cours,
  evt_stats.total     as evenements_total,
  evt_stats.a_venir   as evenements_a_venir,
  evt_insc.total      as evenements_inscrits_total,
  mooc_stats.total    as moocs_total,
  mooc_insc.total     as moocs_inscrits_total,
  mooc_stats.en_cours as moocs_en_cours,
  livre_stats.total   as livres_total,
  radio_stats.total   as stations_radio,
  tv_stats.total      as chaines_tv,
  fc_stats.total      as factchecks_total,
  fc_stats.vrai       as factchecks_vrai,
  fc_stats.faux       as factchecks_faux,
  fc_stats.partiellement_vrai as factchecks_partiellement_vrai,
  fc_stats.trompeur   as factchecks_trompeur,
  fc_stats.non_verifie as factchecks_non_verifie,
  bh_stats.total      as bad_habits_total,
  bh_stats.faible     as bad_habits_faible,
  bh_stats.elevee     as bad_habits_elevee,
  bh_stats.critique   as bad_habits_critique,
  if_stats.total      as idea_forces_total,
  fp_stats.total      as fiches_pays_total,
  contrib_stats.en_attente as contributions_en_attente,
  audit_stats.aujourd_hui as audit_actions_aujourd_hui,
  audit_stats.cette_semaine as audit_actions_cette_semaine
FROM u_stats, org_stats, ann_stats, prog_stats, cand_stats, innov_stats,
     proj_stats, afri_stats, cc_stats, codi_stats, sess_stats, evt_stats,
     evt_insc, mooc_stats, mooc_insc, livre_stats, radio_stats, tv_stats,
     fc_stats, bh_stats, if_stats, fp_stats, contrib_stats, audit_stats
";

/// GET /api/admin/dashboard/stats
pub async fn obtenir_stats(
    _admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let row = sqlx::query_as::<_, StatsRow>(STATS_SQL)
        .fetch_one(pool.get_ref())
        .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_dashboard_stats()),
        error: None,
    }))
}

/// GET /api/admin/dashboard/activite-recente
pub async fn lister_activite_recente(
    _admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let items = sqlx::query_as::<_, ActiviteRecenteItem>(
        "SELECT
           a.id, a.action, a.schema_name, a.table_name, a.record_id,
           a.created_at,
           u.nom || ' ' || u.prenom AS utilisateur_nom,
           a.utilisateur_id
         FROM shared.audit_log a
         LEFT JOIN iam.utilisateur u ON a.utilisateur_id = u.id
         ORDER BY a.created_at DESC
         LIMIT 20"
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(items),
        error: None,
    }))
}

/// GET /api/admin/dashboard/tendances?periode=7j|30j|90j
pub async fn obtenir_tendances(
    _admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<TendancesQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let (jours, periode_label) = match params.periode.as_deref() {
        Some("30j") => (30, "30j"),
        Some("90j") => (90, "90j"),
        _ => (7, "7j"),
    };

    // Inscriptions utilisateurs par jour
    let inscriptions = sqlx::query_as::<_, TendancePoint>(&format!(
        "SELECT d::date as jour, COUNT(u.id)::BIGINT as total
         FROM generate_series(CURRENT_DATE - INTERVAL '{jours} days', CURRENT_DATE, '1 day') d
         LEFT JOIN iam.utilisateur u
           ON u.created_at::date = d::date AND u.deleted_at IS NULL
         GROUP BY d::date
         ORDER BY d::date"
    ))
    .fetch_all(pool.get_ref())
    .await?;

    // Annonces publiees par jour
    let annonces = sqlx::query_as::<_, TendancePoint>(&format!(
        "SELECT d::date as jour, COUNT(a.id)::BIGINT as total
         FROM generate_series(CURRENT_DATE - INTERVAL '{jours} days', CURRENT_DATE, '1 day') d
         LEFT JOIN marketplace.annonce a
           ON a.created_at::date = d::date AND a.deleted_at IS NULL
         GROUP BY d::date
         ORDER BY d::date"
    ))
    .fetch_all(pool.get_ref())
    .await?;

    // Evenements crees par jour
    let evenements = sqlx::query_as::<_, TendancePoint>(&format!(
        "SELECT d::date as jour, COUNT(e.id)::BIGINT as total
         FROM generate_series(CURRENT_DATE - INTERVAL '{jours} days', CURRENT_DATE, '1 day') d
         LEFT JOIN media_content.evenement e
           ON e.created_at::date = d::date AND e.deleted_at IS NULL
         GROUP BY d::date
         ORDER BY d::date"
    ))
    .fetch_all(pool.get_ref())
    .await?;

    // Contributions fiches pays par jour
    let contributions = sqlx::query_as::<_, TendancePoint>(&format!(
        "SELECT d::date as jour, COUNT(c.id)::BIGINT as total
         FROM generate_series(CURRENT_DATE - INTERVAL '{jours} days', CURRENT_DATE, '1 day') d
         LEFT JOIN country_profile.contribution_fiche c
           ON c.created_at::date = d::date AND c.deleted_at IS NULL
         GROUP BY d::date
         ORDER BY d::date"
    ))
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(TendancesResponse {
            periode: periode_label.to_string(),
            inscriptions_utilisateurs: inscriptions,
            annonces_publiees: annonces,
            evenements,
            contributions_pays: contributions,
        }),
        error: None,
    }))
}
