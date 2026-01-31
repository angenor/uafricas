<template>
  <div
    id="moboliser_expertise"
    class="bg-custom-chocolat w-full z-0 relative"
    data-aos-duration="1300"
    data-aos="fade-up"
  >
    <div class="flex justify-center absolute -top-10 w-full z-50">
      <button
        class="uppercase text-xl cursor-default top-5 mx-auto relative rounded-full bg-custom-chocolat text-white font-bold px-4 py-2"
      >
        <span class="font-bold text-2xl">M</span>obiliser une expertise
      </button>
    </div>

    <div class="flex overflow-hidden relative">
      <NuxtLink
        @mouseenter="goHover = true"
        @mouseleave="goHover = false"
        class="ml-72 rounded-full text-white whitespace-nowrap my-auto shadow-md hover:shadow-none"
        to="/experts/index"
      >
        <button class="h-10 rounded-full bg-custom-green px-4 hover:scale-105 transition-all">
          Trouver des experts
          <font-awesome-icon icon="fa-solid fa-arrow-right" />
        </button>
      </NuxtLink>

      <div class="my-auto space-y-10 ml-5">
        <span
          :class="goHover ? '-left-0' : '-left-32'"
          class="flex transition-all duration-700 relative drop-shadow-lg items-center text-lg text-white font-bold ml-5"
        >
          <span class="flex w-2.5 h-2.5 bg-blue-600 rounded-full mr-1.5 flex-shrink-0"></span>
          Agriculture
        </span>
        <span
          :class="goHover ? '-left-0' : '-left-20'"
          class="flex transition-all duration-500 relative drop-shadow-lg items-center text-lg text-white font-bold ml-5"
        >
          <span class="flex w-2.5 h-2.5 bg-purple-500 rounded-full mr-1.5 flex-shrink-0"></span>
          Informatique
        </span>
        <span class="flex items-center text-lg drop-shadow-lg text-white font-bold ml-5">
          <span class="flex w-2.5 h-2.5 bg-indigo-500 rounded-full mr-1.5 flex-shrink-0"></span>
          Mécanique
        </span>
        <span
          :class="goHover ? '-left-0' : '-left-20'"
          class="flex transition-all duration-500 relative drop-shadow-lg items-center text-lg text-white font-bold ml-5"
        >
          <span class="flex w-2.5 h-2.5 bg-teal-400 rounded-full mr-1.5 flex-shrink-0"></span>
          Elévage
        </span>
        <span
          :class="goHover ? '-left-0' : '-left-32'"
          class="flex transition-all duration-700 relative drop-shadow-lg items-center text-lg text-white font-bold ml-5"
        >
          <span class="flex w-2.5 h-2.5 bg-teal-400 rounded-full mr-1.5 flex-shrink-0"></span>
          Etc...
        </span>
      </div>

      <div class="w-11/12 ml-24 text-white flex justify-around">
        <div class="w-96 h-96 p-3 font-bold"></div>
      </div>

      <div
        class="absolute -right-72 h-120 w-[62rem] bg-slate-500 -top-72 rotate-45 bg-font-danse-afrique bg-cover bg-no-repeat"
      ></div>

      <div
        class="absolute text-3xl text-center font-bold my-auto w-96 right-10 top-20 text-white drop-shadow-[0_1.9px_1.9px_rgba(0,0,0,0.9)] p-3 bg-gray-600 bg-opacity-30 border border-opacity-80 rounded-2xl"
        data-aos-duration="1300"
        data-aos="fade-up"
      >
        Vous êtes ressortissant africain ou africain d'origine ? retrouvez des
        milliers d'experts près à s'engager pour l'Afrique et pour vous.
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const goHover = ref(false)
useAOS()
</script>

<style scoped>
.outline-text {
  color: white;
  text-shadow: -1px -1px 0 #000, 1px -1px 0 #000, -1px 1px 0 #000, 1px 1px 0 #000;
}
</style>
