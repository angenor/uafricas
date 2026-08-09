<script setup lang="ts">
/**
 * Page d'un **programme** de télévision (feature 009, US1 §3 et US5).
 *
 * Elle existe pour deux raisons : donner une adresse propre à une série — un
 * aperçu social sur un épisode ne dit rien du programme — et porter la vie
 * communautaire du programme lui-même. Ses compteurs sont ceux du **programme
 * seul**, jamais la somme de ceux de ses épisodes (FR-048) : additionner les
 * deux ferait passer 40 commentaires d'épisodes pour un débat sur la série.
 *
 * `useAsyncData` au niveau racine ⇒ rendu côté serveur, donc lisible par les
 * robots des réseaux sociaux.
 *
 * Tailwind v4 pur — page publique, aucune classe daisyUI (principe VI).
 */
import { LIBELLES_CADENCE } from '~/composables/useMediaEmissions'

const route = useRoute()
const slug = route.params.slug as string

const { obtenirEmission } = useMediaEmissions()
const { redirigerVersConnexion } = useAuth()

const { data: emission, pending: chargement } = await useAsyncData(
  `emission-tele-${slug}`,
  () => obtenirEmission('chaine_tv', slug),
)

const showPartage = ref(false)
const nombreCommentaires = ref(0)

const lienChaine = computed(() =>
  emission.value?.support?.slug ? `/medias/chaines/${emission.value.support.slug}` : null,
)

const breadcrumbs = computed(() => [
  { label: 'Médias', to: '/medias' },
  { label: 'Télévision', to: '/medias/tele' },
  { label: emission.value?.titre || 'Programme', to: undefined },
])

// ── SEO / Open Graph ──────────────────────────────────────────────
const requete = useRequestURL()
const urlCanonique = `${requete.protocol}//${requete.host}/medias/emissions-tele/${slug}`
const imageOg = computed(() => emission.value?.image_couverture_url || '')
const descriptionOg = computed(() =>
  (emission.value?.description || `Programme ${emission.value?.titre ?? ''}`).slice(0, 200),
)

useHead(() => {
  if (!emission.value) return {}
  const titre = `${emission.value.titre} — Télévision — UAfricas`
  return {
    title: titre,
    meta: [
      { name: 'description', content: descriptionOg.value },
      { property: 'og:type', content: 'video.tv_show' },
      { property: 'og:title', content: titre },
      { property: 'og:description', content: descriptionOg.value },
      { property: 'og:url', content: urlCanonique },
      { property: 'og:site_name', content: 'UAfricas' },
      ...(imageOg.value ? [{ property: 'og:image', content: imageOg.value }] : []),
      { name: 'twitter:card', content: imageOg.value ? 'summary_large_image' : 'summary' },
      { name: 'twitter:title', content: titre },
      { name: 'twitter:description', content: descriptionOg.value },
    ],
    link: [{ rel: 'canonical', href: urlCanonique }],
  }
})
</script>

