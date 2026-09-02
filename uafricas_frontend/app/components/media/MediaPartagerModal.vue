<script setup lang="ts">
import type { TypeMedia } from '~/composables/useMediaSocial'

/** Partage d'un contenu média (chaîne, station, émission, épisode). */
const props = defineProps<{
  isOpen: boolean
  /** Titre lisible du contenu (nom de chaîne, titre d'émission…). */
  titre: string
  typeMedia: TypeMedia
  mediaId: string
  /**
   * Chemin de la page de détail, pour un aperçu social correct (FR-026).
   * Ouverte depuis une SECTION, la modale partagerait sinon la page de liste.
   */
  urlDetail?: string
}>()

const emit = defineEmits<{ (e: 'close'): void, (e: 'partage'): void }>()

const { partager, estConnecte } = useMediaSocial()

const coquille = ref<{
  setLoading: (v: boolean) => void
  setError: (m: string) => void
  setSuccess: () => void
} | null>(null)

/**
 * L'appel est porté ICI, et non par les huit pages qui montent la modale.
 *
 * La coquille `AfricansModalePartage` se contente de ré-émettre `submit` ; un
 * `@submit` non branché ne produit aucune erreur, ni au build ni à l'exécution.
 * Le bouton « Partager » restait donc inerte en silence : pas de chargement,
 * pas de message, rien d'enregistré. Ce composant a `typeMedia` et `mediaId`,
 * les deux seuls arguments dont `partager` a besoin — le déléguer aux pages
 * n'apporterait rien et multiplierait par huit les occasions de l'oublier.
 */
const soumettre = async (legende: string) => {
  coquille.value?.setLoading(true)
  const res = await partager(props.typeMedia, props.mediaId, legende || undefined)
  if (res) {
    coquille.value?.setSuccess()
    emit('partage')
  } else {
    coquille.value?.setError('Erreur lors du partage. Veuillez réessayer.')
  }
}
</script>

<template>
  <AfricansModalePartage
    ref="coquille"
    :is-open="isOpen"
    titre="Partager ce contenu"
    :texte-partage="`Découvrez ${titre} sur AfricanS`"
    :type-objet="typeMedia"
    :objet-id="mediaId"
    :url="urlDetail"
    :est-connecte="estConnecte()"
    @close="$emit('close')"
    @submit="soumettre"
  >
    Vous partagez <strong class="font-bold text-af-encre">{{ titre }}</strong>.
  </AfricansModalePartage>
</template>
