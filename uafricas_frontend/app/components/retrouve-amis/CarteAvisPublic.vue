<script setup lang="ts">
import type { AvisPublicResume } from '~/composables/useRetrouvAmis'
import { TYPES_RELATION, formatDate } from '~/composables/useRetrouvAmis'

const props = defineProps<{
  avis: AvisPublicResume
}>()

const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string

const photoComplete = computed(() => {
  if (!props.avis.photo_url) return null
  if (props.avis.photo_url.startsWith('http')) return props.avis.photo_url
  return `${apiBase}${props.avis.photo_url}`
})

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
    class="group relative flex flex-col overflow-hidden rounded-2xl bg-white shadow-md ring-1 ring-gray-200/60 transition-all duration-300 hover:shadow-xl hover:ring-amber-300 hover:-translate-y-1"
  >
    <!-- Badge etat en haut a gauche -->
    <div
      v-if="props.avis.etat === 'cloture'"
      class="absolute top-3 left-3 z-20 flex items-center gap-1.5 rounded-full bg-green-600 px-3 py-1 text-xs font-bold text-white shadow-lg"
    >
      <font-awesome-icon :icon="['fas', 'heart']" />
      Retrouve(e) !
    </div>

    <!-- Zone visuelle : photo ou avatar stylise -->
    <div class="relative h-56 overflow-hidden bg-linear-to-br from-amber-100 to-amber-50">
      <img
        v-if="photoComplete"
        :src="photoComplete"
        :alt="`Photo de ${props.avis.nom_recherche}`"
        class="h-full w-full object-cover transition-transform duration-500 group-hover:scale-105"
        :class="{ 'grayscale opacity-60': props.avis.etat === 'cloture' }"
      >
      <!-- Placeholder si pas de photo -->
      <div v-else class="flex h-full w-full items-center justify-center">
        <div class="flex h-24 w-24 items-center justify-center rounded-full bg-amber-200/60 text-amber-600">
          <font-awesome-icon :icon="['fas', 'user']" class="text-4xl" />
        </div>
      </div>
      <!-- Degrade bas pour lisibilite -->
      <div class="absolute inset-x-0 bottom-0 h-20 bg-linear-to-t from-black/50 to-transparent" />
      <!-- Nom en overlay bas -->
      <div class="absolute inset-x-0 bottom-0 p-4">
        <h3 class="text-lg font-bold text-white drop-shadow-md">
          {{ props.avis.prenom_recherche }}
          <span class="uppercase">{{ props.avis.nom_recherche }}</span>
        </h3>
      </div>
    </div>

    <!-- Contenu -->
    <div class="flex flex-1 flex-col p-4">
      <!-- Badges : relation + genre -->
      <div class="mb-3 flex flex-wrap items-center gap-2">
        <span
          v-if="labelRelation"
          class="inline-flex items-center gap-1 rounded-full bg-amber-600/10 px-2.5 py-1 text-xs font-semibold text-amber-700"
        >
          <font-awesome-icon :icon="['fas', 'link']" class="text-[10px]" />
          {{ labelRelation }}
        </span>
        <span
          v-if="props.avis.genre_recherche"
          class="inline-flex items-center gap-1 rounded-full bg-gray-100 px-2.5 py-1 text-xs font-medium text-gray-600"
        >
          <font-awesome-icon :icon="['fas', props.avis.genre_recherche === 'homme' ? 'mars' : 'venus']" class="text-[10px]" />
          {{ props.avis.genre_recherche === 'homme' ? 'Homme' : 'Femme' }}
        </span>
      </div>

      <!-- Infos cles compactes -->
      <div class="space-y-2 text-sm text-gray-600">
        <div v-if="lieuRencontre" class="flex items-center gap-2">
          <span class="flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-red-50 text-red-400">
            <font-awesome-icon :icon="['fas', 'location-dot']" class="text-xs" />
          </span>
          <span class="truncate">{{ lieuRencontre }}</span>
        </div>
        <div v-if="props.avis.ecole_rencontre" class="flex items-center gap-2">
          <span class="flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-blue-50 text-blue-400">
            <font-awesome-icon :icon="['fas', 'graduation-cap']" class="text-xs" />
          </span>
          <span class="truncate">{{ props.avis.ecole_rencontre }}</span>
        </div>
        <div v-if="props.avis.description_physique" class="flex items-start gap-2">
          <span class="flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-purple-50 text-purple-400 mt-0.5">
            <font-awesome-icon :icon="['fas', 'id-card']" class="text-xs" />
          </span>
          <span class="line-clamp-2 text-gray-500">{{ props.avis.description_physique }}</span>
        </div>
      </div>

      <!-- Spacer -->
      <div class="flex-1" />

      <!-- Footer -->
      <div class="mt-4 flex items-center justify-between border-t border-gray-100 pt-3">
        <div class="flex items-center gap-2 text-xs text-gray-500">
          <div class="flex h-6 w-6 items-center justify-center rounded-full bg-gray-100">
            <font-awesome-icon :icon="['fas', 'user']" class="text-[10px] text-gray-400" />
          </div>
          <span>{{ auteurDisplay }}</span>
        </div>
        <div class="flex items-center gap-3 text-xs text-gray-400">
          <span v-if="props.avis.compteur_partages > 0" class="flex items-center gap-1">
            <font-awesome-icon :icon="['fas', 'share-nodes']" />
            {{ props.avis.compteur_partages }}
          </span>
          <span>{{ formatDate(props.avis.created_at) }}</span>
        </div>
      </div>
    </div>
  </NuxtLink>
</template>
