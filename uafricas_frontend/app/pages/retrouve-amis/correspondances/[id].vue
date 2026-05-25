<script setup lang="ts">
import type { CorrespondanceDetail, CoordonneesChoix, MotifSignalement } from '~/composables/useRetrouvAmis'

definePageMeta({ layout: 'default' })

const userStore = useUserStore()
const route = useRoute()
const { detailCorrespondance, accepterCorrespondance, refuserCorrespondance, signalerAvis, chargement, erreur } = useRetrouvAmis()

const correspondance = ref<CorrespondanceDetail | null>(null)
const erreurPage = ref<'not_found' | 'forbidden' | null>(null)
const messageSucces = ref('')

// Chargement du detail
const charger = async () => {
  erreurPage.value = null
  const id = route.params.id as string

  try {
    const res = await detailCorrespondance(id)
    if (res) {
      correspondance.value = res
    } else {
      erreurPage.value = 'not_found'
    }
  } catch (e: any) {
    const status = e?.response?.status || e?.statusCode
    if (status === 403) {
      erreurPage.value = 'forbidden'
      navigateTo('/retrouve-amis/correspondances')
    } else if (status === 404) {
      erreurPage.value = 'not_found'
    } else {
      erreurPage.value = 'not_found'
    }
  }
}

// Accepter la correspondance
const onAccepter = async (id: string, coordonnees: CoordonneesChoix) => {
  const res = await accepterCorrespondance(id, coordonnees)
  if (res) {
    messageSucces.value = 'Correspondance acceptee avec succes !'
    setTimeout(() => { messageSucces.value = '' }, 4000)
    await charger()
  }
}

// Refuser la correspondance
const onRefuser = async (id: string) => {
  const confirmer = window.confirm(
    'Etes-vous sur de vouloir refuser cette correspondance ? Cette action est irreversible.'
  )
  if (!confirmer) return

  const res = await refuserCorrespondance(id)
  if (res) {
    navigateTo('/retrouve-amis/correspondances')
  }
}

// Signaler l'avis
const onSignaler = async (avisId: string, motif: MotifSignalement, description: string) => {
  const res = await signalerAvis(avisId, { motif, description: description || undefined })
  if (res) {
    messageSucces.value = 'Signalement envoye. Merci pour votre contribution.'
    setTimeout(() => { messageSucces.value = '' }, 4000)
  }
}

onMounted(() => {
  if (!userStore.isAuthenticated) {
    navigateTo('/login')
    return
  }
  charger()
})
</script>

<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section (compact, titre ↔ description au survol) -->
    <div
      class="group relative bg-cover bg-center"
      style="background-image: url('https://images.unsplash.com/photo-1529156069898-49953e39b3ac?ixlib=rb-4.0.3&auto=format&fit=crop&w=1900&q=80')"
    >
      <div class="absolute inset-0 bg-gradient-to-r from-custom-chocolat/90 to-black/70" />
      <div class="relative max-w-4xl mx-auto px-4 pt-16 pb-6 text-center select-none">
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12">
          <h1 class="absolute inset-0 flex items-center justify-center text-white text-2xl md:text-4xl font-bold transition-opacity duration-300 group-hover:opacity-0">
            Correspondance
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Examinez les details et decidez d'accepter ou de refuser le contact.
          </p>
        </div>
      </div>
    </div>

    <div class="max-w-6xl mx-auto lg:flex lg:gap-8 px-4 py-8">
      <RetrouveAmisSideBar />
      <div class="flex-1 max-w-3xl">
      <!-- Retour -->
      <button
        class="flex items-center gap-2 text-sm text-gray-500 hover:text-amber-700 transition-colors mb-6 cursor-pointer"
        @click="navigateTo('/retrouve-amis/correspondances')"
      >
        <font-awesome-icon :icon="['fas', 'arrow-left']" />
        Retour aux correspondances
      </button>

      <!-- Message succes -->
      <div
        v-if="messageSucces"
        class="mb-6 p-4 bg-green-50 border border-green-200 text-green-700 text-sm rounded-lg"
      >
        <font-awesome-icon :icon="['fas', 'check-circle']" class="mr-2" />
        {{ messageSucces }}
      </div>

      <!-- Message erreur -->
      <div
        v-if="erreur"
        class="mb-6 p-4 bg-red-50 border border-red-200 text-red-700 text-sm rounded-lg"
      >
        <font-awesome-icon :icon="['fas', 'exclamation-triangle']" class="mr-2" />
        {{ erreur }}
      </div>

      <!-- Chargement -->
      <div v-if="chargement && !correspondance" class="flex flex-col items-center justify-center py-24">
        <div class="w-12 h-12 border-4 border-amber-200 border-t-amber-700 rounded-full animate-spin mb-4" />
        <p class="text-gray-500 text-sm">Chargement de la correspondance...</p>
      </div>

      <!-- Non trouvee -->
      <div
        v-else-if="erreurPage === 'not_found'"
        class="text-center py-24 bg-white rounded-2xl border border-gray-200"
      >
        <font-awesome-icon :icon="['fas', 'circle-question']" class="text-5xl text-gray-300 mb-4" />
        <p class="text-gray-600 text-lg mb-2">Correspondance introuvable</p>
        <p class="text-gray-400 text-sm mb-6">
          Cette correspondance n'existe pas ou vous n'avez pas les droits pour y acceder.
        </p>
        <button
          class="px-6 py-2.5 bg-amber-700 text-white font-medium rounded-lg hover:bg-amber-800 transition-colors cursor-pointer"
          @click="navigateTo('/retrouve-amis/correspondances')"
        >
          Retour aux correspondances
        </button>
      </div>

      <!-- Detail -->
      <RetrouveAmisCorrespondanceDetail
        v-else-if="correspondance"
        :correspondance="correspondance"
        @accepter="onAccepter"
        @refuser="onRefuser"
        @signaler="onSignaler"
      />
      </div>
    </div>
  </div>
</template>
