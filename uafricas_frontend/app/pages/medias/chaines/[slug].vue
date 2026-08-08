<script setup lang="ts">
/**
 * Page de détail d'une chaîne de télévision (US3) : son identité, ses contenus
 * et ses interactions. Rendue côté serveur pour que l'aperçu social porte le
 * nom de la chaîne et non celui de la page de liste (FR-026).
 */
import { useTelevision, type TvProgram } from '~/composables/useTelevision'

const route = useRoute()
const slug = route.params.slug as string

const { obtenirChaineParSlug, listerProgrammesVedettes } = useTelevision()
const { redirigerVersConnexion } = useAuth()

const { data: chaine, pending: chargement } = await useAsyncData(
  `chaine-${slug}`,
  () => obtenirChaineParSlug(slug),
)

// Les contenus ne conditionnent ni le rendu serveur ni l'aperçu social : ils
// sont chargés après coup, sans bloquer l'affichage de la fiche.
const contenus = ref<TvProgram[]>([])
onMounted(async () => {
  if (!chaine.value) return
  const res = await listerProgrammesVedettes({ chaine: chaine.value.id, par_page: 50 })
  contenus.value = res?.programmes ?? []
})

const showPartage = ref(false)
const propositionOuverte = ref(false)
// Mise en relation avec l'equipe du support (US6, FR-046)
const contactOuvert = ref(false)
const nombreCommentaires = ref(0)

// Rechargé après qu'un cadeau vient d'être offert.
const cadeauxRef = ref<{ rafraichir: () => void } | null>(null)

const breadcrumbs = computed(() => [
  { label: 'Médias', to: '/medias' },
  { label: 'Télévision', to: '/medias/tele' },
  { label: chaine.value?.name || 'Chaîne', to: undefined },
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
  const titre = `${chaine.value.name} — Chaîne de télévision — UAfricas`
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
  <div class="min-h-screen bg-neutral-950">
    <div v-if="chargement" class="flex items-center justify-center h-screen">
      <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-yellow-400"></div>
    </div>

    <!-- Un contenu retiré est indiscernable d'un contenu inexistant (FR-028). -->
    <div v-else-if="!chaine" class="flex flex-col items-center justify-center h-screen px-4 text-center">
      <font-awesome-icon :icon="['fas', 'tv']" class="w-14 h-14 text-neutral-700 mb-4" />
      <h1 class="text-2xl font-bold text-white mb-2">Chaîne introuvable</h1>
      <p class="text-gray-400 mb-4">
        Cette chaîne n’existe pas, ou elle a été retirée de l’antenne.
      </p>
      <NuxtLink to="/medias/tele" class="text-yellow-400 hover:underline">
        &#8592; Retour à la télévision
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
            <img v-if="chaine.cover" :src="chaine.cover" :alt="chaine.name" class="w-full h-full object-cover">
            <span v-else class="w-full h-full flex items-center justify-center">
              <font-awesome-icon :icon="['fas', 'tv']" class="text-2xl text-neutral-700" />
            </span>
          </div>

          <div class="min-w-0">
            <div class="flex items-center gap-3 flex-wrap mb-2">
              <h1 class="font-oswald text-3xl sm:text-4xl font-bold text-white">{{ chaine.name }}</h1>
              <span
                v-if="chaine.isLive"
                class="rounded-full bg-red-600 text-white text-xs font-bold px-2.5 py-0.5 uppercase"
              >
                En direct
              </span>
            </div>
            <p class="text-gray-400 text-sm flex flex-wrap items-center gap-x-3 gap-y-1">
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

        <p v-if="chaine.description" class="text-gray-300 leading-relaxed whitespace-pre-line mb-10">
          {{ chaine.description }}
        </p>

        <!-- Coordonnées publiques renseignées par l'équipe de la chaîne (09p) -->
        <MediaBlocContacts :contacts="chaine.contacts" :nom-support="chaine.name" />

        <!-- Contenus de la chaîne -->
        <section v-if="contenus.length" class="mb-12">
          <h2 class="font-oswald text-xl font-bold text-white mb-4">Ses émissions</h2>
          <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-4">
            <NuxtLink
              v-for="contenu in contenus"
              :key="contenu.id"
              :to="contenu.slug ? `/medias/programmes-tele/${contenu.slug}` : ''"
              class="group block"
            >
              <div class="aspect-video rounded-lg overflow-hidden bg-neutral-900">
                <img
                  v-if="contenu.banner"
                  :src="contenu.banner"
                  :alt="contenu.title"
                  loading="lazy"
                  class="w-full h-full object-cover transition-transform duration-300 group-hover:scale-105"
                >
                <span v-else class="w-full h-full flex items-center justify-center">
                  <font-awesome-icon :icon="['fas', 'image']" class="text-neutral-700" />
                </span>
              </div>
              <p class="mt-2 text-sm font-semibold text-white truncate group-hover:text-yellow-400 transition-colors">
                {{ contenu.title }}
              </p>
            </NuxtLink>
          </div>
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
        :types-offerts="['programme_tele']"
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
  </div>
</template>
