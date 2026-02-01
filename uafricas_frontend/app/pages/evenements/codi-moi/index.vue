<template>
  <div class="min-h-screen z-0">
    <!-- Hero -->
    <CodiMoiCodiMoiHero
      titre="Codi-Moi"
      sous-titre="Codification des valeurs africaines et afro-descendantes"
    />

    <!-- Breadcrumb -->
    <div class="backdrop-blur-sm mx-auto px-4 py-3 bg-gray-50 border-b border-gray-200">
      <div class="max-w-7xl mx-auto">
        <CommonBreadcrumbNav :custom-breadcrumbs="breadcrumbs" />
      </div>
    </div>

    <!-- Container principal -->
    <div class="max-w-7xl mx-auto bg-white min-h-screen rounded-lg shadow-xl relative -mt-6">
      <div class="px-4 py-6">
        <div class="flex flex-col lg:flex-row gap-8">
          <!-- Sidebar -->
          <CodiMoiCodiMoiSidebar
            :amis="amis"
            :stats="stats"
            :popular-posts="popularPosts"
            @go-to-post="goToPost"
            class="order-2 lg:order-1"
          />

          <!-- Colonne principale -->
          <div class="flex-1 order-1 lg:order-2">
            <!-- Filtres -->
            <CodiMoiCodiMoiFilters
              v-model:active-category="activeCategory"
              :user-name="user.prenom"
              :user-photo="user.photo_url"
              v-model:filters="filters"
              v-model:search-keywords="searchKeywords"
              v-model:search-pays="searchPays"
              @create-post="showCreateModal = true"
              @apply-search="applySearch"
            />

            <!-- Liste des posts -->
            <div class="space-y-6">
              <CodiMoiCodiMoiCard
                v-for="post in filteredPosts"
                :key="post.id"
                :post="post"
                @click="openPostDetail(post)"
                @like="handleLike(post)"
                @dislike="handleDislike(post)"
                @comment="openPostDetail(post)"
                @share="handleShare(post)"
              />
            </div>

            <!-- État vide -->
            <div v-if="filteredPosts.length === 0" class="text-center py-16">
              <div class="text-5xl text-gray-300 mb-4">
                <font-awesome-icon icon="fa-solid fa-file-circle-xmark" />
              </div>
              <h3 class="text-xl font-semibold text-gray-500">
                Aucun post trouvé
              </h3>
              <p class="text-gray-400 mt-2">
                Modifiez vos filtres ou créez un nouveau post
              </p>
              <button
                @click="showCreateModal = true"
                class="mt-4 text-white bg-custom-green rounded-md py-2 px-4 hover:bg-custom-green/90 transition-colors"
              >
                Créer un post
              </button>
            </div>

            <!-- Pagination -->
            <div v-if="hasMorePosts" class="mt-8 text-center">
              <button
                @click="loadMore"
                :disabled="loading"
                class="px-6 py-3 bg-custom-green text-white rounded-lg hover:bg-green-600 transition-colors disabled:opacity-50"
              >
                <font-awesome-icon v-if="loading" icon="fa-solid fa-spinner" class="animate-spin mr-2" />
                {{ loading ? 'Chargement...' : 'Charger plus' }}
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  codiMoiPostsMock,
  getStats,
  getPopularPosts,
  filterPosts,
  searchPosts,
  type CodiMoiPost,
  type CategoriePost,
  type UserInfo
} from '~/mocks/codi-moi'

useHead({
  title: 'Codi-Moi - Codification des valeurs | UAfricas'
})

const router = useRouter()

const breadcrumbs = [
  { label: 'Centre Culturel', to: '/africa-culture' },
  { label: 'Promotion des Valeurs', to: '/promotion-valeur' },
  { label: 'Événements', to: '/evenements' },
  { label: 'Codi-Moi', to: null }
]

// Mock user
const user = {
  uid: 'user-current',
  prenom: 'Utilisateur',
  nom: 'Test',
  email: 'test@example.com',
  photo_url: null
}

const posts = ref<CodiMoiPost[]>([])
const amis = ref<UserInfo[]>([])
const stats = ref(getStats())
const popularPosts = ref<CodiMoiPost[]>([])

const showCreateModal = ref(false)
const activeCategory = ref('')
const loading = ref(false)
const hasMorePosts = ref(true)

const filters = ref({
  mesPublications: false,
  bonnesPratiques: false,
  citations: false,
  proverbesAdages: false,
  ressourcesHistoriques: false
})

const searchKeywords = ref('')
const searchPays = ref('')

const filteredPosts = computed(() => {
  let result = [...posts.value]

  // Filtre par catégorie
  if (activeCategory.value) {
    result = result.filter(p => p.categorie === activeCategory.value)
  }

  // Filtres rapides
  if (filters.value.proverbesAdages) {
    result = result.filter(p => p.categorie === 'proverbe_adage')
  }
  if (filters.value.citations) {
    result = result.filter(p => p.categorie === 'citation')
  }
  if (filters.value.bonnesPratiques) {
    result = result.filter(p => p.categorie === 'bonne_pratique')
  }
  if (filters.value.ressourcesHistoriques) {
    result = result.filter(p => p.categorie === 'ressource_historique')
  }

  // Recherche par mots-clés
  if (searchKeywords.value) {
    const kw = searchKeywords.value.toLowerCase()
    result = result.filter(p =>
      p.contenu.toLowerCase().includes(kw) ||
      p.explication?.toLowerCase().includes(kw) ||
      p.hashtags.some(h => h.toLowerCase().includes(kw))
    )
  }

  // Filtre par pays
  if (searchPays.value) {
    result = result.filter(p => p.pays === searchPays.value)
  }

  return result
})

const openPostDetail = (post: CodiMoiPost) => {
  router.push(`/evenements/codi-moi/${post.id}`)
}

const goToPost = (post: CodiMoiPost) => {
  router.push(`/evenements/codi-moi/${post.id}`)
}

const handleLike = (post: CodiMoiPost) => {
  const index = posts.value.findIndex(p => p.id === post.id)
  if (index !== -1) {
    if (posts.value[index].userReaction === 'like') {
      posts.value[index].userReaction = null
      posts.value[index].stats.likes--
    } else {
      if (posts.value[index].userReaction === 'dislike') {
        posts.value[index].stats.dislikes--
      }
      posts.value[index].userReaction = 'like'
      posts.value[index].stats.likes++
    }
  }
}

const handleDislike = (post: CodiMoiPost) => {
  const index = posts.value.findIndex(p => p.id === post.id)
  if (index !== -1) {
    if (posts.value[index].userReaction === 'dislike') {
      posts.value[index].userReaction = null
      posts.value[index].stats.dislikes--
    } else {
      if (posts.value[index].userReaction === 'like') {
        posts.value[index].stats.likes--
      }
      posts.value[index].userReaction = 'dislike'
      posts.value[index].stats.dislikes++
    }
  }
}

const handleShare = (post: CodiMoiPost) => {
  const index = posts.value.findIndex(p => p.id === post.id)
  if (index !== -1) {
    posts.value[index].stats.partages++
  }
  alert('Lien copié ! (Mode démo)')
}

const applySearch = () => {
  // La recherche est réactive via computed
}

const loadMore = () => {
  loading.value = true
  setTimeout(() => {
    loading.value = false
    hasMorePosts.value = false
  }, 1000)
}

onMounted(() => {
  posts.value = [...codiMoiPostsMock]
  popularPosts.value = getPopularPosts(5)
})
</script>
