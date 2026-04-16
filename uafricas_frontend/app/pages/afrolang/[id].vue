<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50">
    <div class="text-center">
      <div class="animate-spin rounded-full h-12 w-12 border-4 border-blue-500 border-t-transparent mx-auto mb-4" />
      <p class="text-gray-500">Ouverture du livestream...</p>
    </div>
  </div>
</template>

<script setup lang="ts">
// Refonte 2026-04 (FR-003) : la fiche salle publique legacy redirige
// directement vers la session live. Plus de page intermédiaire.
const route = useRoute()

onMounted(() => {
  const id = String(route.params.id)
  navigateTo(`/afrolang/session/${id}`, { replace: true })
})
</script>
