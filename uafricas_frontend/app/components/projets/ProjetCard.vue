<template>
  <div
    class="relative bg-white rounded-2xl overflow-hidden shadow-lg hover:shadow-2xl transition-all duration-300 group cursor-pointer"
    @click="$emit('click', projet)"
  >
    <!-- Image avec overlay -->
    <div class="relative h-48 overflow-hidden">
      <img
        class="w-full h-full object-cover transition-transform duration-500 group-hover:scale-110"
        :src="projet.imageCouverture || '/images/investissement-afrique.jpg'"
        :alt="projet.titre"
      />

      <!-- Gradient overlay -->
      <div class="absolute inset-0 bg-gradient-to-t from-black/70 via-black/20 to-transparent" />

      <!-- Badge statut -->
      <div class="absolute top-4 right-4">
        <span :class="['px-3 py-1 rounded-full text-xs font-medium', statutInfo.color]">
          {{ statutInfo.label }}
        </span>
      </div>

      <!-- Badge pays -->
      <div class="absolute top-4 left-4">
        <span class="bg-white/90 backdrop-blur-sm px-3 py-1 rounded-full text-xs font-medium text-gray-700">
          {{ paysLabel }}
        </span>
      </div>

      <!-- Titre en bas de l'image -->
      <div class="absolute bottom-0 left-0 right-0 p-4 text-white">
        <h3 class="text-lg font-bold line-clamp-2">{{ projet.titre }}</h3>
        <p class="text-sm text-white/80">{{ projet.organisation }}</p>
      </div>
    </div>

    <!-- Contenu de la carte -->
    <div class="p-5">
      <!-- Description courte -->
      <p class="text-gray-600 text-sm line-clamp-2 mb-4">
        {{ projet.resume || projet.description }}
      </p>

      <!-- Informations principales -->
      <div class="space-y-2 mb-4">
        <div class="flex items-center gap-2 text-sm text-gray-700">
          <font-awesome-icon :icon="['fas', 'coins']" class="w-4 h-4 text-custom-green" />
          <span class="font-semibold">{{ coutFormate }}</span>
        </div>
        <div class="flex items-center gap-2 text-sm text-gray-600">
          <font-awesome-icon :icon="['fas', 'clock']" class="w-4 h-4 text-gray-400" />
          <span>{{ projet.duree }}</span>
        </div>
        <div v-if="projet.dateDebutSouhaitee" class="flex items-center gap-2 text-sm text-gray-600">
          <font-awesome-icon :icon="['fas', 'calendar']" class="w-4 h-4 text-gray-400" />
          <span>Début: {{ dateDebutFormatee }}</span>
        </div>
      </div>

      <!-- Objectifs (2 premiers) -->
      <div v-if="projet.objectifs && projet.objectifs.length > 0" class="mb-4">
        <div class="flex flex-wrap gap-1">
          <span
            v-for="(objectif, index) in projet.objectifs.slice(0, 2)"
            :key="index"
            class="px-2 py-1 bg-emerald-50 text-emerald-700 rounded text-xs"
          >
            {{ truncateObjectif(objectif) }}
          </span>
          <span
            v-if="projet.objectifs.length > 2"
            class="px-2 py-1 bg-gray-100 text-gray-600 rounded text-xs"
          >
            +{{ projet.objectifs.length - 2 }}
          </span>
        </div>
      </div>

      <!-- Footer avec avatar et bouton -->
      <div class="flex items-center justify-between pt-4 border-t border-gray-100">
        <div class="flex items-center gap-2">
          <div
            v-if="projet.userInfo?.photoURL"
            class="w-8 h-8 rounded-full overflow-hidden"
          >
            <img
              :src="projet.userInfo.photoURL"
              :alt="projet.userInfo.prenom"
              class="w-full h-full object-cover"
            />
          </div>
          <div
            v-else
            class="w-8 h-8 rounded-full bg-custom-green text-white flex items-center justify-center text-xs font-semibold"
          >
            {{ initiales }}
          </div>
          <span class="text-xs text-gray-500">
            {{ projet.userInfo?.prenom }} {{ projet.userInfo?.nom }}
          </span>
        </div>
        <NuxtLink
          :to="`/financer-projet/${projet.id}`"
          class="px-4 py-2 bg-gradient-to-r from-custom-green to-emerald-600 text-white text-sm rounded-lg font-medium hover:shadow-lg transform hover:scale-[1.02] transition-all"
          @click.stop
        >
          Voir plus
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Projet } from '~/mocks/projets'
import { getStatutInfo, getPaysLabel, formatCurrency, formatDate, getInitiales } from '~/mocks/projets'

const props = defineProps<{
  projet: Projet
}>()

defineEmits<{
  click: [projet: Projet]
}>()

// Computed pour le statut
const statutInfo = computed(() => {
  return getStatutInfo(props.projet.statut)
})

// Computed pour le pays
const paysLabel = computed(() => {
  return getPaysLabel(props.projet.pays)
})

// Computed pour le coût formaté
const coutFormate = computed(() => {
  return formatCurrency(props.projet.coutTotal, props.projet.devise)
})

// Computed pour la date de début formatée
const dateDebutFormatee = computed(() => {
  if (!props.projet.dateDebutSouhaitee) return ''
  return formatDate(props.projet.dateDebutSouhaitee)
})

// Computed pour les initiales
const initiales = computed(() => {
  return getInitiales(props.projet.userInfo?.nom, props.projet.userInfo?.prenom)
})

// Fonction pour tronquer les objectifs
const truncateObjectif = (objectif: string, maxLength = 25): string => {
  if (objectif.length <= maxLength) return objectif
  return objectif.substring(0, maxLength) + '...'
}
</script>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
