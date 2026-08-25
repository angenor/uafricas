<script setup lang="ts">
/**
 * Ancienne adresse du fil d'actualité, désormais servi à la racine.
 *
 * La redirection est PERMANENTE et se fait avant le rendu : `/publications` a
 * circulé en partages : chaque carte de partage du mur construisait ses liens
 * à partir de cette page : et en signets. La supprimer casserait des liens
 * déjà publiés.
 */
definePageMeta({
  middleware: () => navigateTo('/', { redirectCode: 301, replace: true }),
})
</script>

<template>
  <div />
</template>
