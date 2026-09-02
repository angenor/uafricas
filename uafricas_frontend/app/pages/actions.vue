<script setup lang="ts">
import { actionCards } from '~/mocks/actions'

/**
 * Actions : porté sur le gabarit de la refonte.
 *
 * Trois choses partent avec l'ancienne mise en page :
 *   - l'image de fond du bandeau, hébergée sur unsplash.com. Une image chez un
 *     tiers dépend de son hébergeur ET du navigateur du visiteur ;
 *   - le titre qui se changeait en description au survol du bandeau. Un titre
 *     qui disparaît quand la souris passe dessus n'est pas une révélation,
 *     c'est une perte de repère : les deux sont désormais affichés ;
 *   - GSAP, qui n'animait que l'entrée en scène de quatre cartes.
 */
definePageMeta({ layout: false })

useHead({
  title: 'Actions | AfricanS',
  meta: [
    { name: 'description', content: "Découvrez nos initiatives pour l'Afrique" }],
})

const LienNuxt = resolveComponent('NuxtLink')
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Actions"
        sous-titre="Initiatives pour l'Afrique"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane :segments="[{ libelle: 'Actions' }]">
        <template #centre>
          <p class="text-base font-bold text-af-encre">Quatre axes d'engagement</p>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="grid gap-5 sm:grid-cols-2">
      <!-- Une carte sans destination reste une <div> : `component :is` évite le
           lien mort. Un `<a href="#">` remonterait en haut de la page, ce qui
           se lit comme une panne. -->
      <component
        :is="card.link ? LienNuxt : 'div'"
        v-for="card in actionCards"
        :key="card.id"
        :to="card.link ?? undefined"
        class="group relative block h-72 overflow-hidden rounded-[10px] border border-af-bordure"
        :class="card.link ? 'transition hover:border-af-chocolat' : 'cursor-default'"
      >
        <img
          :src="card.image"
          alt=""
          class="absolute inset-0 size-full object-cover transition-transform duration-700 group-hover:scale-105"
          loading="lazy"
        />
        <div class="absolute inset-0 bg-linear-to-t from-black/85 via-black/35 to-transparent" />

        <span
          class="absolute top-4 right-4 grid size-12 place-items-center rounded-full text-white"
          :class="card.id % 2 === 0 ? 'bg-af-chocolat' : 'bg-af-vert'"
        >
          <font-awesome-icon :icon="card.icon" class="text-lg" />
        </span>

        <div class="relative flex h-full flex-col justify-end gap-2 p-6 text-white">
          <h2 class="text-[24px]/[1.3] font-bold">{{ card.title }}</h2>
          <p class="text-[14px]/[1.5] text-white/90">{{ card.description }}</p>

          <span v-if="card.link" class="mt-1 flex items-center gap-2 text-[14px]/[1.4] font-bold">
            En savoir plus
            <font-awesome-icon icon="fa-solid fa-arrow-right" class="transition-transform group-hover:translate-x-1" />
          </span>
          <!-- Dit ce qui est, plutôt que de simuler un lien : le thème est
               annoncé, le module n'existe pas encore. -->
          <span v-else class="mt-1 w-fit rounded bg-white/20 px-3 py-1 text-[12px]/[1.4] font-bold">
            Bientôt disponible
          </span>
        </div>
      </component>
    </div>
  </NuxtLayout>
</template>
