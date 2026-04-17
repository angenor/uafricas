<template>
  <section class="bg-font1 bg-cover bg-center bg-no-repeat w-full h-svh relative flex items-center justify-center md:justify-end">
    <div
      class="text-center text-custom-chocolat px-6 md:pr-16 lg:pr-24 max-w-xl"
    >
      <NuxtLink
        to="/publications"
        data-aos="fade-down"
        data-aos-duration="1000"
        class="inline-flex items-center gap-2 mb-4 px-5 py-2.5 bg-custom-green text-white font-semibold rounded-full shadow-lg hover:bg-custom-green/90 hover:shadow-xl hover:scale-105 transition-all duration-300"
      >
        <span>Publications de la Communauté</span>
        <font-awesome-icon :icon="['fas', 'arrow-right']" />
      </NuxtLink>
      <p data-aos="fade-left" data-aos-duration="1200" class="text-2xl sm:text-3xl lg:text-4xl font-display">
        Bienvenue sur le site officiel de
      </p>
      <h1 data-aos="zoom-in" data-aos-delay="1300" class="text-5xl sm:text-6xl lg:text-8xl font-extrabold font-display">
        AfricanS
      </h1>
      <p data-aos="zoom-in-up" data-aos-delay="1400" class="font-semibold text-base sm:text-lg font-body">
        Afrique, une Nation pour le développement durable
      </p>
    </div>

    <a
      href="#histoire"
      aria-label="Défiler vers le bas"
      class="absolute bottom-8 left-1/2 -translate-x-1/2 bg-white/70 rounded-full animate-bounce p-2 border border-gray-400"
    >
      <font-awesome-icon icon="fa-solid fa-arrow-down" class="w-8 h-8 sm:w-10 sm:h-10 text-custom-green" />
    </a>
  </section>
</template>

<script setup lang="ts">
useAOS()
</script>
