<template>
  <div class="min-h-screen bg-gray-50">
    <div v-if="chargement" class="flex items-center justify-center h-screen">
      <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-custom-chocolat"></div>
    </div>

    <div v-else-if="!site" class="flex flex-col items-center justify-center h-screen px-4 text-center">
      <font-awesome-icon :icon="['fas', 'location-dot']" class="w-14 h-14 text-gray-300 mb-4" />
      <h1 class="text-2xl font-bold text-gray-700 mb-2">Site introuvable</h1>
      <p class="text-gray-500 mb-4">Ce site touristique n'existe pas ou a été retiré.</p>
      <NuxtLink :to="`/opportunite-afrique/${ficheId}`" class="text-custom-chocolat hover:underline">
        &#8592; Retour au territoire
      </NuxtLink>
    </div>

    <template v-else>
      <!-- Hero immersif -->
      <OpportuniteAfriqueDetailHero
        :titre="site.nom"
        :sous-titre="localisation"
        :image="galerie.length ? resoudreUrlImage(galerie[0]!) : null"
        :breadcrumbs="breadcrumbs"
        degrade-fond="bg-gradient-to-br from-custom-chocolat to-amber-900"
      >
        <template #badges>
          <span v-if="libelleSousType" class="inline-flex items-center gap-1.5 rounded-full bg-custom-chocolat px-3 py-1 text-sm font-medium text-white shadow-sm">
            <font-awesome-icon :icon="['fas', 'location-dot']" class="w-3.5 h-3.5" />
            {{ libelleSousType }}
          </span>
          <span
            v-if="site.verifie"
            class="inline-flex items-center gap-1.5 rounded-full bg-custom-green px-3 py-1 text-sm font-medium text-white shadow-sm"
          >
            <font-awesome-icon :icon="['fas', 'circle-check']" class="w-3.5 h-3.5" />
            Vérifié
          </span>
          <span
            v-if="site.nombre_avis > 0 && site.note_moyenne !== null"
            class="inline-flex items-center gap-1.5 rounded-full bg-white/90 px-3 py-1 text-sm font-semibold text-gray-900 shadow-sm"
          >
            <font-awesome-icon :icon="['fas', 'star']" class="w-3.5 h-3.5 text-amber-400" />
            {{ site.note_moyenne.toFixed(1) }}
            <span class="font-normal text-gray-500">({{ site.nombre_avis }})</span>
          </span>
        </template>
        <template #sous-titre-icon>
          <font-awesome-icon :icon="['fas', 'location-dot']" class="w-4 h-4 text-custom-chocolat" />
        </template>
      </OpportuniteAfriqueDetailHero>

      <!-- Corps chevauchant le hero -->
      <div class="relative z-10 mx-auto -mt-10 max-w-5xl px-4 pb-16 sm:px-6 lg:px-8">
        <div class="grid grid-cols-1 gap-6 lg:grid-cols-3">
          <!-- Colonne principale -->
          <div class="space-y-6 lg:col-span-2">
            <!-- Bandeau de suspension -->
            <div
              v-if="site.suspendu"
              class="flex items-start gap-2 rounded-xl border border-orange-200 bg-orange-50 px-4 py-3 text-sm text-orange-800 shadow-sm"
            >
              <font-awesome-icon :icon="['fas', 'triangle-exclamation']" class="w-4 h-4 mt-0.5 shrink-0" />
              <span>Contribution suspendue — en cours de vérification par la modération.</span>
            </div>

            <!-- Galerie -->
            <section v-if="imageCourante" class="overflow-hidden rounded-2xl border border-gray-100 bg-white shadow-sm">
              <div class="relative aspect-video bg-gray-100">
                <img :src="resoudreUrlImage(imageCourante)" :alt="site.nom" class="h-full w-full object-contain" />
                <button
                  v-if="galerie.length > 1"
                  type="button"
                  class="absolute left-2 top-1/2 -translate-y-1/2 rounded-full bg-black/45 p-2 text-white hover:bg-black/65"
                  aria-label="Image précédente"
                  @click="naviguer(-1)"
                >
                  <font-awesome-icon :icon="['fas', 'chevron-left']" class="w-4 h-4" />
                </button>
                <button
                  v-if="galerie.length > 1"
                  type="button"
                  class="absolute right-2 top-1/2 -translate-y-1/2 rounded-full bg-black/45 p-2 text-white hover:bg-black/65"
                  aria-label="Image suivante"
                  @click="naviguer(1)"
                >
                  <font-awesome-icon :icon="['fas', 'chevron-right']" class="w-4 h-4" />
                </button>
                <span
                  v-if="galerie.length > 1"
                  class="absolute bottom-2 right-2 rounded bg-black/55 px-2 py-0.5 text-xs font-medium text-white"
                >
                  {{ indexCourant + 1 }} / {{ galerie.length }}
                </span>
              </div>

              <!-- Miniatures -->
              <div v-if="galerie.length > 1" class="flex flex-wrap gap-1.5 p-3">
                <button
                  v-for="(url, i) in galerie"
                  :key="url"
                  type="button"
                  class="h-14 w-20 overflow-hidden rounded border transition-opacity"
                  :class="i === indexCourant ? 'border-custom-chocolat opacity-100' : 'border-gray-200 opacity-70 hover:opacity-100'"
                  @click="indexCourant = i"
                >
                  <img :src="resoudreUrlImage(url)" :alt="`${site.nom} ${i + 1}`" class="h-full w-full object-cover" />
                </button>
              </div>
            </section>

            <!-- Description -->
            <section v-if="site.info_pertinente || site.description" class="rounded-2xl border border-gray-100 bg-white p-6 shadow-sm sm:p-8">
              <h2 class="mb-4 flex items-center gap-2 font-oswald text-xl font-bold text-gray-900">
                <span class="flex h-8 w-8 items-center justify-center rounded-lg bg-custom-chocolat/10 text-custom-chocolat">
                  <font-awesome-icon :icon="['fas', 'circle-info']" class="w-4 h-4" />
                </span>
                À propos de ce lieu
              </h2>
              <div class="space-y-3 leading-relaxed text-gray-700">
                <p v-if="site.info_pertinente">{{ site.info_pertinente }}</p>
                <p v-if="site.description" class="text-gray-600">{{ site.description }}</p>
              </div>
            </section>

            <!-- Avis -->
            <section class="rounded-2xl border border-gray-100 bg-white p-6 shadow-sm sm:p-8">
              <OpportuniteAfriqueSiteAvisListe :site-id="site.id" :est-authentifie="userStore.isAuthenticated" />
            </section>
          </div>

          <!-- Sidebar -->
          <aside class="space-y-6">
            <!-- Informations pratiques -->
            <section
              v-if="localisation || site.gestionnaire || aGps || site.site_web_url"
              class="rounded-2xl border border-gray-100 bg-white p-6 shadow-sm"
            >
              <h2 class="mb-4 text-xs font-semibold uppercase tracking-wide text-gray-500">Informations pratiques</h2>
              <div class="space-y-3 text-sm">
                <div v-if="localisation" class="flex items-start gap-3">
                  <span class="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-custom-chocolat/10 text-custom-chocolat">
                    <font-awesome-icon :icon="['fas', 'location-dot']" class="w-4 h-4" />
                  </span>
                  <div>
                    <p class="text-xs text-gray-500">Localisation</p>
                    <p class="font-medium text-gray-900">{{ localisation }}</p>
                  </div>
                </div>
                <div v-if="site.gestionnaire" class="flex items-start gap-3">
                  <span class="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-custom-chocolat/10 text-custom-chocolat">
                    <font-awesome-icon :icon="['fas', 'user']" class="w-4 h-4" />
                  </span>
                  <div>
                    <p class="text-xs text-gray-500">Gestionnaire</p>
                    <p class="font-medium text-gray-900">{{ site.gestionnaire }}</p>
                  </div>
                </div>
                <div v-if="aGps" class="flex items-start gap-3">
                  <span class="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-custom-chocolat/10 text-custom-chocolat">
                    <font-awesome-icon :icon="['fas', 'map-pin']" class="w-4 h-4" />
                  </span>
                  <div>
                    <p class="text-xs text-gray-500">Coordonnées GPS</p>
                    <p class="font-medium text-gray-900">{{ site.latitude!.toFixed(4) }}, {{ site.longitude!.toFixed(4) }}</p>
                  </div>
                </div>
              </div>

              <a
                v-if="site.site_web_url"
                :href="site.site_web_url"
                target="_blank"
                rel="noopener noreferrer"
                class="mt-4 inline-flex w-full items-center justify-center gap-2 rounded-lg bg-custom-chocolat px-4 py-2.5 text-sm font-medium text-white transition hover:bg-custom-chocolat/90"
              >
                <font-awesome-icon :icon="['fas', 'arrow-up-right-from-square']" class="w-3.5 h-3.5" />
                Visiter le site web
              </a>
            </section>

            <!-- Contacts -->
            <section v-if="aContact" class="rounded-2xl border border-gray-100 bg-white p-6 shadow-sm">
              <h2 class="mb-4 text-xs font-semibold uppercase tracking-wide text-gray-500">Contacts</h2>
              <div class="space-y-3 text-sm">
                <a v-if="site.contact_telephone" :href="`tel:${site.contact_telephone}`" class="flex items-center gap-3 text-gray-700 hover:text-custom-green">
                  <span class="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-custom-green/10 text-custom-green">
                    <font-awesome-icon :icon="['fas', 'phone']" class="w-4 h-4" />
                  </span>
                  {{ site.contact_telephone }}
                </a>
                <a v-if="site.contact_courriel" :href="`mailto:${site.contact_courriel}`" class="flex items-center gap-3 break-all text-gray-700 hover:text-custom-green">
                  <span class="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-custom-green/10 text-custom-green">
                    <font-awesome-icon :icon="['fas', 'envelope']" class="w-4 h-4" />
                  </span>
                  {{ site.contact_courriel }}
                </a>
                <div v-if="site.contact_adresse" class="flex items-center gap-3 text-gray-700">
                  <span class="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-custom-green/10 text-custom-green">
                    <font-awesome-icon :icon="['fas', 'location-dot']" class="w-4 h-4" />
                  </span>
                  {{ site.contact_adresse }}
                </div>
              </div>
            </section>

            <!-- Constitution légale -->
            <section v-if="aConstitution" class="rounded-2xl border border-gray-100 bg-white p-6 shadow-sm text-sm text-gray-600">
              <h2 class="mb-3 text-xs font-semibold uppercase tracking-wide text-gray-500">Constitution légale</h2>
              <p v-if="site.constitution_statut_juridique">Statut : {{ site.constitution_statut_juridique }}</p>
              <p v-if="site.constitution_numero">N° : {{ site.constitution_numero }}</p>
              <a
                v-if="site.constitution_document_url"
                :href="resoudreUrlImage(site.constitution_document_url)"
                target="_blank"
                rel="noopener noreferrer"
                class="mt-2 inline-flex items-center gap-1.5 text-xs font-medium text-custom-chocolat hover:underline"
              >
                <font-awesome-icon :icon="['fas', 'file-lines']" class="w-3.5 h-3.5" />
                Voir le document
              </a>
            </section>

            <!-- Réactions & partage -->
            <section class="rounded-2xl border border-gray-100 bg-white p-6 shadow-sm">
              <h2 class="mb-4 text-xs font-semibold uppercase tracking-wide text-gray-500">Ce lieu vous plaît ?</h2>
              <OpportuniteAfriqueReactionsBar
                type-objet="site_touristique"
                :objet-id="site.id"
                :nombre-likes="site.nombre_likes"
                :nombre-dislikes="site.nombre_dislikes"
                :ma-reaction="site.ma_reaction"
                :est-authentifie="userStore.isAuthenticated"
                @require-login="redirigerVersConnexion()"
              />
              <button
                type="button"
                class="mt-3 flex w-full items-center justify-center gap-2 rounded-lg bg-custom-chocolat px-4 py-2.5 text-sm font-medium text-white transition hover:bg-custom-chocolat/90 cursor-pointer"
                @click="showPartage = true"
              >
                <font-awesome-icon :icon="['fas', 'share-nodes']" class="w-4 h-4" />
                Partager
              </button>
            </section>

            <!-- Actions -->
            <section class="rounded-2xl border border-gray-100 bg-white p-6 shadow-sm">
              <NuxtLink
                :to="`/opportunite-afrique/${ficheId}`"
                class="flex w-full items-center justify-center gap-2 rounded-lg border border-gray-300 px-4 py-2.5 text-sm font-medium text-gray-700 transition hover:bg-gray-50"
              >
                <font-awesome-icon :icon="['fas', 'arrow-left']" class="w-3.5 h-3.5" />
                Retour au territoire
              </NuxtLink>
              <div class="mt-3 flex justify-center">
                <OpportuniteAfriqueContributionSignalerBouton
                  type-objet="site_touristique"
                  :objet-id="site.id"
                  :libelle="site.nom"
                  :a-signale="site.a_signale"
                  :est-authentifie="userStore.isAuthenticated"
                  @require-login="redirigerVersConnexion()"
                  @suspendu="site.suspendu = true"
                />
              </div>
            </section>
          </aside>
        </div>
      </div>

      <!-- Modal partage (réseaux sociaux + mur communautaire) -->
      <OpportuniteAfriquePartagerElementModal
        :is-open="showPartage"
        :titre="site.nom"
        type-label="le site"
        type-objet="site_touristique"
        :objet-id="site.id"
        :est-connecte="userStore.isAuthenticated"
        @close="showPartage = false"
      />
    </template>
  </div>
