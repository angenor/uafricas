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
      <div v-if="amis.length" class="grid grid-cols-2 gap-3">
        <div
          v-for="ami in amis"
          :key="ami.uid"
          class="flex flex-col items-center p-3 rounded-lg border border-gray-200 hover:border-custom-green hover:bg-green-50 transition-all cursor-pointer"
        >
          <img
            :src="ami.photoURL || 'https://www.pngall.com/wp-content/uploads/5/Profile-PNG-Clipart.png'"
            class="w-12 h-12 rounded-full object-cover mb-2"
          />
          <span class="text-sm text-center font-medium text-gray-700">
            {{ ami.prenom }}
          </span>
        </div>
      </div>
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
            <span>{{ post.userInfo.prenom }}</span>
            <span class="flex items-center">
              <font-awesome-icon icon="fa-solid fa-heart" class="text-red-500 mr-1" />
              {{ post.stats.likes }}
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
import type { CodiMoiPost, UserInfo } from '~/mocks/codi-moi'

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
  amis: UserInfo[]
  stats: Stats
  popularPosts: CodiMoiPost[]
}>()

const emit = defineEmits<{
  goToPost: [post: CodiMoiPost]
}>()
</script>
