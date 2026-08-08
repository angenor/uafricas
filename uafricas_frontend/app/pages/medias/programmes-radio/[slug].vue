<script setup lang="ts">
/**
 * Page de détail d'une émission de radio (US3).
 *
 * Pendant exact de la page des émissions télé : `useAsyncData` au niveau racine
 * pour un rendu serveur, et des balises Open Graph propres au contenu, sans
 * quoi tout partage produirait le même aperçu (FR-026).
 */
import { useStationsRadio } from '~/composables/useStationsRadio'

const route = useRoute()
const slug = route.params.slug as string

const { obtenirProgrammeRadioParSlug } = useStationsRadio()
const { redirigerVersConnexion } = useAuth()

const { data: emission, pending: chargement } = await useAsyncData(
  `programme-radio-${slug}`,
  () => obtenirProgrammeRadioParSlug(slug),
)

const showPartage = ref(false)
const propositionOuverte = ref(false)
const nombreCommentaires = ref(0)

// Rechargé après qu'un cadeau vient d'être offert.
const cadeauxRef = ref<{ rafraichir: () => void } | null>(null)

const lienStation = computed(() =>
  emission.value?.stationSlug ? `/medias/stations/${emission.value.stationSlug}` : null,
)

const breadcrumbs = computed(() => [
  { label: 'Médias', to: '/medias' },
  { label: 'Radio', to: '/medias/radios' },
  { label: emission.value?.title || 'Émission', to: undefined },
])

// ── SEO / Open Graph ──
const requete = useRequestURL()
const urlCanonique = `${requete.protocol}//${requete.host}/medias/programmes-radio/${slug}`
const imageOg = computed(() => emission.value?.cover || '')
const descriptionOg = computed(() =>
  (emission.value?.description || `Émission radio ${emission.value?.title ?? ''}`).slice(0, 200),
)

