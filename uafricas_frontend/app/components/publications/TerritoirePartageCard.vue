<template>
  <div class="bg-white rounded-xl shadow-md hover:shadow-xl transition-all duration-300 overflow-hidden border border-gray-100 hover:border-gray-200">
    <!-- Bande colorée -->
    <div class="h-1.5 bg-linear-to-r from-custom-green to-emerald-600"></div>

    <div class="p-6">
      <!-- En-tête auteur -->
      <div class="flex items-center gap-3 mb-4">
        <div class="shrink-0">
          <img
            v-if="photoAuteur"
            :src="photoAuteur"
            :alt="nomAuteur"
            class="w-11 h-11 rounded-full object-cover ring-2 ring-custom-green/20"
          >
          <div
            v-else
            class="w-11 h-11 rounded-full bg-linear-to-br from-custom-green to-emerald-600 flex items-center justify-center text-white font-bold text-sm"
          >
            {{ initiales }}
          </div>
        </div>
        <div class="min-w-0">
          <p class="text-sm text-gray-900">
            <span class="font-bold">{{ nomAuteur }}</span>
            <span class="text-gray-500"> a partagé un territoire</span>
          </p>
          <p class="flex items-center gap-1.5 text-xs text-gray-400 mt-0.5">
            <font-awesome-icon :icon="['fas', 'calendar-days']" />
            {{ dateFormatee }}
          </p>
        </div>
        <span class="ml-auto shrink-0 flex items-center gap-1.5 px-2.5 py-0.5 bg-custom-green/10 text-custom-green rounded-full text-xs font-bold uppercase tracking-wide">
          <font-awesome-icon :icon="['fas', 'share-nodes']" class="text-[10px]" />
          Territoire
        </span>
      </div>

      <!-- Légende -->
      <p v-if="partage.legende" class="text-gray-700 text-sm leading-relaxed mb-4 italic border-l-4 border-custom-green/40 pl-3">
        « {{ partage.legende }} »
      </p>

      <!-- Aperçu cliquable de la fiche -->
      <NuxtLink
        :to="`/opportunite-afrique/${partage.fiche.id}`"
        class="group block rounded-xl overflow-hidden border border-gray-100 hover:border-custom-green/40 transition-all duration-300"
      >
        <!-- Couverture -->
        <div class="relative h-40 overflow-hidden">
          <img
            v-if="couverture"
            :src="couverture"
            :alt="partage.fiche.nom"
            class="w-full h-full object-cover transition-transform duration-500 group-hover:scale-105"
          >
          <div
            v-else
            class="w-full h-full bg-linear-to-br from-custom-chocolat via-amber-700 to-custom-green"
          ></div>
          <div class="absolute inset-0 bg-linear-to-t from-black/60 to-transparent"></div>

          <!-- Drapeau -->
          <img
            v-if="drapeau"
            :src="drapeau"
            :alt="`Drapeau ${partage.fiche.nom}`"
            class="absolute top-3 left-3 w-9 h-6 object-cover rounded shadow ring-1 ring-white/40"
          >

          <!-- Badge région -->
          <span class="absolute top-3 right-3 flex items-center gap-1.5 px-2.5 py-0.5 bg-white/90 backdrop-blur-sm text-gray-800 rounded-full text-xs font-semibold">
            <font-awesome-icon :icon="['fas', 'earth-africa']" class="text-custom-green text-[10px]" />
            {{ partage.fiche.region }}
          </span>

          <!-- Nom + slogan -->
          <div class="absolute bottom-0 left-0 right-0 p-4">
            <h3 class="text-white font-bold text-lg leading-tight group-hover:text-amber-200 transition-colors">
              {{ partage.fiche.nom }}
            </h3>
            <p v-if="partage.fiche.slogan" class="text-white/90 text-xs italic mt-0.5 line-clamp-1">
              {{ partage.fiche.slogan }}
            </p>
          </div>
        </div>

        <!-- Pied : capitale -->
        <div class="flex items-center justify-between gap-2 px-4 py-3 bg-gray-50">
          <span v-if="partage.fiche.capitale" class="flex items-center gap-1.5 text-xs text-gray-500">
            <font-awesome-icon :icon="['fas', 'location-dot']" class="text-custom-chocolat" />
            Capitale : <span class="font-medium text-gray-700">{{ partage.fiche.capitale }}</span>
          </span>
          <span v-else class="text-xs text-gray-400">Découvrir le territoire</span>
          <span class="flex items-center gap-1.5 text-xs font-semibold text-custom-green group-hover:gap-2.5 transition-all">
            Voir la fiche
            <font-awesome-icon :icon="['fas', 'arrow-up-right-from-square']" class="text-[10px]" />
          </span>
        </div>
      </NuxtLink>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { PartageFicheAPI } from '~/composables/useOpportuniteAfrique'

const props = defineProps<{
  partage: PartageFicheAPI
}>()

const { resoudreUrlImage } = useOpportuniteAfrique()

const photoAuteur = computed(() => resoudreUrlImage(props.partage.auteur.photo_url))
const couverture = computed(() => resoudreUrlImage(props.partage.fiche.image_couverture))
const drapeau = computed(() => resoudreUrlImage(props.partage.fiche.drapeau_url))

const nomAuteur = computed(() => {
  const { prenom, nom } = props.partage.auteur
  return `${prenom ?? ''} ${nom ?? ''}`.trim() || 'Anonyme'
})

const initiales = computed(() => {
  const { prenom, nom } = props.partage.auteur
  const p = (prenom ?? '').charAt(0)
  const n = (nom ?? '').charAt(0)
  return (p + n).toUpperCase() || '?'
})

const dateFormatee = computed(() => {
  const d = new Date(props.partage.created_at)
  return new Intl.DateTimeFormat('fr-FR', {
    day: 'numeric',
    month: 'short',
    year: 'numeric',
  }).format(d)
})
</script>
