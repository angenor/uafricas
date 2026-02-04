<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Breadcrumb -->
    <div class="backdrop-blur-xs py-3 px-4 md:px-72 bg-white shadow-xs">
      <CommonBreadcrumbNav :custom-breadcrumbs="breadcrumbs" />
    </div>

    <!-- Contenu -->
    <div v-if="forum" class="max-w-3xl mx-auto py-8 px-4">
      <ForumsForumCard
        :forum="forum"
        :comments="forumComments"
        @add-comment="handleAddComment"
      />
    </div>

    <!-- État non trouvé -->
    <div v-else class="text-center py-16">
      <div class="text-5xl text-gray-300 mb-4">
        <font-awesome-icon icon="fa-solid fa-comment-slash" />
      </div>
      <h3 class="text-xl font-semibold text-gray-500">
        Publication non trouvée
      </h3>
      <NuxtLink to="/evenements/forums" class="mt-4 inline-block text-custom-green hover:underline">
        Retour aux forums
      </NuxtLink>
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  getForumById,
  getForumComments as getComments,
  commentsMock,
  type Forum,
  type ForumComment
} from '~/mocks/forums'

const route = useRoute()
const forumId = route.params.id as string

const forum = ref<Forum | undefined>(undefined)
const forumComments = ref<ForumComment[]>([])

const breadcrumbs = computed(() => [
  { label: 'Centre Culturel', to: '/africa-culture' },
  { label: 'Événements', to: '/evenements' },
  { label: 'Forums', to: '/evenements/forums' },
  { label: 'Détail', to: null }
])

useHead({
  title: 'Détail Forum | UAfricas'
})

const handleAddComment = (data: { forumId: string; content: string }) => {
  const newComment: ForumComment = {
    id: `comment-${Date.now()}`,
    forumId: data.forumId,
    content: data.content,
    user: {
      uid: 'user-current',
      email: 'test@example.com',
      nom: 'Test',
      prenom: 'Utilisateur',
      photo_url: null
    },
    created_at: new Date(),
    likes: 0
  }
  forumComments.value.push(newComment)
}

onMounted(() => {
  forum.value = getForumById(forumId)
  if (forum.value) {
    forumComments.value = getComments(forumId)
  }
})
</script>
