<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Breadcrumb -->
    <div class="backdrop-blur-sm py-3 px-4 md:px-8 bg-white shadow-sm">
      <div class="max-w-4xl mx-auto">
        <CommonBreadcrumbNav :custom-breadcrumbs="breadcrumbs" />
      </div>
    </div>

    <!-- Contenu -->
    <div v-if="post" class="max-w-4xl mx-auto py-8 px-4">
      <!-- Post -->
      <div class="bg-white rounded-xl shadow-lg overflow-hidden mb-6">
        <!-- En-tête -->
        <div class="p-6 border-b border-gray-100">
          <div class="flex items-start justify-between">
            <div class="flex items-center space-x-4">
              <img
                :src="post.userInfo.photoURL || 'https://www.pngall.com/wp-content/uploads/5/Profile-PNG-Clipart.png'"
                class="w-14 h-14 rounded-full object-cover"
              />
              <div>
                <h2 class="font-bold text-xl text-gray-800">
                  {{ post.userInfo.prenom }} {{ post.userInfo.nom }}
                </h2>
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
              class="px-4 py-2 rounded-full text-sm font-medium"
              :class="getCategoryStyle(post.categorie)"
            >
              {{ getCategoryLabel(post.categorie) }}
            </span>
          </div>
        </div>

        <!-- Contenu -->
        <div class="p-6">
          <!-- Style proverbe/citation -->
          <div
            v-if="post.categorie === 'proverbe_adage' || post.categorie === 'citation'"
            class="relative p-8 rounded-xl text-white text-center mb-6"
            :style="{ backgroundColor: post.couleurFond || '#2D5A27' }"
          >
            <p class="text-2xl font-medium leading-relaxed mb-4">
              "{{ post.contenu }}"
            </p>
            <p v-if="post.nomAuteur" class="text-lg opacity-90">
              - {{ post.nomAuteur }}
            </p>
          </div>

          <!-- Style bonne pratique/histoire -->
          <div v-else class="mb-6">
            <img
              v-if="post.imageCouverture"
              :src="post.imageCouverture"
              class="w-full h-80 object-cover rounded-xl mb-4"
            />
            <p class="text-lg text-gray-800 leading-relaxed">{{ post.contenu }}</p>
          </div>

          <!-- Explication -->
          <div
            v-if="post.explication"
            class="relative p-6 rounded-xl overflow-hidden"
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
              <h3
                class="font-bold text-lg mb-2"
                :class="post.imageArrierePlan ? 'text-white' : 'text-gray-800'"
              >
                Explication
              </h3>
              <p
                :class="post.imageArrierePlan ? 'text-gray-100' : 'text-gray-700'"
                class="leading-relaxed"
              >
                {{ post.explication }}
              </p>
            </div>
          </div>

          <!-- Hashtags -->
          <div v-if="post.hashtags && post.hashtags.length" class="mt-6">
            <div class="flex flex-wrap gap-2">
              <span
                v-for="hashtag in post.hashtags"
                :key="hashtag"
                class="text-custom-green hover:text-green-600 text-sm cursor-pointer bg-green-50 px-3 py-1 rounded-full"
              >
                #{{ hashtag }}
              </span>
            </div>
          </div>
        </div>

        <!-- Actions -->
        <div class="px-6 py-4 border-t border-gray-100 bg-gray-50">
          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-6">
              <button
                @click="handleLike"
                class="flex items-center space-x-2 text-gray-600 hover:text-red-500 transition-colors"
                :class="{ 'text-red-500': post.userReaction === 'like' }"
              >
                <font-awesome-icon icon="fa-solid fa-heart" class="text-xl" />
                <span>{{ post.stats.likes }}</span>
              </button>
              <button
                @click="handleDislike"
                class="flex items-center space-x-2 text-gray-600 hover:text-gray-800 transition-colors"
                :class="{ 'text-gray-800': post.userReaction === 'dislike' }"
              >
                <font-awesome-icon icon="fa-solid fa-thumbs-down" class="text-xl" />
                <span>{{ post.stats.dislikes }}</span>
              </button>
              <button class="flex items-center space-x-2 text-gray-600">
                <font-awesome-icon icon="fa-solid fa-comment" class="text-xl" />
                <span>{{ comments.length }}</span>
              </button>
              <button
                @click="handleShare"
                class="flex items-center space-x-2 text-gray-600 hover:text-green-500 transition-colors"
              >
                <font-awesome-icon icon="fa-solid fa-share" class="text-xl" />
                <span>{{ post.stats.partages }}</span>
              </button>
            </div>
            <div class="text-sm text-gray-500">
              <font-awesome-icon icon="fa-solid fa-eye" class="mr-1" />
              {{ post.stats.vues }} vues
            </div>
          </div>
        </div>
      </div>

      <!-- Commentaires -->
      <div class="bg-white rounded-xl shadow-lg p-6">
        <h3 class="font-bold text-xl text-gray-800 mb-6">
          Commentaires ({{ comments.length }})
        </h3>

        <!-- Formulaire -->
        <div class="flex items-start space-x-4 mb-6 pb-6 border-b border-gray-100">
          <img
            class="w-12 h-12 rounded-full object-cover"
            src="https://www.pngall.com/wp-content/uploads/5/Profile-PNG-Clipart.png"
          />
          <div class="flex-1">
            <textarea
              v-model="newComment"
              placeholder="Partagez votre réflexion..."
              class="w-full p-4 border border-gray-200 rounded-xl resize-none focus:outline-none focus:ring-2 focus:ring-custom-green"
              rows="3"
            ></textarea>
            <div class="flex justify-end mt-3">
              <button
                @click="addComment"
                :disabled="!newComment.trim()"
                class="px-6 py-2 bg-custom-green text-white rounded-lg font-medium hover:bg-green-600 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
              >
                <font-awesome-icon icon="fa-solid fa-paper-plane" class="mr-2" />
                Publier
              </button>
            </div>
          </div>
        </div>

        <!-- Liste des commentaires -->
        <div class="space-y-4">
          <div
            v-for="comment in comments"
            :key="comment.id"
            class="flex items-start space-x-4 p-4 bg-gray-50 rounded-xl"
          >
            <img
              class="w-10 h-10 rounded-full object-cover"
              :src="comment.userInfo.photoURL || 'https://www.pngall.com/wp-content/uploads/5/Profile-PNG-Clipart.png'"
            />
            <div class="flex-1">
              <div class="flex items-center space-x-2 mb-2">
                <span class="font-semibold text-gray-800">
                  {{ comment.userInfo.prenom }} {{ comment.userInfo.nom }}
                </span>
                <span class="text-xs text-gray-500">
                  {{ formatCommentDate(comment.dateCreation) }}
                </span>
              </div>
              <p class="text-gray-700">{{ comment.contenu }}</p>
              <div class="flex items-center space-x-4 mt-2">
                <button class="text-xs text-gray-500 hover:text-custom-green flex items-center">
                  <font-awesome-icon icon="fa-regular fa-heart" class="mr-1" />
                  {{ comment.likes }}
                </button>
                <button class="text-xs text-gray-500 hover:text-custom-green">
                  Répondre
                </button>
              </div>
            </div>
          </div>

          <div v-if="comments.length === 0" class="text-center py-8">
            <font-awesome-icon icon="fa-regular fa-comment-dots" class="text-4xl text-gray-300 mb-3" />
            <p class="text-gray-500">Aucun commentaire pour le moment.</p>
            <p class="text-gray-400 text-sm mt-1">Soyez le premier à commenter !</p>
          </div>
        </div>
      </div>
    </div>

    <!-- État non trouvé -->
    <div v-else class="text-center py-16">
      <div class="text-5xl text-gray-300 mb-4">
        <font-awesome-icon icon="fa-solid fa-file-circle-xmark" />
      </div>
      <h3 class="text-xl font-semibold text-gray-500">
        Post non trouvé
      </h3>
      <NuxtLink to="/evenements/codi-moi" class="mt-4 inline-block text-custom-green hover:underline">
        Retour à Codi-Moi
      </NuxtLink>
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  getPostById,
  getPostComments,
  formatDate,
  getCategoryStyle,
  getCategoryLabel,
  type CodiMoiPost,
  type CodiMoiComment
} from '~/mocks/codi-moi'

