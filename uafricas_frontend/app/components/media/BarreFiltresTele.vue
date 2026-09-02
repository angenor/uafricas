<script setup lang="ts">
/**
 * Barre de filtres de la page Télé.
 *
 * Elle occupe le bas de la vedette plein écran, là où se trouvait le repère de
 * défilement animé : le visiteur voit d'emblée par quoi il peut entrer dans le
 * catalogue, et la présence de la barre suffit à signaler qu'il y a du contenu
 * sous le pli.
 *
 * Trois entrées, toutes résolues côté serveur (`GET /television/sections`) :
 *   • Africans Télé International : chaînes produites par la plateforme (09o),
 *     qui PORTE aussi le choix des thématiques déclarées (US3, multiple) ;
 *   • Territoire : pays de rattachement de la chaîne ;
 *   • En direct : chaînes actuellement à l'antenne.
 *
 * Les thématiques n'ont plus de pastille à elles : elles ne qualifient que les
 * chaînes de la plateforme, donc ouvrir leur panneau active `origine=africans`
 * et relâcher cette origine vide la sélection. Deux filtres indépendants
 * laissaient composer « territoire + thématique », combinaison qui ne décrit
 * rien de ce que la page met en avant.
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
  /** Nom du territoire de rattachement, ou 'Tous les territoires'. */
  pays: string
  enDirect: boolean
  territoires: string[]
  /** Thématiques déclarées, sélection multiple (US3). */
  thematiques?: string[]
  /** Référentiel `media` complet, chaque thème avec son nombre de chaînes
   * publiées : `0` compris, pour donner à voir l'étendue du catalogue. */
  thematiquesDisponibles?: ThematiqueDecompte[]
  /** Nombre de chaînes remontées, affiché dès qu'un filtre est actif. */
  nombreChaines?: number
}>(), {
  thematiques: () => [],
  thematiquesDisponibles: () => [],
})

const emit = defineEmits<{
  'update:origine': [valeur: string]
  'update:pays': [valeur: string]
  'update:enDirect': [valeur: boolean]
  'update:thematiques': [valeur: string[]]
  reinitialiser: []
}>()

const TOUS_TERRITOIRES = 'Tous les territoires'

const estAfricans = computed(() => props.origine === 'africans')

