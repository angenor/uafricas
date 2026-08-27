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
  <!-- `couche="session"` : cette modale s'ouvre aussi depuis la salle en plein
       écran (`AfrolangRoom`, z-10000), qu'elle doit recouvrir. -->
  <AfricansModale
    :model-value="isOpen"
    titre="Créer ma salle privée"
    sous-titre="Un espace à votre initiative, accessible par un code secret."
    icone="fa-solid fa-lock"
    ton="chocolat"
    couche="session"
    @update:model-value="emit('close')"
  >
    <form id="form-salle-privee" class="flex flex-col gap-5" @submit.prevent="handleSubmit">
      <p
        v-if="form.submitted"
        class="flex items-center gap-3 rounded-lg border border-af-vert/20 bg-af-vert/5 px-4 py-3 text-[14px]/[1.4] font-bold text-af-vert"
      >
        <font-awesome-icon icon="fa-solid fa-circle-check" />
        Salle privée créée avec succès !
      </p>

      <p
        v-if="form.error"
        class="flex items-start gap-3 rounded-lg border border-af-live/20 bg-af-live/5 px-4 py-3 text-[14px]/[1.4] text-af-live"
      >
        <font-awesome-icon icon="fa-solid fa-circle-exclamation" class="mt-0.5 shrink-0" />
        {{ form.errorMessage }}
      </p>

      <AfricansChamp
        v-model="form.titre"
        libelle="Titre de la salle"
        :maxlength="350"
        placeholder="Ex : Mon cercle Wolof du soir"
        aide="Entre 5 et 350 caractères."
        obligatoire
      />

      <div>
        <AfricansChamp
          v-model="form.code_acces"
          libelle="Code secret"
          icone="fa-solid fa-lock"
          :maxlength="16"
          placeholder="wolof2026"
          autocomplete="off"
          obligatoire
        />
        <p class="mt-2 text-[12px]/[1.4] text-af-atone">
          4 à 16 caractères (lettres, chiffres ou <code>!@#$%&amp;*?-</code>).
          Notez-le soigneusement : il n'est plus jamais affiché.
        </p>
      </div>

      <div>
        <AfricansChamp
          v-model="form.description"
          libelle="Description"
          type="textarea"
          :lignes="3"
          :maxlength="1000"
          placeholder="À qui s'adresse cette salle, quand se retrouve-t-on…"
          aide="Facultative"
        />
        <p class="mt-1 text-right text-[12px] text-af-atone-2">{{ form.description.length }} / 1000</p>
      </div>
    </form>

    <template #actions>
      <button
        type="button"
        class="text-base font-bold text-af-corps transition hover:opacity-70"
        @click="emit('close')"
      >
        Annuler
      </button>
      <AfricansBouton
        type="submit"
        form="form-salle-privee"
        :desactive="!isFormValid || form.loading"
        :tourne="form.loading"
        :icone="form.loading ? 'fa-solid fa-spinner' : 'fa-solid fa-lock'"
      >
        {{ form.loading ? 'Création…' : 'Créer la salle privée' }}
      </AfricansBouton>
    </template>
  </AfricansModale>
</template>