</template>

<script setup lang="ts">
import { useOpportuniteAfrique, LIBELLES_SOUS_TYPE, type SiteTouristiqueAPI } from '~/composables/useOpportuniteAfrique'
import { useUserStore } from '~/stores/user'

const route = useRoute()
const userStore = useUserStore()
const { redirigerVersConnexion } = useAuth()
const { obtenirSiteTouristique, resoudreUrlImage } = useOpportuniteAfrique()

const showPartage = ref(false)

const ficheId = route.params.id as string
const siteId = route.params.siteId as string

const { data: siteCharge, pending: chargement } = await useAsyncData(
  `site-${ficheId}-${siteId}`,
  () => obtenirSiteTouristique(ficheId, siteId),
)
const site = ref<SiteTouristiqueAPI | null>(siteCharge.value)

const galerie = computed<string[]>(() => {
  if (!site.value) return []
  if (site.value.images && site.value.images.length) return site.value.images
  return site.value.image_url ? [site.value.image_url] : []
})
const indexCourant = ref(0)
const imageCourante = computed(() => galerie.value[indexCourant.value] ?? null)
const naviguer = (delta: number) => {
  const n = galerie.value.length
  if (n > 0) indexCourant.value = (indexCourant.value + delta + n) % n
}

const libelleSousType = computed(() =>
  site.value?.sous_type ? LIBELLES_SOUS_TYPE[site.value.sous_type] : null,
)
const aContact = computed(() =>
  !!(site.value?.contact_telephone || site.value?.contact_courriel || site.value?.contact_adresse),
)
const aConstitution = computed(() =>
  !!(site.value?.constitution_statut_juridique || site.value?.constitution_numero || site.value?.constitution_document_url),
)
const aGps = computed(() => site.value?.latitude !== null && site.value?.longitude !== null && site.value != null)
const localisation = computed(() =>
  site.value ? [site.value.ville, site.value.village].filter(Boolean).join(', ') || null : null,
)

