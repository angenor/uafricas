<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section (compact, titre ↔ description au survol) -->
    <div class="group relative bg-font-centre-culturel bg-cover bg-center">
      <div class="absolute inset-0 bg-linear-to-r from-custom-chocolat/90 to-black/70"></div>

      <div class="relative max-w-4xl mx-auto px-4 pt-16 pb-6 text-center select-none">
        <!-- Conteneur fixe : le titre et la description se superposent (crossfade au survol) -->
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12">
          <h1 class="absolute inset-0 flex items-center justify-center text-white text-2xl md:text-4xl font-bold transition-opacity duration-300 group-hover:opacity-0">
            Afroculture
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Richesse culturelle africaine
          </p>
        </div>

        <div class="mt-4 flex flex-wrap items-center justify-center gap-3">
          <!-- Bouton d'aide : ouvre la présentation d'Afroculture -->
          <button
            type="button"
            class="inline-flex items-center gap-2 rounded-full bg-white/15 hover:bg-white/25 text-white font-medium text-sm px-4 py-2.5 backdrop-blur-xs ring-1 ring-white/25 transition-colors"
            aria-label="En savoir plus sur Afroculture"
            @click="presentationOuverte = true"
          >
            <font-awesome-icon :icon="['fas', 'circle-question']" class="w-4 h-4" />
            C'est quoi Afroculture&nbsp;?
          </button>
        </div>
      </div>
    </div>

    <!-- Modale de présentation « C'est quoi Afroculture ? » (inline, auto-suffisante) -->
    <Transition name="modal-fade">
      <div
        v-if="presentationOuverte"
        class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/75 backdrop-blur-xs"
        @click.self="presentationOuverte = false"
      >
        <div
          class="relative w-full max-w-2xl max-h-[90vh] flex flex-col bg-white shadow-2xl rounded-3xl overflow-hidden"
          @click.stop
        >
          <!-- En-tête -->
          <div class="relative shrink-0 bg-linear-to-r from-custom-chocolat to-custom-chocolat/80 px-6 py-6 text-white">
            <button
              type="button"
              class="absolute top-4 right-4 text-white/80 hover:text-white transition-colors"
              aria-label="Fermer"
              @click="presentationOuverte = false"
            >
              <font-awesome-icon :icon="['fas', 'xmark']" class="w-5 h-5" />
            </button>

            <div class="flex items-center gap-3">
              <div class="w-12 h-12 rounded-2xl bg-white/15 flex items-center justify-center shrink-0">
                <font-awesome-icon :icon="['fas', 'masks-theater']" class="w-6 h-6" />
              </div>
              <div>
                <h2 class="text-xl md:text-2xl font-bold leading-tight">Afroculture</h2>
                <p class="text-white/90 text-sm">Un pont entre les cultures d'Afrique et de sa diaspora</p>
              </div>
            </div>
          </div>

          <!-- Corps défilant -->
          <div class="overflow-y-auto px-6 py-6 space-y-8">
            <!-- L'accroche -->
            <p class="text-gray-700 leading-relaxed">
              Les cultures africaines et afrodescendantes sont d'une richesse immense, mais
              restent souvent dispersées et peu visibles. <strong class="text-gray-900">Afroculture</strong>
              est un espace en ligne qui relie les communautés culturelles du continent, les
              afrodescendants et la diaspora pour <strong class="text-gray-900">promouvoir,
              transmettre et faire dialoguer nos cultures</strong>.
            </p>

            <!-- Ce que vous pouvez y faire -->
            <div>
              <h3 class="text-sm font-bold uppercase tracking-wide text-custom-chocolat mb-4">
                Ce que vous pouvez y faire
              </h3>
              <div class="grid sm:grid-cols-2 gap-3">
                <div
                  v-for="item in fonctionnalites"
                  :key="item.titre"
                  class="flex gap-3 rounded-2xl border border-gray-100 bg-gray-50/60 p-4"
                >
                  <div class="w-10 h-10 rounded-xl bg-custom-green/10 text-custom-green flex items-center justify-center shrink-0">
                    <font-awesome-icon :icon="['fas', item.icone]" class="w-5 h-5" />
                  </div>
                  <div class="min-w-0">
                    <p class="font-semibold text-gray-900 text-sm">{{ item.titre }}</p>
                    <p class="text-gray-500 text-xs mt-0.5 leading-relaxed">{{ item.texte }}</p>
                  </div>
                </div>
              </div>
            </div>

            <!-- Notre objectif -->
            <div class="rounded-2xl bg-custom-green/5 border border-custom-green/15 p-5">
              <h3 class="flex items-center gap-2 text-sm font-bold text-custom-green mb-2">
                <font-awesome-icon :icon="['fas', 'seedling']" class="w-4 h-4" />
                Notre objectif
              </h3>
              <p class="text-gray-700 text-sm leading-relaxed">
                Renforcer l'identité africaine et afrodescendante, et nourrir le dialogue
                interculturel et la compréhension mutuelle entre nos communautés.
              </p>
            </div>
          </div>

          <!-- Pied -->
          <div class="shrink-0 border-t border-gray-100 px-6 py-4 bg-gray-50/50">
            <button
              type="button"
              class="w-full sm:w-auto sm:ml-auto sm:block px-6 py-2.5 rounded-full bg-custom-chocolat text-white font-semibold text-sm hover:bg-custom-chocolat/90 transition-colors"
              @click="presentationOuverte = false"
            >
              J'ai compris
            </button>
          </div>
        </div>
      </div>
    </Transition>

    <!-- Breadcrumb -->
    <div class="max-w-7xl mx-auto px-4 py-3">
      <BreadcrumbNav :custom-breadcrumbs="breadcrumbs" />
    </div>

    <!-- Cards Grid -->
    <div class="max-w-7xl mx-auto px-4 pb-12 sm:pb-16">
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 sm:gap-6 lg:gap-8">
        <NuxtLink
          v-for="card in africaCulturePageData.cards"
          :key="card.id"
          :to="card.link"
          class="group relative rounded-2xl shadow-lg hover:shadow-2xl transition-all duration-500 overflow-hidden h-48 sm:h-56 lg:h-64 transform hover:-translate-y-1"
        >
          <img
            class="absolute inset-0 w-full h-full object-cover transition-transform duration-700 group-hover:scale-110"
            :src="card.image"
            :alt="card.title"
            loading="lazy"
          />
          <div class="absolute inset-0 bg-linear-to-t from-black/80 via-black/30 to-transparent" />

          <div class="relative z-10 p-4 sm:p-6 h-full flex flex-col justify-end">
            <div
              class="border-l-4 group-hover:border-l-8 pl-3 sm:pl-4 transition-all duration-300"
              :class="card.borderColor === 'green' ? 'border-custom-green' : 'border-custom-chocolat'"
            >
              <h3
                class="text-lg sm:text-xl lg:text-2xl font-bold text-white mb-1 sm:mb-2 transition-colors duration-300"
                :class="card.borderColor === 'green' ? 'group-hover:text-custom-green' : 'group-hover:text-custom-chocolat'"
              >
                {{ card.title }}
              </h3>
              <p class="text-gray-200 text-xs sm:text-sm line-clamp-2">
                {{ card.description }}
              </p>
            </div>
          </div>
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { africaCulturePageData } from '~/mocks/africa-culture'

