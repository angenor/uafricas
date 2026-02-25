<template>
  <main class="relative block">
    <HomeTheWelcome />
    <HomeHistoireSection />
    <HomeApplisSection />
    <HomeMobiliserExpert />
    <HomeDecouvrirSection />
  </main>
</template>

<script setup lang="ts">
useHead({
  title: 'Accueil - UAfricas',
  meta: [
    { name: 'description', content: 'Bienvenue sur UAfricas - United Africa for Sustainable Development' }
  ]
})
</script>
