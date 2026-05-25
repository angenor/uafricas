<script setup lang="ts">
import { useOpportuniteAfrique } from '~/composables/useOpportuniteAfrique'
import { LIBELLES_SOUS_TYPE, type SiteTouristiqueAPI } from '~/composables/useOpportuniteAfrique'

interface Props {
  site: SiteTouristiqueAPI
  estAuthentifie: boolean
}

const props = defineProps<Props>()

const emit = defineEmits<{
  (e: 'edit', site: SiteTouristiqueAPI): void
  (e: 'delete', site: SiteTouristiqueAPI): void
}>()

const { resoudreUrlImage } = useOpportuniteAfrique()

// Panneau détails (GPS, constitution, avis) replié par défaut.
const ouvert = ref(false)

// Galerie : `images` (≤3) avec repli sur `image_url` (sites legacy).
const galerie = computed<string[]>(() => {
  if (props.site.images && props.site.images.length) return props.site.images
  return props.site.image_url ? [props.site.image_url] : []
})
const couverture = computed(() => galerie.value[0] ?? null)

// Visionneuse plein écran (lightbox)
const lightboxOuvert = ref(false)
const lightboxIndex = ref(0)
const imageLightbox = computed(() => galerie.value[lightboxIndex.value] ?? null)

const ouvrirLightbox = (i: number) => {
  lightboxIndex.value = i
  lightboxOuvert.value = true
}
const fermerLightbox = () => { lightboxOuvert.value = false }
const naviguer = (delta: number) => {
  const n = galerie.value.length
  if (n > 0) lightboxIndex.value = (lightboxIndex.value + delta + n) % n
}

const onCleLightbox = (e: KeyboardEvent) => {
  if (!lightboxOuvert.value) return
  if (e.key === 'Escape') fermerLightbox()
  else if (e.key === 'ArrowRight') naviguer(1)
  else if (e.key === 'ArrowLeft') naviguer(-1)
}
onMounted(() => window.addEventListener('keydown', onCleLightbox))
onBeforeUnmount(() => window.removeEventListener('keydown', onCleLightbox))

const libelleSousType = computed(() =>
  props.site.sous_type ? LIBELLES_SOUS_TYPE[props.site.sous_type] : null,
)
const estPrive = computed(() => props.site.categorie === 'prive')
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
</script>

