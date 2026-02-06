<template>
  <div class="min-h-screen pb-10 bg-gray-50">
    <!-- Hero Section -->
    <div
      class="relative h-80 bg-cover bg-center"
      style="background-image: url('https://images.unsplash.com/photo-1516026672322-bc52d61a55d5?ixlib=rb-1.2.1&auto=format&fit=crop&w=1900&q=80')"
    >
      <div class="absolute inset-0 bg-gradient-to-r from-custom-chocolat/90 to-black/70"></div>

      <div class="absolute inset-0 flex flex-col items-center justify-center mt-5">
        <h1 class="text-white text-4xl md:text-5xl font-bold mb-4 animate-title text-center px-4">
          Codi-Moi
        </h1>
        <div class="h-1 w-24 bg-custom-green rounded animate-line"></div>
        <p class="text-white text-xl md:text-2xl mt-4 animate-subtitle max-w-4xl text-center px-4">
          Codification des valeurs africaines et afro-descendantes
        </p>
      </div>
    </div>

    <!-- Barre de recherche flottante -->
    <CodiMoiFilters
      v-model:active-category="activeCategory"
      v-model:search-keywords="searchKeywords"
      v-model:search-pays="searchPays"
    />

    <!-- Breadcrumb -->
    <div class="max-w-7xl mx-auto px-4 mt-6">
      <CommonBreadcrumbNav :custom-breadcrumbs="breadcrumbs" />
    </div>

    <!-- Titre et bouton ajouter -->
    <div class="max-w-7xl mx-auto px-4 mt-6 mb-4 flex justify-between items-center">
      <h2 class="text-2xl font-bold text-gray-800">
        Publications
        <span v-if="hasActiveFilters" class="text-base font-normal text-gray-500 ml-2">
          ({{ totalPosts }} résultat{{ totalPosts > 1 ? 's' : '' }})
        </span>
      </h2>
      <div class="flex items-center gap-3">
        <button
          v-if="hasActiveFilters"
          @click="resetFilters"
          class="flex items-center gap-2 px-4 py-2 text-gray-600 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 transition-all duration-200 text-sm"
        >
          <font-awesome-icon icon="fa-solid fa-xmark" />
          <span class="hidden sm:inline">Effacer les filtres</span>
        </button>
        <button
          @click="showCreateModal = true"
          class="flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-custom-chocolat to-amber-700 text-white rounded-lg transition-all duration-300 hover:shadow-lg transform hover:scale-105"
        >
          <font-awesome-icon icon="fa-solid fa-plus" />
          <span>Nouveau post</span>
        </button>
      </div>
    </div>

    <!-- État de chargement -->
    <div v-if="loading && posts.length === 0" class="flex justify-center py-16">
      <div class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-custom-green"></div>
    </div>

    <!-- État d'erreur -->
    <div v-else-if="pageErreur && posts.length === 0" class="max-w-7xl mx-auto px-4 py-16 text-center">
      <div class="inline-flex items-center justify-center w-20 h-20 rounded-full bg-red-50 mb-4">
        <font-awesome-icon icon="fa-solid fa-triangle-exclamation" class="text-3xl text-red-400" />
      </div>
      <h3 class="text-lg font-semibold text-gray-600">Erreur de chargement</h3>
      <p class="text-gray-400 mt-1 text-sm">{{ pageErreur }}</p>
      <button
        @click="chargerPosts()"
        class="mt-4 px-5 py-2.5 bg-gradient-to-r from-custom-green to-green-600 text-white rounded-lg transition-all duration-300 transform hover:scale-105 text-sm font-medium"
      >
        <font-awesome-icon icon="fa-solid fa-rotate-right" class="mr-2" />
        Réessayer
      </button>
    </div>

    <!-- Contenu principal -->
    <div v-else class="max-w-7xl mx-auto px-4">
      <div class="flex flex-col lg:flex-row gap-6">
        <!-- Colonne principale -->
        <div class="flex-1 min-w-0">
          <!-- Liste des posts animée -->
          <TransitionGroup
            name="post-list"
            tag="div"
            class="space-y-5"
          >
            <CodiMoiCard
              v-for="post in posts"
              :key="post.id"
              :post="post"
              @click="navigateToPost(post.id)"
              @like="handleReaction(post.id, 'like')"
              @dislike="handleReaction(post.id, 'dislike')"
              @comment="navigateToPost(post.id)"
              @share="handleShare(post)"
            />
          </TransitionGroup>

          <!-- État vide -->
          <div v-if="posts.length === 0" class="text-center py-16">
            <div class="inline-flex items-center justify-center w-20 h-20 rounded-full bg-white shadow-md mb-4">
              <font-awesome-icon icon="fa-solid fa-file-circle-xmark" class="text-3xl text-gray-300" />
            </div>
            <h3 class="text-lg font-semibold text-gray-600">Aucune publication trouvée</h3>
            <p class="text-gray-400 mt-1 text-sm">Modifiez vos filtres ou partagez une nouvelle valeur</p>
            <button
              @click="showCreateModal = true"
              class="mt-4 px-5 py-2.5 bg-gradient-to-r from-custom-green to-green-600 text-white rounded-lg transition-all duration-300 transform hover:scale-105 text-sm font-medium"
            >
              <font-awesome-icon icon="fa-solid fa-plus" class="mr-2" />
              Créer un post
            </button>
          </div>

          <!-- Pagination -->
          <div v-if="hasMorePosts" class="mt-8 text-center">
            <button
              @click="loadMore"
              :disabled="loadingMore"
              class="px-6 py-2.5 bg-white text-custom-green border border-custom-green rounded-lg hover:bg-custom-green hover:text-white transition-all duration-300 text-sm font-medium disabled:opacity-50 transform hover:scale-105"
            >
              <font-awesome-icon v-if="loadingMore" icon="fa-solid fa-spinner" class="animate-spin mr-2" />
              {{ loadingMore ? 'Chargement...' : 'Voir plus de publications' }}
            </button>
          </div>
        </div>

        <!-- Sidebar -->
        <CodiMoiSidebar
          :amis="amis"
          :stats="stats"
          :popular-posts="popularPosts"
          @go-to-post="(post: CodiMoiPostAPI) => navigateToPost(post.id)"
        />
      </div>
    </div>

    <!-- Notification toast -->
    <Transition name="toast">
      <div
        v-if="showToast"
        class="fixed bottom-6 left-1/2 -translate-x-1/2 z-50 bg-gray-800 text-white px-5 py-3 rounded-lg shadow-lg text-sm flex items-center gap-2"
      >
        <font-awesome-icon icon="fa-solid fa-check-circle" class="text-custom-green" />
        {{ toastMessage }}
      </div>
    </Transition>

    <!-- Modale création de post -->
    <Transition name="modal-fade">
      <div v-if="showCreateModal" class="z-50 fixed inset-0 flex items-center justify-center">
        <div @click="showCreateModal = false" class="absolute inset-0 bg-black/60 backdrop-blur-xs"></div>

        <div class="relative w-full max-w-2xl mx-4 md:mx-auto animate-slideIn">
          <form
            @submit.prevent="submitPost"
            class="bg-white rounded-xl overflow-hidden shadow-2xl max-h-[90vh] overflow-y-auto"
          >
            <!-- En-tête -->
            <div class="bg-gradient-to-r from-custom-green to-emerald-700 py-4 sticky top-0 z-10">
              <h2 class="text-white text-center text-xl font-bold">
                Partager une valeur africaine
              </h2>
            </div>

            <div class="p-6">
              <!-- Catégorie -->
              <div class="mb-4">
                <label class="block text-gray-700 text-sm font-bold mb-2">Catégorie</label>
                <select
                  v-model="postForm.categorie"
                  required
                  class="w-full px-3 py-2 border rounded-lg bg-white focus:outline-hidden focus:ring-2 focus:ring-custom-green transition-all duration-200"
                >
                  <option value="">Sélectionnez une catégorie</option>
                  <option
                    v-for="cat in categoriesForm"
                    :key="cat.value"
                    :value="cat.value"
                  >
                    {{ cat.label }}
                  </option>
                </select>
              </div>

              <!-- Contenu -->
              <div class="mb-4">
                <label class="block text-gray-700 text-sm font-bold mb-2">
                  {{ isQuoteCategory ? 'Proverbe / Citation' : 'Titre du contenu' }}
                </label>
                <textarea
                  v-model="postForm.contenu"
                  required
                  class="w-full px-3 py-2 border rounded-lg resize-none focus:outline-hidden focus:ring-2 focus:ring-custom-green transition-all duration-200"
                  :rows="isQuoteCategory ? 3 : 2"
                  :placeholder="isQuoteCategory ? 'Saisissez le proverbe ou la citation...' : 'Titre de votre publication...'"
                ></textarea>
              </div>

              <!-- Auteur (citations uniquement) -->
              <div v-if="postForm.categorie === 'citation'" class="mb-4">
                <label class="block text-gray-700 text-sm font-bold mb-2">Auteur de la citation</label>
                <input
                  v-model="postForm.nomAuteur"
                  type="text"
                  class="w-full px-3 py-2 border rounded-lg focus:outline-hidden focus:ring-2 focus:ring-custom-green transition-all duration-200"
                  placeholder="Ex : Nelson Mandela"
                />
              </div>

              <!-- Couleur de fond (proverbes/citations) -->
              <div v-if="isQuoteCategory" class="mb-4">
                <label class="block text-gray-700 text-sm font-bold mb-2">Couleur de fond</label>
                <div class="flex flex-wrap gap-2">
                  <button
                    v-for="couleur in COULEURS_FOND"
                    :key="couleur"
                    type="button"
                    @click="postForm.couleurFond = couleur"
                    class="w-9 h-9 rounded-full border-2 transition-all duration-200"
                    :class="postForm.couleurFond === couleur ? 'border-gray-800 scale-110 ring-2 ring-gray-300' : 'border-transparent hover:scale-105'"
                    :style="{ backgroundColor: couleur }"
                  ></button>
                </div>
              </div>

              <!-- Explication -->
              <div class="mb-4">
                <label class="block text-gray-700 text-sm font-bold mb-2">Explication / Contexte</label>
                <textarea
                  v-model="postForm.explication"
                  required
                  class="w-full px-3 py-2 border rounded-lg resize-none focus:outline-hidden focus:ring-2 focus:ring-custom-green transition-all duration-200"
                  rows="3"
                  placeholder="Expliquez le sens ou le contexte de cette valeur..."
                ></textarea>
              </div>

              <!-- Pays et Groupe ethnique -->
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
                <div>
                  <label class="block text-gray-700 text-sm font-bold mb-2">Pays d'origine</label>
                  <select
                    v-model="postForm.pays"
                    required
                    class="w-full px-3 py-2 border rounded-lg bg-white focus:outline-hidden focus:ring-2 focus:ring-custom-green transition-all duration-200"
                  >
                    <option value="">Sélectionnez un pays</option>
                    <option v-for="pays in PAYS_AFRICAINS" :key="pays" :value="pays">{{ pays }}</option>
                  </select>
                </div>
                <div>
                  <label class="block text-gray-700 text-sm font-bold mb-2">Groupe ethnique (optionnel)</label>
                  <select
                    v-model="postForm.groupeEthnique"
                    class="w-full px-3 py-2 border rounded-lg bg-white focus:outline-hidden focus:ring-2 focus:ring-custom-green transition-all duration-200"
                  >
                    <option value="">Aucun</option>
                    <option v-for="groupe in GROUPES_ETHNIQUES" :key="groupe" :value="groupe">{{ groupe }}</option>
                  </select>
                </div>
              </div>

              <!-- Hashtags -->
              <div class="mb-6">
                <label class="block text-gray-700 text-sm font-bold mb-2">Hashtags (séparés par des virgules)</label>
                <input
                  v-model="postForm.hashtagsRaw"
                  type="text"
                  class="w-full px-3 py-2 border rounded-lg focus:outline-hidden focus:ring-2 focus:ring-custom-green transition-all duration-200"
                  placeholder="sagesse, proverbe, afrique"
                />
              </div>

              <!-- Boutons -->
              <div class="flex justify-end space-x-3">
                <button
                  @click="showCreateModal = false"
                  type="button"
                  class="px-4 py-2 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300 transition-colors duration-200"
                >
                  Annuler
                </button>
                <button
                  type="submit"
                  :disabled="isSubmitting"
                  class="px-4 py-2 bg-gradient-to-r from-custom-green to-green-600 text-white rounded-lg hover:from-custom-green hover:to-green-700 transition-all duration-200 transform hover:scale-105 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  {{ isSubmitting ? 'Publication...' : 'Publier' }}
                </button>
              </div>
            </div>
          </form>
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import {
  CATEGORIES_POST,
  COULEURS_FOND,
  PAYS_AFRICAINS,
  GROUPES_ETHNIQUES,
  type CodiMoiPostAPI,
  type CategoriePost,
} from '~/composables/useCodiMoi'

