<template>
  <!-- Mode visioconference (token obtenu) -->
  <AfrolangRoom
    v-if="tokenData"
    :token="tokenData.token"
    :room-name="tokenData.room_name"
    :livekit-url="tokenData.livekit_url"
    :session="session!"
    :est-moderateur="tokenData.is_moderator"
    @quitter="handleQuitterVisio"
    @terminer="handleTerminerVisio"
  />

  <!-- Mode preview (metadonnees session) -->
  <div v-else class="min-h-screen bg-gradient-to-br from-slate-50 to-slate-100">
    <!-- Loading -->
    <div v-if="loading" class="min-h-screen flex items-center justify-center">
      <div class="text-center">
        <div class="animate-spin rounded-full h-12 w-12 border-4 border-blue-500 border-t-transparent mx-auto mb-4" />
        <p class="text-gray-500">Chargement...</p>
      </div>
    </div>

    <!-- Not found -->
    <div v-else-if="!session" class="min-h-screen flex items-center justify-center px-4">
      <div class="text-center">
        <font-awesome-icon :icon="['fas', 'circle-exclamation']" class="w-20 h-20 text-gray-300 mx-auto mb-4" />
        <h1 class="text-2xl font-bold text-gray-800 mb-2">Session introuvable</h1>
        <p class="text-gray-500 mb-6">Cette session n'existe pas ou a été supprimée.</p>
        <NuxtLink
          to="/afrolang"
          class="inline-flex items-center gap-2 px-6 py-3 bg-blue-500 text-white font-medium rounded-xl hover:bg-blue-600 transition-colors"
        >
          <font-awesome-icon :icon="['fas', 'arrow-left']" class="w-4 h-4" />
          Retour aux salles
        </NuxtLink>
      </div>
    </div>

    <!-- Contenu -->
    <template v-else>
      <!-- Header avec gradient selon etat -->
      <div class="relative h-40 md:h-48" :class="headerGradient">
        <div class="absolute inset-0 bg-black/10" />

        <!-- Badge etat -->
        <div class="absolute top-4 left-4">
          <span
            class="px-4 py-2 rounded-full text-sm font-semibold shadow-lg flex items-center gap-2"
            :class="etatBadgeClass"
          >
            <font-awesome-icon :icon="['fas', etatInfo.icone]" class="w-4 h-4" />
            {{ etatInfo.label }}
          </span>
        </div>

        <!-- Live indicator -->
        <div v-if="session.etat === 'en_cours'" class="absolute top-4 right-4">
          <span class="bg-red-500 text-white px-3 py-1.5 rounded-full text-sm font-medium flex items-center gap-1.5 animate-pulse">
            <font-awesome-icon :icon="['fas', 'circle']" class="w-2 h-2" />
            LIVE
          </span>
        </div>
      </div>

      <!-- Contenu principal -->
      <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 -mt-16 relative z-10 pb-16">
        <div class="bg-white rounded-2xl shadow-xl overflow-hidden">
          <!-- Header -->
          <div class="p-6 md:p-8 border-b border-gray-100">
            <CommonBreadcrumbNav class="mb-6" />

            <h1 class="text-2xl md:text-3xl font-bold text-gray-900 mb-4">
              {{ session.titre || 'Session de visioconférence' }}
            </h1>

            <!-- Info grid -->
            <div class="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
              <div v-if="session.date_debut_prevue" class="flex items-center gap-2 text-gray-600">
                <font-awesome-icon :icon="['fas', 'calendar-days']" class="w-4 h-4 text-blue-500" />
                <span>{{ datePrevue }}</span>
              </div>
              <div v-if="session.demarre_at" class="flex items-center gap-2 text-gray-600">
                <font-awesome-icon :icon="['fas', 'play']" class="w-4 h-4 text-emerald-500" />
                <span>{{ demarreAt }}</span>
              </div>
              <div v-if="session.duree_secondes" class="flex items-center gap-2 text-gray-600">
                <font-awesome-icon :icon="['far', 'clock']" class="w-4 h-4 text-gray-400" />
                <span>{{ dureeFormatee }}</span>
              </div>
              <div v-if="session.max_participants" class="flex items-center gap-2 text-gray-600">
                <font-awesome-icon :icon="['fas', 'users']" class="w-4 h-4 text-gray-400" />
                <span>Max {{ session.max_participants }}</span>
              </div>
            </div>
          </div>

          <!-- Moderateur -->
          <div v-if="session.moderateur" class="p-6 md:p-8 border-b border-gray-100">
            <h2 class="text-lg font-semibold text-gray-800 mb-4 flex items-center gap-2">
              <font-awesome-icon :icon="['fas', 'user']" class="w-4 h-4 text-blue-500" />
              Modérateur
            </h2>

            <div class="flex items-center gap-3">
              <div
                v-if="session.moderateur.photo_url"
                class="w-12 h-12 rounded-full overflow-hidden"
              >
                <img :src="session.moderateur.photo_url" :alt="session.moderateur.nom" class="w-full h-full object-cover" />
              </div>
              <div v-else class="w-12 h-12 bg-gradient-to-br from-blue-400 to-cyan-500 rounded-full flex items-center justify-center text-white font-bold text-lg">
                {{ getInitiales(session.moderateur.nom, session.moderateur.prenom) }}
              </div>
              <div>
                <p class="font-medium text-gray-800">{{ session.moderateur.prenom }} {{ session.moderateur.nom }}</p>
              </div>
            </div>
          </div>

          <!-- Action rejoindre -->
          <div v-if="session.etat === 'en_cours'" class="p-6 md:p-8 border-b border-gray-100">
            <div class="bg-emerald-50 border border-emerald-200 rounded-xl p-6 text-center">
              <font-awesome-icon :icon="['fas', 'video']" class="w-10 h-10 text-emerald-500 mx-auto mb-3" />
              <h3 class="text-lg font-semibold text-emerald-800 mb-2">Session en cours</h3>
              <p class="text-emerald-700 mb-4">Cette session est actuellement en direct. Rejoignez-la maintenant !</p>

              <!-- Erreur de connexion -->
              <div v-if="joinError" class="mb-4 px-4 py-2 bg-red-50 border border-red-200 rounded-lg text-red-700 text-sm">
                {{ joinError }}
              </div>

              <button
                v-if="isAuthenticated"
                class="px-8 py-3 bg-gradient-to-r from-emerald-500 to-teal-500 text-white font-semibold rounded-xl hover:shadow-lg transition-all flex items-center gap-2 mx-auto disabled:opacity-50"
                :disabled="joining"
                @click="handleRejoindre"
              >
                <font-awesome-icon
                  :icon="['fas', joining ? 'spinner' : 'door-open']"
                  class="w-5 h-5"
                  :class="{ 'animate-spin': joining }"
                />
                {{ joining ? 'Connexion...' : 'Rejoindre la visioconférence' }}
              </button>
              <div v-else class="mt-4">
                <p class="text-amber-700 text-sm mb-3">Connectez-vous pour rejoindre cette session.</p>
                <NuxtLink
                  to="/login"
                  class="inline-flex items-center gap-2 px-6 py-3 bg-custom-chocolat text-white font-medium rounded-xl hover:bg-custom-chocolat/90 transition-colors"
                >
                  <font-awesome-icon :icon="['fas', 'right-to-bracket']" class="w-4 h-4" />
                  Se connecter
                </NuxtLink>
              </div>
            </div>
          </div>

          <!-- Participants -->
          <div class="p-6 md:p-8">
            <h2 class="text-lg font-semibold text-gray-800 mb-4 flex items-center gap-2">
              <font-awesome-icon :icon="['fas', 'users']" class="w-4 h-4 text-blue-500" />
              Participants ({{ session.participants.length }})
            </h2>

            <div
              v-if="session.participants.length > 0"
              class="grid grid-cols-1 md:grid-cols-2 gap-3"
            >
              <AfrolangParticipantBadge
                v-for="p in session.participants"
                :key="p.id"
                :participant="p"
              />
            </div>
            <div v-else class="text-center py-6 text-gray-500">
              <p>Aucun participant pour le moment</p>
            </div>
          </div>
        </div>

        <!-- Retour -->
        <div class="mt-8 text-center">
          <NuxtLink
            :to="`/afrolang/salle-privee/${session.salle_privee_id}`"
            class="inline-flex items-center gap-2 text-gray-600 hover:text-blue-500 transition-colors"
          >
            <font-awesome-icon :icon="['fas', 'arrow-left']" class="w-4 h-4" />
            Retour au cours privé
          </NuxtLink>
        </div>
      </div>
    </template>

    <!-- Modal code d'acces -->
    <AfrolangSallePriveeJoinModal
      :is-open="showJoinModal"
      :session-id="sessionId"
      ref="joinModalRef"
      @close="showJoinModal = false"
      @submit="handleJoinWithCode"
    />
  </div>
