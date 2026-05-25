<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section (compact, titre ↔ description au survol) -->
    <div class="group relative bg-font-centre-culturel bg-cover bg-center">
      <div class="absolute inset-0 bg-linear-to-r from-custom-chocolat/90 to-black/70"></div>

      <div class="relative max-w-4xl mx-auto px-4 pt-16 pb-6 text-center select-none">
        <!-- Conteneur fixe : le titre et la description se superposent (crossfade au survol) -->
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12">
          <h1 class="absolute inset-0 flex items-center justify-center text-white text-2xl md:text-4xl font-bold transition-opacity duration-300 group-hover:opacity-0">
            Afroculture
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Richesse culturelle africaine
          </p>
        </div>
      </div>
    </div>

    <!-- Breadcrumb -->
    <div class="max-w-7xl mx-auto px-4 py-3">
      <BreadcrumbNav :custom-breadcrumbs="breadcrumbs" />
    </div>

    <!-- Cards Grid -->
    <div class="max-w-7xl mx-auto px-4 pb-12 sm:pb-16">
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 sm:gap-6 lg:gap-8">
        <NuxtLink
          v-for="card in africaCulturePageData.cards"
          :key="card.id"
          :to="card.link"
          class="group relative rounded-2xl shadow-lg hover:shadow-2xl transition-all duration-500 overflow-hidden h-48 sm:h-56 lg:h-64 transform hover:-translate-y-1"
        >
          <img
            class="absolute inset-0 w-full h-full object-cover transition-transform duration-700 group-hover:scale-110"
            :src="card.image"
            :alt="card.title"
            loading="lazy"
          />
          <div class="absolute inset-0 bg-linear-to-t from-black/80 via-black/30 to-transparent" />

          <div class="relative z-10 p-4 sm:p-6 h-full flex flex-col justify-end">
            <div
              class="border-l-4 group-hover:border-l-8 pl-3 sm:pl-4 transition-all duration-300"
              :class="card.borderColor === 'green' ? 'border-custom-green' : 'border-custom-chocolat'"
            >
              <h3
                class="text-lg sm:text-xl lg:text-2xl font-bold text-white mb-1 sm:mb-2 transition-colors duration-300"
                :class="card.borderColor === 'green' ? 'group-hover:text-custom-green' : 'group-hover:text-custom-chocolat'"
              >
                {{ card.title }}
              </h3>
              <p class="text-gray-200 text-xs sm:text-sm line-clamp-2">
                {{ card.description }}
              </p>
            </div>
          </div>
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { africaCulturePageData } from '~/mocks/africa-culture'

useHead({
  title: 'Afroculture - UAfricas',
  meta: [
    { name: 'description', content: 'Découvrez la richesse culturelle africaine et les opportunités de la diaspora' },
  ],
})

const breadcrumbs = [
  { label: 'Afroculture', to: undefined }
]
</script>