useHead({
  title: 'Codi-Moi - Codification des valeurs | UAfricas'
})

const router = useRouter()
const { erreur: apiErreur, listerPosts, creerPost, reagir } = useCodiMoi()

const breadcrumbs = [
  { label: 'Centre Culturel', to: '/africa-culture' },
  { label: 'Événements', to: '/evenements' },
  { label: 'Codi-Moi', to: undefined }
]

// Données — on utilise directement CodiMoiPostAPI (pas de mapping)
const posts = ref<CodiMoiPostAPI[]>([])
const totalPosts = ref(0)

// Amis (sera remplacé par une API amis plus tard)
const amis = ref([
  { id: '23c32fc1-8cee-45f0-9ccd-b74629cdbf99', nom: 'Traoré', prenom: 'Fatou' },
  { id: '37a5049d-4ece-43a1-bfd9-2b2478993a3d', nom: 'Koné', prenom: 'Ibrahim' },
  { id: '5e5370f5-dcfb-4f6d-aa6c-752a9eb1299b', nom: 'Ndong', prenom: 'Marie' },
  { id: 'e52995bb-06ca-4155-a638-cd4262a5ac4d', nom: 'Ouédraogo', prenom: 'Seydou' },
])

// Stats calculées depuis les posts chargés
const stats = computed(() => {
  const p = posts.value
  return {
    totalPosts: totalPosts.value,
    proverbesAdages: p.filter(x => x.type === 'proverbe_adage').length,
    citations: p.filter(x => x.type === 'citation').length,
    ressourcesHistoriques: p.filter(x => x.type === 'ressource_historique').length,
    bonnesPratiques: p.filter(x => x.type === 'bonne_pratique').length,
    totalLikes: p.reduce((sum, x) => sum + x.nombre_likes, 0),
    totalVues: p.reduce((sum, x) => sum + x.nombre_vues, 0),
  }
})

