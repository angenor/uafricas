<template>
  <!-- Bandeau d'un élément de fiche pays. Même gabarit que
       `AfricansBandeauModule` — pleine largeur, texte borné au conteneur —,
       avec deux différences assumées : l'image est celle de l'élément, et le
       titre est aligné à gauche parce qu'un sous-titre, des badges et une
       ligne de méta l'accompagnent. Le fil d'Ariane, lui, appartient au
       gabarit : il se lit sur fond clair, sous la barre supérieure.

       `min-h` et non `h` : la hauteur du bandeau de module est ici un plancher.
       Un nom sur deux lignes, deux badges et un sous-titre dépassent les
       315 px, et une hauteur figée les aurait rognés. -->
  <section class="relative isolate w-full overflow-hidden bg-af-bordure">
    <img
      v-if="image"
      :src="image"
      :alt="''"
      class="absolute inset-0 size-full object-cover"
      :class="positionImage"
    />
    <!-- Voile : dégradé de marque faute d'image, voile noir par-dessus l'image
         pour garantir la lisibilité du texte blanc. -->
    <div
      class="absolute inset-0"
      :class="image ? 'bg-gradient-to-t from-black/80 via-black/45 to-black/25' : 'bg-af-degrade'"
    />

    <div class="relative mx-auto flex min-h-af-bandeau max-w-af-conteneur items-end gap-6 px-6 pt-20 pb-8">
      <!-- Un PORTRAIT ne se met pas en fond : une bande de 315 px de haut sur
           toute la largeur le recadre sur les yeux. Il est donc encadré à
           gauche du titre, à taille lisible, et le fond reprend le dégradé de
           marque. C'est `image` qui reste le fond pour un lieu ou un plat, où
           le cadrage large est justement le sujet. -->
      <div
        v-if="portrait"
        class="hidden size-40 shrink-0 overflow-hidden rounded-[10px] border-4 border-white/25 bg-af-fond sm:block"
      >
        <img :src="portrait" :alt="titre" class="size-full object-cover object-top" />
      </div>

      <div class="min-w-0 flex-1">
        <div v-if="$slots.badges" class="mb-3 flex flex-wrap items-center gap-2">
          <slot name="badges" />
        </div>

        <h1 class="max-w-3xl text-[32px]/[1.2] font-bold text-white md:text-[48px]/[1.15]">
          {{ titre }}
        </h1>

        <p v-if="sousTitre" class="mt-2 flex items-center gap-2 text-base text-white/90">
          <slot name="sous-titre-icon" />
          {{ sousTitre }}
        </p>

        <slot name="meta" />
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
withDefaults(
  defineProps<{
    titre: string
    sousTitre?: string | null
    /** Image de FOND, plein cadre : un lieu, un plat, une activité. */
    image?: string | null
    /** Portrait ENCADRÉ à gauche du titre : une personne. */
    portrait?: string | null
    /** Position de l'image de fond (ex. `object-top`) */
    positionImage?: string
  }>(),
  {
    sousTitre: null,
    image: null,
    portrait: null,
    positionImage: 'object-center',
  },
)
</script>
