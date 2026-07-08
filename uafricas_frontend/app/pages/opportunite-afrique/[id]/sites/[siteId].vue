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
      <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <CommonBreadcrumbNav class="mb-6" :custom-breadcrumbs="breadcrumbs" />

        <div class="bg-white rounded-2xl shadow-sm overflow-hidden">
          <!-- Galerie -->
          <div v-if="imageCourante" class="relative aspect-video bg-gray-100">
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

          <div class="p-6 sm:p-8 space-y-6">
            <!-- En-tête -->
            <div>
              <h1 class="font-oswald text-3xl sm:text-4xl font-bold text-gray-900 leading-tight">{{ site.nom }}</h1>
              <div class="mt-2 flex flex-wrap items-center gap-2 text-sm">
                <span v-if="libelleSousType" class="rounded bg-custom-chocolat/10 px-2 py-0.5 font-medium text-custom-chocolat">
                  {{ libelleSousType }}
                </span>
                <span
                  v-if="site.verifie"
                  class="inline-flex items-center gap-1 rounded-full bg-custom-green/10 px-2 py-0.5 font-medium text-custom-green"
                >
                  <font-awesome-icon :icon="['fas', 'circle-check']" class="h-3 w-3" />
                  Vérifié
                </span>
                <span
                  v-if="site.nombre_avis > 0 && site.note_moyenne !== null"
                  class="inline-flex items-center gap-1 text-gray-600"
                >
                  <font-awesome-icon :icon="['fas', 'star']" class="h-3.5 w-3.5 text-amber-400" />
                  <span class="font-semibold text-gray-900">{{ site.note_moyenne.toFixed(1) }}</span>
                  ({{ site.nombre_avis }})
                </span>
              </div>
            </div>

            <!-- Bandeau de suspension -->
            <div
              v-if="site.suspendu"
              class="flex items-start gap-2 rounded-md bg-orange-50 border border-orange-200 px-3 py-2 text-sm text-orange-800"
            >
              <font-awesome-icon :icon="['fas', 'triangle-exclamation']" class="w-4 h-4 mt-0.5 shrink-0" />
              <span>Contribution suspendue — en cours de vérification par la modération.</span>
            </div>

            <!-- Miniatures -->
            <div v-if="galerie.length > 1" class="flex flex-wrap gap-1.5">
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

            <!-- Localisation + gestionnaire -->
            <div class="flex flex-wrap items-center gap-x-4 gap-y-1 text-sm text-gray-600">
              <span v-if="localisation" class="inline-flex items-center gap-1.5">
                <font-awesome-icon :icon="['fas', 'location-dot']" class="h-3.5 w-3.5 text-custom-chocolat" />
                {{ localisation }}
              </span>
              <span v-if="site.gestionnaire" class="inline-flex items-center gap-1.5">
                <font-awesome-icon :icon="['fas', 'user']" class="h-3.5 w-3.5 text-gray-400" />
                {{ site.gestionnaire }}
              </span>
            </div>

            <!-- Description -->
            <div v-if="site.info_pertinente || site.description" class="space-y-2 text-gray-700 leading-relaxed">
              <p v-if="site.info_pertinente">{{ site.info_pertinente }}</p>
              <p v-if="site.description" class="text-gray-600">{{ site.description }}</p>
            </div>

            <!-- Lien officiel -->
            <a
              v-if="site.site_web_url"
              :href="site.site_web_url"
              target="_blank"
              rel="noopener noreferrer"
              class="inline-flex items-center gap-2 rounded-md bg-custom-chocolat px-3 py-2 text-sm font-medium text-white hover:bg-custom-chocolat/90"
            >
              <font-awesome-icon :icon="['fas', 'arrow-up-right-from-square']" class="h-3.5 w-3.5" />
              Visiter le site web
            </a>

            <!-- Contacts -->
            <div v-if="aContact" class="space-y-1.5 rounded-md bg-gray-50 px-4 py-3 text-sm">
              <p class="mb-1 text-xs font-medium uppercase tracking-wide text-gray-500">Contacts</p>
              <a v-if="site.contact_telephone" :href="`tel:${site.contact_telephone}`" class="inline-flex items-center gap-2 text-gray-700 hover:text-custom-green">
                <font-awesome-icon :icon="['fas', 'phone']" class="h-3.5 w-3.5 text-custom-green" />
                {{ site.contact_telephone }}
              </a>
              <a v-if="site.contact_courriel" :href="`mailto:${site.contact_courriel}`" class="flex items-center gap-2 break-all text-gray-700 hover:text-custom-green">
                <font-awesome-icon :icon="['fas', 'envelope']" class="h-3.5 w-3.5 text-custom-green" />
                {{ site.contact_courriel }}
              </a>
              <span v-if="site.contact_adresse" class="flex items-center gap-2 text-gray-700">
                <font-awesome-icon :icon="['fas', 'location-dot']" class="h-3.5 w-3.5 text-custom-green" />
                {{ site.contact_adresse }}
              </span>
            </div>

            <!-- GPS -->
            <p v-if="aGps" class="flex items-center gap-1.5 text-xs text-gray-500">
              <font-awesome-icon :icon="['fas', 'map-pin']" class="h-3.5 w-3.5 text-gray-400" />
              {{ site.latitude!.toFixed(4) }}, {{ site.longitude!.toFixed(4) }}
            </p>

            <!-- Constitution légale -->
            <div v-if="aConstitution" class="text-sm text-gray-600">
              <p class="mb-1 text-xs font-medium uppercase tracking-wide text-gray-500">Constitution légale</p>
              <p v-if="site.constitution_statut_juridique">Statut : {{ site.constitution_statut_juridique }}</p>
              <p v-if="site.constitution_numero">N° : {{ site.constitution_numero }}</p>
              <a
                v-if="site.constitution_document_url"
                :href="resoudreUrlImage(site.constitution_document_url)"
                target="_blank"
                rel="noopener noreferrer"
                class="text-xs text-custom-chocolat hover:underline"
              >
                Voir le document
              </a>
            </div>

            <!-- Avis -->
            <div class="border-t border-gray-100 pt-6">
              <OpportuniteAfriqueSiteAvisListe :site-id="site.id" :est-authentifie="userStore.isAuthenticated" />
            </div>

            <!-- Actions -->
            <div class="flex flex-wrap items-center gap-4 pt-2 border-t border-gray-100">
              <NuxtLink
                :to="`/opportunite-afrique/${ficheId}`"
                class="inline-flex items-center gap-1.5 text-sm font-medium text-gray-600 hover:text-custom-chocolat"
              >
                <font-awesome-icon :icon="['fas', 'arrow-left']" class="w-3.5 h-3.5" />
                Retour au territoire
              </NuxtLink>
              <OpportuniteAfriqueContributionSignalerBouton
                type-objet="site_touristique"
                :objet-id="site.id"
                :libelle="site.nom"
                :a-signale="site.a_signale"
                :est-authentifie="userStore.isAuthenticated"
                @require-login="navigateTo('/login')"
                @suspendu="site.suspendu = true"
              />
            </div>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { useOpportuniteAfrique, LIBELLES_SOUS_TYPE, type SiteTouristiqueAPI } from '~/composables/useOpportuniteAfrique'
import { useUserStore } from '~/stores/user'

const route = useRoute()
const userStore = useUserStore()
const { obtenirSiteTouristique, resoudreUrlImage } = useOpportuniteAfrique()

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

useHead(() => ({
  title: site.value ? `${site.value.nom} — Site touristique — UAfricas` : 'Site touristique — UAfricas',
  meta: site.value
    ? [{ name: 'description', content: (site.value.info_pertinente || site.value.description || `Site ${site.value.nom}`).slice(0, 160) }]
    : [],
}))
</script>
