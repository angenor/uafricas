<script setup lang="ts">

definePageMeta({ layout: false })
/**
 * Page de détail d'une station de radio (US3) : son identité, son direct, ses
 * émissions et ses interactions. Pendant exact de la page des chaînes.
 */
import { useStationsRadio, type EmissionRadio, type ProgrammeRadio } from '~/composables/useStationsRadio'
import { LIBELLES_CADENCE } from '~/composables/useMediaEmissions'

const route = useRoute()
const slug = route.params.slug as string

const { obtenirStationParSlug } = useStationsRadio()
const { redirigerVersConnexion } = useAuth()

const { data: detail, pending: chargement } = await useAsyncData(
  `station-${slug}`,
  () => obtenirStationParSlug(slug),
)

/**
 * La fiche et ses **programmes** arrivent d'un seul appel : la page déplie le
 * catalogue à deux niveaux : la série, puis ses épisodes (US1 §3).
 */
const station = computed(() => detail.value?.station ?? null)
const programmes = computed<EmissionRadio[]>(() => detail.value?.emissions ?? [])

const lienEmission = (emission: EmissionRadio) =>
  emission.slug ? `/medias/emissions-radio/${emission.slug}` : null

const showPartage = ref(false)
const propositionOuverte = ref(false)
// Mise en relation avec l'equipe du support (US6, FR-046)
const contactOuvert = ref(false)
const nombreCommentaires = ref(0)

// Rechargé après qu'un cadeau vient d'être offert.
const cadeauxRef = ref<{ rafraichir: () => void } | null>(null)

const filAriane = computed(() => [
  { libelle: 'Médias', vers: '/medias' },
  { libelle: 'Radio', vers: '/medias/radios' },
  { libelle: station.value?.name || 'Station' },
])

// ── SEO / Open Graph ──
const requete = useRequestURL()
const urlCanonique = `${requete.protocol}//${requete.host}/medias/stations/${slug}`
const imageOg = computed(() => station.value?.cover || '')
const descriptionOg = computed(() =>
  (station.value?.description || `Station de radio ${station.value?.name ?? ''}`).slice(0, 200),
)

