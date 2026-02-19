<template>
  <section
    class="bg-custom-gray w-full py-10 md:py-14"
    data-aos="fade-down"
    data-aos-duration="500"
  >
    <h2 class="text-custom-chocolat text-3xl sm:text-4xl text-center">
      <span class="font-bold">N</span>os Partenaires
    </h2>
    <div class="flex flex-wrap justify-center items-center gap-8 sm:gap-12 lg:gap-20 mt-8 px-6">
      <img
        v-for="partenaire in partenaires"
        :key="partenaire.id"
        class="h-14 sm:h-16 lg:h-20 object-contain max-w-30 sm:max-w-37.5"
        :src="partenaire.logo"
        :alt="partenaire.nom"
        loading="lazy"
      />
    </div>
  </section>
</template>

<script setup lang="ts">
import { partenaires } from '~/mocks/homepage'

useAOS()
</script>
