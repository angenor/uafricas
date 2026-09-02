<script setup lang="ts">
import type { CorrespondanceDetail, CoordonneesChoix, MotifSignalement } from '~/composables/useRetrouvAmis'

definePageMeta({ layout: false })

const userStore = useUserStore()
const { redirigerVersConnexion } = useAuth()
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
    messageSucces.value = 'Correspondance acceptée avec succès !'
    setTimeout(() => { messageSucces.value = '' }, 4000)
    await charger()
  }
}

// Refuser la correspondance
const onRefuser = async (id: string) => {
  const confirmer = window.confirm(
    'Êtes-vous sûr de vouloir refuser cette correspondance ? Cette action est irréversible.'
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
    messageSucces.value = 'Signalement envoyé. Merci pour votre contribution.'
    setTimeout(() => { messageSucces.value = '' }, 4000)
  }
}

onMounted(() => {
  if (!userStore.isAuthenticated) {
    redirigerVersConnexion()
    return
  }
  charger()
})
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <!-- L'image était hotlinkée sur Unsplash ; le hero local du module
           existait déjà. -->
      <AfricansBandeauModule
        titre="Correspondance"
        sous-titre="Examinez les détails et décidez d'accepter ou de refuser le contact."
        image="/images/africans/heros/hero-africonnect.jpg"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[
          { libelle: 'Opafrica', vers: '/retrouve-amis' },
          { libelle: 'Africonnect', vers: '/retrouve-amis' },
          { libelle: 'Correspondance' },
        ]"
      />
    </template>

    <div class="min-w-0">
      <!-- Retour -->
      <button
        class="flex items-center gap-2 text-sm text-af-atone hover:text-af-chocolat transition-colors mb-6 cursor-pointer"
        @click="navigateTo('/retrouve-amis/correspondances')"
      >
        <font-awesome-icon :icon="['fas', 'arrow-left']" />
        Retour aux correspondances
      </button>

      <!-- Message succes -->
      <div
        v-if="messageSucces"
        class="mb-6 p-4 bg-af-vert/5 border border-af-vert/30 text-af-vert text-sm rounded-lg"
      >
        <font-awesome-icon :icon="['fas', 'check-circle']" class="mr-2" />
        {{ messageSucces }}
      </div>

      <!-- Message erreur -->
      <div
        v-if="erreur"
        class="mb-6 p-4 bg-af-live/5 border border-af-live/30 text-af-live text-sm rounded-lg"
      >
        <font-awesome-icon :icon="['fas', 'exclamation-triangle']" class="mr-2" />
        {{ erreur }}
      </div>

      <!-- Chargement -->
      <div v-if="chargement && !correspondance" class="flex flex-col items-center justify-center py-24">
        <div class="w-12 h-12 border-4 border-af-chocolat/20 border-t-amber-700 rounded-full animate-spin mb-4" />
        <p class="text-af-atone text-sm">Chargement de la correspondance...</p>
      </div>

      <!-- Non trouvee -->
      <div
        v-else-if="erreurPage === 'not_found'"
        class="text-center py-24 bg-white rounded-2xl border border-af-bordure"
      >
        <font-awesome-icon :icon="['fas', 'circle-question']" class="text-5xl text-af-atone-2 mb-4" />
        <p class="text-af-corps text-lg mb-2">Correspondance introuvable</p>
        <p class="text-af-atone-2 text-sm mb-6">
          Cette correspondance n'existe pas ou vous n'avez pas les droits pour y accéder.
        </p>
        <button
          class="px-6 py-2.5 bg-af-chocolat text-white font-medium rounded-lg hover:opacity-90 transition-colors cursor-pointer"
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

    <template #rail>
      <RetrouveAmisSideBar />
    </template>
  </NuxtLayout>
</template>
