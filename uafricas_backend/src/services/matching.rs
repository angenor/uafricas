// ════════════════════════════════════════════════════════════════════════════
// Service : Matching inter-arbres
// Normalisation phonétique, scoring, matching rapide et profond
// ════════════════════════════════════════════════════════════════════════════

use sqlx::PgPool;
use uuid::Uuid;

// ─── Normalisation phonétique africaine ─────────────────────────────────

/// Normalise un nom pour le matching : lowercase, suppression diacritiques,
/// collapse des variantes phonétiques africaines courantes.
pub fn normaliser_nom(nom: &str) -> String {
    let mut s = nom.to_lowercase();

    // Suppression des diacritiques courants
    s = s
        .replace('é', "e")
        .replace('è', "e")
        .replace('ê', "e")
        .replace('ë', "e")
        .replace('à', "a")
        .replace('â', "a")
        .replace('ä', "a")
        .replace('ô', "o")
        .replace('ö', "o")
        .replace('ù', "u")
        .replace('û', "u")
        .replace('ü', "u")
        .replace('ï', "i")
        .replace('î', "i")
        .replace('ç', "c")
        .replace('ñ', "n");

    // Variantes phonétiques africaines
    s = s
        .replace("ou", "u")
        .replace("dy", "di")
        .replace("ey", "e")
        .replace("ph", "f")
        .replace("th", "t");

    // Double consonnes → simple
    let consonnes = [
        "ll", "ss", "tt", "dd", "nn", "mm", "rr", "pp", "bb", "ff", "gg", "kk",
    ];
    for double in consonnes {
        let simple = &double[..1];
        s = s.replace(double, simple);
    }

    // Suppression du h muet final
    if s.ends_with('h') && s.len() > 1 {
        s.pop();
    }

    // Trim et suppression des espaces multiples
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

// ─── Score de compatibilité des dates ───────────────────────────────────

/// Calcule le score de compatibilité entre deux années de naissance/décès.
/// Utilise une gaussienne (sigma=5 ans) : même année = 1.0, ±5 ans = 0.61
pub fn calculer_score_date(annee_a: Option<i16>, annee_b: Option<i16>) -> f32 {
    match (annee_a, annee_b) {
        (None, None) => 0.5,     // Neutre, pas de pénalité
        (None, _) | (_, None) => 0.4, // Légère pénalité pour donnée manquante
        (Some(a), Some(b)) => {
            let diff = (a as f32 - b as f32).powi(2);
            (-diff / 50.0).exp() // sigma=5 → 2*sigma²=50
        }
    }
}

// ─── Matching rapide (synchrone, nom exact) ─────────────────────────────

/// Recherche les correspondances exactes sur nom+prenoms normalisés
/// dans les arbres des autres utilisateurs. Retourné dans la réponse de création.
pub async fn matching_rapide(
    pool: &PgPool,
    nom_normalise: &str,
    prenoms_normalise: &str,
    arbre_id: Uuid,
) -> Result<Vec<Uuid>, sqlx::Error> {
    let resultats: Vec<(Uuid,)> = sqlx::query_as(
        "SELECT p.id FROM arbre_genealogique.personnes p
         JOIN arbre_genealogique.rattachements r ON r.personne_id = p.id
         WHERE r.arbre_id != $1
           AND r.deleted_at IS NULL AND p.deleted_at IS NULL
           AND p.nom_normalise = $2
           AND p.prenoms_normalise = $3
         LIMIT 10",
    )
    .bind(arbre_id)
    .bind(nom_normalise)
    .bind(prenoms_normalise)
    .fetch_all(pool)
    .await?;

    Ok(resultats.into_iter().map(|(id,)| id).collect())
}

// ─── Matching profond (asynchrone, trigram + scoring) ────────────────────

/// Exécute le matching profond pour une personne nouvellement créée.
/// Compare contre toutes les personnes des autres arbres via pg_trgm.
/// Insère les résultats dans suggestions_correspondance.
pub async fn matching_profond(
    pool: &PgPool,
    rattachement_id: Uuid,
    arbre_id: Uuid,
) -> Result<u64, sqlx::Error> {
    // Récupérer les données normalisées de la personne
    let personne: Option<(String, String, Option<i16>, Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT p.nom_normalise, COALESCE(p.prenoms_normalise, ''),
                p.naissance_annee, p.naissance_lieu, p.genre
         FROM arbre_genealogique.personnes p
         JOIN arbre_genealogique.rattachements r ON r.personne_id = p.id
         WHERE r.id = $1 AND r.deleted_at IS NULL AND p.deleted_at IS NULL",
    )
    .bind(rattachement_id)
    .fetch_optional(pool)
    .await?;

    let Some((nom_norm, prenoms_norm, naissance_annee, naissance_lieu, genre)) = personne else {
        return Ok(0);
    };

    // Matching via pg_trgm avec score composite
    // Score = nom(35%) + prenoms(20%) + date(15%) + lieu(20%) + genre(10%)
    let inseres = sqlx::query(
        "INSERT INTO arbre_genealogique.suggestions_correspondance
            (rattachement_a_id, rattachement_b_id, score, score_nom, score_prenoms, score_date, score_lieu, score_genre)
         SELECT
            $1 AS rattachement_a_id,
            r2.id AS rattachement_b_id,
            (
                0.35 * similarity(p2.nom_normalise, $2)
                + 0.20 * COALESCE(similarity(p2.prenoms_normalise, $3), 0.0)
                + 0.15 * CASE
                    WHEN p2.naissance_annee IS NULL OR $4::SMALLINT IS NULL THEN 0.4
                    ELSE EXP(-POWER(p2.naissance_annee - $4::SMALLINT, 2) / 50.0)
                  END
                + 0.20 * CASE
                    WHEN p2.naissance_lieu IS NULL OR $5::VARCHAR IS NULL THEN 0.5
                    ELSE similarity(LOWER(p2.naissance_lieu), LOWER($5::VARCHAR))
                  END
                + 0.10 * CASE
                    WHEN p2.genre IS NULL OR $6::VARCHAR IS NULL THEN 0.5
                    WHEN p2.genre = $6::VARCHAR THEN 1.0
                    ELSE 0.0
                  END
            ) AS score,
            similarity(p2.nom_normalise, $2) AS score_nom,
            COALESCE(similarity(p2.prenoms_normalise, $3), 0.0) AS score_prenoms,
            CASE
                WHEN p2.naissance_annee IS NULL OR $4::SMALLINT IS NULL THEN 0.4
                ELSE EXP(-POWER(p2.naissance_annee - $4::SMALLINT, 2) / 50.0)
            END AS score_date,
            CASE
                WHEN p2.naissance_lieu IS NULL OR $5::VARCHAR IS NULL THEN 0.5
                ELSE similarity(LOWER(p2.naissance_lieu), LOWER($5::VARCHAR))
            END AS score_lieu,
            CASE
                WHEN p2.genre IS NULL OR $6::VARCHAR IS NULL THEN 0.5
                WHEN p2.genre = $6::VARCHAR THEN 1.0
                ELSE 0.0
            END AS score_genre
         FROM arbre_genealogique.personnes p2
         JOIN arbre_genealogique.rattachements r2 ON r2.personne_id = p2.id
         WHERE r2.arbre_id != $7
           AND r2.deleted_at IS NULL AND p2.deleted_at IS NULL
           AND p2.nom_normalise IS NOT NULL
           AND p2.nom_normalise % $2  -- GIN trigram pre-filter
           AND (
                0.35 * similarity(p2.nom_normalise, $2)
                + 0.20 * COALESCE(similarity(p2.prenoms_normalise, $3), 0.0)
                + 0.15 * CASE
                    WHEN p2.naissance_annee IS NULL OR $4::SMALLINT IS NULL THEN 0.4
                    ELSE EXP(-POWER(p2.naissance_annee - $4::SMALLINT, 2) / 50.0)
                  END
                + 0.20 * CASE
                    WHEN p2.naissance_lieu IS NULL OR $5::VARCHAR IS NULL THEN 0.5
                    ELSE similarity(LOWER(p2.naissance_lieu), LOWER($5::VARCHAR))
                  END
                + 0.10 * CASE
                    WHEN p2.genre IS NULL OR $6::VARCHAR IS NULL THEN 0.5
                    WHEN p2.genre = $6::VARCHAR THEN 1.0
                    ELSE 0.0
                  END
            ) >= 0.55  -- Seuil minimum 55%
           -- Exclure les paires déjà suggérées
           AND NOT EXISTS (
               SELECT 1 FROM arbre_genealogique.suggestions_correspondance sc
               WHERE (
                   (sc.rattachement_a_id = $1 AND sc.rattachement_b_id = r2.id)
                   OR (sc.rattachement_a_id = r2.id AND sc.rattachement_b_id = $1)
               )
               AND sc.deleted_at IS NULL
           )
         ORDER BY score DESC
         LIMIT 20
         ON CONFLICT DO NOTHING",
    )
    .bind(rattachement_id)  // $1
    .bind(&nom_norm)        // $2
    .bind(&prenoms_norm)    // $3
    .bind(naissance_annee)  // $4
    .bind(&naissance_lieu)  // $5
    .bind(&genre)           // $6
    .bind(arbre_id)         // $7
    .execute(pool)
    .await?;

    Ok(inseres.rows_affected())
}
