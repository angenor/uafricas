<script setup lang="ts">
import { Carousel, Slide, Pagination, Navigation } from 'vue3-carousel'
import 'vue3-carousel/dist/carousel.css'

defineProps<{
  images: string[]
}>()
</script>

<template>
  <Carousel
    class="rounded-xl overflow-hidden"
    :pauseAutoplayOnHover="true"
    :wrapAround="true"
    :autoplay="4000"
    :transition="500"
    :itemsToShow="1"
  >
    <Slide v-for="(image, index) in images" :key="index">
      <img
        class="bg-white rounded-xl h-80 md:h-96 w-full object-cover"
        :src="image"
        alt="Culture africaine"
      />
    </Slide>

    <template #addons>
      <Navigation />
      <Pagination />
    </template>
  </Carousel>
</template>

<style>
.carousel__prev,
.carousel__next {
  background-color: rgba(255, 255, 255, 0.8);
  border-radius: 50%;
  color: #4b5563;
}

.carousel__prev:hover,
.carousel__next:hover {
  background-color: white;
}

.carousel__pagination-button::after {
  background-color: rgba(255, 255, 255, 0.6);
}

.carousel__pagination-button--active::after {
  background-color: white;
}
</style>
