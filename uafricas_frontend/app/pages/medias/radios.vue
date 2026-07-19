<script setup lang="ts">
import { radioCategories, radioStats } from '~/mocks/radios'

useHead({
  title: 'Radios Africaines | UAfricas',
  meta: [
    { name: 'description', content: 'Découvrez les meilleures stations de radio africaines. Écoutez en direct les radios africaines internationales et nationales.' }
  ]
})

useAOS()

// Modale de présentation « C'est quoi Africans Radio ? »
const presentationOuverte = ref(false)
</script>

<template>
  <div class="min-h-screen pb-10 bg-gray-50">
    <!-- Hero Section (compact, titre ↔ description au survol) -->
    <div
      class="group relative bg-cover bg-center z-0"
      style="background-image: url('/images/banners/radio-home.jpg')"
    >
      <div class="absolute inset-0 bg-linear-to-r from-custom-chocolat/90 to-black/70"></div>

      <div class="relative max-w-4xl mx-auto px-4 pt-16 pb-6 text-center select-none">
        <!-- Conteneur fixe : le titre et la description se superposent (crossfade au survol) -->
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12">
          <h1 class="absolute inset-0 flex items-center justify-center text-white text-2xl md:text-4xl font-bold transition-opacity duration-300 group-hover:opacity-0">
            Radios Africaines
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Écoutez l'Afrique en direct
          </p>
        </div>

        <div class="mt-4 flex flex-wrap items-center justify-center gap-3">
          <!-- Bouton d'aide : ouvre la présentation d'Africans Radio -->
          <button
            type="button"
            class="inline-flex items-center gap-2 rounded-full bg-white/15 hover:bg-white/25 text-white font-medium text-sm px-4 py-2.5 backdrop-blur-xs ring-1 ring-white/25 transition-colors"
            aria-label="En savoir plus sur Africans Radio"
            @click="presentationOuverte = true"
          >
            <font-awesome-icon :icon="['fas', 'circle-question']" class="w-4 h-4" />
            C'est quoi Africans Radio&nbsp;?
          </button>
        </div>
      </div>
    </div>

    <!-- Modale de présentation « C'est quoi Africans Radio ? » -->
    <MediaRadioPresentationModal
      :open="presentationOuverte"
      @close="presentationOuverte = false"
    />

    <!-- Navigation breadcrumb -->
    <div class="max-w-6xl mx-auto px-4 pt-4">
      <CommonBreadcrumbNav />
    </div>

    <!-- Cartes des catégories -->
    <div class="max-w-6xl mx-auto px-4 relative pt-6">
      <div class="grid md:grid-cols-2 gap-8">
        <NuxtLink
          v-for="category in radioCategories"
          :key="category.id"
          :to="category.link"
          data-aos="fade-up"
          :data-aos-delay="category.id * 100"
        >
          <div class="group cursor-pointer transform transition-all duration-500 hover:scale-105">
            <div class="relative overflow-hidden rounded-2xl shadow-xl h-64">
              <img
                :src="category.image"
                :alt="category.title"
                class="w-full h-full object-cover transition-transform duration-700 group-hover:scale-110"
              />
              <div class="absolute inset-0 bg-linear-to-t from-black/80 via-black/40 to-transparent"></div>

              <!-- Badge -->
              <div class="absolute top-4 right-4">
                <span
                  :class="[
                    'px-3 py-1 rounded-full text-sm font-semibold text-white',
                    category.badgeColor === 'green' ? 'bg-custom-green' : 'bg-custom-chocolat'
                  ]"
                >
                  {{ category.badge }}
                </span>
              </div>

              <!-- Contenu -->
              <div class="absolute bottom-0 left-0 right-0 p-6">
                <h3 class="text-2xl font-bold text-white mb-2">
                  {{ category.title }}
                </h3>
                <p class="text-gray-200 text-sm line-clamp-2">
                  {{ category.description }}
                </p>
                <div class="mt-4 flex items-center text-custom-green group-hover:translate-x-2 transition-transform">
                  <span class="text-sm font-semibold">Explorer</span>
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 ml-2" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z" clip-rule="evenodd" />
                  </svg>
                </div>
              </div>
            </div>
          </div>
        </NuxtLink>
      </div>

      <!-- Section Statistiques -->
      <div
        class="mt-16 bg-linear-to-r from-custom-green to-custom-chocolat rounded-2xl p-8 text-white"
        data-aos="fade-up"
      >
        <div class="grid grid-cols-2 md:grid-cols-4 gap-6 text-center">
          <div v-for="stat in radioStats" :key="stat.label" class="p-4">
            <div class="text-4xl font-bold mb-2">{{ stat.value }}</div>
            <div class="text-sm opacity-80">{{ stat.label }}</div>
          </div>
        </div>
      </div>

      <!-- Section Description -->
      <div class="mt-16 text-center" data-aos="fade-up">
        <h2 class="text-3xl font-bold text-gray-800 mb-6">
          Votre passerelle vers les sons africains
        </h2>
        <p class="text-gray-600 max-w-3xl mx-auto leading-relaxed">
          Plongez dans la diversité musicale et culturelle de l'Afrique à travers nos stations de radio soigneusement sélectionnées.
          Des rythmes traditionnels aux hits afrobeats les plus récents, découvrez la richesse sonore du continent africain.
        </p>
      </div>

      <!-- Features -->
      <div class="mt-12 grid md:grid-cols-3 gap-8" data-aos="fade-up">
        <div class="text-center p-6 bg-white rounded-xl shadow-lg">
          <div class="w-16 h-16 mx-auto mb-4 bg-custom-green/10 rounded-full flex items-center justify-center">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 text-custom-green" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clip-rule="evenodd" />
            </svg>
          </div>
          <h3 class="text-lg font-semibold text-gray-800 mb-2">Streaming en direct</h3>
          <p class="text-gray-600 text-sm">Écoutez vos stations préférées en temps réel, où que vous soyez.</p>
        </div>

        <div class="text-center p-6 bg-white rounded-xl shadow-lg">
          <div class="w-16 h-16 mx-auto mb-4 bg-custom-chocolat/10 rounded-full flex items-center justify-center">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 text-custom-chocolat" viewBox="0 0 20 20" fill="currentColor">
              <path d="M18 3a1 1 0 00-1.196-.98l-10 2A1 1 0 006 5v9.114A4.369 4.369 0 005 14c-1.657 0-3 .895-3 2s1.343 2 3 2 3-.895 3-2V7.82l8-1.6v5.894A4.37 4.37 0 0015 12c-1.657 0-3 .895-3 2s1.343 2 3 2 3-.895 3-2V3z" />
            </svg>
          </div>
          <h3 class="text-lg font-semibold text-gray-800 mb-2">Diversité musicale</h3>
          <p class="text-gray-600 text-sm">Afrobeats, Mbalax, Rumba, Highlife et bien plus encore.</p>
        </div>

        <div class="text-center p-6 bg-white rounded-xl shadow-lg">
          <div class="w-16 h-16 mx-auto mb-4 bg-custom-green/10 rounded-full flex items-center justify-center">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 text-custom-green" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z" clip-rule="evenodd" />
            </svg>
          </div>
          <h3 class="text-lg font-semibold text-gray-800 mb-2">24h/24, 7j/7</h3>
          <p class="text-gray-600 text-sm">Accédez à vos stations favorites à tout moment.</p>
        </div>
      </div>
    </div>
  </div>
</template>
