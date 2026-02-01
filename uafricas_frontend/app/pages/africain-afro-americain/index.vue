<script setup lang="ts">
import { centresCulturelsMock, CAROUSEL_IMAGES } from '~/mocks/centres-culturels'
import type { CentreCulturel } from '~/mocks/centres-culturels'

useHead({
  title: 'Centres Culturels Africain et Afro-Descendant - UAfricas',
  meta: [
    {
      name: 'description',
      content: 'Découvrez les centres culturels africains et afro-descendants à travers le monde. Événements, programmations et activités culturelles.'
    }
  ]
})

const centres = ref<CentreCulturel[]>([])
const carouselImages = ref<string[]>(CAROUSEL_IMAGES)

onMounted(() => {
  centres.value = centresCulturelsMock
})
</script>

<template>
  <div class="min-h-screen bg-gray-100">
    <div class="pt-28 mx-4 md:mx-16 lg:mx-56 pb-10">
      <!-- Carrousel -->
      <CentresCulturelsCentreCulturelCarousel :images="carouselImages" />

      <!-- Hero section -->
      <CentresCulturelsCentreCulturelHero :total="centres.length" />

      <!-- Liste des centres -->
      <div class="mt-4">
        <NuxtLink
          v-for="centre in centres"
          :key="centre.id"
          :to="`/site/${centre.id}`"
        >
          <CentresCulturelsCentreCulturelCard :centre="centre" />
        </NuxtLink>
      </div>
    </div>
  </div>
</template>
