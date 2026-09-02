<script setup lang="ts">

definePageMeta({ layout: false })
/**
 * Page de détail d'une chaîne de télévision (US3) : son identité, ses contenus
 * et ses interactions. Rendue côté serveur pour que l'aperçu social porte le
 * nom de la chaîne et non celui de la page de liste (FR-026).
 */
import { useTelevision, type TvEmission } from '~/composables/useTelevision'
import { LIBELLES_CADENCE } from '~/composables/useMediaEmissions'

const route = useRoute()
const slug = route.params.slug as string

const { obtenirChaineParSlug } = useTelevision()
const { redirigerVersConnexion } = useAuth()

const { data: detail, pending: chargement } = await useAsyncData(
  `chaine-${slug}`,
  () => obtenirChaineParSlug(slug),
)

/**
 * La fiche et ses **programmes** arrivent d'un seul appel : la page déplie le
 * catalogue à deux niveaux : la série, puis ses épisodes, sans second
 * aller-retour (US1 §3).
 */
const chaine = computed(() => detail.value?.chaine ?? null)
const programmes = computed<TvEmission[]>(() => detail.value?.emissions ?? [])

const lienEmission = (emission: TvEmission) =>
  emission.slug ? `/medias/emissions-tele/${emission.slug}` : null
// Les épisodes gardent leur adresse historique (FR-056).
const lienEpisode = (slugEpisode: string | null) =>
  slugEpisode ? `/medias/programmes-tele/${slugEpisode}` : null

const showPartage = ref(false)
const propositionOuverte = ref(false)
// Mise en relation avec l'equipe du support (US6, FR-046)
const contactOuvert = ref(false)
const nombreCommentaires = ref(0)

// Rechargé après qu'un cadeau vient d'être offert.
const cadeauxRef = ref<{ rafraichir: () => void } | null>(null)

const filAriane = computed(() => [
  { libelle: 'Médias', vers: '/medias' },
  { libelle: 'Télévision', vers: '/medias/tele' },
  { libelle: chaine.value?.name || 'Chaîne' },
])

// ── SEO / Open Graph ──
const requete = useRequestURL()
const urlCanonique = `${requete.protocol}//${requete.host}/medias/chaines/${slug}`
const imageOg = computed(() => chaine.value?.cover || '')
const descriptionOg = computed(() =>
  (chaine.value?.description || `Chaîne de télévision ${chaine.value?.name ?? ''}`).slice(0, 200),
)

useHead(() => {
  if (!chaine.value) return {}
  const titre = `${chaine.value.name}, Chaîne de télévision | UAfricas`
  return {
    title: titre,
    meta: [
      { name: 'description', content: descriptionOg.value },
      { property: 'og:type', content: 'website' },
      { property: 'og:title', content: titre },
      { property: 'og:description', content: descriptionOg.value },
      { property: 'og:url', content: urlCanonique },
      { property: 'og:site_name', content: 'UAfricas' },
      ...(imageOg.value ? [{ property: 'og:image', content: imageOg.value }] : []),
      { name: 'twitter:card', content: imageOg.value ? 'summary_large_image' : 'summary' },
      { name: 'twitter:title', content: titre },
      { name: 'twitter:description', content: descriptionOg.value },
      ...(imageOg.value ? [{ name: 'twitter:image', content: imageOg.value }] : []),
    ],
    link: [{ rel: 'canonical', href: urlCanonique }],
  }
})
</script>

