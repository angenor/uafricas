<template>
  <div class="min-h-screen pb-10 bg-gray-50">
    <!-- Hero Section (compact, titre ↔ description au survol) -->
    <div
      class="group relative bg-cover bg-center z-0"
      :style="`background-image: url('${universitePageData.hero.backgroundImage}')`"
    >
      <div class="absolute inset-0 bg-gradient-to-r from-custom-chocolat/90 to-black/70"></div>

      <div class="relative max-w-4xl mx-auto px-4 pt-16 pb-6 text-center select-none">
        <!-- Conteneur fixe : le titre et la description se superposent (crossfade au survol) -->
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12">
          <h1 class="absolute inset-0 flex items-center justify-center text-white text-2xl md:text-4xl font-bold transition-opacity duration-300 group-hover:opacity-0">
            {{ universitePageData.hero.title }}
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            {{ universitePageData.hero.subtitle }}
          </p>
        </div>
      </div>
    </div>

    <!-- Liste des categories -->
    <div class="max-w-6xl mx-auto px-4 mt-10">
      <div class="grid md:grid-cols-2 gap-8">
        <NuxtLink
          v-for="category in universitePageData.categories"
          :key="category.id"
          :to="category.link"
        >
          <div class="group cursor-pointer transform transition-all duration-500 hover:scale-105">
            <div class="bg-white rounded-2xl shadow-lg overflow-hidden hover:shadow-2xl transition-all duration-300">
              <div class="relative overflow-hidden">
                <img
                  class="w-full h-64 object-cover transition-transform duration-500 group-hover:scale-110"
                  :src="category.image"
                  :alt="category.title"
                />
                <div class="absolute inset-0 bg-gradient-to-t from-black/50 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                <div class="absolute top-4 right-4">
                  <div
                    class="text-white px-3 py-1 rounded-full text-sm font-medium"
                    :class="category.badgeColor === 'green' ? 'bg-custom-green' : 'bg-custom-chocolat'"
                  >
                    {{ category.badge }}
                  </div>
                </div>
              </div>
              <div class="p-6">
                <h3 class="text-2xl font-bold text-gray-800 mb-1 group-hover:text-custom-green transition-colors">
                  {{ category.title }}
                </h3>
                <p class="text-gray-600 mb-2">
                  {{ category.description }}
                </p>
                <div class="text-gray-600 mb-4 flex flex-wrap">
                  <div
                    v-for="tag in category.tags"
                    :key="tag"
                    class="border border-custom-green px-2 m-1 text-custom-green rounded-full text-sm"
                  >
                    {{ tag }}
                  </div>
                </div>
                <div class="flex items-center justify-between">
                  <span class="text-custom-chocolat font-semibold">Explorer →</span>
                </div>
              </div>
            </div>
          </div>
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { universitePageData } from '~/mocks/universite'

useHead({
  title: 'Université - UAfricas',
  meta: [
    { name: 'description', content: 'Explorez nos programmes universitaires - Gouvernance citoyenne et INUDA' },
  ],
})

useAOS()
</script>
