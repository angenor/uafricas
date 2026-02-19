<template>
  <Transition name="modal-fade">
    <div v-if="post" class="z-50 fixed inset-0 flex items-center justify-center">
      <div @click="emit('close')" class="absolute inset-0 bg-black/60 backdrop-blur-xs"></div>

      <div class="relative w-full max-w-3xl mx-4 md:mx-auto animate-slideIn">
        <div class="bg-white rounded-xl overflow-hidden shadow-2xl max-h-[90vh] overflow-y-auto">
          <!-- Bouton fermer -->
          <button
            @click="emit('close')"
            class="absolute top-3 right-3 z-20 w-9 h-9 flex items-center justify-center rounded-full bg-white/90 hover:bg-white shadow-md text-gray-600 hover:text-gray-800 transition-colors"
          >
            <font-awesome-icon icon="fa-solid fa-xmark" class="text-lg" />
          </button>

          <!-- En-tête -->
          <div class="p-6 border-b border-gray-100">
            <div class="flex items-start justify-between pr-10">
              <div class="flex items-center space-x-4">
                <div class="w-14 h-14 rounded-full bg-gradient-to-br from-custom-chocolat to-amber-600 flex items-center justify-center text-white font-bold text-xl">
                  {{ (post.auteur.prenom || post.auteur.nom).charAt(0) }}
                </div>
                <div>
                  <h2 class="font-bold text-xl text-gray-800">
                    {{ post.auteur.prenom }} {{ post.auteur.nom }}
                  </h2>
                  <div class="flex items-center flex-wrap gap-x-2 text-sm text-gray-500">
                    <span>{{ formatDateRelative(post.created_at) }}</span>
                    <span>&#8226;</span>
                    <span class="flex items-center">
                      <font-awesome-icon icon="fa-solid fa-location-dot" class="mr-1" />
                      {{ post.pays || 'Non spécifié' }}
                    </span>
                    <template v-if="post.groupe_ethnique">
                      <span>&#8226;</span>
                      <span>{{ post.groupe_ethnique }}</span>
                    </template>
                  </div>
                </div>
              </div>
              <span
                class="px-4 py-2 rounded-full text-sm font-medium shrink-0"
                :class="getCategoryStyle(post.type as CategoriePost)"
              >
                {{ getCategoryLabel(post.type as CategoriePost) }}
              </span>
            </div>
          </div>

          <!-- Contenu -->
          <div class="p-6">
            <!-- Style proverbe/citation -->
            <div
              v-if="post.type === 'proverbe_adage' || post.type === 'citation'"
              class="relative p-8 rounded-xl text-white text-center mb-6"
              :style="{ backgroundColor: post.couleur_fond || '#2D5A27' }"
            >
              <p class="text-2xl font-medium leading-relaxed mb-4">
                "{{ post.contenu }}"
              </p>
              <p v-if="post.nom_auteur_originel" class="text-lg opacity-90">
                - {{ post.nom_auteur_originel }}
              </p>
            </div>

            <!-- Style bonne pratique/histoire -->
            <div v-else class="mb-6">
              <img
                v-if="post.image_couverture_url"
                :src="post.image_couverture_url"
                class="w-full h-80 object-cover rounded-xl mb-4"
              />
              <p class="text-lg text-gray-800 leading-relaxed">{{ post.contenu }}</p>
            </div>

            <!-- Explication -->
            <div
              v-if="post.explication"
              class="relative p-6 rounded-xl overflow-hidden"
              :class="post.image_arriere_plan_url ? '' : 'bg-gray-50'"
            >
              <div
                v-if="post.image_arriere_plan_url"
                class="absolute inset-0 z-0"
                :style="{
                  backgroundImage: `url(${post.image_arriere_plan_url})`,
                  backgroundSize: 'cover',
                  backgroundPosition: 'center'
                }"
              >
                <div class="absolute inset-0 bg-black/60"></div>
              </div>

              <div class="relative z-10" :class="post.image_arriere_plan_url ? 'text-white' : ''">
                <h3
                  class="font-bold text-lg mb-2"
                  :class="post.image_arriere_plan_url ? 'text-white' : 'text-gray-800'"
                >
                  Explication
                </h3>
                <p
                  :class="post.image_arriere_plan_url ? 'text-gray-100' : 'text-gray-700'"
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
                  @click="emit('like')"
                  class="flex items-center space-x-2 transition-colors"
                  :class="post.user_reaction === 'like' ? 'text-red-500' : 'text-gray-600 hover:text-red-500'"
                >
                  <font-awesome-icon :icon="post.user_reaction === 'like' ? 'fa-solid fa-heart' : 'fa-regular fa-heart'" class="text-xl" />
                  <span>{{ post.nombre_likes }}</span>
                </button>
                <button
                  @click="emit('dislike')"
                  class="flex items-center space-x-2 transition-colors"
                  :class="post.user_reaction === 'dislike' ? 'text-orange-500' : 'text-gray-600 hover:text-gray-800'"
                >
                  <font-awesome-icon icon="fa-solid fa-thumbs-down" class="text-xl" />
                  <span>{{ post.nombre_dislikes }}</span>
                </button>
                <span class="flex items-center space-x-2 text-gray-600">
                  <font-awesome-icon icon="fa-solid fa-comment" class="text-xl" />
                  <span>{{ commentaires.length }}</span>
                </span>
                <button
                  @click="emit('share')"
                  class="flex items-center space-x-2 text-gray-600 hover:text-green-500 transition-colors"
                >
                  <font-awesome-icon icon="fa-solid fa-share" class="text-xl" />
                </button>
              </div>
              <div class="text-sm text-gray-500">
                <font-awesome-icon icon="fa-solid fa-eye" class="mr-1" />
                {{ post.nombre_vues }} vues
              </div>
            </div>
          </div>

          <!-- Section Commentaires -->
          <div class="px-6 py-6 border-t border-gray-100">
            <h3 class="font-bold text-xl text-gray-800 mb-6">
              Commentaires ({{ commentaires.length }})
            </h3>

            <!-- Formulaire commentaire -->
            <div class="flex items-start space-x-4 mb-6 pb-6 border-b border-gray-100">
              <div class="w-10 h-10 rounded-full bg-gradient-to-br from-custom-green to-emerald-600 flex items-center justify-center text-white font-bold text-sm shrink-0">
                {{ userInitial }}
              </div>
              <div class="flex-1">
                <textarea
                  v-model="nouveauCommentaire"
                  placeholder="Partagez votre réflexion..."
                  class="w-full p-4 border border-gray-200 rounded-xl resize-none focus:outline-hidden focus:ring-2 focus:ring-custom-green text-sm"
                  rows="2"
                ></textarea>
                <div class="flex justify-end mt-2">
                  <button
                    @click="soumettreCommentaire"
                    :disabled="!nouveauCommentaire.trim() || envoiCommentaire"
                    class="px-5 py-2 bg-custom-green text-white rounded-lg text-sm font-medium hover:bg-green-600 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
                  >
                    <font-awesome-icon v-if="envoiCommentaire" icon="fa-solid fa-spinner" class="animate-spin mr-2" />
                    <font-awesome-icon v-else icon="fa-solid fa-paper-plane" class="mr-2" />
                    Publier
                  </button>
                </div>
              </div>
            </div>

            <!-- Chargement commentaires -->
            <div v-if="chargementCommentaires" class="flex justify-center py-8">
              <div class="animate-spin rounded-full h-8 w-8 border-t-2 border-b-2 border-custom-green"></div>
            </div>

            <!-- Liste des commentaires -->
            <div v-else class="space-y-4">
              <div
                v-for="commentaire in commentaires"
                :key="commentaire.id"
                class="flex items-start space-x-3 p-4 bg-gray-50 rounded-xl"
              >
                <div class="w-9 h-9 rounded-full bg-gradient-to-br from-gray-400 to-gray-600 flex items-center justify-center text-white font-bold text-xs shrink-0">
                  {{ (commentaire.auteur.prenom || commentaire.auteur.nom).charAt(0) }}
                </div>
                <div class="flex-1 min-w-0">
                  <div class="flex items-center space-x-2 mb-1">
                    <span class="font-semibold text-gray-800 text-sm">
                      {{ commentaire.auteur.prenom }} {{ commentaire.auteur.nom }}
                    </span>
                    <span class="text-xs text-gray-500">
                      {{ formatDateRelative(commentaire.created_at) }}
                    </span>
                  </div>
                  <p class="text-gray-700 text-sm">{{ commentaire.contenu }}</p>
                  <div class="flex items-center space-x-4 mt-2">
                    <button class="text-xs text-gray-500 hover:text-custom-green flex items-center">
                      <font-awesome-icon icon="fa-regular fa-heart" class="mr-1" />
                      {{ commentaire.nombre_likes }}
                    </button>
                  </div>
                </div>
              </div>

              <div v-if="commentaires.length === 0" class="text-center py-8">
                <font-awesome-icon icon="fa-regular fa-comment-dots" class="text-4xl text-gray-300 mb-3" />
                <p class="text-gray-500 text-sm">Aucun commentaire pour le moment.</p>
                <p class="text-gray-400 text-xs mt-1">Soyez le premier à commenter !</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import {
  formatDateRelative,
  getCategoryStyle,
  getCategoryLabel,
  type CodiMoiPostAPI,
  type CommentaireAPI,
  type CategoriePost,
} from '~/composables/useCodiMoi'
import { useUserStore } from '~/stores/user'

