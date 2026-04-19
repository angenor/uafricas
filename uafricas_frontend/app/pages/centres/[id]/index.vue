<script setup lang="ts">
import type { CentreCulturelDetailAPI, ProgrammationAPI } from '~/composables/useCentresCulturels'
import { trierProgrammations } from '~/composables/useCentresCulturels'

useAOS()

const route = useRoute()
const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string
const { genererLienGoogleMaps } = useCentresCulturels()

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

const id = computed(() => route.params.id as string)

const showInscription = ref(false)

const { data: centre, status, error: fetchError } = await useAsyncData(
  `centre-${id.value}`,
  async () => {
    const reponse = await $fetch<ApiResponse<CentreCulturelDetailAPI>>(
      `${apiBase}/api/centres-culturels/${id.value}`,
    )
    if (!reponse.success || !reponse.data) {
      throw createError({ message: reponse.error || 'Centre culturel non trouvé' })
    }
    return {
      ...reponse.data,
      image_couverture_url: reponse.data.image_couverture_url
        ? `${apiBase}${reponse.data.image_couverture_url}`
        : null,
    }
  },
)

const chargement = computed(() => status.value === 'pending')
const erreur = computed(() => fetchError.value?.message ?? null)

// Tri FR-017a : à venir croissant puis passées décroissant (Décision 3 research.md).
const programmationsTriees = computed<ProgrammationAPI[]>(() => {
  return centre.value ? trierProgrammations(centre.value.programmations) : []
})

const googleMapsUrl = computed(() => {
  if (!centre.value) return null
  return genererLienGoogleMaps(centre.value.latitude, centre.value.longitude)
})

useHead(() => ({
  title: centre.value
    ? `Centre culturel de ${centre.value.nom} – UAfricas`
    : 'Centre culturel – UAfricas',
  meta: [
    {
      name: 'description',
      content: centre.value
        ? `Découvrez le centre culturel de ${centre.value.nom}. ${centre.value.programmations.length} événements programmés.`
        : 'Centre culturel africain et afro-descendant',
    },
  ],
}))

const handleInscriptionSubmit = (options: { prioritaires: boolean, toutes: boolean }) => {
  console.log('Inscription avec options:', options)
  alert('Inscription enregistrée avec succès!')
  showInscription.value = false
}
</script>

<template>
  <div class="min-h-screen bg-gray-100">
    <!-- Modals -->
    <CentresCulturelsInscriptionModal
      :is-open="showInscription"
      @close="showInscription = false"
      @submit="handleInscriptionSubmit"
    />

    <!-- Loading state -->
    <div v-if="chargement" class="flex justify-center items-center min-h-screen">
      <div class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-custom-green"></div>
    </div>

    <!-- Error state -->
    <div
      v-else-if="erreur && !centre"
      class="flex flex-col justify-center items-center min-h-screen"
    >
      <font-awesome-icon :icon="['fas', 'exclamation-triangle']" class="text-6xl text-yellow-500 mb-4" />
      <h1 class="text-2xl font-bold text-gray-700">Centre non trouvé</h1>
      <p class="text-gray-500 mt-2">{{ erreur }}</p>
      <NuxtLink
        to="/centres"
        class="mt-4 px-4 py-2 bg-custom-green text-white rounded-md hover:bg-custom-green/90 transition-colors"
      >
        Retour à la liste
      </NuxtLink>
    </div>

    <!-- Content -->
    <div v-else-if="centre" class="h-full mx-4 md:mx-16 lg:mx-56">
      <!-- Header avec bannière -->
      <div class="bg-white pt-32 px-4 md:px-7 pb-5 rounded-b-xl shadow-md">
        <CommonBreadcrumbNav
          :custom-breadcrumbs="[
            { label: 'Centres culturels', to: '/centres' },
            { label: centre.nom },
          ]"
        />

        <div class="relative mt-3">
          <img
            v-if="centre.image_couverture_url"
            class="w-full rounded-xl h-48 md:h-72 object-cover"
            :src="centre.image_couverture_url"
            :alt="centre.nom"
          />
          <div
            v-else
            class="w-full rounded-xl h-48 md:h-72 bg-gray-300 flex items-center justify-center"
          >
            <font-awesome-icon class="text-gray-400 text-6xl" :icon="['fas', 'building']" />
          </div>
          <div
            class="bg-gradient-to-t from-black to-transparent absolute h-24 md:h-36 w-full bottom-0 rounded-xl"
          >
            <div class="absolute bottom-3 left-3">
              <div class="text-2xl md:text-4xl text-white font-bold">
                Centre culturel de
                <span class="text-red-500 rounded-md bg-white px-2">{{ centre.nom }}</span>
              </div>
              <div
                class="border mt-2 md:mt-3 inline-flex rounded-md px-2 text-white border-white text-sm md:text-base"
              >
                {{ centre.programmations.length }} événement(s)
              </div>
            </div>
            <div class="absolute bottom-3 right-3">
              <button
                class="transition-all hover:scale-105 active:scale-95 mt-3 inline-flex rounded-md px-3 py-1 bg-custom-green text-white text-sm md:text-base"
                @click="showInscription = true"
              >
                S'inscrire à ce centre
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Localisation + Équipe -->
      <div class="flex flex-col md:flex-row gap-2 mt-3">
        <div
          class="rounded-xl bg-white p-4"
          :class="centre.membres.length > 0 ? 'w-full md:w-1/2' : 'w-full'"
        >
          <div class="flex items-center text-gray-600 border-b-2 pb-2">
            <font-awesome-icon :icon="['fas', 'location-dot']" />
            <div class="text-xl font-extrabold ml-3">Localisation</div>
          </div>
          <div class="mt-3">
            <a
              v-if="googleMapsUrl"
              class="block text-gray-700 hover:text-custom-chocolat transition-colors"
              target="_blank"
              :href="googleMapsUrl"
            >
              <font-awesome-icon :icon="['fas', 'external-link-alt']" class="mr-2" />
              {{ centre.adresse || centre.ville || 'Voir sur la carte' }}
            </a>
            <p v-else class="text-gray-700">{{ centre.adresse || centre.ville || 'Adresse non renseignée' }}</p>
          </div>
          <p v-if="centre.description" class="mt-3 text-gray-600 text-sm">{{ centre.description }}</p>
        </div>

        <CentresCulturelsEquipeSection
          v-if="centre.membres.length > 0"
          :membres="centre.membres"
        />
      </div>

      <!-- Section Programmation -->
      <div class="bg-white rounded-xl mt-3 p-4">
        <div class="text-2xl md:text-3xl text-center font-bold text-gray-800">
          Programmation
        </div>
      </div>

      <!-- Grille des programmations triées (à venir puis passées) -->
      <div
        v-if="programmationsTriees.length > 0"
        class="bg-white mb-4 mt-2 rounded-xl min-h-[20rem] flex flex-wrap justify-center py-4"
      >
        <CentresCulturelsProgrammationCard
          v-for="programmation in programmationsTriees"
          :key="programmation.id"
          :programmation="programmation"
          :site-id="centre.id"
        />
      </div>

      <div
        v-else
        class="bg-white mb-4 mt-2 rounded-xl py-16 text-center"
      >
        <font-awesome-icon :icon="['fas', 'calendar-xmark']" class="text-4xl text-gray-400 mb-3" />
        <p class="text-gray-500">Aucune programmation pour le moment</p>
      </div>
    </div>
  </div>
</template>
