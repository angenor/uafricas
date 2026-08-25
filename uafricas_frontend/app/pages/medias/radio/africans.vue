<script setup lang="ts">
/**
 * Radios Africans : production propre de la plateforme (US2).
 *
 * Page DISTINCTE de `/medias/radio/nationales` : l'adresse est inchangée
 * (FR-012), aucune redirection, aucune fusion. Seule l'origine `africans`,
 * fixée ici, garantit qu'aucune station ne figure sur les deux pages (FR-014).
 *
 * Le corps est partagé : les deux fichiers ne différaient que par cette
 * origine, une teinte et quatre phrases.
 *
 * L'écoute est confiée à la barre de lecture du gabarit, qui survit au
 * défilement et à la navigation (FR-017) : cette page ne monte aucun lecteur.
 */
definePageMeta({ layout: false })

useHead({
  title: 'Radios Africans | AfricanS',
  meta: [
    {
      name: 'description',
      content: 'Écoutez Radio Africans : les stations produites par la plateforme, station par station, avec leurs émissions.',
    }],
})
</script>

<template>
  <MediaVitrineStationsRadio
    origine="africans"
    titre="Radios Africans"
    accroche="Les stations produites par Africans, station par station, avec leurs émissions."
    image="/images/banners/radio-africans.avif"
    segment="Africans"
    vide-titre="Aucune station Africans n’est encore publiée"
    vide-texte="Les stations produites par Africans apparaîtront ici."
  />
</template>
