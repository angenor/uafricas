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
  <article class="max-w-4xl mx-auto">
    <div class="overflow-hidden rounded-2xl bg-white shadow-lg ring-1 ring-gray-200/60">
      <!-- En-tete avec photo cote a cote sur desktop -->
      <div class="relative">
        <!-- Fond degrade -->
        <div class="bg-linear-to-br from-custom-chocolat via-amber-800 to-amber-700 px-6 py-8 md:px-10 md:py-10">
          <div class="flex flex-col md:flex-row md:items-center gap-6">
            <!-- Photo ronde -->
            <div v-if="photoComplete" class="shrink-0">
              <div class="h-28 w-28 md:h-36 md:w-36 overflow-hidden rounded-2xl ring-4 ring-white/20 shadow-xl">
                <img
                  :src="photoComplete"
                  :alt="`Photo de ${props.avis.nom_recherche}`"
                  class="h-full w-full object-cover"
                >
              </div>
            </div>
            <div v-else class="shrink-0">
              <div class="relative flex h-28 w-28 md:h-36 md:w-36 flex-col items-center justify-center overflow-hidden rounded-2xl bg-white/10 ring-4 ring-white/20">
                <!-- Silhouette tete + epaules -->
                <div class="relative mb-0.5">
                  <div class="h-10 w-10 md:h-12 md:w-12 rounded-full bg-white/20" />
                  <div class="absolute -bottom-2.5 left-1/2 -translate-x-1/2 h-7 w-16 md:h-8 md:w-20 rounded-t-full bg-white/20" />
                </div>
                <span class="mt-5 text-[9px] font-semibold uppercase tracking-wider text-white/40">Inconnu(e)</span>
              </div>
            </div>

            <!-- Infos identite -->
            <div class="flex-1">
              <div class="flex flex-wrap items-center gap-2 mb-3">
                <span
                  v-if="labelGenre"
                  class="inline-flex items-center gap-1.5 rounded-full bg-white/15 px-3 py-1 text-xs font-medium text-white backdrop-blur-sm"
                >
                  <font-awesome-icon :icon="['fas', props.avis.genre_recherche === 'homme' ? 'mars' : 'venus']" class="text-[10px]" />
                  {{ labelGenre }}
                </span>
                <span
                  v-if="labelRelation"
                  class="inline-flex items-center gap-1.5 rounded-full bg-white/15 px-3 py-1 text-xs font-medium text-white backdrop-blur-sm"
                >
                  <font-awesome-icon :icon="['fas', 'link']" class="text-[10px]" />
                  {{ labelRelation }}
                </span>
              </div>
              <h1 class="text-2xl md:text-4xl font-bold text-white mb-1">
                {{ props.avis.prenom_recherche }}
                <span class="uppercase">{{ props.avis.nom_recherche }}</span>
              </h1>
              <p v-if="props.avis.surnom" class="text-white/60 text-sm italic mb-3">
                dit « {{ props.avis.surnom }} »
              </p>
              <p class="text-white/70 text-sm flex items-center gap-2">
                <font-awesome-icon :icon="['fas', 'user-pen']" class="text-xs" />
                Publie par <span class="font-semibold text-white/90">{{ auteurDisplay }}</span>
                <span class="text-white/40">·</span>
                {{ formatDate(props.avis.created_at) }}
              </p>
            </div>
          </div>
        </div>
      </div>

      <!-- Corps -->
      <div class="p-6 md:p-10 space-y-8">
        <!-- Grille infos cles -->
        <div v-if="lieuxRencontre.length > 0 || props.avis.jamais_rencontre || props.avis.ecole || props.avis.ville || props.avis.pays || props.avis.periode_debut || props.avis.periode_fin">
          <h2 class="mb-4 flex items-center gap-2 text-xs font-bold uppercase tracking-widest text-gray-400">
            <span class="h-px flex-1 bg-gray-200" />
            Informations
            <span class="h-px flex-1 bg-gray-200" />
          </h2>

          <div v-if="props.avis.jamais_rencontre" class="mb-4 flex items-center gap-3 rounded-xl bg-blue-50 p-4 text-sm text-blue-700">
            <font-awesome-icon :icon="['fas', 'circle-info']" />
            <span class="italic">Jamais rencontre en personne</span>
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
            <div
              v-for="lieu in lieuxRencontre"
              :key="lieu.label"
              class="flex items-center gap-3 rounded-xl bg-gray-50 p-4 ring-1 ring-gray-100"
            >
              <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-amber-100 text-amber-700">
                <font-awesome-icon :icon="['fas', lieu.icone]" />
              </div>
              <div>
                <p class="text-[11px] font-semibold uppercase tracking-wider text-gray-400">{{ lieu.label }}</p>
                <p class="text-sm font-medium text-gray-800">{{ lieu.valeur }}</p>
              </div>
            </div>

            <div
              v-if="props.avis.ecole"
              class="flex items-center gap-3 rounded-xl bg-gray-50 p-4 ring-1 ring-gray-100"
            >
              <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-blue-100 text-blue-600">
                <font-awesome-icon :icon="['fas', 'graduation-cap']" />
              </div>
              <div>
                <p class="text-[11px] font-semibold uppercase tracking-wider text-gray-400">Ecole</p>
                <p class="text-sm font-medium text-gray-800">{{ props.avis.ecole }}</p>
              </div>
            </div>

            <div
              v-if="props.avis.ville || props.avis.pays"
              class="flex items-center gap-3 rounded-xl bg-gray-50 p-4 ring-1 ring-gray-100"
            >
              <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-red-100 text-red-500">
                <font-awesome-icon :icon="['fas', 'location-dot']" />
              </div>
              <div>
                <p class="text-[11px] font-semibold uppercase tracking-wider text-gray-400">Lieu</p>
                <p class="text-sm font-medium text-gray-800">
                  <span v-if="props.avis.ville">{{ props.avis.ville }}</span>
                  <span v-if="props.avis.ville && props.avis.pays">, </span>
                  <span v-if="props.avis.pays">{{ props.avis.pays.nom }}</span>
                </p>
              </div>
            </div>

            <div
              v-if="props.avis.periode_debut || props.avis.periode_fin"
              class="flex items-center gap-3 rounded-xl bg-gray-50 p-4 ring-1 ring-gray-100"
            >
              <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-purple-100 text-purple-600">
                <font-awesome-icon :icon="['fas', 'calendar']" />
              </div>
              <div>
                <p class="text-[11px] font-semibold uppercase tracking-wider text-gray-400">Periode</p>
                <p class="text-sm font-medium text-gray-800">{{ formatPeriode(props.avis.periode_debut, props.avis.periode_fin) }}</p>
              </div>
            </div>
          </div>
        </div>

        <!-- Comment connu -->
        <div v-if="props.avis.comment_connu">
          <h2 class="mb-4 flex items-center gap-2 text-xs font-bold uppercase tracking-widest text-gray-400">
            <span class="h-px flex-1 bg-gray-200" />
            Comment la personne me connait
            <span class="h-px flex-1 bg-gray-200" />
          </h2>
          <div class="rounded-xl bg-amber-50/50 p-5 ring-1 ring-amber-100">
            <p class="text-sm leading-relaxed text-gray-700 whitespace-pre-line">{{ props.avis.comment_connu }}</p>
          </div>
        </div>

        <!-- Description physique -->
        <div v-if="props.avis.description_physique">
          <h2 class="mb-4 flex items-center gap-2 text-xs font-bold uppercase tracking-widest text-gray-400">
            <span class="h-px flex-1 bg-gray-200" />
            Description physique
            <span class="h-px flex-1 bg-gray-200" />
          </h2>
          <div class="rounded-xl bg-gray-50 p-5 ring-1 ring-gray-100">
            <p class="text-sm leading-relaxed text-gray-700 whitespace-pre-line">{{ props.avis.description_physique }}</p>
          </div>
        </div>

        <!-- Description generale -->
        <div v-if="props.avis.description">
          <h2 class="mb-4 flex items-center gap-2 text-xs font-bold uppercase tracking-widest text-gray-400">
            <span class="h-px flex-1 bg-gray-200" />
            Description
            <span class="h-px flex-1 bg-gray-200" />
          </h2>
          <div class="rounded-xl bg-gray-50 p-5 ring-1 ring-gray-100">
            <p class="text-sm leading-relaxed text-gray-700 whitespace-pre-line">{{ props.avis.description }}</p>
          </div>
        </div>

        <!-- Footer partages + date -->
        <div class="flex items-center justify-between rounded-xl bg-gray-50 px-5 py-4 ring-1 ring-gray-100 text-sm text-gray-500">
          <span class="flex items-center gap-2">
            <font-awesome-icon :icon="['fas', 'share-nodes']" class="text-amber-600" />
            <span class="font-semibold text-gray-700">{{ props.avis.compteur_partages }}</span> partage{{ props.avis.compteur_partages !== 1 ? 's' : '' }}
          </span>
          <span class="flex items-center gap-2">
            <font-awesome-icon :icon="['fas', 'clock']" class="text-gray-400" />
            Publie le {{ formatDate(props.avis.created_at) }}
          </span>
        </div>
      </div>
    </div>
  </article>
</template>
