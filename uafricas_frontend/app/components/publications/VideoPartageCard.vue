<script setup lang="ts">
import type { PartageVideoAPI } from '~/composables/useVidafrica'
import { formaterDuree } from '~/mocks/vidafrica'

const props = defineProps<{
  partage: PartageVideoAPI
}>()

const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string

const resoudreUrl = (url: string | null | undefined): string | null => {
  if (!url) return null
  if (url.startsWith('http')) return url
  return `${apiBase}${url}`
}

const photoAuteur = computed(() => resoudreUrl(props.partage.auteur.photo_url))
const vignette = computed(() => resoudreUrl(props.partage.video.vignette_url))

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
    <div class="h-1.5 bg-linear-to-r from-purple-600 to-fuchsia-600"></div>

    <div class="p-6">
      <!-- En-tête auteur -->
      <div class="flex items-center gap-3 mb-4">
        <div class="shrink-0">
          <img
            v-if="photoAuteur"
            :src="photoAuteur"
            :alt="nomAuteur"
            class="w-11 h-11 rounded-full object-cover ring-2 ring-purple-500/20"
          >
          <div
            v-else
            class="w-11 h-11 rounded-full bg-linear-to-br from-purple-600 to-fuchsia-600 flex items-center justify-center text-white font-bold text-sm"
          >
            {{ initiales }}
          </div>
        </div>
        <div class="min-w-0">
          <p class="text-sm text-gray-900">
            <span class="font-bold">{{ nomAuteur }}</span>
            <span class="text-gray-500"> a partagé une vidéo</span>
          </p>
          <p class="flex items-center gap-1.5 text-xs text-gray-400 mt-0.5">
            <font-awesome-icon :icon="['fas', 'calendar-days']" />
            {{ dateFormatee }}
          </p>
        </div>
        <span class="ml-auto shrink-0 flex items-center gap-1.5 px-2.5 py-0.5 bg-purple-100 text-purple-700 rounded-full text-xs font-bold uppercase tracking-wide">
          <font-awesome-icon :icon="['fas', 'share-nodes']" class="text-[10px]" />
          Vidéo
        </span>
      </div>

      <!-- Légende -->
      <p v-if="partage.legende" class="text-gray-700 text-sm leading-relaxed mb-4 italic border-l-4 border-purple-500/40 pl-3">
        « {{ partage.legende }} »
      </p>

      <!-- Aperçu cliquable de la vidéo -->
      <NuxtLink
        :to="`/vidafrica/${partage.video.slug}`"
        class="group block rounded-xl overflow-hidden border border-gray-100 hover:border-purple-500/40 transition-all duration-300"
      >
        <!-- Vignette -->
        <div class="relative aspect-video bg-gray-200 overflow-hidden">
          <img
            v-if="vignette"
            :src="vignette"
            :alt="partage.video.titre"
            class="w-full h-full object-cover transition-transform duration-500 group-hover:scale-105"
          >
          <div v-else class="w-full h-full flex items-center justify-center bg-linear-to-br from-purple-700 to-fuchsia-700">
            <font-awesome-icon :icon="['fas', 'video']" class="text-4xl text-white/70" />
          </div>

          <!-- Icône lecture -->
          <div class="absolute inset-0 flex items-center justify-center">
            <span class="w-14 h-14 rounded-full bg-black/50 backdrop-blur-sm flex items-center justify-center group-hover:bg-purple-600/80 transition-colors">
              <font-awesome-icon :icon="['fas', 'play']" class="text-white text-lg ml-0.5" />
            </span>
          </div>

          <!-- Durée -->
          <span v-if="partage.video.duree_secondes" class="absolute bottom-2 right-2 bg-black/75 text-white text-xs px-2 py-1 rounded">
            {{ formaterDuree(partage.video.duree_secondes) }}
          </span>
        </div>

        <!-- Pied : titre -->
        <div class="flex items-center justify-between gap-2 px-4 py-3 bg-gray-50">
          <h3 class="text-sm font-semibold text-gray-800 line-clamp-1 group-hover:text-purple-700 transition-colors">
            {{ partage.video.titre }}
          </h3>
          <span class="shrink-0 flex items-center gap-1.5 text-xs font-semibold text-purple-600 group-hover:gap-2.5 transition-all">
            Regarder
            <font-awesome-icon :icon="['fas', 'arrow-up-right-from-square']" class="text-[10px]" />
          </span>
        </div>
      </NuxtLink>
    </div>
  </div>
</template>
