<script setup lang="ts">
import { getProgrammationById, formatDateCourteFrancais, formatHeureFrancais, getTypeLabel } from '~/mocks/centres-culturels'
import type { CentreCulturel, Programmation } from '~/mocks/centres-culturels'
import { useUserStore } from '~/stores/user'

const route = useRoute()
const userStore = useUserStore()

const siteId = computed(() => route.params.siteId as string)
const programmationId = computed(() => route.params.programmationId as string)

const centre = ref<CentreCulturel | null>(null)
const programmation = ref<Programmation | null>(null)
const loading = ref(true)

const isAuthenticated = computed(() => userStore.isAuthenticated)

useHead(() => ({
  title: programmation.value
    ? `${programmation.value.titre} - UAfricas`
    : 'Programmation - UAfricas',
  meta: [
    {
      name: 'description',
      content: programmation.value?.description || 'Détails de la programmation culturelle'
    }
  ]
}))

const handleInterest = () => {
  alert('Votre intérêt a été enregistré! (Mode mock)')
}

onMounted(() => {
  const result = getProgrammationById(siteId.value, programmationId.value)
  if (result) {
    centre.value = result.centre
    programmation.value = result.programmation
  }
  loading.value = false
})
</script>

<template>
  <div class="min-h-screen bg-gray-100">
    <!-- Loading state -->
    <div v-if="loading" class="flex justify-center items-center min-h-screen">
      <div class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-custom-green"></div>
    </div>

    <!-- Not found state -->
    <div
      v-else-if="!programmation || !centre"
      class="flex flex-col justify-center items-center min-h-screen"
    >
      <font-awesome-icon :icon="['fas', 'exclamation-triangle']" class="text-6xl text-yellow-500 mb-4" />
      <h1 class="text-2xl font-bold text-gray-700">Programmation non trouvée</h1>
      <p class="text-gray-500 mt-2">La programmation demandée n'existe pas.</p>
      <NuxtLink
        :to="`/site/${siteId}`"
        class="mt-4 px-4 py-2 bg-custom-green text-white rounded-md hover:bg-opacity-90 transition-colors"
      >
        Retour au centre
      </NuxtLink>
    </div>

    <!-- Content -->
    <div v-else class="w-full h-full">
      <div
        class="bg-white mx-4 md:mx-16 lg:mx-72 pt-32 px-4 md:px-7 pb-20 rounded-b-md shadow-md"
      >
        <CommonBreadcrumbNav />

        <!-- Info bar -->
        <div
          class="p-3 mt-3 bg-slate-100 rounded-r-md shadow-md border border-l-4 border-l-custom-green"
          data-aos="fade-right"
          data-aos-duration="600"
        >
          <div class="flex flex-wrap items-center text-custom-chocolat gap-2">
            <NuxtLink
              :to="`/site/${siteId}`"
              class="font-bold underline hover:text-custom-green transition-colors"
            >
              {{ centre.nom }}
            </NuxtLink>
            <span class="hidden md:inline">|</span>
            <div class="text-custom-green">
              {{ formatDateCourteFrancais(programmation.dateDebut) }}
              <span class="underline">au</span>
              {{ formatDateCourteFrancais(programmation.dateFin) }}
            </div>
          </div>
          <div class="mt-2 flex items-center text-gray-700">
            <font-awesome-icon :icon="['fas', 'location-dot']" />
            <span class="ml-2">{{ programmation.adress }}</span>
          </div>
          <div class="mt-2 flex items-center">
            <font-awesome-icon class="text-gray-700" :icon="['far', 'clock']" />
            <span class="ml-2 font-bold text-xl md:text-2xl text-custom-chocolat">
              {{ formatHeureFrancais(programmation.dateDebut) }} - {{ formatHeureFrancais(programmation.dateFin) }}
            </span>
          </div>
          <div class="mt-2 text-sm">
            <span class="text-gray-500">Type:</span>
            <span class="ml-1 font-medium">{{ getTypeLabel(programmation.type) }}</span>
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

        <!-- Image couverture -->
        <img
          class="w-full mb-4 h-64 md:h-96 object-cover rounded-xl shadow-lg"
          :src="programmation.couvertureUrl"
          :alt="programmation.titre"
          data-aos="zoom-in"
          data-aos-duration="800"
        />

        <!-- Bouton intérêt -->
        <div
          v-if="isAuthenticated"
          class="mb-4"
          data-aos="fade-up"
          data-aos-duration="600"
        >
          <button
            @click="handleInterest"
            class="px-4 py-2 bg-custom-green text-white rounded-md hover:bg-opacity-90 transition-all hover:scale-105"
          >
            <font-awesome-icon :icon="['fas', 'star']" class="mr-2" />
            Je suis intéressé(e)
          </button>
        </div>
        <div
          v-else
          class="mb-4 border p-3 text-red-600 bg-red-600 border-red-600 bg-opacity-10 rounded-md"
          data-aos="fade-up"
          data-aos-duration="600"
        >
          <p class="mb-2">Connectez-vous pour pouvoir vous inscrire à cette session</p>
          <NuxtLink
            to="/login"
            class="inline-block px-4 py-1 bg-custom-green text-white rounded-md hover:bg-opacity-90 transition-colors"
          >
            Connexion
          </NuxtLink>
        </div>

        <!-- Description -->
        <div
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
            :to="`/site/${siteId}`"
            class="text-custom-chocolat hover:text-custom-green transition-colors"
          >
            <font-awesome-icon :icon="['fas', 'arrow-left']" class="mr-2" />
            Retour au centre
          </NuxtLink>
          <NuxtLink
            to="/africain-afro-americain"
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
