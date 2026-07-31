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
 *   • Africans Télé International — chaînes produites par la plateforme (09o) ;
 *   • Territoire — référentiel des pays réellement représentés ;
 *   • Chaînes thématiques — thème phare des contenus diffusés ;
 *   • En direct — chaînes actuellement à l'antenne.
 */
import type { ThemePhareAPI } from '~/composables/useMediaProposition'

const props = defineProps<{
  /** '' = toutes origines ; 'africans' = Africans Télé International. */
  origine: string
  /** Nom du territoire, ou 'Tous les territoires'. */
  pays: string
  /** Identifiant du thème phare, '' = tous. */
  theme: string
  enDirect: boolean
  territoires: string[]
  themes: ThemePhareAPI[]
  /** Nombre de chaînes remontées, affiché dès qu'un filtre est actif. */
  nombreChaines?: number
}>()

const emit = defineEmits<{
  'update:origine': [valeur: string]
  'update:pays': [valeur: string]
  'update:theme': [valeur: string]
  'update:enDirect': [valeur: boolean]
  reinitialiser: []
}>()

const TOUS_TERRITOIRES = 'Tous les territoires'

const estAfricans = computed(() => props.origine === 'africans')

const filtresActifs = computed(() =>
  estAfricans.value
  || props.pays !== TOUS_TERRITOIRES
  || props.theme !== ''
  || props.enDirect,
)

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
  </div>
</template>
