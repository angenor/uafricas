<script setup lang="ts">
/** Signalement d'une salle ou d'une session Afrolang. */
defineProps<{
  isOpen: boolean
  /** Nom de la salle / session affiché dans le texte d'aide. */
  libelle: string
}>()

defineEmits<{
  (e: 'close'): void
  (e: 'submit', payload: { motif: string, description: string }): void
}>()

const MOTIFS = [
  { value: 'propos_haineux', label: 'Propos haineux ou discriminatoires' },
  { value: 'harcelement', label: 'Harcèlement ou intimidation' },
  { value: 'contenu_inapproprie', label: 'Contenu inapproprié ou choquant' },
  { value: 'spam', label: 'Spam / publicité' },
  { value: 'hors_sujet', label: 'Hors sujet (sans rapport avec la langue)' },
  { value: 'autre', label: 'Autre' },
]

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
    titre="Signaler cette salle"
    :motifs="MOTIFS"
    @close="$emit('close')"
    @submit="$emit('submit', $event)"
  >
    Vous signalez <strong class="font-bold text-af-encre">{{ libelle }}</strong>
    à l'équipe de modération.
  </AfricansModaleSignalement>
</template>
