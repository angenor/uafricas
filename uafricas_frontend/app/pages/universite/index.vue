<template>
  <div class="min-h-screen pb-10 bg-gray-50">
    <!-- Hero Section -->
    <div
      class="relative h-80 bg-cover bg-center z-0"
      :style="`background-image: url('${universitePageData.hero.backgroundImage}')`"
    >
      <div class="absolute inset-0 bg-gradient-to-r from-custom-chocolat/90 to-black/70"></div>

      <!-- Texte avec animation d'entree -->
      <div class="absolute inset-0 flex flex-col items-center justify-center mt-14">
        <h1 class="text-white text-4xl md:text-5xl font-bold mb-4 animate-title">
          {{ universitePageData.hero.title }}
        </h1>
        <div class="h-1 w-24 bg-custom-green rounded animate-line"></div>
        <p class="text-white text-xl md:text-2xl mt-4 animate-subtitle text-center px-4">
          {{ universitePageData.hero.subtitle }}
        </p>
      </div>
    </div>

    <!-- Liste des categories -->
    <div class="max-w-6xl mx-auto px-4 relative -top-10">
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

<style scoped>
@keyframes fadeIn {
  from { opacity: 0; transform: translateY(-20px); }
  to { opacity: 1; transform: translateY(0); }
}

@keyframes expandLine {
  from { width: 0; }
  to { width: 6rem; }
}

.animate-title {
  animation: fadeIn 1s ease-out forwards;
}

.animate-subtitle {
  animation: fadeIn 1s ease-out 0.3s forwards;
  opacity: 0;
}

.animate-line {
  animation: expandLine 1.2s ease-out 0.1s forwards;
  width: 0;
}
</style>
