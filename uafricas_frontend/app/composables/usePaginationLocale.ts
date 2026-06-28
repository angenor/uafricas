import type { ComputedRef, Ref } from 'vue'

/**
 * Pagination côté client d'une liste déjà chargée en mémoire.
 *
 * Pensé pour les sections Afripulse (sites, recettes, secteurs, personnalités)
 * dont les listes peuvent devenir longues : on n'affiche qu'une tranche par page.
 * La page courante est recadrée automatiquement quand la source change
 * (application d'un filtre, suppression, rechargement…).
 */
export function usePaginationLocale<T>(
  source: Ref<T[]> | ComputedRef<T[]>,
  parPage = 9,
) {
  const page = ref(1)

  const totalPages = computed(() =>
    Math.max(1, Math.ceil(source.value.length / parPage)),
  )

  // La source a changé (nouveau filtre, rechargement) → on revient en page 1.
  watch(source, () => {
    page.value = 1
  })

  // Filet de sécurité : ne jamais rester sur une page hors limites.
  watch(totalPages, (tp) => {
    if (page.value > tp) page.value = tp
  })

  const pageItems = computed(() =>
    source.value.slice((page.value - 1) * parPage, page.value * parPage),
  )

  const pagePrecedente = () => {
    if (page.value > 1) page.value -= 1
  }
  const pageSuivante = () => {
    if (page.value < totalPages.value) page.value += 1
  }
  const allerPage = (p: number) => {
    page.value = Math.min(Math.max(1, p), totalPages.value)
  }

  return { page, totalPages, pageItems, pagePrecedente, pageSuivante, allerPage }
}
