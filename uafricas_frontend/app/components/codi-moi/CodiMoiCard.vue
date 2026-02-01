<template>
  <div
    class="bg-white rounded-xl shadow-md border border-gray-100 overflow-hidden hover:shadow-lg transition-shadow duration-300 cursor-pointer"
    @click="emit('click')"
  >
    <!-- En-tête du post -->
    <div class="p-6 pb-4">
      <div class="flex items-start justify-between">
        <div class="flex items-center space-x-3">
          <img
            :src="post.userInfo.photoURL || 'https://www.pngall.com/wp-content/uploads/5/Profile-PNG-Clipart.png'"
            class="w-12 h-12 rounded-full object-cover"
          />
          <div>
            <h3 class="font-semibold text-gray-800">
              {{ post.userInfo.prenom }} {{ post.userInfo.nom }}
            </h3>
            <div class="flex items-center space-x-2 text-sm text-gray-500">
              <span>{{ formatDate(post.dateCreation) }}</span>
              <span>•</span>
              <span class="flex items-center">
                <font-awesome-icon icon="fa-solid fa-location-dot" class="mr-1" />
                {{ post.pays }}
              </span>
              <template v-if="post.groupeEthnique">
                <span>•</span>
                <span>{{ post.groupeEthnique }}</span>
              </template>
            </div>
          </div>
        </div>
        <span
          class="px-3 py-1 rounded-full text-xs font-medium"
          :class="getCategoryStyle(post.categorie)"
        >
          {{ getCategoryLabel(post.categorie) }}
        </span>
      </div>
    </div>

    <!-- Contenu du post -->
    <div class="px-6 pb-4">
      <!-- Style proverbe/citation -->
      <div
        v-if="post.categorie === 'proverbe_adage' || post.categorie === 'citation'"
        class="relative p-6 rounded-t-lg text-white text-center"
        :style="{ backgroundColor: post.couleurFond || '#2D5A27' }"
      >
        <div class="relative z-10">
          <p class="text-lg font-medium leading-relaxed mb-3">
            "{{ post.contenu }}"
          </p>
          <p v-if="post.nomAuteur" class="text-sm opacity-90">
            - {{ post.nomAuteur }}
          </p>
        </div>
      </div>

      <!-- Style bonne pratique/histoire -->
      <div v-else>
        <img
          v-if="post.imageCouverture"
          :src="post.imageCouverture"
          class="w-full h-64 object-cover rounded-t-lg mb-4"
        />
        <p class="text-gray-800 leading-relaxed">{{ post.contenu }}</p>
      </div>

      <!-- Explication -->
      <div
        v-if="post.explication"
        class="relative p-4 rounded-b-lg overflow-hidden"
        :class="post.imageArrierePlan ? '' : 'bg-gray-50'"
      >
        <div
          v-if="post.imageArrierePlan"
          class="absolute inset-0 z-0"
          :style="{
            backgroundImage: `url(${post.imageArrierePlan})`,
            backgroundSize: 'cover',
            backgroundPosition: 'center'
          }"
        >
          <div class="absolute inset-0 bg-black bg-opacity-60"></div>
        </div>

        <div class="relative z-10" :class="post.imageArrierePlan ? 'text-white' : ''">
          <h4
            class="font-semibold bg-white bg-opacity-40 px-2 rounded-t-md inline-flex underline"
            :class="post.imageArrierePlan ? 'text-white' : 'text-gray-800'"
          >
            Explication :
          </h4>
          <p
            :class="post.imageArrierePlan ? 'text-gray-100' : 'text-gray-700'"
            class="bg-white bg-opacity-40 px-2 rounded-b-md rounded-tr-md"
          >
            {{ post.explication }}
          </p>
        </div>
      </div>

      <!-- Hashtags -->
      <div v-if="post.hashtags && post.hashtags.length" class="mt-4">
        <div class="flex flex-wrap gap-2">
          <span
            v-for="hashtag in post.hashtags"
            :key="hashtag"
            class="text-custom-green hover:text-green-600 text-sm cursor-pointer"
          >
            #{{ hashtag }}
          </span>
        </div>
      </div>
    </div>

    <!-- Actions -->
    <div class="px-6 py-4 border-t border-gray-100">
      <div class="flex items-center justify-between">
        <div class="flex items-center space-x-6">
          <button
            @click.stop="emit('like')"
            class="flex items-center space-x-2 text-gray-600 hover:text-red-500 transition-colors"
            :class="{ 'text-red-500': post.userReaction === 'like' }"
          >
            <font-awesome-icon icon="fa-solid fa-heart" />
            <span class="text-sm">{{ post.stats.likes }}</span>
          </button>
          <button
            @click.stop="emit('dislike')"
            class="flex items-center space-x-2 text-gray-600 hover:text-gray-800 transition-colors"
            :class="{ 'text-gray-800': post.userReaction === 'dislike' }"
          >
            <font-awesome-icon icon="fa-solid fa-thumbs-down" />
            <span class="text-sm">{{ post.stats.dislikes }}</span>
          </button>
          <button
            @click.stop="emit('comment')"
            class="flex items-center space-x-2 text-gray-600 hover:text-blue-500 transition-colors"
          >
            <font-awesome-icon icon="fa-solid fa-comment" />
            <span class="text-sm">{{ post.stats.commentaires }}</span>
          </button>
          <button
            @click.stop="emit('share')"
            class="flex items-center space-x-2 text-gray-600 hover:text-green-500 transition-colors"
          >
            <font-awesome-icon icon="fa-solid fa-share" />
            <span class="text-sm">{{ post.stats.partages }}</span>
          </button>
        </div>
        <div class="text-sm text-gray-500">
          <font-awesome-icon icon="fa-solid fa-eye" class="mr-1" />
          {{ post.stats.vues }} vues
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { formatDate, getCategoryStyle, getCategoryLabel, type CodiMoiPost } from '~/mocks/codi-moi'

defineProps<{
  post: CodiMoiPost
}>()

const emit = defineEmits<{
  click: []
  like: []
  dislike: []
  comment: []
  share: []
}>()
</script>
