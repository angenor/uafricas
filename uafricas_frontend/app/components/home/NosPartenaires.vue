<template>
  <div
    class="bg-custom-gray w-full py-10"
    data-aos="fade-down"
    data-aos-duration="500"
  >
    <div class="flex">
      <div class="text-custom-chocolat text-4xl mx-auto">
        <span class="font-bold">N</span>os Partenaires
      </div>
    </div>
    <div class="flex justify-center space-x-20 mt-10">
      <img
        v-for="partenaire in partenaires"
        :key="partenaire.id"
        class="h-20"
        :src="partenaire.logo"
        :alt="partenaire.nom"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { partenaires } from '~/mocks/homepage'

useAOS()
</script>
