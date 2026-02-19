<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section -->
    <div class="relative h-80 bg-font-centre-culturel bg-cover bg-center">
      <div class="absolute inset-0 bg-linear-to-r from-custom-chocolat/90 to-black/70"></div>

      <div class="absolute inset-0 flex flex-col items-center justify-center mt-5">
        <h1 class="text-white text-4xl md:text-5xl font-bold mb-4 animate-title">
          AfricaCulture
        </h1>
        <div class="h-1 w-24 bg-custom-green rounded animate-line"></div>
        <p class="text-white text-xl md:text-2xl mt-4 animate-subtitle">
          Richesse culturelle africaine
        </p>
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
  title: 'AfricaCulture - UAfricas',
  meta: [
    { name: 'description', content: 'Découvrez la richesse culturelle africaine et les opportunités de la diaspora' },
  ],
})

const breadcrumbs = [
  { label: 'AfricaCulture', to: undefined }
]
</script>

<style scoped>
@reference "~/assets/css/main.css";

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
