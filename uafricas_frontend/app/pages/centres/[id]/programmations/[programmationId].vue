<script setup lang="ts">
import type { ProgrammationDetailAPI } from '~/composables/useCentresCulturels'
import { formatDateCourteFrancais, formatHeureFrancais, getModeLabel } from '~/composables/useCentresCulturels'
import { useUserStore } from '~/stores/user'

useAOS()

const route = useRoute()
const userStore = useUserStore()
const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string
const { obtenirProgrammation, inscrireProgrammation, desinscrireProgrammation } = useCentresCulturels()

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

const centreId = computed(() => route.params.id as string)
const programmationId = computed(() => route.params.programmationId as string)

const isAuthenticated = computed(() => userStore.isAuthenticated)

const { data: detail, status, error: fetchError } = await useAsyncData(
  `programmation-${centreId.value}-${programmationId.value}`,
  async () => {
    const reponse = await $fetch<ApiResponse<ProgrammationDetailAPI>>(
      `${apiBase}/api/centres-culturels/${centreId.value}/programmations/${programmationId.value}`,
    )
    if (!reponse.success || !reponse.data) {
      throw createError({ message: reponse.error || 'Programmation non trouvée' })
    }
    const url = reponse.data.programmation.image_couverture_url
    return {
      ...reponse.data,
      programmation: {
        ...reponse.data.programmation,
        image_couverture_url: url
          ? (url.startsWith('http') ? url : `${apiBase}${url}`)
          : null,
      },
    }
  },
)

const chargement = computed(() => status.value === 'pending')
const erreur = computed(() => fetchError.value?.message ?? null)
const programmation = computed(() => detail.value?.programmation ?? null)
const centreNom = computed(() => detail.value?.centre.nom ?? '')

// État d'inscription (est_inscrit nécessite le JWT, indisponible en SSR → rafraîchi côté client)
const estInscrit = ref(false)
const nombreInscrits = ref(0)
const inscriptionEnCours = ref(false)
const messageInscription = ref<string | null>(null)

watchEffect(() => {
  if (detail.value?.programmation) {
    estInscrit.value = detail.value.programmation.est_inscrit
    nombreInscrits.value = detail.value.programmation.nombre_inscrits
  }
})

const placesRestantes = computed(() => {
  const places = programmation.value?.nombre_places
  if (places === null || places === undefined) return null
  return Math.max(0, places - nombreInscrits.value)
})
const complet = computed(() => placesRestantes.value !== null && placesRestantes.value <= 0 && !estInscrit.value)

// Rafraîchir le statut d'inscription côté client (le token n'existe pas au rendu SSR)
const rafraichirStatut = async () => {
  if (!userStore.isAuthenticated) return
  const maj = await obtenirProgrammation(centreId.value, programmationId.value)
  if (maj) {
    estInscrit.value = maj.programmation.est_inscrit
    nombreInscrits.value = maj.programmation.nombre_inscrits
  }
}

const showInscriptionModal = ref(false)

// Désinscription = un clic ; inscription = ouvre le formulaire
const basculerInscription = async () => {
  messageInscription.value = null
  if (estInscrit.value) {
    inscriptionEnCours.value = true
    try {
      const ok = await desinscrireProgrammation(centreId.value, programmationId.value)
      if (ok) {
        estInscrit.value = false
        nombreInscrits.value = Math.max(0, nombreInscrits.value - 1)
        messageInscription.value = 'Votre inscription a été annulée.'
      }
    }
    finally {
      inscriptionEnCours.value = false
    }
  }
  else {
    showInscriptionModal.value = true
  }
}

const confirmerInscription = async (payload: import('~/composables/useCentresCulturels').InscriptionProgPayload) => {
  messageInscription.value = null
  inscriptionEnCours.value = true
  try {
    const ok = await inscrireProgrammation(centreId.value, programmationId.value, payload)
    if (ok) {
      estInscrit.value = true
      nombreInscrits.value = nombreInscrits.value + 1
      messageInscription.value = 'Inscription confirmée. À bientôt !'
      showInscriptionModal.value = false
    }
    else {
      messageInscription.value = 'Inscription impossible (programmation complète ou erreur).'
    }
  }
  finally {
    inscriptionEnCours.value = false
  }
}

onMounted(() => rafraichirStatut())

useHead(() => ({
  title: programmation.value
    ? `${programmation.value.titre} – AfricanS`
    : 'Programmation – AfricanS',
  meta: [
    {
      name: 'description',
      content: programmation.value?.description || 'Détails de la programmation culturelle',
    },
  ],
}))

