use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── B12.1 — Stats globales ─────────────────────────────────────────────

/// Reponse structuree pour GET /api/admin/dashboard/stats
#[derive(Debug, Serialize)]
pub struct DashboardStats {
    pub utilisateurs: StatsUtilisateurs,
    pub organisations: StatsSimple,
    pub annonces: StatsAnnonces,
    pub programmes: StatsProgrammes,
    pub innovations: StatsInnovations,
    pub projets: StatsProjets,
    pub africantives: StatsSimple,
    pub centres_culturels: StatsSimple,
    pub codimoi: StatsCodimoi,
    pub sessions_afrolang: StatsSessionsAfrolang,
    pub evenements: StatsEvenements,
    pub moocs: StatsMoocs,
    pub livres: StatsSimple,
    pub radio_tv: StatsRadioTv,
    pub factchecks: StatsFactchecks,
    pub bad_habits: StatsBadHabits,
    pub idea_forces: StatsSimple,
    pub fiches_pays: StatsFichesPays,
    pub audit: StatsAudit,
}

#[derive(Debug, Serialize)]
pub struct StatsSimple {
    pub total: i64,
}

#[derive(Debug, Serialize)]
pub struct StatsUtilisateurs {
    pub total: i64,
    pub actifs: i64,
    pub en_attente: i64,
    pub suspendus: i64,
}

#[derive(Debug, Serialize)]
pub struct StatsAnnonces {
    pub total: i64,
    pub publiees: i64,
    pub en_attente: i64,
    pub expirees: i64,
}

#[derive(Debug, Serialize)]
pub struct StatsProgrammes {
    pub total: i64,
    pub actifs: i64,
    pub candidatures_en_attente: i64,
}

#[derive(Debug, Serialize)]
pub struct StatsInnovations {
    pub total: i64,
    pub publiees: i64,
}

#[derive(Debug, Serialize)]
pub struct StatsProjets {
    pub total: i64,
    pub approuves: i64,
    pub en_revue: i64,
    pub soumis: i64,
}

#[derive(Debug, Serialize)]
pub struct StatsCodimoi {
    pub total: i64,
    pub par_type: CodimoiParType,
}

#[derive(Debug, Serialize)]
pub struct CodimoiParType {
    pub proverbe_adage: i64,
    pub citation: i64,
    pub ressource_historique: i64,
    pub bonne_pratique: i64,
}

#[derive(Debug, Serialize)]
pub struct StatsSessionsAfrolang {
    pub total: i64,
    pub en_cours: i64,
}

#[derive(Debug, Serialize)]
pub struct StatsEvenements {
    pub total: i64,
    pub a_venir: i64,
    pub inscrits_total: i64,
}

#[derive(Debug, Serialize)]
pub struct StatsMoocs {
    pub total: i64,
    pub inscrits_total: i64,
    pub en_cours: i64,
}

#[derive(Debug, Serialize)]
pub struct StatsRadioTv {
    pub stations_radio: i64,
    pub chaines_tv: i64,
}

#[derive(Debug, Serialize)]
pub struct StatsFactchecks {
    pub total: i64,
    pub par_verdict: FactcheckParVerdict,
}

#[derive(Debug, Serialize)]
pub struct FactcheckParVerdict {
    pub vrai: i64,
    pub faux: i64,
    pub partiellement_vrai: i64,
    pub trompeur: i64,
    pub non_verifie: i64,
}

#[derive(Debug, Serialize)]
pub struct StatsBadHabits {
    pub total: i64,
    pub par_gravite: BadHabitParGravite,
}

#[derive(Debug, Serialize)]
pub struct BadHabitParGravite {
    pub faible: i64,
    pub elevee: i64,
    pub critique: i64,
}

#[derive(Debug, Serialize)]
pub struct StatsFichesPays {
    pub total: i64,
    pub contributions_en_attente: i64,
}

#[derive(Debug, Serialize)]
pub struct StatsAudit {
    pub actions_aujourd_hui: i64,
    pub actions_cette_semaine: i64,
}

/// Row interne pour mapper le resultat de la requete CTE
#[derive(Debug, FromRow)]
pub struct StatsRow {
    pub utilisateurs_total: i64,
    pub utilisateurs_actifs: i64,
    pub utilisateurs_en_attente: i64,
    pub utilisateurs_suspendus: i64,
    pub organisations_total: i64,
    pub annonces_total: i64,
    pub annonces_publiees: i64,
    pub annonces_en_attente: i64,
    pub annonces_expirees: i64,
    pub programmes_total: i64,
    pub programmes_actifs: i64,
    pub candidatures_en_attente: i64,
    pub innovations_total: i64,
    pub innovations_publiees: i64,
    pub projets_total: i64,
    pub projets_approuves: i64,
    pub projets_en_revue: i64,
    pub projets_soumis: i64,
    pub africantives_total: i64,
    pub centres_culturels_total: i64,
    pub codimoi_total: i64,
    pub codimoi_proverbe_adage: i64,
    pub codimoi_citation: i64,
    pub codimoi_ressource_historique: i64,
    pub codimoi_bonne_pratique: i64,
    pub sessions_total: i64,
    pub sessions_en_cours: i64,
    pub evenements_total: i64,
    pub evenements_a_venir: i64,
    pub evenements_inscrits_total: i64,
    pub moocs_total: i64,
    pub moocs_inscrits_total: i64,
    pub moocs_en_cours: i64,
    pub livres_total: i64,
    pub stations_radio: i64,
    pub chaines_tv: i64,
    pub factchecks_total: i64,
    pub factchecks_vrai: i64,
    pub factchecks_faux: i64,
    pub factchecks_partiellement_vrai: i64,
    pub factchecks_trompeur: i64,
    pub factchecks_non_verifie: i64,
    pub bad_habits_total: i64,
    pub bad_habits_faible: i64,
    pub bad_habits_elevee: i64,
    pub bad_habits_critique: i64,
    pub idea_forces_total: i64,
    pub fiches_pays_total: i64,
    pub contributions_en_attente: i64,
    pub audit_actions_aujourd_hui: i64,
    pub audit_actions_cette_semaine: i64,
}

