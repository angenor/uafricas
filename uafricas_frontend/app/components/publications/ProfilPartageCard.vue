<script setup lang="ts">
import type { PartageProfilAPI } from '~/composables/useMembres'

const props = defineProps<{
  partage: PartageProfilAPI
}>()

const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string

const resoudreUrl = (url: string | null | undefined): string | null => {
  if (!url) return null
  return url.startsWith('http') ? url : `${apiBase}${url}`
}

const photoAuteur = computed(() => resoudreUrl(props.partage.auteur.photo_url))
const photoProfil = computed(() => resoudreUrl(props.partage.profil.photo_url))

const nomAuteur = computed(() => {
  const { prenom, nom } = props.partage.auteur
  return `${prenom ?? ''} ${nom ?? ''}`.trim() || 'Anonyme'
})

const initialesAuteur = computed(() => {
  const { prenom, nom } = props.partage.auteur
  return ((prenom?.[0] ?? '') + (nom?.[0] ?? '')).toUpperCase() || '?'
})

const initialesProfil = computed(() => {
  const { prenom, nom } = props.partage.profil
  return ((prenom?.[0] ?? '') + (nom?.[0] ?? '')).toUpperCase() || '?'
})

const localisation = computed(() => {
  const { pays, ville } = props.partage.profil
  return [ville, pays].filter(Boolean).join(', ')
})

const dateFormatee = computed(() =>
  new Intl.DateTimeFormat('fr-FR', { day: 'numeric', month: 'short', year: 'numeric' })
    .format(new Date(props.partage.created_at)),
)
</script>

<template>
  <div class="bg-white rounded-xl shadow-md hover:shadow-xl transition-all duration-300 overflow-hidden border border-gray-100 hover:border-gray-200">
    <!-- Bande colorée -->
    <div class="h-1.5 bg-linear-to-r from-custom-chocolat to-amber-700"></div>

    <div class="p-6">
      <!-- En-tête auteur du partage -->
      <div class="flex items-center gap-3 mb-4">
        <div class="shrink-0">
          <img
            v-if="photoAuteur"
            :src="photoAuteur"
            :alt="nomAuteur"
            class="w-11 h-11 rounded-full object-cover ring-2 ring-custom-chocolat/20"
          >
          <div
            v-else
            class="w-11 h-11 rounded-full bg-linear-to-br from-custom-chocolat to-amber-700 flex items-center justify-center text-white font-bold text-sm"
          >
            {{ initialesAuteur }}
          </div>
        </div>
        <div class="min-w-0">
          <p class="text-sm text-gray-900">
            <span class="font-bold">{{ nomAuteur }}</span>
            <span class="text-gray-500"> a partagé un profil</span>
          </p>
          <p class="flex items-center gap-1.5 text-xs text-gray-400 mt-0.5">
            <font-awesome-icon :icon="['fas', 'calendar-days']" />
            {{ dateFormatee }}
          </p>
        </div>
        <span class="ml-auto shrink-0 flex items-center gap-1.5 px-2.5 py-0.5 bg-amber-100 text-custom-chocolat rounded-full text-xs font-bold uppercase tracking-wide">
          <font-awesome-icon :icon="['fas', 'user']" class="text-[10px]" />
          Profil
        </span>
      </div>

      <!-- Légende -->
      <p
        v-if="partage.legende"
        class="text-gray-700 text-sm leading-relaxed mb-4 italic border-l-4 border-custom-chocolat/40 pl-3"
      >
        « {{ partage.legende }} »
      </p>

      <!-- Aperçu cliquable du profil -->
      <NuxtLink
        :to="`/profil/${partage.profil.id}`"
        class="group block rounded-xl overflow-hidden border border-gray-100 hover:border-custom-chocolat/40 transition-all duration-300"
      >
        <div class="flex gap-4 p-4 bg-gray-50 group-hover:bg-gray-100 transition">
          <img
            v-if="photoProfil"
            :src="photoProfil"
            :alt="`${partage.profil.prenom} ${partage.profil.nom}`"
            class="w-16 h-16 rounded-lg object-cover shrink-0"
          >
          <div
            v-else
            class="w-16 h-16 rounded-lg bg-linear-to-br from-custom-chocolat to-amber-700 flex items-center justify-center text-white font-bold text-lg shrink-0"
          >
            {{ initialesProfil }}
          </div>
          <div class="flex-1 min-w-0">
            <h3 class="font-bold text-gray-900 group-hover:text-custom-chocolat transition-colors truncate">
              {{ partage.profil.prenom }} {{ partage.profil.nom }}
            </h3>
            <p v-if="partage.profil.fonction" class="text-sm text-gray-600 truncate">{{ partage.profil.fonction }}</p>
            <div class="flex flex-wrap items-center gap-2 mt-1.5">
              <span
                v-if="partage.profil.est_expert"
                class="inline-flex items-center gap-1 px-2 py-0.5 bg-emerald-100 text-emerald-700 text-xs font-semibold rounded-full"
              >
                <font-awesome-icon :icon="['fas', 'briefcase']" class="text-[10px]" /> Expert
              </span>
              <span
                v-if="partage.profil.est_biblio"
                class="inline-flex items-center gap-1 px-2 py-0.5 bg-custom-chocolat/10 text-custom-chocolat text-xs font-semibold rounded-full"
              >
                <font-awesome-icon :icon="['fas', 'book-open']" class="text-[10px]" /> Bibliothèque
              </span>
              <span v-if="localisation" class="text-xs text-gray-500 flex items-center gap-1">
                <font-awesome-icon :icon="['fas', 'location-dot']" class="text-custom-chocolat" />
                {{ localisation }}
              </span>
            </div>
          </div>
          <div class="flex items-center text-custom-chocolat shrink-0">
            <font-awesome-icon :icon="['fas', 'arrow-up-right-from-square']" class="text-xs" />
          </div>
        </div>
      </NuxtLink>
    </div>
  </div>
</template>