// Posts populaires triés par likes
const popularPosts = computed(() => {
  return [...posts.value].sort((a, b) => b.nombre_likes - a.nombre_likes).slice(0, 5)
})

// UI state
const showCreateModal = ref(false)
const loading = ref(false)
const loadingMore = ref(false)
const currentPage = ref(1)
const parPage = 10
const pageErreur = ref<string | null>(null)

// Filtres
const activeCategory = ref('')
const searchKeywords = ref('')
const searchPays = ref('')

// Toast
const showToast = ref(false)
const toastMessage = ref('')

const hasActiveFilters = computed(() => {
  return !!(activeCategory.value || searchKeywords.value || searchPays.value)
})

const hasMorePosts = computed(() => {
  return posts.value.length < totalPosts.value
})

// Charger les posts depuis l'API
async function chargerPosts(append = false) {
  if (!append) loading.value = true
  pageErreur.value = null

  const resultat = await listerPosts({
    type: activeCategory.value || undefined,
    recherche: searchKeywords.value || undefined,
    pays: searchPays.value || undefined,
    page: currentPage.value,
    par_page: parPage,
  })

  if (resultat) {
    if (append) {
      posts.value.push(...resultat.posts)
    } else {
      posts.value = resultat.posts
    }
    totalPosts.value = resultat.total
  } else if (apiErreur.value) {
    pageErreur.value = apiErreur.value
  }

  loading.value = false
}

