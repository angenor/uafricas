<template>
  <section id="decouvrir" class="w-full bg-af-fond py-16 font-af md:py-24">
    <div class="mx-auto max-w-af-conteneur px-6">
      <header class="flex flex-col items-center gap-3 text-center">
        <h2 class="text-[28px]/[1.4] font-bold text-af-chocolat sm:text-[36px]/[1.4] lg:text-[40px]/[1.4]">
          Découvrir AfricanS
        </h2>
        <p class="max-w-3xl text-[16px]/[1.4] text-af-corps sm:text-[20px]/[1.4]">
          Apprenez-en plus sur notre organisation, notre mission et comment nous rejoindre.
        </p>
      </header>

      <div class="mt-12 grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-5">
        <NuxtLink
          v-for="item in ITEMS"
          :key="item.titre"
          :to="item.vers"
          class="group flex flex-col items-center gap-2 rounded-[10px] border border-af-bordure bg-white p-6 text-center transition hover:border-af-chocolat"
        >
          <font-awesome-icon
            :icon="item.icone"
            class="mb-2 size-8 text-af-chocolat transition group-hover:scale-110"
          />
          <h3 class="text-[16px]/[1.4] font-bold text-af-encre transition group-hover:text-af-chocolat">
            {{ item.titre }}
          </h3>
          <p class="text-[12px]/[1.4] text-af-corps">{{ item.description }}</p>
        </NuxtLink>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
const ITEMS = [
  {
    icone: 'fa-solid fa-rocket',
    titre: 'Notre mission',
    description: 'Promouvoir les valeurs africaines et afro-descendantes pour un développement durable.',
    vers: '/a-propos/mission',
  },
  {
    icone: 'fa-solid fa-landmark',
    titre: 'Gouvernance',
    description: "Engager les citoyens africains dans l'amélioration de la gouvernance politique et sociale.",
    vers: '/universite/gouvernance',
  },
  {
    icone: 'fa-solid fa-handshake',
    titre: 'Nos partenaires',
    description: 'Découvrez les organisations qui soutiennent notre mission panafricaine.',
    vers: '/a-propos/partenaires',
  },
  {
    icone: 'fa-solid fa-circle-info',
    titre: 'FAQ',
    description: 'Retrouvez les réponses aux questions les plus fréquemment posées.',
    vers: '/a-propos/faq',
  },
  {
    icone: 'fa-solid fa-envelope',
    titre: 'Nous contacter',
    description: 'Pour toute question, suggestion ou proposition de collaboration.',
    vers: '/a-propos/contact',
  },
]
</script>
