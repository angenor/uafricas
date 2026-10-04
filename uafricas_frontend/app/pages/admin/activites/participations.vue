<script setup lang="ts">
/**
 * File de modération des photos de concours, tous concours confondus
 * (feature 014). Toute photo est relue AVANT publication (H3).
 */
definePageMeta({ layout: 'admin', middleware: ['admin'] })
</script>

<template>
  <div>
    <AdminPageHeader
      titre="Participations aux concours"
      sous-titre="Chaque photo est relue avant d'être publiée ; un refus exige un motif, que le membre reçoit"
    />
    <AdminJeuModerationParticipations />
  </div>
</template>
