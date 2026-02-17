<script setup lang="ts">
import { SECTIONS_FICHE_PAYS } from '~/composables/useOpportuniteAfrique'

interface Props {
  isOpen: boolean
  ficheId: string
  paysNom: string
}

const props = defineProps<Props>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'submit', data: {
    section: string
    type_contribution: string
    nouvelle_valeur: string
    justification: string
  }): void
}>()

const form = reactive({
  section: '',
  type_contribution: 'modification',
  nouvelle_valeur: '',
  justification: '',
  loading: false,
  submitted: false,
  error: false,
  errorMessage: '',
})

const sections = SECTIONS_FICHE_PAYS

const isFormValid = computed(() => {
  return form.section !== '' && form.nouvelle_valeur.trim().length >= 3
})

const sectionLabel = computed(() => {
  const found = sections.find(s => s.value === form.section)
  return found ? found.label : ''
})

const resetForm = () => {
  form.section = ''
  form.type_contribution = 'modification'
  form.nouvelle_valeur = ''
  form.justification = ''
  form.loading = false
  form.submitted = false
  form.error = false
  form.errorMessage = ''
}

const handleSubmit = async () => {
  form.error = false
  form.errorMessage = ''

  if (!form.section) {
    form.error = true
    form.errorMessage = 'Veuillez selectionner une section.'
    return
  }

  if (form.nouvelle_valeur.trim().length < 3) {
    form.error = true
    form.errorMessage = 'La valeur proposee doit contenir au moins 3 caracteres.'
    return
  }

  emit('submit', {
    section: form.section,
    type_contribution: form.type_contribution,
    nouvelle_valeur: form.nouvelle_valeur.trim(),
    justification: form.justification.trim(),
  })
}

// Exposer pour que le parent puisse piloter l'etat
defineExpose({
  setLoading: (val: boolean) => { form.loading = val },
  setError: (msg: string) => { form.error = true; form.errorMessage = msg; form.loading = false },
  setSuccess: () => {
    form.submitted = true
    form.loading = false
    setTimeout(() => {
      resetForm()
      emit('close')
    }, 2500)
  },
})

watch(() => props.isOpen, (isOpen) => {
  if (!isOpen) {
    resetForm()
  }
})
</script>

