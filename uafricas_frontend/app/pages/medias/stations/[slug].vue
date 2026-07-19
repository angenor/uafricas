<script setup lang="ts">
/**
 * Page de détail d'une station de radio (US3) : son identité, son direct, ses
 * émissions et ses interactions. Pendant exact de la page des chaînes.
 */
import { useStationsRadio, type ProgrammeRadio } from '~/composables/useStationsRadio'

const route = useRoute()
const slug = route.params.slug as string

const { obtenirStationParSlug, listerContenusStation } = useStationsRadio()
const { redirigerVersConnexion } = useAuth()

const { data: station, pending: chargement } = await useAsyncData(
  `station-${slug}`,
  () => obtenirStationParSlug(slug),
)

const contenus = ref<ProgrammeRadio[]>([])
onMounted(async () => {
  if (!station.value) return
  contenus.value = await listerContenusStation(station.value.id)
})

const showPartage = ref(false)
const propositionOuverte = ref(false)
// Mise en relation avec l'equipe du support (US6, FR-046)
const contactOuvert = ref(false)
const nombreCommentaires = ref(0)

const breadcrumbs = computed(() => [
  { label: 'Médias', to: '/medias' },
  { label: 'Radio', to: '/medias/radios' },
  { label: station.value?.name || 'Station', to: undefined },
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
  const titre = `${station.value.name} — Station de radio — UAfricas`
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
    titre: `${station.value.name} — en direct`,
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
    type: 'programme_radio',
    titre: emission.title,
    support: station.value.name,
    supportSlug: station.value.slug,
    url: emission.audioUrl,
    image: emission.cover,
  })
}
</script>

<template>
  <div class="min-h-screen bg-neutral-950">
    <div v-if="chargement" class="flex items-center justify-center h-screen">
      <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-yellow-400"></div>
    </div>

    <!-- Un contenu retiré est indiscernable d'un contenu inexistant (FR-028). -->
    <div v-else-if="!station" class="flex flex-col items-center justify-center h-screen px-4 text-center">
      <font-awesome-icon :icon="['fas', 'radio']" class="w-14 h-14 text-neutral-700 mb-4" />
      <h1 class="text-2xl font-bold text-white mb-2">Station introuvable</h1>
      <p class="text-gray-400 mb-4">
        Cette station n’existe pas, ou elle a été retirée de l’antenne.
      </p>
      <NuxtLink to="/medias/radios" class="text-yellow-400 hover:underline">
        &#8592; Retour à la radio
      </NuxtLink>
    </div>

    <template v-else>
      <div class="max-w-5xl mx-auto px-4 pt-24 pb-16">
        <nav aria-label="Fil d'Ariane" class="mb-6 text-sm text-gray-400">
          <template v-for="(fil, i) in breadcrumbs" :key="i">
            <NuxtLink v-if="fil.to" :to="fil.to" class="hover:text-yellow-400">{{ fil.label }}</NuxtLink>
            <span v-else class="text-white">{{ fil.label }}</span>
            <span v-if="i < breadcrumbs.length - 1" class="mx-2">/</span>
          </template>
        </nav>

        <header class="flex flex-col sm:flex-row gap-6 items-start mb-8">
          <div class="w-24 h-24 rounded-2xl overflow-hidden bg-neutral-900 shrink-0">
            <img v-if="station.cover" :src="station.cover" :alt="station.name" class="w-full h-full object-cover">
            <span v-else class="w-full h-full flex items-center justify-center">
              <font-awesome-icon :icon="['fas', 'radio']" class="text-2xl text-neutral-700" />
            </span>
          </div>

          <div class="min-w-0">
            <h1 class="font-oswald text-3xl sm:text-4xl font-bold text-white mb-2">{{ station.name }}</h1>
            <p class="text-gray-400 text-sm flex flex-wrap items-center gap-x-3 gap-y-1 mb-4">
              <span v-if="station.location">{{ station.location }}</span>
              <span v-if="station.location && station.genre">· </span>
              <span v-if="station.genre">{{ station.genre }}</span>
            </p>

            <!-- Le direct est offert au même rang qu'une émission (FR-016). -->
            <button
              v-if="station.streamUrl"
              type="button"
              class="inline-flex items-center gap-2 rounded-full px-5 py-2.5 text-sm font-semibold transition-colors cursor-pointer focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-yellow-400"
              :class="directEnCours ? 'bg-red-600 text-white' : 'border border-red-500 text-red-400 hover:bg-red-500/15'"
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
            class="mt-4 inline-flex items-center gap-2 rounded-full border border-white/25 bg-white/10 text-white px-5 py-2 text-sm font-semibold hover:bg-white/20 transition-colors cursor-pointer"
            @click="propositionOuverte = true"
          >
            <font-awesome-icon :icon="['fas', 'plus']" class="w-4 h-4" />
            Proposer un contenu
          </button>

          <!-- Contacter l'équipe du support (US6, FR-046) -->
          <button
            type="button"
            class="mt-4 sm:ml-3 inline-flex items-center gap-2 rounded-full border border-white/25 text-white px-5 py-2 text-sm font-semibold hover:bg-white/10 transition-colors cursor-pointer"
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
        </div>

        <p v-if="station.description" class="text-gray-300 leading-relaxed whitespace-pre-line mb-10">
          {{ station.description }}
        </p>

        <section v-if="contenus.length" class="mb-12">
          <h2 class="font-oswald text-xl font-bold text-white mb-4">Ses émissions</h2>
          <MediaRangeeContenus>
            <MediaCarteContenu
              v-for="contenu in contenus"
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
          </MediaRangeeContenus>
        </section>

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
        :types-offerts="['programme_radio']"
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
  </div>
</template>
