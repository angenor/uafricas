<script setup lang="ts">
import { computed, reactive, watch } from 'vue'
import type { MotifSallePrivee, VisibiliteSallePrivee } from '~/composables/useAfrolang'
import { MOTIFS_SALLE_PRIVEE } from '~/composables/useAfrolang'

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
    motif: MotifSallePrivee
    declaration_adulte: boolean
    visibilite: VisibiliteSallePrivee
  }): void
}>()

const form = reactive({
  titre: '',
  description: '',
  code_acces: '',
  max_participants: null as number | null,
  motif: 'reseautage_adulte' as MotifSallePrivee,
  declaration_adulte: false,
  visibilite: 'fermee' as VisibiliteSallePrivee,
  loading: false,
  submitted: false,
  error: false,
  errorMessage: '',
})

const isFormValid = computed(() => {
  return (
    form.titre.trim().length >= 3 &&
    form.declaration_adulte === true
  )
})

const motifInfo = computed(() => {
  switch (form.motif) {
    case 'apprentissage_enfants':
      return {
        icon: 'children' as const,
        label: 'Apprentissage par les enfants',
        description:
          'Cet espace accueille des enfants qui apprennent la langue.',
        warning:
          'Un adulte responsable doit être présent auprès des enfants pendant chaque session.',
      }
    case 'reseautage_adulte':
      return {
        icon: 'handshake' as const,
        label: 'Réseautage entre adultes',
        description: 'Échanges professionnels et sociaux entre adultes.',
        warning: null,
      }
    case 'echanges_groupe':
      return {
        icon: 'users' as const,
        label: 'Échanges de groupe',
        description: 'Salle dédiée à la pratique collective.',
        warning: null,
      }
  }
})

const resetForm = () => {
  form.titre = ''
  form.description = ''
  form.code_acces = ''
  form.max_participants = null
  form.motif = 'reseautage_adulte'
  form.declaration_adulte = false
  form.visibilite = 'fermee'
  form.loading = false
  form.submitted = false
  form.error = false
  form.errorMessage = ''
}