<template>
  <article class="bg-white rounded-lg overflow-hidden shadow-sm hover:shadow-md transition-shadow flex flex-col">
    <!-- Image de couverture (clic = agrandir) + miniatures -->
    <div class="aspect-video relative overflow-hidden shrink-0">
      <button
        v-if="couverture"
        type="button"
        class="block w-full h-full cursor-zoom-in group"
        aria-label="Agrandir l'image"
        @click="ouvrirLightbox(0)"
      >
        <img
          :src="resoudreUrlImage(couverture)"
          :alt="site.nom"
          class="w-full h-full object-cover transition-transform group-hover:scale-[1.02]"
        />
      </button>
      <div v-else class="w-full h-full bg-gradient-to-br from-custom-chocolat to-custom-green" />
      <span
        v-if="galerie.length > 1"
        class="absolute bottom-2 right-2 inline-flex items-center gap-1 px-2 py-1 rounded bg-black/55 text-white text-xs font-medium pointer-events-none"
      >
        <font-awesome-icon :icon="['fas', 'images']" class="w-3.5 h-3.5" />
        {{ galerie.length }}
      </span>
      <span
        v-if="site.verifie"
        class="absolute top-2 right-2 inline-flex items-center gap-1 px-2 py-1 rounded-full bg-white/90 text-custom-green text-xs font-medium shadow-sm"
      >
        <font-awesome-icon :icon="['fas', 'circle-check']" class="w-3.5 h-3.5" />
        Vérifié
      </span>
      <span
        v-if="libelleSousType"
        class="absolute bottom-2 left-2 px-2 py-1 rounded bg-black/55 text-white text-xs font-medium"
      >
        {{ libelleSousType }}
      </span>
    </div>
    <div v-if="galerie.length > 1" class="flex gap-1.5 px-3 pt-3">
      <button
        v-for="(url, i) in galerie"
        :key="url"
        type="button"
        class="h-12 w-16 rounded overflow-hidden border border-gray-200 opacity-80 hover:opacity-100 transition-opacity cursor-zoom-in"
        :aria-label="`Agrandir l'image ${i + 1}`"
        @click="ouvrirLightbox(i)"
      >
        <img :src="resoudreUrlImage(url)" :alt="`${site.nom} ${i + 1}`" class="h-full w-full object-cover" />
      </button>
    </div>

    <div class="p-4 flex flex-col flex-1">
      <div class="flex items-start justify-between gap-2 mb-1">
        <h4 class="font-oswald text-lg font-semibold text-gray-900 leading-tight">{{ site.nom }}</h4>
        <span
          v-if="site.nombre_avis > 0 && site.note_moyenne !== null"
          class="shrink-0 inline-flex items-center gap-1 text-sm"
          :title="`${site.nombre_avis} avis`"
        >
          <font-awesome-icon :icon="['fas', 'star']" class="w-3.5 h-3.5 text-amber-400" />
          <span class="font-semibold text-gray-900">{{ site.note_moyenne.toFixed(1) }}</span>
          <span class="text-gray-400 text-xs">({{ site.nombre_avis }})</span>
        </span>
      </div>

      <div class="flex flex-wrap items-center gap-x-3 gap-y-1 text-xs text-gray-500 mb-2">
        <span v-if="localisation" class="inline-flex items-center gap-1">
          <font-awesome-icon :icon="['fas', 'location-dot']" class="w-3 h-3 text-custom-chocolat" />
          {{ localisation }}
        </span>
        <span v-if="site.gestionnaire" class="inline-flex items-center gap-1">
          <font-awesome-icon :icon="['fas', 'user']" class="w-3 h-3 text-gray-400" />
          {{ site.gestionnaire }}
        </span>
      </div>

      <p v-if="site.info_pertinente" class="text-sm text-gray-700 line-clamp-2 mb-2">{{ site.info_pertinente }}</p>
      <p v-else-if="site.description" class="text-sm text-gray-600 line-clamp-2 mb-2">{{ site.description }}</p>

      <!-- Contacts — toujours visibles (publics, l'intérêt principal) -->
      <div v-if="aContact" class="flex flex-col gap-1 text-sm mb-2 rounded-md bg-gray-50 px-3 py-2">
        <a
          v-if="site.contact_telephone"
          :href="`tel:${site.contact_telephone}`"
          class="inline-flex items-center gap-2 text-gray-700 hover:text-custom-green"
        >
          <font-awesome-icon :icon="['fas', 'phone']" class="w-3.5 h-3.5 text-custom-green" />
          {{ site.contact_telephone }}
        </a>
        <a
          v-if="site.contact_courriel"
          :href="`mailto:${site.contact_courriel}`"
          class="inline-flex items-center gap-2 text-gray-700 hover:text-custom-green break-all"
        >
          <font-awesome-icon :icon="['fas', 'envelope']" class="w-3.5 h-3.5 text-custom-green" />
          {{ site.contact_courriel }}
        </a>
        <span v-if="site.contact_adresse" class="inline-flex items-center gap-2 text-gray-700">
          <font-awesome-icon :icon="['fas', 'location-dot']" class="w-3.5 h-3.5 text-custom-green" />
          {{ site.contact_adresse }}
        </span>
      </div>

      <!-- Barre d'actions compacte -->
      <div class="flex flex-wrap items-center gap-x-3 gap-y-2 mt-auto pt-3 border-t border-gray-100 text-xs font-medium">
        <button
          type="button"
          class="inline-flex items-center gap-1 text-gray-700 hover:text-custom-chocolat"
          :aria-expanded="ouvert"
          @click="ouvert = !ouvert"
        >
          <font-awesome-icon :icon="['fas', 'star']" class="w-3.5 h-3.5 text-amber-400" />
          {{ site.nombre_avis > 0 ? `Avis (${site.nombre_avis})` : 'Donner un avis' }}
          <font-awesome-icon :icon="['fas', ouvert ? 'chevron-up' : 'chevron-down']" class="w-3 h-3" />
        </button>
        <span class="ml-auto flex items-center gap-3">
          <button type="button" class="inline-flex items-center gap-1 text-custom-chocolat hover:underline" @click="emit('edit', site)">
            <font-awesome-icon :icon="['fas', 'pen-to-square']" class="w-3.5 h-3.5" />
            Modifier
          </button>
          <button type="button" class="inline-flex items-center gap-1 text-red-600 hover:underline" @click="emit('delete', site)">
            <font-awesome-icon :icon="['fas', 'trash']" class="w-3.5 h-3.5" />
            Supprimer
          </button>
        </span>
      </div>

      <!-- Panneau détails & avis (replié par défaut) -->
      <div v-if="ouvert" class="mt-3 pt-3 border-t border-gray-100 space-y-3 text-sm">
        <p v-if="aGps" class="flex items-center gap-1.5 text-xs text-gray-500">
          <font-awesome-icon :icon="['fas', 'map-pin']" class="w-3.5 h-3.5 text-gray-400" />
          {{ site.latitude!.toFixed(4) }}, {{ site.longitude!.toFixed(4) }}
        </p>

        <div v-if="aConstitution" class="text-gray-600">
          <p class="font-medium text-gray-700 text-xs uppercase tracking-wide mb-1">Constitution légale</p>
          <p v-if="site.constitution_statut_juridique">Statut : {{ site.constitution_statut_juridique }}</p>
          <p v-if="site.constitution_numero">N° : {{ site.constitution_numero }}</p>
          <a
            v-if="site.constitution_document_url"
            :href="resoudreUrlImage(site.constitution_document_url)"
            target="_blank"
            rel="noopener"
            class="text-custom-chocolat hover:underline text-xs"
          >
            Voir le document
          </a>
        </div>

        <OpportuniteAfriqueSiteAvisListe :site-id="site.id" :est-authentifie="estAuthentifie" />
      </div>
    </div>

    <!-- Lightbox plein écran -->
    <Teleport to="body">
      <Transition name="lightbox-fade">
        <div
          v-if="lightboxOuvert && imageLightbox"
          class="fixed inset-0 z-[60] flex items-center justify-center bg-black/85 p-4"
          @click.self="fermerLightbox"
        >
          <button
            type="button"
            class="absolute top-4 right-4 text-white/80 hover:text-white"
            aria-label="Fermer"
            @click="fermerLightbox"
          >
            <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>

          <button
            v-if="galerie.length > 1"
            type="button"
            class="absolute left-4 text-white/80 hover:text-white p-2"
            aria-label="Image précédente"
            @click.stop="naviguer(-1)"
          >
            <svg class="w-9 h-9" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
            </svg>
          </button>

          <figure class="max-w-5xl max-h-[88vh] flex flex-col items-center">
            <img
              :src="resoudreUrlImage(imageLightbox)"
              :alt="site.nom"
              class="max-w-full max-h-[80vh] object-contain rounded-md shadow-2xl"
            />
            <figcaption class="mt-3 text-white/80 text-sm">
              {{ site.nom }}<span v-if="galerie.length > 1"> — {{ lightboxIndex + 1 }} / {{ galerie.length }}</span>
            </figcaption>
          </figure>

          <button
            v-if="galerie.length > 1"
            type="button"
            class="absolute right-4 text-white/80 hover:text-white p-2"
            aria-label="Image suivante"
            @click.stop="naviguer(1)"
          >
            <svg class="w-9 h-9" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
            </svg>
          </button>
        </div>
      </Transition>
    </Teleport>
  </article>
</template>

<style scoped>
.lightbox-fade-enter-active,
.lightbox-fade-leave-active {
  transition: opacity 0.2s ease;
}
.lightbox-fade-enter-from,
.lightbox-fade-leave-to {
  opacity: 0;
}
</style>
