<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section avec animation parallaxe -->
    <div
      class="relative flex items-center justify-center overflow-hidden h-[60vh] min-h-[400px] text-white"
    >
      <div class="parallax-bg absolute inset-0 z-10"></div>
      <div
        class="absolute inset-0 bg-gradient-to-b from-black/50 to-black/70 z-20"
      ></div>
      <div ref="heroContent" class="relative z-30 text-center px-4 max-w-3xl">
        <h1 class="text-shimmer text-5xl md:text-6xl font-extrabold mb-4">
          Actions à mener
        </h1>
        <p class="text-xl md:text-2xl opacity-90 max-w-xl mx-auto">
          Découvrez nos initiatives pour l'Afrique
        </p>
      </div>
    </div>

    <!-- Section des cartes avec animation -->
    <div class="relative -mt-24 z-40 px-4 py-16">
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
                class="absolute inset-0 bg-gradient-to-b from-black/10 to-black/80 z-20"
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
const heroContent = ref<HTMLElement | null>(null)
const cardRefs = ref<HTMLElement[]>([])

onMounted(() => {
  // Animation de la bannière au chargement
  if (heroContent.value) {
    gsap.from(heroContent.value, {
      opacity: 0,
      y: 50,
      duration: 1.2,
      ease: 'power3.out',
    })
  }

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
/* Animations qui ne peuvent pas être faites avec Tailwind */
.parallax-bg {
  background-image: url('/images/africa-banner.jpg');
  background-size: cover;
  background-position: center;
  will-change: transform;
  animation: parallax-effect 15s ease-in-out infinite alternate;
}

.text-shimmer {
  background: linear-gradient(45deg, #fff, #ffd700);
  -webkit-background-clip: text;
  background-clip: text;
  color: transparent;
  animation: text-shimmer 3s infinite;
  background-size: 200% auto;
}

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

@keyframes parallax-effect {
  0% {
    transform: scale(1.1) translateY(0);
  }
  100% {
    transform: scale(1.1) translateY(-20px);
  }
}

@keyframes text-shimmer {
  0% {
    background-position: -100% 0;
  }
  100% {
    background-position: 200% 0;
  }
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
