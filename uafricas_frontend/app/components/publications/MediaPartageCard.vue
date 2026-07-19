<script setup lang="ts">
// Carte du mur /publications pour un média radio ou télé partagé (FR-025).
// Modèle : publications/VideoPartageCard.vue.

import type { PartageMediaAPI, TypeMedia } from '~/composables/useMediaSocial'

const props = defineProps<{
  partage: PartageMediaAPI
}>()

const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string

const resoudreUrl = (url: string | null | undefined): string | null => {
  if (!url) return null
  if (url.startsWith('http')) return url
  return `${apiBase}${url}`
}

// Chaque type a son vocabulaire et son icône : « a partagé une chaîne » ne se
// dit pas de la même façon que « a partagé une émission de radio ».
const HABILLAGE: Record<TypeMedia, { verbe: string; badge: string; icone: string[] }> = {
  chaine_tv: { verbe: 'a partagé une chaîne de télévision', badge: 'Chaîne', icone: ['fas', 'tv'] },
  station_radio: { verbe: 'a partagé une station de radio', badge: 'Station', icone: ['fas', 'radio'] },
  programme_tele: { verbe: 'a partagé une émission de télévision', badge: 'Émission TV', icone: ['fas', 'tv'] },
  programme_radio: { verbe: 'a partagé une émission de radio', badge: 'Émission radio', icone: ['fas', 'microphone'] },
}

const habillage = computed(() => HABILLAGE[props.partage.media.type_media])
const photoAuteur = computed(() => resoudreUrl(props.partage.auteur.photo_url))
const illustration = computed(() => resoudreUrl(props.partage.media.image_url))

// Le serveur calcule l'URL de détail ; sans slug, la carte reste non cliquable
// plutôt que de renvoyer vers une page inexistante.
const lien = computed(() => props.partage.media.url)

const nomAuteur = computed(() => {
  const { prenom, nom } = props.partage.auteur
  return `${prenom ?? ''} ${nom ?? ''}`.trim() || 'Anonyme'
})

const initiales = computed(() => {
  const { prenom, nom } = props.partage.auteur
  return ((prenom ?? '').charAt(0) + (nom ?? '').charAt(0)).toUpperCase() || '?'
})

const dateFormatee = computed(() =>
  new Intl.DateTimeFormat('fr-FR', { day: 'numeric', month: 'short', year: 'numeric' })
    .format(new Date(props.partage.created_at)),
)
</script>

<template>
  <div class="bg-white rounded-xl shadow-md hover:shadow-xl transition-all duration-300 overflow-hidden border border-gray-100 hover:border-gray-200">
    <!-- Bande colorée -->
    <div class="h-1.5 bg-linear-to-r from-custom-chocolat to-amber-600"></div>

    <div class="p-6">
      <!-- En-tête auteur -->
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
            class="w-11 h-11 rounded-full bg-linear-to-br from-custom-chocolat to-amber-600 flex items-center justify-center text-white font-bold text-sm"
          >
            {{ initiales }}
          </div>
        </div>
        <div class="min-w-0">
          <p class="text-sm text-gray-900">
            <span class="font-bold">{{ nomAuteur }}</span>
            <span class="text-gray-500"> {{ habillage.verbe }}</span>
          </p>
          <p class="flex items-center gap-1.5 text-xs text-gray-400 mt-0.5">
            <font-awesome-icon :icon="['fas', 'calendar-days']" />
            {{ dateFormatee }}
          </p>
        </div>
        <span class="ml-auto shrink-0 flex items-center gap-1.5 px-2.5 py-0.5 bg-amber-100 text-custom-chocolat rounded-full text-xs font-bold uppercase tracking-wide">
          <font-awesome-icon :icon="habillage.icone" class="text-[10px]" />
          {{ habillage.badge }}
        </span>
      </div>

      <!-- Légende -->
      <p v-if="partage.legende" class="text-gray-700 text-sm leading-relaxed mb-4 italic border-l-4 border-custom-chocolat/40 pl-3">
        « {{ partage.legende }} »
      </p>

      <!-- Aperçu du média -->
      <!-- `component :is` avec la chaîne « NuxtLink » ne résout PAS le composant
           à l'exécution : il doit l'être au rendu. D'où les deux branches
           explicites, comme dans `media/CarteContenu.vue`. -->
      <NuxtLink
        v-if="lien"
        :to="lien"
        class="group block rounded-xl overflow-hidden border border-gray-100 hover:border-custom-chocolat/40 transition-all duration-300"
      >
        <div class="relative aspect-video bg-gray-200 overflow-hidden">
          <img
            v-if="illustration"
            :src="illustration"
            :alt="partage.media.titre"
            class="w-full h-full object-cover transition-transform duration-500 group-hover:scale-105"
          >
          <div v-else class="w-full h-full flex items-center justify-center bg-linear-to-br from-custom-chocolat to-amber-700">
            <font-awesome-icon :icon="habillage.icone" class="text-4xl text-white/70" />
          </div>

          <div v-if="true" class="absolute inset-0 flex items-center justify-center">
            <span class="w-14 h-14 rounded-full bg-black/50 backdrop-blur-sm flex items-center justify-center group-hover:bg-custom-chocolat/80 transition-colors">
              <font-awesome-icon :icon="['fas', 'play']" class="text-white text-lg ml-0.5" />
            </span>
          </div>
        </div>

        <!-- Pied : titre -->
        <div class="flex items-center justify-between gap-2 px-4 py-3 bg-gray-50">
          <h3 class="text-sm font-semibold text-gray-800 line-clamp-1 group-hover:text-custom-chocolat transition-colors">
            {{ partage.media.titre }}
          </h3>
          <span v-if="true" class="shrink-0 flex items-center gap-1.5 text-xs font-semibold text-custom-chocolat group-hover:gap-2.5 transition-all">
            Découvrir
            <font-awesome-icon :icon="['fas', 'arrow-up-right-from-square']" class="text-[10px]" />
          </span>
        </div>
      </NuxtLink>

      <!-- Sans slug, aucune page de détail à viser : la carte reste inerte. -->
      <div
        v-else
        class="block rounded-xl overflow-hidden border border-gray-100"
      >
        <div class="relative aspect-video bg-gray-200 overflow-hidden">
          <img
            v-if="illustration"
            :src="illustration"
            :alt="partage.media.titre"
            class="w-full h-full object-cover transition-transform duration-500 group-hover:scale-105"
          >
          <div v-else class="w-full h-full flex items-center justify-center bg-linear-to-br from-custom-chocolat to-amber-700">
            <font-awesome-icon :icon="habillage.icone" class="text-4xl text-white/70" />
          </div>

          <div v-if="false" class="absolute inset-0 flex items-center justify-center">
            <span class="w-14 h-14 rounded-full bg-black/50 backdrop-blur-sm flex items-center justify-center group-hover:bg-custom-chocolat/80 transition-colors">
              <font-awesome-icon :icon="['fas', 'play']" class="text-white text-lg ml-0.5" />
            </span>
          </div>
        </div>

        <!-- Pied : titre -->
        <div class="flex items-center justify-between gap-2 px-4 py-3 bg-gray-50">
          <h3 class="text-sm font-semibold text-gray-800 line-clamp-1">
            {{ partage.media.titre }}
          </h3>
          <span v-if="false" class="shrink-0 flex items-center gap-1.5 text-xs font-semibold text-custom-chocolat group-hover:gap-2.5 transition-all">
            Découvrir
            <font-awesome-icon :icon="['fas', 'arrow-up-right-from-square']" class="text-[10px]" />
          </span>
        </div>
      </div>
    </div>
  </div>
</template>
