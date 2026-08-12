<script setup lang="ts">
/**
 * Section d'une station sur les pages Radio — refondue par la feature 010
 * (FR-060 : parité stricte avec la section télé, « station » et « audio »
 * substitués).
 *
 * La vitrine annonce une **offre éditoriale** : identité → extrait de
 * description → équipe → bandeau de programmation → cartes de programme
 * (FR-001). L'épisode mis en avant, ses vignettes d'écoute et sa barre de
 * réactions ont disparu (FR-002) ; ils vivent sur les pages de détail.
 *
 * **Le direct reste**, et c'est délibéré : « écouter le direct » n'est pas la
 * lecture d'un enregistrement mais l'accès au flux de la station, au même titre
 * que sa page. Il est confié au lecteur persistant du layout, qui survit au
 * défilement et à la navigation (FR-017).
 *
 * Tailwind v4 pur (Principe VI).
 */
import type { StationSection } from '~/composables/useStationsRadio'

const props = defineProps<{ section: StationSection }>()

const { lire, estContenuCourant, enLecture } = useLecteurMedia()

const lienStation = computed(() =>
  props.section.station.slug ? `/medias/stations/${props.section.station.slug}` : null,
)

/**
 * **Tous** les programmes servis sont affichés — y compris ceux sans épisode
 * publié (FR-005). Le filtre `episodes.length > 0` qui régnait ici n'avait de
 * sens que tant que la section montrait des enregistrements.
 */
const programmes = computed(() => props.section.emissions)

/** Au-delà du plafond de section, le total est annoncé (FR-008). */
const programmesMasques = computed(() =>
  Math.max(0, props.section.totalEmissions - programmes.value.length),
)

/** Le direct est un contenu comme un autre : il se lance de la même façon. */
const lireDirect = () => {
  lire({
    id: `direct-${props.section.station.id}`,
    type: 'station_radio',
    titre: `${props.section.station.name} — en direct`,
    support: props.section.station.name,
    supportSlug: props.section.station.slug,
    url: props.section.station.streamUrl,
    image: props.section.station.cover,
    estDirect: true,
  })
}

const directEnCours = computed(
  () => estContenuCourant(`direct-${props.section.station.id}`) && enLecture.value,
)

const { redirigerVersConnexion } = useAuth()
const userStore = useUserStore()

const showPartage = ref(false)
const showIdee = ref(false)
const showAnimation = ref(false)

// Voir `SectionChaine.vue` : une fonction d'ouverture, et non le ref lui-même —
// Vue déballe les refs dans un template.
const ouvrirSiConnecte = (ouvrir: () => void) => {
  if (!userStore.accessToken) {
    redirigerVersConnexion()
    return
  }
  ouvrir()
}
</script>