const filtresActifs = computed(() =>
  estAfricans.value
  || props.pays !== TOUS_TERRITOIRES
  || props.enDirect
  || props.thematiques.length > 0,
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

/**
 * La pastille Africans porte deux gestes en un : elle active l'origine ET
 * découvre les thématiques de ces chaînes.
 *
 * • inactive → on l'active et on ouvre le panneau ;
 * • active, panneau fermé → on ouvre le panneau (affiner la sélection) ;
 * • active, panneau ouvert → on relâche l'origine, donc aussi les thématiques,
 *   qui n'ont pas de sens hors de ce périmètre.
 *
 * Sans thématique à proposer, elle redevient une simple bascule : ouvrir un
 * panneau vide n'apprendrait rien.
 */
const basculerAfricans = () => {
  if (!props.thematiquesDisponibles.length) {
    emit('update:origine', estAfricans.value ? '' : 'africans')
    return
  }
  if (estAfricans.value && panneauThematiques.value) {
    panneauThematiques.value = false
    emit('update:origine', '')
    if (props.thematiques.length) emit('update:thematiques', [])
    return
  }
  if (!estAfricans.value) emit('update:origine', 'africans')
  panneauThematiques.value = true
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
        <!-- Africans Télé International (chaînes de la plateforme) : porte aussi
             le panneau des thématiques déclarées. Le panneau lui-même est
             TÉLÉPORTÉ hors de la barre : voir plus bas. -->
        <button
          type="button"
          :class="classePastille(estAfricans)"
          :aria-pressed="estAfricans"
          :aria-haspopup="thematiquesDisponibles.length ? 'dialog' : undefined"
          :aria-expanded="thematiquesDisponibles.length ? panneauThematiques : undefined"
          @click="basculerAfricans"
        >
          <font-awesome-icon :icon="['fas', 'globe']" class="w-4 h-4" />
          Africans Télé International
          <span v-if="thematiques.length" class="rounded-full bg-black/20 px-1.5 text-xs">
            {{ thematiques.length }}
          </span>
          <font-awesome-icon
            v-if="thematiquesDisponibles.length"
            :icon="['fas', 'chevron-down']"
            class="w-3 h-3 opacity-70"
          />
        </button>

        <!-- Territoire -->
        <div class="relative shrink-0">
          <font-awesome-icon
            :icon="['fas', 'earth-africa']"
            class="pointer-events-none absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4"
            :class="pays !== TOUS_TERRITOIRES ? 'text-af-orange' : 'text-white/80'"
          />
          <select
            :value="pays"
            :class="[...classeSelect, 'w-52', pays !== TOUS_TERRITOIRES ? 'ring-af-orange text-af-orange' : '']"
            aria-label="Filtrer par territoire"
            @change="emit('update:pays', ($event.target as HTMLSelectElement).value)"
          >
            <option class="bg-black/80 text-white" :value="TOUS_TERRITOIRES">Tous les territoires</option>
            <option v-for="t in territoires" :key="t" class="bg-black/80 text-white" :value="t">{{ t }}</option>
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

    <!--
      Panneau des thématiques d'Africans Télé International, TÉLÉPORTÉ dans
      `<body>`.

      Il ne peut pas vivre dans la barre : celle-ci est collée au bas d'une
      vedette qui occupe tout l'écran, donc un panneau ouvert vers le bas sort
      du champ ; et la rangée de pastilles porte `overflow-x-auto`, qui le
      rogne. Un simple `fixed` ne suffirait pas non plus, le `backdrop-blur`
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
          aria-label="Africans Télé International, filtrer par thématique"
          class="absolute inset-x-3 bottom-3 sm:left-1/2 sm:right-auto sm:-translate-x-1/2 sm:w-[36rem] max-h-[65vh] flex flex-col rounded-2xl bg-black/80 ring-1 ring-white/15 shadow-2xl"
        >
          <div class="flex items-center justify-between gap-3 px-4 pt-4 pb-3 border-b border-white/10">
            <div class="min-w-0">
              <p class="text-white font-semibold truncate">
                Africans Télé International
                <span v-if="thematiques.length" class="text-af-orange">({{ thematiques.length }})</span>
              </p>
              <p class="text-xs text-white/70">Affiner par thématique</p>
            </div>
            <div class="flex items-center gap-3">
              <button
                v-if="thematiques.length"
                type="button"
                class="text-xs text-white/70 underline hover:text-white"
                @click="emit('update:thematiques', [])"
              >
                Tout décocher
              </button>
              <button
                type="button"
                class="text-white/70 hover:text-white"
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
            <!-- Les thèmes sans chaîne restent proposés et cliquables : leur
                 `(0)` est une information : il dit que le thème existe et
                 n'attend qu'un contenu, là où les masquer laisserait croire
                 à un catalogue plus étroit qu'il n'est. Ils sont simplement
                 estompés pour ne pas concurrencer les thèmes servis. -->
            <button
              v-for="t in thematiquesDisponibles"
              :key="t.id"
              type="button"
              class="rounded-full border px-3 py-1.5 text-xs transition-colors"
              :class="thematiques.includes(t.id)
                ? 'bg-af-orange border-af-orange text-black font-semibold'
                : t.nombre_supports > 0
                  ? 'bg-white/5 border-white/15 text-white/80 hover:border-af-orange'
                  : 'bg-transparent border-white/10 text-white/60 hover:border-af-orange/60 hover:text-white/80'"
              @click="basculerThematique(t.id)"
            >
              {{ t.nom }} ({{ t.nombre_supports }})
            </button>
          </div>

          <div class="px-4 pb-4 pt-1 border-t border-white/10">
            <button
              type="button"
              class="w-full rounded-full bg-af-orange text-black font-semibold py-2.5 text-sm hover:bg-af-orange transition-colors"
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
