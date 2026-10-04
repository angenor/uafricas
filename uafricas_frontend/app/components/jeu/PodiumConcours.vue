<template>
  <section v-if="podium.length" class="flex flex-col gap-4">
    <h2 class="text-[18px]/[1.4] font-bold text-af-encre">Podium</h2>
    <ol class="grid gap-4 sm:grid-cols-3" role="list">
      <li
        v-for="(p, i) in podium"
        :key="p.id"
        class="flex flex-col overflow-hidden rounded-[10px] border-2 bg-af-surface"
        :class="[ORDRE[i], p.rang === 1 ? 'border-af-orange' : 'border-af-bordure', MARCHE[p.rang ?? 3]]"
      >
        <div class="relative">
          <img :src="urlMedia(p.media_url) ?? undefined" :alt="p.legende ?? `Photo classée ${p.rang}`" class="aspect-[4/3] w-full object-cover">
          <span class="absolute left-3 top-3 flex items-center gap-1.5 rounded-full bg-af-surface/95 px-3 py-1 text-[13px] font-bold text-af-encre">
            <font-awesome-icon icon="fa-solid fa-medal" :class="couleurMedaille(p.rang)" />
            {{ p.rang }}<sup>{{ p.rang === 1 ? 're' : 'e' }}</sup>
          </span>
        </div>
        <div class="flex flex-col gap-1 p-4">
          <p class="flex items-center gap-2 text-[15px]/[1.4] font-bold text-af-encre">
            <AfricansAvatar :src="urlMedia(p.auteur.photo_url)" :nom="`${p.auteur.prenom} ${p.auteur.nom}`" :taille="28" />
            {{ p.auteur.prenom }} {{ p.auteur.nom }}
          </p>
          <p v-if="p.legende" class="text-[14px]/[1.4] text-af-corps">« {{ p.legende }} »</p>
          <p class="text-[13px]/[1.4] text-af-atone">
            <template v-if="p.place_jury">Choisie par le jury · </template>{{ p.taux }} % des duels gagnés sur {{ p.duels }}
          </p>
        </div>
      </li>
    </ol>
  </section>
</template>

<script setup lang="ts">
import type { ParticipationClasseeAPI } from '~/composables/useConcours'

/**
 * Le podium d'un concours terminé (feature 014, US7) : les trois premières
 * places, auteurs révélés, avec la part de duels gagnés qui les justifie
 * (FR-045). Les ex aequo partagent un rang : il peut y avoir deux « 1re ».
 */
const props = defineProps<{ elements: ParticipationClasseeAPI[] }>()

const podium = computed(() =>
  props.elements.filter(p => p.rang != null && p.rang <= 3 && (!p.sous_seuil || p.place_jury)).slice(0, 3))

// La place à l'écran suit la POSITION dans le podium (centre, gauche, droite),
// la hauteur de marche suit le RANG : deux 2ᵉ ex aequo se rangent côte à côte,
// à la même hauteur, sans déloger le 1er du centre.
const ORDRE = ['sm:order-2', 'sm:order-1', 'sm:order-3']
const MARCHE: Record<number, string> = { 1: '', 2: 'sm:mt-6', 3: 'sm:mt-10' }

const couleurMedaille = (rang: number | null) =>
  rang === 1 ? 'text-af-orange' : rang === 2 ? 'text-af-atone' : 'text-af-chocolat'
</script>
