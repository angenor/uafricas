<script setup lang="ts">

definePageMeta({ layout: false })
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

const { data: detail, pending: chargement } = await useAsyncData(
  `episode-radio-${slug}`,
  () => obtenirProgrammeRadioParSlug(slug),
)

/**
 * Cette page est désormais la page d'un **ÉPISODE**, son emplacement et son
 * slug sont conservés, ce qui préserve les adresses publiques déjà indexées
 * (FR-056). Ce qui change, c'est qu'elle nomme la série à laquelle il appartient
 * et propose les autres épisodes (US1 §4). La variable garde son nom `emission`
 * pour ne pas réécrire tout le gabarit.
 */
const emission = computed(() => detail.value?.episode ?? null)
const voisins = computed(() => detail.value?.voisins ?? [])

/** Adresse du PROGRAMME auquel l'épisode appartient. */
const lienProgramme = computed(() =>
  emission.value?.emissionSlug ? `/medias/emissions-radio/${emission.value.emissionSlug}` : null,
)

const showPartage = ref(false)
const propositionOuverte = ref(false)
const nombreCommentaires = ref(0)

// Rechargé après qu'un cadeau vient d'être offert.
const cadeauxRef = ref<{ rafraichir: () => void } | null>(null)

const lienStation = computed(() =>
  emission.value?.stationSlug ? `/medias/stations/${emission.value.stationSlug}` : null,
)

