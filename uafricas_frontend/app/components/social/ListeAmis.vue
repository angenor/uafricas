<script setup lang="ts">
import type { ConversationAPI } from '~/composables/useMessagerie'

const props = withDefaults(defineProps<{
  /** Filtre de recherche appliqué au nom et au dernier message. */
  filtre?: string
}>(), { filtre: '' })

defineEmits<{ (e: 'selectionner', amiId: string): void }>()

const { conversations } = useMessagerie()

/**
 * Filtrage CÔTÉ CLIENT sur la liste déjà chargée : le serveur ne propose pas de
 * recherche de conversations, et en réclamer une à chaque frappe ferait un
 * aller-retour réseau pour un jeu qui tient en mémoire.
 */
const conversationsFiltrees = computed(() => {
  const q = props.filtre.trim().toLowerCase()
  if (!q) return conversations.value
  return conversations.value.filter((c) => {
    const nom = `${c.ami.prenom} ${c.ami.nom}`.toLowerCase()
    const extrait = c.dernier_message?.extrait?.toLowerCase() ?? ''
    return nom.includes(q) || extrait.includes(q)
  })
})

const initiaux = (c: ConversationAPI): string =>
  `${c.ami.prenom?.charAt(0)?.toUpperCase() || ''}${c.ami.nom?.charAt(0)?.toUpperCase() || ''}`

/**
 * Aujourd'hui l'heure, cette année le jour et le mois, au-delà l'année.
 * Une heure nue sur un message de l'an dernier laisserait croire qu'il est
 * d'aujourd'hui.
 */
const quand = (iso: string): string => {
  const d = new Date(iso)
  const maintenant = new Date()
  const memeJour = d.toDateString() === maintenant.toDateString()
  if (memeJour) return d.toLocaleTimeString('fr-FR', { hour: '2-digit', minute: '2-digit' })
  if (d.getFullYear() === maintenant.getFullYear()) {
    return d.toLocaleDateString('fr-FR', { day: 'numeric', month: 'short' })
  }
  return d.toLocaleDateString('fr-FR', { month: 'short', year: 'numeric' })
}
</script>

<template>
  <ul class="divide-y divide-af-bordure overflow-y-auto">
    <li
      v-for="c in conversationsFiltrees"
      :key="c.conversation_id"
      class="flex cursor-pointer items-center gap-3 px-4 py-3 transition"
      :class="c.non_lus > 0 ? 'bg-af-chocolat/[0.06] hover:bg-af-chocolat/[0.1]' : 'hover:bg-af-fond'"
      @click="$emit('selectionner', c.ami.id)"
    >
      <div class="relative shrink-0">
        <img
          v-if="urlMedia(c.ami.photoUrl)"
          :src="urlMedia(c.ami.photoUrl)!"
          :alt="`${c.ami.prenom} ${c.ami.nom}`"
          class="size-11 rounded-full border border-af-bordure object-cover"
        />
        <div
          v-else
          class="grid size-11 place-items-center rounded-full bg-af-chocolat/15 text-[14px]/[1.4] font-bold text-af-chocolat"
        >
          {{ initiaux(c) }}
        </div>
      </div>

      <div class="min-w-0 flex-1">
        <div class="flex items-baseline justify-between gap-2">
          <p class="truncate text-[14px]/[1.4] font-bold text-af-encre">
            {{ c.ami.prenom }} {{ c.ami.nom }}
          </p>
          <span v-if="c.dernier_message" class="shrink-0 text-[12px]/[1.4] text-af-atone">
            {{ quand(c.dernier_message.created_at) }}
          </span>
        </div>
        <p
          class="mt-0.5 truncate text-[12px]/[1.4]"
          :class="c.non_lus > 0 ? 'font-bold text-af-encre' : 'text-af-atone'"
        >
          <span v-if="c.verrouillee" class="text-af-atone italic">
            <font-awesome-icon icon="fa-solid fa-ban" class="mr-1" />Conversation verrouillée
          </span>
          <template v-else-if="c.dernier_message">{{ c.dernier_message.extrait }}</template>
          <span v-else class="text-af-atone italic">Aucun message, démarrez la conversation</span>
        </p>
      </div>

      <!-- Pastille de non-lus : à droite comme chez LinkedIn, où elle compte la
           conversation et non chaque message. -->
      <span
        v-if="c.non_lus > 0"
        class="grid size-5 shrink-0 place-items-center rounded-full bg-af-chocolat text-[10px] font-bold text-white"
      >{{ c.non_lus > 9 ? '9+' : c.non_lus }}</span>
    </li>

    <li v-if="!conversationsFiltrees.length && filtre" class="px-4 py-8 text-center text-[12px]/[1.4] text-af-atone">
      Aucune conversation ne correspond à « {{ filtre }} ».
    </li>
  </ul>
</template>
