<script setup lang="ts">
import { DOMAINES_AFRICANTIVES, PAYS_AFRICAINS } from '~/composables/useAfricantives'

interface Props {
  isOpen: boolean
}

const props = defineProps<Props>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'submit', data: {
    titre: string
    description: string
    domaine: string
    pays: string
    ville: string
    couvertureFile: File | null
  }): void
}>()

const form = reactive({
  titre: '',
  description: '',
  domaine: '',
  pays: '',
  ville: '',
  couvertureFile: null as File | null,
  couverturePreview: '' as string,
  loading: false,
  submitted: false,
  error: false,
  errorMessage: '',
})

const domaines = DOMAINES_AFRICANTIVES.filter(d => d.value !== '')
const pays = PAYS_AFRICAINS

const isFormValid = computed(() => {
  return form.titre.trim().length >= 5 && form.description.trim().length >= 20
})

const resetForm = () => {
  form.titre = ''
  form.description = ''
  form.domaine = ''
  form.pays = ''
  form.ville = ''
  form.couvertureFile = null
  form.couverturePreview = ''
  form.loading = false
  form.submitted = false
  form.error = false
  form.errorMessage = ''
}

const handleFileChange = (event: Event) => {
  const target = event.target as HTMLInputElement
  if (target.files && target.files[0]) {
    const file = target.files[0]

    // Verifier la taille (max 5 Mo)
    if (file.size > 5 * 1024 * 1024) {
      form.error = true
      form.errorMessage = 'L\'image ne doit pas dépasser 5 Mo.'
      return
    }

    // Verifier le type
    if (!file.type.startsWith('image/')) {
      form.error = true
      form.errorMessage = 'Veuillez sélectionner un fichier image valide.'
      return
    }

    form.couvertureFile = file
    form.error = false
    form.errorMessage = ''

    // Preview
    const reader = new FileReader()
    reader.onload = (e) => {
      form.couverturePreview = e.target?.result as string
    }
    reader.readAsDataURL(file)
  }
}

const removeImage = () => {
  form.couvertureFile = null
  form.couverturePreview = ''
}

