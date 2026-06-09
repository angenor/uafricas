<template>
  <!-- Mode visioconférence (token obtenu) -->
  <div v-if="tokenData && session">
    <AfrolangRoom
      :token="tokenData.token"
      :room-name="tokenData.room_name"
      :livekit-url="tokenData.livekit_url"
      :session="session"
      :est-moderateur="tokenData.is_moderator"
      :salle-id="salleId"
      :salle-nom="salleNom"
      :sous-titre="session.titre"
      @quitter="handleQuitterVisio"
      @terminer="handleTerminerVisio"
    >
      <!-- Bouton : créer ma salle privée (US2), placé après Terminer / Quitter -->
      <template #apres-actions>
        <button
          type="button"
          class="px-4 h-12 rounded-full bg-custom-chocolat hover:bg-custom-chocolat/90 text-white font-medium text-sm flex items-center gap-2 transition-all"
          title="Créer une salle privée à partir de cette salle publique"
          @click="createModalOpen = true"
        >
          <font-awesome-icon :icon="['fas', 'plus']" class="w-4 h-4" />
          <span class="hidden sm:inline">Créer ma salle privée</span>
        </button>
      </template>
    </AfrolangRoom>

    <AfrolangSallePriveeCreateModal
      ref="createModalRef"
      :is-open="createModalOpen"
      :salle-id="salleId"
      @close="createModalOpen = false"
      @submit="soumettreCreationSallePrivee"
      @existante="handleExistante"
    />

    <!-- Toast succès création -->
    <Transition name="fade-slide">
      <div
        v-if="toastCreation"
        class="fixed bottom-6 right-6 z-10001 max-w-sm bg-green-600 text-white rounded-xl shadow-2xl p-4"
      >
        <div class="flex items-start gap-3">
          <font-awesome-icon :icon="['fas', 'circle-check']" class="w-5 h-5 mt-0.5 shrink-0" />
          <div class="flex-1 min-w-0">
            <p class="font-semibold text-sm">Salle privée créée !</p>
            <p class="text-xs text-green-100 mt-1">
              Code secret :
              <code class="bg-green-700/50 px-1.5 py-0.5 rounded font-mono">{{ toastCreation.code }}</code>
            </p>
            <p class="text-xs text-green-100 mt-1">Notez-le, il ne sera plus jamais affiché.</p>
          </div>
          <button type="button" class="text-white/80 hover:text-white" @click="toastCreation = null">
            <font-awesome-icon :icon="['fas', 'xmark']" class="w-4 h-4" />
          </button>
        </div>
      </div>
    </Transition>
  </div>

  <!-- Mode chargement / erreur -->
  <div v-else class="min-h-screen bg-linear-to-br from-slate-50 to-slate-100">
    <div v-if="loading" class="min-h-screen flex flex-col items-center justify-center p-4">
      <div class="text-center mb-6">
        <div class="animate-spin rounded-full h-12 w-12 border-4 border-blue-500 border-t-transparent mx-auto mb-4" />
        <p class="text-gray-500">Connexion au livestream...</p>
      </div>
      <!-- Feature 001-ressources-fermeture-session : panneau des ressources contribuées
           pré-affiché pendant la phase d'attente. -->
      <div class="w-full max-w-3xl">
        <AfrolangRessourcesContribueesPanel :salle-id="salleId" />
      </div>
    </div>

    <div v-else-if="erreur" class="min-h-screen flex items-center justify-center px-4">
      <div class="text-center max-w-md">
        <font-awesome-icon :icon="['fas', 'circle-exclamation']" class="w-20 h-20 text-red-300 mx-auto mb-4" />
        <h1 class="text-2xl font-bold text-gray-800 mb-2">Livestream indisponible</h1>
        <p class="text-gray-500 mb-6">{{ erreur }}</p>
        <NuxtLink
          to="/afrolang"
          class="inline-flex items-center gap-2 px-6 py-3 bg-blue-500 text-white font-medium rounded-xl hover:bg-blue-600 transition-colors"
        >
          <font-awesome-icon :icon="['fas', 'arrow-left']" class="w-4 h-4" />
          Retour aux salles
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  useAfrolang,
  type SessionDetailAPI,
  type TokenResponse,
} from '~/composables/useAfrolang'
import { useUserStore } from '~/stores/user'

