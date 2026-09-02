<script setup lang="ts">
/** Signalement d'une contribution Afripulse. */
defineProps<{ isOpen: boolean, libelle: string }>()

defineEmits<{
  (e: 'close'): void
  (e: 'submit', payload: { motif: string, description: string }): void
}>()

const MOTIFS = [
  { value: 'contenu_faux', label: 'Information fausse ou trompeuse' },
  { value: 'infos_erronees', label: 'Données erronées (lieu, contact…)' },
  { value: 'inapproprie', label: 'Contenu inapproprié ou offensant' },
  { value: 'doublon', label: "Doublon d'une autre contribution" },
  { value: 'spam', label: 'Spam / publicité' },
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
    titre="Signaler cette contribution"
    :motifs="MOTIFS"
    @close="$emit('close')"
    @submit="$emit('submit', $event)"
  >
    Vous signalez <strong class="font-bold text-af-encre">{{ libelle }}</strong>
    à l'équipe de modération.
  </AfricansModaleSignalement>
</template>
