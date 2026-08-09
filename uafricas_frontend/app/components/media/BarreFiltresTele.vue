<script setup lang="ts">
/**
 * Barre de filtres de la page Télé.
 *
 * Elle occupe le bas de la vedette plein écran, là où se trouvait le repère de
 * défilement animé : le visiteur voit d'emblée par quoi il peut entrer dans le
 * catalogue, et la présence de la barre suffit à signaler qu'il y a du contenu
 * sous le pli.
 *
 * Six entrées, toutes résolues côté serveur (`GET /television/sections`) :
 *   • Africans Télé International — chaînes produites par la plateforme (09o) ;
 *   • Territoire (siège) — référentiel des pays de rattachement ;
 *   • Territoire couvert — couverture déclarée (US4) : une chaîne panafricaine
 *     remonte sur **chaque** territoire, ce que le siège ne dit pas (FR-036) ;
 *   • Thématiques — thèmes DÉCLARÉS par la chaîne, sélection multiple (US3) ;
 *   • Chaînes thématiques — thème phare des contenus diffusés (à ne pas
 *     confondre avec le précédent : celui-ci porte sur les programmes) ;
 *   • En direct — chaînes actuellement à l'antenne.
 */
import type { ThemePhareAPI } from '~/composables/useMediaProposition'
import type { ThematiqueDecompte, TerritoireDecompte } from '~/composables/useMediaSupport'

const props = withDefaults(defineProps<{
  /** '' = toutes origines ; 'africans' = Africans Télé International. */
  origine: string
  /** Nom du territoire de rattachement, ou 'Tous les territoires'. */
  pays: string
  /** Identifiant du thème phare, '' = tous. */
  theme: string
  enDirect: boolean
  territoires: string[]
  themes: ThemePhareAPI[]
  /** Thématiques déclarées, sélection multiple (US3). */
  thematiques?: string[]
  /** Identifiant du territoire couvert, '' = tous (US4). */
  territoire?: string
  /** Référentiels des thématiques et territoires réellement déclarés. */
  thematiquesDisponibles?: ThematiqueDecompte[]
  territoiresDisponibles?: TerritoireDecompte[]
  /** Nombre de chaînes remontées, affiché dès qu'un filtre est actif. */
  nombreChaines?: number
}>(), {
  thematiques: () => [],
  territoire: '',
  thematiquesDisponibles: () => [],
  territoiresDisponibles: () => [],
})

const emit = defineEmits<{
  'update:origine': [valeur: string]
  'update:pays': [valeur: string]
  'update:theme': [valeur: string]
  'update:enDirect': [valeur: boolean]
  'update:thematiques': [valeur: string[]]
  'update:territoire': [valeur: string]
  reinitialiser: []
}>()

const TOUS_TERRITOIRES = 'Tous les territoires'

const estAfricans = computed(() => props.origine === 'africans')

const filtresActifs = computed(() =>
  estAfricans.value
  || props.pays !== TOUS_TERRITOIRES
  || props.theme !== ''
  || props.enDirect
  || props.thematiques.length > 0
  || props.territoire !== '',
)

/** Panneau des thématiques : une sélection multiple ne tient pas dans un
 * `<select>` natif sans devenir illisible sur mobile. */
const panneauThematiques = ref(false)

/**
 * Échap ferme la feuille. Elle recouvre le bas de l'écran et n'a pas d'autre
 * sortie au clavier ; sans cela, une navigation sans souris s'y trouve coincée.
 * L'écouteur n'existe que pendant l'ouverture.
 */
const fermerSurEchap = (evenement: KeyboardEvent) => {
  if (evenement.key === 'Escape') panneauThematiques.value = false
}

watch(panneauThematiques, (ouvert) => {
  if (!import.meta.client) return
  if (ouvert) window.addEventListener('keydown', fermerSurEchap)
  else window.removeEventListener('keydown', fermerSurEchap)
})

onBeforeUnmount(() => {
  if (import.meta.client) window.removeEventListener('keydown', fermerSurEchap)
})

const basculerThematique = (id: string) => {
  emit(
    'update:thematiques',
    props.thematiques.includes(id)
      ? props.thematiques.filter(x => x !== id)
      : [...props.thematiques, id],
  )
}

/** Un second clic sur la pastille active la relâche : elle vaut bascule. */
const basculerAfricans = () => emit('update:origine', estAfricans.value ? '' : 'africans')
const basculerDirect = () => emit('update:enDirect', !props.enDirect)

const classePastille = (actif: boolean) => [
  'inline-flex shrink-0 items-center gap-2 rounded-full px-4 py-2 text-sm font-semibold whitespace-nowrap transition-colors cursor-pointer',
  'focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-yellow-400',
  actif
    ? 'bg-yellow-400 text-black hover:bg-yellow-300'
    : 'bg-white/10 text-white ring-1 ring-white/25 backdrop-blur-xs hover:bg-white/20',
]

