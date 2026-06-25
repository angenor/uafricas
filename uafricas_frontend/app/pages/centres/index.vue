<script setup lang="ts">
import type { CentreCulturelAPI } from '~/composables/useCentresCulturels'

useAOS()

const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

// Images fixes du carrousel.
const carouselImages: string[] = [
  'https://africangreens.org/wp-content/uploads/2024/07/joyinafrica.jpg',
  'https://static.vecteezy.com/ti/vecteur-libre/p1/3500664-banniere-culture-africaine-tradition-vectoriel.jpg',
]

useHead({
  title: 'Centres culturels africains et afro-descendants – UAfricas',
  meta: [
    {
      name: 'description',
      content: 'Découvrez les centres culturels africains et afro-descendants à travers le monde. Événements, programmations et activités culturelles.',
    },
  ],
})

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

// Répartition par type : internationaux d'abord, puis locaux.
const centresInternationaux = computed(() =>
  centres.value.filter(c => c.type_centre === 'international'),
)
const centresLocaux = computed(() =>
  centres.value.filter(c => c.type_centre !== 'international'),
)
</script>

<template>
  <div class="min-h-screen bg-gray-100">
    <div class="pt-28 mx-4 md:mx-16 lg:mx-56 pb-10">
      <!-- Carrousel -->
      <CentresCulturelsCentreCulturelCarousel :images="carouselImages" />

      <!-- Hero section -->
      <CentresCulturelsCentreCulturelHero :total="centres.length" />


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
      <div v-else>
        <!-- Section : Centres culturels internationaux -->
        <section v-if="centresInternationaux.length" class="mt-8">
          <div class="flex items-center gap-3 mb-4">
            <font-awesome-icon :icon="['fas', 'earth-africa']" class="text-custom-chocolat text-xl" />
            <h2 class="text-xl md:text-2xl font-bold text-custom-chocolat">
              Africans International
            </h2>
            <span class="text-sm font-medium text-custom-green border border-custom-chocolat rounded-md px-2.5 py-0.5">
              {{ centresInternationaux.length }}
            </span>
          </div>
          <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-5">
            <NuxtLink
              v-for="centre in centresInternationaux"
              :key="centre.id"
              :to="`/centres/${centre.id}`"
              class="block"
            >
              <CentresCulturelsCentreCulturelCard :centre="centre" />
            </NuxtLink>
          </div>
        </section>

        <!-- Section : Centres culturels locaux -->
        <section v-if="centresLocaux.length" class="mt-10">
          <div class="flex items-center gap-3 mb-4">
            <font-awesome-icon :icon="['fas', 'location-dot']" class="text-custom-chocolat text-xl" />
            <h2 class="text-xl md:text-2xl font-bold text-custom-chocolat">
              Centres culturels locaux
            </h2>
            <span class="text-sm font-medium text-custom-green border border-custom-chocolat rounded-md px-2.5 py-0.5">
              {{ centresLocaux.length }}
            </span>
          </div>
          <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-5">
            <NuxtLink
              v-for="centre in centresLocaux"
              :key="centre.id"
              :to="`/centres/${centre.id}`"
              class="block"
            >
              <CentresCulturelsCentreCulturelCard :centre="centre" />
            </NuxtLink>
          </div>
        </section>

        <!-- Aucun centre -->
        <div v-if="centres.length === 0" class="text-center py-16">
          <p class="text-gray-500 text-lg">Aucun centre culturel pour le moment</p>
        </div>
      </div>
    </div>
  </div>
</template>
