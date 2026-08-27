<template>
  <AfricansModale
    :model-value="isOpen"
    titre="Rejoindre la salle privée"
    :sous-titre="sallePriveeTitre"
    icone="fa-solid fa-lock"
    @update:model-value="!$event && emit('close')"
  >
    <form class="flex flex-col gap-5" @submit.prevent="handleSubmit">
      <p class="text-[14px]/[1.5] text-af-corps">
        Cette salle est protégée par un code secret, communiqué par la personne
        qui l'anime.
      </p>

      <div class="flex flex-col gap-2">
        <label for="code-acces" class="text-[14px]/[1.4] text-af-atone italic">Code secret *</label>
        <input
          id="code-acces"
          v-model="codeAcces"
          type="text"
          autocomplete="off"
          :disabled="loading"
          placeholder="Saisissez le code"
          class="h-11 rounded-md border bg-white px-4 text-center text-[16px]/[1.4] tracking-widest placeholder:tracking-normal placeholder:text-af-atone-2 focus:outline-none disabled:opacity-50"
          :class="error ? 'border-af-live' : 'border-af-bordure focus:border-af-chocolat'"
        />
        <p v-if="error" role="alert" class="flex items-start gap-2 text-[12px]/[1.4] text-af-live">
          <font-awesome-icon icon="fa-solid fa-circle-exclamation" class="mt-0.5 shrink-0" />
          {{ errorMessage }}
        </p>
      </div>
    </form>

    <template #actions>
      <button
        type="button"
        :disabled="loading"
        class="text-base font-bold text-af-corps transition hover:opacity-70 disabled:opacity-50"
        @click="emit('close')"
      >
        Annuler
      </button>
      <AfricansBouton
        :desactive="loading"
        :tourne="loading"
        :icone="loading ? 'fa-solid fa-spinner' : 'fa-solid fa-right-to-bracket'"
        @click="handleSubmit"
      >
        {{ loading ? 'Vérification…' : 'Rejoindre' }}
      </AfricansBouton>
    </template>
  </AfricansModale>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'

interface Props {
  isOpen: boolean
  sallePriveeTitre?: string
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

  const code = codeAcces.value.trim()
  if (!code) {
    error.value = true
    errorMessage.value = 'Veuillez saisir le code secret.'
    return
  }

  emit('submit', code)
}

const resetForm = () => {
  codeAcces.value = ''
  loading.value = false
  error.value = false
  errorMessage.value = ''
}

defineExpose({
  setLoading: (val: boolean) => {
    loading.value = val
  },
  setError: (msg: string) => {
    error.value = true
    errorMessage.value = msg
    loading.value = false
  },
  setSuccess: () => {
    loading.value = false
    resetForm()
    emit('close')
  },
})

watch(
  () => props.isOpen,
  (isOpen) => {
    if (!isOpen) resetForm()
  },
)
</script>
