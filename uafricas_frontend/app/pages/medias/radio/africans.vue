<script setup lang="ts">
/**
 * Radio Africans — production propre de la plateforme (US2).
 *
 * Page DISTINCTE de `/medias/radio/nationales` : l'adresse est inchangée
 * (FR-012) et l'origine `africans` est fixée ici, en dur. C'est elle qui
 * garantit qu'aucune station n'apparaît sur les deux pages (FR-014).
 *
 * L'écoute est confiée à la barre de lecture du layout, qui survit au
 * défilement et à la navigation (FR-017) : cette page ne monte aucun lecteur.
 */
import type { StationSection } from '~/composables/useStationsRadio'

const { listerSections, listerPays, listerGenres, chargement } = useStationsRadio()
// Référentiels de FILTRE (US3, US4) : seulement ce qui est réellement déclaré.
const { listerThematiquesDisponibles, listerTerritoiresDisponibles } = useMediaSupport()
const { aUnContenu, enLecture } = useLecteurMedia()

useHead({
  title: 'Radios Africans | AfricanS',
  meta: [
    {
      name: 'description',
      content: 'Écoutez Radio Africans : les stations produites par la plateforme, station par station, avec leurs émissions.',
    },
  ],
})

const sections = ref<StationSection[]>([])
const paysDisponibles = ref<string[]>([])
const genresDisponibles = ref<string[]>([])

const propositionOuverte = ref(false)
const reglesOuvertes = ref(false)

const page = ref(1)
const totalPages = ref(1)
const totalStations = ref(0)

const thematiquesSelectionnees = ref<string[]>([])
const territoireSelectionne = ref('')
const thematiquesDisponibles = ref<{ id: string, nom: string, nombre_supports: number }[]>([])
const territoiresDisponibles = ref<{ id: string, nom: string, nombre_supports: number }[]>([])
const stationsContinentales = ref(0)

const typeSelectionne = ref('Tous les types')
const paysSelectionne = ref('Tous les territoires')
const genreSelectionne = ref('Tous les genres')

const typesProgramme = ['Tous les types', 'Nationales', 'Local', 'International']

const chargerSections = async (numero = 1) => {
  const resultat = await listerSections({
    // Fixée par la page, jamais offerte au visiteur comme filtre.
    origine: 'africans',
    type_station: typeSelectionne.value,
    pays: paysSelectionne.value,
    genre: genreSelectionne.value,
    thematiques: thematiquesSelectionnees.value,
    territoire: territoireSelectionne.value,
    page: numero,
    par_page: 6,
  })
  if (resultat) {
    sections.value = numero === 1 ? resultat.sections : [...sections.value, ...resultat.sections]
    totalPages.value = resultat.totalPages
    totalStations.value = resultat.total
    page.value = resultat.page
  }
}

const reinitialiserFiltres = () => {
  typeSelectionne.value = 'Tous les types'
  paysSelectionne.value = 'Tous les territoires'
  genreSelectionne.value = 'Tous les genres'
  thematiquesSelectionnees.value = []
  territoireSelectionne.value = ''
}

watch(
  [typeSelectionne, paysSelectionne, genreSelectionne, thematiquesSelectionnees, territoireSelectionne],
  () => chargerSections(1),
)

const encoreDesStations = computed(() => page.value < totalPages.value)
const filtresActifs = computed(() =>
  typeSelectionne.value !== 'Tous les types'
  || paysSelectionne.value !== 'Tous les territoires'
  || genreSelectionne.value !== 'Tous les genres'
  || thematiquesSelectionnees.value.length > 0
  || territoireSelectionne.value !== '',
)

onMounted(async () => {
  const [pays, genres, thematiques, territoires] = await Promise.all([
    listerPays(),
    listerGenres(),
    listerThematiquesDisponibles('station_radio'),
    listerTerritoiresDisponibles('station_radio'),
  ])
  if (pays) paysDisponibles.value = pays
  if (genres) genresDisponibles.value = genres
  thematiquesDisponibles.value = thematiques
  territoiresDisponibles.value = territoires.territoires
  stationsContinentales.value = territoires.continentales
  await chargerSections(1)
})
</script>

