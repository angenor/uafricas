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
  if (props.avis.ecole_rencontre) lieux.push({ label: 'École / Lieu', valeur: props.avis.ecole_rencontre, icone: 'school' })
  if (props.avis.ville_rencontre) lieux.push({ label: 'Ville', valeur: props.avis.ville_rencontre, icone: 'city' })
  return lieux
})
</script>

<template>
  <article class="max-w-4xl mx-auto">
    <div class="overflow-hidden rounded-2xl bg-white shadow-lg ring-1 ring-gray-200/60">

      <!-- ═══════════════════════════════════════════════════════
           BANDEAU "AVIS DE RECHERCHE"
           ═══════════════════════════════════════════════════════ -->
      <div class="bg-custom-chocolat px-6 py-4 text-center">
        <div class="flex items-center justify-center gap-3">
          <span class="hidden sm:block h-px w-12 bg-white/30" />
          <div>
            <p class="text-[11px] font-semibold uppercase tracking-[0.25em] text-white/60">Retrouv'Amis</p>
            <h1 class="text-xl md:text-2xl font-black uppercase tracking-wider text-white font-[Oswald]">
              Avis de recherche
            </h1>
          </div>
          <span class="hidden sm:block h-px w-12 bg-white/30" />
        </div>
      </div>

      <!-- ═══════════════════════════════════════════════════════
           QUI RECHERCHE QUI : la zone principale
           ═══════════════════════════════════════════════════════ -->
      <div class="bg-linear-to-b from-amber-50/80 to-white px-6 py-8 md:px-10 md:py-10">
        <!-- Phrase narrative -->
        <p class="mb-6 text-center text-sm text-gray-500">
          <span class="font-semibold text-custom-chocolat">{{ auteurDisplay }}</span>
          recherche
          <span v-if="labelRelation" class="text-gray-600">{{ labelRelation === 'Autre' ? 'une connaissance' : (['Ami(e)', 'Collegue', 'Voisin(e)'].includes(labelRelation!) ? `un(e) ${labelRelation!.toLowerCase()}` : `un(e) ancien(ne) ${labelRelation!.toLowerCase()}`) }}</span>
          <span v-else>une personne</span>
          perdue de vue
          <span class="text-gray-400 mx-1">·</span>
          <span class="text-gray-400">{{ formatDate(props.avis.created_at) }}</span>
        </p>

        <!-- Carte personne recherchee -->
        <div class="relative rounded-2xl bg-white p-6 md:p-8 shadow-md ring-1 ring-gray-200/60">
          <!-- Badge "Personne recherchee" -->
          <div class="absolute -top-3 left-6 md:left-8">
            <span class="inline-flex items-center gap-1.5 rounded-full bg-amber-600 px-4 py-1.5 text-xs font-bold uppercase tracking-wider text-white shadow-sm">
              <font-awesome-icon :icon="['fas', 'magnifying-glass']" class="text-[10px]" />
              Personne recherchee
            </span>
          </div>

          <div class="mt-3 flex flex-col sm:flex-row gap-6 items-start">
            <!-- Photo ou silhouette -->
            <div class="shrink-0 mx-auto sm:mx-0">
              <div v-if="photoComplete" class="h-40 w-40 md:h-48 md:w-48 overflow-hidden rounded-xl ring-2 ring-gray-200 shadow-lg">
                <img
                  :src="photoComplete"
                  :alt="`Photo de ${props.avis.nom_recherche}`"
                  class="h-full w-full object-cover"
                >
              </div>
              <div v-else class="relative flex h-40 w-40 md:h-48 md:w-48 flex-col items-center justify-center overflow-hidden rounded-xl bg-gray-100 ring-2 ring-gray-200">
                <!-- Silhouette tete + epaules -->
                <div class="relative mb-1">
                  <div class="h-14 w-14 md:h-16 md:w-16 rounded-full bg-gray-300/60" />
                  <div class="absolute -bottom-3 left-1/2 -translate-x-1/2 h-9 w-20 md:h-10 md:w-24 rounded-t-full bg-gray-300/60" />
                </div>
                <span class="mt-6 text-[10px] font-semibold uppercase tracking-wider text-gray-400">Photo non disponible</span>
              </div>
            </div>

            <!-- Identite -->
            <div class="flex-1 text-center sm:text-left">
              <h2 class="text-2xl md:text-3xl font-bold text-gray-900 mb-1">
                {{ props.avis.prenom_recherche }}
                <span class="uppercase">{{ props.avis.nom_recherche }}</span>
              </h2>
              <p v-if="props.avis.surnom" class="text-gray-500 text-sm italic mb-3">
                dit « {{ props.avis.surnom }} »
              </p>

              <!-- Badges -->
              <div class="flex flex-wrap justify-center sm:justify-start gap-2 mb-4">
                <span
                  v-if="labelGenre"
                  class="inline-flex items-center gap-1.5 rounded-full bg-gray-100 px-3 py-1 text-xs font-medium text-gray-600"
                >
                  <font-awesome-icon :icon="['fas', props.avis.genre_recherche === 'homme' ? 'mars' : 'venus']" class="text-[10px]" />
                  {{ labelGenre }}
                </span>
                <span
                  v-if="labelRelation"
                  class="inline-flex items-center gap-1.5 rounded-full bg-amber-100 px-3 py-1 text-xs font-semibold text-amber-700"
                >
                  <font-awesome-icon :icon="['fas', 'link']" class="text-[10px]" />
                  {{ labelRelation }}
                </span>
              </div>

              <!-- Description physique en apercu rapide -->
              <p v-if="props.avis.description_physique" class="text-sm text-gray-600 leading-relaxed line-clamp-3">
                {{ props.avis.description_physique }}
              </p>
            </div>
          </div>
        </div>
      </div>

      <!-- ═══════════════════════════════════════════════════════
           DETAILS : ou et quand
           ═══════════════════════════════════════════════════════ -->
      <div class="px-6 md:px-10 pb-8 space-y-8">

        <!-- Grille infos cles -->
        <div v-if="lieuxRencontre.length > 0 || props.avis.jamais_rencontre || props.avis.ecole || props.avis.ville || props.avis.pays || props.avis.periode_debut || props.avis.periode_fin">
          <h2 class="mb-4 flex items-center gap-2 text-xs font-bold uppercase tracking-widest text-gray-400">
            <font-awesome-icon :icon="['fas', 'map-location-dot']" class="text-amber-600" />
            Ou et quand se sont-ils connus ?
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
                <p class="text-[11px] font-semibold uppercase tracking-wider text-gray-400">École</p>
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
                <p class="text-[11px] font-semibold uppercase tracking-wider text-gray-400">Période</p>
                <p class="text-sm font-medium text-gray-800">{{ formatPeriode(props.avis.periode_debut, props.avis.periode_fin) }}</p>
              </div>
            </div>
          </div>
        </div>

        <!-- Comment connu -->
        <div v-if="props.avis.comment_connu">
          <h2 class="mb-4 flex items-center gap-2 text-xs font-bold uppercase tracking-widest text-gray-400">
            <font-awesome-icon :icon="['fas', 'comments']" class="text-amber-600" />
            Comment la personne me connait
            <span class="h-px flex-1 bg-gray-200" />
          </h2>
          <div class="rounded-xl bg-amber-50/50 p-5 ring-1 ring-amber-100">
            <p class="text-sm leading-relaxed text-gray-700 whitespace-pre-line">{{ props.avis.comment_connu }}</p>
          </div>
        </div>

        <!-- Description physique complete -->
        <div v-if="props.avis.description_physique">
          <h2 class="mb-4 flex items-center gap-2 text-xs font-bold uppercase tracking-widest text-gray-400">
            <font-awesome-icon :icon="['fas', 'id-card']" class="text-amber-600" />
            Description physique
            <span class="h-px flex-1 bg-gray-200" />
          </h2>
          <div class="rounded-xl bg-gray-50 p-5 ring-1 ring-gray-100">
            <p class="text-sm leading-relaxed text-gray-700 whitespace-pre-line">{{ props.avis.description_physique }}</p>
          </div>
        </div>

        <!-- Description generale / message de l'auteur -->
        <div v-if="props.avis.description">
          <h2 class="mb-4 flex items-center gap-2 text-xs font-bold uppercase tracking-widest text-gray-400">
            <font-awesome-icon :icon="['fas', 'pen-nib']" class="text-amber-600" />
            Message de l'auteur
            <span class="h-px flex-1 bg-gray-200" />
          </h2>
          <div class="rounded-xl border-l-4 border-custom-chocolat bg-amber-50/30 p-5">
            <p class="text-sm leading-relaxed text-gray-700 whitespace-pre-line italic">{{ props.avis.description }}</p>
            <p class="mt-3 text-xs text-gray-400">
              {{ auteurDisplay }}
            </p>
          </div>
        </div>

        <!-- Appel a l'action -->
        <div class="rounded-xl bg-linear-to-r from-custom-green/10 to-emerald-50 p-5 ring-1 ring-green-200/60 text-center">
          <p class="text-sm font-medium text-gray-700 mb-1">
            <font-awesome-icon :icon="['fas', 'heart']" class="text-custom-green mr-1" />
            Vous reconnaissez cette personne ?
          </p>
          <p class="text-xs text-gray-500">
            Repondez a cet avis ci-dessous ou partagez-le pour augmenter les chances de retrouvailles.
          </p>
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