impl StatsRow {
    pub fn to_dashboard_stats(self) -> DashboardStats {
        DashboardStats {
            utilisateurs: StatsUtilisateurs {
                total: self.utilisateurs_total,
                actifs: self.utilisateurs_actifs,
                en_attente: self.utilisateurs_en_attente,
                suspendus: self.utilisateurs_suspendus,
            },
            organisations: StatsSimple { total: self.organisations_total },
            annonces: StatsAnnonces {
                total: self.annonces_total,
                publiees: self.annonces_publiees,
                en_attente: self.annonces_en_attente,
                expirees: self.annonces_expirees,
            },
            programmes: StatsProgrammes {
                total: self.programmes_total,
                actifs: self.programmes_actifs,
                candidatures_en_attente: self.candidatures_en_attente,
            },
            innovations: StatsInnovations {
                total: self.innovations_total,
                publiees: self.innovations_publiees,
            },
            projets: StatsProjets {
                total: self.projets_total,
                approuves: self.projets_approuves,
                en_revue: self.projets_en_revue,
                soumis: self.projets_soumis,
            },
            africantives: StatsSimple { total: self.africantives_total },
            centres_culturels: StatsSimple { total: self.centres_culturels_total },
            codimoi: StatsCodimoi {
                total: self.codimoi_total,
                par_type: CodimoiParType {
                    proverbe_adage: self.codimoi_proverbe_adage,
                    citation: self.codimoi_citation,
                    ressource_historique: self.codimoi_ressource_historique,
                    bonne_pratique: self.codimoi_bonne_pratique,
                },
            },
            sessions_afrolang: StatsSessionsAfrolang {
                total: self.sessions_total,
                en_cours: self.sessions_en_cours,
            },
            evenements: StatsEvenements {
                total: self.evenements_total,
                a_venir: self.evenements_a_venir,
                inscrits_total: self.evenements_inscrits_total,
            },
            moocs: StatsMoocs {
                total: self.moocs_total,
                inscrits_total: self.moocs_inscrits_total,
                en_cours: self.moocs_en_cours,
            },
            livres: StatsSimple { total: self.livres_total },
            radio_tv: StatsRadioTv {
                stations_radio: self.stations_radio,
                chaines_tv: self.chaines_tv,
            },
            factchecks: StatsFactchecks {
                total: self.factchecks_total,
                par_verdict: FactcheckParVerdict {
                    vrai: self.factchecks_vrai,
                    faux: self.factchecks_faux,
                    partiellement_vrai: self.factchecks_partiellement_vrai,
                    trompeur: self.factchecks_trompeur,
                    non_verifie: self.factchecks_non_verifie,
                },
            },
            bad_habits: StatsBadHabits {
                total: self.bad_habits_total,
                par_gravite: BadHabitParGravite {
                    faible: self.bad_habits_faible,
                    elevee: self.bad_habits_elevee,
                    critique: self.bad_habits_critique,
                },
            },
            idea_forces: StatsSimple { total: self.idea_forces_total },
            fiches_pays: StatsFichesPays {
                total: self.fiches_pays_total,
                contributions_en_attente: self.contributions_en_attente,
            },
            audit: StatsAudit {
                actions_aujourd_hui: self.audit_actions_aujourd_hui,
                actions_cette_semaine: self.audit_actions_cette_semaine,
            },
        }
    }
}

// ── B12.2 — Activite recente ────────────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct ActiviteRecenteItem {
    pub id: Uuid,
    pub action: String,
    pub schema_name: String,
    pub table_name: String,
    pub record_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub utilisateur_nom: Option<String>,
    pub utilisateur_id: Option<Uuid>,
}

// ── B12.3 — Tendances ──────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct TendancesQueryParams {
    pub periode: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TendancesResponse {
    pub periode: String,
    pub inscriptions_utilisateurs: Vec<TendancePoint>,
    pub annonces_publiees: Vec<TendancePoint>,
    pub evenements: Vec<TendancePoint>,
    pub contributions_pays: Vec<TendancePoint>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct TendancePoint {
    pub jour: NaiveDate,
    pub total: i64,
}
