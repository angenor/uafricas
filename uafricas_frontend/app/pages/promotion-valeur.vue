<script setup lang="ts">
import { promotionValeurCards } from '~/mocks/promotion-valeur'

/**
 * Promotion des Valeurs : porté sur le gabarit de la refonte.
 *
 * Les TROIS images des cartes étaient hébergées chez des tiers (wikimedia,
 * istockphoto, static-rmg.be) ; la première ne chargeait pas et la carte ne
 * montrait plus que son texte de remplacement. Elles sont rapatriées en local.
 *
 * Le dégradé coloré derrière chaque image disparaît avec elles : `from-green-500
 * to-blue-500`, `from-purple-500 to-pink-500`… trois duos pris hors de la
 * palette de marque, qui teintaient les photos en `mix-blend-overlay`.
 *
 * Le fil d'Ariane annonçait « AfricaCulture » : le module s'appelle
 * **Afroculture** depuis la refonte, et son univers est Africarise.
 *
 * Cartes horizontales, et non une grille de trois : la colonne principale fait
 * 739 px, trois colonnes y donneraient 220 px chacune, assez pour l'image,
 * trop peu pour le texte.
 */
definePageMeta({ layout: false })

/** Les trois autres portes d'entrée du carrefour Afroculture. */
const AUTRES_ENTREES = [
  { libelle: 'Expertise de la diaspora', to: '/experts', icone: 'fa-solid fa-user-tie' },
  { libelle: 'Opportunités en Afrique', to: '/opportunite-afrique', icone: 'fa-solid fa-earth-africa' },
  { libelle: 'Échanges sabbatiques', to: '/echanges-sabbatiques', icone: 'fa-solid fa-right-left' }]

useHead({
  title: 'Promotion des Valeurs Africaines | AfricanS',
  meta: [
    {
      name: 'description',
      content: 'Découvrez les valeurs africaines et afro-descendantes à travers ForAfrica, Afrocult et Afromarket',
    },
  ],
})
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Promotion des Valeurs"
        sous-titre="Valeurs africaines et afro-descendantes"
        image="/images/centre-culturel.jpg"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[
          { libelle: 'Africarise', vers: '/codi-moi' },
          { libelle: 'Afroculture', vers: '/africa-culture' },
          { libelle: 'Promotion des Valeurs' }]"
      >
        <template #centre>
          <p class="text-base font-bold text-af-encre">Trois façons de faire vivre notre héritage</p>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="flex flex-col gap-5">
      <article
        v-for="card in promotionValeurCards"
        :key="card.id"
        class="group flex flex-col overflow-hidden rounded-[10px] border border-af-bordure bg-white transition hover:border-af-chocolat sm:flex-row"
      >
        <div class="relative aspect-[16/10] shrink-0 overflow-hidden sm:aspect-auto sm:w-64">
          <img
            :src="card.image"
            alt=""
            class="size-full object-cover transition-transform duration-500 group-hover:scale-105"
            loading="lazy"
          />
        </div>

        <div class="flex min-w-0 flex-1 flex-col items-start gap-3 p-6">
          <h2 class="text-[24px]/[1.3] font-bold text-af-encre transition group-hover:text-af-chocolat">
            {{ card.title }}
          </h2>
          <p class="text-[14px]/[1.5] text-af-corps">{{ card.description }}</p>
          <AfricansBouton class="mt-auto" icone="fa-solid fa-arrow-right" :vers="card.link">
            {{ card.buttonText }}
          </AfricansBouton>
        </div>
      </article>
    </div>

    <template #rail>
      <AfricansPanneau titre="Aussi dans Afroculture" icone="fa-solid fa-masks-theater">
        <ul class="flex flex-col gap-1">
          <li v-for="lien in AUTRES_ENTREES" :key="lien.to">
            <NuxtLink
              :to="lien.to"
              class="flex items-center gap-3 rounded-lg px-3 py-2.5 text-[14px]/[1.4] font-bold text-af-corps transition hover:bg-af-chocolat/[0.07] hover:text-af-chocolat"
            >
              <font-awesome-icon :icon="lien.icone" class="size-5 shrink-0" />
              {{ lien.libelle }}
            </NuxtLink>
          </li>
        </ul>
      </AfricansPanneau>
    </template>
  </NuxtLayout>
</template>
