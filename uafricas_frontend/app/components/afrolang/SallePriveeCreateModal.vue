<script setup lang="ts">
interface Props {
  isOpen: boolean
  salleId: string
}

const props = defineProps<Props>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'submit', data: {
    titre: string
    description: string
    code_acces: string
    max_participants: number | null
  }): void
}>()

const form = reactive({
  titre: '',
  description: '',
  code_acces: '',
  max_participants: null as number | null,
  loading: false,
  submitted: false,
  error: false,
  errorMessage: '',
})

const isFormValid = computed(() => {
  return form.titre.trim().length >= 3
})

const resetForm = () => {
  form.titre = ''
  form.description = ''
  form.code_acces = ''
  form.max_participants = null
  form.loading = false
  form.submitted = false
  form.error = false
  form.errorMessage = ''
}

const handleSubmit = async () => {
  form.error = false
  form.errorMessage = ''

  if (!form.titre.trim()) {
    form.error = true
    form.errorMessage = 'Le titre est requis.'
    return
  }

  if (form.titre.trim().length < 3) {
    form.error = true
    form.errorMessage = 'Le titre doit contenir au moins 3 caractères.'
    return
  }

  emit('submit', {
    titre: form.titre.trim(),
    description: form.description.trim(),
    code_acces: form.code_acces.trim(),
    max_participants: form.max_participants,
  })
}

defineExpose({
  setLoading: (val: boolean) => { form.loading = val },
  setError: (msg: string) => { form.error = true; form.errorMessage = msg; form.loading = false },
  setSuccess: () => {
    form.submitted = true
    form.loading = false
    setTimeout(() => {
      resetForm()
      emit('close')
    }, 2000)
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
        class="relative w-full max-w-lg bg-white shadow-2xl rounded-2xl border-t-4 border-blue-500 transform transition-all duration-300 max-h-[90vh] overflow-hidden"
        @click.stop
      >
        <!-- En-tete -->
        <div class="bg-gradient-to-r from-blue-500 to-cyan-500 text-white p-6">
          <div class="flex items-center justify-between">
            <div>
              <h2 class="text-2xl font-bold">Créer un cours privé</h2>
              <p class="text-blue-100 text-sm mt-1">Créez votre espace de cours privé</p>
            </div>
            <button @click="emit('close')" class="text-white hover:text-blue-200 transition-colors">
              <font-awesome-icon :icon="['fas', 'xmark']" class="w-6 h-6" />
            </button>
          </div>
        </div>

        <!-- Formulaire -->
        <form @submit.prevent="handleSubmit" class="p-6 space-y-5 bg-white max-h-[70vh] overflow-y-auto">
          <!-- Message de succes -->
          <Transition name="fade-slide">
            <div v-if="form.submitted" class="bg-green-50 border-l-4 border-green-500 p-4 rounded-lg">
              <div class="flex items-center">
                <font-awesome-icon :icon="['fas', 'circle-check']" class="w-6 h-6 text-green-500 mr-3" />
                <p class="text-green-700 font-medium">Cours privé créé avec succès !</p>
              </div>
            </div>
          </Transition>

          <!-- Message d'erreur -->
          <Transition name="fade-slide">
            <div v-if="form.error" class="bg-red-50 border-l-4 border-red-500 p-4 rounded-lg">
              <div class="flex items-center">
                <font-awesome-icon :icon="['fas', 'circle-exclamation']" class="w-6 h-6 text-red-500 mr-3" />
                <p class="text-red-700">{{ form.errorMessage }}</p>
              </div>
            </div>
          </Transition>

          <!-- Titre -->
          <div>
            <label class="block text-sm font-semibold text-gray-700 mb-2">
              Titre du cours *
            </label>
            <input
              v-model="form.titre"
              type="text"
              class="w-full border-2 rounded-lg p-3 border-gray-200 focus:outline-hidden focus:border-blue-500 transition-colors"
              placeholder="Ex: Cours de Wolof débutant"
            />
          </div>

          <!-- Description -->
          <div>
            <label class="block text-sm font-semibold text-gray-700 mb-2">
              Description
            </label>
            <textarea
              v-model="form.description"
              rows="3"
              class="w-full border-2 rounded-lg p-3 border-gray-200 focus:outline-hidden focus:border-blue-500 transition-colors resize-none"
              placeholder="Décrivez votre cours..."
            />
          </div>

          <!-- Code d'acces -->
          <div>
            <label class="block text-sm font-semibold text-gray-700 mb-2">
              <font-awesome-icon :icon="['fas', 'lock']" class="w-4 h-4 mr-1 text-gray-400" />
              Code d'accès (optionnel)
            </label>
            <input
              v-model="form.code_acces"
              type="text"
              class="w-full border-2 rounded-lg p-3 border-gray-200 focus:outline-hidden focus:border-blue-500 transition-colors"
              placeholder="Laissez vide pour un cours ouvert"
            />
            <p class="text-xs text-gray-400 mt-1">Si défini, les participants devront saisir ce code pour rejoindre les sessions.</p>
          </div>

          <!-- Max participants -->
          <div>
            <label class="block text-sm font-semibold text-gray-700 mb-2">
              Nombre max de participants
            </label>
            <input
              v-model.number="form.max_participants"
              type="number"
              min="2"
              max="100"
              class="w-full border-2 rounded-lg p-3 border-gray-200 focus:outline-hidden focus:border-blue-500 transition-colors"
              placeholder="Illimité par défaut"
            />
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
              class="flex-1 p-3 bg-gradient-to-r from-blue-500 to-cyan-500 text-white rounded-xl font-medium hover:shadow-lg transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
            >
              <font-awesome-icon v-if="form.loading" :icon="['fas', 'spinner']" class="w-4 h-4 animate-spin" />
              {{ form.loading ? 'Création...' : 'Créer le cours' }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </Transition>
</template>
