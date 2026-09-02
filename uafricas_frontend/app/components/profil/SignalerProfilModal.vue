<script setup lang="ts">
/**
 * Signalement d'un profil de membre.
 *
 * Quatre modales de signalement portaient la même mécanique ; seuls les
 * motifs et la phrase d'accroche les distinguaient.
 */
defineProps<{ isOpen: boolean, profilNom: string, profilPrenom: string }>()

defineEmits<{
  (e: 'close'): void
  (e: 'submit', payload: { motif: string, description: string }): void
}>()

const MOTIFS = [
  { value: 'faux_profil', label: 'Faux profil' },
  { value: 'arnaque', label: 'Arnaque / escroquerie' },
  { value: 'usurpation', label: "Usurpation d'identité" },
  { value: 'harcelement', label: 'Harcèlement' },
  { value: 'contenu_abusif', label: 'Contenu abusif ou inapproprié' },
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
    titre="Signaler ce profil"
    :motifs="MOTIFS"
    @close="$emit('close')"
    @submit="$emit('submit', $event)"
  >
    Vous signalez le profil de
    <strong class="font-bold text-af-encre">{{ profilPrenom }} {{ profilNom }}</strong>
    à l'équipe de modération.
  </AfricansModaleSignalement>
</template>
