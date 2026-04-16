<script setup lang="ts">
import { computed, reactive, watch } from 'vue'

interface Props {
  isOpen: boolean
  salleId: string
}

const props = defineProps<Props>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'submit', data: { titre: string; description: string; code_acces: string }): void
  (e: 'existante', sallePriveeId?: string): void
}>()

const CODE_PATTERN = /^[A-Za-z0-9!@#$%&*?-]{4,16}$/

const form = reactive({
  titre: '',
  description: '',
  code_acces: '',
  loading: false,
  submitted: false,
  error: false,
  errorMessage: '',
})

const isFormValid = computed(() => {
  const titreOk = form.titre.trim().length >= 5 && form.titre.trim().length <= 350
  const codeOk = CODE_PATTERN.test(form.code_acces)
  const descOk = form.description.length <= 1000
  return titreOk && codeOk && descOk
})

const resetForm = () => {
  form.titre = ''
  form.description = ''
  form.code_acces = ''
  form.loading = false
  form.submitted = false
  form.error = false
  form.errorMessage = ''
}

const handleSubmit = () => {
  form.error = false
  form.errorMessage = ''

  const titre = form.titre.trim()
  if (titre.length < 5 || titre.length > 350) {
    form.error = true
    form.errorMessage = 'Le titre doit contenir entre 5 et 350 caractères.'
    return
  }
  if (form.description.length > 1000) {
    form.error = true
    form.errorMessage = 'La description ne peut dépasser 1000 caractères.'
    return
  }
  if (!CODE_PATTERN.test(form.code_acces)) {
    form.error = true
    form.errorMessage = 'Le code secret doit contenir 4 à 16 caractères (lettres, chiffres ou !@#$%&*?-).'
    return
  }

  emit('submit', {
    titre,
    description: form.description.trim(),
    code_acces: form.code_acces,
  })
}

defineExpose({
  setLoading: (val: boolean) => {
    form.loading = val
  },
  setError: (msg: string) => {
    form.error = true
    form.errorMessage = msg
    form.loading = false
  },
  setSuccess: () => {
    form.submitted = true
    form.loading = false
    setTimeout(() => {
      resetForm()
      emit('close')
    }, 1200)
  },
  setExistante: (sallePriveeId?: string) => {
    form.loading = false
    form.error = true
    form.errorMessage = 'Vous avez déjà une salle privée pour cette salle publique.'
    emit('existante', sallePriveeId)
  },
})

watch(
  () => props.isOpen,
  (isOpen) => {
    if (!isOpen) resetForm()
  },
)
</script>

<template>
  <Transition name="modal-fade">
    <div
      v-if="isOpen"
      class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/75 backdrop-blur-xs"
      @click.self="emit('close')"
    >
      <div
        class="relative w-full max-w-lg bg-white shadow-2xl rounded-2xl border-t-4 border-custom-chocolat transition-all duration-300 max-h-[92vh] overflow-hidden"
        @click.stop
      >
        <!-- En-tête -->
        <div class="bg-gradient-to-r from-custom-chocolat to-custom-chocolat/80 text-white p-6">
          <div class="flex items-center justify-between">
            <div>
              <h2 class="text-2xl font-bold">Créer ma salle privée</h2>
              <p class="text-white/80 text-sm mt-1">
                Un espace à votre initiative, accessible par un code secret.
              </p>
            </div>
            <button
              type="button"
              class="text-white/80 hover:text-white transition-colors"
              @click="emit('close')"
            >
              <font-awesome-icon :icon="['fas', 'xmark']" class="w-6 h-6" />
            </button>
          </div>
        </div>

        <!-- Formulaire -->
        <form
          class="p-6 space-y-5 bg-white max-h-[72vh] overflow-y-auto"
          @submit.prevent="handleSubmit"
        >
          <!-- Message de succès -->
          <Transition name="fade-slide">
            <div
              v-if="form.submitted"
              class="bg-green-50 border-l-4 border-green-500 p-4 rounded-lg"
            >
              <div class="flex items-center">
                <font-awesome-icon
                  :icon="['fas', 'circle-check']"
                  class="w-6 h-6 text-green-500 mr-3"
                />
                <p class="text-green-700 font-medium">Salle privée créée avec succès !</p>
              </div>
            </div>
          </Transition>

          <!-- Message d'erreur -->
          <Transition name="fade-slide">
            <div
              v-if="form.error"
              class="bg-red-50 border-l-4 border-red-500 p-4 rounded-lg"
            >
              <div class="flex items-start gap-3">
                <font-awesome-icon
                  :icon="['fas', 'circle-exclamation']"
                  class="w-6 h-6 text-red-500 mt-0.5 shrink-0"
                />
                <p class="text-red-700 text-sm">{{ form.errorMessage }}</p>
              </div>
            </div>
          </Transition>

          <!-- Titre -->
          <div>
            <label class="block text-sm font-semibold text-gray-700 mb-2">
              Titre de la salle <span class="text-red-500">*</span>
            </label>
            <input
              v-model="form.titre"
              type="text"
              minlength="5"
              maxlength="350"
              class="w-full border-2 rounded-lg p-3 border-gray-200 focus:outline-hidden focus:border-custom-chocolat transition-colors"
              placeholder="Ex : Mon cercle Wolof du soir"
              required
            >
            <p class="text-xs text-gray-400 mt-1">
              Entre 5 et 350 caractères.
            </p>
          </div>

          <!-- Code secret -->
          <div>
            <label class="block text-sm font-semibold text-gray-700 mb-2">
              <font-awesome-icon :icon="['fas', 'lock']" class="w-4 h-4 mr-1 text-gray-400" />
              Code secret <span class="text-red-500">*</span>
            </label>
            <input
              v-model="form.code_acces"
              type="text"
              pattern="^[A-Za-z0-9!@#$%&*?-]{4,16}$"
              autocomplete="off"
              class="w-full border-2 rounded-lg p-3 border-gray-200 focus:outline-hidden focus:border-custom-chocolat transition-colors font-mono tracking-wider"
              placeholder="wolof2026"
              required
            >
            <p class="text-xs text-gray-400 mt-1">
              4 à 16 caractères (lettres, chiffres ou <code>!@#$%&amp;*?-</code>).
              Notez-le soigneusement : il n'est plus jamais affiché.
            </p>
          </div>

          <!-- Description -->
          <div>
            <label class="block text-sm font-semibold text-gray-700 mb-2">
              Description (facultative)
            </label>
            <textarea
              v-model="form.description"
              rows="3"
              maxlength="1000"
              class="w-full border-2 rounded-lg p-3 border-gray-200 focus:outline-hidden focus:border-custom-chocolat transition-colors resize-none"
              placeholder="À qui s'adresse cette salle, quand se retrouve-t-on…"
            />
            <p class="text-xs text-gray-400 mt-1">
              {{ form.description.length }} / 1000
            </p>
          </div>

          <!-- Boutons -->
          <div class="flex gap-3 pt-2">
            <button
              type="button"
              class="flex-1 p-3 bg-gray-100 text-gray-700 rounded-xl font-medium hover:bg-gray-200 transition-all"
              @click="emit('close')"
            >
              Annuler
            </button>
            <button
              type="submit"
              :disabled="!isFormValid || form.loading"
              class="flex-1 p-3 bg-custom-chocolat text-white rounded-xl font-medium hover:bg-custom-chocolat/90 transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
            >
              <font-awesome-icon
                v-if="form.loading"
                :icon="['fas', 'spinner']"
                class="w-4 h-4 animate-spin"
              />
              {{ form.loading ? 'Création...' : 'Créer la salle privée' }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </Transition>
</template>
