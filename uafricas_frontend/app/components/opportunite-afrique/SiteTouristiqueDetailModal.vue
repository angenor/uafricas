<script setup lang="ts">
import { useOpportuniteAfrique } from '~/composables/useOpportuniteAfrique'
import { LIBELLES_SOUS_TYPE, type SiteTouristiqueAPI } from '~/composables/useOpportuniteAfrique'

interface Props {
  site: SiteTouristiqueAPI
  estAuthentifie: boolean
  ouvert: boolean
}

const props = defineProps<Props>()

const emit = defineEmits<{
  (e: 'fermer'): void
}>()

const { resoudreUrlImage } = useOpportuniteAfrique()

// Galerie : `images` (≤5) avec repli sur `image_url` (sites legacy).
const galerie = computed<string[]>(() => {
  if (props.site.images && props.site.images.length) return props.site.images
  return props.site.image_url ? [props.site.image_url] : []
})

const indexCourant = ref(0)
const imageCourante = computed(() => galerie.value[indexCourant.value] ?? null)
const naviguer = (delta: number) => {
  const n = galerie.value.length
  if (n > 0) indexCourant.value = (indexCourant.value + delta + n) % n
}

const libelleSousType = computed(() =>
  props.site.sous_type ? LIBELLES_SOUS_TYPE[props.site.sous_type] : null,
)
const aContact = computed(() =>
  !!(props.site.contact_telephone || props.site.contact_courriel || props.site.contact_adresse),
)
const aConstitution = computed(() =>
  !!(props.site.constitution_statut_juridique || props.site.constitution_numero || props.site.constitution_document_url),
)
const aGps = computed(() => props.site.latitude !== null && props.site.longitude !== null)
const localisation = computed(() =>
  [props.site.ville, props.site.village].filter(Boolean).join(', ') || null,
)

// Réinitialise la 1re image à chaque ouverture.
watch(() => props.ouvert, (v) => {
  if (v) indexCourant.value = 0
})

const onCle = (e: KeyboardEvent) => {
  if (!props.ouvert) return
  if (e.key === 'Escape') emit('fermer')
  else if (e.key === 'ArrowRight') naviguer(1)
  else if (e.key === 'ArrowLeft') naviguer(-1)
}
onMounted(() => window.addEventListener('keydown', onCle))
onBeforeUnmount(() => window.removeEventListener('keydown', onCle))
</script>

<template>
  <Teleport to="body">
    <Transition name="modal-fade">
      <div
        v-if="ouvert"
        class="fixed inset-0 z-[70] flex items-start justify-center overflow-y-auto bg-black/60 p-4 sm:p-6"
        @click.self="emit('fermer')"
      >
        <div class="relative my-4 w-full max-w-2xl rounded-xl bg-white shadow-2xl">
          <!-- En-tête -->
          <div class="flex items-start justify-between gap-3 border-b border-gray-100 px-5 py-4">
            <div class="min-w-0">
              <h3 class="font-oswald text-xl font-semibold text-gray-900 leading-tight">{{ site.nom }}</h3>
              <div class="mt-1 flex flex-wrap items-center gap-2 text-xs">
                <span
                  v-if="libelleSousType"
                  class="rounded bg-custom-chocolat/10 px-2 py-0.5 font-medium text-custom-chocolat"
                >
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
                  <font-awesome-icon :icon="['fas', 'star']" class="h-3 w-3 text-amber-400" />
                  <span class="font-semibold text-gray-900">{{ site.note_moyenne.toFixed(1) }}</span>
                  ({{ site.nombre_avis }})
                </span>
              </div>
            </div>
            <button
              type="button"
              class="shrink-0 rounded-full p-1.5 text-gray-400 hover:bg-gray-100 hover:text-gray-700"
              aria-label="Fermer"
              @click="emit('fermer')"
            >
              <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>

          <!-- Corps défilable -->
          <div class="max-h-[75vh] space-y-5 overflow-y-auto px-5 py-4">
            <!-- Galerie -->
            <div v-if="imageCourante">
              <div class="relative aspect-video overflow-hidden rounded-lg bg-gray-100">
                <img
                  :src="resoudreUrlImage(imageCourante)"
                  :alt="site.nom"
                  class="h-full w-full object-contain"
                />
                <button
                  v-if="galerie.length > 1"
                  type="button"
                  class="absolute left-2 top-1/2 -translate-y-1/2 rounded-full bg-black/45 p-1.5 text-white hover:bg-black/65"
                  aria-label="Image précédente"
                  @click="naviguer(-1)"
                >
                  <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                  </svg>
                </button>
                <button
                  v-if="galerie.length > 1"
                  type="button"
                  class="absolute right-2 top-1/2 -translate-y-1/2 rounded-full bg-black/45 p-1.5 text-white hover:bg-black/65"
                  aria-label="Image suivante"
                  @click="naviguer(1)"
                >
                  <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                  </svg>
                </button>
                <span
                  v-if="galerie.length > 1"
                  class="absolute bottom-2 right-2 rounded bg-black/55 px-2 py-0.5 text-xs font-medium text-white"
                >
                  {{ indexCourant + 1 }} / {{ galerie.length }}
                </span>
              </div>
              <div v-if="galerie.length > 1" class="mt-2 flex flex-wrap gap-1.5">
                <button
                  v-for="(url, i) in galerie"
                  :key="url"
                  type="button"
                  class="h-12 w-16 overflow-hidden rounded border transition-opacity"
                  :class="i === indexCourant ? 'border-custom-chocolat opacity-100' : 'border-gray-200 opacity-70 hover:opacity-100'"
                  :aria-label="`Image ${i + 1}`"
                  @click="indexCourant = i"
                >
                  <img :src="resoudreUrlImage(url)" :alt="`${site.nom} ${i + 1}`" class="h-full w-full object-cover" />
                </button>
              </div>
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

            <!-- Description / info pertinente -->
            <div v-if="site.info_pertinente || site.description" class="space-y-2 text-sm text-gray-700">
              <p v-if="site.info_pertinente">{{ site.info_pertinente }}</p>
              <p v-if="site.description" class="text-gray-600">{{ site.description }}</p>
            </div>

            <!-- Lien officiel du site web -->
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
              <a
                v-if="site.contact_telephone"
                :href="`tel:${site.contact_telephone}`"
                class="inline-flex items-center gap-2 text-gray-700 hover:text-custom-green"
              >
                <font-awesome-icon :icon="['fas', 'phone']" class="h-3.5 w-3.5 text-custom-green" />
                {{ site.contact_telephone }}
              </a>
              <a
                v-if="site.contact_courriel"
                :href="`mailto:${site.contact_courriel}`"
                class="flex items-center gap-2 break-all text-gray-700 hover:text-custom-green"
              >
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
            <div class="border-t border-gray-100 pt-4">
              <OpportuniteAfriqueSiteAvisListe :site-id="site.id" :est-authentifie="estAuthentifie" />
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: opacity 0.2s ease;
}
.modal-fade-enter-from,
.modal-fade-leave-to {
  opacity: 0;
}
</style>
