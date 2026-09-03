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

definePageMeta({ layout: false })

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
/** Genres de grille déclarés par la chaîne (US3, 09s), sélection multiple. */
const thematiquesSelectionnees = ref<string[]>([])
/** Lignes éditoriales d'Africans Télé International (09u), sélection multiple
 * — fusionnée avec `origine` par la barre : en choisir une active
 * `origine=africans`, et relâcher l'origine les vide. */
const rubriquesInternational = ref<string[]>([])

/** Référentiel de FILTRE des 22 genres : un genre encore vide s'affiche
 * « (0) » plutôt que de disparaître, ce qui laisserait croire qu'il n'existe
 * pas. */
const thematiquesDisponibles = ref<ThematiqueDecompte[]>([])
/** Référentiel des 44 lignes éditoriales (09u), même principe de décompte. */
const rubriquesInternationalDisponibles = ref<ThematiqueDecompte[]>([])

const filtresActifs = computed(() =>
  origine.value !== ''
  || paysSelectionne.value !== TOUS_TERRITOIRES
  || enDirect.value
  || thematiquesSelectionnees.value.length > 0
  || rubriquesInternational.value.length > 0,
)

const reinitialiserFiltres = () => {
  origine.value = ''
  paysSelectionne.value = TOUS_TERRITOIRES
  enDirect.value = false
  thematiquesSelectionnees.value = []
  rubriquesInternational.value = []
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
    // Deux référentiels distincts (genres de grille + lignes éditoriales),
    // un seul paramètre côté API : `thematique` est un OU sur l'ensemble des
    // identifiants, quel que soit le panneau d'où ils viennent.
    thematiques: [...thematiquesSelectionnees.value, ...rubriquesInternational.value],
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
watch([origine, paysSelectionne, enDirect, thematiquesSelectionnees, rubriquesInternational], async () => {
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
  const [resultatVedette, resultatPays, resultatThematiques, resultatRubriques] = await Promise.all([
    obtenirVedette(),
    listerPays(),
    // « Africans Thématique » est indépendante de l'origine (pastille
    // séparée de « Africans Télé International ») : le décompte porte donc
    // sur toutes les chaînes publiées, africans et territoire confondues.
    listerThematiquesDisponibles('chaine_tv'),
    // Les 44 lignes éditoriales (09u, groupe dédié) n'ont de sens que pour
    // les chaînes de la plateforme : décompte borné à `origine=africans`.
    listerThematiquesDisponibles('chaine_tv', 'africans', 'media-groupe-africans-tele-international'),
    // Sans incidence sur l'affichage de la vitrine : elle ne doit pas attendre
    // cette réponse, ni échouer avec elle.
    chargerMesChaines(),
  ])
  vedette.value = resultatVedette
  if (resultatPays) territoires.value = resultatPays
  thematiquesDisponibles.value = resultatThematiques
  rubriquesInternationalDisponibles.value = resultatRubriques
  await chargerPageSections(1)
})
</script>

<template>
  <NuxtLayout name="africans">
    <!-- La vedette occupe le slot BANDEAU, qui est pleine largeur comme celui
         des autres modules : c'est la seule façon de garder l'écran d'accueil
         cinématographique de FR-002 sans sortir du gabarit. La barre de
         filtres reste posée dedans, elle en est le pied. -->
    <template #bandeau>
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
            v-model:rubriques-international="rubriquesInternational"
            :territoires="territoires"
            :thematiques-disponibles="thematiquesDisponibles"
            :rubriques-international-disponibles="rubriquesInternationalDisponibles"
            :nombre-chaines="totalChaines"
            @reinitialiser="reinitialiserFiltres"
          />
        </template>
      </MediaVedettePleinEcran>

    </template>

    <template #fil-ariane>
      <AfricansFilAriane :segments="[{ libelle: 'Africamood', vers: '/medias' }, { libelle: 'Télé' }]">
        <template #action>
          <AfricansBouton icone="fa-solid fa-plus" @click="propositionOuverte = true">
            Proposer un contenu
          </AfricansBouton>
        </template>
      </AfricansFilAriane>
    </template>

    <div ref="ancreSections" class="flex flex-col gap-8">
      <div>
        <h2 class="text-[24px]/[1.3] font-bold text-af-encre">Nos télés africaines</h2>
        <p class="mt-1 text-[14px]/[1.5] text-af-corps">
          Une section par chaîne, découverte au fil du défilement.
        </p>
      </div>

      <!-- Actions secondaires : le bouton de proposition principal vit dans le
           fil d'Ariane, comme sur les autres modules. Ne restent ici que ce qui
           explique et ce qui encadre. -->
      <div class="flex flex-wrap gap-3">
        <AfricansBouton variante="secondaire" icone="fa-solid fa-circle-question" @click="presentationOuverte = true">
          C'est quoi Africans Télé ?
        </AfricansBouton>
        <AfricansBouton variante="secondaire" icone="fa-solid fa-shield-halved" @click="reglesOuvertes = true">
          Règles de contenu
        </AfricansBouton>
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
          <div class="animate-spin rounded-full h-12 w-12 text-3xl text-af-chocolat" />
        </div>

        <!-- Aucune chaîne publiée, ou les filtres de la barre ne laissent
             rien passer : les deux cas ne se soignent pas de la même façon, ils
             ne se disent donc pas pareil. Depuis la feature 010, une chaîne sans
             contenu N'EST PLUS filtrée : ce message ne parle donc plus de
             contenu mais de publication. -->
        <div v-else-if="!sections.length" class="text-center py-16">
          <p class="text-[16px]/[1.4] font-bold text-af-encre">
            {{ filtresActifs ? 'Aucune chaîne ne correspond à ces filtres' : 'Aucune chaîne n’est encore publiée' }}
          </p>
          <p class="mt-2 text-[14px]/[1.4] text-af-corps">
            {{ filtresActifs ? 'Essayez d’élargir votre recherche.' : 'Revenez bientôt : les chaînes arrivent.' }}
          </p>
          <button
            v-if="filtresActifs"
            type="button"
            class="mt-5 rounded-lg border border-af-bordure px-5 py-2 text-[14px]/[1.4] font-bold text-af-corps transition hover:border-af-chocolat hover:text-af-chocolat"
            @click="reinitialiserFiltres"
          >
            Réinitialiser les filtres
          </button>
        </div>

      <div ref="sentinelle" class="h-px" />

        <div v-if="encoreDesSections && !chargementSections" class="flex justify-center pt-4">
          <button
            type="button"
            class="rounded-lg border border-af-bordure px-6 py-2.5 text-[14px]/[1.4] font-bold text-af-corps transition hover:border-af-chocolat hover:text-af-chocolat"
            @click="chargerPageSections(page + 1)"
          >
            Voir plus de chaînes
          </button>
        </div>
    </div>
  </NuxtLayout>
</template>
