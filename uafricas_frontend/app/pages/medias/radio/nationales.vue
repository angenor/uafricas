<script setup lang="ts">
/**
 * Radios Nationales : stations rattachées à un territoire africain (US2).
 *
 * Page DISTINCTE de `/medias/radio/africans` : l'adresse est inchangée
 * (FR-012), aucune redirection, aucune fusion. L'origine `territoire` est fixée
 * ici.
 *
 * Auparavant, les deux pages ne différaient que par un pré-filtre sur le TYPE
 * de station : une même station pouvait donc figurer sur les deux. L'origine de
 * publication est ce qui les sépare réellement (FR-014).
 */
definePageMeta({ layout: false })

useHead({
  title: 'Radios Nationales | AfricanS',
  meta: [
    {
      name: 'description',
      content: 'Écoutez les radios nationales africaines de chaque territoire, station par station, avec leurs émissions.',
    }],
})
</script>

<template>
  <MediaVitrineStationsRadio
    origine="territoire"
    titre="Radios Nationales"
    accroche="Les stations des territoires africains, station par station, avec leurs émissions."
    image="/images/banners/radio-national.jpg"
    segment="Nationales"
    vide-titre="Aucune station nationale n’est encore publiée"
    vide-texte="Les stations des territoires apparaîtront ici."
  />
</template>
