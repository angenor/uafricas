<template>
  <div class="max-w-xl mx-auto my-3">
    <!-- Style Proverbe/Citation -->
    <template v-if="forum.type === 'Proverbe' || forum.type === 'Citation'">
      <div
        class="flex relative min-h-72 items-center text-center font-bold h-32 text-3xl text-white px-12 py-4 rounded-t-xl"
        :class="forum.couleurSelected"
      >
        <!-- Info auteur -->
        <div class="absolute font-normal top-1 left-1 rounded-full flex items-center text-sm bg-white/30 px-2">
          <div class="mr-2">
            Publié par
            <span class="underline">{{ forum.user.prenom }} {{ forum.user.nom }}</span>
          </div>
          <img
            class="h-8 w-8 mx-auto rounded-full border border-green-500 object-cover"
            :src="forum.user.photo_url || 'https://www.pngall.com/wp-content/uploads/5/Profile-PNG-Clipart.png'"
          />
        </div>

        <!-- Contenu -->
        {{ forum.type === 'Proverbe' ? forum.proverbe.libellet : forum.citation.libellet }}

        <!-- Likes -->
        <div class="absolute text-xl bottom-2 left-2 px-3 bg-white/50 border border-white rounded-full text-custom-green">
          <font-awesome-icon icon="fa-solid fa-heart" class="mr-2 hover:scale-125 active:scale-95 transition-all cursor-pointer" />
          {{ forum.likes }}
        </div>

        <!-- Vues -->
        <div class="absolute text-sm font-normal bottom-1 right-1 px-2 bg-white/50 border border-white rounded-full text-custom-green">
          <font-awesome-icon icon="fa-solid fa-eye" class="mr-1" />{{ forum.vues }}
        </div>

        <!-- Date -->
        <div class="text-white bg-white/25 px-2 rounded-l-full absolute top-2 right-0 text-sm font-normal">
          {{ formatDate(forum.created_at) }}
        </div>
      </div>

      <!-- Explication -->
      <div class="relative bg-white rounded-b-xl border">
        <div class="relative">
          <img
            v-if="forum.proverbe.photo_url"
            class="object-cover absolute top-0 left-0 w-full h-full rounded-md"
            :src="forum.proverbe.photo_url"
            alt=""
          />
          <div class="relative z-10 font-bold justify-center px-2 rounded-t-md bg-white/70 inline-flex mt-2 ml-2">
            Explication du {{ forum.type.toLowerCase() }}
          </div>
          <div class="relative z-10 bg-white/70 p-2 rounded-b-md rounded-tr-md ml-2">
            {{ forum.type === 'Proverbe' ? forum.proverbe.explication : forum.citation.explication }}
          </div>

          <div class="flex relative flex-wrap space-x-2 mb-4 z-20">
            <div class="text-sm px-2 bg-white/80 m-2 border border-custom-green rounded-full text-green-800 font-bold">
              <font-awesome-icon icon="fa-solid fa-flag" class="mr-1" />
              {{ forum.type === 'Proverbe' ? forum.proverbe.pays : forum.citation.pays }}
            </div>
          </div>
        </div>

        <!-- Commentaires -->
        <ForumsForumComments
          :forum-id="forum.id"
          :comments="comments"
          @add-comment="emit('addComment', $event)"
        />
      </div>
    </template>

    <!-- Style Bonne pratique/Histoire -->
    <template v-else>
      <div class="relative min-h-72 items-center text-center font-bold h-32 text-3xl text-white rounded-t-xl">
        <img
          v-if="forum.bonne_pratique.photo_url || forum.histoire.photo_url"
          class="absolute w-full h-full rounded-t-xl object-cover"
          :src="forum.bonne_pratique.photo_url || forum.histoire.photo_url"
          alt=""
        />

        <!-- Info auteur -->
        <div class="font-bold absolute left-2 top-2 rounded-full flex items-center text-sm bg-custom-green/30 text-white px-2">
          <div class="mr-2">
            Publié par
            <span class="underline">{{ forum.user.prenom }} {{ forum.user.nom }}</span>
          </div>
          <img
            class="h-8 w-8 mx-auto rounded-full border border-green-500 object-cover"
            :src="forum.user.photo_url || 'https://www.pngall.com/wp-content/uploads/5/Profile-PNG-Clipart.png'"
          />
        </div>

        <!-- Date -->
        <div class="text-white font-bold bg-custom-green/25 px-2 rounded-l-full absolute top-2 right-0 text-sm">
          {{ formatDate(forum.created_at) }}
        </div>

        <!-- Likes -->
        <div class="absolute text-xl bottom-2 left-2 px-3 bg-white/50 border border-white rounded-full text-custom-green">
          <font-awesome-icon icon="fa-solid fa-heart" class="mr-2 hover:scale-125 active:scale-95 transition-all cursor-pointer" />
          {{ forum.likes }}
        </div>

        <!-- Vues -->
        <div class="absolute text-sm font-normal bottom-1 right-1 px-2 bg-white/50 border border-white rounded-full text-custom-green">
          <font-awesome-icon icon="fa-solid fa-eye" class="mr-1" />{{ forum.vues }}
        </div>
      </div>

      <div class="bg-white p-3 rounded-b-xl border">
        <div class="font-bold text-center underline mb-2">
          {{ forum.bonne_pratique.description ? 'Bonne pratique' : 'Histoire' }}
        </div>
        <div>
          {{ forum.bonne_pratique.description || forum.histoire.description }}
        </div>
        <div class="flex flex-wrap space-x-2 mt-3">
          <div class="text-sm font-normal px-2 bg-custom-green/10 border border-custom-green rounded-full text-custom-green">
            <font-awesome-icon icon="fa-solid fa-flag" class="mr-1" />
            {{ forum.bonne_pratique.pays || forum.histoire.pays }}
          </div>
        </div>

        <!-- Commentaires -->
        <ForumsForumComments
          :forum-id="forum.id"
          :comments="comments"
          @add-comment="emit('addComment', $event)"
        />
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { formatDate, type Forum, type ForumComment } from '~/mocks/forums'

defineProps<{
  forum: Forum
  comments: ForumComment[]
}>()

const emit = defineEmits<{
  addComment: [data: { forumId: string; content: string }]
}>()
</script>
