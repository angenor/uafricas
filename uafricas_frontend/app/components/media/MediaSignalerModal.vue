<script setup lang="ts">
import { MOTIFS_SIGNALEMENT_MEDIA } from '~/composables/useMediaSocial'

/** Signalement d'un contenu média (chaîne, station, émission, épisode). */
defineProps<{
  isOpen: boolean
  /** Titre du contenu visé, rappelé au membre avant qu'il ne confirme. */
  titre: string
}>()

defineEmits<{
  (e: 'close'): void
  (e: 'submit', payload: { motif: string, description: string }): void
}>()

const coquille = ref<{ setLoading: (v: boolean) => void, setError: (m: string) => void, setSuccess: (m: string) => void } | null>(null)
defineExpose({
  setLoading: (v: boolean) => coquille.value?.setLoading(v),
  setError: (m: string) => coquille.value?.setError(m),
  setSuccess: (m: string) => coquille.value?.setSuccess(m),
})
</script>

<template>
  <AfricansModaleSignalement
    ref="coquille"
    :is-open="isOpen"
    titre="Signaler ce contenu"
    :motifs="MOTIFS_SIGNALEMENT_MEDIA"
    @close="$emit('close')"
    @submit="$emit('submit', $event)"
  >
    Vous signalez <strong class="font-bold text-af-encre">{{ titre }}</strong>
    à l'équipe de modération.
  </AfricansModaleSignalement>
</template>
