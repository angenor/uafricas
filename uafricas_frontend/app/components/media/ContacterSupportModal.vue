<script setup lang="ts">
/**
 * Mise en relation avec l'équipe d'un support média (US6, FR-046).
 *
 * La messagerie n'autorise normalement l'envoi qu'entre membres amis : seul
 * l'endpoint métier `POST /api/medias/{type}/{id}/contacter` peut ouvrir ce
 * canal. Le message part vers le propriétaire du support — ou, s'il n'y en a
 * plus, vers son co-détenteur actif le plus ancien.
 */
import type { TypeSupportMedia } from '~/composables/useMediaDetention'

const props = defineProps<{
  isOpen: boolean
  typeSupport: TypeSupportMedia
  supportId: string
  nomSupport: string
}>()

const emit = defineEmits<{ close: [], envoye: [conversationId: string] }>()

const { contacter, erreur } = useMediaDetention()
const userStore = useUserStore()
const { redirigerVersConnexion } = useAuth()

const message = ref('')
const envoi = ref(false)
const conversationId = ref<string | null>(null)

const MESSAGE_MAX = 2000
const messageValide = computed(() => {
  const t = message.value.trim()
  return t.length > 0 && t.length <= MESSAGE_MAX
})

const fermer = () => {
  if (envoi.value) return
  emit('close')
}

/** Réinitialiser à la réouverture : une modale rouverte doit repartir vierge. */
watch(() => props.isOpen, (ouvert) => {
  if (ouvert) {
    message.value = ''
    conversationId.value = null
  }
})

const envoyer = async () => {
  if (!messageValide.value || envoi.value) return
  envoi.value = true
  const resultat = await contacter(props.typeSupport, props.supportId, message.value.trim())
  envoi.value = false
  if (resultat) {
    conversationId.value = resultat.conversation_id
    emit('envoye', resultat.conversation_id)
  }
}

const onEchap = (e: KeyboardEvent) => {
  if (e.key === 'Escape') fermer()
}
onMounted(() => window.addEventListener('keydown', onEchap))
onBeforeUnmount(() => window.removeEventListener('keydown', onEchap))
</script>

<template>
  <Teleport to="body">
    <Transition name="modal-fade">
      <div
        v-if="isOpen"
        class="fixed inset-0 z-[90] flex items-center justify-center p-4 bg-black/70"
        @click.self="fermer"
      >
        <div class="w-full max-w-lg rounded-2xl bg-white shadow-xl overflow-hidden">
          <header class="flex items-start justify-between gap-4 px-6 py-4 border-b border-gray-100">
            <div>
              <h2 class="text-lg font-bold text-gray-900">Contacter « {{ nomSupport }} »</h2>
              <p class="text-sm text-gray-500">
                Votre message ouvre une conversation privée avec l’équipe du support.
              </p>
            </div>
            <button
              type="button"
              class="shrink-0 text-gray-400 hover:text-gray-700 transition-colors cursor-pointer"
              aria-label="Fermer"
              @click="fermer"
            >
              <font-awesome-icon :icon="['fas', 'xmark']" class="w-5 h-5" />
            </button>
          </header>

          <!-- Succès -->
          <div v-if="conversationId" class="px-6 py-8 text-center">
            <font-awesome-icon
              :icon="['fas', 'circle-check']"
              class="w-10 h-10 text-emerald-500 mb-3"
            />
            <p class="text-gray-900 font-semibold mb-1">Message envoyé</p>
            <p class="text-sm text-gray-600 mb-5">
              La conversation est ouverte : la suite des échanges se poursuit dans votre messagerie.
            </p>
            <NuxtLink
              to="/messagerie"
              class="inline-flex items-center gap-2 rounded-full bg-custom-chocolat text-white px-5 py-2 text-sm font-semibold hover:opacity-90 transition-opacity"
            >
              <font-awesome-icon :icon="['fas', 'comments']" class="w-4 h-4" />
              Ouvrir la messagerie
            </NuxtLink>
          </div>

          <!-- Formulaire -->
          <div v-else class="px-6 py-5">
            <div
              v-if="!userStore.accessToken"
              class="rounded-xl bg-amber-50 border border-amber-200 px-4 py-3 text-sm text-amber-900 mb-4"
            >
              Vous devez être connecté pour envoyer un message.
              <button
                type="button"
                class="underline font-semibold cursor-pointer"
                @click="redirigerVersConnexion()"
              >
                Se connecter
              </button>
            </div>

            <label for="message-support" class="block text-sm font-semibold text-gray-800 mb-2">
              Votre message
            </label>
            <textarea
              id="message-support"
              v-model="message"
              rows="6"
              :maxlength="MESSAGE_MAX"
              placeholder="Présentez votre projet, votre demande ou votre proposition de collaboration…"
              class="w-full px-4 py-3 bg-gray-50 border border-gray-200 rounded-xl text-sm text-gray-800 focus:outline-hidden focus:ring-2 focus:ring-custom-chocolat focus:border-transparent transition-all resize-y"
            />
            <p class="mt-1 text-xs text-gray-400 text-right">
              {{ message.trim().length }} / {{ MESSAGE_MAX }}
            </p>

            <p
              v-if="erreur"
              class="mt-3 rounded-xl bg-red-50 border border-red-200 px-4 py-3 text-sm text-red-700"
            >
              {{ erreur }}
            </p>

            <div class="flex justify-end gap-3 mt-5">
              <button
                type="button"
                class="rounded-full border border-gray-200 text-gray-600 px-5 py-2 text-sm hover:bg-gray-50 transition-colors cursor-pointer"
                :disabled="envoi"
                @click="fermer"
              >
                Annuler
              </button>
              <button
                type="button"
                class="inline-flex items-center gap-2 rounded-full bg-custom-chocolat text-white px-5 py-2 text-sm font-semibold hover:opacity-90 transition-opacity disabled:opacity-40 disabled:cursor-not-allowed cursor-pointer"
                :disabled="!messageValide || envoi || !userStore.accessToken"
                @click="envoyer"
              >
                <span
                  v-if="envoi"
                  class="animate-spin rounded-full h-4 w-4 border-b-2 border-white"
                />
                <font-awesome-icon v-else :icon="['fas', 'paper-plane']" class="w-4 h-4" />
                Envoyer
              </button>
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: opacity 0.2s ease;
}
.modal-fade-enter-from,
.modal-fade-leave-to {
  opacity: 0;
}
</style>
