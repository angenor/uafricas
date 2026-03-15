<script setup lang="ts">
import type { AvisPublicResume } from '~/composables/useRetrouvAmis'
import { TYPES_RELATION, formatDate } from '~/composables/useRetrouvAmis'

const props = defineProps<{
  avis: AvisPublicResume
}>()

const labelRelation = computed(() => {
  if (!props.avis.type_relation) return null
  return TYPES_RELATION.find(t => t.value === props.avis.type_relation)?.label ?? props.avis.type_relation
})

const lieuRencontre = computed(() => {
  const parts = [props.avis.localite_rencontre, props.avis.ville_rencontre].filter(Boolean)
  return parts.length > 0 ? parts.join(', ') : null
})

const auteurDisplay = computed(() => {
  if (props.avis.auteur_anonyme) return 'Anonyme'
  return props.avis.auteur_pseudonyme ?? 'Anonyme'
})
</script>

<template>
  <NuxtLink
    :to="`/retrouve-amis/public/${props.avis.slug}`"
    class="block bg-white rounded-xl shadow-sm border border-gray-200 hover:shadow-md hover:border-amber-200 transition-all group overflow-hidden relative"
  >
    <!-- Bandeau "Personne retrouvee !" pour les avis clotures -->
    <div
      v-if="props.avis.etat === 'cloture'"
      class="absolute top-0 left-0 right-0 z-10 bg-green-600 text-white text-center py-2 text-sm font-semibold"
    >
      <font-awesome-icon :icon="['fas', 'heart']" class="mr-1.5" />
      Personne retrouvee !
    </div>

    <!-- Photo si disponible -->
    <div v-if="props.avis.photo_url" class="relative h-48 bg-gray-100" :class="{ 'mt-9': props.avis.etat === 'cloture' }">
      <img
        :src="props.avis.photo_url"
        :alt="`Photo de ${props.avis.nom_recherche}`"
        class="w-full h-full object-cover"
        :class="{ 'opacity-70': props.avis.etat === 'cloture' }"
      >
    </div>

    <div class="p-5">
      <!-- Nom recherche + genre -->
      <div class="flex items-start justify-between mb-2">
        <h3 class="text-lg font-semibold text-gray-800 group-hover:text-amber-700 transition-colors">
          {{ props.avis.nom_recherche }}
          <span v-if="props.avis.prenom_recherche" class="font-normal text-gray-600">
            {{ props.avis.prenom_recherche }}
          </span>
        </h3>
        <span
          v-if="props.avis.genre_recherche"
          class="text-xs px-2 py-0.5 rounded-full bg-gray-100 text-gray-500 shrink-0 ml-2"
        >
          <font-awesome-icon :icon="['fas', props.avis.genre_recherche === 'homme' ? 'mars' : 'venus']" class="mr-1" />
          {{ props.avis.genre_recherche === 'homme' ? 'Homme' : 'Femme' }}
        </span>
      </div>

      <!-- Type de relation -->
      <span
        v-if="labelRelation"
        class="inline-block text-xs px-2.5 py-1 rounded-full bg-amber-50 text-amber-700 border border-amber-200 mb-3"
      >
        {{ labelRelation }}
      </span>

      <!-- Infos cles -->
      <div class="space-y-1.5 text-sm text-gray-600 mb-3">
        <div v-if="lieuRencontre || props.avis.ecole_rencontre" class="flex items-center gap-2">
          <font-awesome-icon :icon="['fas', 'location-dot']" class="w-4 text-gray-400" />
          <span>
            <span v-if="lieuRencontre">{{ lieuRencontre }}</span>
            <span v-if="lieuRencontre && props.avis.ecole_rencontre"> — </span>
            <span v-if="props.avis.ecole_rencontre" class="italic">{{ props.avis.ecole_rencontre }}</span>
          </span>
        </div>

        <div v-if="props.avis.description_physique" class="flex items-start gap-2">
          <font-awesome-icon :icon="['fas', 'user']" class="w-4 text-gray-400 mt-0.5" />
          <span class="line-clamp-2">{{ props.avis.description_physique }}</span>
        </div>
      </div>

      <!-- Footer : auteur + partages + date -->
      <div class="flex items-center justify-between pt-3 border-t border-gray-100 text-xs text-gray-500">
        <span class="flex items-center gap-1.5">
          <font-awesome-icon :icon="['fas', 'user-circle']" class="text-gray-400" />
          {{ auteurDisplay }}
        </span>
        <span class="flex items-center gap-3">
          <span class="flex items-center gap-1">
            <font-awesome-icon :icon="['fas', 'share-nodes']" />
            {{ props.avis.compteur_partages }}
          </span>
          <span>{{ formatDate(props.avis.created_at) }}</span>
        </span>
      </div>
    </div>
  </NuxtLink>
</template>
