<script setup lang="ts">
/**
 * Page Télé — vitrine éditorialisée (US1).
 *
 * Une vedette occupe tout l'écran à l'ouverture, puis le défilement révèle une
 * section par chaîne, chargée par pages successives. La grille de vignettes
 * filtrable qui tenait cette page auparavant a disparu : elle donnait à voir un
 * catalogue, non une programmation.
 */
import type { ProgrammeVedette, TeleSection, TvStat } from '~/composables/useTelevision'

const { obtenirVedette, listerSections, obtenirStats, chargement } = useTelevision()

useHead({
  title: 'Télévision Africaine | UAfricas',
  meta: [
    {
      name: 'description',
      content: 'Regardez les télés africaines : un programme à la une plein écran, puis une section par chaîne avec ses contenus.',
    },
  ],
})

const vedette = ref<ProgrammeVedette | null>(null)
const sections = ref<TeleSection[]>([])
const stats = ref<TvStat[]>([])

const page = ref(1)
const totalPages = ref(1)
const chargementSections = ref(false)

const presentationOuverte = ref(false)
const reglesOuvertes = ref(false)
const propositionOuverte = ref(false)
const ancreSections = ref<HTMLElement | null>(null)

const chargerPageSections = async (numero: number) => {
  if (chargementSections.value) return
  chargementSections.value = true
  const resultat = await listerSections({ page: numero, par_page: 6 })
  if (resultat) {
    sections.value = numero === 1 ? resultat.sections : [...sections.value, ...resultat.sections]
    totalPages.value = resultat.totalPages
    page.value = resultat.page
  }
  chargementSections.value = false
}

const encoreDesSections = computed(() => page.value < totalPages.value)

const allerAuxSections = () => {
  ancreSections.value?.scrollIntoView({ behavior: 'smooth', block: 'start' })
}

/**
 * Sentinelle de fin de liste : atteindre le bas charge la page suivante, sans
 * bouton à presser. Le repli manuel reste offert si l'API n'est pas disponible.
 */
const sentinelle = ref<HTMLElement | null>(null)
const { estVisible: sentinelleVisible } = useObservateurVisibilite(sentinelle, {
  uneSeuleFois: false,
  marge: '400px',
})

watch(sentinelleVisible, (visible) => {
  if (visible && encoreDesSections.value && !chargementSections.value) {
    chargerPageSections(page.value + 1)
  }
})

onMounted(async () => {
  const [resultatVedette, resultatStats] = await Promise.all([
    obtenirVedette(),
    obtenirStats(),
  ])
  vedette.value = resultatVedette
  if (resultatStats) stats.value = resultatStats
  await chargerPageSections(1)
})
</script>

<template>
  <div class="min-h-screen bg-gray-900">
    <!-- Vedette plein écran (FR-002) -->
    <MediaVedettePleinEcran
      :programme="vedette"
      :chargement="chargement && !vedette"
      @defiler="allerAuxSections"
    />

    <div ref="ancreSections" class="px-4 py-12">
      <div class="max-w-6xl mx-auto">
        <h2 class="text-3xl font-bold text-white mb-4 text-center">
          Nos télés <span class="text-yellow-400">Africaines</span>
        </h2>

        <div class="mb-10 flex flex-wrap items-center justify-center gap-3">
          <button
            type="button"
            class="inline-flex items-center gap-2 rounded-full bg-white/15 hover:bg-white/25 text-white font-medium text-sm px-4 py-2.5 backdrop-blur-xs ring-1 ring-white/25 transition-colors"
            aria-label="En savoir plus sur Africans Télé"
            @click="presentationOuverte = true"
          >
            <font-awesome-icon :icon="['fas', 'circle-question']" class="w-4 h-4" />
            C'est quoi Africans Télé&nbsp;?
          </button>
          <button
            type="button"
            class="inline-flex items-center gap-2 rounded-full border border-custom-green bg-custom-green/10 text-custom-green px-5 py-2 text-sm font-semibold hover:bg-custom-green/20 transition-colors cursor-pointer"
            @click="propositionOuverte = true"
          >
            <font-awesome-icon :icon="['fas', 'plus']" class="w-4 h-4" />
            Proposer un contenu
          </button>
          <button
            type="button"
            class="inline-flex items-center gap-2 rounded-full border border-white/25 text-gray-300 px-5 py-2 text-sm font-semibold hover:border-orange-400 hover:text-orange-400 transition-colors cursor-pointer"
            @click="reglesOuvertes = true"
          >
            <font-awesome-icon :icon="['fas', 'shield-halved']" class="w-4 h-4" />
            Règles de contenu
          </button>
        </div>

        <MediaTelePresentationModal
          :open="presentationOuverte"
          @close="presentationOuverte = false"
        />

        <!-- Contenus interdits et conséquences d'un signalement (FR-048). -->
        <MediaReglesContenuModal
          :open="reglesOuvertes"
          @close="reglesOuvertes = false"
        />

        <!-- Toute proposition part en attente de validation (FR-031). -->
        <MediaProposerMediaModal
          :is-open="propositionOuverte"
          :types-offerts="['chaine_tv', 'programme_tele']"
          @close="propositionOuverte = false"
        />

        <!-- Statistiques -->
        <div
          v-if="stats.length > 0"
          class="bg-linear-to-r from-custom-green to-custom-chocolat rounded-2xl p-8 text-white mb-4"
        >
          <div class="grid grid-cols-2 md:grid-cols-4 gap-6 text-center">
            <div v-for="stat in stats" :key="stat.label" class="p-4">
              <div class="text-4xl font-bold mb-2">{{ stat.value }}</div>
              <div class="text-sm opacity-80">{{ stat.label }}</div>
            </div>
          </div>
        </div>

        <!-- Une section par chaîne, empilées et découvertes au défilement -->
        <MediaSectionChaine
          v-for="section in sections"
          :key="section.chaine.id"
          :section="section"
        />

        <div v-if="chargementSections" class="flex justify-center py-12">
          <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-yellow-400" />
        </div>

        <!-- Aucune chaîne ne porte de contenu publié (FR-008) -->
        <div v-else-if="!sections.length" class="text-center py-16">
          <p class="text-gray-400 text-lg">Aucune chaîne ne diffuse encore de contenu</p>
          <p class="text-gray-500 text-sm mt-2">Revenez bientôt : les programmes arrivent.</p>
        </div>

        <div ref="sentinelle" class="h-px" />

        <div v-if="encoreDesSections && !chargementSections" class="flex justify-center pt-4">
          <button
            type="button"
            class="rounded-full border border-white/25 text-white text-sm px-6 py-2.5 hover:bg-white/10 transition-colors"
            @click="chargerPageSections(page + 1)"
          >
            Voir plus de chaînes
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
