<template>
  <button
    v-if="!signalee"
    type="button"
    class="inline-flex items-center gap-2 text-[14px]/[1.4] text-af-atone underline-offset-2 transition hover:text-af-chocolat hover:underline"
    @click="ouverte = true"
  >
    <font-awesome-icon icon="fa-solid fa-flag" />
    Signaler cette épreuve
  </button>
  <span v-else class="inline-flex items-center gap-2 text-[14px]/[1.4] text-af-atone">
    <font-awesome-icon icon="fa-solid fa-circle-check" class="text-af-vert" />
    Épreuve signalée
  </span>

  <AfricansModaleSignalement
    ref="modale"
    :is-open="ouverte"
    titre="Signaler cette épreuve"
    :motifs="MOTIFS"
    @close="ouverte = false"
    @submit="envoyer"
  />
</template>

<script setup lang="ts">
import { messageErreurJeu } from '~/composables/useJeu'

/**
 * Signalement d'une épreuve par le membre qui vient d'y répondre : la réponse
 * donnée pour bonne est fausse, l'énoncé est ambigu, le contenu est déplacé.
 * Une seule fois par épreuve ; un signalement confirmé par l'équipe rapporte de
 * la réputation.
 */
const props = defineProps<{ epreuveId: string }>()

// Les valeurs sont celles du CHECK `ck_signalement_epreuve_motif`.
const MOTIFS = [
  { value: 'reponse_erronee', label: 'La réponse donnée pour bonne est fausse' },
  { value: 'enonce_ambigu', label: 'L\'énoncé est ambigu ou mal formulé' },
  { value: 'contenu_deplace', label: 'Le contenu est déplacé' },
  { value: 'autre', label: 'Autre' },
]

const { signalerEpreuve } = useJeu()

const ouverte = ref(false)
const signalee = ref(false)
const modale = ref<{
  setLoading: (v: boolean) => void
  setError: (m: string) => void
  setSuccess: (m: string) => void
} | null>(null)

const envoyer = async ({ motif, description }: { motif: string, description: string }) => {
  modale.value?.setLoading(true)
  try {
    await signalerEpreuve(props.epreuveId, motif, description)
    signalee.value = true
    modale.value?.setSuccess('Merci. L\'équipe va examiner cette épreuve.')
  }
  catch (e) {
    modale.value?.setError(messageErreurJeu(e, 'Le signalement n\'a pas pu être envoyé.'))
  }
}
</script>
