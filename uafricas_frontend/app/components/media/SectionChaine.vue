<script setup lang="ts">
/**
 * Section d'une chaîne sur la page Télé, refondue par la feature 010.
 *
 * La vitrine annonce une **offre éditoriale**, plus un catalogue de fichiers :
 * identité → extrait de description → équipe → bandeau de programmation →
 * cartes de programme (FR-001). Aucun média n'y est lisible (FR-002), le
 * lecteur inline, l'épisode mis en avant, les rangées d'épisodes et la barre de
 * réactions sur épisode ont disparu ; ils vivent sur les pages de détail.
 *
 * Deux conséquences visibles et voulues : une chaîne sans aucun programme
 * s'affiche quand même, avec son identité et son équipe (FR-005), et la
 * description est coupée par une **ellipse figée**, le dépliage « voir plus »
 * appartient à la page de la chaîne (FR-003 vs FR-021).
 *
 * Tailwind v4 pur (Principe VI).
 */
import type { TeleSection } from '~/composables/useTelevision'
import { LIBELLES_ROLE_DETENTEUR, type RoleDetenteur } from '~/composables/useMediaDetention'

const props = defineProps<{
  section: TeleSection
  /**
   * Rôle du visiteur sur cette chaîne, s'il la détient, renseigné par la page,
   * qui connaît ses supports (un appel pour toutes les sections). `null` pour
   * un visiteur ordinaire : la vitrine reste alors strictement publique.
   */
  monRole?: RoleDetenteur | null
}>()

const { redirigerVersConnexion } = useAuth()
const userStore = useUserStore()

const showPartage = ref(false)
const showIdee = ref(false)
const showAnimation = ref(false)

// Reçoit une fonction d'ouverture, et non le ref lui-même : dans un template,
// Vue déballe les refs, si bien que `ouvrirSiConnecte(showIdee)` transmettait le
// booléen `false` et l'affectation de `.value` levait une TypeError.
const ouvrirSiConnecte = (ouvrir: () => void) => {
  if (!userStore.accessToken) {
    redirigerVersConnexion()
    return
  }
  ouvrir()
}

const lienChaine = computed(() =>
  props.section.chaine.slug ? `/medias/chaines/${props.section.chaine.slug}` : null,
)

/**
 * Passerelle vers la gestion de sa propre chaîne (grille, demandes, équipe).
 *
 * L'identifiant du support est porté en requête pour que « Mes supports »
 * déplie directement le bon panneau.
 */
const lienGestion = computed(() =>
  props.monRole ? `/mon-compte/mes-supports?support=${props.section.chaine.id}` : null,
)

/**
 * **Tous** les programmes servis sont affichés, y compris ceux qui n'ont
 * encore aucun épisode publié (FR-005). Le filtre `episodes.length > 0` qui
 * régnait ici n'avait de sens que tant que la section montrait des vidéos.
 */
const programmes = computed(() => props.section.emissions)

/**
 * La section n'affiche pas tout : le plafond serveur borne la liste à 30
 * programmes. Le dire est une exigence (FR-008) : une troncature silencieuse du
 * contenu principal serait pire que le plafond lui-même.
 */
const programmesMasques = computed(() =>
  Math.max(0, props.section.totalEmissions - programmes.value.length),
)
</script>

