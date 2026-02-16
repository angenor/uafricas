<script setup lang="ts">
interface Props {
  isOpen: boolean
  sessionId: string
}

const props = defineProps<Props>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'submit', codeAcces: string): void
}>()

const codeAcces = ref('')
const loading = ref(false)
const error = ref(false)
const errorMessage = ref('')

const handleSubmit = () => {
  error.value = false
  errorMessage.value = ''

  if (!codeAcces.value.trim()) {
    error.value = true
    errorMessage.value = 'Veuillez saisir le code d\'accès.'
    return
  }

  emit('submit', codeAcces.value.trim())
}

const resetForm = () => {
  codeAcces.value = ''
  loading.value = false
  error.value = false
  errorMessage.value = ''
}

defineExpose({
  setLoading: (val: boolean) => { loading.value = val },
  setError: (msg: string) => { error.value = true; errorMessage.value = msg; loading.value = false },
  setSuccess: () => {
    loading.value = false
    resetForm()
    emit('close')
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
        class="relative w-full max-w-md bg-white shadow-2xl rounded-2xl border-t-4 border-amber-500 transform transition-all duration-300"
        @click.stop
      >
        <!-- En-tete -->
        <div class="bg-gradient-to-r from-amber-500 to-orange-500 text-white p-6">
          <div class="flex items-center justify-between">
            <div>
              <h2 class="text-xl font-bold">Rejoindre la session</h2>
              <p class="text-amber-100 text-sm mt-1">Cette salle est protégée par un code</p>
            </div>
            <button @click="emit('close')" class="text-white hover:text-amber-200 transition-colors">
              <font-awesome-icon :icon="['fas', 'xmark']" class="w-6 h-6" />
            </button>
          </div>
        </div>

        <!-- Formulaire -->
        <form @submit.prevent="handleSubmit" class="p-6 space-y-5">
          <!-- Message d'erreur -->
          <Transition name="fade-slide">
            <div v-if="error" class="bg-red-50 border-l-4 border-red-500 p-4 rounded-lg">
              <div class="flex items-center">
                <font-awesome-icon :icon="['fas', 'circle-exclamation']" class="w-5 h-5 text-red-500 mr-3" />
                <p class="text-red-700 text-sm">{{ errorMessage }}</p>
              </div>
            </div>
          </Transition>

          <!-- Icone cadenas -->
          <div class="text-center">
            <div class="w-16 h-16 bg-amber-100 rounded-full flex items-center justify-center mx-auto mb-4">
              <font-awesome-icon :icon="['fas', 'lock']" class="w-8 h-8 text-amber-600" />
            </div>
          </div>

          <!-- Code d'acces -->
          <div>
            <label class="block text-sm font-semibold text-gray-700 mb-2">
              Code d'accès
            </label>
            <input
              v-model="codeAcces"
              type="text"
              class="w-full border-2 rounded-lg p-3 border-gray-200 focus:outline-hidden focus:border-amber-500 transition-colors text-center text-lg tracking-wider"
              placeholder="Saisissez le code"
              autofocus
            />
          </div>

          <!-- Boutons -->
          <div class="flex gap-3">
            <button
              type="button"
              class="flex-1 p-3 bg-gray-100 text-gray-700 rounded-xl font-medium hover:bg-gray-200 transition-all"
              @click="emit('close')"
            >
              Annuler
            </button>
            <button
              type="submit"
              :disabled="!codeAcces.trim() || loading"
              class="flex-1 p-3 bg-gradient-to-r from-amber-500 to-orange-500 text-white rounded-xl font-medium hover:shadow-lg transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
            >
              <font-awesome-icon v-if="loading" :icon="['fas', 'spinner']" class="w-4 h-4 animate-spin" />
              {{ loading ? 'Vérification...' : 'Rejoindre' }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </Transition>
</template>
