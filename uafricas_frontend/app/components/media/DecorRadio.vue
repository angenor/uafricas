<script setup lang="ts">
/**
 * Décor animé des pages Radio : ciel étoilé et ondes sonores.
 *
 * Extrait des deux pages, qui en portaient chacune une copie intégrale.
 * Ce n'est PAS une fusion des pages : elles restent deux fichiers distincts,
 * chacune avec sa propre origine de publication (FR-012) : seul le décor,
 * strictement identique et sans logique, est mis en commun.
 */
defineProps<{
  /** Anime les ondes quand un contenu est en cours d'écoute. */
  actif?: boolean
}>()
</script>

<template>
  <div aria-hidden="true">
    <div class="stars-container absolute inset-0 z-0">
      <div v-for="n in 20" :key="n" class="star" />
    </div>

    <div class="absolute inset-0 z-0 flex items-center justify-center overflow-hidden">
      <div
        v-for="n in 3"
        :key="n"
        :class="`audio-wave wave-${n} ${actif ? 'animate-wave' : ''}`"
      />
    </div>
  </div>
</template>

<style scoped>
@keyframes wave {
  0% { transform: scale(1); opacity: 1; }
  100% { transform: scale(1.5); opacity: 0; }
}

@keyframes twinkle {
  0%, 100% { opacity: 0.3; transform: scale(0.8); }
  50% { opacity: 1; transform: scale(1); }
}

.animate-wave {
  animation: wave 3s infinite;
}

.audio-wave {
  position: absolute;
  border: 2px solid rgba(251, 191, 36, 0.3);
  border-radius: 50%;
  width: 40vh;
  height: 40vh;
  opacity: 0;
}

.wave-1 { animation-delay: 0s; }
.wave-2 { animation-delay: 1s; }
.wave-3 { animation-delay: 2s; }

.stars-container { z-index: 0; }

.star {
  position: absolute;
  background-color: white;
  border-radius: 50%;
  width: 2px;
  height: 2px;
  animation: twinkle 3s infinite;
  top: calc(var(--star-y, 50) * 1%);
  left: calc(var(--star-x, 50) * 1%);
  animation-delay: calc(var(--star-delay, 0) * 1s);
}

.stars-container .star:nth-child(3n) { width: 3px; height: 3px; }
.stars-container .star:nth-child(7n) { background-color: #fbbf24; }

.stars-container .star:nth-child(1)  { --star-x: 5;  --star-y: 10; --star-delay: 0.1; }
.stars-container .star:nth-child(2)  { --star-x: 15; --star-y: 20; --star-delay: 0.3; }
.stars-container .star:nth-child(3)  { --star-x: 25; --star-y: 5;  --star-delay: 0.5; }
.stars-container .star:nth-child(4)  { --star-x: 35; --star-y: 30; --star-delay: 0.7; }
.stars-container .star:nth-child(5)  { --star-x: 45; --star-y: 12; --star-delay: 0.9; }
.stars-container .star:nth-child(6)  { --star-x: 55; --star-y: 22; --star-delay: 1.1; }
.stars-container .star:nth-child(7)  { --star-x: 65; --star-y: 7;  --star-delay: 1.3; }
.stars-container .star:nth-child(8)  { --star-x: 75; --star-y: 15; --star-delay: 1.5; }
.stars-container .star:nth-child(9)  { --star-x: 85; --star-y: 25; --star-delay: 1.7; }
.stars-container .star:nth-child(10) { --star-x: 95; --star-y: 18; --star-delay: 1.9; }
.stars-container .star:nth-child(11) { --star-x: 10; --star-y: 35; --star-delay: 0.2; }
.stars-container .star:nth-child(12) { --star-x: 20; --star-y: 45; --star-delay: 0.4; }
.stars-container .star:nth-child(13) { --star-x: 30; --star-y: 55; --star-delay: 0.6; }
.stars-container .star:nth-child(14) { --star-x: 40; --star-y: 40; --star-delay: 0.8; }
.stars-container .star:nth-child(15) { --star-x: 50; --star-y: 50; --star-delay: 1.0; }
.stars-container .star:nth-child(16) { --star-x: 60; --star-y: 35; --star-delay: 1.2; }
.stars-container .star:nth-child(17) { --star-x: 70; --star-y: 45; --star-delay: 1.4; }
.stars-container .star:nth-child(18) { --star-x: 80; --star-y: 55; --star-delay: 1.6; }
.stars-container .star:nth-child(19) { --star-x: 90; --star-y: 40; --star-delay: 1.8; }
.stars-container .star:nth-child(20) { --star-x: 8;  --star-y: 60; --star-delay: 2.0; }
</style>