</script>

<template>
  <div class="min-h-screen bg-gray-100">
    <!-- Modal d'inscription -->
    <CentresCulturelsInscriptionProgrammationModal
      :is-open="showInscriptionModal"
      :loading="inscriptionEnCours"
      :titre-programmation="programmation?.titre"
      :defaut-nom="userStore.user?.nom"
      :defaut-prenom="userStore.user?.prenom"
      @close="showInscriptionModal = false"
      @submit="confirmerInscription"
    />

    <!-- Loading state -->
    <div v-if="chargement" class="flex justify-center items-center min-h-screen">
      <div class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-custom-green"></div>
    </div>

    <!-- Not found state -->
    <div
      v-else-if="erreur && !programmation"
      class="flex flex-col justify-center items-center min-h-screen"
    >
      <font-awesome-icon :icon="['fas', 'exclamation-triangle']" class="text-6xl text-yellow-500 mb-4" />
      <h1 class="text-2xl font-bold text-gray-700">Programmation non trouvée</h1>
      <p class="text-gray-500 mt-2">{{ erreur }}</p>
      <NuxtLink
        :to="`/centres/${centreId}`"
        class="mt-4 px-4 py-2 bg-custom-green text-white rounded-md hover:bg-custom-green/90 transition-colors"
      >
        Retour au centre
      </NuxtLink>
    </div>

    <!-- Content -->
    <div v-else-if="programmation" class="w-full h-full">
      <div
        class="bg-white mx-4 md:mx-16 lg:mx-72 pt-32 px-4 md:px-7 pb-20 rounded-b-md shadow-md"
      >
        <CommonBreadcrumbNav
          :custom-breadcrumbs="[
            { label: 'Centres culturels', to: '/centres' },
            { label: centreNom, to: `/centres/${centreId}` },
            { label: programmation.titre },
          ]"
        />

        <!-- Image de couverture -->
        <img
          v-if="programmation.image_couverture_url"
          class="w-full rounded-xl h-48 md:h-80 object-cover mt-3"
          :src="programmation.image_couverture_url"
          :alt="programmation.titre"
          data-aos="fade-up"
          data-aos-duration="600"
        />

        <!-- Info bar -->
        <div
          class="p-3 mt-3 bg-slate-100 rounded-r-md shadow-md border border-l-4 border-l-custom-green"
          data-aos="fade-right"
          data-aos-duration="600"
        >
          <div class="flex flex-wrap items-center text-custom-chocolat gap-2">
            <NuxtLink
              :to="`/centres/${centreId}`"
              class="font-bold underline hover:text-custom-green transition-colors"
            >
              {{ centreNom }}
            </NuxtLink>
            <span class="hidden md:inline">|</span>
            <div class="text-custom-green">
              {{ formatDateCourteFrancais(programmation.date_heure_debut) }}
              <template v-if="programmation.date_heure_fin">
                <span class="underline">au</span>
                {{ formatDateCourteFrancais(programmation.date_heure_fin) }}
              </template>
            </div>
          </div>
          <div class="mt-2 flex items-center text-gray-700">
            <font-awesome-icon :icon="['fas', 'location-dot']" />
            <span class="ml-2">{{ programmation.lieu || 'Lieu non précisé' }}</span>
          </div>
          <div class="mt-2 flex items-center">
            <font-awesome-icon class="text-gray-700" :icon="['far', 'clock']" />
            <span class="ml-2 font-bold text-xl md:text-2xl text-custom-chocolat">
              {{ formatHeureFrancais(programmation.date_heure_debut) }}
              <template v-if="programmation.date_heure_fin">
                - {{ formatHeureFrancais(programmation.date_heure_fin) }}
              </template>
            </span>
          </div>
          <div class="mt-2 text-sm">
            <span class="text-gray-500">Type:</span>
            <span class="ml-1 font-medium">{{ getModeLabel(programmation.mode) }}</span>
          </div>
          <div v-if="programmation.lien_en_ligne && (programmation.mode === 'en-ligne' || programmation.mode === 'hybride')" class="mt-2">
            <a
              :href="programmation.lien_en_ligne"
              target="_blank"
              class="text-custom-green hover:underline text-sm"
            >
              <font-awesome-icon :icon="['fas', 'video']" class="mr-1" />
              Rejoindre en ligne
            </a>
          </div>
          <div v-if="programmation.nombre_places" class="mt-2 text-sm text-gray-600">
            <font-awesome-icon :icon="['fas', 'users']" class="mr-1" />
            {{ programmation.nombre_places }} places disponibles
          </div>
        </div>

        <!-- Titre -->
        <h1
          class="font-bold text-2xl md:text-4xl mb-4 mt-4 text-gray-900"
          data-aos="fade-up"
          data-aos-duration="600"
        >
          {{ programmation.titre }}
        </h1>

        <!-- Inscription -->
        <div
          v-if="isAuthenticated"
          class="mb-4"
          data-aos="fade-up"
          data-aos-duration="600"
        >
          <div class="flex flex-wrap items-center gap-3">
            <button
              v-if="!estInscrit"
              class="px-4 py-2 bg-custom-green text-white rounded-md hover:bg-custom-green/90 transition-all hover:scale-105 disabled:opacity-50 disabled:hover:scale-100"
              :disabled="inscriptionEnCours || complet"
              @click="basculerInscription"
            >
              <font-awesome-icon
                :icon="['fas', inscriptionEnCours ? 'spinner' : 'user-plus']"
                :class="{ 'animate-spin': inscriptionEnCours }"
                class="mr-2"
              />
              {{ complet ? 'Complet' : "S'inscrire à cette programmation" }}
            </button>

            <template v-else>
              <span class="inline-flex items-center px-3 py-2 bg-custom-green/10 text-custom-green rounded-md font-medium">
                <font-awesome-icon :icon="['fas', 'circle-check']" class="mr-2" />
                Vous êtes inscrit(e)
              </span>
              <button
                class="px-3 py-2 text-red-600 border border-red-600 rounded-md hover:bg-red-600/10 transition-colors disabled:opacity-50"
                :disabled="inscriptionEnCours"
                @click="basculerInscription"
              >
                <font-awesome-icon
                  :icon="['fas', inscriptionEnCours ? 'spinner' : 'user-minus']"
                  :class="{ 'animate-spin': inscriptionEnCours }"
                  class="mr-2"
                />
                Se désinscrire
              </button>
            </template>
          </div>

          <p v-if="placesRestantes !== null" class="mt-2 text-sm text-gray-600">
            <font-awesome-icon :icon="['fas', 'users']" class="mr-1" />
            {{ placesRestantes }} place(s) restante(s) sur {{ programmation.nombre_places }}
          </p>
          <p v-else class="mt-2 text-sm text-gray-600">
            <font-awesome-icon :icon="['fas', 'users']" class="mr-1" />
            {{ nombreInscrits }} inscrit(s)
          </p>
          <p v-if="messageInscription" class="mt-1 text-sm text-custom-chocolat">{{ messageInscription }}</p>
        </div>
        <div
          v-else
          class="mb-4 border p-3 text-red-600 bg-red-600/10 border-red-600 rounded-md"
          data-aos="fade-up"
          data-aos-duration="600"
        >
          <p class="mb-2">Connectez-vous pour pouvoir vous inscrire à cette session</p>
          <NuxtLink
            to="/login"
            class="inline-block px-4 py-1 bg-custom-green text-white rounded-md hover:bg-custom-green/90 transition-colors"
          >
            Connexion
          </NuxtLink>
        </div>

        <!-- Description -->
        <div
          v-if="programmation.description"
          class="mt-4 bg-slate-100 rounded-r-md shadow-md border border-l-4 border-l-custom-green p-4"
          data-aos="fade-up"
          data-aos-duration="800"
        >
          <h2 class="text-xl font-bold mb-2 text-gray-800">Description</h2>
          <p class="text-gray-700 leading-relaxed">
            {{ programmation.description }}
          </p>
        </div>

        <!-- Navigation -->
        <div class="mt-6 flex justify-between">
          <NuxtLink
            :to="`/centres/${centreId}`"
            class="text-custom-chocolat hover:text-custom-green transition-colors"
          >
            <font-awesome-icon :icon="['fas', 'arrow-left']" class="mr-2" />
            Retour au centre
          </NuxtLink>
          <NuxtLink
            to="/centres"
            class="text-custom-chocolat hover:text-custom-green transition-colors"
          >
            Tous les centres
            <font-awesome-icon :icon="['fas', 'arrow-right']" class="ml-2" />
          </NuxtLink>
        </div>
      </div>
    </div>
  </div>
</template>
