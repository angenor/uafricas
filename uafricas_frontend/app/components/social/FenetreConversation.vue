<script setup lang="ts">
import type { MembreLightAPI } from '~/composables/useAmis'
import { useUserStore } from '~/stores/user'

const props = defineProps<{
  ami: MembreLightAPI
  verrouillee?: boolean
}>()

defineEmits<{ (e: 'retour'): void }>()

const userStore = useUserStore()
const { messages, chargementMessages, ouvrirConversation, fermerConversation, envoyerMessage, supprimerMessage } = useMessagerie()

const CONTENU_MAX = 2000
const brouillon = ref('')
const envoiEnCours = ref(false)
const filRef = ref<HTMLElement | null>(null)

const moi = computed(() => userStore.user?.id)
const peutEnvoyer = computed(() =>
  !props.verrouillee && brouillon.value.trim().length > 0 && brouillon.value.length <= CONTENU_MAX,
)

const defilerEnBas = async () => {
  await nextTick()
  if (filRef.value) filRef.value.scrollTop = filRef.value.scrollHeight
}

const envoyer = async () => {
  if (!peutEnvoyer.value || envoiEnCours.value) return
  const contenu = brouillon.value
  brouillon.value = ''
  envoiEnCours.value = true
  await envoyerMessage(props.ami.id, contenu)
  envoiEnCours.value = false
  defilerEnBas()
}

const heure = (iso: string): string =>
  new Date(iso).toLocaleTimeString('fr-FR', { hour: '2-digit', minute: '2-digit' })

// Charger la conversation à l'ouverture et défiler en bas dès l'arrivée de messages
onMounted(async () => {
  await ouvrirConversation(props.ami.id)
  defilerEnBas()
})

onBeforeUnmount(() => fermerConversation())

watch(() => messages.value.length, defilerEnBas)
</script>

<template>
  <div class="flex flex-col h-full">
    <!-- En-tête -->
    <header class="flex items-center gap-2 px-3 py-2.5 border-b border-gray-100 shrink-0">
      <button
        type="button"
        class="p-1.5 text-gray-500 hover:text-custom-chocolat rounded-lg hover:bg-gray-100 transition"
        aria-label="Retour"
        @click="$emit('retour')"
      >
        <font-awesome-icon icon="fa-solid fa-chevron-left" />
      </button>
      <NuxtLink :to="`/profil/${ami.id}`" class="font-semibold text-gray-800 text-sm truncate hover:text-custom-chocolat">
        {{ ami.prenom }} {{ ami.nom }}
      </NuxtLink>
    </header>

    <!-- Fil de messages -->
    <div ref="filRef" class="flex-1 overflow-y-auto px-3 py-3 space-y-2 bg-gray-50">
      <div v-if="chargementMessages" class="flex justify-center py-8">
        <font-awesome-icon icon="fa-solid fa-spinner" class="text-xl text-custom-chocolat animate-spin" />
      </div>

      <template v-else>
        <p v-if="messages.length === 0" class="text-center text-xs text-gray-400 py-8">
          Aucun message. Dites bonjour à {{ ami.prenom }} !
        </p>

        <div
          v-for="m in messages"
          :key="m.id"
          class="flex group"
          :class="m.expediteur_id === moi ? 'justify-end' : 'justify-start'"
        >
          <div class="flex items-end gap-1 max-w-[78%]">
            <!-- Bouton supprimer (ses propres messages, non supprimés) -->
            <button
              v-if="m.expediteur_id === moi && !m.supprime"
              type="button"
              class="opacity-0 group-hover:opacity-100 p-1 text-gray-300 hover:text-red-500 transition shrink-0"
              aria-label="Supprimer le message"
              @click="supprimerMessage(m.id)"
            >
              <font-awesome-icon icon="fa-solid fa-trash" class="text-[11px]" />
            </button>

            <div
              class="rounded-2xl px-3.5 py-2 text-sm break-words"
              :class="m.expediteur_id === moi
                ? 'bg-custom-chocolat text-white rounded-br-sm'
                : 'bg-white text-gray-800 border border-gray-200 rounded-bl-sm'"
            >
              <span v-if="m.supprime" class="italic opacity-70">
                <font-awesome-icon icon="fa-solid fa-ban" class="mr-1 text-[11px]" />Message supprimé
              </span>
              <span v-else>{{ m.contenu }}</span>
              <span
                class="block text-[10px] mt-0.5 text-right"
                :class="m.expediteur_id === moi ? 'text-white/70' : 'text-gray-400'"
              >{{ heure(m.created_at) }}</span>
            </div>
          </div>
        </div>
      </template>
    </div>

    <!-- Saisie -->
    <footer class="border-t border-gray-100 p-2.5 shrink-0">
      <div v-if="verrouillee" class="text-center text-xs text-gray-400 italic py-2">
        <font-awesome-icon icon="fa-solid fa-ban" class="mr-1" />
        Conversation verrouillée (vous n'êtes plus amis ou ce membre est bloqué).
      </div>
      <form v-else class="flex items-end gap-2" @submit.prevent="envoyer">
        <div class="flex-1">
          <textarea
            v-model="brouillon"
            rows="1"
            :maxlength="CONTENU_MAX"
            placeholder="Écrire un message…"
            class="w-full resize-none rounded-xl border border-gray-200 px-3 py-2 text-sm bg-gray-50 focus:bg-white focus:ring-2 focus:ring-custom-green focus:border-transparent transition max-h-28"
            @keydown.enter.exact.prevent="envoyer"
          />
          <p v-if="brouillon.length > CONTENU_MAX * 0.9" class="text-[10px] text-gray-400 text-right mt-0.5">
            {{ brouillon.length }}/{{ CONTENU_MAX }}
          </p>
        </div>
        <button
          type="submit"
          :disabled="!peutEnvoyer || envoiEnCours"
          class="shrink-0 w-9 h-9 flex items-center justify-center rounded-full bg-custom-chocolat text-white hover:shadow-md transition disabled:opacity-40 disabled:cursor-not-allowed"
          aria-label="Envoyer"
        >
          <font-awesome-icon :icon="envoiEnCours ? 'fa-solid fa-spinner' : 'fa-solid fa-paper-plane'" :class="{ 'animate-spin': envoiEnCours }" />
        </button>
      </form>
    </footer>
  </div>
</template>
