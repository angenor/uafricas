<script setup lang="ts">
import type { MembreLightAPI } from '~/composables/useAmis'

const { conversations, nonLusTotal, listerConversations, fermerConversation, demandeOuverture } = useMessagerie()
const { nbAttenteMoi, compterAttenteMoi } = useRendezVous()

const ouvert = ref(false)
const amiSelectionne = ref<MembreLightAPI | null>(null)
const verrouilleeSelection = ref(false)
const chargee = ref(false)

// Onglets : conversations, annuaire des inscrits, rendez-vous visio.
const ongletActif = ref<'discussions' | 'membres' | 'rendezvous'>('discussions')

// Ouverture programmatique depuis l'extérieur (ex. /codi-moi → « Envoyer un message »).
watch(demandeOuverture, async (ami) => {
  if (!ami) return
  ouvert.value = true
  ongletActif.value = 'discussions'
  if (!chargee.value) {
    await listerConversations()
    chargee.value = true
  }
  const conv = conversations.value.find(c => c.ami.id === ami.id)
  amiSelectionne.value = conv ? conv.ami : ami
  verrouilleeSelection.value = conv ? conv.verrouillee : false
  demandeOuverture.value = null
})

const badge = computed(() => (nonLusTotal.value > 9 ? '9+' : String(nonLusTotal.value)))

const basculer = async () => {
  ouvert.value = !ouvert.value
  if (ouvert.value) {
    compterAttenteMoi()
    if (!chargee.value) {
      await listerConversations()
      chargee.value = true
    }
  }
}

const selectionner = (amiId: string) => {
  const conv = conversations.value.find(c => c.ami.id === amiId)
  if (!conv) return
  amiSelectionne.value = conv.ami
  verrouilleeSelection.value = conv.verrouillee
}

const retourListe = () => {
  amiSelectionne.value = null
  fermerConversation()
}

const fermerFenetre = () => {
  ouvert.value = false
  retourListe()
}
</script>

<template>
  <div>
    <!-- Bouton flottant -->
    <button
      type="button"
      class="fixed bottom-6 right-6 z-50 w-14 h-14 rounded-full bg-linear-to-r from-custom-chocolat to-custom-green text-white shadow-lg hover:shadow-xl flex items-center justify-center transition hover:scale-105"
      :aria-label="ouvert ? 'Fermer la messagerie' : 'Ouvrir la messagerie'"
      @click="basculer"
    >
      <font-awesome-icon :icon="ouvert ? 'fa-solid fa-xmark' : 'fa-solid fa-comments'" class="text-xl" />
      <span
        v-if="!ouvert && nonLusTotal > 0"
        class="absolute -top-1 -right-1 min-w-6 h-6 px-1 bg-red-500 text-white text-xs font-bold rounded-full flex items-center justify-center border-2 border-white"
      >{{ badge }}</span>
    </button>

    <!-- Fenêtre -->
    <Transition name="fade-slide">
      <div
        v-if="ouvert"
        class="fixed bottom-24 right-6 z-50 w-[22rem] max-w-[calc(100vw-3rem)] h-[30rem] max-h-[calc(100vh-8rem)] bg-white rounded-2xl shadow-2xl border border-gray-100 flex flex-col overflow-hidden"
      >
        <!-- En-tête -->
        <header class="flex items-center justify-between px-4 py-3 bg-linear-to-r from-custom-chocolat to-custom-green text-white shrink-0">
          <h2 class="font-semibold text-sm flex items-center gap-2">
            <font-awesome-icon icon="fa-solid fa-comments" />
            Messagerie
          </h2>
          <button type="button" class="p-1 hover:bg-white/20 rounded-lg transition" aria-label="Fermer" @click="fermerFenetre">
            <font-awesome-icon icon="fa-solid fa-xmark" />
          </button>
        </header>

        <!-- Conversation ouverte -->
        <div v-if="amiSelectionne" class="flex-1 min-h-0">
          <SocialFenetreConversation
            :ami="amiSelectionne"
            :verrouillee="verrouilleeSelection"
            @retour="retourListe"
          />
        </div>

        <!-- Liste / annuaire -->
        <div v-else class="flex-1 min-h-0 flex flex-col">
          <!-- Sélecteur d'onglets -->
          <div class="flex shrink-0 border-b border-gray-100">
            <button
              type="button"
              class="flex-1 py-2.5 text-xs font-semibold transition flex items-center justify-center gap-1.5"
              :class="ongletActif === 'discussions' ? 'text-custom-chocolat border-b-2 border-custom-chocolat' : 'text-gray-400 hover:text-gray-600'"
              @click="ongletActif = 'discussions'"
            >
              <font-awesome-icon icon="fa-solid fa-comments" />
              Discussions
            </button>
            <button
              type="button"
              class="flex-1 py-2.5 text-xs font-semibold transition flex items-center justify-center gap-1.5"
              :class="ongletActif === 'membres' ? 'text-custom-chocolat border-b-2 border-custom-chocolat' : 'text-gray-400 hover:text-gray-600'"
              @click="ongletActif = 'membres'"
            >
              <font-awesome-icon icon="fa-solid fa-users" />
              Membres
            </button>
            <button
              type="button"
              class="relative flex-1 py-2.5 text-xs font-semibold transition flex items-center justify-center gap-1.5"
              :class="ongletActif === 'rendezvous' ? 'text-custom-chocolat border-b-2 border-custom-chocolat' : 'text-gray-400 hover:text-gray-600'"
              @click="ongletActif = 'rendezvous'"
            >
              <font-awesome-icon icon="fa-solid fa-video" />
              Rendez-vous
              <span
                v-if="nbAttenteMoi > 0"
                class="absolute top-1.5 right-2 min-w-4 h-4 px-1 bg-red-500 text-white text-[10px] font-bold rounded-full flex items-center justify-center"
              >{{ nbAttenteMoi > 9 ? '9+' : nbAttenteMoi }}</span>
            </button>
          </div>

          <!-- Onglet Discussions -->
          <div v-if="ongletActif === 'discussions'" class="flex-1 min-h-0 flex flex-col">
            <div v-if="conversations.length === 0" class="flex-1 flex flex-col items-center justify-center text-center px-6">
              <div class="w-16 h-16 bg-gray-100 rounded-full flex items-center justify-center mb-4">
                <font-awesome-icon icon="fa-solid fa-users" class="text-2xl text-gray-400" />
              </div>
              <p class="text-sm font-semibold text-gray-700 mb-1">Aucune conversation</p>
              <p class="text-xs text-gray-400 mb-4">Faites-vous des amis pour commencer à discuter.</p>
              <button
                type="button"
                class="text-xs font-semibold text-white bg-custom-chocolat px-4 py-2 rounded-xl hover:shadow-md transition"
                @click="ongletActif = 'membres'"
              >
                Parcourir les membres
              </button>
            </div>
            <SocialListeAmis v-else class="flex-1" @selectionner="selectionner" />
          </div>

          <!-- Onglet Membres : annuaire des inscrits + demande d'amitié -->
          <SocialAnnuaireMembres v-else-if="ongletActif === 'membres'" class="flex-1" />

          <!-- Onglet Rendez-vous : prise & gestion des entretiens vidéo -->
          <SocialRendezVousListe v-else class="flex-1" />
        </div>
      </div>
    </Transition>
  </div>
</template>

<style scoped>
@reference "~/assets/css/main.css";

.fade-slide-enter-active,
.fade-slide-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}
.fade-slide-enter-from,
.fade-slide-leave-to {
  opacity: 0;
  transform: translateY(10px);
}
</style>