<template>
  <NuxtLayout name="africans">
    <template #fil-ariane>
      <AfricansFilAriane :segments="filAriane" />
    </template>

    <div v-if="chargement" class="flex items-center justify-center py-24">
      <div class="animate-spin rounded-full h-12 w-12 text-3xl text-af-chocolat"></div>
    </div>

    <!-- Un contenu retiré est indiscernable d'un contenu inexistant (FR-028). -->
    <div v-else-if="!chaine" class="flex flex-col items-center justify-center py-24 px-4 text-center">
      <font-awesome-icon :icon="['fas', 'tv']" class="w-14 h-14 text-af-atone-2 mb-4" />
      <h1 class="text-2xl font-bold text-af-encre mb-2">Chaîne introuvable</h1>
      <p class="text-af-corps mb-4">
        Cette chaîne n’existe pas, ou elle a été retirée de l’antenne.
      </p>
      <NuxtLink to="/medias/tele" class="font-bold text-af-chocolat hover:underline">
        &#8592; Retour à la télévision
      </NuxtLink>
    </div>

    <template v-else>
      <div class="flex flex-col gap-6">

        <header class="flex flex-col sm:flex-row gap-6 items-start mb-8">
          <div class="w-24 h-24 rounded-2xl overflow-hidden bg-af-fond shrink-0">
            <img v-if="chaine.cover" :src="chaine.cover" :alt="chaine.name" class="w-full h-full object-cover">
            <span v-else class="w-full h-full flex items-center justify-center">
              <font-awesome-icon :icon="['fas', 'tv']" class="text-2xl text-af-atone-2" />
            </span>
          </div>

          <div class="min-w-0">
            <div class="flex items-center gap-3 flex-wrap mb-2">
              <h1 class="font-oswald text-3xl sm:text-4xl font-bold text-af-encre">{{ chaine.name }}</h1>
              <span
                v-if="chaine.isLive"
                class="rounded-full bg-af-live text-white text-xs font-bold px-2.5 py-0.5 uppercase"
              >
                En direct
              </span>
            </div>
            <p class="text-af-corps text-sm flex flex-wrap items-center gap-x-3 gap-y-1">
              <span v-if="chaine.category">{{ chaine.category }}</span>
              <span v-if="chaine.country">· {{ chaine.country }}</span>
              <span v-if="chaine.language">· {{ chaine.language }}</span>
            </p>
          </div>
        </header>

        <div class="mb-8">
          <MediaReactionsBar
            type-media="chaine_tv"
            :media-id="chaine.id"
            :nombre-likes="chaine.interactions?.nombre_likes ?? 0"
            :nombre-dislikes="chaine.interactions?.nombre_dislikes ?? 0"
            :ma-reaction="chaine.interactions?.ma_reaction ?? null"
            :nombre-commentaires="nombreCommentaires || (chaine.interactions?.nombre_commentaires ?? 0)"
            :nombre-partages="chaine.interactions?.nombre_partages ?? 0"
            @require-login="redirigerVersConnexion()"
            @partager="showPartage = true"
          />

          <!-- Proposer un contenu rattaché à ce support (US4) -->
          <button
            type="button"
            class="mt-4 inline-flex items-center gap-2 rounded-full border border-white/25 bg-af-fond text-af-encre px-5 py-2 text-sm font-semibold hover:bg-white/20 transition-colors cursor-pointer"
            @click="propositionOuverte = true"
          >
            <font-awesome-icon :icon="['fas', 'plus']" class="w-4 h-4" />
            Proposer un contenu
          </button>

          <!-- Contacter l'équipe du support (US6, FR-046) -->
          <button
            type="button"
            class="mt-4 sm:ml-3 inline-flex items-center gap-2 rounded-full border border-white/25 text-af-encre px-5 py-2 text-sm font-semibold hover:bg-af-fond transition-colors cursor-pointer"
            @click="contactOuvert = true"
          >
            <font-awesome-icon :icon="['fas', 'envelope']" class="w-4 h-4" />
            Contacter
          </button>

          <!-- Signaler ce support (US7, FR-049) -->
          <span class="mt-4 sm:ml-3 inline-flex align-middle">
            <MediaSignalerBouton
              type-media="chaine_tv"
              :media-id="chaine.id"
              :titre="chaine.name"
              variante="pilule"
            />
          </span>
          <span class="mt-4 sm:ml-3 inline-flex align-middle">
            <EngagementOffrirCadeauBouton
              type-objet="chaine_tv"
              :objet-id="chaine.id"
              :destinataire="chaine.name"
              @offert="cadeauxRef?.rafraichir()"
            />
          </span>
        </div>

        <!-- Description dépliable (FR-021) : une chaîne à la présentation longue
             ne doit ni écraser la page, ni obliger à quitter pour la lire. Un
             texte court s'affiche entier, SANS bouton (FR-022). -->
        <CommonTexteRepliable
          v-if="chaine.description"
          :texte="chaine.description"
          :lignes="5"
          sombre
          class="mb-8 text-af-corps leading-relaxed"
        />

        <!-- Équipe éditoriale de la chaîne (FR-023) : repliée au-delà de six
             fiches (FR-024). Aucun cadre si la chaîne n'en déclare pas. -->
        <MediaEquipeMedia
          :membres="chaine.equipe"
          :seuil="6"
          sombre
          class="mb-10"
        />

        <!-- Coordonnées publiques renseignées par l'équipe de la chaîne (09p) -->
        <!-- Thématiques déclarées et couverture territoriale (US3, US4) -->
        <MediaBlocIdentiteSupport
          :thematiques="chaine.thematiques"
          :couverture="chaine.couverture"
        />

        <MediaBlocContacts :contacts="chaine.contacts" :nom-support="chaine.name" />

        <!-- Les PROGRAMMES de la chaîne, chacun dépliant ses épisodes
             (US1 §3). Une chaîne se lit désormais par ses séries, pas par une
             mosaïque de vidéos sans lien entre elles. -->
        <section v-if="programmes.length" class="mb-12">
          <h2 class="font-oswald text-xl font-bold text-af-encre mb-4">Ses programmes</h2>

          <article
            v-for="emission in programmes"
            :key="emission.id"
            class="mb-8 last:mb-0"
          >
            <!-- Périodicité D'ABORD, et jamais masquée : « Non périodique » est
                 une information sur le rythme, pas une absence d'information
                 (FR-044, US5-3). -->
            <p class="mb-1 text-xs uppercase tracking-wide text-af-chocolat">
              {{ LIBELLES_CADENCE[emission.cadence] || emission.cadence }}
            </p>

            <div class="flex items-baseline justify-between gap-4 mb-2">
              <NuxtLink
                v-if="lienEmission(emission)"
                :to="lienEmission(emission)!"
                class="font-semibold text-af-encre hover:text-af-chocolat transition-colors truncate"
              >
                {{ emission.titre }}
              </NuxtLink>
              <h3 v-else class="font-semibold text-af-encre truncate">{{ emission.titre }}</h3>
              <span class="text-xs text-af-corps shrink-0">
                {{ emission.nombreEpisodes }} vidéo{{ emission.nombreEpisodes > 1 ? 's' : '' }}
              </span>
            </div>

            <!-- Aucune image de couverture de programme sur cette page (FR-026) :
                 elle appartient à la vitrine et à la page du programme. -->
            <CommonTexteRepliable
              v-if="emission.description"
              :texte="emission.description"
              :lignes="3"
              sombre
              class="mb-3 text-sm text-af-corps"
            />

            <!-- Équipe PROPRE au programme (FR-025), distincte de celle de la
                 chaîne : les deux coexistent sur cette page sans se confondre. -->
            <MediaEquipeMedia
              :membres="emission.equipe"
              titre=""
              :seuil="4"
              compact
              sombre
              class="mb-3"
            />

            <div v-if="emission.episodes.length" class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-4">
              <NuxtLink
                v-for="contenu in emission.episodes"
                :key="contenu.id"
                :to="lienEpisode(contenu.slug) ?? ''"
                class="group block"
              >
                <div class="aspect-video rounded-lg overflow-hidden bg-af-fond">
                  <img
                    v-if="contenu.banner"
                    :src="contenu.banner"
                    :alt="contenu.title"
                    loading="lazy"
                    class="w-full h-full object-cover transition-transform duration-300 group-hover:scale-105"
                  >
                  <span v-else class="w-full h-full flex items-center justify-center">
                    <font-awesome-icon :icon="['fas', 'image']" class="text-af-atone-2" />
                  </span>
                </div>
                <p class="mt-2 text-sm font-semibold text-af-encre truncate group-hover:text-af-chocolat transition-colors">
                  {{ contenu.title }}
                </p>
              </NuxtLink>
            </div>

            <p v-if="!emission.episodes.length" class="text-sm text-af-atone">
              Aucune vidéo publiée pour l'instant.
            </p>

            <!-- Au-delà de l'aperçu, la page du programme prend le relais :
                 c'est elle qui tient la promesse de 500 épisodes navigables. -->
            <NuxtLink
              v-if="lienEmission(emission) && emission.nombreEpisodes > emission.episodes.length"
              :to="lienEmission(emission)!"
              class="inline-block mt-3 text-sm font-bold text-af-chocolat hover:underline"
            >
              Voir les {{ emission.nombreEpisodes }} épisodes
            </NuxtLink>
          </article>
        </section>

        <!-- Cadeaux reçus par ce support (fond sombre : variante claire) -->
        <div class="mb-10">
          <EngagementCadeauxRecus
            ref="cadeauxRef"
            sombre
            type-objet="chaine_tv"
            :objet-id="chaine.id"
          />
        </div>

        <MediaCommentaires
          sombre
          type-media="chaine_tv"
          :media-id="chaine.id"
          @require-login="redirigerVersConnexion()"
          @total="nombreCommentaires = $event"
        />
      </div>

      <MediaProposerMediaModal
        :is-open="propositionOuverte"
        :types-offerts="['emission_tele']"
        :target-id="chaine.id"
        @close="propositionOuverte = false"
      />

      <MediaContacterSupportModal
        :is-open="contactOuvert"
        type-support="chaine_tv"
        :support-id="chaine.id"
        :nom-support="chaine.name"
        @close="contactOuvert = false"
      />

      <MediaPartagerModal
        :is-open="showPartage"
        :titre="chaine.name"
        type-media="chaine_tv"
        :media-id="chaine.id"
        :url-detail="`/medias/chaines/${slug}`"
        @close="showPartage = false"
      />
    </template>
  </NuxtLayout>
</template>
