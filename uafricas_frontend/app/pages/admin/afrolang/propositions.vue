<script setup lang="ts">
definePageMeta({
  layout: 'admin',
  middleware: ['admin'],
})

useHead({
  title: 'Afrolang — Propositions | Admin',
})
</script>

<template>
  <div>
    <AdminPageHeader
      titre="Afrolang — Propositions de salles"
      description="File de validation des nouvelles salles publiques proposées par les membres."
      icone="circle-check"
    />

    <div class="card bg-base-100 shadow mt-4">
      <div class="card-body">
        <AdminAfrolangValidationPropositionsList />
      </div>
    </div>
  </div>
</template>
