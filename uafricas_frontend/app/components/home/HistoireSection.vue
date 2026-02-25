<template>
  <section id="histoire" class="py-16 md:py-24 lg:py-32 w-full bg-red-50 relative overflow-hidden">
    <div class="container mx-auto px-6 md:px-12 lg:px-20 flex flex-col lg:flex-row items-center gap-8 lg:gap-12">
      <!-- Contenu texte -->
      <div data-aos="fade-right" data-aos-duration="1000" class="text-center w-full lg:w-8/12">
        <h2 class="text-custom-green text-3xl sm:text-4xl lg:text-5xl">
          <span class="font-bold">C</span>onnaissez-vous l'histoire de l'Afrique ?
        </h2>
        <p class="mt-3 px-2 sm:px-5 text-lg sm:text-xl lg:text-2xl font-bold text-custom-chocolat">
          Découvrez toute la richesse culturelle de l'Afrique, les pays,
          cultures, les ethnies, les contes et histoires ainsi que le lien entre
          chaque culture
        </p>

        <div class="flex flex-wrap justify-center gap-3 sm:gap-5 my-4">
          <span
            v-for="tag in tags"
            :key="tag.label"
            :class="goHover ? tag.hoverClass : ''"
            class="flex relative transition-all duration-700 items-center text-sm text-black font-bold"
          >
            <span class="flex w-2.5 h-2.5 rounded-full mr-1.5 shrink-0" :class="tag.color"></span>
            {{ tag.label }}
          </span>
        </div>

        <NuxtLink
          to="/africa-culture"
          @mouseenter="goHover = true"
          @mouseleave="goHover = false"
        >
          <button class="h-10 rounded-full bg-custom-green text-white px-6 hover:scale-105 transition-transform shadow-md hover:shadow-none">
            Découvrir
            <font-awesome-icon icon="fa-solid fa-arrow-right" class="ml-1" />
          </button>
        </NuxtLink>
      </div>

      <!-- Illustration -->
      <div data-aos="fade-left" data-aos-duration="1000" class="w-full lg:w-4/12 flex justify-center lg:justify-end">
        <img
          class="h-48 sm:h-64 lg:h-96 object-contain"
          src="/images/flutte_inde.png"
          alt="Illustration flûte africaine"
          loading="lazy"
        />
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
const goHover = ref(false)

const tags = [
  { label: 'Histoires', color: 'bg-blue-600', hoverClass: '-translate-x-4' },
  { label: 'Ethnies', color: 'bg-purple-500', hoverClass: '-translate-x-2' },
  { label: 'Coutumes', color: 'bg-indigo-500', hoverClass: 'translate-x-2' },
  { label: 'Alliances', color: 'bg-teal-400', hoverClass: 'translate-x-4' },
]

useAOS()
</script>
