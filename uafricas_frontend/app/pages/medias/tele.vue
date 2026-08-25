<script setup lang="ts">
/**
 * Page Télé : vitrine éditorialisée (US1).
 *
 * Une vedette occupe tout l'écran à l'ouverture, puis le défilement révèle une
 * section par chaîne, chargée par pages successives. La grille de vignettes
 * filtrable qui tenait cette page auparavant a disparu : elle donnait à voir un
 * catalogue, non une programmation.
 */
import type { ProgrammeVedette, TeleSection } from '~/composables/useTelevision'
import type { RoleDetenteur } from '~/composables/useMediaDetention'
import type { ThematiqueDecompte } from '~/composables/useMediaSupport'

const { obtenirVedette, listerSections, listerPays, chargement } = useTelevision()
// Référentiel de filtre US3 : tous les thèmes actifs, avec leur décompte.
const { listerThematiquesDisponibles } = useMediaSupport()
const { mesSupports } = useMediaDetention()
const userStore = useUserStore()

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

/**
 * Rôle de détention de l'utilisateur sur les chaînes qu'il détient, indexé par
 * identifiant de chaîne.
 *
 * Un seul appel au montage, plutôt qu'une interrogation par section : la liste
 * de ses propres supports est courte, et les sections arrivent par pages
 * successives. C'est ce qui laisse `SectionChaine` révéler l'accès à la gestion
 * sur les seules chaînes qui lui appartiennent, le serveur restant seul juge
 * des droits réels (`garde_detenteur`).
 */
const rolesParChaine = ref<Record<string, RoleDetenteur>>({})

const chargerMesChaines = async () => {
  if (!userStore.accessToken) return
  const supports = await mesSupports()
  rolesParChaine.value = Object.fromEntries(
    supports
      .filter(s => s.type_support === 'chaine_tv')
      .map(s => [s.support_id, s.role]),
  )
}

const page = ref(1)
const totalPages = ref(1)
const totalChaines = ref(0)
const chargementSections = ref(false)

// ── Filtres de la barre montée dans la vedette ────────────────────────
const TOUS_TERRITOIRES = 'Tous les territoires'

const territoires = ref<string[]>([])

const origine = ref('')
const paysSelectionne = ref(TOUS_TERRITOIRES)
const enDirect = ref(false)
/** Thématiques déclarées par la chaîne (US3), sélection multiple. */
const thematiquesSelectionnees = ref<string[]>([])

/** Référentiel de FILTRE : tout le catalogue de thèmes, chacun avec son nombre
 * de chaînes publiées : un thème encore vide s'affiche « (0) » plutôt que de
 * disparaître, ce qui laisserait croire qu'il n'existe pas. */
const thematiquesDisponibles = ref<ThematiqueDecompte[]>([])

const filtresActifs = computed(() =>
  origine.value !== ''
  || paysSelectionne.value !== TOUS_TERRITOIRES
  || enDirect.value
  || thematiquesSelectionnees.value.length > 0,
)

const reinitialiserFiltres = () => {
  origine.value = ''
  paysSelectionne.value = TOUS_TERRITOIRES
  enDirect.value = false
  thematiquesSelectionnees.value = []
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
    en_direct: enDirect.value,
    thematiques: thematiquesSelectionnees.value,
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
watch([origine, paysSelectionne, enDirect, thematiquesSelectionnees], async () => {
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
  const [resultatVedette, resultatPays, resultatThematiques] = await Promise.all([
    obtenirVedette(),
    listerPays(),
    listerThematiquesDisponibles('chaine_tv'),
    // Sans incidence sur l'affichage de la vitrine : elle ne doit pas attendre
    // cette réponse, ni échouer avec elle.
    chargerMesChaines(),
  ])
  vedette.value = resultatVedette
  if (resultatPays) territoires.value = resultatPays
  thematiquesDisponibles.value = resultatThematiques
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
          v-model:en-direct="enDirect"
          v-model:thematiques="thematiquesSelectionnees"
          :territoires="territoires"
          :thematiques-disponibles="thematiquesDisponibles"
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

        <MediaTeleDecouverteModale v-model="presentationOuverte" />

        <!-- Contenus interdits et conséquences d'un signalement (FR-048). -->
        <MediaReglesContenuModal
          :open="reglesOuvertes"
          @close="reglesOuvertes = false"
        />

        <!-- Toute proposition part en attente de validation (FR-031). -->
        <MediaProposerMediaModal
          :is-open="propositionOuverte"
          :types-offerts="['chaine_tv', 'emission_tele']"
          @close="propositionOuverte = false"
        />

        <!-- Une section par chaîne, empilées et découvertes au défilement -->
        <MediaSectionChaine
          v-for="section in sections"
          :key="section.chaine.id"
          :section="section"
          :mon-role="rolesParChaine[section.chaine.id] ?? null"
        />

        <div v-if="chargementSections" class="flex justify-center py-12">
          <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-yellow-400" />
        </div>

        <!-- Aucune chaîne publiée, ou les filtres de la barre ne laissent
             rien passer : les deux cas ne se soignent pas de la même façon, ils
             ne se disent donc pas pareil. Depuis la feature 010, une chaîne sans
             contenu N'EST PLUS filtrée : ce message ne parle donc plus de
             contenu mais de publication. -->
        <div v-else-if="!sections.length" class="text-center py-16">
          <p class="text-gray-400 text-lg">
            {{ filtresActifs ? 'Aucune chaîne ne correspond à ces filtres' : 'Aucune chaîne n’est encore publiée' }}
          </p>
          <p class="text-gray-500 text-sm mt-2">
            {{ filtresActifs ? 'Essayez d’élargir votre recherche.' : 'Revenez bientôt : les chaînes arrivent.' }}
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
