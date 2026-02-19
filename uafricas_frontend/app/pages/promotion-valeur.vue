<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section -->
    <div class="relative h-80 bg-font-centre-culturel bg-cover bg-center">
      <div class="absolute inset-0 bg-linear-to-r from-custom-chocolat/90 to-black/70"></div>

      <div class="absolute inset-0 flex flex-col items-center justify-center mt-5">
        <h1 class="text-white text-4xl md:text-5xl font-bold mb-4 animate-title">
          Promotion des Valeurs
        </h1>
        <div class="h-1 w-24 bg-custom-green rounded animate-line"></div>
        <p class="text-white text-xl md:text-2xl mt-4 animate-subtitle">
          Valeurs africaines et afro-descendantes
        </p>
      </div>
    </div>

    <!-- Breadcrumb Navigation -->
    <div class="backdrop-blur-xs">
      <div class="mx-auto px-4 py-3">
        <CommonBreadcrumbNav :custom-breadcrumbs="breadcrumbs" />
      </div>
    </div>

    <!-- Main Content -->
    <div class="container mx-auto px-4 py-12">
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-8 mb-12">
        <div
          v-for="card in promotionValeurPageData.cards"
          :key="card.id"
          class="bg-white rounded-xl shadow-lg overflow-hidden hover:shadow-xl transition-all duration-300 transform hover:-translate-y-2"
          data-aos="fade-up"
          :data-aos-delay="card.id * 100"
          :data-aos-duration="800"
        >
          <!-- Image avec gradient -->
          <div
            class="h-48 relative overflow-hidden flex items-center justify-center bg-linear-to-br"
            :class="card.gradient"
          >
            <img
              :class="card.imageStyle === 'icon'
                ? 'h-24 w-24 object-contain filter brightness-0 invert opacity-80'
                : 'w-full h-full object-cover mix-blend-overlay'"
              :src="card.image"
              :alt="card.altText"
            />
            <div class="absolute inset-0 bg-linear-to-t from-black/30 to-transparent"></div>
          </div>

          <!-- Contenu -->
          <div class="p-6">
            <h3 class="text-2xl font-bold text-gray-800 mb-3">{{ card.title }}</h3>
            <p class="text-gray-600 mb-4">{{ card.description }}</p>
            <NuxtLink
              :to="card.link"
              class="inline-flex items-center gap-2 bg-custom-green text-white px-6 py-3 rounded-full hover:bg-green-600 transition-colors duration-300"
            >
              {{ card.buttonText }}
              <font-awesome-icon icon="fa-solid fa-arrow-right" />
            </NuxtLink>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { promotionValeurPageData } from '~/mocks/promotion-valeur'

useHead({
  title: 'Promotion des Valeurs Africaines - UAfricas',
  meta: [
    {
      name: 'description',
      content: 'Découvrez les valeurs africaines et afro-descendantes à travers ForAfrica, Afrocult et Afromarket',
    },
  ],
})

useAOS()

const breadcrumbs = [
  { label: 'AfricaCulture', to: '/africa-culture' },
  { label: 'Promotion des Valeurs', to: undefined },
]
</script>

<style scoped>
@reference "~/assets/css/main.css";

.container {
  @apply max-w-7xl;
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .grid {
    @apply gap-6;
  }
}

/* Hero animations */
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
