<script setup lang="ts">
import { getCentreById } from '~/mocks/centres-culturels'
import type { CentreCulturel } from '~/mocks/centres-culturels'
import { useUserStore } from '~/stores/user'

const route = useRoute()
const userStore = useUserStore()

const id = computed(() => route.params.id as string)

const centre = ref<CentreCulturel | null>(null)
const loading = ref(true)
const showInscription = ref(false)
const showCreateProg = ref(false)

const isAdmin = computed(() => userStore.user?.roles?.includes('admin'))

useHead(() => ({
  title: centre.value
    ? `Centre Culturel de ${centre.value.nom} - UAfricas`
    : 'Centre Culturel - UAfricas',
  meta: [
    {
      name: 'description',
      content: centre.value
        ? `Découvrez le centre culturel de ${centre.value.nom}. ${centre.value.programmations.length} événements programmés.`
        : 'Centre culturel africain et afro-descendant'
    }
  ]
}))

const handleInscriptionSubmit = (options: { prioritaires: boolean; toutes: boolean }) => {
  console.log('Inscription avec options:', options)
  alert('Inscription enregistrée avec succès!')
  showInscription.value = false
}

const handleCreateProgrammation = (programmation: any) => {
  console.log('Nouvelle programmation:', programmation)
  alert('Programmation créée avec succès! (Mode mock - non persisté)')
  showCreateProg.value = false
}

onMounted(() => {
  const foundCentre = getCentreById(id.value)
  if (foundCentre) {
    centre.value = foundCentre
  }
  loading.value = false
})
</script>

<template>
  <div class="min-h-screen bg-gray-100">
    <!-- Modals -->
    <CentresCulturelsInscriptionModal
      :is-open="showInscription"
      @close="showInscription = false"
      @submit="handleInscriptionSubmit"
    />

    <CentresCulturelsCreateProgrammationModal
      v-if="centre"
      :is-open="showCreateProg"
      :centre-id="centre.id"
      @close="showCreateProg = false"
      @submit="handleCreateProgrammation"
    />

    <!-- Loading state -->
    <div v-if="loading" class="flex justify-center items-center min-h-screen">
      <div class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-custom-green"></div>
    </div>

    <!-- Not found state -->
    <div
      v-else-if="!centre"
      class="flex flex-col justify-center items-center min-h-screen"
    >
      <font-awesome-icon :icon="['fas', 'exclamation-triangle']" class="text-6xl text-yellow-500 mb-4" />
      <h1 class="text-2xl font-bold text-gray-700">Centre non trouvé</h1>
      <p class="text-gray-500 mt-2">Le centre culturel demandé n'existe pas.</p>
      <NuxtLink
        to="/africain-afro-americain"
        class="mt-4 px-4 py-2 bg-custom-green text-white rounded-md hover:bg-opacity-90 transition-colors"
      >
        Retour à la liste
      </NuxtLink>
    </div>

    <!-- Content -->
    <div v-else class="h-full mx-4 md:mx-16 lg:mx-56">
      <!-- Header avec bannière -->
      <div class="bg-white pt-32 px-4 md:px-7 pb-5 rounded-b-xl shadow-md">
        <CommonBreadcrumbNav />

        <div class="relative mt-3">
          <img
            class="w-full rounded-xl h-48 md:h-72 object-cover"
            :src="centre.urlBanniere"
            :alt="centre.nom"
          />
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
                @click="showInscription = true"
                class="transition-all hover:scale-105 active:scale-95 mt-3 inline-flex rounded-md px-3 py-1 bg-custom-green text-white text-sm md:text-base"
              >
                S'inscrire à ce centre
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Équipe et Localisation -->
      <div class="flex flex-col md:flex-row gap-2 mt-3">
        <CentresCulturelsEquipeSection
          :president="centre.president"
          :vice-president="centre.vicePresident"
          :resp-communication="centre.respCommunication"
        />

        <div class="rounded-xl bg-white w-full md:w-1/2 p-4">
          <div class="flex items-center text-gray-600 border-b-2 pb-2">
            <font-awesome-icon :icon="['fas', 'location-dot']" />
            <div class="text-xl font-extrabold ml-3">Localisation</div>
          </div>
          <a
            v-if="centre.urlGoogleMap"
            class="mt-3 block text-gray-700 hover:text-custom-chocolat transition-colors"
            target="_blank"
            :href="centre.urlGoogleMap"
          >
            <font-awesome-icon :icon="['fas', 'external-link-alt']" class="mr-2" />
            {{ centre.adress }}
          </a>
          <p v-else class="mt-3 text-gray-700">{{ centre.adress }}</p>
        </div>
      </div>

      <!-- Section Programmation -->
      <div class="bg-white rounded-xl mt-3 p-4">
        <div class="text-2xl md:text-3xl text-center font-bold text-gray-800">
          Programmation
        </div>
      </div>

      <!-- Grille des programmations -->
      <div
        v-if="centre.programmations.length > 0"
        class="bg-white mb-4 mt-2 rounded-xl min-h-[20rem] flex flex-wrap justify-center py-4"
      >
        <CentresCulturelsProgrammationCard
          v-for="programmation in centre.programmations"
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

      <!-- Bouton Admin -->
      <button
        v-if="isAdmin"
        @click="showCreateProg = true"
        class="bg-custom-chocolat text-white rounded-md mb-4 px-4 py-2 hover:bg-opacity-90 transition-colors"
      >
        <font-awesome-icon :icon="['fas', 'plus']" class="mr-2" />
        Ajouter une programmation
      </button>
    </div>
  </div>
</template>