<template>
  <div class="min-h-screen bg-linear-to-br from-custom-chocolat to-black relative overflow-hidden">
    <MediaDecorRadio :actif="aUnContenu && enLecture" />

    <div class="container mx-auto px-4 pt-28 pb-16 relative z-10">
      <!-- Bandeau d'accroche, sans lecteur en tête (FR-013) -->
      <header class="text-center mb-10">
        <h1 class="text-4xl md:text-6xl font-bold text-white mb-4">
          Radios <span class="text-yellow-400">Africans</span>
        </h1>
        <p class="text-gray-300 max-w-2xl mx-auto">
          Les stations produites par Africans, station par station, avec leurs émissions.
        </p>

        <div class="mt-6 flex flex-wrap items-center justify-center gap-3">
          <button
            type="button"
            class="inline-flex items-center gap-2 rounded-full border border-yellow-400 bg-yellow-400/10 text-yellow-400 px-5 py-2 text-sm font-semibold hover:bg-yellow-400/20 transition-colors cursor-pointer"
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
      </header>


      <!-- Toute proposition part en attente de validation (FR-031). -->
      <MediaProposerMediaModal
        :is-open="propositionOuverte"
        :types-offerts="['station_radio', 'emission_radio']"
        @close="propositionOuverte = false"
      />

      <!-- Contenus interdits et conséquences d'un signalement (FR-048). -->
      <MediaReglesContenuModal
        :open="reglesOuvertes"
        @close="reglesOuvertes = false"
      />

      <MediaFilters
        :types="typesProgramme"
        :countries="paysDisponibles"
        :genres="genresDisponibles"
        :selected-type="typeSelectionne"
        :selected-country="paysSelectionne"
        :selected-genre="genreSelectionne"
        @select-type="typeSelectionne = $event"
        @select-country="paysSelectionne = $event"
        @select-genre="genreSelectionne = $event"
        @reset="reinitialiserFiltres"
      />

      <!-- Thématique déclarée et couverture : d'une autre nature que le genre
           et le pays de siège ci-dessus (US3, US4). -->
      <MediaBarreFiltresSupport
        v-model:thematiques="thematiquesSelectionnees"
        v-model:territoire="territoireSelectionne"
        :thematiques-disponibles="thematiquesDisponibles"
        :territoires-disponibles="territoiresDisponibles"
        :nombre-continentales="stationsContinentales"
      />

      <div class="max-w-6xl mx-auto mt-8">
        <div v-if="chargement && !sections.length" class="text-center py-20">
          <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-yellow-400 mx-auto" />
          <p class="text-gray-300 mt-4">Chargement des stations...</p>
        </div>

        <template v-else-if="sections.length">
          <p class="text-gray-400 text-sm mb-2">
            {{ totalStations }} station{{ totalStations > 1 ? 's' : '' }}
          </p>
          <MediaSectionStation
            v-for="section in sections"
            :key="section.station.id"
            :section="section"
          />

          <div v-if="encoreDesStations" class="flex justify-center pt-8">
            <button
              type="button"
              class="rounded-full border border-white/25 text-white text-sm px-6 py-2.5 hover:bg-white/10 transition-colors"
              @click="chargerSections(page + 1)"
            >
              Voir plus de stations
            </button>
          </div>
        </template>

        <!-- État vide explicite (FR-019) : dire ce qui manque, et pourquoi -->
        <div v-else class="text-center py-20">
          <font-awesome-icon :icon="['fas', 'radio']" class="text-4xl text-gray-600 mb-4" />
          <p class="text-gray-300 text-lg">
            {{ filtresActifs ? 'Aucune station ne correspond à ces filtres' : 'Aucune station Africans n’est encore diffusée' }}
          </p>
          <p class="text-gray-500 text-sm mt-2">
            {{ filtresActifs ? 'Essayez d’élargir votre recherche.' : 'Les stations produites par Africans apparaîtront ici.' }}
          </p>
          <button
            v-if="filtresActifs"
            type="button"
            class="mt-5 rounded-full border border-yellow-400 text-yellow-400 text-sm px-5 py-2 hover:bg-yellow-400/15 transition-colors"
            @click="reinitialiserFiltres"
          >
            Réinitialiser les filtres
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
