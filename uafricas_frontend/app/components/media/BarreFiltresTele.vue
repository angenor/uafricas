<script setup lang="ts">
/**
 * Barre de filtres de la page Télé.
 *
 * Elle occupe le bas de la vedette plein écran, là où se trouvait le repère de
 * défilement animé : le visiteur voit d'emblée par quoi il peut entrer dans le
 * catalogue, et la présence de la barre suffit à signaler qu'il y a du contenu
 * sous le pli.
 *
 * Quatre entrées, toutes résolues côté serveur (`GET /television/sections`) :
 *   • Africans Télé International : chaînes produites par la plateforme (09o),
 *     dont le panneau propose les 44 LIGNES ÉDITORIALES propres à cette ligne
 *     de production (09u — « Retour des cerveaux », « Journal de l'Afrique »…),
 *     et non les genres génériques ci-dessous. Cocher une ligne active
 *     `origine=africans` (elles n'ont pas de sens hors de ce périmètre) ;
 *     relâcher l'origine vide la sélection — les deux sont donc FUSIONNÉS,
 *     comme avant l'introduction d'« Africans Thématique » ;
 *   • Africans Thématique : les 22 GENRES DE GRILLE (09s — Culture, Sport,
 *     Journal télévisé…), INDÉPENDANTE de l'origine ci-dessus — cocher un
 *     genre filtre toutes les chaînes qui le déclarent, « africans » et
 *     « territoire » confondues ;
 *   • Territoire : territoire COUVERT par la chaîne (`support_territoire`), et
 *     non plus son pays de rattachement, qui n'existe plus (09v) : une chaîne
 *     en couvre un, plusieurs, ou tous — une chaîne continentale remonte donc
 *     sous chaque territoire ;
 *   • En direct : chaînes actuellement à l'antenne.
 *
 * Les deux référentiels ne se recouvrent jamais (09u rattache les 44 lignes à
 * un `parent_id` dédié, exclu du référentiel générique de 09s) : une même
 * chaîne peut donc être retrouvée par un genre ET par une ligne éditoriale,
 * sans qu'aucune des deux listes ne s'allonge de l'autre.
 *
 * Deux entrées ont été retirées avant elles parce qu'elles faisaient DOUBLON :
 *   • « Chaînes thématiques » interrogeait le thème phare des PROGRAMMES là où
 *     « Thématiques » interroge ce que la chaîne déclare, deux mécanismes
 *     distincts, mais qui puisent depuis 09s dans le même référentiel de genres
 *     et renvoient donc presque toujours le même résultat. Le déclaré l'emporte :
 *     il est explicite, multiple, et exigé pour publier.
 *   • « Territoire couvert » doublonnait « Territoire » à l'écran.
 * Les paramètres `theme` et `territoire` restent servis par l'API : c'est la
 * barre qui ne les propose plus, pas le backend qui les a perdus.
 */
import type { ThematiqueDecompte } from '~/composables/useMediaSupport'

const props = withDefaults(defineProps<{
  /** '' = toutes origines ; 'africans' = Africans Télé International. */
  origine: string
  /** Identifiant du territoire filtré ; chaîne vide = tous. */
  territoire: string
  enDirect: boolean
  /** Territoires réellement couverts, avec leur décompte de chaînes. */
  territoires: { id: string, nom: string, nombre_supports: number }[]
  /** Genres de grille déclarés (US3, 09s), sélection multiple. */
  thematiques?: string[]
  /** Référentiel des 22 genres, chacun avec son nombre de chaînes publiées :
   * `0` compris, pour donner à voir l'étendue du catalogue. */
  thematiquesDisponibles?: ThematiqueDecompte[]
  /** Lignes éditoriales d'Africans Télé International (09u), sélection
   * multiple — sémantiquement liée à `origine`, jamais choisie hors
   * `origine=africans`. */
  rubriquesInternational?: string[]
  /** Référentiel des 44 lignes éditoriales (09u), même principe de décompte. */
  rubriquesInternationalDisponibles?: ThematiqueDecompte[]
  /** Nombre de chaînes remontées, affiché dès qu'un filtre est actif. */
  nombreChaines?: number
}>(), {
  thematiques: () => [],
  thematiquesDisponibles: () => [],
  rubriquesInternational: () => [],
  rubriquesInternationalDisponibles: () => [],
})

const emit = defineEmits<{
  'update:origine': [valeur: string]
  'update:territoire': [valeur: string]
  'update:enDirect': [valeur: boolean]
  'update:thematiques': [valeur: string[]]
  'update:rubriquesInternational': [valeur: string[]]
  reinitialiser: []
}>()


const estAfricans = computed(() => props.origine === 'africans')

