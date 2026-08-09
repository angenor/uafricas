<script setup lang="ts">
/**
 * Section d'une chaîne sur la page Télé.
 *
 * Bloc empilé de hauteur naturelle : identité de la chaîne, bandeau de
 * l'épisode joué, puis **une rangée par programme** — et non plus une vignette
 * par vidéo (feature 009, US1 §3). Chaque rangée annonce le nombre d'épisodes
 * du programme et mène à sa page.
 *
 * Le lecteur n'est monté qu'une fois la section approchée du cadre : cinquante
 * sections montées d'emblée déclencheraient autant de chargements de médias
 * pour des contenus jamais vus.
 */
import type { TeleSection, TvEmission, TvProgram } from '~/composables/useTelevision'
import { LIBELLES_CADENCE } from '~/composables/useMediaEmissions'
import { LIBELLES_ROLE_DETENTEUR, type RoleDetenteur } from '~/composables/useMediaDetention'

const props = defineProps<{
  section: TeleSection
  /**
   * Rôle du visiteur sur cette chaîne, s'il la détient — renseigné par la page,
   * qui connaît ses supports (un appel pour toutes les sections). `null` pour
   * un visiteur ordinaire : la vitrine reste alors strictement publique.
   */
  monRole?: RoleDetenteur | null
}>()

const racine = ref<HTMLElement | null>(null)
const { aEteVisible } = useObservateurVisibilite(racine)

/**
 * Épisode joué dans le bandeau. À défaut de choix, le plus récent du premier
 * programme : la section doit montrer quelque chose sans attendre un clic.
 */
const contenuActif = ref<TvProgram | null>(null)
const contenuAffiche = computed<TvProgram | null>(
  () => contenuActif.value ?? props.section.emissions[0]?.episodes[0] ?? null,
)

/** Programme auquel appartient l'épisode joué — il nomme la série au-dessus du titre. */
const emissionAffichee = computed<TvEmission | null>(() => {
  const id = contenuAffiche.value?.emissionId
  return id ? (props.section.emissions.find(e => e.id === id) ?? null) : null
})

const lire = (contenu: TvProgram) => {
  contenuActif.value = contenu
  // Ramener le bandeau dans le champ de vision : sans cela, un clic sur une
  // carte du bas de la rangée changerait un lecteur resté hors écran.
  racine.value?.scrollIntoView({ behavior: 'smooth', block: 'start' })
}

// Les pages de détail sont livrées avec le lot « Participation » : les liens
// peuvent désormais être produits. Un contenu sans slug reste non cliquable,
// faute d'URL à lui donner.
const lienChaine = computed(() =>
  props.section.chaine.slug ? `/medias/chaines/${props.section.chaine.slug}` : null,
)

// Les épisodes restent à `/medias/programmes-tele/<slug>` : les slugs ayant été
// conservés par la migration 09q, les adresses déjà indexées continuent de
// résoudre (FR-056).
const lienContenu = (contenu: TvProgram) =>
  contenu.slug ? `/medias/programmes-tele/${contenu.slug}` : null

const lienEmission = (emission: TvEmission) =>
  emission.slug ? `/medias/emissions-tele/${emission.slug}` : null

const { redirigerVersConnexion } = useAuth()
const showPartage = ref(false)

// Engagement (US6) : proposer un sujet à la chaîne, ou demander à animer un de
// ses programmes. Les deux exigent un compte — la modale n'est ouverte qu'après
// vérification, sinon l'envoi échouerait au dernier moment.
const userStore = useUserStore()
const showIdee = ref(false)
const showAnimation = ref(false)

// Reçoit une fonction d'ouverture, et non le ref lui-même : dans un template,
// Vue déballe les refs, si bien que `ouvrirSiConnecte(showIdee)` transmettait le
// booléen `false` et l'affectation de `.value` levait une TypeError. Le bouton
// restait donc sans effet.
const ouvrirSiConnecte = (ouvrir: () => void) => {
  if (!userStore.accessToken) {
    redirigerVersConnexion()
    return
  }
  ouvrir()
}

/** Commenter suppose la page de détail, seule à héberger le fil complet. */
const ouvrirDetail = () => {
  const lien = contenuAffiche.value ? lienContenu(contenuAffiche.value) : null
  if (lien) navigateTo(lien)
}

/**
 * Passerelle vers la gestion de sa propre chaîne (grille, demandes, équipe).
 *
 * L'identifiant du support est porté en requête pour que « Mes supports »
 * déplie directement le bon panneau : sans lui, le détenteur atterrit sur une
 * liste où il doit retrouver la chaîne qu'il vient de quitter.
 */
const lienGestion = computed(() =>
  props.monRole ? `/mon-compte/mes-supports?support=${props.section.chaine.id}` : null,
)

/** Les programmes qui ont au moins un épisode à montrer. */
const programmes = computed(() => props.section.emissions.filter(e => e.episodes.length > 0))
</script>

