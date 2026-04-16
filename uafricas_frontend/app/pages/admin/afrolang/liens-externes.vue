<script setup lang="ts">
definePageMeta({
  layout: 'admin',
  middleware: ['admin'],
})

useHead({
  title: 'Afrolang — Liens externes | Admin',
})
</script>

<template>
  <div>
    <AdminPageHeader
      titre="Afrolang — Liens externes"
      description="Validation des liens externes soumis par les membres pour enrichir les ressources des salles publiques."
      icone="link"
    />

    <div class="card bg-base-100 shadow mt-4">
      <div class="card-body">
        <AdminAfrolangLiensExternesValidation />
      </div>
    </div>
  </div>
</template>