// Actions
const navigateToPost = (postId: string) => {
  router.push(`/evenements/codi-moi/${postId}`)
}

const handleReaction = async (postId: string, type: 'like' | 'dislike') => {
  const updatedPost = await reagir(postId, type)
  if (updatedPost) {
    // Mettre à jour le post dans la liste avec les nouvelles valeurs du serveur
    const index = posts.value.findIndex(p => p.id === postId)
    if (index !== -1) {
      posts.value[index] = updatedPost
    }
  }
}

const handleShare = (post: CodiMoiPostAPI) => {
  if (navigator.clipboard) {
    navigator.clipboard.writeText(`${window.location.origin}/evenements/codi-moi/${post.id}`)
  }
  showNotification('Lien copié dans le presse-papiers !')
}

const showNotification = (message: string) => {
  toastMessage.value = message
  showToast.value = true
  setTimeout(() => { showToast.value = false }, 2500)
}

// Formulaire de création
const categoriesForm = CATEGORIES_POST.filter(c => c.value !== '')
const isSubmitting = ref(false)

const postForm = ref({
  categorie: '' as CategoriePost | '',
  contenu: '',
  nomAuteur: '',
  explication: '',
  pays: '',
  groupeEthnique: '',
  couleurFond: COULEURS_FOND[0],
  hashtagsRaw: ''
})

