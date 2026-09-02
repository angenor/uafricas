<script setup lang="ts">
import type { TypeMedia } from '~/composables/useMediaSocial'

/** Partage d'un contenu média (chaîne, station, émission, épisode). */
defineProps<{
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

defineEmits<{ (e: 'close'): void, (e: 'submit', legende: string): void }>()

const coquille = ref<{ setLoading: (v: boolean) => void, setError: (m: string) => void, setSuccess: () => void } | null>(null)
defineExpose({
  setLoading: (v: boolean) => coquille.value?.setLoading(v),
  setError: (m: string) => coquille.value?.setError(m),
  setSuccess: () => coquille.value?.setSuccess(),
})
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
    @close="$emit('close')"
    @submit="$emit('submit', $event)"
  >
    Vous partagez <strong class="font-bold text-af-encre">{{ titre }}</strong>.
  </AfricansModalePartage>
</template>
