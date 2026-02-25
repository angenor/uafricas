<template>
  <section id="decouvrir" class="py-16 md:py-24 w-full bg-gray-50">
    <div class="container mx-auto px-6 md:px-12 lg:px-20">
      <!-- Titre -->
      <div class="text-center mb-12">
        <h2 class="text-custom-chocolat text-3xl sm:text-4xl lg:text-5xl font-display">
          <span class="font-bold">D</span>écouvrir UAfricas
        </h2>
        <p class="text-gray-500 mt-3 text-lg max-w-2xl mx-auto">
          Apprenez-en plus sur notre organisation, notre mission et comment nous rejoindre.
        </p>
      </div>

      <!-- Grille de cartes -->
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-5 gap-6">
        <NuxtLink
          v-for="(item, index) in items"
          :key="item.titre"
          :to="item.lien"
          data-aos="fade-up"
          :data-aos-delay="index * 100"
          class="bg-white rounded-xl shadow-md hover:shadow-lg transition-all duration-300 p-6 text-center group border border-gray-100 hover:border-custom-green/30 block"
        >
          <!-- Icône -->
          <div
            class="w-14 h-14 mx-auto rounded-full flex items-center justify-center mb-4"
            :class="item.couleur === 'green'
              ? 'bg-custom-green/10 text-custom-green'
              : 'bg-custom-chocolat/10 text-custom-chocolat'"
          >
            <font-awesome-icon :icon="item.icone" class="w-6 h-6" />
          </div>

          <!-- Titre -->
          <h3 class="font-display text-lg font-bold text-gray-800 mb-2">
            {{ item.titre }}
          </h3>

          <!-- Description -->
          <p class="text-sm text-gray-600 leading-relaxed mb-4">
            {{ item.description }}
          </p>

          <!-- Lien -->
          <span class="inline-flex items-center gap-1 text-custom-green text-sm font-semibold group-hover:underline transition-colors">
            En savoir plus
            <font-awesome-icon icon="fa-solid fa-arrow-right" class="w-3 h-3" />
          </span>
        </NuxtLink>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
interface DecouvrirItem {
  icone: string
  titre: string
  description: string
  lien: string
  couleur: 'green' | 'chocolat'
}

const items: DecouvrirItem[] = [
  {
    icone: 'fa-solid fa-rocket',
    titre: 'Notre Mission',
    description: 'Promouvoir les valeurs africaines et afro-descendantes pour un développement durable.',
    lien: '/a-propos/mission',
    couleur: 'green'
  },
  {
    icone: 'fa-solid fa-landmark',
    titre: 'Gouvernance',
    description: 'Engager les citoyens africains dans l\'amélioration de la gouvernance politique et sociale.',
    lien: '/universite/gouvernance',
    couleur: 'chocolat'
  },
  {
    icone: 'fa-solid fa-handshake',
    titre: 'Nos Partenaires',
    description: 'Découvrez les organisations qui soutiennent notre mission panafricaine.',
    lien: '/a-propos/partenaires',
    couleur: 'green'
  },
  {
    icone: 'fa-solid fa-circle-info',
    titre: 'FAQ',
    description: 'Retrouvez les réponses aux questions les plus fréquemment posées.',
    lien: '/a-propos/faq',
    couleur: 'chocolat'
  },
  {
    icone: 'fa-solid fa-envelope',
    titre: 'Contactez-nous',
    description: 'Pour toute question, suggestion ou proposition de collaboration.',
    lien: '/a-propos/contact',
    couleur: 'green'
  }
]

useAOS()
</script>
