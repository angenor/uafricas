<script setup lang="ts">
import { africaCulturePageData } from '~/mocks/africa-culture'

/**
 * Afroculture : porté sur le gabarit de la refonte.
 *
 * La modale « C'est quoi Afroculture ? » vivait en ligne dans la page : 100
 * lignes de balisage qui refaisaient, en moins bien, ce que
 * `AfricansModaleDecouverte` fait déjà pour Afripulse et Codimoi. Le texte est
 * repris mot pour mot dans `AfricaCultureDecouverteModale`.
 */
definePageMeta({ layout: false })

useHead({
  title: 'Afroculture | AfricanS',
  meta: [
    { name: 'description', content: 'Découvrez la richesse culturelle africaine et les opportunités de la diaspora' },
  ],
})

const presentationOuverte = ref(false)
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Afroculture"
        sous-titre="Richesse culturelle africaine"
        image="/images/africans/heros/hero-afroculture.jpg"
        aide="C'est quoi Afroculture ?"
        @aide="presentationOuverte = true"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane :segments="[{ libelle: 'Africarise', vers: '/codi-moi' }, { libelle: 'Afroculture' }]">
        <template #centre>
          <p class="text-base font-bold text-af-encre">
            Quatre portes d'entrée vers les cultures du continent
          </p>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="grid gap-5 sm:grid-cols-2">
      <NuxtLink
        v-for="card in africaCulturePageData.cards"
        :key="card.id"
        :to="card.link"
        class="group relative block h-64 overflow-hidden rounded-[10px] border border-af-bordure transition hover:border-af-chocolat"
      >
        <img
          :src="card.image"
          alt=""
          class="absolute inset-0 size-full object-cover transition-transform duration-700 group-hover:scale-105"
          loading="lazy"
        />
        <div class="absolute inset-0 bg-linear-to-t from-black/85 via-black/35 to-transparent" />

        <div class="relative flex h-full flex-col justify-end p-6">
          <!-- Le filet coloré est la seule chose que la carte d'origine
               distinguait entre vert et chocolat : conservé tel quel. -->
          <div
            class="border-l-4 pl-4 transition-all duration-300 group-hover:border-l-8"
            :class="card.borderColor === 'green' ? 'border-af-vert' : 'border-af-chocolat'"
          >
            <h2 class="text-[24px]/[1.3] font-bold text-white">{{ card.title }}</h2>
            <p class="mt-1 text-[14px]/[1.5] text-white/90">{{ card.description }}</p>
          </div>
        </div>
      </NuxtLink>
    </div>

    <AfricaCultureDecouverteModale v-model="presentationOuverte" />
  </NuxtLayout>
</template>
