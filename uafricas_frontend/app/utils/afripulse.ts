/**
 * Couleurs de la carte Afripulse.
 *
 * Le reste des données de carte : noms français, viewBox du continent,
 * agrandissement des petites îles : vit dans `carteAfrique.ts`, partagé avec
 * Retrouv'Amis. Seules les couleurs sont ici, parce qu'elles sont propres à
 * Afripulse : Retrouv'Amis colore ses territoires par NOMBRE D'AVIS, pas par
 * région, et les deux échelles n'ont rien à voir.
 *
 * Elles sont partagées entre la carte et la légende du rail : une légende qui
 * ne décrit plus les couleurs affichées est pire que pas de légende.
 */

/** Couleur de chaque région sur la carte. */
export const COULEURS_REGION: Record<string, string> = {
  "Afrique de l'Ouest": '#228B22',
  'Afrique Centrale': '#C2185B',
  "Afrique de l'Est": '#1565C0',
  'Afrique du Nord': '#E65100',
  'Afrique Australe': '#7B1FA2',
}

/** Territoire sélectionné sur la carte. */
export const COULEUR_SELECTION = '#FFD700'
/** Territoire sans fiche publiée : au repos, puis au survol. */
export const COULEUR_SANS_FICHE = '#e5e7eb'
export const COULEUR_SANS_FICHE_SURVOL = '#bdbdbd'
