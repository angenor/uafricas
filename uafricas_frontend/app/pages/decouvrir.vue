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
/**
 * Page de présentation de la plateforme, le calque « /site-web » du Figma
 * (cadre « Accueil »).
 *
 * Elle N'EST PLUS la racine : `/` sert désormais le fil d'actualité, la
 * plateforme démarrant sur le fil. Cette page garde donc son rôle : dire ce
 * qu'est AfricanS à qui ne le sait pas, mais à une adresse propre, atteignable
 * depuis « Découvrir » dans la navigation et depuis le pied de page.
 */
useHead({
  title: 'Découvrir AfricanS',
  meta: [
    { name: 'description', content: "AfricanS, Africains, afrodescendants, Nation et développement soutenable. Découvrez l'écosystème d'applications panafricain." }
  ]
})
</script>