useHead({
  title: 'Afroculture - AfricanS',
  meta: [
    { name: 'description', content: 'Découvrez la richesse culturelle africaine et les opportunités de la diaspora' },
  ],
})

const breadcrumbs = [
  { label: 'Afroculture', to: undefined }
]

// Modale de présentation « C'est quoi Afroculture ? »
const presentationOuverte = ref(false)

const fonctionnalites = [
  {
    icone: 'masks-theater',
    titre: 'Promotion des valeurs',
    texte: 'Découvrez le patrimoine culturel africain et afrodescendant, matériel comme immatériel, et les rendez-vous, centres culturels et produits qui le font vivre.',
  },
  {
    icone: 'user-graduate',
    titre: 'Expertise de la diaspora',
    texte: 'Explorez les compétences et les talents de la diaspora africaine et découvrez ses experts.',
  },
  {
    icone: 'earth-africa',
    titre: 'Opportunités en Afrique',
    texte: 'Consultez des fiches d\'information détaillées sur les opportunités, secteur par secteur et territoire par territoire.',
  },
  {
    icone: 'right-left',
    titre: 'Échanges Sabbafrica',
    texte: 'Proposez ou découvrez des échanges d\'expérience et de formation entre la diaspora et les organisations d\'Afrique.',
  },
]
</script>

<style scoped>
.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: opacity 0.25s ease;
}
.modal-fade-enter-from,
.modal-fade-leave-to {
  opacity: 0;
}
</style>
