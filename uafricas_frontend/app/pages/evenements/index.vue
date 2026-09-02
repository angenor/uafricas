<script setup lang="ts">
/**
 * Forum : porté sur le gabarit de la refonte.
 *
 * La page est un simple carrefour vers trois modules. Elle chargeait ses trois
 * illustrations depuis **trois sites tiers** (agoraafricaine.info, fratmat.info
 * et learnthings.fr) : images hotlinkées, qui cassent le jour où ces sites les
 * déplacent, et qui envoient l'adresse IP de chaque visiteur à trois hôtes
 * étrangers au projet. La troisième, en prime, était une photo de banque
 * d'images sans rapport avec l'Afrique. Les trois visuels du dépôt les
 * remplacent.
 */
definePageMeta({ layout: false })

useHead({
  title: 'Forum : Événements & rencontres | AfricanS',
  meta: [
    {
      name: 'description',
      content: 'Codi-Moi, Afrolang et Africalive : les trois espaces de rencontre de la plateforme.',
    }],
})

const decouverteOuverte = ref(false)

const ESPACES = [
  {
    titre: 'Codi-Moi',
    description: 'Codifier et transmettre les savoirs africains, éducation, santé, agriculture.',
    image: '/images/africans/heros/hero-codimoi.jpg',
    vers: '/codi-moi',
  },
  {
    titre: 'Afrolang',
    description: 'Découvrez les langues et cultures africaines à travers notre programme linguistique.',
    image: '/images/africans/heros/hero-afrolang.jpg',
    vers: '/afrolang',
  },
  {
    titre: 'Événements & ateliers',
    description: 'Participez aux événements culturels, ateliers et rencontres de la communauté.',
    image: '/images/even1.png',
    vers: '/evenements/liste',
  }]
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Forum"
        sous-titre="Événements & rencontres"
        image="/images/even1.png"
        aide="C'est quoi Africalive ?"
        @aide="decouverteOuverte = true"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane :segments="[{ libelle: 'Forum' }]">
        <template #action>
          <AfricansBouton icone="fa-solid fa-calendar-days" vers="/evenements/liste">
            Voir les événements
          </AfricansBouton>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="grid gap-5 sm:grid-cols-2">
      <NuxtLink
        v-for="espace in ESPACES"
        :key="espace.titre"
        :to="espace.vers"
        class="group flex flex-col overflow-hidden rounded-[10px] border border-af-bordure bg-white transition hover:border-af-chocolat"
      >
        <div class="aspect-[16/10] w-full overflow-hidden bg-af-fond">
          <img
            :src="espace.image"
            alt=""
            class="size-full object-cover transition duration-300 group-hover:scale-105"
          />
        </div>
        <div class="flex flex-1 flex-col gap-2 p-4">
          <h2 class="text-[17px]/[1.4] font-bold text-af-encre">{{ espace.titre }}</h2>
          <p class="text-[14px]/[1.4] text-af-corps">{{ espace.description }}</p>
          <span class="mt-auto flex items-center gap-2 pt-2 text-[14px]/[1.4] font-bold text-af-chocolat">
            Découvrir
            <font-awesome-icon icon="fa-solid fa-arrow-right" class="transition group-hover:translate-x-1" />
          </span>
        </div>
      </NuxtLink>
    </div>

    <EvenementsDecouverteModale v-model="decouverteOuverte" />
  </NuxtLayout>
</template>
