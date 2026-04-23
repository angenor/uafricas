<template>
  <div
    class="relative bg-white rounded-2xl overflow-hidden shadow-lg hover:shadow-2xl transition-all duration-300 group"
    @mouseenter="cardHover = true"
    @mouseleave="cardHover = false"
  >
    <!-- Image avec overlay moderne -->
    <div class="relative h-64 overflow-hidden">
      <img
        class="w-full h-full object-cover transition-transform duration-500 group-hover:scale-110"
        :src="expert.photoURL || '/images/default-avatar.jpg'"
        :alt="`Photo de ${expert.prenom} ${expert.nom}`"
      />

      <!-- Gradient overlay moderne -->
      <div class="absolute inset-0 bg-gradient-to-t from-black/70 via-black/20 to-transparent" />

      <!-- Badge de statut -->
      <div class="absolute top-4 right-4">
        <div
          class="bg-white/90 backdrop-blur-xs px-3 py-1 rounded-full text-xs font-medium text-emerald-600 flex items-center gap-1"
        >
          <div class="w-2 h-2 bg-emerald-500 rounded-full animate-pulse" />
          Disponible
        </div>
      </div>

      <!-- Informations principales -->
      <div class="absolute bottom-0 left-0 right-0 p-6 text-white">
        <h3 class="text-xl font-bold mb-1">{{ expert.prenom }} {{ expert.nom }}</h3>
        <div class="flex items-center gap-4 text-sm text-white/80">
          <span class="flex items-center gap-1">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"
              />
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"
              />
            </svg>
            {{ countryLabel }}
          </span>
          <span class="flex items-center gap-1">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M21 13.255A23.931 23.931 0 0112 15c-3.183 0-6.22-.62-9-1.745M16 6V4a2 2 0 00-2-2h-4a2 2 0 00-2 2v2m4 6h.01M5 20h14a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"
              />
            </svg>
            {{ expertDomain }}
          </span>
        </div>
      </div>
    </div>

    <!-- Contenu de la carte -->
    <div class="p-6">
      <!-- Tags de competences -->
      <div class="flex flex-wrap gap-2 mb-4">
        <span class="px-3 py-1 bg-emerald-50 text-emerald-700 rounded-full text-xs font-medium">
          {{ expertExperience }} ans d'exp.
        </span>
        <span class="px-3 py-1 bg-blue-50 text-blue-700 rounded-full text-xs font-medium">
          {{ expertRating }}
        </span>
      </div>

      <!-- Biographie courte -->
      <p class="text-gray-600 text-sm line-clamp-3 mb-4">
        {{ expertBiography }}
      </p>

      <!-- Actions -->
      <div class="flex gap-3">
        <NuxtLink
          :to="`/profil/${expert.id}`"
          class="flex-1 bg-gradient-to-r from-emerald-500 to-teal-500 text-white text-center py-3 rounded-xl font-medium hover:shadow-lg transform hover:scale-[1.02] transition-all"
        >
          Voir le profil
        </NuxtLink>
        <button
          class="p-3 bg-gray-100 rounded-xl hover:bg-gray-200 transition-all group/btn"
          title="Contacter"
          @click="$emit('contact', expert)"
        >
          <svg
            class="w-5 h-5 text-gray-600 group-hover/btn:text-emerald-600 transition-colors"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"
            />
          </svg>
        </button>
      </div>
    </div>

    <!-- Hover card moderne -->
    <Transition
      enter-active-class="transition-all duration-300 ease-out"
      enter-from-class="opacity-0 translate-y-4"
      enter-to-class="opacity-100 translate-y-0"
      leave-active-class="transition-all duration-200 ease-in"
      leave-from-class="opacity-100 translate-y-0"
      leave-to-class="opacity-0 translate-y-4"
    >
      <div
        v-if="cardHover"
        class="absolute inset-0 bg-gradient-to-br from-emerald-600/95 to-teal-600/95 backdrop-blur-xs p-6 flex flex-col justify-center items-center text-white"
      >
        <div class="text-center space-y-4">
          <div
            class="w-20 h-20 bg-white/20 rounded-full flex items-center justify-center mx-auto backdrop-blur-xs"
          >
            <svg class="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
              />
            </svg>
          </div>
          <h4 class="text-xl font-bold">{{ expert.prenom }} {{ expert.nom }}</h4>
          <p class="text-white/90 text-sm">{{ expertDomain }}</p>
          <div class="flex gap-2 justify-center">
            <NuxtLink
              :to="`/profil/${expert.id}`"
              class="px-6 py-2 bg-white text-emerald-600 rounded-full font-medium hover:bg-white/90 transition-all"
            >
              Voir le profil
            </NuxtLink>
          </div>
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import type { ExpertAPI } from '~/composables/useExperts'

const props = defineProps<{
  expert: ExpertAPI
}>()

defineEmits<{
  contact: [expert: ExpertAPI]
}>()

const cardHover = ref(false)

// Computed pour recuperer le domaine
const expertDomain = computed(() => {
  return props.expert.expertiseInfo?.domaine || 'Expert'
})

// Computed pour recuperer la biographie
const expertBiography = computed(() => {
  return (
    props.expert.expertiseInfo?.biographie ||
    'Expert passionne avec une grande experience dans son domaine.'
  )
})

// Computed pour recuperer l'experience
const expertExperience = computed(() => {
  return props.expert.expertiseInfo?.nbAnneesExperience || 0
})

// Computed pour recuperer la note
const expertRating = computed(() => {
  const rating = props.expert.expertiseInfo?.rating || 4.5
  return `⭐ ${rating.toFixed(1)}`
})

// Computed pour le label du pays (le backend retourne directement le nom)
const countryLabel = computed(() => {
  return props.expert.pays || 'Non specifie'
})
</script>

<style scoped>
.line-clamp-3 {
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

@keyframes pulse {
  0%,
  100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}

.animate-pulse {
  animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}
</style>
