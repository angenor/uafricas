<template>
  <div class="bg-font1 bg-cover bg-no-repeat w-full h-screen z-0 relative">
    <div class="welcome-text w-[34rem] right-0 text-center text-custom-chocolat">
      <div data-aos="fade-left" data-aos-duration="1200" class="text-4xl">
        Bienvenue sur le site officiel de
      </div>
      <div data-aos="zoom-in" data-aos-delay="1300" class="text-8xl font-extrabold">
        AfricanS
      </div>
      <div data-aos="zoom-in-up" data-aos-delay="1400" class="font-semibold text-lg">
        United Africa for Sustainable développment
      </div>
    </div>

    <a
      href="#qui"
      class="absolute bottom-10 right-1/2 bg-white/70 rounded-full animate-bounce p-2 border border-gray-500 cursor-pointer"
    >
      <font-awesome-icon
        icon="fa-solid fa-arrow-down"
        class="w-10 h-10 text-custom-green"
      />
    </a>
  </div>
</template>

<script setup lang="ts">
useAOS()
</script>

<style scoped>
.welcome-text {
  position: absolute;
  top: 247px;
  font-family: "Roboto";
  font-style: normal;
}
</style>