<template>
  <section ref="racine" class="py-10 border-t border-white/10 scroll-mt-24">
    <!-- Identité de la chaîne -->
    <header class="flex items-center gap-4 mb-6 px-1">
      <img
        v-if="section.chaine.cover"
        :src="section.chaine.cover"
        :alt="section.chaine.name"
        loading="lazy"
        class="h-14 w-14 rounded-lg object-cover shrink-0"
      >
      <span
        v-else
        class="h-14 w-14 rounded-lg shrink-0 bg-neutral-800 flex items-center justify-center"
      >
        <font-awesome-icon :icon="['fas', 'tv']" class="text-neutral-600" />
      </span>
      <div class="min-w-0 flex-1">
        <div class="flex items-center gap-3 flex-wrap">
          <NuxtLink
            v-if="lienChaine"
            :to="lienChaine"
            class="text-xl sm:text-2xl font-bold text-white truncate hover:text-yellow-400 transition-colors"
          >
            {{ section.chaine.name }}
          </NuxtLink>
          <h2 v-else class="text-xl sm:text-2xl font-bold text-white truncate">
            {{ section.chaine.name }}
          </h2>
          <span
            v-if="section.chaine.isLive"
            class="rounded-full bg-red-600 text-white text-[10px] font-bold px-2 py-0.5 uppercase"
          >
            En direct
          </span>
        </div>
        <p class="text-gray-400 text-sm truncate">
          <span v-if="section.chaine.country">{{ section.chaine.country }}</span>
          <span v-if="section.chaine.country && section.chaine.category"> · </span>
          <span v-if="section.chaine.category">{{ section.chaine.category }}</span>
        </p>
      </div>
      <div class="shrink-0 flex items-center gap-3">
        <span class="text-xs text-gray-500 hidden sm:block">
          {{ section.totalEmissions }} programme{{ section.totalEmissions > 1 ? 's' : '' }}
        </span>

        <!-- Le détenteur retrouve sa chaîne dans la vitrine : la gestion se
             rejoint d'ici, sans repasser par son compte (grille, demandes
             reçues, équipe). Invisible pour tout autre visiteur. -->
        <NuxtLink
          v-if="lienGestion"
          :to="lienGestion"
          class="inline-flex items-center gap-2 rounded-full border border-yellow-400 bg-yellow-400/10 text-yellow-400 px-4 py-1.5 text-xs sm:text-sm font-semibold hover:bg-yellow-400/20 transition-colors"
          :title="`Vous êtes ${LIBELLES_ROLE_DETENTEUR[monRole!].toLowerCase()} de cette chaîne`"
        >
          <font-awesome-icon :icon="['fas', 'sliders']" class="w-3.5 h-3.5" />
          <span class="hidden sm:inline">Gérer ma chaîne</span>
          <span class="sm:hidden">Gérer</span>
        </NuxtLink>
      </div>
    </header>

    <!-- Ce que la grille programme à cet instant (US5, FR-039) -->
    <MediaBandeauDiffusion
      :en-cours="section.diffusionEnCours"
      :suivant="section.creneauSuivant"
      base-lien-contenu="programmes-tele"
    />

    <!-- Bandeau du contenu mis en évidence -->
    <div v-if="contenuAffiche" class="grid lg:grid-cols-5 gap-6 mb-8">
      <div class="lg:col-span-3 aspect-video rounded-xl overflow-hidden bg-black">
        <MediaLecteurMedia
          v-if="aEteVisible"
          :url="contenuAffiche.videoUrl"
          type="video"
          :titre="contenuAffiche.title"
          :poster="contenuAffiche.banner"
          controles
        />
        <!-- Avant l'entrée dans le cadre, seule l'affiche est chargée. -->
        <img
          v-else-if="contenuAffiche.banner"
          :src="contenuAffiche.banner"
          :alt="contenuAffiche.title"
          loading="lazy"
          class="w-full h-full object-cover"
        >
        <span v-else class="w-full h-full flex items-center justify-center bg-neutral-800">
          <font-awesome-icon :icon="['fas', 'tv']" class="text-3xl text-neutral-600" />
        </span>
      </div>

      <div class="lg:col-span-2 flex flex-col justify-center">
        <div class="flex items-center gap-2 mb-2 flex-wrap">
          <span
            v-if="contenuAffiche.aLaUne"
            class="rounded-full px-3 py-0.5 text-xs bg-yellow-400/20 border border-yellow-400 text-yellow-400 uppercase"
          >
            À la une
          </span>
          <span v-if="emissionAffichee?.themePhare" class="text-xs text-gray-400">
            {{ emissionAffichee.themePhare }}
          </span>
        </div>
        <!-- La série avant l'épisode : le visiteur doit savoir ce qu'il regarde
             AVANT de savoir quel numéro c'est (US1 §4). -->
        <NuxtLink
          v-if="emissionAffichee && lienEmission(emissionAffichee)"
          :to="lienEmission(emissionAffichee)!"
          class="text-yellow-400 text-sm font-semibold hover:underline mb-1"
        >
          {{ emissionAffichee.titre }}
        </NuxtLink>
        <h3 class="text-white text-lg sm:text-xl font-bold mb-2">
          <span v-if="contenuAffiche.numeroEpisode" class="text-gray-400 font-normal">
            Épisode {{ contenuAffiche.numeroEpisode }} —
          </span>
          {{ contenuAffiche.title }}
        </h3>
        <p v-if="contenuAffiche.description" class="text-gray-400 text-sm line-clamp-4 mb-4">
          {{ contenuAffiche.description }}
        </p>
        <NuxtLink
          v-if="lienContenu(contenuAffiche)"
          :to="lienContenu(contenuAffiche)!"
          class="inline-flex items-center gap-2 self-start rounded-full border border-yellow-400 bg-yellow-400/10 text-yellow-400 px-5 py-2 text-sm hover:bg-yellow-400/20 transition-colors"
        >
          <font-awesome-icon :icon="['fas', 'circle-info']" />
          Voir ce contenu
        </NuxtLink>

        <!-- Réagir et partager sans quitter la page (US3). Le fil de
             commentaires reste sur la page de détail : une section en affiche
             déjà beaucoup. -->
        <MediaReactionsBar
          compact
          class="mt-4"
          type-media="episode_tele"
          :media-id="contenuAffiche.id"
          :nombre-likes="contenuAffiche.interactions?.nombre_likes ?? 0"
          :nombre-dislikes="contenuAffiche.interactions?.nombre_dislikes ?? 0"
          :ma-reaction="contenuAffiche.interactions?.ma_reaction ?? null"
          :nombre-commentaires="contenuAffiche.interactions?.nombre_commentaires ?? 0"
          :nombre-partages="contenuAffiche.interactions?.nombre_partages ?? 0"
          @require-login="redirigerVersConnexion()"
          @commenter="ouvrirDetail()"
          @partager="showPartage = true"
        />
      </div>
    </div>

    <!-- Une rangée PAR PROGRAMME (US1 §3) : le catalogue se lit à deux
         niveaux, la série puis ses épisodes. -->
    <div v-for="emission in programmes" :key="emission.id" class="mb-6">
      <MediaRangeeContenus :titre="emission.titre">
        <template #entete>
          <div class="flex items-center gap-3 text-xs text-gray-400">
            <span>
              {{ emission.nombreEpisodes }} épisode{{ emission.nombreEpisodes > 1 ? 's' : '' }}
            </span>
            <span v-if="emission.cadence !== 'ponctuelle'">
              · {{ LIBELLES_CADENCE[emission.cadence] }}
            </span>
            <NuxtLink
              v-if="lienEmission(emission)"
              :to="lienEmission(emission)!"
              class="text-yellow-400 hover:underline"
            >
              Tout voir
            </NuxtLink>
          </div>
        </template>
        <MediaCarteContenu
          v-for="contenu in emission.episodes"
          :key="contenu.id"
          :titre="contenu.title"
          :image="contenu.banner"
          :description="contenu.description"
          :a-la-une="contenu.aLaUne"
          :lien="lienContenu(contenu)"
          @lire="lire(contenu)"
        />
      </MediaRangeeContenus>
    </div>

    <!-- S'engager auprès de la chaîne (US6, FR-044, FR-045) -->
    <div class="flex flex-wrap gap-3 mt-6">
      <button
        type="button"
        class="inline-flex items-center gap-2 rounded-full border border-white/20 text-gray-300 px-4 py-2 text-sm hover:border-yellow-400 hover:text-yellow-400 transition-colors"
        @click="ouvrirSiConnecte(() => showIdee = true)"
      >
        <font-awesome-icon :icon="['fas', 'lightbulb']" />
        Proposer une idée
      </button>
      <button
        type="button"
        class="inline-flex items-center gap-2 rounded-full border border-white/20 text-gray-300 px-4 py-2 text-sm hover:border-yellow-400 hover:text-yellow-400 transition-colors"
        @click="ouvrirSiConnecte(() => showAnimation = true)"
      >
        <font-awesome-icon :icon="['fas', 'microphone']" />
        Demander à animer
      </button>
      <!-- Signaler le contenu mis en évidence (US7, FR-049) -->
      <MediaSignalerBouton
        v-if="contenuAffiche"
        type-media="episode_tele"
        :media-id="contenuAffiche.id"
        :titre="contenuAffiche.title"
        variante="pilule"
      />
    </div>

    <MediaPartagerModal
      v-if="contenuAffiche"
      :is-open="showPartage"
      :titre="contenuAffiche.title"
      type-media="episode_tele"
      :media-id="contenuAffiche.id"
      :url-detail="lienContenu(contenuAffiche) ?? undefined"
      @close="showPartage = false"
    />

    <MediaProposerIdeeModal
      :is-open="showIdee"
      type-support="chaine_tv"
      :support-id="section.chaine.id"
      :nom-support="section.chaine.name"
      @close="showIdee = false"
    />
    <MediaDemanderAnimationModal
      :is-open="showAnimation"
      type-support="chaine_tv"
      :support-id="section.chaine.id"
      :nom-support="section.chaine.name"
      @close="showAnimation = false"
    />
  </section>
</template>

<style scoped>
.line-clamp-4 {
  display: -webkit-box;
  -webkit-line-clamp: 4;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