useHead(() => {
  if (!emission.value) return {}
  const titre = `${emission.value.title} — Radio — UAfricas`
  return {
    title: titre,
    meta: [
      { name: 'description', content: descriptionOg.value },
      { property: 'og:type', content: 'music.song' },
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

// L'écoute est confiée au lecteur persistant du layout : elle doit survivre au
// départ de cette page (FR-017).
const { lire, estContenuCourant, enLecture } = useLecteurMedia()

const ecouter = () => {
  if (!emission.value) return
  lire({
    id: emission.value.id,
    type: 'programme_radio',
    titre: emission.value.title,
    support: emission.value.stationNom ?? '',
    supportSlug: emission.value.stationSlug,
    url: emission.value.audioUrl,
    image: emission.value.cover,
  })
}

const enCours = computed(
  () => !!emission.value && estContenuCourant(emission.value.id) && enLecture.value,
)
</script>

<template>
  <div class="min-h-screen bg-neutral-950">
    <div v-if="chargement" class="flex items-center justify-center h-screen">
      <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-yellow-400"></div>
    </div>

    <!-- Un contenu retiré est indiscernable d'un contenu inexistant (FR-028). -->
    <div v-else-if="!emission" class="flex flex-col items-center justify-center h-screen px-4 text-center">
      <font-awesome-icon :icon="['fas', 'microphone']" class="w-14 h-14 text-neutral-700 mb-4" />
      <h1 class="text-2xl font-bold text-white mb-2">Émission introuvable</h1>
      <p class="text-gray-400 mb-4">
        Cette émission n’existe pas, ou elle a été retirée de l’antenne.
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

        <!-- Identité et écoute -->
        <header class="flex flex-col sm:flex-row gap-6 mb-8">
          <div class="w-full sm:w-64 aspect-square rounded-2xl overflow-hidden bg-neutral-900 shrink-0">
            <img
              v-if="emission.cover"
              :src="emission.cover"
              :alt="emission.title"
              class="w-full h-full object-cover"
            >
            <span v-else class="w-full h-full flex items-center justify-center">
              <font-awesome-icon :icon="['fas', 'microphone']" class="text-4xl text-neutral-700" />
            </span>
          </div>

          <div class="min-w-0 flex flex-col justify-center">
            <h1 class="font-oswald text-3xl sm:text-4xl font-bold text-white mb-2">
              {{ emission.title }}
            </h1>
            <p class="text-gray-400 text-sm flex flex-wrap items-center gap-x-3 gap-y-1 mb-5">
              <NuxtLink v-if="lienStation" :to="lienStation" class="hover:text-yellow-400">
                {{ emission.stationNom }}
              </NuxtLink>
              <span v-else-if="emission.stationNom">{{ emission.stationNom }}</span>
              <span v-if="emission.themePhare">· {{ emission.themePhare }}</span>
            </p>

            <button
              type="button"
              class="self-start inline-flex items-center gap-2 rounded-full px-6 py-3 font-semibold transition-colors cursor-pointer focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-yellow-400"
              :class="enCours ? 'bg-yellow-400 text-black' : 'bg-white text-black hover:bg-gray-200'"
              @click="ecouter"
            >
              <font-awesome-icon :icon="['fas', enCours ? 'volume-high' : 'play']" />
              {{ enCours ? 'En cours d’écoute' : 'Écouter' }}
            </button>
          </div>
        </header>

        <div class="mb-8">
          <MediaReactionsBar
            type-media="programme_radio"
            :media-id="emission.id"
            :nombre-likes="emission.interactions?.nombre_likes ?? 0"
            :nombre-dislikes="emission.interactions?.nombre_dislikes ?? 0"
            :ma-reaction="emission.interactions?.ma_reaction ?? null"
            :nombre-commentaires="nombreCommentaires || (emission.interactions?.nombre_commentaires ?? 0)"
            :nombre-partages="emission.interactions?.nombre_partages ?? 0"
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

          <!-- Signaler ce contenu (US7, FR-049) -->
          <span class="mt-4 sm:ml-3 inline-flex align-middle">
            <MediaSignalerBouton
              type-media="programme_radio"
              :media-id="emission.id"
              :titre="emission.title"
              variante="pilule"
            />
          </span>
          <span class="mt-4 sm:ml-3 inline-flex align-middle">
            <EngagementOffrirCadeauBouton
              type-objet="programme_radio"
              :objet-id="emission.id"
              :destinataire="emission.title"
              @offert="cadeauxRef?.rafraichir()"
            />
          </span>
        </div>

        <p v-if="emission.description" class="text-gray-300 leading-relaxed whitespace-pre-line mb-4">
          {{ emission.description }}
        </p>

        <dl v-if="emission.animator || emission.producer" class="text-sm text-gray-400 space-y-1 mb-10">
          <div v-if="emission.animator" class="flex gap-2">
            <dt class="font-semibold text-gray-300">Animation :</dt>
            <dd>{{ emission.animator }}</dd>
          </div>
          <div v-if="emission.producer" class="flex gap-2">
            <dt class="font-semibold text-gray-300">Production :</dt>
            <dd>{{ emission.producer }}</dd>
          </div>
        </dl>

        <!-- Cadeaux reçus par ce support (fond sombre : variante claire) -->
        <div class="mb-10">
          <EngagementCadeauxRecus
            ref="cadeauxRef"
            sombre
            type-objet="programme_radio"
            :objet-id="emission.id"
          />
        </div>

        <MediaCommentaires
          sombre
          type-media="programme_radio"
          :media-id="emission.id"
          @require-login="redirigerVersConnexion()"
          @total="nombreCommentaires = $event"
        />
      </div>

      <MediaProposerMediaModal
        :is-open="propositionOuverte"
        :types-offerts="['programme_radio']"
        :target-id="emission.stationId ?? undefined"
        @close="propositionOuverte = false"
      />

      <MediaPartagerModal
        :is-open="showPartage"
        :titre="emission.title"
        type-media="programme_radio"
        :media-id="emission.id"
        :url-detail="`/medias/programmes-radio/${slug}`"
        @close="showPartage = false"
      />
    </template>
  </div>
</template>
