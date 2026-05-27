<script setup lang="ts">
// Le detail d'un expert est desormais servi par la page de profil unifiee /profil/[id]
// (qui agrege profil membre + bibliotheque humaine + expertise).
const route = useRoute()
await navigateTo(`/profil/${route.params.id}`, { redirectCode: 301, replace: true })
</script>

<template>
  <div />
</template>
