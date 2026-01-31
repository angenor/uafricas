<template>
  <div id="qui" class="py-32 w-[80] flex space-x-10 px-32 bg-red-50 relative">
    <div data-aos="fade-right" data-aos-duration="1000" class="text-center w-8/12">
      <div class="text-custom-green text-5xl">
        <span class="font-bold">C</span>onnaissez-vous l'histoire de l'afrique ?
      </div>
      <div class="mt-2 px-5 text-2xl font-bold text-custom-chocolat">
        Découvrez toutes la richesse culturelle de l'Afrique, les pays,
        cultures, les ethnies, les contes et histoire ainsi que le lien entre
        chaque culture
      </div>

      <div class="inline-flex mx-auto my-3">
        <span
          :class="goHover ? '-left-16' : '-left-0'"
          class="flex relative transition-all duration-700 items-center text-sm text-black font-bold"
        >
          <span class="flex w-2.5 h-2.5 bg-blue-600 rounded-full mr-1.5 flex-shrink-0"></span>
          Histoires
        </span>
        <span
          :class="goHover ? '-left-10' : '-left-0'"
          class="flex relative transition-all duration-500 items-center text-sm text-black font-bold ml-5"
        >
          <span class="flex w-2.5 h-2.5 bg-purple-500 rounded-full mr-1.5 flex-shrink-0"></span>
          Ethnies
        </span>
        <span
          :class="goHover ? '-right-10' : '-right-0'"
          class="flex relative transition-all duration-500 items-center text-sm text-black font-bold ml-5"
        >
          <span class="flex w-2.5 h-2.5 bg-indigo-500 rounded-full mr-1.5 flex-shrink-0"></span>
          Coutumes
        </span>
        <span
          :class="goHover ? '-right-16' : '-right-0'"
          class="flex relative transition-all duration-700 items-center text-sm text-black font-bold ml-5"
        >
          <span class="flex w-2.5 h-2.5 bg-teal-400 rounded-full mr-1.5 flex-shrink-0"></span>
          Alliances
        </span>
      </div>

      <br />
      <NuxtLink
        @mouseenter="goHover = true"
        @mouseleave="goHover = false"
        class="rounded-full text-white whitespace-nowrap mx-auto shadow-md hover:shadow-none"
        to="/africa-culture"
      >
        <button class="h-10 rounded-full bg-custom-green px-4 hover:scale-105 transition-all">
          Découvrir
          <font-awesome-icon icon="fa-solid fa-arrow-right" />
        </button>
      </NuxtLink>
    </div>

    <div class="w-52 text-center opacity-5">
      <img class="h-52 w-52 object-cover rounded-full" src="" alt="" />
    </div>

    <div class="absolute right-0" data-aos="fade-left" data-aos-duration="1000">
      <img class="h-96" src="/images/flutte_inde.png" alt="Illustration" />
    </div>
  </div>
</template>

<script setup lang="ts">
const goHover = ref(false)
useAOS()
</script>