const handleSubmit = async () => {
  form.error = false
  form.errorMessage = ''

  if (!form.titre.trim()) {
    form.error = true
    form.errorMessage = 'Le titre est requis.'
    return
  }

  if (form.titre.trim().length < 5) {
    form.error = true
    form.errorMessage = 'Le titre doit contenir au moins 5 caractères.'
    return
  }

  if (!form.description.trim()) {
    form.error = true
    form.errorMessage = 'La description est requise.'
    return
  }

  if (form.description.trim().length < 20) {
    form.error = true
    form.errorMessage = 'La description doit contenir au moins 20 caractères.'
    return
  }

  emit('submit', {
    titre: form.titre.trim(),
    description: form.description.trim(),
    domaine: form.domaine,
    pays: form.pays,
    ville: form.ville.trim(),
    couvertureFile: form.couvertureFile,
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
        class="relative w-full max-w-2xl bg-white/95 shadow-2xl backdrop-blur-md rounded-2xl border border-white/20 transform transition-all duration-300 max-h-[90vh] overflow-hidden"
        @click.stop
      >
        <!-- En-tete -->
        <div class="bg-gradient-to-r from-orange-500 to-red-500 text-white p-6">
          <div class="flex items-center justify-between">
            <div>
              <h2 class="text-2xl font-bold">Publier une initiative</h2>
              <p class="text-orange-100 text-sm mt-1">Partagez votre initiative africaine avec la communauté</p>
            </div>
            <button @click="emit('close')" class="text-white hover:text-red-200 transition-colors">
              <font-awesome-icon :icon="['fas', 'xmark']" class="w-6 h-6" />
            </button>
          </div>
        </div>

        <!-- Formulaire -->
        <form @submit.prevent="handleSubmit" class="p-6 space-y-6 bg-white/90 backdrop-blur-xs max-h-[70vh] overflow-y-auto">
          <!-- Message de succes -->
          <Transition name="fade-slide">
            <div v-if="form.submitted" class="bg-green-50 border-l-4 border-green-500 p-4 rounded-lg">
              <div class="flex items-center">
                <font-awesome-icon :icon="['fas', 'circle-check']" class="w-6 h-6 text-green-500 mr-3" />
                <p class="text-green-700 font-medium">Initiative publiée avec succès !</p>
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

          <!-- Section: Informations principales -->
          <div class="space-y-4 bg-gray-50 p-4 rounded-xl">
            <h3 class="text-lg font-semibold text-gray-800 mb-4">
              <font-awesome-icon :icon="['fas', 'lightbulb']" class="w-4 h-4 text-orange-500 mr-2" />
              Informations principales
            </h3>

            <!-- Titre -->
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">Titre de l'initiative *</label>
              <input
                v-model="form.titre"
                type="text"
                required
                maxlength="350"
                class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-orange-500 focus:border-transparent transition-all"
                placeholder="Ex: Plateforme de micro-crédit pour les agricultrices"
              />
              <p class="text-xs text-gray-400 mt-1">{{ form.titre.length }}/350 caractères (minimum 5)</p>
            </div>

            <!-- Description -->
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">Description *</label>
              <textarea
                v-model="form.description"
                rows="5"
                required
                class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-orange-500 focus:border-transparent transition-all resize-y"
                placeholder="Décrivez votre initiative en détail : objectifs, impact, bénéficiaires..."
              ></textarea>
              <p class="text-xs text-gray-400 mt-1">{{ form.description.length }} caractères (minimum 20)</p>
            </div>
          </div>

          <!-- Section: Domaine et localisation -->
          <div class="space-y-4 bg-gray-50 p-4 rounded-xl">
            <h3 class="text-lg font-semibold text-gray-800 mb-4">
              <font-awesome-icon :icon="['fas', 'location-dot']" class="w-4 h-4 text-orange-500 mr-2" />
              Domaine et localisation
            </h3>

            <!-- Domaine -->
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">Domaine d'activité</label>
              <select
                v-model="form.domaine"
                class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-orange-500 focus:border-transparent transition-all"
              >
                <option value="">Sélectionnez un domaine (optionnel)</option>
                <option v-for="dom in domaines" :key="dom.value" :value="dom.value">
                  {{ dom.label }}
                </option>
              </select>
            </div>

            <!-- Pays et Ville -->
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-2">Pays</label>
                <select
                  v-model="form.pays"
                  class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-orange-500 focus:border-transparent transition-all"
                >
                  <option value="">Sélectionnez un pays</option>
                  <option v-for="p in pays" :key="p" :value="p">{{ p }}</option>
                </select>
              </div>

              <div>
                <label class="block text-sm font-medium text-gray-700 mb-2">Ville</label>
                <input
                  v-model="form.ville"
                  type="text"
                  maxlength="200"
                  class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-orange-500 focus:border-transparent transition-all"
                  placeholder="Ex: Dakar"
                />
              </div>
            </div>
          </div>

          <!-- Section: Image de couverture -->
          <div class="space-y-4 bg-gray-50 p-4 rounded-xl">
            <h3 class="text-lg font-semibold text-gray-800 mb-4">
              <font-awesome-icon :icon="['fas', 'image']" class="w-4 h-4 text-orange-500 mr-2" />
              Image de couverture
            </h3>

            <!-- Preview -->
            <div v-if="form.couverturePreview" class="relative">
              <img
                :src="form.couverturePreview"
                alt="Aperçu"
                class="w-full h-48 object-cover rounded-lg border border-gray-200"
              />
              <button
                type="button"
                @click="removeImage"
                class="absolute top-2 right-2 w-8 h-8 bg-red-500 text-white rounded-full flex items-center justify-center hover:bg-red-600 transition-colors shadow-lg"
              >
                <font-awesome-icon :icon="['fas', 'xmark']" class="w-4 h-4" />
              </button>
            </div>

            <!-- Input fichier -->
            <div v-else class="border-2 border-dashed border-gray-300 rounded-lg p-6 text-center hover:border-orange-400 transition-colors">
              <font-awesome-icon :icon="['fas', 'cloud-arrow-up']" class="w-10 h-10 text-gray-400 mx-auto mb-3" />
              <p class="text-sm text-gray-600 mb-2">Cliquez pour ajouter une image</p>
              <p class="text-xs text-gray-400 mb-3">JPG, PNG, WebP - Max 5 Mo</p>
              <input
                type="file"
                accept="image/jpeg,image/png,image/webp"
                @change="handleFileChange"
                class="w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-lg file:border-0 file:text-sm file:font-medium file:bg-orange-100 file:text-orange-700 hover:file:bg-orange-200 file:cursor-pointer cursor-pointer"
              />
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
              class="flex-1 bg-gradient-to-r from-orange-500 to-red-500 hover:from-orange-600 hover:to-red-600 text-white font-medium py-3 px-6 rounded-lg transition-all disabled:opacity-50 disabled:cursor-not-allowed shadow-lg hover:shadow-xl"
            >
              <span v-if="form.loading" class="flex items-center justify-center">
                <font-awesome-icon :icon="['fas', 'spinner']" class="w-5 h-5 animate-spin mr-2" />
                Publication en cours...
              </span>
              <span v-else class="flex items-center justify-center">
                <font-awesome-icon :icon="['fas', 'paper-plane']" class="w-4 h-4 mr-2" />
                Publier l'initiative
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
