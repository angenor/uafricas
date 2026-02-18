use serde::{Deserialize, Serialize};

/// Parametres de pagination communs a tous les endpoints de listing
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    /// Numero de page (1-indexed, defaut: 1)
    pub page: Option<i64>,
    /// Nombre d'elements par page (defaut: 12, max: 50)
    pub par_page: Option<i64>,
    /// Colonne de tri (defaut: "created_at")
    pub tri_par: Option<String>,
    /// Direction du tri : "asc" ou "desc" (defaut: "desc")
    pub tri_dir: Option<String>,
}

impl PaginationParams {
    /// Retourne le numero de page normalise (min 1)
    pub fn page(&self) -> i64 {
        self.page.unwrap_or(1).max(1)
    }

    /// Retourne le nombre d'elements par page normalise (entre 1 et 50)
    pub fn par_page(&self) -> i64 {
        self.par_page.unwrap_or(12).clamp(1, 50)
    }

    /// Retourne l'offset SQL
    pub fn offset(&self) -> i64 {
        (self.page() - 1) * self.par_page()
    }

    /// Retourne la direction de tri validee ("ASC" ou "DESC")
    pub fn direction_tri(&self) -> &str {
        match self.tri_dir.as_deref() {
            Some("asc" | "ASC") => "ASC",
            _ => "DESC",
        }
    }

    /// Valide et retourne la colonne de tri (protege contre l'injection SQL)
    /// Seules les colonnes presentes dans `colonnes_autorisees` sont acceptees
    pub fn colonne_tri(&self, colonnes_autorisees: &[&str], defaut: &str) -> String {
        match &self.tri_par {
            Some(col) if colonnes_autorisees.contains(&col.as_str()) => col.clone(),
            _ => defaut.to_string(),
        }
    }
}

/// Reponse paginee generique
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    /// Elements de la page courante
    pub data: Vec<T>,
    /// Nombre total d'elements
    pub total: i64,
    /// Page courante
    pub page: i64,
    /// Elements par page
    pub par_page: i64,
    /// Nombre total de pages
    pub total_pages: i64,
}

impl<T: Serialize> PaginatedResponse<T> {
    /// Construit une reponse paginee a partir des donnees et du total
    pub fn new(data: Vec<T>, total: i64, page: i64, par_page: i64) -> Self {
        let total_pages = if total == 0 {
            1
        } else {
            (total + par_page - 1) / par_page
        };
        Self {
            data,
            total,
            page,
            par_page,
            total_pages,
        }
    }

    /// Construit a partir des PaginationParams
    pub fn from_params(data: Vec<T>, total: i64, params: &PaginationParams) -> Self {
        Self::new(data, total, params.page(), params.par_page())
    }
}