const route = useRoute()
const postId = route.params.id as string

const post = ref<CodiMoiPost | undefined>(undefined)
const comments = ref<CodiMoiComment[]>([])
const newComment = ref('')

const breadcrumbs = computed(() => [
  { label: 'Centre Culturel', to: '/africa-culture' },
  { label: 'Événements', to: '/evenements' },
  { label: 'Codi-Moi', to: '/evenements/codi-moi' },
  { label: 'Détail', to: null }
])

useHead({
  title: computed(() => post.value ? `${post.value.contenu.substring(0, 50)}... | UAfricas` : 'Post | UAfricas')
})

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

const handleLike = () => {
  if (post.value) {
    if (post.value.userReaction === 'like') {
      post.value.userReaction = null
      post.value.stats.likes--
    } else {
      if (post.value.userReaction === 'dislike') {
        post.value.stats.dislikes--
      }
      post.value.userReaction = 'like'
      post.value.stats.likes++
    }
  }
}

const handleDislike = () => {
  if (post.value) {
    if (post.value.userReaction === 'dislike') {
      post.value.userReaction = null
      post.value.stats.dislikes--
    } else {
      if (post.value.userReaction === 'like') {
        post.value.stats.likes--
      }
      post.value.userReaction = 'dislike'
      post.value.stats.dislikes++
    }
  }
}

const handleShare = () => {
  if (post.value) {
    post.value.stats.partages++
  }
  alert('Lien copié ! (Mode démo)')
}

const addComment = () => {
  if (newComment.value.trim() && post.value) {
    const comment: CodiMoiComment = {
      id: `cmt-${Date.now()}`,
      postId: post.value.id,
      contenu: newComment.value.trim(),
      userId: 'user-current',
      userInfo: {
        uid: 'user-current',
        email: 'test@example.com',
        nom: 'Test',
        prenom: 'Utilisateur',
        photoURL: null
      },
      dateCreation: new Date(),
      likes: 0
    }
    comments.value.unshift(comment)
    newComment.value = ''
  }
}

onMounted(() => {
  post.value = getPostById(postId)
  if (post.value) {
    comments.value = getPostComments(postId)
    // Incrémenter les vues
    post.value.stats.vues++
  }
})
</script>
