<template>
  <div class="mt-4 border-t border-gray-100">
    <!-- En-tête des commentaires -->
    <div
      class="px-4 py-3 bg-gray-50 flex items-center justify-between cursor-pointer hover:bg-gray-100 transition-colors"
      @click="showComments = !showComments"
    >
      <div class="flex items-center space-x-2">
        <font-awesome-icon icon="fa-regular fa-comment" class="text-custom-green text-lg" />
        <span class="font-medium text-gray-700">
          {{ comments.length }} commentaire{{ comments.length > 1 ? 's' : '' }}
        </span>
      </div>
      <div class="flex items-center space-x-2 text-custom-green">
        <span class="text-sm font-medium">{{ showComments ? 'Masquer' : 'Afficher' }}</span>
        <font-awesome-icon
          :icon="showComments ? 'fa-solid fa-chevron-up' : 'fa-solid fa-chevron-down'"
          class="text-sm transition-transform"
        />
      </div>
    </div>

    <!-- Contenu des commentaires -->
    <div v-if="showComments" class="p-4 space-y-4 bg-white">
      <!-- Formulaire d'ajout -->
      <div class="border border-gray-200 rounded-lg p-4 bg-gray-50">
        <div class="flex items-start space-x-3">
          <img
            class="h-10 w-10 rounded-full border-2 border-custom-green object-cover shadow-sm"
            src="https://www.pngall.com/wp-content/uploads/5/Profile-PNG-Clipart.png"
          />
          <div class="flex-1">
            <textarea
              v-model="newComment"
              placeholder="Partagez votre réflexion..."
              class="w-full p-3 border border-gray-200 rounded-lg resize-none focus:outline-none focus:ring-2 focus:ring-custom-green focus:border-transparent bg-white shadow-sm"
              rows="3"
            ></textarea>
            <div class="flex justify-between items-center mt-3">
              <span class="text-xs text-gray-500">Soyez respectueux dans vos échanges</span>
              <button
                @click="handleAddComment"
                :disabled="!newComment.trim()"
                class="px-6 py-2 bg-custom-green text-white rounded-lg text-sm font-medium hover:bg-opacity-90 disabled:opacity-50 disabled:cursor-not-allowed transition-all transform hover:scale-105 active:scale-95 shadow-sm"
              >
                <font-awesome-icon icon="fa-solid fa-paper-plane" class="mr-2" />
                Publier
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Liste des commentaires -->
      <div class="space-y-4">
        <div
          v-for="comment in comments"
          :key="comment.id"
          class="flex items-start space-x-3 p-4 bg-white border border-gray-100 rounded-lg hover:shadow-md transition-shadow"
        >
          <img
            class="h-10 w-10 rounded-full border-2 border-gray-200 object-cover"
            :src="comment.user.photo_url || 'https://www.pngall.com/wp-content/uploads/5/Profile-PNG-Clipart.png'"
          />
          <div class="flex-1 min-w-0">
            <div class="flex items-center space-x-2 mb-2">
              <span class="font-semibold text-gray-800">
                {{ comment.user.prenom }} {{ comment.user.nom }}
              </span>
              <span class="text-xs text-gray-500 bg-gray-100 px-2 py-1 rounded-full">
                {{ formatCommentDate(comment.created_at) }}
              </span>
            </div>
            <p class="text-gray-700 leading-relaxed">{{ comment.content }}</p>
            <div class="flex items-center space-x-4 mt-3">
              <button class="text-xs text-gray-500 hover:text-custom-green transition-colors flex items-center space-x-1">
                <font-awesome-icon icon="fa-regular fa-heart" />
                <span>J'aime ({{ comment.likes }})</span>
              </button>
              <button class="text-xs text-gray-500 hover:text-custom-green transition-colors">
                Répondre
              </button>
            </div>
          </div>
        </div>

        <div v-if="comments.length === 0" class="text-center py-8">
          <font-awesome-icon icon="fa-regular fa-comment-dots" class="text-4xl text-gray-300 mb-3" />
          <p class="text-gray-500 text-sm">Aucun commentaire pour le moment.</p>
          <p class="text-gray-400 text-xs mt-1">Soyez le premier à partager votre opinion !</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ForumComment } from '~/mocks/forums'

defineProps<{
  forumId: string
  comments: ForumComment[]
}>()

const emit = defineEmits<{
  addComment: [data: { forumId: string; content: string }]
}>()

const showComments = ref(false)
const newComment = ref('')

const formatCommentDate = (date: Date): string => {
  const now = new Date()
  const diffInMinutes = Math.floor((now.getTime() - date.getTime()) / (1000 * 60))

  if (diffInMinutes < 1) return 'À l\'instant'
  if (diffInMinutes < 60) return `Il y a ${diffInMinutes} min`
  if (diffInMinutes < 1440) return `Il y a ${Math.floor(diffInMinutes / 60)} h`

  return date.toLocaleDateString('fr-FR', {
    day: 'numeric',
    month: 'short'
  })
}

const handleAddComment = () => {
  if (newComment.value.trim()) {
    emit('addComment', {
      forumId: props.forumId,
      content: newComment.value.trim()
    })
    newComment.value = ''
  }
}

const props = defineProps<{
  forumId: string
  comments: ForumComment[]
}>()
</script>