const handleSubmit = async () => {
  form.error = false
  form.errorMessage = ''

  if (!form.titre.trim() || form.titre.trim().length < 3) {
    form.error = true
    form.errorMessage = 'Le titre doit contenir au moins 3 caractères.'
    return
  }

  if (!form.declaration_adulte) {
    form.error = true
    form.errorMessage =
      'Vous devez cocher la déclaration de majorité pour créer la salle privée.'
    return
  }

  emit('submit', {
    titre: form.titre.trim(),
    description: form.description.trim(),
    code_acces: form.code_acces.trim(),
    max_participants: form.max_participants,
    motif: form.motif,
    declaration_adulte: form.declaration_adulte,
    visibilite: form.visibilite,
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
    }, 1500)
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
        class="relative w-full max-w-lg bg-white shadow-2xl rounded-2xl border-t-4 border-custom-chocolat transform transition-all duration-300 max-h-[92vh] overflow-hidden"
        @click.stop
      >
        <!-- En-tete -->
        <div class="bg-gradient-to-r from-custom-chocolat to-custom-chocolat/80 text-white p-6">
          <div class="flex items-center justify-between">
            <div>
              <h2 class="text-2xl font-bold">Créer une salle privée</h2>
              <p class="text-white/80 text-sm mt-1">
                Un espace à votre initiative, dédié à un motif précis.
              </p>
            </div>
            <button @click="emit('close')" class="text-white/80 hover:text-white transition-colors">
              <font-awesome-icon :icon="['fas', 'xmark']" class="w-6 h-6" />
            </button>
          </div>
        </div>

        <!-- Formulaire -->
        <form @submit.prevent="handleSubmit" class="p-6 space-y-5 bg-white max-h-[72vh] overflow-y-auto">
          <!-- Message de succes -->
          <Transition name="fade-slide">
            <div v-if="form.submitted" class="bg-green-50 border-l-4 border-green-500 p-4 rounded-lg">
              <div class="flex items-center">
                <font-awesome-icon :icon="['fas', 'circle-check']" class="w-6 h-6 text-green-500 mr-3" />
                <p class="text-green-700 font-medium">Salle privée créée avec succès !</p>
              </div>
            </div>
          </Transition>

          <!-- Message d'erreur -->
          <Transition name="fade-slide">
            <div v-if="form.error" class="bg-red-50 border-l-4 border-red-500 p-4 rounded-lg">
              <div class="flex items-start gap-3">
                <font-awesome-icon :icon="['fas', 'circle-exclamation']" class="w-6 h-6 text-red-500 mt-0.5 shrink-0" />
                <p class="text-red-700 text-sm">{{ form.errorMessage }}</p>
              </div>
            </div>
          </Transition>

          <!-- Motif -->
          <div>
            <label class="block text-sm font-semibold text-gray-700 mb-2">
              Motif de la salle <span class="text-red-500">*</span>
            </label>
            <div class="grid grid-cols-1 sm:grid-cols-3 gap-2">
              <button
                v-for="m in MOTIFS_SALLE_PRIVEE"
                :key="m.value"
                type="button"
                class="p-3 rounded-lg border-2 text-sm font-medium text-left transition"
                :class="
                  form.motif === m.value
                    ? 'border-custom-chocolat bg-custom-chocolat/5 text-custom-chocolat'
                    : 'border-gray-200 text-gray-700 hover:border-gray-400'
                "
                @click="form.motif = m.value"
              >
                {{ m.label }}
              </button>
            </div>
            <p class="text-xs text-gray-500 mt-2">{{ motifInfo.description }}</p>
            <div
              v-if="motifInfo.warning"
              class="mt-3 border border-amber-300 bg-amber-50 rounded-lg p-3 flex items-start gap-2"
            >
              <font-awesome-icon :icon="['fas', 'triangle-exclamation']" class="w-5 h-5 text-amber-600 mt-0.5 shrink-0" />
              <p class="text-xs text-amber-900">{{ motifInfo.warning }}</p>
            </div>
          </div>

          <!-- Titre -->
          <div>
            <label class="block text-sm font-semibold text-gray-700 mb-2">
              Titre de la salle <span class="text-red-500">*</span>
            </label>
            <input
              v-model="form.titre"
              type="text"
              class="w-full border-2 rounded-lg p-3 border-gray-200 focus:outline-hidden focus:border-custom-chocolat transition-colors"
              placeholder="Ex : Club Wolof du vendredi"
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
              class="w-full border-2 rounded-lg p-3 border-gray-200 focus:outline-hidden focus:border-custom-chocolat transition-colors resize-none"
              placeholder="À qui s'adresse cette salle, quand se retrouve-t-on…"
            />
          </div>

          <!-- Visibilité -->
          <div>
            <label class="block text-sm font-semibold text-gray-700 mb-2">
              Visibilité
            </label>
            <div class="flex gap-2">
              <button
                type="button"
                class="flex-1 p-3 rounded-lg border-2 text-sm font-medium transition"
                :class="
                  form.visibilite === 'fermee'
                    ? 'border-custom-chocolat bg-custom-chocolat/5 text-custom-chocolat'
                    : 'border-gray-200 text-gray-700 hover:border-gray-400'
                "
                @click="form.visibilite = 'fermee'"
              >
                <font-awesome-icon :icon="['fas', 'lock']" class="mr-1" /> Fermée (sur invitation)
              </button>
              <button
                type="button"
                class="flex-1 p-3 rounded-lg border-2 text-sm font-medium transition"
                :class="
                  form.visibilite === 'visible'
                    ? 'border-custom-chocolat bg-custom-chocolat/5 text-custom-chocolat'
                    : 'border-gray-200 text-gray-700 hover:border-gray-400'
                "
                @click="form.visibilite = 'visible'"
              >
                <font-awesome-icon :icon="['fas', 'eye']" class="mr-1" /> Visible (demandes)
              </button>
            </div>
            <p class="text-xs text-gray-500 mt-1">
              Vous pourrez changer la visibilité plus tard depuis la salle.
            </p>
          </div>

          <!-- Nombre max de participants -->
          <div>
            <label class="block text-sm font-semibold text-gray-700 mb-2">
              Nombre max de participants
            </label>
            <input
              v-model.number="form.max_participants"
              type="number"
              min="2"
              max="50"
              class="w-full border-2 rounded-lg p-3 border-gray-200 focus:outline-hidden focus:border-custom-chocolat transition-colors"
              placeholder="Jusqu'à 50"
            />
          </div>

          <!-- Code d'accès (optionnel) -->
          <div>
            <label class="block text-sm font-semibold text-gray-700 mb-2">
              <font-awesome-icon :icon="['fas', 'lock']" class="w-4 h-4 mr-1 text-gray-400" />
              Code d'accès (optionnel)
            </label>
            <input
              v-model="form.code_acces"
              type="text"
              class="w-full border-2 rounded-lg p-3 border-gray-200 focus:outline-hidden focus:border-custom-chocolat transition-colors"
              placeholder="Laissez vide pour un accès ouvert"
            />
          </div>

          <!-- Déclaration d'âge adulte -->
          <div class="border border-gray-300 rounded-lg p-3 bg-gray-50">
            <label class="flex items-start gap-3 cursor-pointer">
              <input
                v-model="form.declaration_adulte"
                type="checkbox"
                class="mt-0.5 w-5 h-5 text-custom-chocolat border-gray-300 rounded focus:ring-custom-chocolat"
              />
              <span class="text-sm text-gray-800">
                <strong>Je déclare être majeur·e (18 ans ou plus)</strong>
                et reconnais ma responsabilité en tant que créateur·trice de cette salle.
                <span class="text-red-500">*</span>
              </span>
            </label>
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
              <font-awesome-icon v-if="form.loading" :icon="['fas', 'spinner']" class="w-4 h-4 animate-spin" />
              {{ form.loading ? 'Création...' : 'Créer la salle privée' }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </Transition>
</template>