const filtresActifs = computed(() =>
  estAfricans.value
  || props.territoire !== ''
  || props.enDirect
  || props.thematiques.length > 0,
)

/** Panneaux de sélection : une sélection multiple ne tient pas dans un
 * `<select>` natif sans devenir illisible sur mobile. */
const panneauInternational = ref(false)
const panneauThematiques = ref(false)

const basculerThematiques = (valeur: string[]) => emit('update:thematiques', valeur)

/**
 * Panneau des 44 lignes éditoriales : sémantiquement rattaché à l'origine
 * « Africans Télé International », qui n'a de sens que pour ces chaînes-là.
 *
 * • inactive → on l'active et on ouvre le panneau ;
 * • active, panneau fermé → on ouvre le panneau (affiner la sélection) ;
 * • active, panneau ouvert → on relâche l'origine, donc aussi les lignes
 *   éditoriales choisies, qui n'ont pas de sens hors de ce périmètre.
 *
 * Sans ligne à proposer, elle redevient une simple bascule : ouvrir un
 * panneau vide n'apprendrait rien.
 */
const basculerAfricans = () => {
  if (!props.rubriquesInternationalDisponibles.length) {
    emit('update:origine', estAfricans.value ? '' : 'africans')
    return
  }
  if (estAfricans.value && panneauInternational.value) {
    panneauInternational.value = false
    emit('update:origine', '')
    if (props.rubriquesInternational.length) emit('update:rubriquesInternational', [])
    return
  }
  if (!estAfricans.value) emit('update:origine', 'africans')
  panneauInternational.value = true
}

/** Cocher une ligne éditoriale active l'origine si elle ne l'était pas déjà
 * (ex. réouverture du panneau après un « Tout décocher » qui n'a pas fermé
 * la feuille). */
const majRubriquesInternational = (valeur: string[]) => {
  emit('update:rubriquesInternational', valeur)
  if (valeur.length && props.origine !== 'africans') emit('update:origine', 'africans')
}

/** Pastille « Africans Thématique », INDÉPENDANTE de l'origine ci-dessus. */
const basculerPanneauThematiques = () => {
  panneauThematiques.value = !panneauThematiques.value
}
const basculerDirect = () => emit('update:enDirect', !props.enDirect)

const classePastille = (actif: boolean) => [
  'inline-flex shrink-0 items-center gap-2 rounded-full px-4 py-2 text-sm font-semibold whitespace-nowrap transition-colors cursor-pointer',
  'focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-af-orange',
  actif
    ? 'bg-af-orange text-black hover:bg-af-orange'
    : 'bg-white/10 text-white ring-1 ring-white/25 backdrop-blur-xs hover:bg-white/20',
]

/**
 * Les listes déroulantes natives héritent du fond clair : on les force en
 * sombre. La largeur est fixée : sans elle, le sélecteur s'étire à la largeur
 * de son option la plus longue : « Valeurs africaines et développement » y
 * suffit à faire déborder la barre.
 */
const classeSelect = [
  'appearance-none truncate rounded-full bg-white/10 text-white text-sm font-medium ring-1 ring-white/25 backdrop-blur-xs',
  'pl-9 pr-9 py-2 cursor-pointer hover:bg-white/20 transition-colors',
  'focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-af-orange',
]
</script>

