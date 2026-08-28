<template>
  <!-- Un LIEN, et non un `<div @click>` : la carte mène à une page. En div,
       elle ne s'ouvrait pas dans un nouvel onglet, n'était pas atteignable au
       clavier et restait invisible aux moteurs comme aux lecteurs d'écran.
       C'est ce que fait déjà MarcheAnnonceCard.

       `flex flex-col` sans hauteur figée : la grille étire déjà les cartes à
       la hauteur de leur rangée. `h-80` l'empêchait, et laissait un grand vide
       entre le titre et la date dès que le titre tenait sur une seule ligne. -->
  <NuxtLink
    :to="`/echanges-sabbatiques/${programme.id}`"
    class="group flex flex-col overflow-hidden rounded-[10px] border border-af-bordure bg-white transition hover:-translate-y-1 hover:border-af-chocolat"
  >
    <div class="relative aspect-[16/10] shrink-0 overflow-hidden">
      <img
        v-if="couverture"
        :src="couverture"
        :alt="''"
        class="size-full object-cover transition-transform duration-500 group-hover:scale-110"
      />
      <!-- Repli en BALISAGE plutôt qu'en fichier. `/images/carte-afrique.jpg`
           existe bien, mais une carte de l'Afrique en couverture d'un
           programme donne à croire à une illustration choisie : elle ne dit
           pas qu'il manque une image. Le symbole, lui, le dit. -->
      <div v-else class="grid size-full place-items-center bg-af-fond">
        <font-awesome-icon icon="fa-solid fa-plane" class="text-4xl text-af-atone-2" />
      </div>

      <span
        class="absolute top-3 right-3 rounded-full px-3 py-1.5 text-xs font-bold text-white"
        :class="programme.interafricain ? 'bg-af-vert' : 'bg-af-chocolat'"
      >
        {{ programme.interafricain ? 'Interafricain' : 'Hors Afrique' }}
      </span>
    </div>

    <div class="flex flex-1 flex-col p-4">
      <p class="flex items-center gap-1.5 text-[14px]/[1.4] text-af-atone">
        <font-awesome-icon icon="fa-solid fa-location-dot" class="shrink-0 text-af-vert" />
        <span>{{ programme.pays }}</span>
        <template v-if="programme.ville">
          <span class="text-af-bordure">•</span>
          <span>{{ programme.ville }}</span>
        </template>
      </p>

      <h3 class="mt-2 line-clamp-2 text-[16px]/[1.4] font-bold text-af-encre transition-colors group-hover:text-af-chocolat">
        {{ programme.titre }}
      </h3>

      <!-- Le domaine décide autant que le lieu : c'est le premier critère de
           tri d'un candidat, et la vignette ne le montrait nulle part. -->
      <p v-if="programme.domaine" class="mt-2 w-fit rounded bg-af-fond px-2 py-1 text-xs text-af-atone">
        {{ programme.domaine }}
      </p>

      <!-- `mt-auto` : la date reste au bas de la carte quelle que soit la
           hauteur du titre, donc alignée d'une carte à l'autre. -->
      <p class="mt-auto flex flex-wrap items-center gap-2 border-t border-af-bordure pt-3 text-xs">
        <font-awesome-icon icon="fa-solid fa-calendar-days" class="shrink-0 text-af-atone-2" />
        <span class="font-bold text-af-chocolat">{{ dateDebut }}</span>
        <span class="text-af-atone">{{ programme.duree_label }}</span>
      </p>
    </div>
  </NuxtLink>
</template>

<script setup lang="ts">
import type { SabbatiqueAPI } from '~/composables/useSabbatiques'

const props = defineProps<{
  programme: SabbatiqueAPI
}>()

const couverture = computed(() => urlMedia(props.programme.couverture_url))

/**
 * `date_debut` est un DATE nu (« 2026-06-01 »). Sans le `T00:00:00`, il serait
 * lu en UTC puis rendu en heure locale : à l'ouest de Greenwich, le 1er juin
 * s'afficherait « 31 mai ».
 */
const dateDebut = computed(() =>
  new Intl.DateTimeFormat('fr-FR', { day: 'numeric', month: 'short', year: 'numeric' })
    .format(new Date(`${props.programme.date_debut}T00:00:00`)))
</script>
