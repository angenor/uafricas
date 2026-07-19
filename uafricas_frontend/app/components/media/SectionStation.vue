<script setup lang="ts">
/**
 * Section d'une station sur les pages Radio (FR-016, FR-022).
 *
 * Même structure que la section télé, à deux différences près : le direct est
 * proposé au même titre qu'une émission enregistrée, et la lecture est confiée
 * au lecteur persistant du layout — elle doit survivre au défilement et à la
 * navigation (FR-017).
 */
import type { StationSection, ProgrammeRadio } from '~/composables/useStationsRadio'

const props = defineProps<{ section: StationSection }>()

const { lire, estContenuCourant, enLecture } = useLecteurMedia()

// Voir `SectionChaine.vue` : les pages de détail sont livrées avec ce lot.
const lienStation = computed(() =>
  props.section.station.slug ? `/medias/stations/${props.section.station.slug}` : null,
)

const lienContenu = (contenu: ProgrammeRadio) =>
  contenu.slug ? `/medias/programmes-radio/${contenu.slug}` : null

const lireContenu = (contenu: ProgrammeRadio) => {
  lire({
    id: contenu.id,
    type: 'programme_radio',
    titre: contenu.title,
    support: props.section.station.name,
    supportSlug: props.section.station.slug,
    url: contenu.audioUrl,
    image: contenu.cover,
  })
}

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

const { redirigerVersConnexion } = useAuth()
const showPartage = ref(false)

// Engagement (US6) : proposer un sujet à la station, ou demander à animer une
// de ses émissions. Voir `SectionChaine.vue` — même contrat.
const userStore = useUserStore()
const showIdee = ref(false)
const showAnimation = ref(false)

const ouvrirSiConnecte = (cible: Ref<boolean>) => {
  if (!userStore.accessToken) {
    redirigerVersConnexion()
    return
  }
  cible.value = true
}

/** Commenter suppose la page de détail, seule à héberger le fil complet. */
const ouvrirDetail = () => {
  const lien = props.section.misEnEvidence ? lienContenu(props.section.misEnEvidence) : null
  if (lien) navigateTo(lien)
}

const directEnCours = computed(
  () => estContenuCourant(`direct-${props.section.station.id}`) && enLecture.value,
)
</script>