</template>

<script setup lang="ts">
import {
  useAfrolang,
  getInitiales,
  getEtatInfo,
  formatDuree,
  formatDateHeure,
  type SessionDetailAPI,
  type TokenResponse,
} from '~/composables/useAfrolang'
import { useUserStore } from '~/stores/user'

const route = useRoute()
const router = useRouter()
const userStore = useUserStore()
const { obtenirSession, genererTokenSession, quitterSession, terminerSession, erreur } = useAfrolang()

const sessionId = computed(() => route.params.id as string)

// State
const loading = ref(true)
const session = ref<SessionDetailAPI | null>(null)
const showJoinModal = ref(false)
const joinModalRef = ref<any>(null)
const joining = ref(false)
const joinError = ref<string | null>(null)

// Phase 3 : Token LiveKit
const tokenData = ref<TokenResponse | null>(null)

const isAuthenticated = computed(() => userStore.isAuthenticated)

const etatInfo = computed(() => {
  if (!session.value) return getEtatInfo('planifiee')
  return getEtatInfo(session.value.etat)
})

const headerGradient = computed(() => {
  if (!session.value) return 'bg-gradient-to-r from-gray-500 to-gray-600'
  switch (session.value.etat) {
    case 'en_cours': return 'bg-gradient-to-r from-emerald-500 via-teal-500 to-cyan-500'
    case 'planifiee': return 'bg-gradient-to-r from-blue-500 via-cyan-500 to-teal-500'
    case 'terminee': return 'bg-gradient-to-r from-gray-500 to-gray-600'
    case 'annulee': return 'bg-gradient-to-r from-red-500 to-rose-500'
    default: return 'bg-gradient-to-r from-gray-500 to-gray-600'
  }
})

