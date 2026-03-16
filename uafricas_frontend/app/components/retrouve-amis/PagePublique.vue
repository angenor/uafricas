<script setup lang="ts">
import type { AvisPublicDetail } from '~/composables/useRetrouvAmis'
import { TYPES_RELATION, GENRES_PERSONNE, formatDate, formatPeriode } from '~/composables/useRetrouvAmis'

const props = defineProps<{
  avis: AvisPublicDetail
}>()

const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string

const auteurDisplay = computed(() => {
  if (props.avis.auteur_anonyme) return 'Anonyme'
  return props.avis.auteur_pseudonyme ?? 'Anonyme'
})

const photoComplete = computed(() => {
  if (!props.avis.photo_url) return null
  if (props.avis.photo_url.startsWith('http')) return props.avis.photo_url
  return `${apiBase}${props.avis.photo_url}`
})

const labelRelation = computed(() => {
  if (!props.avis.type_relation) return null
  return TYPES_RELATION.find(t => t.value === props.avis.type_relation)?.label ?? props.avis.type_relation
})

const labelGenre = computed(() => {
  if (!props.avis.genre_recherche) return null
  return GENRES_PERSONNE.find(g => g.value === props.avis.genre_recherche)?.label ?? props.avis.genre_recherche
})

const lieuxRencontre = computed(() => {
  const lieux: { label: string; valeur: string; icone: string }[] = []
  if (props.avis.localite_rencontre) lieux.push({ label: 'Localite', valeur: props.avis.localite_rencontre, icone: 'map-pin' })
  if (props.avis.ecole_rencontre) lieux.push({ label: 'Ecole / Lieu', valeur: props.avis.ecole_rencontre, icone: 'school' })
  if (props.avis.ville_rencontre) lieux.push({ label: 'Ville', valeur: props.avis.ville_rencontre, icone: 'city' })
  return lieux
})
</script>

<template>
  <article class="max-w-3xl mx-auto">
    <div class="bg-white rounded-xl shadow-sm border border-gray-200 overflow-hidden">
      <!-- En-tete -->
      <div class="bg-gradient-to-r from-custom-chocolat to-amber-700 px-6 py-8 text-white">
        <div class="flex items-center gap-3 mb-2">
          <span
            v-if="labelGenre"
            class="text-xs px-2.5 py-1 rounded-full bg-white/20 text-white"
          >
            <font-awesome-icon :icon="['fas', props.avis.genre_recherche === 'homme' ? 'mars' : 'venus']" class="mr-1" />
            {{ labelGenre }}
          </span>
          <span
            v-if="labelRelation"
            class="text-xs px-2.5 py-1 rounded-full bg-white/20 text-white"
          >
            {{ labelRelation }}
          </span>
        </div>
        <h1 class="text-2xl md:text-3xl font-bold mb-1">
          {{ props.avis.nom_recherche }}
          <span v-if="props.avis.prenom_recherche" class="font-normal">
            {{ props.avis.prenom_recherche }}
          </span>
        </h1>
        <p v-if="props.avis.surnom" class="text-white/70 text-sm italic mb-2">
          dit « {{ props.avis.surnom }} »
        </p>
        <p class="text-white/80 text-sm">
          Avis de recherche publie par <span class="font-medium text-white">{{ auteurDisplay }}</span>
        </p>
      </div>

      <!-- Photo -->
      <div v-if="photoComplete" class="relative">
        <img
          :src="photoComplete"
          :alt="`Photo de ${props.avis.nom_recherche}`"
          class="w-full max-h-96 object-cover"
        >
      </div>

      <div class="p-6 space-y-6">
        <!-- Lieux de rencontre -->
        <div v-if="lieuxRencontre.length > 0 || props.avis.jamais_rencontre">
          <h2 class="text-sm text-gray-500 uppercase tracking-wide font-medium mb-3">Lieu de rencontre</h2>
          <div v-if="props.avis.jamais_rencontre" class="flex items-center gap-2 text-gray-600">
            <font-awesome-icon :icon="['fas', 'circle-info']" class="text-gray-400" />
            <span class="italic">Jamais rencontre en personne</span>
          </div>
          <div v-else class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <div v-for="lieu in lieuxRencontre" :key="lieu.label" class="flex items-start gap-3">
              <div class="w-10 h-10 rounded-full bg-amber-50 flex items-center justify-center shrink-0">
                <font-awesome-icon :icon="['fas', lieu.icone]" class="text-amber-700" />
              </div>
              <div>
                <p class="text-xs text-gray-500 uppercase tracking-wide font-medium">{{ lieu.label }}</p>
                <p class="text-gray-800 font-medium">{{ lieu.valeur }}</p>
              </div>
            </div>
          </div>
        </div>

        <!-- Informations complementaires (anciens champs) -->
        <div v-if="props.avis.ecole || props.avis.ville || props.avis.pays || props.avis.periode_debut || props.avis.periode_fin" class="grid grid-cols-1 sm:grid-cols-2 gap-4">
          <div v-if="props.avis.ecole" class="flex items-start gap-3">
            <div class="w-10 h-10 rounded-full bg-amber-50 flex items-center justify-center shrink-0">
              <font-awesome-icon :icon="['fas', 'graduation-cap']" class="text-amber-700" />
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
        </div>

        <!-- Comment connu -->
        <div v-if="props.avis.comment_connu" class="border-t border-gray-100 pt-6">
          <h2 class="text-sm text-gray-500 uppercase tracking-wide font-medium mb-3">Comment la personne me connait</h2>
          <p class="text-gray-700 leading-relaxed whitespace-pre-line">{{ props.avis.comment_connu }}</p>
        </div>

        <!-- Description physique -->
        <div v-if="props.avis.description_physique" class="border-t border-gray-100 pt-6">
          <h2 class="text-sm text-gray-500 uppercase tracking-wide font-medium mb-3">Description physique</h2>
          <p class="text-gray-700 leading-relaxed whitespace-pre-line">{{ props.avis.description_physique }}</p>
        </div>

        <!-- Description generale -->
        <div v-if="props.avis.description" class="border-t border-gray-100 pt-6">
          <h2 class="text-sm text-gray-500 uppercase tracking-wide font-medium mb-3">Description</h2>
          <p class="text-gray-700 leading-relaxed whitespace-pre-line">{{ props.avis.description }}</p>
        </div>

        <!-- Partages + date -->
        <div class="border-t border-gray-100 pt-4 flex items-center justify-between text-sm text-gray-500">
          <span class="flex items-center gap-1.5">
            <font-awesome-icon :icon="['fas', 'share-nodes']" class="text-gray-400" />
            {{ props.avis.compteur_partages }} partage(s)
          </span>
          <span>
            Publie le {{ formatDate(props.avis.created_at) }}
          </span>
        </div>
      </div>
    </div>
  </article>
</template>