const filAriane = computed(() => [
  { libelle: 'Médias', vers: '/medias' },
  { libelle: 'Radio', vers: '/medias/radios' },
  { libelle: emission.value?.title || 'Émission' },
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
  const titre = `${emission.value.title}, Radio | UAfricas`
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
    type: 'episode_radio',
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
  <NuxtLayout name="africans">
    <template #fil-ariane>
      <AfricansFilAriane :segments="filAriane" />
    </template>

    <div v-if="chargement" class="flex items-center justify-center py-24">
      <div class="animate-spin rounded-full h-12 w-12 text-3xl text-af-chocolat"></div>
    </div>

    <!-- Un contenu retiré est indiscernable d'un contenu inexistant (FR-028). -->
    <div v-else-if="!emission" class="flex flex-col items-center justify-center py-24 px-4 text-center">
      <font-awesome-icon :icon="['fas', 'microphone']" class="w-14 h-14 text-af-atone-2 mb-4" />
      <h1 class="text-2xl font-bold text-af-encre mb-2">Émission introuvable</h1>
      <p class="text-af-corps mb-4">
        Cette émission n’existe pas, ou elle a été retirée de l’antenne.
      </p>
      <NuxtLink to="/medias/radios" class="font-bold text-af-chocolat hover:underline">
        &#8592; Retour à la radio
      </NuxtLink>
    </div>

    <template v-else>
      <div class="flex flex-col gap-6">

        <!-- Identité et écoute -->
        <header class="flex flex-col sm:flex-row gap-6 mb-8">
          <div class="w-full sm:w-64 aspect-square rounded-2xl overflow-hidden bg-af-fond shrink-0">
            <img
              v-if="emission.cover"
              :src="emission.cover"
              :alt="emission.title"
              class="w-full h-full object-cover"
            >
            <span v-else class="w-full h-full flex items-center justify-center">
              <font-awesome-icon :icon="['fas', 'microphone']" class="text-4xl text-af-atone-2" />
            </span>
          </div>

          <div class="min-w-0 flex flex-col justify-center">
            <!-- La série AVANT l'épisode : le visiteur doit savoir ce qu'il
                 écoute avant d'en connaître le numéro (US1 §4). -->
            <NuxtLink
              v-if="lienProgramme"
              :to="lienProgramme"
              class="inline-flex items-center gap-2 text-af-chocolat text-sm font-semibold hover:underline mb-2 self-start"
            >
              <font-awesome-icon :icon="['fas', 'layer-group']" class="w-3.5 h-3.5" />
              {{ emission.emissionTitre }}
            </NuxtLink>
            <h1 class="font-oswald text-3xl sm:text-4xl font-bold text-af-encre mb-2">
              <span v-if="emission.numeroEpisode" class="text-af-corps font-normal">
                Épisode {{ emission.numeroEpisode }}, 
              </span>
              {{ emission.title }}
            </h1>
            <p class="text-af-corps text-sm flex flex-wrap items-center gap-x-3 gap-y-1 mb-5">
              <NuxtLink v-if="lienStation" :to="lienStation" class="hover:text-af-chocolat">
                {{ emission.stationNom }}
              </NuxtLink>
              <span v-else-if="emission.stationNom">{{ emission.stationNom }}</span>
              <span v-if="emission.themePhare">· {{ emission.themePhare }}</span>
            </p>

            <button
              type="button"
              class="self-start inline-flex items-center gap-2 rounded-full px-6 py-3 font-semibold transition-colors cursor-pointer focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-af-chocolat"
              :class="enCours ? 'bg-af-chocolat text-black' : 'bg-white text-black hover:bg-af-bordure'"
              @click="ecouter"
            >
              <font-awesome-icon :icon="['fas', enCours ? 'volume-high' : 'play']" />
              {{ enCours ? 'En cours d’écoute' : 'Écouter' }}
            </button>
          </div>
        </header>

        <div class="mb-8">
          <MediaReactionsBar
            type-media="episode_radio"
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
            class="mt-4 inline-flex items-center gap-2 rounded-full border border-white/25 bg-af-fond text-af-encre px-5 py-2 text-sm font-semibold hover:bg-white/20 transition-colors cursor-pointer"
            @click="propositionOuverte = true"
          >
            <font-awesome-icon :icon="['fas', 'plus']" class="w-4 h-4" />
            Proposer un contenu
          </button>

          <!-- Signaler ce contenu (US7, FR-049) -->
          <span class="mt-4 sm:ml-3 inline-flex align-middle">
            <MediaSignalerBouton
              type-media="episode_radio"
              :media-id="emission.id"
              :titre="emission.title"
              variante="pilule"
            />
          </span>
          <span class="mt-4 sm:ml-3 inline-flex align-middle">
            <EngagementOffrirCadeauBouton
              type-objet="episode_radio"
              :objet-id="emission.id"
              :destinataire="emission.title"
              @offert="cadeauxRef?.rafraichir()"
            />
          </span>
        </div>

        <p v-if="emission.description" class="text-af-corps leading-relaxed whitespace-pre-line mb-4">
          {{ emission.description }}
        </p>

        <dl v-if="emission.animator || emission.producer" class="text-sm text-af-corps space-y-1 mb-10">
          <div v-if="emission.animator" class="flex gap-2">
            <dt class="font-semibold text-af-corps">Animation :</dt>
            <dd>{{ emission.animator }}</dd>
          </div>
          <div v-if="emission.producer" class="flex gap-2">
            <dt class="font-semibold text-af-corps">Production :</dt>
            <dd>{{ emission.producer }}</dd>
          </div>
        </dl>

        <!-- Cadeaux reçus par ce support (fond sombre : variante claire) -->
        <div class="mb-10">
          <EngagementCadeauxRecus
            ref="cadeauxRef"
            sombre
            type-objet="episode_radio"
            :objet-id="emission.id"
          />
        </div>

        <MediaCommentaires
          sombre
          type-media="episode_radio"
          :media-id="emission.id"
          @require-login="redirigerVersConnexion()"
          @total="nombreCommentaires = $event"
        />
      
        <!-- Les autres épisodes du même programme (US1 §4) : c'est ce qui rend
             la série navigable depuis n'importe lequel de ses épisodes. -->
        <section v-if="voisins.length" class="mt-10">
          <div class="flex items-baseline justify-between gap-4 mb-4">
            <h2 class="font-oswald text-xl font-bold text-af-encre">
              Autres épisodes
            </h2>
            <NuxtLink
              v-if="lienProgramme"
              :to="lienProgramme"
              class="text-af-chocolat text-sm hover:underline"
            >
              Voir le programme
            </NuxtLink>
          </div>
          <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-4">
            <NuxtLink
              v-for="voisin in voisins"
              :key="voisin.id"
              :to="`/medias/programmes-radio/${voisin.slug}`"
              class="group block rounded-xl overflow-hidden bg-af-fond border border-af-bordure hover:border-af-chocolat/60 transition-colors"
            >
              <div class="aspect-video bg-af-fond overflow-hidden">
                <img
                  v-if="voisin.cover"
                  :src="voisin.cover"
                  :alt="voisin.title"
                  loading="lazy"
                  class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300"
                >
              </div>
              <p class="p-3 text-sm text-af-encre line-clamp-2">{{ voisin.title }}</p>
            </NuxtLink>
          </div>
        </section>
</div>

      <MediaProposerMediaModal
        :is-open="propositionOuverte"
        :types-offerts="['episode_radio']"
        :target-id="emission.stationId ?? undefined"
        @close="propositionOuverte = false"
      />

      <MediaPartagerModal
        :is-open="showPartage"
        :titre="emission.title"
        type-media="episode_radio"
        :media-id="emission.id"
        :url-detail="`/medias/programmes-radio/${slug}`"
        @close="showPartage = false"
      />
    </template>
  </NuxtLayout>
</template>