const etatBadgeClass = computed(() => {
  if (!session.value) return 'bg-gray-100 text-gray-700'
  switch (session.value.etat) {
    case 'en_cours': return 'bg-emerald-100 text-emerald-700'
    case 'planifiee': return 'bg-blue-100 text-blue-700'
    case 'terminee': return 'bg-gray-100 text-gray-700'
    case 'annulee': return 'bg-red-100 text-red-700'
    default: return 'bg-gray-100 text-gray-700'
  }
})

const datePrevue = computed(() => {
  if (!session.value?.date_debut_prevue) return ''
  return formatDateHeure(session.value.date_debut_prevue)
})

const demarreAt = computed(() => {
  if (!session.value?.demarre_at) return ''
  return formatDateHeure(session.value.demarre_at)
})

const dureeFormatee = computed(() => formatDuree(session.value?.duree_secondes ?? null))

// Rejoindre la visioconference (Phase 3)
const handleRejoindre = async () => {
  joining.value = true
  joinError.value = null

  const result = await genererTokenSession(sessionId.value)

  if (result) {
    tokenData.value = result
  }
  else {
    // Si echec par code d'acces requis, ouvrir la modal
    if (erreur.value?.includes('code') || erreur.value?.includes('acces') || erreur.value?.includes('accès')) {
      showJoinModal.value = true
    }
    else {
      joinError.value = erreur.value || 'Impossible de rejoindre la session'
    }
  }

  joining.value = false
}

const handleJoinWithCode = async (codeAcces: string) => {
  joinModalRef.value?.setLoading(true)

  const result = await genererTokenSession(sessionId.value, codeAcces)

  if (result) {
    joinModalRef.value?.setSuccess()
    showJoinModal.value = false
    tokenData.value = result
  }
  else {
    joinModalRef.value?.setError(erreur.value || 'Code d\'accès incorrect')
  }
}

// Quitter la visioconference
const handleQuitterVisio = async () => {
  await quitterSession(sessionId.value)
  tokenData.value = null
  // Recharger les donnees de la session
  const resultat = await obtenirSession(sessionId.value)
  session.value = resultat
}

// Terminer la session (moderateur)
const handleTerminerVisio = async () => {
  await terminerSession(sessionId.value)
  tokenData.value = null
  // Retourner a la salle privee
  if (session.value) {
    router.push(`/afrolang/salle-privee/${session.value.salle_privee_id}`)
  }
}

onMounted(async () => {
  const id = route.params.id as string
  const resultat = await obtenirSession(id)
  session.value = resultat

  if (session.value) {
    useHead({
      title: `${session.value.titre || 'Session'} - Afrolang - UAfricas`,
    })
  }

  loading.value = false
})
</script>
