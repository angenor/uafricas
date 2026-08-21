<template>
  <!-- Carte de centre culturel. Mesures Figma : 357 x 425, vignette 355 x 213
       (soit un ratio 5/3, et non le 16/9 des cartes de salle). -->
  <article class="flex flex-col overflow-hidden rounded-[10px] border border-af-bordure bg-white transition hover:border-af-chocolat">
    <div class="aspect-[5/3] w-full overflow-hidden bg-af-bordure">
      <img v-if="image" :src="image" alt="" class="size-full object-cover" />
      <div v-else class="grid size-full place-items-center text-af-atone-2">
        <font-awesome-icon icon="fa-solid fa-masks-theater" class="text-4xl" />
      </div>
    </div>

    <div class="flex flex-1 flex-col gap-2 p-4">
      <h3 class="text-[14px]/[1.4] font-bold">{{ nom }}</h3>

      <p v-if="description" class="line-clamp-3 text-[12px]/[1.4] text-af-corps">
        {{ description }}
      </p>

      <p v-if="lieu" class="flex items-center gap-2 text-[12px]/[1.4] text-af-corps">
        <font-awesome-icon icon="fa-solid fa-location-dot" class="shrink-0 text-af-chocolat" />
        {{ lieu }}
      </p>

      <p class="flex items-center gap-2 text-[12px]/[1.4] text-af-corps">
        <font-awesome-icon icon="fa-solid fa-fire" class="shrink-0 text-af-chocolat" />
        {{ libelleProgrammations }}
      </p>

      <NuxtLink
        :to="vers"
        class="mt-auto flex items-center gap-2 pt-2 text-[16px]/[1.4] font-bold text-af-chocolat transition hover:gap-3"
      >
        Découvrir
        <font-awesome-icon icon="fa-solid fa-arrow-right" />
      </NuxtLink>
    </div>
  </article>
</template>

<script setup lang="ts">
const props = withDefaults(defineProps<{
  nom: string
  description?: string | null
  lieu?: string | null
  image?: string | null
  programmations?: number
  vers: string
}>(), { programmations: 0 })

/**
 * La maquette écrit « 02 évènements à venir » — deux chiffres, et le pluriel
 * même à zéro. Le zéro pluriel est correct en français ; le zéro padding est
 * conservé parce qu'il aligne les compteurs d'une carte à l'autre dans la
 * grille, ce qu'un « 2 » nu ne ferait pas.
 */
const libelleProgrammations = computed(() => {
  const n = props.programmations
  const compte = String(n).padStart(2, '0')
  return n === 1
    ? `${compte} évènement à venir`
    : `${compte} évènements à venir`
})
</script>