const isQuoteCategory = computed(() => {
  return postForm.value.categorie === 'proverbe_adage' || postForm.value.categorie === 'citation'
})

const submitPost = async () => {
  if (!postForm.value.categorie || !postForm.value.contenu || !postForm.value.pays) return

  isSubmitting.value = true

  const hashtags = postForm.value.hashtagsRaw
    .split(',')
    .map(h => h.trim())
    .filter(Boolean)

  const nouveauPost = await creerPost({
    type: postForm.value.categorie,
    contenu: postForm.value.contenu,
    explication: postForm.value.explication || undefined,
    nom_auteur_originel: postForm.value.nomAuteur || undefined,
    pays: postForm.value.pays || undefined,
    groupe_ethnique: postForm.value.groupeEthnique || undefined,
    couleur_fond: isQuoteCategory.value ? (postForm.value.couleurFond ?? '#2D5A27') : undefined,
    hashtags: hashtags.length > 0 ? hashtags : undefined,
  })

  showCreateModal.value = false
  isSubmitting.value = false
  resetForm()

  if (nouveauPost) {
    showNotification('Publication créée avec succès !')
  } else {
    showNotification(apiErreur.value || 'Erreur lors de la création')
  }

  // Recharger la liste depuis l'API
  await chargerPosts()
}

const resetForm = () => {
  postForm.value = {
    categorie: '',
    contenu: '',
    nomAuteur: '',
    explication: '',
    pays: '',
    groupeEthnique: '',
    couleurFond: COULEURS_FOND[0],
    hashtagsRaw: ''
  }
}

const resetFilters = () => {
  activeCategory.value = ''
  searchKeywords.value = ''
  searchPays.value = ''
  currentPage.value = 1
  chargerPosts()
}

const loadMore = async () => {
  loadingMore.value = true
  currentPage.value++
  await chargerPosts(true)
  loadingMore.value = false
}

// Recharger quand les filtres changent (avec debounce)
let debounceTimer: ReturnType<typeof setTimeout> | null = null
watch([activeCategory, searchKeywords, searchPays], () => {
  currentPage.value = 1
  if (debounceTimer) clearTimeout(debounceTimer)
  debounceTimer = setTimeout(() => chargerPosts(), 300)
})

onMounted(() => {
  chargerPosts()
})
</script>

<style scoped>
@keyframes fadeIn {
  from { opacity: 0; transform: translateY(-20px); }
  to { opacity: 1; transform: translateY(0); }
}

@keyframes expandLine {
  from { width: 0; }
  to { width: 6rem; }
}

.animate-title {
  animation: fadeIn 1s ease-out forwards;
}

.animate-subtitle {
  animation: fadeIn 1s ease-out 0.3s forwards;
  opacity: 0;
}

.animate-line {
  animation: expandLine 1.2s ease-out 0.1s forwards;
  width: 0;
}

.post-list-enter-active,
.post-list-leave-active {
  transition: all 0.5s ease;
}

.post-list-enter-from,
.post-list-leave-to {
  opacity: 0;
  transform: translateY(30px);
}

.toast-enter-active {
  transition: all 0.3s ease-out;
}
.toast-leave-active {
  transition: all 0.2s ease-in;
}
.toast-enter-from,
.toast-leave-to {
  opacity: 0;
  transform: translate(-50%, 20px);
}

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