<template>
  <div class="w-full bg-linear-to-t from-black via-black/80 to-transparent pt-12 pb-5">
    <!-- Le dégradé seul laissait les pastilles à nu sur l'image : un panneau
         teinté et flouté leur donne un socle commun, qui tient la barre lisible
         quelle que soit la vedette diffusée derrière. Le chocolat de la charte
         plutôt qu'un gris neutre : la barre se lit alors comme un élément de la
         page, non comme un voile posé sur la vidéo. Le jaune de l'anneau est
         celui des filtres actifs, il détache le panneau sur les images sombres. -->
    <div class="max-w-6xl mx-auto px-4">
      <!-- Défilement horizontal sous `sm` : la barre reste sur une ligne et ne
           mange pas la vedette sur les petits écrans. Le fond est porté par le
           conteneur défilant lui-même, il reste donc fixe sous les pastilles
           qui glissent. -->
      <div
        class="flex flex-nowrap sm:flex-wrap items-center justify-start sm:justify-center gap-2 sm:gap-3 overflow-x-auto sm:overflow-visible rounded-2xl bg-linear-to-r from-black/85 via-af-chocolat/45 to-black/85 ring-1 ring-af-orange/35 backdrop-blur-md shadow-lg shadow-black/50 px-3 py-2.5"
      >
        <!-- Africans Télé International (chaînes de la plateforme) : porte
             aussi le panneau de ses 44 lignes éditoriales (09u). -->
        <button
          type="button"
          :class="classePastille(estAfricans)"
          :aria-pressed="estAfricans"
          :aria-haspopup="rubriquesInternationalDisponibles.length ? 'dialog' : undefined"
          :aria-expanded="rubriquesInternationalDisponibles.length ? panneauInternational : undefined"
          @click="basculerAfricans"
        >
          <font-awesome-icon :icon="['fas', 'globe']" class="w-4 h-4" />
          Africans Télé International
          <span v-if="rubriquesInternational.length" class="rounded-full bg-black/20 px-1.5 text-xs">
            {{ rubriquesInternational.length }}
          </span>
          <font-awesome-icon
            v-if="rubriquesInternationalDisponibles.length"
            :icon="['fas', 'chevron-down']"
            class="w-3 h-3 opacity-70"
          />
        </button>

        <!-- Africans Thématique : INDÉPENDANTE de l'origine ci-dessus, filtre
             toutes les chaînes qui déclarent le genre. -->
        <button
          v-if="thematiquesDisponibles.length"
          type="button"
          :class="classePastille(thematiques.length > 0)"
          :aria-pressed="thematiques.length > 0"
          aria-haspopup="dialog"
          :aria-expanded="panneauThematiques"
          @click="basculerPanneauThematiques"
        >
          <font-awesome-icon :icon="['fas', 'tags']" class="w-4 h-4" />
          Africans Thématique
          <span v-if="thematiques.length" class="rounded-full bg-black/20 px-1.5 text-xs">
            {{ thematiques.length }}
          </span>
          <font-awesome-icon :icon="['fas', 'chevron-down']" class="w-3 h-3 opacity-70" />
        </button>

        <!-- Territoire -->
        <div class="relative shrink-0">
          <font-awesome-icon
            :icon="['fas', 'earth-africa']"
            class="pointer-events-none absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4"
            :class="territoire !== '' ? 'text-af-orange' : 'text-white/80'"
          />
          <select
            :value="territoire"
            :class="[...classeSelect, 'w-52', territoire !== '' ? 'ring-af-orange text-af-orange' : '']"
            aria-label="Filtrer par territoire"
            @change="emit('update:territoire', ($event.target as HTMLSelectElement).value)"
          >
            <option class="bg-black/80 text-white" value="">Tous les territoires</option>
            <option v-for="t in territoires" :key="t.id" class="bg-black/80 text-white" :value="t.id">
              {{ t.nom }} ({{ t.nombre_supports }})
            </option>
          </select>
          <font-awesome-icon
            :icon="['fas', 'chevron-down']"
            class="pointer-events-none absolute right-3 top-1/2 -translate-y-1/2 w-3 h-3 text-white/70"
          />
        </div>

        <!-- En direct -->
        <button
          type="button"
          :class="classePastille(enDirect)"
          :aria-pressed="enDirect"
          @click="basculerDirect"
        >
          <span
            class="w-2 h-2 rounded-full"
            :class="enDirect ? 'bg-af-live' : 'bg-af-live animate-pulse'"
          />
          En direct
        </button>

        <!-- Compte-rendu du filtrage : sans lui, un résultat vide ressemble à une
             page cassée plutôt qu'à un filtre trop étroit. -->
        <div v-if="filtresActifs" class="flex shrink-0 items-center gap-2">
          <span v-if="nombreChaines !== undefined" class="text-xs text-white/80 whitespace-nowrap">
            {{ nombreChaines }} chaîne{{ nombreChaines > 1 ? 's' : '' }}
          </span>
          <button
            type="button"
            class="inline-flex shrink-0 items-center gap-1.5 rounded-full border border-white/25 text-white/80 px-3 py-2 text-xs font-medium whitespace-nowrap hover:border-af-orange hover:text-af-orange transition-colors cursor-pointer"
            @click="emit('reinitialiser')"
          >
            <font-awesome-icon :icon="['fas', 'xmark']" class="w-3 h-3" />
            Réinitialiser
          </button>
        </div>
      </div>
    </div>

    <MediaPanneauSelectionThematiques
      :ouvert="panneauInternational"
      titre="Africans Télé International"
      sous-titre="Affiner par ligne éditoriale"
      :items="rubriquesInternationalDisponibles"
      :selection="rubriquesInternational"
      :nombre-resultats="nombreChaines"
      @fermer="panneauInternational = false"
      @update:selection="majRubriquesInternational"
    />

    <MediaPanneauSelectionThematiques
      :ouvert="panneauThematiques"
      titre="Africans Thématique"
      sous-titre="Affiner par thématique, toutes origines confondues"
      :items="thematiquesDisponibles"
      :selection="thematiques"
      :nombre-resultats="nombreChaines"
      @fermer="panneauThematiques = false"
      @update:selection="basculerThematiques"
    />
  </div>
</template>