<template>
  <section class="py-10 border-t border-white/10">
    <!-- Identité de la station -->
    <header class="flex items-center gap-4 mb-6 px-1">
      <img
        v-if="section.station.cover"
        :src="section.station.cover"
        :alt="section.station.name"
        loading="lazy"
        class="h-14 w-14 rounded-lg object-cover shrink-0"
      >
      <span
        v-else
        class="h-14 w-14 rounded-lg shrink-0 bg-neutral-800 flex items-center justify-center"
      >
        <font-awesome-icon :icon="['fas', 'radio']" class="text-neutral-600" />
      </span>
      <div class="min-w-0 flex-1">
        <NuxtLink
          v-if="lienStation"
          :to="lienStation"
          class="block text-xl sm:text-2xl font-bold text-white truncate hover:text-yellow-400 transition-colors"
        >
          {{ section.station.name }}
        </NuxtLink>
        <h2 v-else class="block text-xl sm:text-2xl font-bold text-white truncate">
          {{ section.station.name }}
        </h2>
        <p class="text-gray-400 text-sm truncate">
          <span v-if="section.station.location">{{ section.station.location }}</span>
          <span v-if="section.station.location && section.station.genre"> · </span>
          <span v-if="section.station.genre">{{ section.station.genre }}</span>
        </p>
      </div>

      <!-- Le direct, offert au même rang que les émissions (FR-016) -->
      <button
        v-if="section.directDisponible"
        type="button"
        class="shrink-0 inline-flex items-center gap-2 rounded-full px-4 py-2 text-sm font-semibold transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-yellow-400"
        :class="directEnCours
          ? 'bg-red-600 text-white'
          : 'border border-red-500 text-red-400 hover:bg-red-500/15'"
        @click="lireDirect"
      >
        <font-awesome-icon :icon="['fas', directEnCours ? 'volume-high' : 'play']" />
        <span class="hidden sm:inline">{{ directEnCours ? 'En direct' : 'Écouter le direct' }}</span>
      </button>
    </header>

    <!-- Émission mise en évidence -->
    <!-- Ce que la grille programme à cet instant (US5, FR-039) -->
    <MediaBandeauDiffusion
      :en-cours="section.diffusionEnCours"
      :suivant="section.creneauSuivant"
      base-lien-contenu="programmes-radio"
    />

    <div v-if="section.misEnEvidence" class="flex flex-col sm:flex-row gap-5 mb-8 px-1">
      <button
        type="button"
        class="relative w-full sm:w-56 aspect-video rounded-xl overflow-hidden bg-gray-800 shrink-0 group/mev focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-yellow-400"
        :aria-label="`Écouter ${section.misEnEvidence.title}`"
        @click="lireContenu(section.misEnEvidence)"
      >
        <img
          v-if="section.misEnEvidence.cover"
          :src="section.misEnEvidence.cover"
          :alt="section.misEnEvidence.title"
          loading="lazy"
          class="w-full h-full object-cover"
        >
        <span v-else class="w-full h-full flex items-center justify-center bg-neutral-800">
          <font-awesome-icon :icon="['fas', 'radio']" class="text-2xl text-neutral-600" />
        </span>
        <span class="absolute inset-0 bg-black/40 flex items-center justify-center">
          <span class="h-14 w-14 rounded-full bg-white/90 text-black flex items-center justify-center">
            <font-awesome-icon
              :icon="['fas', estContenuCourant(section.misEnEvidence.id) && enLecture ? 'volume-high' : 'play']"
              class="text-lg"
            />
          </span>
        </span>
      </button>

      <div class="min-w-0 flex flex-col justify-center">
        <div class="flex items-center gap-2 mb-1 flex-wrap">
          <span
            v-if="section.misEnEvidence.aLaUne"
            class="rounded-full px-3 py-0.5 text-xs bg-yellow-400/20 border border-yellow-400 text-yellow-400 uppercase"
          >
            À la une
          </span>
          <span v-if="section.misEnEvidence.themePhare" class="text-xs text-gray-400">
            {{ section.misEnEvidence.themePhare }}
          </span>
        </div>
        <h3 class="text-white text-lg font-bold">{{ section.misEnEvidence.title }}</h3>
        <p v-if="section.misEnEvidence.description" class="text-gray-400 text-sm line-clamp-3 mt-1">
          {{ section.misEnEvidence.description }}
        </p>
        <NuxtLink
          v-if="lienContenu(section.misEnEvidence)"
          :to="lienContenu(section.misEnEvidence)!"
          class="mt-3 inline-flex items-center gap-2 self-start text-yellow-400 text-sm hover:text-yellow-300 underline underline-offset-4"
        >
          En savoir plus
        </NuxtLink>

        <!-- Réagir et partager sans quitter la page (US3). Le fil de
             commentaires reste sur la page de détail. -->
        <MediaReactionsBar
          compact
          class="mt-4"
          type-media="programme_radio"
          :media-id="section.misEnEvidence.id"
          :nombre-likes="section.misEnEvidence.interactions?.nombre_likes ?? 0"
          :nombre-dislikes="section.misEnEvidence.interactions?.nombre_dislikes ?? 0"
          :ma-reaction="section.misEnEvidence.interactions?.ma_reaction ?? null"
          :nombre-commentaires="section.misEnEvidence.interactions?.nombre_commentaires ?? 0"
          :nombre-partages="section.misEnEvidence.interactions?.nombre_partages ?? 0"
          @require-login="redirigerVersConnexion()"
          @commenter="ouvrirDetail()"
          @partager="showPartage = true"
        />
      </div>
    </div>

    <!-- Rangée des autres émissions -->
    <MediaRangeeContenus v-if="section.contenus.length" titre="Autres émissions">
      <MediaCarteContenu
        v-for="contenu in section.contenus"
        :key="contenu.id"
        compacte
        :titre="contenu.title"
        :image="contenu.cover"
        :description="contenu.description"
        :a-la-une="contenu.aLaUne"
        :en-lecture="estContenuCourant(contenu.id) && enLecture"
        :lien="lienContenu(contenu)"
        @lire="lireContenu(contenu)"
      />
    </MediaRangeeContenus>

    <p v-else-if="!section.misEnEvidence" class="text-gray-500 text-sm px-1">
      Cette station n'a pas encore publié d'émission.
    </p>

    <!-- S'engager auprès de la station (US6, FR-044, FR-045) -->
    <div class="flex flex-wrap gap-3 mt-6 px-1">
      <button
        type="button"
        class="inline-flex items-center gap-2 rounded-full border border-white/20 text-gray-300 px-4 py-2 text-sm hover:border-yellow-400 hover:text-yellow-400 transition-colors"
        @click="ouvrirSiConnecte(showIdee)"
      >
        <font-awesome-icon :icon="['fas', 'lightbulb']" />
        Proposer une idée
      </button>
      <button
        type="button"
        class="inline-flex items-center gap-2 rounded-full border border-white/20 text-gray-300 px-4 py-2 text-sm hover:border-yellow-400 hover:text-yellow-400 transition-colors"
        @click="ouvrirSiConnecte(showAnimation)"
      >
        <font-awesome-icon :icon="['fas', 'microphone']" />
        Demander à animer
      </button>
      <!-- Signaler l'émission mise en évidence (US7, FR-049) -->
      <MediaSignalerBouton
        v-if="section.misEnEvidence"
        type-media="programme_radio"
        :media-id="section.misEnEvidence.id"
        :titre="section.misEnEvidence.title"
        variante="pilule"
      />
    </div>

    <MediaPartagerModal
      v-if="section.misEnEvidence"
      :is-open="showPartage"
      :titre="section.misEnEvidence.title"
      type-media="programme_radio"
      :media-id="section.misEnEvidence.id"
      :url-detail="lienContenu(section.misEnEvidence) ?? undefined"
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

<style scoped>
.line-clamp-3 {
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