useHead(() => {
  if (!station.value) return {}
  const titre = `${station.value.name}, Station de radio | UAfricas`
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

// L'écoute survit au départ de cette page : elle est confiée au lecteur
// persistant monté dans le layout (FR-017).
const { lire, estContenuCourant, enLecture } = useLecteurMedia()

const ecouterDirect = () => {
  if (!station.value) return
  lire({
    id: `direct-${station.value.id}`,
    type: 'station_radio',
    titre: `${station.value.name} : en direct`,
    support: station.value.name,
    supportSlug: station.value.slug,
    url: station.value.streamUrl,
    image: station.value.cover,
    estDirect: true,
  })
}

const directEnCours = computed(
  () => !!station.value && estContenuCourant(`direct-${station.value.id}`) && enLecture.value,
)

const ecouterEmission = (emission: ProgrammeRadio) => {
  if (!station.value) return
  lire({
    id: emission.id,
    type: 'episode_radio',
    titre: emission.title,
    support: station.value.name,
    supportSlug: station.value.slug,
    url: emission.audioUrl,
    image: emission.cover,
  })
}
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
    <div v-else-if="!station" class="flex flex-col items-center justify-center py-24 px-4 text-center">
      <font-awesome-icon :icon="['fas', 'radio']" class="w-14 h-14 text-af-atone-2 mb-4" />
      <h1 class="text-2xl font-bold text-af-encre mb-2">Station introuvable</h1>
      <p class="text-af-corps mb-4">
        Cette station n’existe pas, ou elle a été retirée de l’antenne.
      </p>
      <NuxtLink to="/medias/radios" class="font-bold text-af-chocolat hover:underline">
        &#8592; Retour à la radio
      </NuxtLink>
    </div>

    <template v-else>
      <div class="flex flex-col gap-6">

        <header class="flex flex-col sm:flex-row gap-6 items-start mb-8">
          <div class="w-24 h-24 rounded-2xl overflow-hidden bg-af-fond shrink-0">
            <img v-if="station.cover" :src="station.cover" :alt="station.name" class="w-full h-full object-cover">
            <span v-else class="w-full h-full flex items-center justify-center">
              <font-awesome-icon :icon="['fas', 'radio']" class="text-2xl text-af-atone-2" />
            </span>
          </div>

          <div class="min-w-0">
            <h1 class="font-oswald text-3xl sm:text-4xl font-bold text-af-encre mb-2">{{ station.name }}</h1>
            <p class="text-af-corps text-sm flex flex-wrap items-center gap-x-3 gap-y-1 mb-4">
              <span v-if="station.location">{{ station.location }}</span>
              <span v-if="station.location && station.genre">· </span>
              <span v-if="station.genre">{{ station.genre }}</span>
            </p>

            <!-- Le direct est offert au même rang qu'une émission (FR-016). -->
            <button
              v-if="station.streamUrl"
              type="button"
              class="inline-flex items-center gap-2 rounded-full px-5 py-2.5 text-sm font-semibold transition-colors cursor-pointer focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-af-chocolat"
              :class="directEnCours ? 'bg-af-live text-white' : 'border border-af-live text-af-live hover:bg-af-live/15'"
              @click="ecouterDirect"
            >
              <font-awesome-icon :icon="['fas', directEnCours ? 'volume-high' : 'play']" />
              {{ directEnCours ? 'En direct' : 'Écouter le direct' }}
            </button>
          </div>
        </header>

        <div class="mb-8">
          <MediaReactionsBar
            type-media="station_radio"
            :media-id="station.id"
            :nombre-likes="station.interactions?.nombre_likes ?? 0"
            :nombre-dislikes="station.interactions?.nombre_dislikes ?? 0"
            :ma-reaction="station.interactions?.ma_reaction ?? null"
            :nombre-commentaires="nombreCommentaires || (station.interactions?.nombre_commentaires ?? 0)"
            :nombre-partages="station.interactions?.nombre_partages ?? 0"
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
              type-media="station_radio"
              :media-id="station.id"
              :titre="station.name"
              variante="pilule"
            />
          </span>
          <span class="mt-4 sm:ml-3 inline-flex align-middle">
            <EngagementOffrirCadeauBouton
              type-objet="station_radio"
              :objet-id="station.id"
              :destinataire="station.name"
              @offert="cadeauxRef?.rafraichir()"
            />
          </span>
        </div>

        <!-- Description dépliable (FR-021) ; un texte court s'affiche entier,
             SANS bouton (FR-022). -->
        <CommonTexteRepliable
          v-if="station.description"
          :texte="station.description"
          :lignes="5"
          sombre
          class="mb-8 text-af-corps leading-relaxed"
        />

        <!-- Équipe éditoriale de la station (FR-023), repliée au-delà de six
             fiches (FR-024). -->
        <MediaEquipeMedia
          :membres="station.equipe"
          :seuil="6"
          sombre
          class="mb-10"
        />

        <!-- Coordonnées publiques renseignées par l'équipe de la station (09p) -->
        <!-- Thématiques déclarées et couverture territoriale (US3, US4) -->
        <MediaBlocIdentiteSupport
          :thematiques="station.thematiques"
          :couverture="station.couverture"
        />

        <MediaBlocContacts :contacts="station.contacts" :nom-support="station.name" />

        <!-- Les PROGRAMMES de la station (FR-025, FR-060). Rendu ALIGNÉ sur la
             page chaîne : une grille et non un carrousel, deux pages jumelles
             qui ne se lisent pas de la même façon sont deux pages à maintenir
             deux fois. -->
        <section v-if="programmes.length" class="mb-12">
          <h2 class="font-oswald text-xl font-bold text-af-encre mb-4">Ses programmes</h2>

          <article v-for="emission in programmes" :key="emission.id" class="mb-8 last:mb-0">
            <!-- Périodicité D'ABORD, et jamais masquée (FR-044, US5-3). -->
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
                {{ emission.nombreEpisodes }} enregistrement{{ emission.nombreEpisodes > 1 ? 's' : '' }}
              </span>
            </div>

            <!-- Aucune image de couverture de programme sur cette page (FR-026). -->
            <CommonTexteRepliable
              v-if="emission.description"
              :texte="emission.description"
              :lignes="3"
              sombre
              class="mb-3 text-sm text-af-corps"
            />

            <!-- Équipe PROPRE au programme (FR-025). -->
            <MediaEquipeMedia
              :membres="emission.equipe"
              titre=""
              :seuil="4"
              compact
              sombre
              class="mb-3"
            />

            <div
              v-if="emission.episodes.length"
              class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-4"
            >
              <MediaCarteContenu
                v-for="contenu in emission.episodes"
                :key="contenu.id"
                compacte
                :titre="contenu.title"
                :image="contenu.cover"
                :description="contenu.description"
                :a-la-une="contenu.aLaUne"
                :en-lecture="estContenuCourant(contenu.id) && enLecture"
                :lien="contenu.slug ? `/medias/programmes-radio/${contenu.slug}` : null"
                @lire="ecouterEmission(contenu)"
              />
            </div>
            <p v-else class="text-sm text-af-atone">
              Aucun enregistrement publié pour l'instant.
            </p>

            <NuxtLink
              v-if="lienEmission(emission) && emission.nombreEpisodes > emission.episodes.length"
              :to="lienEmission(emission)!"
              class="inline-block mt-3 text-sm text-af-chocolat hover:underline"
            >
              Voir les {{ emission.nombreEpisodes }} enregistrements
            </NuxtLink>
          </article>
        </section>

        <!-- Cadeaux reçus par ce support (fond sombre : variante claire) -->
        <div class="mb-10">
          <EngagementCadeauxRecus
            ref="cadeauxRef"
            sombre
            type-objet="station_radio"
            :objet-id="station.id"
          />
        </div>

        <MediaCommentaires
          sombre
          type-media="station_radio"
          :media-id="station.id"
          @require-login="redirigerVersConnexion()"
          @total="nombreCommentaires = $event"
        />
      </div>

      <MediaProposerMediaModal
        :is-open="propositionOuverte"
        :types-offerts="['emission_radio']"
        :target-id="station.id"
        @close="propositionOuverte = false"
      />

      <MediaContacterSupportModal
        :is-open="contactOuvert"
        type-support="station_radio"
        :support-id="station.id"
        :nom-support="station.name"
        @close="contactOuvert = false"
      />

      <MediaPartagerModal
        :is-open="showPartage"
        :titre="station.name"
        type-media="station_radio"
        :media-id="station.id"
        :url-detail="`/medias/stations/${slug}`"
        @close="showPartage = false"
      />
    </template>
  </NuxtLayout>
</template>
