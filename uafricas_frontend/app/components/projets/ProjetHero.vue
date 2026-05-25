<template>
  <div class="group relative bg-gradient-to-br from-custom-green via-emerald-600 to-teal-600 overflow-hidden">
    <!-- Background pattern -->
    <div class="absolute inset-0 opacity-10">
      <div class="absolute inset-0" style="background-image: url('data:image/svg+xml,%3Csvg width=&quot;60&quot; height=&quot;60&quot; viewBox=&quot;0 0 60 60&quot; xmlns=&quot;http://www.w3.org/2000/svg&quot;%3E%3Cg fill=&quot;none&quot; fill-rule=&quot;evenodd&quot;%3E%3Cg fill=&quot;%23ffffff&quot; fill-opacity=&quot;0.4&quot;%3E%3Cpath d=&quot;M36 34v-4h-2v4h-4v2h4v4h2v-4h4v-2h-4zm0-30V0h-2v4h-4v2h4v4h2V6h4V4h-4zM6 34v-4H4v4H0v2h4v4h2v-4h4v-2H6zM6 4V0H4v4H0v2h4v4h2V6h4V4H6z&quot;/%3E%3C/g%3E%3C/g%3E%3C/svg%3E');" />
    </div>

    <div class="relative max-w-7xl mx-auto px-4 pt-16 pb-6">
      <div class="text-center text-white" data-aos="fade-up">
        <!-- Badge -->
        <div class="inline-flex items-center gap-2 bg-white/20 backdrop-blur-xs px-4 py-2 rounded-full text-sm font-medium mb-6">
          <font-awesome-icon :icon="['fas', 'seedling']" class="w-4 h-4" />
          Investissez dans l'avenir de l'Afrique
        </div>

        <!-- Titre ↔ sous-titre en crossfade au survol -->
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12 mb-8 select-none">
          <h1 class="absolute inset-0 flex items-center justify-center text-2xl md:text-4xl font-bold transition-opacity duration-300 group-hover:opacity-0">
            Projets de Développement
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-sm md:text-base text-white/90 max-w-2xl mx-auto px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Découvrez des projets innovants et contribuez au développement durable du continent africain
          </p>
        </div>

        <!-- Statistiques -->
        <div class="grid grid-cols-2 md:grid-cols-4 gap-4 max-w-4xl mx-auto" data-aos="fade-up" data-aos-delay="200">
          <div class="bg-white/10 backdrop-blur-xs rounded-2xl p-4 md:p-6">
            <div class="text-3xl md:text-4xl font-bold mb-1">{{ statistics.total }}</div>
            <div class="text-white/80 text-sm">Projets au total</div>
          </div>
          <div class="bg-white/10 backdrop-blur-xs rounded-2xl p-4 md:p-6">
            <div class="text-3xl md:text-4xl font-bold mb-1 text-emerald-300">{{ statistics.valides }}</div>
            <div class="text-white/80 text-sm">Projets validés</div>
          </div>
          <div class="bg-white/10 backdrop-blur-xs rounded-2xl p-4 md:p-6">
            <div class="text-3xl md:text-4xl font-bold mb-1 text-blue-300">{{ statistics.en_cours }}</div>
            <div class="text-white/80 text-sm">En cours</div>
          </div>
          <div class="bg-white/10 backdrop-blur-xs rounded-2xl p-4 md:p-6">
            <div class="text-3xl md:text-4xl font-bold mb-1 text-yellow-300">{{ statistics.termines }}</div>
            <div class="text-white/80 text-sm">Terminés</div>
          </div>
        </div>
      </div>
    </div>

    <!-- Wave bottom -->
    <div class="absolute bottom-0 left-0 right-0">
      <svg class="w-full h-12 md:h-16" viewBox="0 0 1440 54" fill="none" xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="none">
        <path d="M0 22L60 16.7C120 11 240 1 360 0.7C480 1 600 11 720 16.7C840 22 960 22 1080 16.7C1200 11 1320 1 1380 0.7L1440 0V54H1380C1320 54 1200 54 1080 54C960 54 840 54 720 54C600 54 480 54 360 54C240 54 120 54 60 54H0V22Z" fill="#f9fafb"/>
      </svg>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ProjetStatistiquesAPI } from '~/composables/useProjets'

defineProps<{
  statistics: ProjetStatistiquesAPI
}>()
</script>
