<template>
  <div class="bg-white rounded-xl shadow-md hover:shadow-xl transition-all duration-300 overflow-hidden border border-gray-100 hover:border-gray-200">
    <!-- Bande colorée -->
    <div class="h-1.5" :class="meta.bande"></div>

    <div class="p-6">
      <!-- En-tête auteur -->
      <div class="flex items-center gap-3 mb-4">
        <div class="shrink-0">
          <img
            v-if="photoAuteur"
            :src="photoAuteur"
            :alt="nomAuteur"
            class="w-11 h-11 rounded-full object-cover ring-2"
            :class="meta.ring"
          >
          <div
            v-else
            class="w-11 h-11 rounded-full flex items-center justify-center text-white font-bold text-sm"
            :class="meta.avatar"
          >
            {{ initiales }}
          </div>
        </div>
        <div class="min-w-0">
          <p class="text-sm text-gray-900">
            <span class="font-bold">{{ nomAuteur }}</span>
            <span class="text-gray-500"> a partagé {{ meta.article }}</span>
          </p>
          <p class="flex items-center gap-1.5 text-xs text-gray-400 mt-0.5">
            <font-awesome-icon :icon="['fas', 'calendar-days']" />
            {{ dateFormatee }}
          </p>
        </div>
        <span
          class="ml-auto shrink-0 flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-xs font-bold uppercase tracking-wide"
          :class="meta.badge"
        >
          <font-awesome-icon :icon="meta.icone" class="text-[10px]" />
          {{ meta.label }}
        </span>
      </div>

      <!-- Légende -->
      <p
        v-if="partage.legende"
        class="text-gray-700 text-sm leading-relaxed mb-4 italic border-l-4 pl-3"
        :class="meta.legende"
      >
        « {{ partage.legende }} »
      </p>

      <!-- Aperçu cliquable de l'élément -->
      <NuxtLink
        :to="lien"
        class="group block rounded-xl overflow-hidden border border-gray-100 transition-all duration-300"
        :class="meta.hoverBorder"
      >
        <div class="relative h-40 overflow-hidden">
          <img
            v-if="image"
            :src="image"
            :alt="partage.element.titre"
            class="w-full h-full object-cover transition-transform duration-500 group-hover:scale-105"
          >
          <div v-else class="w-full h-full flex items-center justify-center" :class="meta.fond">
            <font-awesome-icon :icon="meta.icone" class="text-white/70 text-4xl" />
          </div>
          <div class="absolute inset-0 bg-linear-to-t from-black/60 to-transparent"></div>

          <!-- Badge territoire -->
          <span class="absolute top-3 right-3 flex items-center gap-1.5 px-2.5 py-0.5 bg-white/90 backdrop-blur-sm text-gray-800 rounded-full text-xs font-semibold">
            <font-awesome-icon :icon="['fas', 'earth-africa']" class="text-custom-green text-[10px]" />
            {{ partage.element.territoire_nom }}
          </span>

          <!-- Titre -->
          <div class="absolute bottom-0 left-0 right-0 p-4">
            <h3 class="text-white font-bold text-lg leading-tight group-hover:text-amber-200 transition-colors line-clamp-2">
              {{ partage.element.titre }}
            </h3>
          </div>
        </div>

        <div class="flex items-center justify-between gap-2 px-4 py-3 bg-gray-50">
          <span class="text-xs text-gray-500">{{ meta.legende_pied }}</span>
          <span class="flex items-center gap-1.5 text-xs font-semibold group-hover:gap-2.5 transition-all" :class="meta.lien">
            Découvrir
            <font-awesome-icon :icon="['fas', 'arrow-up-right-from-square']" class="text-[10px]" />
          </span>
        </div>
      </NuxtLink>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { PartageElementAPI, TypeObjetElement } from '~/composables/useOpportuniteAfrique'

const props = defineProps<{
  partage: PartageElementAPI
}>()

const { resoudreUrlImage } = useOpportuniteAfrique()

