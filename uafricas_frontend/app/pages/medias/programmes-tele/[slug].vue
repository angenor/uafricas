<script setup lang="ts">

definePageMeta({ layout: false })
/**
 * Page de détail d'une émission de télévision (US3).
 *
 * Elle existe d'abord pour le partage : un aperçu social (FR-026) exige une URL
 * propre au contenu, et les pages de liste en donneraient un identique pour
 * tous. `useAsyncData` est appelé au niveau racine : c'est ce qui rend la page
 * rendue côté serveur, donc lisible par les robots des réseaux sociaux.
 */
import { useTelevision } from '~/composables/useTelevision'

const route = useRoute()
const slug = route.params.slug as string

const { obtenirProgrammeParSlug } = useTelevision()
const { redirigerVersConnexion } = useAuth()

const { data: detail, pending: chargement } = await useAsyncData(
  `episode-tele-${slug}`,
  () => obtenirProgrammeParSlug(slug),
)

/**
 * Cette page est désormais la page d'un **ÉPISODE**, son emplacement et son
 * slug sont conservés, ce qui préserve les adresses publiques déjà indexées
 * (FR-056). Ce qui change, c'est qu'elle nomme la série à laquelle il
 * appartient et propose les autres épisodes (US1 §4).
 */
const programme = computed(() => detail.value?.episode ?? null)
const voisins = computed(() => detail.value?.voisins ?? [])

/** Adresse du PROGRAMME auquel l'épisode appartient (US1 §4). */
const lienProgramme = computed(() =>
  programme.value?.emissionSlug ? `/medias/emissions-tele/${programme.value.emissionSlug}` : null,
)

const showPartage = ref(false)
const propositionOuverte = ref(false)
const nombreCommentaires = ref(0)

// Rechargé après qu'un cadeau vient d'être offert.
const cadeauxRef = ref<{ rafraichir: () => void } | null>(null)

const lienChaine = computed(() =>
  programme.value?.chaineSlug ? `/medias/chaines/${programme.value.chaineSlug}` : null,
)

const filAriane = computed(() => [
  { libelle: 'Médias', vers: '/medias' },
  { libelle: 'Télévision', vers: '/medias/tele' },
  { libelle: programme.value?.title || 'Émission' },
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
  const titre = `${programme.value.title}, Télévision | UAfricas`
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
  <NuxtLayout name="africans">
    <template #fil-ariane>
      <AfricansFilAriane :segments="filAriane" />
    </template>

    <div v-if="chargement" class="flex items-center justify-center py-24">
      <div class="animate-spin rounded-full h-12 w-12 text-3xl text-af-chocolat"></div>
    </div>

    <!-- Un contenu retiré est indiscernable d'un contenu inexistant (FR-028) :
         le serveur renvoie 404 dans les deux cas, la page dit la même chose. -->
    <div v-else-if="!programme" class="flex flex-col items-center justify-center py-24 px-4 text-center">
      <font-awesome-icon :icon="['fas', 'tv']" class="w-14 h-14 text-af-atone-2 mb-4" />
      <h1 class="text-2xl font-bold text-af-encre mb-2">Émission introuvable</h1>
      <p class="text-af-corps mb-4">
        Cette émission n’existe pas, ou elle a été retirée de l’antenne.
      </p>
      <NuxtLink to="/medias/tele" class="font-bold text-af-chocolat hover:underline">
        &#8592; Retour à la télévision
      </NuxtLink>
    </div>

    <template v-else>
      <div class="flex flex-col gap-6">
        <!-- Fil d'Ariane -->

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
          <!-- La série AVANT l'épisode : le visiteur doit savoir ce qu'il
               regarde avant d'en connaître le numéro (US1 §4). -->
          <NuxtLink
            v-if="lienProgramme"
            :to="lienProgramme"
            class="inline-flex items-center gap-2 text-af-chocolat text-sm font-semibold hover:underline mb-2"
          >
            <font-awesome-icon :icon="['fas', 'layer-group']" class="w-3.5 h-3.5" />
            {{ programme.emissionTitre }}
          </NuxtLink>
          <h1 class="font-oswald text-3xl sm:text-4xl font-bold text-af-encre mb-2">
            <span v-if="programme.numeroEpisode" class="text-af-corps font-normal">
              Épisode {{ programme.numeroEpisode }}, 
            </span>
            {{ programme.title }}
          </h1>
          <p class="text-af-corps text-sm flex flex-wrap items-center gap-x-3 gap-y-1">
            <NuxtLink v-if="lienChaine" :to="lienChaine" class="hover:text-af-chocolat">
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
            type-media="episode_tele"
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
            class="mt-4 inline-flex items-center gap-2 rounded-full border border-white/25 bg-af-fond text-af-encre px-5 py-2 text-sm font-semibold hover:bg-white/20 transition-colors cursor-pointer"
            @click="propositionOuverte = true"
          >
            <font-awesome-icon :icon="['fas', 'plus']" class="w-4 h-4" />
            Proposer un contenu
          </button>

          <!-- Signaler ce contenu (US7, FR-049) -->
          <span class="mt-4 sm:ml-3 inline-flex align-middle">
            <MediaSignalerBouton
              type-media="episode_tele"
              :media-id="programme.id"
              :titre="programme.title"
              variante="pilule"
            />
          </span>
          <span class="mt-4 sm:ml-3 inline-flex align-middle">
            <EngagementOffrirCadeauBouton
              type-objet="episode_tele"
              :objet-id="programme.id"
              :destinataire="programme.title"
              @offert="cadeauxRef?.rafraichir()"
            />
          </span>
        </div>

        <p v-if="programme.description" class="text-af-corps leading-relaxed whitespace-pre-line mb-4">
          {{ programme.description }}
        </p>

        <dl v-if="programme.animator || programme.producer" class="text-sm text-af-corps space-y-1 mb-10">
          <div v-if="programme.animator" class="flex gap-2">
            <dt class="font-semibold text-af-corps">Animation :</dt>
            <dd>{{ programme.animator }}</dd>
          </div>
          <div v-if="programme.producer" class="flex gap-2">
            <dt class="font-semibold text-af-corps">Production :</dt>
            <dd>{{ programme.producer }}</dd>
          </div>
        </dl>

        <!-- Cadeaux reçus par ce support (fond sombre : variante claire) -->
        <div class="mb-10">
          <EngagementCadeauxRecus
            ref="cadeauxRef"
            sombre
            type-objet="episode_tele"
            :objet-id="programme.id"
          />
        </div>

        <MediaCommentaires
          sombre
          type-media="episode_tele"
          :media-id="programme.id"
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
              :to="`/medias/programmes-tele/${voisin.slug}`"
              class="group block rounded-xl overflow-hidden bg-af-fond border border-af-bordure hover:border-af-chocolat/60 transition-colors"
            >
              <div class="aspect-video bg-af-fond overflow-hidden">
                <img
                  v-if="voisin.banner"
                  :src="voisin.banner"
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
        :types-offerts="['episode_tele']"
        :target-id="programme.chaineId ?? undefined"
        @close="propositionOuverte = false"
      />

      <MediaPartagerModal
        :is-open="showPartage"
        :titre="programme.title"
        type-media="episode_tele"
        :media-id="programme.id"
        :url-detail="`/medias/programmes-tele/${slug}`"
        @close="showPartage = false"
      />
    </template>
  </NuxtLayout>
</template>
