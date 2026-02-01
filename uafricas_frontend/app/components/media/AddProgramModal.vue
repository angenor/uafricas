<script setup lang="ts">
import { availableGenres, availableCountries, createEmptyProgramForm, type ProgramForm } from '~/mocks/radios'

interface Props {
  isOpen: boolean
}

const props = defineProps<Props>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'submit', form: ProgramForm): void
}>()

const programForm = reactive(createEmptyProgramForm())

const resetForm = () => {
  Object.assign(programForm, createEmptyProgramForm())
}

const handleSubmit = async () => {
  // Validation des champs requis
  if (!programForm.nom || !programForm.description || !programForm.emailProducteur || !programForm.pays || !programForm.langue || !programForm.programType || !programForm.typeMedia) {
    programForm.error = true
    programForm.errorMessage = 'Veuillez remplir tous les champs obligatoires.'
    return
  }

  if (!programForm.imageCouverture) {
    programForm.error = true
    programForm.errorMessage = "L'image de couverture est obligatoire."
    return
  }

  if (programForm.typeMedia === 'Radio' && !programForm.urlAudio) {
    programForm.error = true
    programForm.errorMessage = "L'URL audio est obligatoire pour une radio."
    return
  }

  if (programForm.typeMedia === 'Télévision' && !programForm.urlVideo) {
    programForm.error = true
    programForm.errorMessage = "L'URL vidéo est obligatoire pour une télévision."
    return
  }

  if (programForm.typeMedia === 'Mixte' && !programForm.urlAudio && !programForm.urlVideo) {
    programForm.error = true
    programForm.errorMessage = 'Au moins une URL (audio ou vidéo) est requise pour un média mixte.'
    return
  }

  if (programForm.genres.length === 0) {
    programForm.error = true
    programForm.errorMessage = 'Veuillez sélectionner au moins un genre musical.'
    return
  }

  if (programForm.langue === 'Langue locale' && !programForm.langueLocale) {
    programForm.error = true
    programForm.errorMessage = 'Veuillez préciser la langue locale.'
    return
  }

  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
  if (!emailRegex.test(programForm.emailProducteur)) {
    programForm.error = true
    programForm.errorMessage = 'Veuillez entrer une adresse email valide.'
    return
  }

  try {
    programForm.loading = true
    programForm.error = false
    programForm.errorMessage = ''

    // Simulation de l'envoi
    await new Promise(resolve => setTimeout(resolve, 2000))

    programForm.submitted = true

    setTimeout(() => {
      emit('submit', { ...programForm })
      resetForm()
      emit('close')
    }, 2000)

  } catch {
    programForm.error = true
    programForm.errorMessage = "Une erreur est survenue lors de l'ajout du programme."
  } finally {
    programForm.loading = false
  }
}

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
      class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black bg-opacity-75 backdrop-blur-sm"
      @click.self="emit('close')"
    >
      <div
        class="relative w-full max-w-2xl bg-white shadow-2xl bg-opacity-95 backdrop-blur-md rounded-2xl border border-white border-opacity-20 transform transition-all duration-300 max-h-[90vh] overflow-hidden"
        @click.stop
      >
        <!-- En-tête du modal -->
        <div class="bg-gradient-to-r from-yellow-400 to-yellow-600 text-black p-6">
          <div class="flex items-center justify-between">
            <div>
              <h2 class="text-2xl font-bold">Ajouter un programme</h2>
              <p class="text-yellow-800 text-sm mt-1">Partagez votre chaîne radio/télé avec la communauté</p>
            </div>
            <button @click="emit('close')" class="text-black hover:text-red-600 transition-colors">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>
        </div>

        <!-- Formulaire -->
        <form @submit.prevent="handleSubmit" class="p-6 space-y-6 bg-white bg-opacity-90 backdrop-blur-sm max-h-[70vh] overflow-y-auto">
          <!-- Messages de succès/erreur -->
          <Transition name="fade-slide">
            <div v-if="programForm.submitted" class="bg-green-50 border-l-4 border-green-500 p-4 rounded-lg">
              <div class="flex items-center">
                <svg class="w-6 h-6 text-green-500 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                </svg>
                <p class="text-green-700 font-medium">Programme ajouté avec succès !</p>
              </div>
            </div>
          </Transition>

          <Transition name="fade-slide">
            <div v-if="programForm.error" class="bg-red-50 border-l-4 border-red-500 p-4 rounded-lg">
              <div class="flex items-center">
                <svg class="w-6 h-6 text-red-500 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                </svg>
                <p class="text-red-700">{{ programForm.errorMessage }}</p>
              </div>
            </div>
          </Transition>

          <!-- Informations générales -->
          <div class="space-y-4 bg-gray-50 p-4 rounded-xl">
            <h3 class="text-lg font-semibold text-gray-800 mb-4">Informations générales</h3>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">Nom de la chaîne *</label>
              <input v-model="programForm.nom" type="text" required class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-yellow-500 focus:border-transparent transition-all" placeholder="Ex: Africa Music Radio" />
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">Description *</label>
              <textarea v-model="programForm.description" rows="3" required class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-yellow-500 focus:border-transparent transition-all" placeholder="Décrivez votre chaîne..."></textarea>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">Image de couverture (URL) *</label>
              <input v-model="programForm.imageCouverture" type="url" required class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-yellow-500 focus:border-transparent transition-all" placeholder="https://example.com/image.jpg" />
            </div>
          </div>

          <!-- Informations de contact -->
          <div class="space-y-4 bg-gray-50 p-4 rounded-xl">
            <h3 class="text-lg font-semibold text-gray-800 mb-4">Informations de contact</h3>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">Téléphone</label>
              <input v-model="programForm.telephone" type="tel" class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-yellow-500 focus:border-transparent transition-all" placeholder="+225 01 02 03 04 05" />
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">Email producteur *</label>
              <input v-model="programForm.emailProducteur" type="email" required class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-yellow-500 focus:border-transparent transition-all" placeholder="producteur@example.com" />
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">Pays *</label>
              <select v-model="programForm.pays" required class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-yellow-500 focus:border-transparent transition-all">
                <option value="">Sélectionnez un pays</option>
                <option v-for="country in availableCountries" :key="country" :value="country">{{ country }}</option>
              </select>
            </div>
          </div>

          <!-- URLs de diffusion -->
          <div class="space-y-4 bg-gray-50 p-4 rounded-xl">
            <h3 class="text-lg font-semibold text-gray-800 mb-4">URLs de diffusion *</h3>
            <p class="text-sm text-gray-600 mb-4">Fournissez au moins une URL (audio pour les radios, vidéo pour les télévisions)</p>

            <div class="mb-4">
              <label class="block text-sm font-medium text-gray-700 mb-2">Type de média *</label>
              <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                <label class="flex items-center space-x-3 cursor-pointer">
                  <input v-model="programForm.typeMedia" type="radio" value="Radio" class="w-4 h-4 text-yellow-600 bg-gray-100 border-gray-300 focus:ring-yellow-500" />
                  <span class="text-gray-700">Radio (Audio)</span>
                </label>
                <label class="flex items-center space-x-3 cursor-pointer">
                  <input v-model="programForm.typeMedia" type="radio" value="Télévision" class="w-4 h-4 text-yellow-600 bg-gray-100 border-gray-300 focus:ring-yellow-500" />
                  <span class="text-gray-700">Télévision (Vidéo)</span>
                </label>
                <label class="flex items-center space-x-3 cursor-pointer">
                  <input v-model="programForm.typeMedia" type="radio" value="Mixte" class="w-4 h-4 text-yellow-600 bg-gray-100 border-gray-300 focus:ring-yellow-500" />
                  <span class="text-gray-700">Mixte (Audio + Vidéo)</span>
                </label>
              </div>
            </div>

            <div v-if="programForm.typeMedia === 'Radio' || programForm.typeMedia === 'Mixte'">
              <label class="block text-sm font-medium text-gray-700 mb-2">
                URL Audio (Stream Radio) {{ programForm.typeMedia === 'Radio' ? '*' : '' }}
              </label>
              <input v-model="programForm.urlAudio" type="url" :required="programForm.typeMedia === 'Radio'" class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-yellow-500 focus:border-transparent transition-all" placeholder="https://stream.example.com/audio.mp3" />
              <p class="text-xs text-gray-500 mt-1">Formats acceptés : MP3, AAC, OGG</p>
            </div>

            <div v-if="programForm.typeMedia === 'Télévision' || programForm.typeMedia === 'Mixte'">
              <label class="block text-sm font-medium text-gray-700 mb-2">
                URL Vidéo (Stream TV) {{ programForm.typeMedia === 'Télévision' ? '*' : '' }}
              </label>
              <input v-model="programForm.urlVideo" type="url" :required="programForm.typeMedia === 'Télévision'" class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-yellow-500 focus:border-transparent transition-all" placeholder="https://stream.example.com/video.m3u8" />
              <p class="text-xs text-gray-500 mt-1">Formats acceptés : M3U8, MP4, WebM</p>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">Site web (optionnel)</label>
              <input v-model="programForm.siteWeb" type="url" class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-yellow-500 focus:border-transparent transition-all" placeholder="https://www.example.com" />
            </div>
          </div>

          <!-- Langue -->
          <div class="space-y-4 bg-gray-50 p-4 rounded-xl">
            <h3 class="text-lg font-semibold text-gray-800 mb-4">Langue de diffusion *</h3>
            <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
              <label class="flex items-center space-x-3 cursor-pointer">
                <input v-model="programForm.langue" type="radio" value="Français" class="w-4 h-4 text-yellow-600 bg-gray-100 border-gray-300 focus:ring-yellow-500" />
                <span class="text-gray-700">Français</span>
              </label>
              <label class="flex items-center space-x-3 cursor-pointer">
                <input v-model="programForm.langue" type="radio" value="Anglais" class="w-4 h-4 text-yellow-600 bg-gray-100 border-gray-300 focus:ring-yellow-500" />
                <span class="text-gray-700">Anglais</span>
              </label>
              <label class="flex items-center space-x-3 cursor-pointer">
                <input v-model="programForm.langue" type="radio" value="Arabe" class="w-4 h-4 text-yellow-600 bg-gray-100 border-gray-300 focus:ring-yellow-500" />
                <span class="text-gray-700">Arabe</span>
              </label>
              <label class="flex items-center space-x-3 cursor-pointer">
                <input v-model="programForm.langue" type="radio" value="Langue locale" class="w-4 h-4 text-yellow-600 bg-gray-100 border-gray-300 focus:ring-yellow-500" />
                <span class="text-gray-700">Langue locale</span>
              </label>
            </div>

            <div v-if="programForm.langue === 'Langue locale'" class="mt-4">
              <label class="block text-sm font-medium text-gray-700 mb-2">Précisez la langue locale</label>
              <input v-model="programForm.langueLocale" type="text" class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-yellow-500 focus:border-transparent transition-all" placeholder="Ex: Wolof, Bambara, Yoruba, Swahili..." />
            </div>
          </div>

          <!-- Type de programme -->
          <div class="space-y-4 bg-gray-50 p-4 rounded-xl">
            <h3 class="text-lg font-semibold text-gray-800 mb-4">Type de programme *</h3>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
              <label class="flex items-center space-x-3 cursor-pointer">
                <input v-model="programForm.programType" type="radio" value="Local" class="w-4 h-4 text-yellow-600 bg-gray-100 border-gray-300 focus:ring-yellow-500" />
                <span class="text-gray-700">Local</span>
              </label>
              <label class="flex items-center space-x-3 cursor-pointer">
                <input v-model="programForm.programType" type="radio" value="Nationales" class="w-4 h-4 text-yellow-600 bg-gray-100 border-gray-300 focus:ring-yellow-500" />
                <span class="text-gray-700">Nationales</span>
              </label>
              <label class="flex items-center space-x-3 cursor-pointer">
                <input v-model="programForm.programType" type="radio" value="International" class="w-4 h-4 text-yellow-600 bg-gray-100 border-gray-300 focus:ring-yellow-500" />
                <span class="text-gray-700">International</span>
              </label>
            </div>
          </div>

          <!-- Genre musical -->
          <div class="space-y-4 bg-gray-50 p-4 rounded-xl">
            <h3 class="text-lg font-semibold text-gray-800 mb-4">Genres musicaux *</h3>
            <p class="text-sm text-gray-600 mb-3">Sélectionnez un ou plusieurs genres qui correspondent à votre programmation</p>

            <div class="grid grid-cols-2 md:grid-cols-3 gap-3 max-h-48 overflow-y-auto">
              <label v-for="genre in availableGenres" :key="genre" class="flex items-center space-x-2 cursor-pointer hover:bg-gray-100 p-2 rounded">
                <input v-model="programForm.genres" type="checkbox" :value="genre" class="w-4 h-4 text-yellow-600 bg-gray-100 border-gray-300 rounded focus:ring-yellow-500" />
                <span class="text-sm text-gray-700">{{ genre }}</span>
              </label>
            </div>

            <div v-if="programForm.genres.length > 0" class="mt-3">
              <p class="text-sm text-gray-600 mb-2">Genres sélectionnés :</p>
              <div class="flex flex-wrap gap-2">
                <span v-for="genre in programForm.genres" :key="genre" class="bg-yellow-100 text-yellow-800 text-xs px-2 py-1 rounded-full">
                  {{ genre }}
                </span>
              </div>
            </div>
          </div>

          <!-- Boutons d'action -->
          <div class="flex space-x-4 pt-6">
            <button type="button" @click="emit('close')" class="flex-1 bg-gray-300 hover:bg-gray-400 text-gray-800 font-medium py-3 px-6 rounded-lg transition-all">
              Annuler
            </button>
            <button type="submit" :disabled="programForm.loading" class="flex-1 bg-yellow-400 hover:bg-yellow-500 text-black font-medium py-3 px-6 rounded-lg transition-all disabled:opacity-50 disabled:cursor-not-allowed">
              <span v-if="programForm.loading" class="flex items-center justify-center">
                <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-black" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                  <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                  <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                Ajout en cours...
              </span>
              <span v-else>Ajouter le programme</span>
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