<template>
  <section class="border-t border-white/10 py-10">
    <!-- Identité de la station -->
    <header class="mb-4 flex items-center gap-4 px-1">
      <img
        v-if="section.station.cover"
        :src="section.station.cover"
        :alt="section.station.name"
        loading="lazy"
        class="h-14 w-14 shrink-0 rounded-lg object-cover"
      >
      <span
        v-else
        class="flex h-14 w-14 shrink-0 items-center justify-center rounded-lg bg-neutral-800"
      >
        <font-awesome-icon :icon="['fas', 'radio']" class="text-neutral-600" />
      </span>
      <div class="min-w-0 flex-1">
        <NuxtLink
          v-if="lienStation"
          :to="lienStation"
          class="block truncate text-xl font-bold text-white transition-colors hover:text-custom-chocolat sm:text-2xl"
        >
          {{ section.station.name }}
        </NuxtLink>
        <h2 v-else class="block truncate text-xl font-bold text-white sm:text-2xl">
          {{ section.station.name }}
        </h2>
        <p class="truncate text-sm text-gray-400">
          <span v-if="section.station.location">{{ section.station.location }}</span>
          <span v-if="section.station.location && section.station.genre"> · </span>
          <span v-if="section.station.genre">{{ section.station.genre }}</span>
        </p>
      </div>

      <div class="flex shrink-0 items-center gap-3">
        <span class="hidden text-xs text-gray-500 sm:block">
          {{ section.totalEmissions }} programme{{ section.totalEmissions > 1 ? 's' : '' }}
        </span>

        <!-- Le direct, offert au même rang que la page de la station (FR-016) -->
        <button
          v-if="section.directDisponible"
          type="button"
          class="inline-flex shrink-0 items-center gap-2 rounded-full px-4 py-2 text-sm font-semibold transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-custom-chocolat"
          :class="directEnCours
            ? 'bg-red-600 text-white'
            : 'border border-red-500 text-red-400 hover:bg-red-500/15'"
          @click="lireDirect"
        >
          <font-awesome-icon :icon="['fas', directEnCours ? 'volume-high' : 'play']" />
          <span class="hidden sm:inline">{{ directEnCours ? 'En direct' : 'Écouter le direct' }}</span>
        </button>
      </div>
    </header>

    <!-- Extrait de description : ellipse figée, sans dépliage (FR-003). -->
    <CommonTexteRepliable
      v-if="section.station.description"
      :texte="section.station.description"
      :lignes="3"
      :repliable="false"
      sombre
      class="mb-4 max-w-4xl px-1 text-sm text-gray-300"
    />

    <!-- Équipe éditoriale : aucun cadre quand la station n'en déclare pas. -->
    <MediaEquipeMedia
      :membres="section.station.equipe"
      :seuil="0"
      sombre
      class="mb-6 px-1"
    />

    <!-- Ce que la grille programme à cet instant (US5, FR-039) — texte seul. -->
    <MediaBandeauDiffusion
      :en-cours="section.diffusionEnCours"
      :suivant="section.creneauSuivant"
      base-lien-contenu="programmes-radio"
    />

    <!-- Les programmes de la station (FR-004) -->
    <div
      v-if="programmes.length"
      class="mt-6 grid grid-cols-2 gap-4 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5"
    >
      <MediaCarteProgramme
        v-for="programme in programmes"
        :key="programme.id"
        :programme="programme"
        type-support="station_radio"
      />
    </div>
    <p v-else class="mt-6 px-1 text-sm text-gray-500">
      Cette station n'a pas encore annoncé de programme.
    </p>

    <!-- Jamais de disparition silencieuse (FR-008). -->
    <NuxtLink
      v-if="programmesMasques > 0 && lienStation"
      :to="lienStation"
      class="mt-4 inline-flex items-center gap-2 px-1 text-sm font-medium text-custom-chocolat underline underline-offset-2 transition-colors hover:text-white"
    >
      Voir les {{ section.totalEmissions }} programmes
      <font-awesome-icon :icon="['fas', 'arrow-right']" class="h-3 w-3" />
    </NuxtLink>

    <!-- Réagir et partager LA STATION : la cible a changé avec le retrait de
         l'épisode mis en avant. -->
    <MediaReactionsBar
      compact
      class="mt-6 px-1"
      type-media="station_radio"
      :media-id="section.station.id"
      :nombre-likes="section.station.interactions?.nombre_likes ?? 0"
      :nombre-dislikes="section.station.interactions?.nombre_dislikes ?? 0"
      :ma-reaction="section.station.interactions?.ma_reaction ?? null"
      :nombre-commentaires="section.station.interactions?.nombre_commentaires ?? 0"
      :nombre-partages="section.station.interactions?.nombre_partages ?? 0"
      @require-login="redirigerVersConnexion()"
      @commenter="lienStation && navigateTo(lienStation)"
      @partager="showPartage = true"
    />

    <!-- S'engager auprès de la station (US6, FR-044, FR-045) -->
    <div class="mt-4 flex flex-wrap gap-3 px-1">
      <button
        type="button"
        class="inline-flex items-center gap-2 rounded-full border border-white/20 px-4 py-2 text-sm text-gray-300 transition-colors hover:border-custom-chocolat hover:text-custom-chocolat"
        @click="ouvrirSiConnecte(() => showIdee = true)"
      >
        <font-awesome-icon :icon="['fas', 'lightbulb']" />
        Proposer une idée
      </button>
      <button
        type="button"
        class="inline-flex items-center gap-2 rounded-full border border-white/20 px-4 py-2 text-sm text-gray-300 transition-colors hover:border-custom-chocolat hover:text-custom-chocolat"
        @click="ouvrirSiConnecte(() => showAnimation = true)"
      >
        <font-awesome-icon :icon="['fas', 'microphone']" />
        Demander à animer
      </button>
      <MediaSignalerBouton
        type-media="station_radio"
        :media-id="section.station.id"
        :titre="section.station.name"
        variante="pilule"
      />
    </div>

    <MediaPartagerModal
      :is-open="showPartage"
      :titre="section.station.name"
      type-media="station_radio"
      :media-id="section.station.id"
      :url-detail="lienStation ?? undefined"
      @close="showPartage = false"
    />

    <MediaProposerIdeeModal
      :is-open="showIdee"
      type-support="station_radio"
      :support-id="section.station.id"
      :nom-support="section.station.name"
      @close="showIdee = false"
    />
    <MediaDemanderAnimationModal
      :is-open="showAnimation"
      type-support="station_radio"
      :support-id="section.station.id"
      :nom-support="section.station.name"
      @close="showAnimation = false"
    />
  </section>
</template>
