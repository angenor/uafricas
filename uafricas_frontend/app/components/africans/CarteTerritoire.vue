<template>
  <component
    :is="vers ? 'NuxtLink' : 'article'"
    :to="vers"
    class="block overflow-hidden rounded-[10px] border border-af-bordure bg-white transition"
    :class="vers && 'hover:border-af-chocolat'"
  >
    <div class="relative aspect-[16/10] w-full overflow-hidden bg-af-bordure">
      <img v-if="image" :src="image" alt="" class="size-full object-cover" />
      <span v-if="drapeau" class="absolute top-3 left-3 text-2xl leading-none">{{ drapeau }}</span>
      <AfricansEtiquette v-if="region" ton="vert" class="absolute top-3 right-3">
        {{ region }}
      </AfricansEtiquette>
    </div>

    <div class="flex flex-col gap-1.5 p-4">
      <h3 class="text-[14px]/[1.4] font-bold">{{ nom }}</h3>
      <p v-if="devise" class="text-[12px]/[1.4] text-af-corps italic">{{ devise }}</p>

      <p v-if="capitale" class="flex items-center gap-1.5 text-[12px]/[1.4] text-af-corps">
        <font-awesome-icon icon="fa-solid fa-location-dot" class="text-af-chocolat" />
        {{ capitale }}
      </p>
      <p v-if="population" class="flex items-center gap-1.5 text-[12px]/[1.4] text-af-corps">
        <font-awesome-icon icon="fa-solid fa-users" class="text-af-chocolat" />
        {{ population }}
      </p>

      <p class="text-[12px]/[1.4] text-af-atone">
        {{ contributions }} Contribution{{ contributions > 1 ? 's' : '' }}
      </p>
    </div>
  </component>
</template>

<script setup lang="ts">
/**
 * Carte de territoire Afripulse. `population` est reçue déjà mise en forme
 * (« 63.2 millions ») : le formatage dépend de la source de la donnée, pas de
 * l'affichage, et le faire ici obligerait à deviner l'unité.
 *
 * Le compteur de contributions s'accorde — la maquette écrit « 0 Contibution »
 * au singulier ET avec une faute ; les deux sont corrigés.
 */
withDefaults(defineProps<{
  nom: string
  region?: string
  devise?: string
  capitale?: string
  population?: string
  drapeau?: string
  image?: string | null
  contributions?: number
  vers?: string
}>(), { contributions: 0 })
</script>
