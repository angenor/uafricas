<script setup lang="ts">
import type { AvisPublicResume } from '~/composables/useRetrouvAmis'
import { formatDate, formatPeriode } from '~/composables/useRetrouvAmis'

const props = defineProps<{
  avis: AvisPublicResume
}>()
</script>

<template>
  <NuxtLink
    :to="`/retrouve-amis/public/${props.avis.slug}`"
    class="block bg-white rounded-xl shadow-sm border border-gray-200 hover:shadow-md hover:border-amber-200 transition-all group"
  >
    <div class="p-5">
      <!-- Nom recherche -->
      <h3 class="text-lg font-semibold text-gray-800 group-hover:text-amber-700 transition-colors mb-2">
        {{ props.avis.nom_recherche }}
        <span v-if="props.avis.prenom_recherche" class="font-normal text-gray-600">
          {{ props.avis.prenom_recherche }}
        </span>
      </h3>

      <!-- Infos cles -->
      <div class="space-y-1.5 text-sm text-gray-600 mb-3">
        <div v-if="props.avis.ville || props.avis.pays" class="flex items-center gap-2">
          <font-awesome-icon :icon="['fas', 'location-dot']" class="w-4 text-gray-400" />
          <span>
            <span v-if="props.avis.ville">{{ props.avis.ville }}</span>
            <span v-if="props.avis.ville && props.avis.pays">, </span>
            <span v-if="props.avis.pays">{{ props.avis.pays.nom }}</span>
          </span>
        </div>

        <div v-if="props.avis.periode_debut || props.avis.periode_fin" class="flex items-center gap-2">
          <font-awesome-icon :icon="['fas', 'calendar']" class="w-4 text-gray-400" />
          <span>{{ formatPeriode(props.avis.periode_debut, props.avis.periode_fin) }}</span>
        </div>
      </div>

      <!-- Footer : partages + date -->
      <div class="flex items-center justify-between pt-3 border-t border-gray-100 text-xs text-gray-500">
        <span class="flex items-center gap-1.5">
          <font-awesome-icon :icon="['fas', 'share-nodes']" />
          {{ props.avis.compteur_partages }} partage{{ props.avis.compteur_partages !== 1 ? 's' : '' }}
        </span>
        <span>{{ formatDate(props.avis.created_at) }}</span>
      </div>
    </div>
  </NuxtLink>
</template>
