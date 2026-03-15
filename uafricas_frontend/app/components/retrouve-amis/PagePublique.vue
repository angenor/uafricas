<script setup lang="ts">
import type { AvisPublicDetail } from '~/composables/useRetrouvAmis'
import { formatDate, formatPeriode } from '~/composables/useRetrouvAmis'

const props = defineProps<{
  avis: AvisPublicDetail
}>()

const auteurDisplay = computed(() => {
  if (props.avis.auteur_anonyme) return 'Anonyme'
  return props.avis.auteur_pseudonyme ?? 'Anonyme'
})
</script>

<template>
  <article class="max-w-3xl mx-auto">
    <!-- En-tete -->
    <div class="bg-white rounded-xl shadow-sm border border-gray-200 overflow-hidden">
      <div class="bg-gradient-to-r from-custom-chocolat to-amber-700 px-6 py-8 text-white">
        <h1 class="text-2xl md:text-3xl font-bold mb-2">
          {{ props.avis.nom_recherche }}
          <span v-if="props.avis.prenom_recherche" class="font-normal">
            {{ props.avis.prenom_recherche }}
          </span>
        </h1>
        <p class="text-white/80 text-sm">
          Avis de recherche publie par <span class="font-medium text-white">{{ auteurDisplay }}</span>
        </p>
      </div>

      <div class="p-6 space-y-6">
        <!-- Informations cles -->
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
          <div v-if="props.avis.ecole" class="flex items-start gap-3">
            <div class="w-10 h-10 rounded-full bg-amber-50 flex items-center justify-center shrink-0">
              <font-awesome-icon :icon="['fas', 'school']" class="text-amber-700" />
            </div>
            <div>
              <p class="text-xs text-gray-500 uppercase tracking-wide font-medium">Ecole</p>
              <p class="text-gray-800 font-medium">{{ props.avis.ecole }}</p>
            </div>
          </div>

          <div v-if="props.avis.ville || props.avis.pays" class="flex items-start gap-3">
            <div class="w-10 h-10 rounded-full bg-amber-50 flex items-center justify-center shrink-0">
              <font-awesome-icon :icon="['fas', 'location-dot']" class="text-amber-700" />
            </div>
            <div>
              <p class="text-xs text-gray-500 uppercase tracking-wide font-medium">Lieu</p>
              <p class="text-gray-800 font-medium">
                <span v-if="props.avis.ville">{{ props.avis.ville }}</span>
                <span v-if="props.avis.ville && props.avis.pays">, </span>
                <span v-if="props.avis.pays">{{ props.avis.pays.nom }}</span>
              </p>
            </div>
          </div>

          <div v-if="props.avis.periode_debut || props.avis.periode_fin" class="flex items-start gap-3">
            <div class="w-10 h-10 rounded-full bg-amber-50 flex items-center justify-center shrink-0">
              <font-awesome-icon :icon="['fas', 'calendar']" class="text-amber-700" />
            </div>
            <div>
              <p class="text-xs text-gray-500 uppercase tracking-wide font-medium">Periode</p>
              <p class="text-gray-800 font-medium">{{ formatPeriode(props.avis.periode_debut, props.avis.periode_fin) }}</p>
            </div>
          </div>

          <div class="flex items-start gap-3">
            <div class="w-10 h-10 rounded-full bg-amber-50 flex items-center justify-center shrink-0">
              <font-awesome-icon :icon="['fas', 'share-nodes']" class="text-amber-700" />
            </div>
            <div>
              <p class="text-xs text-gray-500 uppercase tracking-wide font-medium">Partages</p>
              <p class="text-gray-800 font-medium">{{ props.avis.compteur_partages }}</p>
            </div>
          </div>
        </div>

        <!-- Description -->
        <div v-if="props.avis.description" class="border-t border-gray-100 pt-6">
          <h2 class="text-sm text-gray-500 uppercase tracking-wide font-medium mb-3">Description</h2>
          <p class="text-gray-700 leading-relaxed whitespace-pre-line">{{ props.avis.description }}</p>
        </div>

        <!-- Pied de page -->
        <div class="border-t border-gray-100 pt-4 flex items-center justify-between text-sm text-gray-500">
          <span v-if="props.avis.date_publication_publique">
            Publie le {{ formatDate(props.avis.date_publication_publique) }}
          </span>
          <span>
            Cree le {{ formatDate(props.avis.created_at) }}
          </span>
        </div>
      </div>
    </div>
  </article>
</template>
