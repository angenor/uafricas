<template>
  <!-- Mode visioconférence (token obtenu) -->
  <AfrolangRoom
    v-if="tokenData && session"
    :token="tokenData.token"
    :room-name="tokenData.room_name"
    :livekit-url="tokenData.livekit_url"
    :session="session"
    :est-moderateur="tokenData.is_moderator"
    @quitter="handleQuitterVisio"
    @terminer="handleTerminerVisio"
  />

  <!-- Mode chargement / erreur -->
  <div v-else class="min-h-screen bg-linear-to-br from-slate-50 to-slate-100">
    <div v-if="loading" class="min-h-screen flex items-center justify-center">
      <div class="text-center">
        <div class="animate-spin rounded-full h-12 w-12 border-4 border-amber-500 border-t-transparent mx-auto mb-4" />
        <p class="text-gray-500">Connexion à la salle privée...</p>
      </div>
    </div>

    <div v-else-if="erreur" class="min-h-screen flex items-center justify-center px-4">
      <div class="text-center max-w-md">
        <font-awesome-icon :icon="['fas', 'circle-exclamation']" class="w-20 h-20 text-red-300 mx-auto mb-4" />
        <h1 class="text-2xl font-bold text-gray-800 mb-2">Salle privée indisponible</h1>
        <p class="text-gray-500 mb-6">{{ erreur }}</p>
        <NuxtLink
          to="/afrolang"
          class="inline-flex items-center gap-2 px-6 py-3 bg-amber-500 text-white font-medium rounded-xl hover:bg-amber-600 transition-colors"
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
  demarrerOuRejoindreSallePrivee,
  recupererAccesJeton,
  obtenirSession,
  quitterSession,
  terminerSession,
} = useAfrolang()

const sallePriveeId = computed(() => String(route.params.id))

const loading = ref(true)
const erreur = ref<string | null>(null)
const session = ref<SessionDetailAPI | null>(null)
const tokenData = ref<TokenResponse | null>(null)

useHead({ title: 'Salle privée Afrolang - UAfricas' })

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

onMounted(async () => {
  if (!userStore.isAuthenticated) {
    router.push('/login')
    return
  }

  const jeton = recupererAccesJeton(sallePriveeId.value)
  if (!jeton) {
    erreur.value = 'Accès non autorisé — veuillez saisir le code secret depuis la liste des salles privées.'
    loading.value = false
    return
  }

  const resultat = await demarrerOuRejoindreSallePrivee(sallePriveeId.value, jeton.jeton)
  if (!resultat) {
    erreur.value = 'Impossible de rejoindre la salle privée.'
    loading.value = false
    return
  }
  if ('erreur' in resultat) {
    erreur.value = resultat.erreur === 'archivee'
      ? 'Cette salle privée a été archivée.'
      : 'Erreur lors de la connexion.'
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
    room_name: `afrolang-privee-${resultat.session_id}`,
    livekit_url: resultat.livekit_url,
    is_moderator: resultat.moderateur_id === userStore.user?.id,
  }
  loading.value = false
})
</script>