<template>
  <Transition name="modal-fade">
    <div
      v-if="isOpen"
      class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/75 backdrop-blur-xs"
      @click.self="emit('close')"
    >
      <div
        class="relative w-full max-w-2xl bg-white/95 shadow-2xl backdrop-blur-md rounded-2xl border border-white/20 transform transition-all duration-300 max-h-[90vh] overflow-hidden"
        @click.stop
      >
        <!-- En-tete -->
        <div class="bg-gradient-to-r from-custom-chocolat to-custom-green text-white p-6">
          <div class="flex items-center justify-between">
            <div>
              <h2 class="text-2xl font-bold">Proposer une contribution</h2>
              <p class="text-white/80 text-sm mt-1">Contribuez a la fiche de {{ paysNom }}</p>
            </div>
            <button @click="emit('close')" class="text-white hover:text-white/60 transition-colors">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>
        </div>

        <!-- Formulaire -->
        <form @submit.prevent="handleSubmit" class="p-6 space-y-6 bg-white/90 backdrop-blur-xs max-h-[70vh] overflow-y-auto">
          <!-- Message de succes -->
          <Transition name="fade-slide">
            <div v-if="form.submitted" class="bg-green-50 border-l-4 border-green-500 p-4 rounded-lg">
              <div class="flex items-center">
                <svg class="w-6 h-6 text-green-500 mr-3 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                <div>
                  <p class="text-green-700 font-medium">Contribution soumise avec succes !</p>
                  <p class="text-green-600 text-sm mt-1">Elle sera examinee par un administrateur avant publication.</p>
                </div>
              </div>
            </div>
          </Transition>

          <!-- Message d'erreur -->
          <Transition name="fade-slide">
            <div v-if="form.error" class="bg-red-50 border-l-4 border-red-500 p-4 rounded-lg">
              <div class="flex items-center">
                <svg class="w-6 h-6 text-red-500 mr-3 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                </svg>
                <p class="text-red-700">{{ form.errorMessage }}</p>
              </div>
            </div>
          </Transition>

          <!-- Section: Choix de la section et du type -->
          <div class="space-y-4 bg-gray-50 p-4 rounded-xl">
            <h3 class="text-lg font-semibold text-gray-800 mb-4">
              <svg class="w-4 h-4 text-custom-chocolat mr-2 inline" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
              </svg>
              Informations de la contribution
            </h3>

            <!-- Section a modifier -->
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">Section concernee *</label>
              <select
                v-model="form.section"
                required
                class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-custom-chocolat focus:border-transparent transition-all"
              >
                <option value="">Selectionnez une section</option>
                <option v-for="s in sections" :key="s.value" :value="s.value">
                  {{ s.label }}
                </option>
              </select>
            </div>

            <!-- Type de contribution -->
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-3">Type de contribution *</label>
              <div class="flex flex-wrap gap-3">
                <label class="flex items-center space-x-2 cursor-pointer">
                  <input
                    v-model="form.type_contribution"
                    type="radio"
                    value="modification"
                    class="radio radio-sm radio-warning"
                  />
                  <span class="text-sm font-medium text-gray-700">Modification</span>
                </label>
                <label class="flex items-center space-x-2 cursor-pointer">
                  <input
                    v-model="form.type_contribution"
                    type="radio"
                    value="ajout"
                    class="radio radio-sm radio-success"
                  />
                  <span class="text-sm font-medium text-gray-700">Ajout</span>
                </label>
                <label class="flex items-center space-x-2 cursor-pointer">
                  <input
                    v-model="form.type_contribution"
                    type="radio"
                    value="suppression"
                    class="radio radio-sm radio-error"
                  />
                  <span class="text-sm font-medium text-gray-700">Suppression</span>
                </label>
              </div>
            </div>
          </div>

          <!-- Section: Contenu de la contribution -->
          <div class="space-y-4 bg-gray-50 p-4 rounded-xl">
            <h3 class="text-lg font-semibold text-gray-800 mb-4">
              <svg class="w-4 h-4 text-custom-chocolat mr-2 inline" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
              </svg>
              Contenu propose
            </h3>

            <!-- Nouvelle valeur -->
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">
                <template v-if="form.type_contribution === 'modification'">Nouvelle valeur proposee *</template>
                <template v-else-if="form.type_contribution === 'ajout'">Valeur a ajouter *</template>
                <template v-else>Raison de la suppression *</template>
              </label>
              <textarea
                v-model="form.nouvelle_valeur"
                rows="4"
                required
                class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-custom-chocolat focus:border-transparent transition-all resize-y"
                :placeholder="form.type_contribution === 'suppression'
                  ? 'Expliquez pourquoi cette information devrait etre supprimee...'
                  : form.section
                    ? `Entrez la nouvelle valeur pour ${sectionLabel}...`
                    : 'Selectionnez d\'abord une section ci-dessus'"
              ></textarea>
              <p class="text-xs text-gray-400 mt-1">{{ form.nouvelle_valeur.length }} caracteres (minimum 3)</p>
            </div>

            <!-- Justification -->
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">Justification (optionnel)</label>
              <textarea
                v-model="form.justification"
                rows="2"
                class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-custom-chocolat focus:border-transparent transition-all resize-y"
                placeholder="Expliquez pourquoi cette modification est necessaire, source d'information..."
              ></textarea>
            </div>
          </div>

          <!-- Boutons d'action -->
          <div class="flex space-x-4 pt-4">
            <button
              type="button"
              @click="emit('close')"
              class="flex-1 bg-gray-200 hover:bg-gray-300 text-gray-800 font-medium py-3 px-6 rounded-lg transition-all"
            >
              Annuler
            </button>
            <button
              type="submit"
              :disabled="form.loading || !isFormValid"
              class="flex-1 bg-gradient-to-r from-custom-chocolat to-custom-green hover:from-custom-chocolat/90 hover:to-custom-green/90 text-white font-medium py-3 px-6 rounded-lg transition-all disabled:opacity-50 disabled:cursor-not-allowed shadow-lg hover:shadow-xl"
            >
              <span v-if="form.loading" class="flex items-center justify-center">
                <svg class="w-5 h-5 animate-spin mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                </svg>
                Envoi en cours...
              </span>
              <span v-else class="flex items-center justify-center">
                <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" />
                </svg>
                Soumettre la contribution
              </span>
            </button>
          </div>
        </form>
      </div>
    </div>
  </Transition>
</template>

<style scoped>
.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: opacity 0.3s ease;
}

.modal-fade-enter-from,
.modal-fade-leave-to {
  opacity: 0;
}

.fade-slide-enter-active,
.fade-slide-leave-active {
  transition: all 0.3s ease;
}

.fade-slide-enter-from,
.fade-slide-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}
</style>
