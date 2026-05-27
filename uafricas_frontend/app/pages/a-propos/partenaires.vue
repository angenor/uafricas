<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section (compact, titre ↔ description au survol) -->
    <div class="group relative bg-gradient-to-br from-custom-chocolat via-amber-700 to-orange-700 overflow-hidden">
      <div class="absolute inset-0 opacity-10">
        <div class="absolute inset-0" style="background-image: url('data:image/svg+xml,%3Csvg width=&quot;60&quot; height=&quot;60&quot; viewBox=&quot;0 0 60 60&quot; xmlns=&quot;http://www.w3.org/2000/svg&quot;%3E%3Cg fill=&quot;none&quot; fill-rule=&quot;evenodd&quot;%3E%3Cg fill=&quot;%23ffffff&quot; fill-opacity=&quot;0.4&quot;%3E%3Cpath d=&quot;M36 34v-4h-2v4h-4v2h4v4h2v-4h4v-2h-4zm0-30V0h-2v4h-4v2h4v4h2V6h4V4h-4zM6 34v-4H4v4H0v2h4v4h2v-4h4v-2H6zM6 4V0H4v4H0v2h4v4h2V6h4V4H6z&quot;/%3E%3C/g%3E%3C/g%3E%3C/svg%3E');" />
      </div>

      <div class="relative max-w-4xl mx-auto px-4 pt-16 pb-8 text-center text-white select-none">
        <div class="inline-flex items-center gap-2 bg-white/20 backdrop-blur-xs px-4 py-1.5 rounded-full text-sm font-medium mb-4">
          <font-awesome-icon :icon="['fas', 'handshake']" class="w-4 h-4" />
          Ensemble pour l'Afrique
        </div>
        <!-- Conteneur fixe : le titre et la description se superposent (crossfade au survol) -->
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12">
          <h1 class="absolute inset-0 flex items-center justify-center text-2xl md:text-4xl font-bold font-display transition-opacity duration-300 group-hover:opacity-0">
            Nos Partenaires
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Découvrez les organisations qui soutiennent notre mission panafricaine de développement durable.
          </p>
        </div>
      </div>

      <div class="absolute bottom-0 left-0 right-0">
        <svg class="w-full h-12 md:h-16" viewBox="0 0 1440 54" fill="none" xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="none">
          <path d="M0 22L60 16.7C120 11 240 1 360 0.7C480 1 600 11 720 16.7C840 22 960 22 1080 16.7C1200 11 1320 1 1380 0.7L1440 0V54H1380C1320 54 1200 54 1080 54C960 54 840 54 720 54C600 54 480 54 360 54C240 54 120 54 60 54H0V22Z" fill="#f9fafb"/>
        </svg>
      </div>
    </div>

    <!-- Breadcrumb -->
    <div class="bg-gray-50">
      <div class="max-w-7xl mx-auto px-4 py-4">
        <CommonBreadcrumbNav />
      </div>
    </div>

    <!-- Introduction -->
    <section class="max-w-7xl mx-auto px-4 md:px-8 py-12">
      <div class="text-center max-w-3xl mx-auto mb-12">
        <h2 class="text-custom-chocolat text-2xl sm:text-3xl font-display mb-4">
          <span class="font-bold">U</span>n réseau de partenaires engagés
        </h2>
        <p class="text-gray-600 leading-relaxed">
          UAfricas collabore avec des organisations internationales, des entreprises et des institutions
          partageant notre vision d'une Afrique unie et prospère. Nos partenaires contribuent activement
          au développement culturel, économique et scientifique du continent.
        </p>
      </div>

      <!-- Logos partenaires -->
      <div class="bg-white rounded-2xl shadow-sm border border-gray-100 p-8 md:p-12">
        <div class="flex flex-wrap justify-center items-center gap-10 sm:gap-14 lg:gap-20">
          <img
            v-for="partenaire in partenaires"
            :key="partenaire.id"
            class="h-14 sm:h-16 lg:h-20 object-contain max-w-30 sm:max-w-37.5 grayscale hover:grayscale-0 transition-all duration-300"
            :src="partenaire.logo"
            :alt="partenaire.nom"
            loading="lazy"
          />
        </div>
      </div>

      <!-- CTA -->
      <div class="text-center mt-12">
        <p class="text-gray-600 mb-4">Vous souhaitez rejoindre notre réseau de partenaires ?</p>
        <NuxtLink
          to="/devenir-partenaire"
          class="inline-flex items-center gap-2 px-6 py-3 bg-custom-green text-white rounded-full hover:bg-custom-green/90 transition-colors shadow-md"
        >
          <font-awesome-icon :icon="['fas', 'handshake']" />
          Devenir partenaire
        </NuxtLink>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { partenaires } from '~/mocks/homepage'

useHead({
  title: 'Nos Partenaires - UAfricas',
  meta: [
    { name: 'description', content: 'Découvrez les partenaires de UAfricas qui soutiennent le développement durable en Afrique.' }
  ]
})
</script>
