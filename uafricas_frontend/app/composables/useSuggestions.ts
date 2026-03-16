// ════════════════════════════════════════════════════════════════════════════
// Composable — Suggestions proactives de complétion d'arbre (côté client)
// ════════════════════════════════════════════════════════════════════════════

import type { NoeudArbre } from '~/composables/useLayoutArbre'
import type { SuggestionProactive } from '~/mocks/notifications'

export const useSuggestions = () => {
  const calculerSuggestions = (
    graphe: Map<string, NoeudArbre>,
  ): SuggestionProactive[] => {
    const suggestions: SuggestionProactive[] = []

    for (const noeud of graphe.values()) {
      const nomComplet = noeud.prenoms ? `${noeud.prenoms} ${noeud.nom}` : noeud.nom
      const nbLiens = noeud.parents.length + noeud.enfants.length + noeud.conjoints.length

      // Parents manquants (< 2)
      if (noeud.parents.length < 2) {
        const manquant = noeud.parents.length === 0 ? 'les parents' : 'un parent'
        suggestions.push({
          type: 'parents_manquants',
          personneId: noeud.id,
          personneNom: nomComplet,
          message: `Vous n'avez pas renseigné ${manquant} de ${nomComplet}`,
          action: `/arbre-genealogique/visualisation?centre=${noeud.id}`,
          priorite: nbLiens,
        })
      }

      // Date de naissance manquante
      if (!noeud.naissance?.annee) {
        suggestions.push({
          type: 'date_manquante',
          personneId: noeud.id,
          personneNom: nomComplet,
          message: `${nomComplet} n'a pas de date de naissance`,
          action: `/arbre-genealogique/visualisation?centre=${noeud.id}`,
          priorite: nbLiens,
        })
      }
    }

    // Trier par priorité (personnes les plus connectées en premier) et limiter à 10
    return suggestions
      .sort((a, b) => b.priorite - a.priorite)
      .slice(0, 10)
  }

  return { calculerSuggestions }
}
