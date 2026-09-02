<script setup lang="ts">
import { MODULES_AFRICANS } from '~/utils/navigation-modules'

/**
 * Mindshiftlab : carrefour de l'univers Formation & savoir.
 *
 * Cette page manquait. Mindshiftlab était le seul des six univers à atterrir
 * sur l'une de ses PROPRES applications : sa cible était `/universite`, qui est
 * la page Muniversa. Le menu allumait donc l'univers ET l'application, sans se
 * tromper : les deux menaient bien au même endroit.
 *
 * Les cartes sont dérivées de `MODULES_AFRICANS`, la source que la barre
 * supérieure et la navigation latérale lisent déjà. Recopier les quatre
 * applications ici aurait créé une troisième liste, libre de diverger des deux
 * autres sans que rien ne le signale.
 *
 * Pas d'images : les applications n'en déclarent pas dans la source. Une
 * illustration choisie ici ne vivrait que sur cette page, et il faudrait
 * l'inventer. Les icônes, elles, sont déjà celles du menu.
 */
definePageMeta({ layout: false })

const univers = MODULES_AFRICANS.find(m => m.id === 'mindshiftlab')

useHead({
  title: 'Mindshiftlab : Formation & savoir | AfricanS',
  meta: [
    { name: 'description', content: univers?.description }],
})
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Mindshiftlab"
        :sous-titre="univers?.description"
        :image="univers?.image"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane :segments="[{ libelle: 'Mindshiftlab' }]">
        <template #centre>
          <p class="text-base font-bold text-af-encre">Se former, transmettre, se rencontrer</p>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="grid gap-5 sm:grid-cols-2">
      <NuxtLink
        v-for="app in univers?.items ?? []"
        :key="app.to"
        :to="app.to"
        class="group flex flex-col gap-3 rounded-[10px] border border-af-bordure bg-white p-6 transition hover:border-af-chocolat"
      >
        <span class="grid size-12 place-items-center rounded-full bg-af-chocolat/10 text-af-chocolat">
          <font-awesome-icon :icon="app.icon" class="text-xl" />
        </span>

        <h2 class="text-[24px]/[1.3] font-bold text-af-encre transition group-hover:text-af-chocolat">
          {{ app.label }}
        </h2>
        <p class="text-[14px]/[1.5] text-af-corps">{{ app.description }}</p>

        <span class="mt-auto flex items-center gap-2 pt-3 text-[14px]/[1.4] font-bold text-af-chocolat">
          Découvrir
          <font-awesome-icon icon="fa-solid fa-arrow-right" class="transition-transform group-hover:translate-x-1" />
        </span>
      </NuxtLink>
    </div>
  </NuxtLayout>
</template>
