<script setup lang="ts">
/**
 * Vedette de la page Télé : occupe toute la surface visible à l'ouverture
 * (FR-002) et démarre d'elle-même le son coupé (FR-003).
 *
 * Le bas de la vedette est laissé au parent (slot `filtres`), qui y monte la
 * barre de filtres : c'est elle qui invite désormais à descendre vers les
 * sections (SC-002).
 */
import type { ProgrammeVedette } from '~/composables/useTelevision'

const props = defineProps<{
  programme: ProgrammeVedette | null
  chargement?: boolean
}>()

const sonCoupe = ref(true)
const enPause = ref(false)
const lecteur = ref<{ lire: () => void, pause: () => void } | null>(null)

const basculerSon = () => {
  sonCoupe.value = !sonCoupe.value
}

const basculerLecture = () => {
  enPause.value = !enPause.value
  if (enPause.value) lecteur.value?.pause()
  else lecteur.value?.lire()
}

/**
 * Un média tiers est joué dans une iframe, que nous ne pilotons pas : proposer
 * nos propres commandes tromperait le visiteur, elles resteraient sans effet.
 *
 * Dans ce cas : et dans ce cas seulement, les contrôles natifs du lecteur
 * d'origine sont affichés. Les masquer AUSSI priverait le visiteur de tout
 * moyen de rétablir le son, que la lecture automatique impose de couper (FR-003).
 */
const commandesDisponibles = computed(() => props.programme?.sourceMedia === 'hebergee')

/** Voir `SectionChaine.vue` : la page de détail arrive au lot suivant. */
const PAGES_DETAIL_DISPONIBLES = false

const lienDetail = computed(() =>
  PAGES_DETAIL_DISPONIBLES && props.programme?.slug
    ? `/medias/programmes-tele/${props.programme.slug}`
    : null,
)
</script>

<template>
  <!-- `100svh` et non `100vh` : sur mobile, `vh` inclut la barre d'URL et fait
       déborder la vedette sous le pli. -->
  <section class="relative h-[100svh] w-full overflow-hidden bg-black">
    <!-- Le lecteur occupe tout le cadre ; `top-24` dégage la NavBar, qui est
         positionnée en absolu et défile avec la page. -->
    <div class="absolute inset-x-0 top-24 bottom-0">
      <MediaLecteurMedia
        v-if="programme"
        ref="lecteur"
        :url="programme.videoUrl"
        type="video"
        :titre="programme.title"
        :poster="programme.banner"
        lecture-auto
        boucle
        :son-coupe="sonCoupe"
        :controles="!commandesDisponibles"
      />
      <div v-else-if="chargement" class="w-full h-full flex items-center justify-center">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-af-orange" />
      </div>
      <!-- État vide explicite : aucun programme publié à mettre en avant. -->
      <div v-else class="w-full h-full flex flex-col items-center justify-center gap-4 px-6 text-center">
        <font-awesome-icon :icon="['fas', 'tv']" class="text-4xl text-white/50" />
        <p class="text-white/80 text-lg font-semibold">Aucun programme n'est encore diffusé</p>
        <p class="text-white/60 text-sm">Les contenus apparaîtront ici dès leur publication.</p>
      </div>
    </div>

    <!-- Voile de lisibilité : le texte doit rester lisible sur toute image. -->
    <div
      v-if="programme"
      class="absolute inset-x-0 bottom-0 h-2/3 bg-linear-to-t from-black via-black/70 to-transparent pointer-events-none"
    />

    <!-- Identité du contenu -->
    <div v-if="programme" class="absolute bottom-24 left-6 sm:left-10 right-6 max-w-2xl text-white">
      <div class="flex flex-wrap items-center gap-3 mb-3">
        <span v-if="programme.chaineNom" class="text-sm uppercase tracking-wide text-af-orange font-semibold">
          {{ programme.chaineNom }}
        </span>
        <span
          v-if="!programme.estRepli"
          class="rounded-full px-3 py-0.5 text-xs bg-af-orange/20 border border-af-orange text-af-orange uppercase"
        >
          À la une
        </span>
        <span v-if="programme.themePhare" class="text-xs text-white/80">{{ programme.themePhare }}</span>
      </div>

      <h1 class="text-3xl sm:text-5xl font-bold uppercase leading-tight mb-3">
        {{ programme.title }}
      </h1>
      <p v-if="programme.description" class="text-white/90 text-sm sm:text-base line-clamp-3 mb-5">
        {{ programme.description }}
      </p>

      <div class="flex flex-wrap items-center gap-3">
        <button
          v-if="commandesDisponibles"
          type="button"
          class="inline-flex items-center gap-2 rounded-full bg-white text-black font-semibold px-6 py-2.5 hover:bg-white/90 transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-af-orange"
          :aria-label="enPause ? 'Reprendre la lecture' : 'Mettre en pause'"
          @click="basculerLecture"
        >
          <font-awesome-icon :icon="['fas', enPause ? 'play' : 'pause']" />
          {{ enPause ? 'Reprendre' : 'Pause' }}
        </button>

        <button
          v-if="commandesDisponibles"
          type="button"
          class="inline-flex items-center justify-center h-11 w-11 rounded-full bg-white/15 hover:bg-white/25 text-white ring-1 ring-white/25 transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-af-orange"
          :aria-label="sonCoupe ? 'Rétablir le son' : 'Couper le son'"
          :aria-pressed="sonCoupe"
          @click="basculerSon"
        >
          <font-awesome-icon :icon="['fas', sonCoupe ? 'volume-xmark' : 'volume-high']" />
        </button>

        <NuxtLink
          v-if="lienDetail"
          :to="lienDetail"
          class="inline-flex items-center gap-2 rounded-full border border-af-orange bg-af-orange/10 text-af-orange px-5 py-2.5 hover:bg-af-orange/20 transition-colors"
        >
          <font-awesome-icon :icon="['fas', 'circle-info']" />
          En savoir plus
        </NuxtLink>
      </div>
    </div>

    <!-- Bas de vedette : la barre de filtres a remplacé le repère de défilement
         animé (SC-002). Elle signale tout aussi bien qu'il y a du contenu sous
         le pli, et donne en plus une prise pour y entrer. -->
    <div class="absolute inset-x-0 bottom-0">
      <slot name="filtres" />
    </div>
  </section>
</template>

<style scoped>
.line-clamp-3 {
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
