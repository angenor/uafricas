<template>
  <span class="inline-flex">
    <button
      type="button"
      class="inline-flex items-center gap-1 text-[12px]/[1.3] text-af-atone hover:text-af-live focus-visible:outline-2 focus-visible:outline-af-chocolat"
      :aria-label="fait ? 'Photo signalée' : 'Signaler cette photo'"
      :disabled="fait"
      @click.stop="ouvert = true"
    >
      <font-awesome-icon icon="fa-solid fa-flag" />
      {{ fait ? 'Signalée' : 'Signaler' }}
    </button>
    <AfricansModaleSignalement
      :is-open="ouvert"
      titre="Signaler cette photo"
      :motifs="MOTIFS"
      @close="ouvert = false"
      @submit="envoyer"
    />
    <span v-if="erreur" class="ml-2 text-[12px] text-af-live" role="alert">{{ erreur }}</span>
  </span>
</template>

<script setup lang="ts">
import { messageErreurJeu } from '~/composables/useJeu'

/**
 * Signaler une photo de concours (feature 014, US9). Une fois par membre ; au
 * 11ᵉ signalement distinct, la photo est suspendue et l'équipe la relit.
 */
const props = defineProps<{ concoursId: string, participationId: string }>()

const MOTIFS = [
  { value: 'inapproprie', label: 'Contenu choquant ou inapproprié' },
  { value: 'mineur', label: 'Montre un mineur reconnaissable' },
  { value: 'hors_sujet', label: 'Hors sujet' },
  { value: 'plagiat', label: 'Photo qui n\'est pas de son auteur' },
  { value: 'autre', label: 'Autre raison' },
]

const { signaler } = useConcours()
const ouvert = ref(false)
const fait = ref(false)
const erreur = ref('')

const envoyer = async ({ motif, description }: { motif: string, description: string }) => {
  erreur.value = ''
  const libelle = MOTIFS.find(m => m.value === motif)?.label ?? motif
  try {
    await signaler(props.concoursId, props.participationId, description ? `${libelle} : ${description}` : libelle)
    fait.value = true
    ouvert.value = false
  }
  catch (e) {
    ouvert.value = false
    erreur.value = messageErreurJeu(e, 'Le signalement n\'a pas pu être envoyé.')
  }
}
</script>