interface MetaType {
  label: string
  article: string
  segment: string
  icone: string[]
  bande: string
  badge: string
  ring: string
  avatar: string
  fond: string
  legende: string
  legende_pied: string
  hoverBorder: string
  lien: string
}

const METAS: Record<TypeObjetElement, MetaType> = {
  secteur_developpement: {
    label: 'Secteur', article: "un secteur d'opportunité", segment: 'secteurs',
    icone: ['fas', 'briefcase'], bande: 'bg-linear-to-r from-custom-green to-emerald-600',
    badge: 'bg-green-100 text-custom-green', ring: 'ring-custom-green/20',
    avatar: 'bg-linear-to-br from-custom-green to-emerald-600', fond: 'bg-linear-to-br from-custom-green to-emerald-700',
    legende: 'border-custom-green/40', legende_pied: "Secteur d'opportunité",
    hoverBorder: 'hover:border-custom-green/40', lien: 'text-custom-green',
  },
  recette_culinaire: {
    label: 'Recette', article: 'une recette', segment: 'recettes',
    icone: ['fas', 'utensils'], bande: 'bg-linear-to-r from-amber-500 to-orange-600',
    badge: 'bg-amber-100 text-amber-700', ring: 'ring-amber-500/20',
    avatar: 'bg-linear-to-br from-amber-500 to-orange-600', fond: 'bg-linear-to-br from-amber-500 to-orange-700',
    legende: 'border-amber-500/40', legende_pied: 'Recette culinaire',
    hoverBorder: 'hover:border-amber-500/40', lien: 'text-amber-700',
  },
  site_touristique: {
    label: 'Site', article: 'un site touristique', segment: 'sites',
    icone: ['fas', 'location-dot'], bande: 'bg-linear-to-r from-custom-chocolat to-amber-700',
    badge: 'bg-amber-100 text-custom-chocolat', ring: 'ring-custom-chocolat/20',
    avatar: 'bg-linear-to-br from-custom-chocolat to-amber-700', fond: 'bg-linear-to-br from-custom-chocolat to-amber-800',
    legende: 'border-custom-chocolat/40', legende_pied: 'Site touristique',
    hoverBorder: 'hover:border-custom-chocolat/40', lien: 'text-custom-chocolat',
  },
  personnalite_connue: {
    label: 'Personnalité', article: 'une personnalité', segment: 'personnalites',
    icone: ['fas', 'user'], bande: 'bg-linear-to-r from-purple-600 to-fuchsia-600',
    badge: 'bg-purple-100 text-purple-700', ring: 'ring-purple-500/20',
    avatar: 'bg-linear-to-br from-purple-600 to-fuchsia-600', fond: 'bg-linear-to-br from-purple-600 to-fuchsia-700',
    legende: 'border-purple-500/40', legende_pied: 'Personnalité connue',
    hoverBorder: 'hover:border-purple-500/40', lien: 'text-purple-700',
  },
}

const meta = computed<MetaType>(() => METAS[props.partage.element.type_objet])
const lien = computed(() =>
  `/opportunite-afrique/${props.partage.element.fiche_pays_id}/${meta.value.segment}/${props.partage.element.objet_id}`,
)
const image = computed(() => resoudreUrlImage(props.partage.element.image_url))
const photoAuteur = computed(() => resoudreUrlImage(props.partage.auteur.photo_url))

const nomAuteur = computed(() => {
  const { prenom, nom } = props.partage.auteur
  return `${prenom ?? ''} ${nom ?? ''}`.trim() || 'Anonyme'
})

const initiales = computed(() => {
  const { prenom, nom } = props.partage.auteur
  return ((prenom ?? '').charAt(0) + (nom ?? '').charAt(0)).toUpperCase() || '?'
})

const dateFormatee = computed(() =>
  new Intl.DateTimeFormat('fr-FR', { day: 'numeric', month: 'short', year: 'numeric' }).format(
    new Date(props.partage.created_at),
  ),
)
</script>
