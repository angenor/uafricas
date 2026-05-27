<template>
  <div
    class="pointer-events-none fixed inset-0 z-9999 overflow-hidden"
    aria-hidden="true"
  >
    <div
      v-for="r in reactions"
      :key="r.id"
      class="reaction-float absolute bottom-20 select-none"
      :style="{
        left: r.leftPct + '%',
        animationDuration: r.durationMs + 'ms',
        '--sway': r.swayPx + 'px',
      }"
    >
      <div class="flex flex-col items-center gap-1">
        <span class="text-5xl drop-shadow-lg sm:text-6xl">{{ r.emoji }}</span>
        <span
          v-if="r.identite"
          class="rounded-full bg-black/60 px-2 py-0.5 text-xs font-medium text-white backdrop-blur-sm"
        >
          {{ r.identite }}
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface ActiveReaction {
  id: number
  emoji: string
  identite: string | null
  leftPct: number
  durationMs: number
  swayPx: number
}

const reactions = ref<ActiveReaction[]>([])
let compteur = 0

const jouer = (emoji: string, identite: string | null = null): void => {
  const id = ++compteur
  const r: ActiveReaction = {
    id,
    emoji,
    identite,
    leftPct: 10 + Math.random() * 80,
    durationMs: 3800 + Math.random() * 1400,
    swayPx: (Math.random() < 0.5 ? -1 : 1) * (40 + Math.random() * 60),
  }
  reactions.value = [...reactions.value, r]
  setTimeout(() => {
    reactions.value = reactions.value.filter(x => x.id !== id)
  }, r.durationMs + 200)
}

defineExpose({ jouer })
</script>

<style scoped>
.reaction-float {
  animation-name: floatUp;
  animation-timing-function: ease-out;
  animation-fill-mode: forwards;
  will-change: transform, opacity;
}

@keyframes floatUp {
  0% {
    transform: translate(0, 0) scale(0.6);
    opacity: 0;
  }
  10% {
    transform: translate(0, -10vh) scale(1.1);
    opacity: 1;
  }
  50% {
    transform: translate(calc(var(--sway, 0px) * 0.5), -45vh) scale(1);
    opacity: 1;
  }
  90% {
    transform: translate(var(--sway, 0px), -85vh) scale(0.95);
    opacity: 0.85;
  }
  100% {
    transform: translate(var(--sway, 0px), -100vh) scale(0.8);
    opacity: 0;
  }
}

@media (prefers-reduced-motion: reduce) {
  .reaction-float {
    animation-duration: 1500ms !important;
  }
}
</style>
