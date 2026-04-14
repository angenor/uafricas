<script setup lang="ts">
// Bloc sessions live de salle publique (Option A — Feature 005)
// Tailwind v4 pur
import type { SessionAPI } from '~/composables/useAfrolang'

interface Props {
  salleId: string
}

const props = defineProps<Props>()

const { listerSessionsSallePublique, creerSessionSallePublique, demarrerSession } = useAfrolang()
const userStore = useUserStore()

const sessions = ref<SessionAPI[]>([])
const chargement = ref(false)
const modaleOuverte = ref(false)
const envoiEnCours = ref(false)
const titreSession = ref('')
const messageErreur = ref<string | null>(null)

const sessionEnCours = computed(() => sessions.value.find(s => s.etat === 'en_cours') ?? null)
const sessionPlanifiee = computed(() => sessions.value.find(s => s.etat === 'planifiee') ?? null)

const recharger = async () => {
  chargement.value = true
  const liste = await listerSessionsSallePublique(props.salleId, { par_page: 10 })
  sessions.value = liste?.sessions ?? []
  chargement.value = false
}

const ouvrirModale = () => {
  titreSession.value = ''
  messageErreur.value = null
  modaleOuverte.value = true
}

const creer = async () => {
  envoiEnCours.value = true
  messageErreur.value = null
  const session = await creerSessionSallePublique(props.salleId, {
    titre: titreSession.value.trim() || undefined,
    max_participants: 50,
    tableau_blanc_actif: true,
  })
  if (!session) {
    envoiEnCours.value = false
    messageErreur.value = 'Échec de la création'
    return
  }
  const ok = await demarrerSession(session.id)
  envoiEnCours.value = false
  if (!ok) {
    messageErreur.value = 'Session créée mais démarrage refusé'
    return
  }
  modaleOuverte.value = false
  await recharger()
  await navigateTo(`/afrolang/session/${session.id}`)
}

const rejoindre = async (sessionId: string) => {
  await navigateTo(`/afrolang/session/${sessionId}`)
}

const demarrerExistante = async (sessionId: string) => {
  const ok = await demarrerSession(sessionId)
  if (ok) await navigateTo(`/afrolang/session/${sessionId}`)
}

onMounted(recharger)
watch(() => props.salleId, recharger)
</script>

<template>
  <section class="rounded-lg border border-gray-200 bg-white">
    <header class="flex items-center justify-between border-b border-gray-200 px-4 py-3">
      <div>
        <h3 class="text-sm font-semibold text-gray-900">Session live</h3>
        <p class="text-xs text-gray-500">Visioconférence communautaire de la salle</p>
      </div>
      <button
        v-if="userStore.isAuthenticated && !sessionEnCours && !sessionPlanifiee"
        type="button"
        class="rounded-md bg-custom-chocolat px-4 py-2 text-sm font-medium text-white hover:bg-amber-800"
        @click="ouvrirModale"
      >
        Démarrer une session
      </button>
    </header>

    <div class="p-4">
      <p v-if="chargement" class="text-center text-sm text-gray-500">Chargement...</p>

      <div v-else-if="sessionEnCours" class="rounded-lg border border-red-200 bg-red-50 p-4 space-y-3">
        <div class="flex items-center justify-between gap-3">
          <div class="flex items-center gap-2">
            <span class="relative flex h-2.5 w-2.5">
              <span class="absolute inline-flex h-full w-full animate-ping rounded-full bg-red-400 opacity-75" />
              <span class="relative inline-flex h-2.5 w-2.5 rounded-full bg-red-500" />
            </span>
            <span class="text-sm font-semibold text-red-900">En direct maintenant</span>
          </div>
          <span v-if="sessionEnCours.titre" class="text-xs text-red-800">{{ sessionEnCours.titre }}</span>
        </div>
        <button
          type="button"
          class="w-full rounded-md bg-red-600 px-4 py-2 text-sm font-medium text-white hover:bg-red-700"
          @click="rejoindre(sessionEnCours.id)"
        >
          Rejoindre la visioconférence
        </button>
      </div>

      <div v-else-if="sessionPlanifiee" class="rounded-lg border border-amber-200 bg-amber-50 p-4 space-y-3">
        <div class="flex items-center justify-between gap-3">
          <div>
            <p class="text-sm font-semibold text-amber-900">Session planifiée</p>
            <p v-if="sessionPlanifiee.titre" class="text-xs text-amber-800">{{ sessionPlanifiee.titre }}</p>
          </div>
        </div>
        <button
          type="button"
          class="w-full rounded-md bg-amber-600 px-4 py-2 text-sm font-medium text-white hover:bg-amber-700"
          @click="demarrerExistante(sessionPlanifiee.id)"
        >
          Démarrer maintenant
        </button>
      </div>

      <div v-else class="text-center py-6 text-sm text-gray-500">
        <p>Aucune session live pour l'instant.</p>
        <p v-if="!userStore.isAuthenticated" class="mt-1 text-xs">
          Connectez-vous pour démarrer une session.
        </p>
      </div>
    </div>

    <!-- Modale création -->
    <Teleport to="body">
      <div
        v-if="modaleOuverte"
        class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 px-4"
        @click.self="modaleOuverte = false"
      >
        <div class="w-full max-w-md rounded-lg bg-white p-6 space-y-4">
          <h4 class="text-lg font-semibold text-gray-900">Démarrer une session live</h4>
          <label class="block">
            <span class="text-sm font-medium text-gray-700">Titre (optionnel)</span>
            <input
              v-model="titreSession"
              type="text"
              maxlength="350"
              placeholder="Ex. Atelier du vendredi - Gurunsi"
              class="mt-1 w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-custom-chocolat focus:outline-none"
            />
          </label>
          <p v-if="messageErreur" class="text-sm text-red-700">{{ messageErreur }}</p>
          <div class="flex justify-end gap-2">
            <button
              type="button"
              class="rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50"
              @click="modaleOuverte = false"
            >
              Annuler
            </button>
            <button
              type="button"
              :disabled="envoiEnCours"
              class="rounded-md bg-custom-chocolat px-4 py-2 text-sm font-medium text-white hover:bg-amber-800 disabled:opacity-50"
              @click="creer"
            >
              Démarrer
            </button>
          </div>
        </div>
      </div>
    </Teleport>
  </section>
</template>
