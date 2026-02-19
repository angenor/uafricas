<template>
  <section
    class="bg-custom-chocolat w-full relative pt-10 pb-12 md:pb-16"
    data-aos="fade-up"
    data-aos-duration="1300"
  >
    <!-- Badge titre flottant -->
    <div class="flex justify-center absolute -top-5 w-full z-10">
      <span class="uppercase text-lg sm:text-xl rounded-full bg-custom-chocolat text-white font-bold px-5 py-2 shadow-lg">
        <span class="font-bold text-xl sm:text-2xl">M</span>obiliser une expertise
      </span>
    </div>

    <div class="container mx-auto px-6 md:px-12 lg:px-20">
      <div class="flex flex-col md:flex-row items-center gap-8 md:gap-12 mt-6">

        <!-- Colonne gauche : bouton + tags -->
        <div class="flex flex-col items-center md:items-start gap-6 w-full md:w-1/3">
          <NuxtLink
            to="/experts"
            @mouseenter="goHover = true"
            @mouseleave="goHover = false"
          >
            <button class="h-10 rounded-full bg-custom-green text-white px-6 hover:scale-105 transition-transform shadow-md hover:shadow-none">
              Trouver des experts
              <font-awesome-icon icon="fa-solid fa-arrow-right" class="ml-1" />
            </button>
          </NuxtLink>

          <div class="space-y-3">
            <span
              v-for="tag in tags"
              :key="tag.label"
              :class="goHover ? 'translate-x-0 opacity-100' : tag.hiddenClass"
              class="flex transition-all duration-500 items-center text-lg text-white font-bold"
            >
              <span class="flex w-2.5 h-2.5 rounded-full mr-2 shrink-0" :class="tag.color"></span>
              {{ tag.label }}
            </span>
          </div>
        </div>

        <!-- Colonne droite : message + image -->
        <div class="w-full md:w-2/3 relative min-h-70 sm:min-h-80 rounded-2xl overflow-hidden">
          <div class="absolute inset-0 bg-font-danse-afrique bg-cover bg-center brightness-50 rounded-2xl"></div>
          <div
            class="relative z-10 flex items-center justify-center h-full min-h-70 sm:min-h-80 p-6 sm:p-10"
            data-aos="fade-up"
            data-aos-duration="1300"
          >
            <p class="text-xl sm:text-2xl lg:text-3xl text-center font-bold text-white drop-shadow-lg max-w-lg">
              Vous êtes ressortissant africain ou africain d'origine ? Retrouvez des
              milliers d'experts prêts à s'engager pour l'Afrique et pour vous.
            </p>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
const goHover = ref(false)

const tags = [
  { label: 'Agriculture', color: 'bg-blue-600', hiddenClass: '-translate-x-8 opacity-60' },
  { label: 'Informatique', color: 'bg-purple-500', hiddenClass: '-translate-x-5 opacity-60' },
  { label: 'Mécanique', color: 'bg-indigo-500', hiddenClass: 'translate-x-0 opacity-100' },
  { label: 'Élevage', color: 'bg-teal-400', hiddenClass: '-translate-x-5 opacity-60' },
  { label: 'Et plus...', color: 'bg-amber-400', hiddenClass: '-translate-x-8 opacity-60' },
]

useAOS()
</script>
