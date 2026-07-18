<template>
  <div class="min-h-screen z-0">
    <!-- Hero (compact, titre ↔ description au survol) -->
    <div class="group relative bg-linear-to-r from-custom-chocolat to-black/90">
      <div class="relative max-w-4xl mx-auto px-4 pt-16 pb-6 text-center select-none">
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12">
          <h1 class="absolute inset-0 flex items-center justify-center text-white text-2xl md:text-4xl font-bold transition-opacity duration-300 group-hover:opacity-0">
            Forum
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Préservons nos cultures les meilleures
          </p>
        </div>
      </div>
    </div>

    <!-- Filtres -->
    <ForumsForumFilters
      :user-name="`${user.prenom} ${user.nom}`"
      :user-email="user.email"
      :user-photo="user.photo_url"
      v-model:filters="filters"
      v-model:filter-country="filterCountry"
      v-model:filter-ethnie="filterEthnie"
      v-model:filter-category="filterCategory"
      @apply-filters="applyFilters"
    />

    <!-- Contenu -->
    <div class="flex">
      <div class="w-full relative z-10">
        <!-- Bouton créer publication -->
        <div class="flex justify-center mb-2 mx-auto">
          <button
            @click="showCreateModal = true"
            class="text-custom-green border-2 border-custom-green px-4 py-2 my-2 rounded-md hover:scale-105 active:scale-95 transition-transform"
          >
            Faire une publication
          </button>
        </div>

        <!-- Liste des forums -->
        <ForumsForumCard
          v-for="forum in filteredForums"
          :key="forum.id"
          :forum="forum"
          :comments="getForumComments(forum.id)"
          @add-comment="handleAddComment"
        />

        <!-- État vide -->
        <div v-if="filteredForums.length === 0" class="text-center py-16">
          <div class="text-5xl text-gray-300 mb-4">
            <font-awesome-icon icon="fa-solid fa-comments" />
          </div>
          <h3 class="text-xl font-semibold text-gray-500">
            Aucune publication trouvée
          </h3>
          <p class="text-gray-400 mt-2">
            Soyez le premier à partager votre savoir !
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  forumsMock,
  commentsMock,
  getForumComments as getComments,
  filterForums,
  type Forum,
  type ForumComment,
  type TypeForum
} from '~/mocks/forums'

useHead({
  title: 'Forums - Codification des valeurs | AfricanS'
})

// Mock user
const user = {
  prenom: 'Utilisateur',
  nom: 'Test',
  email: 'test@example.com',
  photo_url: null
}

const showCreateModal = ref(false)
const forums = ref<Forum[]>([])
const comments = ref<ForumComment[]>([])

const filters = ref({
  mesPublications: false,
  bonnesPratiques: false,
  citations: false,
  proverbesAdages: false,
  histoire: false
})

const filterCountry = ref('')
const filterEthnie = ref('')
const filterCategory = ref('')

const filteredForums = computed(() => {
  let result = [...forums.value]

  if (filterCategory.value) {
    result = result.filter(f => f.type === filterCategory.value)
  }

  if (filters.value.proverbesAdages) {
    result = result.filter(f => f.type === 'Proverbe')
  }
  if (filters.value.citations) {
    result = result.filter(f => f.type === 'Citation')
  }
  if (filters.value.bonnesPratiques) {
    result = result.filter(f => f.type === 'Bonne pratique')
  }
  if (filters.value.histoire) {
    result = result.filter(f => f.type === 'Histoire')
  }

  return result
})

const getForumComments = (forumId: string): ForumComment[] => {
  return comments.value.filter(c => c.forumId === forumId)
}

const applyFilters = () => {
  forums.value = filterForums(
    filterCategory.value as TypeForum | '',
    filterCountry.value,
    filterEthnie.value
  )
}

const handleAddComment = (data: { forumId: string; content: string }) => {
  const newComment: ForumComment = {
    id: `comment-${Date.now()}`,
    forumId: data.forumId,
    content: data.content,
    user: {
      uid: 'user-current',
      email: user.email,
      nom: user.nom,
      prenom: user.prenom,
      photo_url: user.photo_url
    },
    created_at: new Date(),
    likes: 0
  }
  comments.value.push(newComment)
}

onMounted(() => {
  forums.value = [...forumsMock]
  comments.value = [...commentsMock]
})
</script>
