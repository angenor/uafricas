<template>
  <div class="w-full lg:w-80">
    <!-- Mes amis -->
    <div class="bg-white rounded-xl shadow-md border border-gray-100 p-6 mb-6">
      <div class="flex items-center mb-4">
        <div class="bg-orange-100 p-2 rounded-lg mr-3">
          <font-awesome-icon icon="fa-solid fa-users" class="text-orange-600 text-lg" />
        </div>
        <h3 class="font-bold text-lg text-gray-800">Mes ami(e)s</h3>
      </div>
      <ul v-if="amis.length" class="divide-y divide-gray-100">
        <li
          v-for="ami in amis"
          :key="ami.id"
          class="flex items-center gap-3 py-2.5 first:pt-0 last:pb-0"
        >
          <NuxtLink :to="`/profil/${ami.id}`" class="flex items-center gap-3 flex-1 min-w-0 group">
            <img
              v-if="photoComplete(ami.photoUrl)"
              :src="photoComplete(ami.photoUrl)!"
              :alt="`${ami.prenom} ${ami.nom}`"
              class="w-9 h-9 rounded-full object-cover border border-gray-200 shrink-0"
            />
            <div
              v-else
              class="w-9 h-9 rounded-full bg-gradient-to-br from-custom-green to-emerald-600 flex items-center justify-center text-white font-bold text-sm shrink-0"
            >
              {{ (ami.prenom || ami.nom).charAt(0) }}
            </div>
            <span class="text-sm font-medium text-gray-700 group-hover:text-custom-green transition-colors truncate">
              {{ ami.prenom }} {{ ami.nom }}
            </span>
          </NuxtLink>
          <button
            type="button"
            class="shrink-0 w-8 h-8 flex items-center justify-center text-custom-chocolat bg-custom-chocolat/10 rounded-full hover:bg-custom-chocolat hover:text-white transition-colors"
            :aria-label="`Envoyer un message à ${ami.prenom} ${ami.nom}`"
            :title="`Envoyer un message à ${ami.prenom}`"
            @click="emit('envoyerMessage', ami)"
          >
            <font-awesome-icon icon="fa-solid fa-paper-plane" class="text-xs" />
          </button>
        </li>
      </ul>
      <div v-else class="text-center py-4 text-gray-500 text-sm">
        Aucun ami pour le moment
      </div>
    </div>

    <!-- Statistiques -->
    <div class="bg-white rounded-xl shadow-md border border-gray-100 p-6 mb-6">
      <h3 class="font-bold text-lg text-gray-800 mb-4">Statistiques Codi-Moi</h3>
      <div class="space-y-3">
        <div class="flex justify-between">
          <span class="text-gray-600">Publications totales</span>
          <span class="font-semibold text-custom-green">{{ stats.totalPosts }}</span>
        </div>
        <div class="flex justify-between">
          <span class="text-gray-600">Proverbes & Adages</span>
          <span class="font-semibold">{{ stats.proverbesAdages }}</span>
        </div>
        <div class="flex justify-between">
          <span class="text-gray-600">Citations</span>
          <span class="font-semibold">{{ stats.citations }}</span>
        </div>
        <div class="flex justify-between">
          <span class="text-gray-600">Ressources historiques</span>
          <span class="font-semibold">{{ stats.ressourcesHistoriques }}</span>
        </div>
        <div class="flex justify-between">
          <span class="text-gray-600">Bonnes pratiques</span>
          <span class="font-semibold">{{ stats.bonnesPratiques }}</span>
        </div>
        <div class="border-t pt-3 mt-3">
          <div class="flex justify-between">
            <span class="text-gray-600">Total likes</span>
            <span class="font-semibold text-red-500">{{ stats.totalLikes }}</span>
          </div>
          <div class="flex justify-between mt-2">
            <span class="text-gray-600">Total vues</span>
            <span class="font-semibold text-blue-500">{{ stats.totalVues }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Posts populaires -->
    <div class="bg-white rounded-xl shadow-md border border-gray-100 p-6">
      <h3 class="font-bold text-lg text-gray-800 mb-4">Posts populaires</h3>
      <div v-if="popularPosts.length" class="space-y-3">
        <div
          v-for="post in popularPosts"
          :key="post.id"
          class="p-3 border border-gray-200 rounded-lg hover:bg-gray-50 cursor-pointer transition-colors"
          @click="emit('goToPost', post)"
        >
          <p class="text-sm text-gray-800 line-clamp-2 mb-2">
            {{ post.contenu }}
          </p>
          <div class="flex items-center justify-between text-xs text-gray-500">
            <span>{{ post.auteur.prenom || post.auteur.nom }}</span>
            <span class="flex items-center">
              <font-awesome-icon icon="fa-solid fa-heart" class="text-red-500 mr-1" />
              {{ post.nombre_likes }}
            </span>
          </div>
        </div>
      </div>
      <div v-else class="text-center py-4 text-gray-500 text-sm">
        Aucun post populaire
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { CodiMoiPostAPI } from '~/composables/useCodiMoi'
import type { MembreLightAPI } from '~/composables/useAmis'

interface Stats {
  totalPosts: number
  proverbesAdages: number
  citations: number
  ressourcesHistoriques: number
  bonnesPratiques: number
  totalLikes: number
  totalVues: number
}

defineProps<{
  amis: MembreLightAPI[]
  stats: Stats
  popularPosts: CodiMoiPostAPI[]
}>()

const emit = defineEmits<{
  goToPost: [post: CodiMoiPostAPI]
  envoyerMessage: [ami: MembreLightAPI]
}>()

const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string

const photoComplete = (url: string | null): string | null => {
  if (!url) return null
  return url.startsWith('http') ? url : `${apiBase}${url}`
}
</script>
