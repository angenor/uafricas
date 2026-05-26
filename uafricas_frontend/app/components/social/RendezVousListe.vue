<script setup lang="ts">
import type { FiltreRendezVous, RendezVousAPI } from '~/composables/useRendezVous'

const {
  rendezVous, filtreCourant, chargement,
  lister, accepter, refuser, annuler, rafraichir,
} = useRendezVous()
const { demanderOuverture } = useMessagerie()

const onglets: { id: FiltreRendezVous, label: string }[] = [
  { id: 'attente_moi', label: 'À répondre' },
  { id: 'attente_autre', label: 'En attente' },
  { id: 'a_venir', label: 'À venir' },
  { id: 'passes', label: 'Passés' },
]

const messageErreur = ref('')
const rdvContre = ref<RendezVousAPI | null>(null)
const rdvSalle = ref<RendezVousAPI | null>(null)

onMounted(() => { lister(filtreCourant.value) })

const changerFiltre = (filtre: FiltreRendezVous) => {
  if (filtre === filtreCourant.value) return
  lister(filtre)
}

const executer = async (operation: Promise<unknown>) => {
  messageErreur.value = ''
  try {
    await operation
    await rafraichir()
  }
  catch (e) {
    messageErreur.value = e instanceof Error ? e.message : 'Action impossible.'
  }
}

const onAccepter = (rdv: RendezVousAPI) => executer(accepter(rdv.id))
const onRefuser = (rdv: RendezVousAPI) => executer(refuser(rdv.id))
const onAnnuler = (rdv: RendezVousAPI) => {
  if (typeof window !== 'undefined' && !window.confirm('Annuler ce rendez-vous ?')) return
  executer(annuler(rdv.id))
}
const onMessagerie = (rdv: RendezVousAPI) => demanderOuverture(rdv.autre)
</script>

<template>
  <div class="flex flex-col min-h-0">
    <!-- Filtres -->
    <div class="flex shrink-0 border-b border-gray-100 overflow-x-auto">
      <button
        v-for="o in onglets"
        :key="o.id"
        type="button"
        class="flex-1 min-w-max px-3 py-2.5 text-xs font-semibold transition whitespace-nowrap"
        :class="filtreCourant === o.id
          ? 'text-custom-chocolat border-b-2 border-custom-chocolat'
          : 'text-gray-400 hover:text-gray-600'"
        @click="changerFiltre(o.id)"
      >
        {{ o.label }}
      </button>
    </div>

    <!-- Erreur transitoire -->
    <p v-if="messageErreur" class="m-3 text-xs text-red-600 bg-red-50 border border-red-100 rounded-xl px-3 py-2">
      {{ messageErreur }}
    </p>

    <!-- Contenu -->
    <div class="flex-1 min-h-0 overflow-y-auto p-3 space-y-3">
      <div v-if="chargement" class="flex justify-center py-10 text-gray-400">
        <font-awesome-icon icon="fa-solid fa-spinner" spin class="text-2xl" />
      </div>

      <div v-else-if="rendezVous.length === 0" class="flex flex-col items-center justify-center text-center py-10 px-6">
        <div class="w-16 h-16 bg-gray-100 rounded-full flex items-center justify-center mb-4">
          <font-awesome-icon icon="fa-solid fa-video" class="text-2xl text-gray-400" />
        </div>
        <p class="text-sm font-semibold text-gray-700 mb-1">Aucun rendez-vous</p>
        <p class="text-xs text-gray-400">Proposez un rendez-vous depuis le profil d'un ami.</p>
      </div>

      <template v-else>
        <SocialRendezVousCarte
          v-for="rdv in rendezVous"
          :key="rdv.id"
          :rdv="rdv"
          @accepter="onAccepter(rdv)"
          @refuser="onRefuser(rdv)"
          @contre="rdvContre = rdv"
          @annuler="onAnnuler(rdv)"
          @rejoindre="rdvSalle = rdv"
          @messagerie="onMessagerie(rdv)"
        />
      </template>
    </div>

    <!-- Modale contre-proposition -->
    <SocialRendezVousContreModal
      v-if="rdvContre"
      :rdv="rdvContre"
      @fermer="rdvContre = null"
      @contre="rafraichir()"
    />

    <!-- Salle visioconférence -->
    <SocialRendezVousSalle
      v-if="rdvSalle"
      :rdv="rdvSalle"
      @fermer="rdvSalle = null"
    />
  </div>
</template>