const route = useRoute()
const router = useRouter()
const userStore = useUserStore()
const {
  demarrerOuRejoindreSallePublique,
  obtenirSession,
  obtenirSalle,
  quitterSession,
  terminerSession,
  creerSallePrivee,
} = useAfrolang()

const salleNom = ref<string | null>(null)

// Refonte 2026-04 : `route.params.id` porte désormais l'ID de la salle
// publique (US1). Le backend crée/rejoint la session live en 1 appel.
const salleId = computed(() => String(route.params.id))

const loading = ref(true)
const erreur = ref<string | null>(null)
const session = ref<SessionDetailAPI | null>(null)
const tokenData = ref<TokenResponse | null>(null)

// US2 : modale de création de salle privée depuis le livestream
const createModalRef = ref<{
  setLoading: (v: boolean) => void
  setError: (m: string) => void
  setSuccess: () => void
  setExistante: (id?: string) => void
} | null>(null)
const createModalOpen = ref(false)
const toastCreation = ref<{ code: string } | null>(null)
let codeSecretEnAttente = ''

useHead({ title: 'Livestream Afrolang - UAfricas' })

const handleQuitterVisio = async () => {
  if (session.value) {
    await quitterSession(session.value.id)
  }
  tokenData.value = null
  router.push('/afrolang')
}

const handleTerminerVisio = async () => {
  if (session.value) {
    await terminerSession(session.value.id)
  }
  tokenData.value = null
  router.push('/afrolang')
}

const soumettreCreationSallePrivee = async (payload: { titre: string; description: string; code_acces: string }) => {
  createModalRef.value?.setLoading(true)
  codeSecretEnAttente = payload.code_acces

  const resultat = await creerSallePrivee(salleId.value, {
    titre: payload.titre,
    description: payload.description,
    code_acces: payload.code_acces,
  })

  if (!resultat) {
    createModalRef.value?.setError('Échec de la création — veuillez réessayer.')
    return
  }
  if ('erreur' in resultat && resultat.erreur === 'salle_privee_unicite') {
    createModalRef.value?.setExistante(resultat.salle_privee_existante_id)
    return
  }

  createModalRef.value?.setSuccess()
  toastCreation.value = { code: codeSecretEnAttente }
  setTimeout(() => { toastCreation.value = null }, 8000)
}

const handleExistante = (_id?: string) => {
  createModalOpen.value = false
}

onMounted(async () => {
  if (!userStore.isAuthenticated) {
    router.push('/login')
    return
  }

  // Récupère le nom de la salle (langue / groupe ethnique) pour l'afficher dans le header.
  obtenirSalle(salleId.value).then((s) => {
    salleNom.value = s?.titre ?? null
  })

  const resultat = await demarrerOuRejoindreSallePublique(salleId.value)
  if (!resultat) {
    erreur.value = 'Impossible de démarrer ou rejoindre le livestream.'
    loading.value = false
    return
  }

  const detail = await obtenirSession(resultat.session_id)
  if (!detail) {
    erreur.value = 'Session introuvable après démarrage.'
    loading.value = false
    return
  }
  session.value = detail

  tokenData.value = {
    token: resultat.livekit_token,
    room_name: `afrolang-${resultat.session_id}`,
    livekit_url: resultat.livekit_url,
    // Refonte multi-modérateurs : valeur initiale optimiste (le statut effectif est
    // ensuite dérivé du set de modérateurs côté AfrolangRoom). `suis_je_moderateur`
    // couvre office/attitré activé/placeholder ; repli sur l'ancien calcul.
    is_moderator: resultat.suis_je_moderateur ?? (resultat.moderateur_id === userStore.user?.id),
  }
  loading.value = false
})
</script>
