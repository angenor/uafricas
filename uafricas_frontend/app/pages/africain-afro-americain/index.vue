<script setup lang="ts">
import { CAROUSEL_IMAGES } from '~/mocks/centres-culturels'
import type { CentreCulturelAPI } from '~/composables/useCentresCulturels'

useAOS()

const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

useHead({
  title: 'Centres Culturels Africain et Afro-Descendant - UAfricas',
  meta: [
    {
      name: 'description',
      content: 'Découvrez les centres culturels africains et afro-descendants à travers le monde. Événements, programmations et activités culturelles.'
    }
  ]
})

const carouselImages = CAROUSEL_IMAGES

const { data: centresData, status, error: fetchError, refresh } = await useAsyncData(
  'centres-culturels',
  async () => {
    const reponse = await $fetch<ApiResponse<CentreCulturelAPI[]>>(
      `${apiBase}/api/centres-culturels`,
    )
    if (!reponse.success || !reponse.data) {
      throw createError({ message: reponse.error || 'Erreur lors du chargement des centres culturels' })
    }
    return reponse.data.map(c => ({
      ...c,
      image_couverture_url: c.image_couverture_url
        ? `${apiBase}${c.image_couverture_url}`
        : null,
    }))
  },
)

const centres = computed(() => centresData.value ?? [])
const chargement = computed(() => status.value === 'pending')
const erreur = computed(() => fetchError.value?.message ?? null)
</script>

<template>
  <div class="min-h-screen bg-gray-100">
    <div class="pt-28 mx-4 md:mx-16 lg:mx-56 pb-10">
      <!-- Carrousel -->
      <CentresCulturelsCentreCulturelCarousel :images="carouselImages" />

      <!-- Hero section -->
      <CentresCulturelsCentreCulturelHero :total="centres.length" />

      <!-- Description Afroculture -->
      <div class="mt-4 p-4 bg-linear-to-r from-custom-chocolat/5 to-custom-green/5 border-l-4 border-custom-green rounded-r-lg">
        <p class="text-gray-700 text-sm md:text-base leading-relaxed">
          <span class="font-semibold text-custom-chocolat">Afroculture</span> — Enrichissons nous ici et ailleurs de notre culture diversifiée
        </p>
      </div>

      <!-- Chargement -->
      <div v-if="chargement" class="flex justify-center items-center py-16">
        <div class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-custom-green"></div>
      </div>

      <!-- Erreur -->
      <div v-else-if="erreur" class="bg-red-50 border border-red-200 rounded-xl p-6 mt-4 text-center">
        <p class="text-red-600">{{ erreur }}</p>
        <button
          class="mt-3 px-4 py-2 bg-custom-green text-white rounded-md hover:bg-custom-green/90 transition-colors"
          @click="refresh()"
        >
          Réessayer
        </button>
      </div>

      <!-- Liste des centres -->
      <div v-else class="mt-4">
        <NuxtLink
          v-for="centre in centres"
          :key="centre.id"
          :to="`/site/${centre.id}`"
        >
          <CentresCulturelsCentreCulturelCard :centre="centre" />
        </NuxtLink>

        <!-- Aucun centre -->
        <div v-if="centres.length === 0" class="text-center py-16">
          <p class="text-gray-500 text-lg">Aucun centre culturel pour le moment</p>
        </div>
      </div>
    </div>
  </div>
</template>
