<script setup lang="ts">
import { mediaCards } from '~/mocks/medias'

/**
 * Africamood : porté sur le gabarit de la refonte.
 *
 * Le bandeau de statistiques est SUPPRIMÉ. Il annonçait « 150+ médias
 * disponibles, 54 territoires couverts, 24/7, HD+ » : quatre valeurs écrites en
 * dur dans `mocks/medias.ts`, qu'aucune requête n'a jamais vérifiées. Les
 * endpoints existent pourtant (`/api/television/sections`,
 * `/api/stations-radio/sections`) et renvoient aujourd'hui 0, le chiffre
 * affiché n'était pas seulement invérifié, il était faux.
 *
 * Rien ne le remplace : un décompte réel appartient aux deux pages qui
 * l'établissent, pas à un carrefour qui n'a que deux liens à donner.
 */
definePageMeta({ layout: false })

useHead({
  title: 'Africamood : Radios & Télés africaines | AfricanS',
  meta: [
    { name: 'description', content: 'Explorez notre collection de radios et télés africaines' }],
})
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Africamood"
        sous-titre="Explorez notre collection de radios et télés africaines"
        image="/images/banners/radio-home.jpg"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane :segments="[{ libelle: 'Africamood' }]">
        <template #centre>
          <p class="text-base font-bold text-af-encre">Deux antennes, un continent</p>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="grid gap-5 sm:grid-cols-2">
      <NuxtLink
        v-for="card in mediaCards"
        :key="card.id"
        :to="card.link"
        class="group flex flex-col overflow-hidden rounded-[10px] border border-af-bordure bg-white transition hover:border-af-chocolat"
      >
        <div class="relative aspect-[16/10] overflow-hidden">
          <img
            :src="card.image"
            alt=""
            class="size-full object-cover transition-transform duration-500 group-hover:scale-105"
            loading="lazy"
          />
          <span
            class="absolute top-4 right-4 rounded px-3 py-1 text-[12px]/[1.4] font-bold text-white"
            :class="card.badgeColor === 'green' ? 'bg-af-vert' : 'bg-af-chocolat'"
          >
            {{ card.badge }}
          </span>
        </div>

        <div class="flex flex-1 flex-col gap-2 p-5">
          <h2 class="text-[24px]/[1.3] font-bold text-af-encre transition group-hover:text-af-chocolat">
            {{ card.title }}
          </h2>
          <p class="text-[14px]/[1.5] text-af-corps">{{ card.description }}</p>
          <span class="mt-auto flex items-center gap-2 pt-3 text-[14px]/[1.4] font-bold text-af-chocolat">
            Explorer
            <font-awesome-icon icon="fa-solid fa-arrow-right" class="transition-transform group-hover:translate-x-1" />
          </span>
        </div>
      </NuxtLink>
    </div>
  </NuxtLayout>
</template>
