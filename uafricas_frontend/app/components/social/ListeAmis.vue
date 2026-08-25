<script setup lang="ts">
import type { ConversationAPI } from '~/composables/useMessagerie'

defineEmits<{ (e: 'selectionner', amiId: string): void }>()

const { conversations } = useMessagerie()
const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string

const photoComplete = (url: string | null): string | null => {
  if (!url) return null
  return url.startsWith('http') ? url : `${apiBase}${url}`
}

const initiaux = (c: ConversationAPI): string =>
  `${c.ami.prenom?.charAt(0)?.toUpperCase() || ''}${c.ami.nom?.charAt(0)?.toUpperCase() || ''}`

const heure = (iso: string): string =>
  new Date(iso).toLocaleTimeString('fr-FR', { hour: '2-digit', minute: '2-digit' })
</script>

<template>
  <ul class="divide-y divide-gray-100 overflow-y-auto">
    <li
      v-for="c in conversations"
      :key="c.conversation_id"
      class="flex items-center gap-3 px-4 py-3 cursor-pointer hover:bg-gray-50 transition"
      @click="$emit('selectionner', c.ami.id)"
    >
      <div class="relative shrink-0">
        <img
          v-if="photoComplete(c.ami.photoUrl)"
          :src="photoComplete(c.ami.photoUrl)!"
          :alt="`${c.ami.prenom} ${c.ami.nom}`"
          class="w-11 h-11 rounded-full object-cover border border-gray-200"
        />
        <div
          v-else
          class="w-11 h-11 rounded-full bg-custom-chocolat text-white flex items-center justify-center text-sm font-bold"
        >
          {{ initiaux(c) }}
        </div>
        <span
          v-if="c.non_lus > 0"
          class="absolute -top-1 -right-1 min-w-5 h-5 px-1 bg-custom-chocolat text-white text-[10px] font-bold rounded-full flex items-center justify-center"
        >{{ c.non_lus > 9 ? '9+' : c.non_lus }}</span>
      </div>

      <div class="flex-1 min-w-0">
        <div class="flex items-center justify-between gap-2">
          <p class="font-semibold text-gray-800 text-sm truncate">{{ c.ami.prenom }} {{ c.ami.nom }}</p>
          <span v-if="c.dernier_message" class="text-[11px] text-gray-400 shrink-0">
            {{ heure(c.dernier_message.created_at) }}
          </span>
        </div>
        <p
          class="text-xs truncate mt-0.5"
          :class="c.non_lus > 0 ? 'text-gray-700 font-medium' : 'text-gray-400'"
        >
          <span v-if="c.verrouillee" class="italic text-gray-400">
            <font-awesome-icon icon="fa-solid fa-ban" class="mr-1" />Conversation verrouillée
          </span>
          <template v-else-if="c.dernier_message">{{ c.dernier_message.extrait }}</template>
          <span v-else class="italic text-gray-400">Aucun message, démarrez la conversation</span>
        </p>
      </div>
    </li>
  </ul>
</template>
