<script setup lang="ts">
// Chat de session (US6) — reprise d'historique REST + diffusion LiveKit si dispo
// Tailwind v4 pur
import type { MessageSessionAPI } from '~/composables/useAfrolang'

interface Props {
  sessionId: string
}

const props = defineProps<Props>()

const { listerMessagesSession, envoyerMessageSession } = useAfrolang()
const userStore = useUserStore()

const messages = ref<MessageSessionAPI[]>([])
const nouveauMessage = ref('')
const envoyerEnCours = ref(false)
const chargementHistorique = ref(false)
const zoneFlux = ref<HTMLElement | null>(null)

const initiales = (auteur: MessageSessionAPI) => {
  const p = (auteur.auteur_prenom ?? '')[0] ?? ''
  const n = (auteur.auteur_nom ?? '')[0] ?? ''
  return (p + n).toUpperCase() || '?'
}

const estMoi = (auteurId: string) => userStore.user?.id === auteurId

const defiler = async () => {
  await nextTick()
  if (zoneFlux.value) zoneFlux.value.scrollTop = zoneFlux.value.scrollHeight
}

const chargerHistorique = async () => {
  chargementHistorique.value = true
  messages.value = await listerMessagesSession(props.sessionId, { limit: 200 })
  chargementHistorique.value = false
  await defiler()
}

const envoyer = async () => {
  const contenu = nouveauMessage.value.trim()
  if (!contenu || envoyerEnCours.value) return
  envoyerEnCours.value = true
  const msg = await envoyerMessageSession(props.sessionId, contenu)
  envoyerEnCours.value = false
  if (msg) {
    messages.value = [...messages.value, msg]
    nouveauMessage.value = ''
    await defiler()
  }
}

onMounted(() => {
  chargerHistorique()
})

watch(() => props.sessionId, () => {
  chargerHistorique()
})
</script>

<template>
  <section class="flex flex-col h-full rounded-lg border border-gray-200 bg-white">
    <header class="border-b border-gray-200 px-4 py-3">
      <h3 class="text-sm font-semibold text-gray-900">Messagerie</h3>
    </header>

    <div
      ref="zoneFlux"
      class="flex-1 overflow-y-auto px-4 py-3 space-y-3"
      style="min-height: 320px;"
    >
      <p v-if="chargementHistorique" class="text-center text-sm text-gray-500">
        Chargement de l'historique...
      </p>
      <p v-else-if="messages.length === 0" class="text-center text-sm text-gray-500">
        Aucun message pour l'instant — soyez le premier à écrire.
      </p>
      <div
        v-for="message in messages"
        :key="message.id"
        :class="['flex gap-2', estMoi(message.auteur_id) ? 'justify-end' : 'justify-start']"
      >
        <div
          v-if="!estMoi(message.auteur_id)"
          class="flex h-8 w-8 flex-shrink-0 items-center justify-center rounded-full bg-custom-chocolat text-xs font-semibold text-white"
        >
          {{ initiales(message) }}
        </div>
        <div
          :class="[
            'max-w-[70%] rounded-lg px-3 py-2 text-sm',
            estMoi(message.auteur_id)
              ? 'bg-custom-chocolat text-white'
              : 'bg-gray-100 text-gray-900',
          ]"
        >
          <p v-if="!estMoi(message.auteur_id)" class="text-xs font-medium opacity-80">
            {{ message.auteur_prenom }} {{ message.auteur_nom }}
          </p>
          <p class="whitespace-pre-wrap break-words">{{ message.contenu }}</p>
        </div>
      </div>
    </div>

    <form class="border-t border-gray-200 p-3 flex gap-2" @submit.prevent="envoyer">
      <input
        v-model="nouveauMessage"
        type="text"
        placeholder="Votre message..."
        maxlength="4000"
        class="flex-1 rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-custom-chocolat focus:outline-none"
      />
      <button
        type="submit"
        :disabled="envoyerEnCours || !nouveauMessage.trim()"
        class="rounded-md bg-custom-chocolat px-4 py-2 text-sm font-medium text-white hover:bg-amber-800 disabled:opacity-50"
      >
        Envoyer
      </button>
    </form>
  </section>
</template>
