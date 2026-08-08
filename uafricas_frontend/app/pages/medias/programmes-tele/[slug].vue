<script setup lang="ts">
/**
 * Page de détail d'une émission de télévision (US3).
 *
 * Elle existe d'abord pour le partage : un aperçu social (FR-026) exige une URL
 * propre au contenu, et les pages de liste en donneraient un identique pour
 * tous. `useAsyncData` est appelé au niveau racine — c'est ce qui rend la page
 * rendue côté serveur, donc lisible par les robots des réseaux sociaux.
 */
import { useTelevision } from '~/composables/useTelevision'

const route = useRoute()
const slug = route.params.slug as string

const { obtenirProgrammeParSlug } = useTelevision()
const { redirigerVersConnexion } = useAuth()

const { data: programme, pending: chargement } = await useAsyncData(
  `programme-tele-${slug}`,
  () => obtenirProgrammeParSlug(slug),
)

const showPartage = ref(false)
const propositionOuverte = ref(false)
const nombreCommentaires = ref(0)

// Rechargé après qu'un cadeau vient d'être offert.
const cadeauxRef = ref<{ rafraichir: () => void } | null>(null)

const lienChaine = computed(() =>
  programme.value?.chaineSlug ? `/medias/chaines/${programme.value.chaineSlug}` : null,
)

const breadcrumbs = computed(() => [
  { label: 'Médias', to: '/medias' },
  { label: 'Télévision', to: '/medias/tele' },
  { label: programme.value?.title || 'Émission', to: undefined },
])

// ── SEO / Open Graph (aperçu lors du partage sur les réseaux sociaux) ──
const requete = useRequestURL()
const urlCanonique = `${requete.protocol}//${requete.host}/medias/programmes-tele/${slug}`
const imageOg = computed(() => programme.value?.banner || '')
const descriptionOg = computed(() =>
  (programme.value?.description || `Émission ${programme.value?.title ?? ''}`).slice(0, 200),
)

useHead(() => {
  if (!programme.value) return {}
  const titre = `${programme.value.title} — Télévision — UAfricas`
  return {
    title: titre,
    meta: [
      { name: 'description', content: descriptionOg.value },
      { property: 'og:type', content: 'video.episode' },
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
  <div class="min-h-screen bg-neutral-950">
    <div v-if="chargement" class="flex items-center justify-center h-screen">
      <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-yellow-400"></div>
    </div>

    <!-- Un contenu retiré est indiscernable d'un contenu inexistant (FR-028) :
         le serveur renvoie 404 dans les deux cas, la page dit la même chose. -->
    <div v-else-if="!programme" class="flex flex-col items-center justify-center h-screen px-4 text-center">
      <font-awesome-icon :icon="['fas', 'tv']" class="w-14 h-14 text-neutral-700 mb-4" />
      <h1 class="text-2xl font-bold text-white mb-2">Émission introuvable</h1>
      <p class="text-gray-400 mb-4">
        Cette émission n’existe pas, ou elle a été retirée de l’antenne.
      </p>
      <NuxtLink to="/medias/tele" class="text-yellow-400 hover:underline">
        &#8592; Retour à la télévision
      </NuxtLink>
    </div>

    <template v-else>
      <div class="max-w-5xl mx-auto px-4 pt-24 pb-16">
        <!-- Fil d'Ariane -->
        <nav aria-label="Fil d'Ariane" class="mb-6 text-sm text-gray-400">
          <template v-for="(fil, i) in breadcrumbs" :key="i">
            <NuxtLink v-if="fil.to" :to="fil.to" class="hover:text-yellow-400">{{ fil.label }}</NuxtLink>
            <span v-else class="text-white">{{ fil.label }}</span>
            <span v-if="i < breadcrumbs.length - 1" class="mx-2">/</span>
          </template>
        </nav>

        <!-- Lecteur -->
        <div class="rounded-2xl overflow-hidden bg-black mb-8">
          <MediaLecteurMedia
            :url="programme.videoUrl"
            type="video"
            :titre="programme.title"
            :poster="programme.banner"
          />
        </div>

        <!-- Identité -->
        <header class="mb-6">
          <h1 class="font-oswald text-3xl sm:text-4xl font-bold text-white mb-2">
            {{ programme.title }}
          </h1>
          <p class="text-gray-400 text-sm flex flex-wrap items-center gap-x-3 gap-y-1">
            <NuxtLink v-if="lienChaine" :to="lienChaine" class="hover:text-yellow-400">
              {{ programme.chaineNom }}
            </NuxtLink>
            <span v-else-if="programme.chaineNom">{{ programme.chaineNom }}</span>
            <span v-if="programme.themePhare">· {{ programme.themePhare }}</span>
            <span v-if="programme.country">· {{ programme.country }}</span>
            <span v-if="programme.language">· {{ programme.language }}</span>
          </p>
        </header>

        <!-- Interactions -->
        <div class="mb-8">
          <MediaReactionsBar
            type-media="programme_tele"
            :media-id="programme.id"
            :nombre-likes="programme.interactions?.nombre_likes ?? 0"
            :nombre-dislikes="programme.interactions?.nombre_dislikes ?? 0"
            :ma-reaction="programme.interactions?.ma_reaction ?? null"
            :nombre-commentaires="nombreCommentaires || (programme.interactions?.nombre_commentaires ?? 0)"
            :nombre-partages="programme.interactions?.nombre_partages ?? 0"
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
              type-media="programme_tele"
              :media-id="programme.id"
              :titre="programme.title"
              variante="pilule"
            />
          </span>
          <span class="mt-4 sm:ml-3 inline-flex align-middle">
            <EngagementOffrirCadeauBouton
              type-objet="programme_tele"
              :objet-id="programme.id"
              :destinataire="programme.title"
              @offert="cadeauxRef?.rafraichir()"
            />
          </span>
        </div>

        <p v-if="programme.description" class="text-gray-300 leading-relaxed whitespace-pre-line mb-4">
          {{ programme.description }}
        </p>

        <dl v-if="programme.animator || programme.producer" class="text-sm text-gray-400 space-y-1 mb-10">
          <div v-if="programme.animator" class="flex gap-2">
            <dt class="font-semibold text-gray-300">Animation :</dt>
            <dd>{{ programme.animator }}</dd>
          </div>
          <div v-if="programme.producer" class="flex gap-2">
            <dt class="font-semibold text-gray-300">Production :</dt>
            <dd>{{ programme.producer }}</dd>
          </div>
        </dl>

        <!-- Cadeaux reçus par ce support (fond sombre : variante claire) -->
        <div class="mb-10">
          <EngagementCadeauxRecus
            ref="cadeauxRef"
            sombre
            type-objet="programme_tele"
            :objet-id="programme.id"
          />
        </div>

        <MediaCommentaires
          sombre
          type-media="programme_tele"
          :media-id="programme.id"
          @require-login="redirigerVersConnexion()"
          @total="nombreCommentaires = $event"
        />
      </div>

      <MediaProposerMediaModal
        :is-open="propositionOuverte"
        :types-offerts="['programme_tele']"
        :target-id="programme.chaineId ?? undefined"
        @close="propositionOuverte = false"
      />

      <MediaPartagerModal
        :is-open="showPartage"
        :titre="programme.title"
        type-media="programme_tele"
        :media-id="programme.id"
        :url-detail="`/medias/programmes-tele/${slug}`"
        @close="showPartage = false"
      />
    </template>
  </div>
</template>