const onCle = (e: KeyboardEvent) => {
  if (e.key === 'ArrowRight') naviguer(1)
  else if (e.key === 'ArrowLeft') naviguer(-1)
}
onMounted(() => window.addEventListener('keydown', onCle))
onBeforeUnmount(() => window.removeEventListener('keydown', onCle))

const breadcrumbs = computed(() => [
  { label: 'Opportunités en Afrique', to: '/opportunite-afrique' },
  { label: 'Territoire', to: `/opportunite-afrique/${ficheId}` },
  { label: site.value?.nom || 'Site', to: undefined },
])

// ── SEO / Open Graph (aperçu lors du partage sur les réseaux sociaux) ──
const requete = useRequestURL()
const urlCanonique = `${requete.protocol}//${requete.host}/opportunite-afrique/${ficheId}/sites/${siteId}`
const imageOg = computed(() => (galerie.value.length ? resoudreUrlImage(galerie.value[0]!) : ''))
const descriptionOg = computed(() =>
  (site.value?.info_pertinente || site.value?.description || `Site touristique ${site.value?.nom ?? ''}`).slice(0, 200),
)

useHead(() => {
  if (!site.value) return {}
  const titre = `${site.value.nom} — Site touristique — UAfricas`
  return {
    title: titre,
    meta: [
      { name: 'description', content: descriptionOg.value },
      { property: 'og:type', content: 'article' },
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
