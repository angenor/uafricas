<script setup lang="ts">
import { MOTIFS_SIGNALEMENT_MEDIA } from '~/composables/useMediaSocial'

const props = defineProps<{
  isOpen: boolean
  /** Titre du contenu visé, rappelé au membre avant qu'il ne confirme. */
  titre: string
}>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'submit', payload: { motif: string, description: string }): void
}>()

const motif = ref('')
const description = ref('')
const enCours = ref(false)
const erreur = ref('')
const succes = ref(false)
const messageSucces = ref('')

const MAX = 1000
const restant = computed(() => MAX - description.value.length)

watch(
  () => props.isOpen,
  (ouvert) => {
    if (ouvert) {
      motif.value = ''
      description.value = ''
      erreur.value = ''
      succes.value = false
      messageSucces.value = ''
      enCours.value = false
    }
  },
)

const fermer = () => {
  if (enCours.value) return
  emit('close')
}

const soumettre = () => {
  if (enCours.value) return
  if (!motif.value) {
    erreur.value = 'Veuillez sélectionner un motif.'
    return
  }
  erreur.value = ''
  emit('submit', { motif: motif.value, description: description.value.trim() })
}

// API impérative : le parent pilote les trois états depuis sa réponse serveur.
const setLoading = (v: boolean) => { enCours.value = v }
const setError = (msg: string) => { enCours.value = false; erreur.value = msg }
const setSuccess = (message: string) => {
  enCours.value = false
  succes.value = true
  messageSucces.value = message
  // Le message de suspension fait deux phrases : 1,8 s ne suffit pas à le lire,
  // et c'est précisément celui qui doit être lu.
  setTimeout(() => emit('close'), 4000)
}
defineExpose({ setLoading, setError, setSuccess })
</script>

<template>
  <Teleport to="body">
    <Transition name="modal-fade">
      <div
        v-if="isOpen"
        class="fixed inset-0 z-[95] flex items-center justify-center p-4"
        @click.self="fermer"
      >
        <div class="absolute inset-0 bg-black/60 backdrop-blur-sm"></div>
        <div class="relative bg-white rounded-2xl shadow-2xl w-full max-w-lg overflow-hidden">
          <div class="flex items-center justify-between px-6 py-4 border-b border-gray-100">
            <h3 class="font-oswald text-xl font-bold text-gray-900 flex items-center gap-2">
              <font-awesome-icon :icon="['fas', 'flag']" class="text-orange-500" />
              Signaler ce contenu
            </h3>
            <button
              type="button"
              :disabled="enCours"
              class="w-9 h-9 flex items-center justify-center rounded-full text-gray-400 hover:bg-gray-100 hover:text-gray-700 transition cursor-pointer"
              @click="fermer"
            >
              <font-awesome-icon :icon="['fas', 'xmark']" class="w-5 h-5" />
            </button>
          </div>

          <div class="px-6 py-5">
            <div v-if="succes" class="flex flex-col items-center justify-center py-8 text-center">
              <div class="w-14 h-14 rounded-full bg-orange-100 flex items-center justify-center mb-4">
                <font-awesome-icon :icon="['fas', 'check']" class="w-7 h-7 text-orange-500" />
              </div>
              <p class="font-medium text-gray-900">{{ messageSucces }}</p>
            </div>

            <template v-else>
              <p class="text-sm text-gray-600 mb-4">
                Vous signalez
                <span class="font-semibold text-gray-900">{{ titre }}</span>.
              </p>

              <label class="block text-sm font-medium text-gray-700 mb-1.5">Motif</label>
              <div class="space-y-1.5 mb-4">
                <label
                  v-for="m in MOTIFS_SIGNALEMENT_MEDIA"
                  :key="m.value"
                  class="flex items-center gap-2.5 px-3 py-2 rounded-lg border cursor-pointer transition"
                  :class="motif === m.value ? 'border-orange-400 bg-orange-50' : 'border-gray-200 hover:bg-gray-50'"
                >
                  <input v-model="motif" type="radio" :value="m.value" class="accent-orange-500" :disabled="enCours" />
                  <span class="text-sm text-gray-800">{{ m.label }}</span>
                </label>
              </div>

              <label class="block text-sm font-medium text-gray-700 mb-1.5">
                Précision <span class="text-gray-400 font-normal">(facultatif)</span>
              </label>
              <textarea
                v-model="description"
                rows="3"
                :maxlength="MAX"
                placeholder="Décrivez le problème (facultatif)…"
                class="w-full px-3.5 py-2.5 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-orange-400 focus:border-transparent resize-none"
                :disabled="enCours"
              ></textarea>
              <div class="flex items-center justify-between mt-1.5">
                <p v-if="erreur" class="text-sm text-red-600">{{ erreur }}</p>
                <span v-else></span>
                <span class="text-xs text-gray-400">{{ restant }}</span>
              </div>
            </template>
          </div>

          <div v-if="!succes" class="flex items-center justify-end gap-3 px-6 py-4 border-t border-gray-100 bg-gray-50">
            <button
              type="button"
              :disabled="enCours"
              class="px-4 py-2 text-sm font-medium text-gray-700 rounded-lg hover:bg-gray-200 transition cursor-pointer disabled:opacity-50"
              @click="fermer"
            >
              Annuler
            </button>
            <button
              type="button"
              :disabled="enCours"
              class="px-5 py-2 text-sm font-medium text-white bg-orange-500 rounded-lg hover:bg-orange-600 transition cursor-pointer disabled:opacity-60 inline-flex items-center gap-2"
              @click="soumettre"
            >
              <font-awesome-icon v-if="enCours" :icon="['fas', 'spinner']" class="w-4 h-4 animate-spin" />
              <font-awesome-icon v-else :icon="['fas', 'flag']" class="w-4 h-4" />
              Signaler
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.modal-fade-enter-active, .modal-fade-leave-active { transition: opacity 0.2s ease; }
.modal-fade-enter-from, .modal-fade-leave-to { opacity: 0; }
</style>
