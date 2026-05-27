/**
 * Gère le lien profond `?pub={id}` partagé sur les réseaux sociaux :
 * après chargement de la liste, on défile vers la carte `#contrib-{id}`
 * et on la surligne temporairement.
 */
export function usePartagePublication() {
  const route = useRoute()
  const pubCible = ref<string | null>(null)

  /** À appeler une fois la liste chargée, avec les ids disponibles. */
  function cibler(ids: string[]) {
    const cible = route.query.pub ? String(route.query.pub) : null
    if (!cible || !ids.includes(cible)) return

    pubCible.value = cible
    nextTick(() => {
      document
        .getElementById(`contrib-${cible}`)
        ?.scrollIntoView({ behavior: 'smooth', block: 'center' })
    })
    setTimeout(() => {
      if (pubCible.value === cible) pubCible.value = null
    }, 3500)
  }

  return { pubCible, cibler }
}
