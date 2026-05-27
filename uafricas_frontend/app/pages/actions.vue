<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section (compact, titre ↔ description au survol) -->
    <div
      class="group relative bg-cover bg-center"
      style="background-image: url('https://images.unsplash.com/photo-1504384308090-c894fdcc538d?ixlib=rb-1.2.1&auto=format&fit=crop&w=1900&q=80')"
    >
      <div class="absolute inset-0 bg-linear-to-r from-custom-chocolat/90 to-black/70"></div>

      <div class="relative max-w-4xl mx-auto px-4 pt-16 pb-6 text-center select-none">
        <!-- Conteneur fixe : le titre et la description se superposent (crossfade au survol) -->
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12">
          <h1 class="absolute inset-0 flex items-center justify-center text-white text-2xl md:text-4xl font-bold transition-opacity duration-300 group-hover:opacity-0">
            Actions
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Initiatives pour l'Afrique
          </p>
        </div>
      </div>
    </div>

    <!-- Section des cartes avec animation -->
    <div class="relative z-40 px-4 py-16">
      <TransitionGroup
        name="stagger-fade"
        tag="div"
        class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-8 max-w-7xl mx-auto"
      >
        <div
          v-for="(card, index) in actionCards"
          :key="card.id"
          ref="cardRefs"
          class="card-wrapper transform translate-y-12 opacity-0"
          :style="{ transitionDelay: `${index * 100}ms` }"
        >
          <NuxtLink :to="card.link" class="block">
            <div
              @mouseenter="sectionHover = card.id"
              @mouseleave="sectionHover = 0"
              class="relative h-[350px] rounded-2xl overflow-hidden shadow-lg transition-all duration-500 cursor-pointer"
              :class="{
                'transform -translate-y-2 shadow-2xl': sectionHover === card.id,
              }"
            >
              <div
                class="absolute inset-0 bg-linear-to-b from-black/10 to-black/80 z-20"
              ></div>
              <img
                :src="card.image"
                :alt="card.title"
                class="absolute inset-0 w-full h-full object-cover z-10 transition-transform duration-1500"
                :class="{ 'scale-110': sectionHover === card.id }"
              />
              <div
                class="absolute top-4 right-4 w-12 h-12 rounded-full flex items-center justify-center z-30 transition-transform duration-300"
                :class="[
                  index % 2 === 0
                    ? 'bg-green-700 shadow-green-700/30 shadow-md'
                    : 'bg-amber-900 shadow-amber-800/30 shadow-md',
                  {
                    'transform scale-110 rotate-12': sectionHover === card.id,
                  },
                ]"
              >
                <div class="text-white text-xl">
                  <font-awesome-icon :icon="card.icon" />
                </div>
              </div>
              <div
                class="absolute bottom-0 left-0 w-full p-8 z-30 text-white transition-transform duration-500"
                :class="{
                  'transform -translate-y-2': sectionHover === card.id,
                }"
              >
                <h3 class="text-2xl font-bold mb-3 relative card-title">
                  {{ card.title }}
                </h3>
                <div
                  class="overflow-hidden transition-all duration-500"
                  :class="{
                    'max-h-0 opacity-0': sectionHover !== card.id,
                    'max-h-40 opacity-100 mt-4': sectionHover === card.id,
                  }"
                >
                  <p class="mb-4 text-sm leading-relaxed">
                    {{ card.description }}
                  </p>
                  <span
                    class="inline-block text-yellow-400 font-semibold transition-transform duration-300 group"
                  >
                    En savoir plus
                    <font-awesome-icon
                      icon="fa-solid fa-arrow-right"
                      class="ml-1 transition-transform duration-300 transform group-hover:translate-x-1"
                    />
                  </span>
                </div>
              </div>
            </div>
          </NuxtLink>
        </div>
      </TransitionGroup>
    </div>
  </div>
</template>

<script setup lang="ts">
import gsap from 'gsap'
import { actionCards } from '~/mocks/actions'

useHead({
  title: 'Actions - UAfricas',
  meta: [
    { name: 'description', content: "Découvrez nos initiatives pour l'Afrique" },
  ],
})

const sectionHover = ref(0)
const cardRefs = ref<HTMLElement[]>([])

onMounted(() => {
  // Animation des cartes
  gsap.to('.card-wrapper', {
    opacity: 1,
    y: 0,
    stagger: 0.1,
    duration: 0.8,
    ease: 'power3.out',
    delay: 0.5,
  })
})
</script>

<style scoped>
.card-title::after {
  content: '';
  position: absolute;
  left: 0;
  bottom: -8px;
  width: 50px;
  height: 3px;
  background-color: #facc15; /* yellow-400 */
  transition: width 0.3s ease;
}

div:hover .card-title::after {
  width: 80px;
}

/* Transitions pour les cartes */
.stagger-fade-enter-active,
.stagger-fade-leave-active {
  transition: opacity 0.5s ease, transform 0.5s ease;
}

.stagger-fade-enter-from,
.stagger-fade-leave-to {
  opacity: 0;
  transform: translateY(30px);
}

/* Duration for image scale transition */
.duration-1500 {
  transition-duration: 1500ms;
}
</style>
