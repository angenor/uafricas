<template>
  <div class="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100">
    <!-- Hero Section -->
    <div class="relative h-80 w-full overflow-hidden z-0">
      <div class="absolute inset-0 bg-font-centre-culturel bg-cover bg-center">
        <div class="absolute inset-0 bg-gradient-to-r from-black/60 via-black/40 to-transparent"></div>
        <div class="absolute inset-0 backdrop-blur-sm bg-black/20"></div>
      </div>
      <div class="relative z-10 flex items-center justify-center h-full">
        <div class="text-center text-white px-4 bg-white/10 rounded-2xl">
          <h1 class="text-5xl md:text-6xl font-bold mb-8 tracking-tight">
            {{ africaCulturePageData.hero.title }}
          </h1>
        </div>
      </div>
    </div>

    <!-- Breadcrumb Navigation -->
    <div class="backdrop-blur-sm">
      <div class="mx-auto px-4 py-3">
        <BreadcrumbNav :custom-breadcrumbs="breadcrumbs" />
      </div>
    </div>

    <!-- Main Content -->
    <div class="container mx-auto px-4 pb-16 z-50 relative -top-32">
      <!-- Cards Grid -->
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-8 max-w-7xl mx-auto">
        <NuxtLink
          v-for="card in africaCulturePageData.cards"
          :key="card.id"
          :to="card.link"
          class="group"
        >
          <div
            @mouseenter="sectionHover = card.id"
            @mouseleave="sectionHover = 0"
            class="relative bg-white rounded-2xl shadow-lg hover:shadow-2xl transition-all duration-500 overflow-hidden h-64 transform hover:-translate-y-2"
          >
            <div class="absolute inset-0">
              <img
                :class="sectionHover === card.id ? 'scale-110' : 'scale-100'"
                class="w-full h-full object-cover transition-transform duration-700"
                :src="card.image"
                :alt="card.title"
              />
              <div class="absolute inset-0 bg-gradient-to-t from-black/80 via-black/30 to-transparent"></div>
            </div>
            <div class="relative z-10 p-6 h-full flex flex-col justify-end">
              <div
                class="border-l-4 pl-4 transition-all duration-300"
                :class="[
                  getBorderColorClass(card.borderColor),
                  { 'border-l-8': sectionHover === card.id }
                ]"
              >
                <h3
                  class="text-2xl font-bold text-white mb-2 transition-colors duration-300"
                  :class="getHoverTextClass(card.borderColor)"
                >
                  {{ card.title }}
                </h3>
                <p class="text-gray-200 text-sm opacity-90">
                  {{ card.description }}
                </p>
              </div>
            </div>
          </div>
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  africaCulturePageData,
  getBorderColorClass,
  getHoverTextClass
} from '~/mocks/africa-culture'

useHead({
  title: 'AfricaCulture - UAfricas',
  meta: [
    { name: 'description', content: 'Découvrez la richesse culturelle africaine et les opportunités de la diaspora' },
  ],
})

const sectionHover = ref(0)

const breadcrumbs = [
  {
    label: 'AfricaCulture',
    to: undefined
  }
]
</script>

<style scoped>
.container {
  @apply max-w-7xl;
}

/* Effet de parallaxe subtil */
@media (prefers-reduced-motion: no-preference) {
  .group:hover img {
    transform: scale(1.1) rotate(1deg);
  }
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .grid {
    @apply grid-cols-1 gap-6;
  }

  .h-64 {
    @apply h-56;
  }
}
</style>
