<template>
  <NuxtLink
    :to="`/marche-africain/${annonce.id}`"
    class="group block bg-white rounded-xl overflow-hidden shadow-md hover:shadow-xl transition-all duration-300 hover:-translate-y-1"
  >
    <!-- Image container -->
    <div class="relative aspect-[16/10] overflow-hidden">
      <img
        :src="annonce.photo_url || '/images/placeholder.jpg'"
        :alt="annonce.titre"
        class="w-full h-full object-cover transition-transform duration-500 group-hover:scale-110"
      />

      <!-- Badge type d'échange -->
      <span
        class="absolute top-3 left-3 px-3 py-1.5 rounded-full text-xs font-semibold shadow-xs"
        :class="getTypeColor(annonce.type_echange)"
      >
        {{ annonce.type_echange }}
      </span>

      <!-- Badge quantité minimum -->
      <span
        v-if="annonce.quantite && annonce.quantite > 1"
        class="absolute top-3 right-3 px-2 py-1 bg-amber-100/90 text-amber-700 rounded-full text-xs font-medium"
      >
        Min. {{ annonce.quantite }} unités
      </span>

      <!-- Overlay gradient -->
      <div class="absolute inset-0 bg-gradient-to-t from-black/20 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
    </div>

    <!-- Contenu -->
    <div class="p-4">
      <!-- Localisation -->
      <div class="flex items-center text-sm text-gray-500 mb-2">
        <font-awesome-icon :icon="['fas', 'location-dot']" class="w-3 h-3 mr-1.5 text-custom-green" />
        <span>{{ annonce.pays }}</span>
        <span v-if="annonce.ville" class="text-gray-300 mx-1">•</span>
        <span v-if="annonce.ville" class="text-gray-400">{{ annonce.ville }}</span>
      </div>

      <!-- Titre -->
      <h3 class="font-semibold text-gray-800 line-clamp-2 mb-3 group-hover:text-custom-green transition-colors">
        {{ annonce.titre }}
      </h3>

      <!-- Prix et catégorie -->
      <div class="flex items-center justify-between">
        <span
          class="text-lg font-bold"
          :class="annonce.type_echange === 'Don' ? 'text-blue-600' : 'text-custom-chocolat'"
        >
          {{ prixFormate }}
        </span>
        <span class="text-xs text-gray-400 bg-gray-100 px-2 py-1 rounded">
          {{ annonce.categorie }}
        </span>
      </div>

      <!-- Date -->
      <div class="mt-3 pt-3 border-t border-gray-100 flex items-center text-xs text-gray-400">
        <font-awesome-icon :icon="['fas', 'calendar-days']" class="w-3 h-3 mr-1.5" />
        {{ dateFormatee }}
      </div>
    </div>
  </NuxtLink>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { formatPrix, formatDateCourte, type AnnonceAPI, type TypeEchange } from '~/composables/useMarcheAfricain'

const props = defineProps<{
  annonce: AnnonceAPI
}>()

const prixFormate = computed(() => {
  return formatPrix(props.annonce.prix, props.annonce.devise)
})

const dateFormatee = computed(() => {
  return formatDateCourte(props.annonce.created_at)
})

const getTypeColor = (type: string): string => {
  switch (type as TypeEchange) {
    case 'Vente':
      return 'bg-white/95 text-gray-700 border border-gray-200'
    case 'Troc':
      return 'bg-purple-100/95 text-purple-700'
    case 'Don':
      return 'bg-blue-100/95 text-blue-700'
    default:
      return 'bg-gray-100 text-gray-700'
  }
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