/**
 * Les listes déroulantes natives héritent du fond clair : on les force en
 * sombre. La largeur est fixée : sans elle, le sélecteur s'étire à la largeur
 * de son option la plus longue — « Valeurs africaines et développement » y
 * suffit à faire déborder la barre.
 */
const classeSelect = [
  'appearance-none truncate rounded-full bg-white/10 text-white text-sm font-medium ring-1 ring-white/25 backdrop-blur-xs',
  'pl-9 pr-9 py-2 cursor-pointer hover:bg-white/20 transition-colors',
  'focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-yellow-400',
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
        class="flex flex-nowrap sm:flex-wrap items-center justify-start sm:justify-center gap-2 sm:gap-3 overflow-x-auto sm:overflow-visible rounded-2xl bg-linear-to-r from-black/85 via-custom-chocolat/45 to-black/85 ring-1 ring-yellow-400/35 backdrop-blur-md shadow-lg shadow-black/50 px-3 py-2.5"
      >
        <!-- Africans Télé International (FR — chaînes de la plateforme) -->
        <button
          type="button"
          :class="classePastille(estAfricans)"
          :aria-pressed="estAfricans"
          @click="basculerAfricans"
        >
          <font-awesome-icon :icon="['fas', 'globe']" class="w-4 h-4" />
          Africans Télé International
        </button>

        <!-- Territoire -->
        <div class="relative shrink-0">
          <font-awesome-icon
            :icon="['fas', 'earth-africa']"
            class="pointer-events-none absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4"
            :class="pays !== TOUS_TERRITOIRES ? 'text-yellow-400' : 'text-gray-300'"
          />
          <select
            :value="pays"
            :class="[...classeSelect, 'w-52', pays !== TOUS_TERRITOIRES ? 'ring-yellow-400 text-yellow-400' : '']"
            aria-label="Filtrer par territoire"
            @change="emit('update:pays', ($event.target as HTMLSelectElement).value)"
          >
            <option class="bg-gray-900 text-white" :value="TOUS_TERRITOIRES">Tous les territoires</option>
            <option v-for="t in territoires" :key="t" class="bg-gray-900 text-white" :value="t">{{ t }}</option>
          </select>
          <font-awesome-icon
            :icon="['fas', 'chevron-down']"
            class="pointer-events-none absolute right-3 top-1/2 -translate-y-1/2 w-3 h-3 text-gray-400"
          />
        </div>

        <!-- Chaînes thématiques -->
        <div class="relative shrink-0">
          <font-awesome-icon
            :icon="['fas', 'layer-group']"
            class="pointer-events-none absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4"
            :class="theme ? 'text-yellow-400' : 'text-gray-300'"
          />
          <select
            :value="theme"
            :class="[...classeSelect, 'w-60', theme ? 'ring-yellow-400 text-yellow-400' : '']"
            aria-label="Filtrer par thème"
            @change="emit('update:theme', ($event.target as HTMLSelectElement).value)"
          >
            <option class="bg-gray-900 text-white" value="">Chaînes thématiques</option>
            <option v-for="t in themes" :key="t.id" class="bg-gray-900 text-white" :value="t.id">{{ t.nom }}</option>
          </select>
          <font-awesome-icon
            :icon="['fas', 'chevron-down']"
            class="pointer-events-none absolute right-3 top-1/2 -translate-y-1/2 w-3 h-3 text-gray-400"
          />
        </div>

        <!-- Territoire couvert (US4) : distinct du siège ci-dessus — une chaîne
             panafricaine remonte ici sur chaque territoire. -->
        <div class="relative shrink-0">
          <font-awesome-icon
            :icon="['fas', 'globe']"
            class="pointer-events-none absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4"
            :class="territoire ? 'text-yellow-400' : 'text-gray-300'"
          />
          <select
            :value="territoire"
            :class="[...classeSelect, 'w-56', territoire ? 'ring-yellow-400 text-yellow-400' : '']"
            aria-label="Filtrer par territoire couvert"
            @change="emit('update:territoire', ($event.target as HTMLSelectElement).value)"
          >
            <option class="bg-gray-900 text-white" value="">Territoire couvert</option>
            <option
              v-for="t in territoiresDisponibles"
              :key="t.id"
              class="bg-gray-900 text-white"
              :value="t.id"
            >
              {{ t.nom }} ({{ t.nombre_supports }})
            </option>
          </select>
          <font-awesome-icon
            :icon="['fas', 'chevron-down']"
            class="pointer-events-none absolute right-3 top-1/2 -translate-y-1/2 w-3 h-3 text-gray-400"
          />
        </div>

        <!-- Thématiques déclarées (US3), sélection multiple.
             Le panneau lui-même est TÉLÉPORTÉ hors de la barre : voir plus bas. -->
        <button
          v-if="thematiquesDisponibles.length"
          type="button"
          :class="classePastille(thematiques.length > 0)"
          aria-haspopup="dialog"
          :aria-expanded="panneauThematiques"
          @click="panneauThematiques = !panneauThematiques"
        >
          <font-awesome-icon :icon="['fas', 'tags']" class="w-4 h-4" />
          Thématiques
          <span v-if="thematiques.length" class="rounded-full bg-black/20 px-1.5 text-xs">
            {{ thematiques.length }}
          </span>
        </button>

        <!-- En direct -->
        <button
          type="button"
          :class="classePastille(enDirect)"
          :aria-pressed="enDirect"
          @click="basculerDirect"
        >
          <span
            class="w-2 h-2 rounded-full"
            :class="enDirect ? 'bg-red-600' : 'bg-red-500 animate-pulse'"
          />
          En direct
        </button>

        <!-- Compte-rendu du filtrage : sans lui, un résultat vide ressemble à une
             page cassée plutôt qu'à un filtre trop étroit. -->
        <div v-if="filtresActifs" class="flex shrink-0 items-center gap-2">
          <span v-if="nombreChaines !== undefined" class="text-xs text-gray-300 whitespace-nowrap">
            {{ nombreChaines }} chaîne{{ nombreChaines > 1 ? 's' : '' }}
          </span>
          <button
            type="button"
            class="inline-flex shrink-0 items-center gap-1.5 rounded-full border border-white/25 text-gray-300 px-3 py-2 text-xs font-medium whitespace-nowrap hover:border-yellow-400 hover:text-yellow-400 transition-colors cursor-pointer"
            @click="emit('reinitialiser')"
          >
            <font-awesome-icon :icon="['fas', 'xmark']" class="w-3 h-3" />
            Réinitialiser
          </button>
        </div>
      </div>
    </div>

    <!--
      Panneau des thématiques, TÉLÉPORTÉ dans `<body>`.

      Il ne peut pas vivre dans la barre : celle-ci est collée au bas d'une
      vedette qui occupe tout l'écran, donc un panneau ouvert vers le bas sort
      du champ ; et la rangée de pastilles porte `overflow-x-auto`, qui le
      rogne. Un simple `fixed` ne suffirait pas non plus — le `backdrop-blur`
      de la barre crée un bloc conteneur qui capture jusqu'aux éléments fixes.
      Sortir du sous-arbre est le seul remède qui tienne dans les trois cas.

      La forme retenue est une feuille ancrée en bas : c'est là que se trouve
      le bouton, et c'est ce qui reste atteignable au pouce sur mobile.
    -->
    <Teleport to="body">
      <div v-if="panneauThematiques" class="fixed inset-0 z-60">
        <div class="absolute inset-0 bg-black/60" @click="panneauThematiques = false" />

        <div
          role="dialog"
          aria-label="Filtrer par thématique"
          class="absolute inset-x-3 bottom-3 sm:left-1/2 sm:right-auto sm:-translate-x-1/2 sm:w-[36rem] max-h-[65vh] flex flex-col rounded-2xl bg-neutral-900 ring-1 ring-white/15 shadow-2xl"
        >
          <div class="flex items-center justify-between gap-3 px-4 pt-4 pb-3 border-b border-white/10">
            <p class="text-white font-semibold">
              Thématiques
              <span v-if="thematiques.length" class="text-yellow-400">({{ thematiques.length }})</span>
            </p>
            <div class="flex items-center gap-3">
              <button
                v-if="thematiques.length"
                type="button"
                class="text-xs text-gray-400 underline hover:text-white"
                @click="emit('update:thematiques', [])"
              >
                Tout décocher
              </button>
              <button
                type="button"
                class="text-gray-400 hover:text-white"
                aria-label="Fermer"
                @click="panneauThematiques = false"
              >
                <font-awesome-icon :icon="['fas', 'xmark']" class="w-4 h-4" />
              </button>
            </div>
          </div>

          <!-- Le défilement vit ici, pas sur la feuille : l'en-tête et le pied
               doivent rester visibles quand la liste est longue. -->
          <div class="flex flex-wrap gap-2 overflow-y-auto px-4 py-4">
            <button
              v-for="t in thematiquesDisponibles"
              :key="t.id"
              type="button"
              class="rounded-full border px-3 py-1.5 text-xs transition-colors"
              :class="thematiques.includes(t.id)
                ? 'bg-yellow-400 border-yellow-400 text-neutral-900 font-semibold'
                : 'bg-white/5 border-white/15 text-gray-300 hover:border-yellow-400'"
              @click="basculerThematique(t.id)"
            >
              {{ t.nom }} ({{ t.nombre_supports }})
            </button>
          </div>

          <div class="px-4 pb-4 pt-1 border-t border-white/10">
            <button
              type="button"
              class="w-full rounded-full bg-yellow-400 text-neutral-900 font-semibold py-2.5 text-sm hover:bg-yellow-300 transition-colors"
              @click="panneauThematiques = false"
            >
              Voir les résultats
              <span v-if="nombreChaines !== undefined">({{ nombreChaines }})</span>
            </button>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>