<template>
  <section class="scroll-mt-24 border-t border-white/10 py-10">
    <!-- Identité de la chaîne -->
    <header class="mb-4 flex items-center gap-4 px-1">
      <img
        v-if="section.chaine.cover"
        :src="section.chaine.cover"
        :alt="section.chaine.name"
        loading="lazy"
        class="h-14 w-14 shrink-0 rounded-lg object-cover"
      >
      <span
        v-else
        class="flex h-14 w-14 shrink-0 items-center justify-center rounded-lg bg-neutral-800"
      >
        <font-awesome-icon :icon="['fas', 'tv']" class="text-neutral-600" />
      </span>
      <div class="min-w-0 flex-1">
        <div class="flex flex-wrap items-center gap-3">
          <NuxtLink
            v-if="lienChaine"
            :to="lienChaine"
            class="truncate text-xl font-bold text-white transition-colors hover:text-custom-chocolat sm:text-2xl"
          >
            {{ section.chaine.name }}
          </NuxtLink>
          <h2 v-else class="truncate text-xl font-bold text-white sm:text-2xl">
            {{ section.chaine.name }}
          </h2>
          <span
            v-if="section.chaine.isLive"
            class="rounded-full bg-red-600 px-2 py-0.5 text-[10px] font-bold uppercase text-white"
          >
            En direct
          </span>
        </div>
        <p class="truncate text-sm text-gray-400">
          <span v-if="section.chaine.country">{{ section.chaine.country }}</span>
          <span v-if="section.chaine.country && section.chaine.category"> · </span>
          <span v-if="section.chaine.category">{{ section.chaine.category }}</span>
        </p>
      </div>
      <div class="flex shrink-0 items-center gap-3">
        <span class="hidden text-xs text-gray-500 sm:block">
          {{ section.totalEmissions }} programme{{ section.totalEmissions > 1 ? 's' : '' }}
        </span>

        <!-- Le détenteur retrouve sa chaîne dans la vitrine : la gestion se
             rejoint d'ici, sans repasser par son compte. Invisible pour tout
             autre visiteur. -->
        <NuxtLink
          v-if="lienGestion"
          :to="lienGestion"
          class="inline-flex items-center gap-2 rounded-full border border-custom-chocolat bg-custom-chocolat/10 px-4 py-1.5 text-xs font-semibold text-custom-chocolat transition-colors hover:bg-custom-chocolat/20 sm:text-sm"
          :title="`Vous êtes ${LIBELLES_ROLE_DETENTEUR[monRole!].toLowerCase()} de cette chaîne`"
        >
          <font-awesome-icon :icon="['fas', 'sliders']" class="h-3.5 w-3.5" />
          <span class="hidden sm:inline">Gérer ma chaîne</span>
          <span class="sm:hidden">Gérer</span>
        </NuxtLink>
      </div>
    </header>

    <!-- Extrait de description : ellipse figée, sans commande de dépliage 
         c'est la page de la chaîne qui déplie (FR-003). -->
    <CommonTexteRepliable
      v-if="section.chaine.description"
      :texte="section.chaine.description"
      :lignes="3"
      :repliable="false"
      sombre
      class="mb-4 max-w-4xl px-1 text-sm text-gray-300"
    />

    <!-- Équipe éditoriale : aucun seuil de repli en vitrine, et aucun cadre
         quand la chaîne n'en déclare pas (FR-007). -->
    <MediaEquipeMedia
      :membres="section.chaine.equipe"
      :seuil="0"
      sombre
      class="mb-6 px-1"
    />

    <!-- Ce que la grille programme à cet instant (US5, FR-039), texte seul,
         aucun lecteur : l'exigence « aucun média lisible » ne vise que les
         lecteurs et les vignettes (Q3 → A). -->
    <MediaBandeauDiffusion
      :en-cours="section.diffusionEnCours"
      :suivant="section.creneauSuivant"
      base-lien-contenu="programmes-tele"
    />

    <!-- Les programmes de la chaîne (FR-004) -->
    <div
      v-if="programmes.length"
      class="mt-6 grid grid-cols-2 gap-4 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5"
    >
      <MediaCarteProgramme
        v-for="programme in programmes"
        :key="programme.id"
        :programme="programme"
        type-support="chaine_tv"
      />
    </div>
    <p v-else class="mt-6 px-1 text-sm text-gray-500">
      Cette chaîne n'a pas encore annoncé de programme.
    </p>

    <!-- Jamais de disparition silencieuse : au-delà du plafond de section, le
         total est annoncé et la page de la chaîne prend le relais (FR-008). -->
    <NuxtLink
      v-if="programmesMasques > 0 && lienChaine"
      :to="lienChaine"
      class="mt-4 inline-flex items-center gap-2 px-1 text-sm font-medium text-custom-chocolat underline underline-offset-2 transition-colors hover:text-white"
    >
      Voir les {{ section.totalEmissions }} programmes
      <font-awesome-icon :icon="['fas', 'arrow-right']" class="h-3 w-3" />
    </NuxtLink>

    <!-- Réagir et partager la CHAÎNE sans quitter la page. La cible a changé
         avec le retrait de l'épisode mis en avant : c'est désormais le support,
         dont les compteurs sont servis par la même requête. -->
    <MediaReactionsBar
      compact
      class="mt-6 px-1"
      type-media="chaine_tv"
      :media-id="section.chaine.id"
      :nombre-likes="section.chaine.interactions?.nombre_likes ?? 0"
      :nombre-dislikes="section.chaine.interactions?.nombre_dislikes ?? 0"
      :ma-reaction="section.chaine.interactions?.ma_reaction ?? null"
      :nombre-commentaires="section.chaine.interactions?.nombre_commentaires ?? 0"
      :nombre-partages="section.chaine.interactions?.nombre_partages ?? 0"
      @require-login="redirigerVersConnexion()"
      @commenter="lienChaine && navigateTo(lienChaine)"
      @partager="showPartage = true"
    />

    <!-- S'engager auprès de la chaîne (US6, FR-044, FR-045) -->
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
        type-media="chaine_tv"
        :media-id="section.chaine.id"
        :titre="section.chaine.name"
        variante="pilule"
      />
    </div>

    <MediaPartagerModal
      :is-open="showPartage"
      :titre="section.chaine.name"
      type-media="chaine_tv"
      :media-id="section.chaine.id"
      :url-detail="lienChaine ?? undefined"
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
