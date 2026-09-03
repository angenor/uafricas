<script setup lang="ts">
/**
 * Section d'une chaîne sur la page Télé : une **rangée de catalogue**.
 *
 * La chaîne joue le rôle qu'une catégorie tient sur une plateforme de
 * streaming : son nom titre la rangée, et ses programmes défilent
 * horizontalement sous lui, couverture en avant. Ce que la grille verticale
 * mettait sur un même plan : identité, description, équipe, réactions,
 * boutons d'engagement, encadre désormais la rangée sans la disputer :
 *
 *   • en-tête          nom, direct, territoire, décompte, accès à la chaîne ;
 *   • bandeau          ce que la grille programme à cet instant (US5) ;
 *   • rangée           les programmes (FR-004), tuile finale vers la chaîne ;
 *   • pied             réactions, puis « À propos » qui déplie description,
 *                      équipe (FR-003, FR-007) et gestes d'engagement (US6).
 *
 * Aucune exigence n'est perdue au passage : le repli est une mise en second
 * plan, pas une suppression. La vitrine annonce une **offre éditoriale**, plus
 * un catalogue de fichiers : aucun média n'y est lisible (FR-002), et une
 * chaîne sans aucun programme s'affiche quand même (FR-005).
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
/** « À propos » : description, équipe et gestes d'engagement, replié par défaut. */
const detailsOuverts = ref(false)

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
 * contenu principal serait pire que le plafond lui-même. En rangée, l'aveu tient
 * dans une tuile terminale, à la place où le défilement s'arrête.
 */
const programmesMasques = computed(() =>
  Math.max(0, props.section.totalEmissions - programmes.value.length),
)

/** Territoire et catégorie, sur une ligne, sans séparateur orphelin. */
const meta = computed(() =>
  [props.section.chaine.country, props.section.chaine.category].filter(Boolean).join(' · '),
)
</script>

