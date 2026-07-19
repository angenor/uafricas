<script setup lang="ts">
/**
 * Page dédiée aux supports que je détiens — US5, US6.
 *
 * Le contenu vit dans `MediaMesSupports`, également monté en onglet de
 * `/mon-compte/profil` : cette URL reste le point d'arrivée des liens envoyés
 * par courriel et des invitations, sans dupliquer l'écran.
 */
definePageMeta({ middleware: 'auth' })

useHead({ title: 'Mes supports médias — UAfricas' })

const breadcrumbs = [
  { label: 'Mon compte', to: '/mon-compte/profil' },
  { label: 'Mes supports médias', to: undefined },
]
</script>

<template>
  <div class="min-h-screen bg-linear-to-br from-gray-50 via-white to-gray-50 pt-28 pb-16">
    <div class="max-w-4xl mx-auto px-4">
      <nav aria-label="Fil d'Ariane" class="mb-6 text-sm text-gray-500">
        <template v-for="(fil, i) in breadcrumbs" :key="i">
          <NuxtLink v-if="fil.to" :to="fil.to" class="hover:text-custom-chocolat">{{ fil.label }}</NuxtLink>
          <span v-else class="text-gray-900">{{ fil.label }}</span>
          <span v-if="i < breadcrumbs.length - 1" class="mx-2">/</span>
        </template>
      </nav>

      <header class="mb-8">
        <h1 class="font-oswald text-3xl font-bold text-gray-900 mb-2">Mes supports médias</h1>
        <p class="text-gray-500">
          Les chaînes et stations que vous détenez : leur grille de programmation,
          les demandes reçues et les personnes qui vous accompagnent.
        </p>
      </header>

      <MediaMesSupports />
    </div>
  </div>
</template>