const props = defineProps<{
  post: CodiMoiPostAPI | null
  commentaires: CommentaireAPI[]
  chargementCommentaires: boolean
}>()

const emit = defineEmits<{
  close: []
  like: []
  dislike: []
  share: []
  commenter: [contenu: string]
}>()

const userStore = useUserStore()
const nouveauCommentaire = ref('')
const envoiCommentaire = ref(false)

const userInitial = computed(() => {
  if (userStore.utilisateur?.prenom) return userStore.utilisateur.prenom.charAt(0)
  if (userStore.utilisateur?.nom) return userStore.utilisateur.nom.charAt(0)
  return '?'
})

const soumettreCommentaire = async () => {
  if (!nouveauCommentaire.value.trim()) return
  envoiCommentaire.value = true
  emit('commenter', nouveauCommentaire.value.trim())
  nouveauCommentaire.value = ''
  envoiCommentaire.value = false
}
</script>

<style scoped>
@keyframes slideIn {
  from { transform: translateY(-20px); opacity: 0; }
  to { transform: translateY(0); opacity: 1; }
}

.animate-slideIn {
  animation: slideIn 0.3s ease-out forwards;
}

.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: opacity 0.3s ease;
}

.modal-fade-enter-from,
.modal-fade-leave-to {
  opacity: 0;
}
</style>