<template>
  <section class="group/section scroll-mt-24">
    <!-- En-tête de rangée : le nom de la chaîne titre le catalogue. -->
    <header class="mb-3 flex items-center gap-3 px-1">
      <img
        v-if="section.chaine.cover"
        :src="section.chaine.cover"
        :alt="section.chaine.name"
        loading="lazy"
        class="h-11 w-11 shrink-0 rounded-lg object-cover ring-1 ring-af-bordure"
      >
      <span
        v-else
        class="flex h-11 w-11 shrink-0 items-center justify-center rounded-lg bg-af-bordure/60"
      >
        <font-awesome-icon :icon="['fas', 'tv']" class="text-af-atone-2" />
      </span>

      <div class="min-w-0 flex-1">
        <div class="flex flex-wrap items-center gap-2">
          <NuxtLink
            v-if="lienChaine"
            :to="lienChaine"
            class="truncate text-lg font-bold text-af-encre transition-colors hover:text-af-chocolat sm:text-xl"
          >
            {{ section.chaine.name }}
          </NuxtLink>
          <h2 v-else class="truncate text-lg font-bold text-af-encre sm:text-xl">
            {{ section.chaine.name }}
          </h2>
          <span
            v-if="section.chaine.isLive"
            class="rounded-full bg-af-live px-2 py-0.5 text-[10px] font-bold uppercase text-white"
          >
            En direct
          </span>
        </div>
        <p v-if="meta || section.totalEmissions" class="truncate text-xs text-af-atone">
          <span v-if="meta">{{ meta }}</span>
          <span v-if="meta && section.totalEmissions"> · </span>
          <span v-if="section.totalEmissions">
            {{ section.totalEmissions }} programme{{ section.totalEmissions > 1 ? 's' : '' }}
          </span>
        </p>
      </div>

      <div class="flex shrink-0 items-center gap-2">
        <!-- Le détenteur retrouve sa chaîne dans la vitrine : la gestion se
             rejoint d'ici, sans repasser par son compte. Invisible pour tout
             autre visiteur. -->
        <NuxtLink
          v-if="lienGestion"
          :to="lienGestion"
          class="inline-flex items-center gap-2 rounded-full border border-af-chocolat bg-af-chocolat/10 px-3 py-1.5 text-xs font-semibold text-af-chocolat transition-colors hover:bg-af-chocolat/20"
          :title="`Vous êtes ${LIBELLES_ROLE_DETENTEUR[monRole!].toLowerCase()} de cette chaîne`"
        >
          <font-awesome-icon :icon="['fas', 'sliders']" class="h-3.5 w-3.5" />
          <span class="hidden sm:inline">Gérer ma chaîne</span>
          <span class="sm:hidden">Gérer</span>
        </NuxtLink>

        <!-- Au doigt, aucun survol n'existe : le lien reste alors visible, et
             ne s'efface qu'à partir du grand écran, où le survol le rappelle. -->
        <NuxtLink
          v-if="lienChaine"
          :to="lienChaine"
          class="inline-flex items-center gap-1 text-xs font-semibold text-af-chocolat transition-opacity hover:opacity-70 focus-visible:opacity-100 md:opacity-0 md:group-hover/section:opacity-100 md:group-focus-within/section:opacity-100"
        >
          <span class="hidden sm:inline">Explorer la chaîne</span>
          <span class="sm:hidden">Explorer</span>
          <font-awesome-icon :icon="['fas', 'chevron-right']" class="h-2.5 w-2.5" />
        </NuxtLink>
      </div>
    </header>

    <!-- Ce que la grille programme à cet instant (US5, FR-039), texte seul,
         aucun lecteur : l'exigence « aucun média lisible » ne vise que les
         lecteurs et les vignettes (Q3 → A). -->
    <MediaBandeauDiffusion
      :en-cours="section.diffusionEnCours"
      :suivant="section.creneauSuivant"
      base-lien-contenu="programmes-tele"
    />

    <!-- Les programmes de la chaîne (FR-004), parcourus à l'horizontale -->
    <MediaRangeeContenus v-if="programmes.length">
      <MediaCarteProgramme
        v-for="programme in programmes"
        :key="programme.id"
        :programme="programme"
        type-support="chaine_tv"
        format="rangee"
      />

      <!-- Jamais de disparition silencieuse : au-delà du plafond de section, le
           total est annoncé et la page de la chaîne prend le relais (FR-008). -->
      <NuxtLink
        v-if="programmesMasques > 0 && lienChaine"
        :to="lienChaine"
        role="listitem"
        class="group/tout flex w-[210px] shrink-0 snap-start flex-col items-center justify-center gap-2 self-start rounded-lg border border-dashed border-af-bordure bg-af-fond text-af-corps transition hover:border-af-chocolat hover:text-af-chocolat sm:w-[238px] lg:w-[262px]"
        style="aspect-ratio: 16 / 9"
      >
        <span class="flex h-11 w-11 items-center justify-center rounded-full bg-white ring-1 ring-af-bordure transition group-hover/tout:ring-af-chocolat">
          <font-awesome-icon :icon="['fas', 'arrow-right']" />
        </span>
        <span class="px-3 text-center text-sm font-semibold">
          Voir les {{ section.totalEmissions }} programmes
        </span>
      </NuxtLink>
    </MediaRangeeContenus>
    <p v-else class="px-1 py-3 text-sm text-af-atone">
      Cette chaîne n'a pas encore annoncé de programme.
    </p>

    <!-- Pied de rangée : réagir et partager LA CHAÎNE sans quitter la page (la
         cible a changé avec le retrait de l'épisode mis en avant), et la
         commande qui déplie tout le reste. -->
    <div class="mt-2 flex flex-wrap items-center justify-between gap-3 px-1">
      <MediaReactionsBar
        compact
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

      <button
        type="button"
        class="inline-flex items-center gap-2 text-xs font-semibold text-af-corps transition-colors hover:text-af-chocolat"
        :aria-expanded="detailsOuverts"
        @click="detailsOuverts = !detailsOuverts"
      >
        À propos de la chaîne
        <font-awesome-icon :icon="['fas', detailsOuverts ? 'chevron-up' : 'chevron-down']" class="h-2.5 w-2.5" />
      </button>
    </div>

    <div v-if="detailsOuverts" class="mt-3 rounded-xl border border-af-bordure bg-white/60 px-4 py-4">
      <!-- Extrait de description : ellipse figée, sans commande de dépliage :
           c'est la page de la chaîne qui déplie (FR-003). -->
      <CommonTexteRepliable
        v-if="section.chaine.description"
        :texte="section.chaine.description"
        :lignes="3"
        :repliable="false"
        class="max-w-4xl text-sm text-af-corps"
      />

      <!-- Équipe éditoriale : aucun seuil de repli en vitrine, et aucun cadre
           quand la chaîne n'en déclare pas (FR-007). -->
      <MediaEquipeMedia
        :membres="section.chaine.equipe"
        :seuil="0"
      />

      <!-- S'engager auprès de la chaîne (US6, FR-044, FR-045) -->
      <div class="mt-4 flex flex-wrap gap-3">
        <button
          type="button"
          class="inline-flex items-center gap-2 rounded-full border border-af-bordure px-4 py-2 text-sm text-af-corps transition-colors hover:border-af-chocolat hover:text-af-chocolat"
          @click="ouvrirSiConnecte(() => showIdee = true)"
        >
          <font-awesome-icon :icon="['fas', 'lightbulb']" />
          Proposer une idée
        </button>
        <button
          type="button"
          class="inline-flex items-center gap-2 rounded-full border border-af-bordure px-4 py-2 text-sm text-af-corps transition-colors hover:border-af-chocolat hover:text-af-chocolat"
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
