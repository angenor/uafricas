<template>
  <article class="flex flex-col overflow-hidden rounded-[10px] border border-af-bordure bg-white">
    <div class="aspect-[16/9] w-full overflow-hidden bg-af-bordure">
      <img v-if="image" :src="image" alt="" class="size-full object-cover" />
    </div>

    <div class="flex flex-1 flex-col gap-2 p-4">
      <h3 class="text-[14px]/[1.4] font-bold">{{ titre }}</h3>
      <p v-if="type" class="text-[12px]/[1.4] text-af-corps">Type : {{ type }}</p>

      <p v-if="lieu" class="flex items-start gap-2 text-[12px]/[1.4] font-bold text-af-corps">
        <font-awesome-icon icon="fa-solid fa-location-dot" class="mt-0.5 shrink-0 text-af-chocolat" />
        {{ lieu }}
      </p>
      <p v-if="date" class="flex items-center gap-2 text-[12px]/[1.4] font-bold text-af-corps">
        <font-awesome-icon icon="fa-solid fa-calendar-days" class="shrink-0 text-af-chocolat" />
        {{ date }}
      </p>
      <p v-if="heure" class="flex items-center gap-2 text-[12px]/[1.4] font-bold text-af-corps">
        <font-awesome-icon icon="fa-solid fa-clock" class="shrink-0 text-af-chocolat" />
        {{ heure }}
      </p>

      <!-- Lien-action en pied, pas un bouton : la maquette le pose en texte
           chocolat avec une flèche, sans cadre. -->
      <NuxtLink
        v-if="vers"
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
defineProps<{
  titre: string
  type?: string
  lieu?: string
  /** Date déjà mise en forme, ex. « Dimanche 09 Août 2026 ». */
  date?: string
  heure?: string
  image?: string | null
  vers?: string
}>()
</script>
