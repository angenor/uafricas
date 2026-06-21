<script setup lang="ts">
const props = defineProps<{
  isOpen: boolean
  /** Titre/aperçu de la contribution partagée */
  titre: string
}>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'submit', legende: string): void
}>()

const legende = ref('')
const enCours = ref(false)
const erreur = ref('')
const succes = ref(false)

const MAX = 500
const restant = computed(() => MAX - legende.value.length)

watch(
  () => props.isOpen,
  (ouvert) => {
    if (ouvert) {
      legende.value = ''
      erreur.value = ''
      succes.value = false
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
  if (legende.value.length > MAX) {
    erreur.value = `La légende ne doit pas dépasser ${MAX} caractères.`
    return
  }
  erreur.value = ''
  emit('submit', legende.value.trim())
}

const setLoading = (v: boolean) => { enCours.value = v }
const setError = (msg: string) => { enCours.value = false; erreur.value = msg }
const setSuccess = () => {
  enCours.value = false
  succes.value = true
  setTimeout(() => emit('close'), 1400)
}
defineExpose({ setLoading, setError, setSuccess })
</script>

<template>
  <Teleport to="body">
    <Transition name="modal-fade">
      <div
        v-if="isOpen"
        class="fixed inset-0 z-[90] flex items-center justify-center p-4"
        @click.self="fermer"
      >
        <div class="absolute inset-0 bg-black/50 backdrop-blur-sm"></div>
        <div class="relative bg-white rounded-2xl shadow-2xl w-full max-w-lg overflow-hidden">
          <div class="flex items-center justify-between px-6 py-4 border-b border-gray-100">
            <h3 class="font-display text-xl font-bold text-gray-900">Partager cette publication</h3>
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
              <div class="w-14 h-14 rounded-full bg-custom-green/10 flex items-center justify-center mb-4">
                <font-awesome-icon :icon="['fas', 'check']" class="w-7 h-7 text-custom-green" />
              </div>
              <p class="font-medium text-gray-900">Publication partagée dans la communauté !</p>
              <p class="text-sm text-gray-500 mt-1">Votre partage apparaît désormais sur la page Publications.</p>
            </div>

            <template v-else>
              <p class="text-sm text-gray-600 mb-4">
                Vous partagez
                <span class="font-semibold text-gray-900">« {{ titre }} »</span>
                sur le mur communautaire. Ajoutez un mot (facultatif).
              </p>

              <label class="block text-sm font-medium text-gray-700 mb-1.5">
                Légende <span class="text-gray-400 font-normal">(facultatif)</span>
              </label>
              <textarea
                v-model="legende"
                rows="4"
                :maxlength="MAX"
                placeholder="Ex. : Une publication à découvrir absolument !"
                class="w-full px-3.5 py-2.5 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-custom-green focus:border-transparent resize-none"
                :disabled="enCours"
              ></textarea>
              <div class="flex items-center justify-between mt-1.5">
                <p v-if="erreur" class="text-sm text-red-600">{{ erreur }}</p>
                <span v-else></span>
                <span class="text-xs" :class="restant < 0 ? 'text-red-600' : 'text-gray-400'">{{ restant }}</span>
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
              class="px-5 py-2 text-sm font-medium text-white bg-custom-green rounded-lg hover:bg-custom-green/90 transition cursor-pointer disabled:opacity-60 inline-flex items-center gap-2"
              @click="soumettre"
            >
              <font-awesome-icon v-if="enCours" :icon="['fas', 'spinner']" class="w-4 h-4 animate-spin" />
              <font-awesome-icon v-else :icon="['fas', 'share-nodes']" class="w-4 h-4" />
              Partager
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
