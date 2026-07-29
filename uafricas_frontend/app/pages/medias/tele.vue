<script setup lang="ts">
/**
 * Page Télé — vitrine éditorialisée (US1).
 *
 * Une vedette occupe tout l'écran à l'ouverture, puis le défilement révèle une
 * section par chaîne, chargée par pages successives. La grille de vignettes
 * filtrable qui tenait cette page auparavant a disparu : elle donnait à voir un
 * catalogue, non une programmation.
 */
import type { ProgrammeVedette, TeleSection } from '~/composables/useTelevision'
import type { ThemePhareAPI } from '~/composables/useMediaProposition'

const { obtenirVedette, listerSections, listerPays, chargement } = useTelevision()
// Le référentiel des thèmes phares est déjà servi publiquement pour le
// formulaire de proposition : le filtre s'y branche plutôt que d'en dupliquer un.
const { listerThemes } = useMediaProposition()

useHead({
  title: 'Télévision Africaine | AfricanS',
  meta: [
    {
      name: 'description',
      content: 'Regardez les télés africaines : un programme à la une plein écran, puis une section par chaîne avec ses contenus.',
    },
  ],
})

const vedette = ref<ProgrammeVedette | null>(null)
const sections = ref<TeleSection[]>([])

const page = ref(1)
const totalPages = ref(1)
const totalChaines = ref(0)
const chargementSections = ref(false)

// ── Filtres de la barre montée dans la vedette ────────────────────────
const TOUS_TERRITOIRES = 'Tous les territoires'

const territoires = ref<string[]>([])
const themes = ref<ThemePhareAPI[]>([])

const origine = ref('')
const paysSelectionne = ref(TOUS_TERRITOIRES)
const themeSelectionne = ref('')
const enDirect = ref(false)

const filtresActifs = computed(() =>
  origine.value !== ''
  || paysSelectionne.value !== TOUS_TERRITOIRES
  || themeSelectionne.value !== ''
  || enDirect.value,
)

const reinitialiserFiltres = () => {
  origine.value = ''
  paysSelectionne.value = TOUS_TERRITOIRES
  themeSelectionne.value = ''
  enDirect.value = false
}

const presentationOuverte = ref(false)
const reglesOuvertes = ref(false)
const propositionOuverte = ref(false)
const ancreSections = ref<HTMLElement | null>(null)

/**
 * Départage les réponses concurrentes : changer deux fois de filtre coup sur
 * coup lance deux requêtes, et la première à revenir n'est pas forcément celle
 * qui décrit l'état courant des filtres.
 */
let jetonChargement = 0

const chargerPageSections = async (numero: number, forcer = false) => {
  // Le défilement infini n'empile pas les pages ; un changement de filtre, lui,
  // doit passer même si une page est encore en vol.
  if (chargementSections.value && !forcer) return
  const jeton = ++jetonChargement
  chargementSections.value = true
  const resultat = await listerSections({
    origine: origine.value,
    pays: paysSelectionne.value,
    theme: themeSelectionne.value,
    en_direct: enDirect.value,
    page: numero,
    par_page: 6,
  })
  // Réponse dépassée : une requête plus récente fait foi, y compris pour
  // relâcher l'indicateur de chargement.
  if (jeton !== jetonChargement) return

  if (resultat) {
    sections.value = numero === 1 ? resultat.sections : [...sections.value, ...resultat.sections]
    totalPages.value = resultat.totalPages
    totalChaines.value = resultat.total
    page.value = resultat.page
  }
  chargementSections.value = false
}

const encoreDesSections = computed(() => page.value < totalPages.value)

const allerAuxSections = () => {
  ancreSections.value?.scrollIntoView({ behavior: 'smooth', block: 'start' })
}

/**
 * Un filtre relance la liste depuis la première page et amène le visiteur au
 * résultat : la barre siégeant en bas de la vedette, sans ce défilement il
 * agirait à l'aveugle sur des sections qu'il ne voit pas encore.
 */
watch([origine, paysSelectionne, themeSelectionne, enDirect], async () => {
  page.value = 1
  await chargerPageSections(1, true)
  if (filtresActifs.value) allerAuxSections()
})

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
  const [resultatVedette, resultatPays, resultatThemes] = await Promise.all([
    obtenirVedette(),
    listerPays(),
    listerThemes(),
  ])
  vedette.value = resultatVedette
  if (resultatPays) territoires.value = resultatPays
  themes.value = resultatThemes
  await chargerPageSections(1)
})
</script>

<template>
  <div class="min-h-screen bg-gray-900">
    <!-- Vedette plein écran (FR-002), close par la barre de filtres -->
    <MediaVedettePleinEcran
      :programme="vedette"
      :chargement="chargement && !vedette"
    >
      <template #filtres>
        <MediaBarreFiltresTele
          v-model:origine="origine"
          v-model:pays="paysSelectionne"
          v-model:theme="themeSelectionne"
          v-model:en-direct="enDirect"
          :territoires="territoires"
          :themes="themes"
          :nombre-chaines="totalChaines"
          @reinitialiser="reinitialiserFiltres"
        />
      </template>
    </MediaVedettePleinEcran>

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

        <!-- Une section par chaîne, empilées et découvertes au défilement -->
        <MediaSectionChaine
          v-for="section in sections"
          :key="section.chaine.id"
          :section="section"
        />

        <div v-if="chargementSections" class="flex justify-center py-12">
          <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-yellow-400" />
        </div>

        <!-- Aucune chaîne ne porte de contenu publié (FR-008), ou les filtres
             de la barre ne laissent rien passer : les deux cas ne se soignent
             pas de la même façon, ils ne se disent donc pas pareil. -->
        <div v-else-if="!sections.length" class="text-center py-16">
          <p class="text-gray-400 text-lg">
            {{ filtresActifs ? 'Aucune chaîne ne correspond à ces filtres' : 'Aucune chaîne ne diffuse encore de contenu' }}
          </p>
          <p class="text-gray-500 text-sm mt-2">
            {{ filtresActifs ? 'Essayez d’élargir votre recherche.' : 'Revenez bientôt : les programmes arrivent.' }}
          </p>
          <button
            v-if="filtresActifs"
            type="button"
            class="mt-5 rounded-full border border-yellow-400 text-yellow-400 text-sm px-5 py-2 hover:bg-yellow-400/15 transition-colors cursor-pointer"
            @click="reinitialiserFiltres"
          >
            Réinitialiser les filtres
          </button>
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
