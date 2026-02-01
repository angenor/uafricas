<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section -->
    <div class="relative h-[40vh] bg-font-centre-culturel bg-cover bg-center flex items-center justify-center">
      <div class="absolute inset-0 bg-gradient-to-r from-custom-chocolat/70 to-amber-700/70"></div>
      <div class="relative z-10 text-center text-white px-4">
        <h1 class="text-4xl md:text-5xl font-bold mb-4" data-aos="fade-up">
          {{ promotionValeurPageData.hero.title }}
        </h1>
      </div>
    </div>

    <!-- Breadcrumb Navigation -->
    <div class="backdrop-blur-sm">
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
            class="h-48 relative overflow-hidden flex items-center justify-center bg-gradient-to-br"
            :class="card.gradient"
          >
            <img
              :class="card.imageStyle === 'icon'
                ? 'h-24 w-24 object-contain filter brightness-0 invert opacity-80'
                : 'w-full h-full object-cover mix-blend-overlay'"
              :src="card.image"
              :alt="card.altText"
            />
            <div class="absolute inset-0 bg-gradient-to-t from-black/30 to-transparent"></div>
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
.container {
  @apply max-w-7xl;
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .grid {
    @apply gap-6;
  }
}
</style>
