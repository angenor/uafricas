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
  (e: 'suspendu', site: SiteTouristiqueAPI): void
  (e: 'require-login'): void
}>()

const { resoudreUrlImage } = useOpportuniteAfrique()

// Navigation vers la page de détail dédiée (galerie, contacts, lien web, GPS, constitution, avis).
const ouvrirDetail = () => {
  navigateTo(`/opportunite-afrique/${props.site.fiche_pays_id}/sites/${props.site.id}`)
}

// Galerie : `images` (≤5) avec repli sur `image_url` (sites legacy).
const galerie = computed<string[]>(() => {
  if (props.site.images && props.site.images.length) return props.site.images
  return props.site.image_url ? [props.site.image_url] : []
})
const couverture = computed(() => galerie.value[0] ?? null)

const libelleSousType = computed(() =>
  props.site.sous_type ? LIBELLES_SOUS_TYPE[props.site.sous_type] : null,
)
const localisation = computed(() =>
  [props.site.ville, props.site.village].filter(Boolean).join(', ') || null,
)
</script>

<template>
  <article class="flex flex-col overflow-hidden rounded-lg bg-white shadow-sm transition-shadow hover:shadow-md">
    <!-- Couverture (clic = ouvrir les détails) -->
    <button
      type="button"
      class="group relative block aspect-video w-full shrink-0 cursor-pointer overflow-hidden"
      :aria-label="`Voir les détails de ${site.nom}`"
      @click="ouvrirDetail"
    >
      <img
        v-if="couverture"
        :src="resoudreUrlImage(couverture)"
        :alt="site.nom"
        class="h-full w-full object-cover transition-transform group-hover:scale-[1.02]"
      />
      <span v-else class="grid h-full w-full place-items-center bg-af-fond">
        <font-awesome-icon icon="fa-solid fa-location-dot" class="text-3xl text-af-atone-2" />
      </span>
      <span
        v-if="galerie.length > 1"
        class="pointer-events-none absolute bottom-2 right-2 inline-flex items-center gap-1 rounded bg-black/55 px-2 py-1 text-xs font-medium text-white"
      >
        <font-awesome-icon :icon="['fas', 'images']" class="h-3.5 w-3.5" />
        {{ galerie.length }}
      </span>
      <span
        v-if="site.verifie"
        class="absolute right-2 top-2 inline-flex items-center gap-1 rounded-full bg-white/90 px-2 py-1 text-[12px]/[1.4] font-bold text-af-vert shadow-sm"
      >
        <font-awesome-icon :icon="['fas', 'circle-check']" class="h-3.5 w-3.5" />
        Vérifié
      </span>
      <span
        v-if="libelleSousType"
        class="absolute bottom-2 left-2 rounded bg-black/55 px-2 py-1 text-xs font-medium text-white"
      >
        {{ libelleSousType }}
      </span>
    </button>

    <div class="flex flex-1 flex-col p-4">
      <div class="mb-1 flex items-start justify-between gap-2">
        <h4 class="text-[17px]/[1.4] font-bold text-af-encre">{{ site.nom }}</h4>
        <span
          v-if="site.nombre_avis > 0 && site.note_moyenne !== null"
          class="inline-flex shrink-0 items-center gap-1 text-sm"
          :title="`${site.nombre_avis} avis`"
        >
          <font-awesome-icon :icon="['fas', 'star']" class="h-3.5 w-3.5 text-amber-400" />
          <span class="font-bold text-af-encre">{{ site.note_moyenne.toFixed(1) }}</span>
          <span class="text-xs text-gray-400">({{ site.nombre_avis }})</span>
        </span>
      </div>

      <div class="mb-2 flex flex-wrap items-center gap-x-3 gap-y-1 text-xs text-gray-500">
        <span v-if="localisation" class="inline-flex items-center gap-1">
          <font-awesome-icon :icon="['fas', 'location-dot']" class="h-3 w-3 text-af-atone" />
          {{ localisation }}
        </span>
        <span v-if="site.gestionnaire" class="inline-flex items-center gap-1">
          <font-awesome-icon :icon="['fas', 'user']" class="h-3 w-3 text-gray-400" />
          {{ site.gestionnaire }}
        </span>
      </div>

      <p v-if="site.info_pertinente" class="mb-3 line-clamp-2 text-[14px]/[1.4] text-af-corps">{{ site.info_pertinente }}</p>
      <p v-else-if="site.description" class="mb-3 line-clamp-2 text-sm text-gray-600">{{ site.description }}</p>

      <!-- Bandeau de suspension (>10 signalements) -->
      <div
        v-if="site.suspendu"
        class="mb-3 flex items-start gap-2 rounded-[10px] border border-af-live/30 bg-af-live/5 px-3 py-2 text-[12px]/[1.4] text-af-live"
      >
        <font-awesome-icon :icon="['fas', 'triangle-exclamation']" class="h-3.5 w-3.5 mt-0.5 shrink-0" />
        <span>Contribution suspendue : en cours de vérification.</span>
      </div>

      <!-- Barre d'actions compacte -->
      <div class="mt-auto flex flex-wrap items-center gap-x-3 gap-y-2 border-t border-af-bordure pt-3 text-xs font-medium">
        <button
          type="button"
          class="inline-flex items-center gap-1 text-af-corps transition hover:text-af-chocolat"
          @click="ouvrirDetail"
        >
          <font-awesome-icon :icon="['fas', 'circle-info']" class="h-3.5 w-3.5" />
          Détails<span v-if="site.nombre_avis > 0"> &amp; avis ({{ site.nombre_avis }})</span>
        </button>
        <span class="ml-auto flex items-center gap-3">
          <template v-if="!site.suspendu">
            <button type="button" class="inline-flex items-center gap-1 text-af-corps transition hover:text-af-chocolat" @click="emit('edit', site)">
              <font-awesome-icon :icon="['fas', 'pen-to-square']" class="h-3.5 w-3.5" />
              Modifier
            </button>
            <button type="button" class="inline-flex items-center gap-1 text-af-corps transition hover:text-af-live" @click="emit('delete', site)">
              <font-awesome-icon :icon="['fas', 'trash']" class="h-3.5 w-3.5" />
              Supprimer
            </button>
          </template>
          <OpportuniteAfriqueContributionSignalerBouton
            type-objet="site_touristique"
            :objet-id="site.id"
            :libelle="site.nom"
            :a-signale="site.a_signale"
            :est-authentifie="estAuthentifie"
            @require-login="emit('require-login')"
            @suspendu="emit('suspendu', site)"
          />
        </span>
      </div>
    </div>
  </article>
</template>