<template>
  <main class="min-h-screen bg-neutral-950">
    <div v-if="chargement" class="flex items-center justify-center h-screen">
      <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-yellow-400" />
    </div>

    <!-- Un programme retiré est indiscernable d'un programme inexistant. -->
    <div
      v-else-if="!emission"
      class="flex flex-col items-center justify-center h-screen px-4 text-center"
    >
      <font-awesome-icon :icon="['fas', 'tv']" class="w-14 h-14 text-neutral-700 mb-4" />
      <h1 class="text-2xl font-bold text-white mb-2">Programme introuvable</h1>
      <p class="text-gray-400 mb-4">
        Ce programme n'existe pas, n'est pas encore publié, ou n'a aucun épisode diffusé.
      </p>
      <NuxtLink to="/medias/tele" class="text-yellow-400 hover:underline">
        &#8592; Retour à la télévision
      </NuxtLink>
    </div>

    <div v-else class="max-w-5xl mx-auto px-4 sm:px-6 py-10">
      <CommonFilAriane :items="breadcrumbs" sombre class="mb-6" />

      <!-- Identité du programme -->
      <header class="flex flex-col sm:flex-row gap-6 mb-10">
        <div class="w-full sm:w-72 shrink-0 aspect-video rounded-xl overflow-hidden bg-neutral-900">
          <img
            v-if="emission.image_couverture_url"
            :src="emission.image_couverture_url"
            :alt="emission.titre"
            class="w-full h-full object-cover"
          >
          <span v-else class="w-full h-full flex items-center justify-center">
            <font-awesome-icon :icon="['fas', 'layer-group']" class="text-4xl text-neutral-700" />
          </span>
        </div>

        <div class="min-w-0 flex flex-col justify-center">
          <h1 class="font-oswald text-3xl sm:text-4xl font-bold text-white mb-2">
            {{ emission.titre }}
          </h1>
          <p class="text-gray-400 text-sm flex flex-wrap items-center gap-x-3 gap-y-1 mb-4">
            <NuxtLink v-if="lienChaine" :to="lienChaine" class="hover:text-yellow-400">
              {{ emission.support?.nom }}
            </NuxtLink>
            <span v-else-if="emission.support">{{ emission.support.nom }}</span>
            <span>· {{ emission.nombre_episodes }} épisode{{ emission.nombre_episodes > 1 ? 's' : '' }}</span>
            <span v-if="emission.cadence !== 'ponctuelle'">
              · {{ LIBELLES_CADENCE[emission.cadence] }}
            </span>
            <span v-if="emission.theme_phare">· {{ emission.theme_phare.nom }}</span>
          </p>
          <p v-if="emission.description" class="text-gray-300 text-sm">
            {{ emission.description }}
          </p>
          <p
            v-if="emission.info_animateur || emission.info_producteur"
            class="text-gray-500 text-xs mt-3"
          >
            <span v-if="emission.info_animateur">Animation : {{ emission.info_animateur }}</span>
            <span v-if="emission.info_animateur && emission.info_producteur"> · </span>
            <span v-if="emission.info_producteur">Production : {{ emission.info_producteur }}</span>
          </p>
        </div>
      </header>

      <!--
        Interactions du PROGRAMME. Le libellé le dit explicitement : sans lui, un
        visiteur croirait ces compteurs cumulés avec ceux des épisodes, alors
        qu'ils ne le sont jamais (FR-048).
      -->
      <div class="mb-4">
        <p class="text-xs uppercase tracking-wide text-gray-500 mb-2">
          Réactions au programme
        </p>
        <MediaReactionsBar
          type-media="emission_tele"
          :media-id="emission.id"
          :nombre-likes="emission.interactions?.nombre_likes ?? 0"
          :nombre-dislikes="emission.interactions?.nombre_dislikes ?? 0"
          :ma-reaction="emission.interactions?.ma_reaction ?? null"
          :nombre-commentaires="nombreCommentaires || (emission.interactions?.nombre_commentaires ?? 0)"
          :nombre-partages="emission.interactions?.nombre_partages ?? 0"
          @require-login="redirigerVersConnexion()"
          @partager="showPartage = true"
        />
        <div class="mt-3">
          <MediaSignalerBouton
            type-media="emission_tele"
            :media-id="emission.id"
            :titre="emission.titre"
            variante="pilule"
          />
        </div>
      </div>

      <!-- Cadeaux reçus par ce programme -->
      <div class="mb-10">
        <EngagementCadeauxRecus sombre type-objet="emission_tele" :objet-id="emission.id" />
      </div>

      <!-- Les épisodes, paginés (SC-009) : leurs compteurs sont les leurs. -->
      <div class="mb-12">
        <MediaListeEpisodes
          type-support="chaine_tv"
          :emission-id="emission.id"
          :initiaux="emission.episodes_apercu ?? []"
          :total="emission.nombre_episodes"
        />
      </div>

      <MediaCommentaires
        sombre
        type-media="emission_tele"
        :media-id="emission.id"
        @require-login="redirigerVersConnexion()"
        @total="nombreCommentaires = $event"
      />

      <MediaPartagerModal
        :is-open="showPartage"
        :titre="emission.titre"
        type-media="emission_tele"
        :media-id="emission.id"
        :url-detail="`/medias/emissions-tele/${slug}`"
        @close="showPartage = false"
      />
    </div>
  </main>
</template>
