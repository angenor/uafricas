<template>
  <!-- PDF Viewer -->
  <CommonPdfViewer
    v-if="pdfSelected.url"
    :url="pdfSelected.url"
    :acces="pdfSelected.acces"
    @close="pdfSelected.url = null"
  />

  <div class="min-h-screen pb-10 bg-gray-50">
    <!-- Hero Section -->
    <div
      class="relative h-80 bg-cover bg-center"
      style="background-image: url('https://images.unsplash.com/photo-1507842217343-583bb7270b66?ixlib=rb-1.2.1&auto=format&fit=crop&w=1900&q=80')"
    >
      <div class="absolute inset-0 bg-gradient-to-r from-custom-chocolat/90 to-black/70"></div>

      <div class="absolute inset-0 flex flex-col items-center justify-center mt-5">
        <h1 class="text-white text-4xl md:text-5xl font-bold mb-4 animate-title">
          Numetech
        </h1>
        <div class="h-1 w-24 bg-custom-green rounded animate-line"></div>
        <p class="text-white text-xl md:text-2xl mt-4 animate-subtitle">
          Bibliothèque Numérique
        </p>
      </div>
    </div>

    <!-- Barre de recherche -->
    <div class="max-w-4xl mx-auto -mt-8 relative z-10 px-4">
      <div class="bg-white rounded-xl shadow-xl p-5 transform transition-all hover:shadow-2xl">
        <div class="flex flex-col md:flex-row gap-3">
          <div class="flex-1">
            <input
              v-model="searchQuery"
              type="text"
              class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-custom-green transition-all"
              placeholder="Rechercher un document..."
            />
          </div>
          <button
            class="bg-gradient-to-r from-custom-green to-green-600 hover:from-green-600 hover:to-custom-green text-white px-6 py-3 rounded-lg transition-all duration-300 transform hover:scale-105 focus:outline-none focus:ring-2 focus:ring-custom-green flex items-center justify-center"
          >
            <font-awesome-icon icon="fa-solid fa-search" class="mr-2" />
            Recherche
          </button>
        </div>

        <!-- Filtres -->
        <div class="flex flex-wrap mt-3 gap-2">
          <label
            v-for="type in documentTypes"
            :key="type"
            class="filter-option"
          >
            <input
              type="radio"
              name="filter"
              v-model="selectedType"
              :value="type"
              class="hidden"
            />
            <div
              class="px-4 py-2 rounded-full text-sm cursor-pointer transition-all duration-200"
              :class="[
                selectedType === type
                  ? 'bg-custom-chocolat text-white'
                  : 'bg-gray-100 text-gray-600 hover:bg-gray-200',
              ]"
            >
              {{ type }}
            </div>
          </label>
        </div>
      </div>
    </div>

    <!-- Titre et bouton ajouter -->
    <div class="max-w-7xl mx-auto px-4 mt-8 mb-4 flex justify-between items-center">
      <h2 class="text-2xl font-bold text-gray-800">Documents disponibles</h2>
      <button
        @click="showAddPopup = true"
        class="flex items-center space-x-2 px-4 py-2 bg-gradient-to-r from-custom-chocolat to-amber-700 text-white rounded-lg transition-all duration-300 hover:shadow-lg transform hover:scale-105"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
        </svg>
        <span>Ajouter un document</span>
      </button>
    </div>

    <!-- Grille de documents -->
    <div class="max-w-7xl mx-auto px-4">
      <TransitionGroup
        name="document-list"
        tag="div"
        class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6"
      >
        <div
          v-for="document in filteredDocuments"
          :key="document.id"
          class="bg-white rounded-xl overflow-hidden shadow-md hover:shadow-xl transition-all duration-300"
        >
          <!-- Image avec effet hover -->
          <div
            @click="pdfSelected = { url: document.doc_url, acces: document.acces }"
            class="relative h-72 overflow-hidden cursor-pointer group"
          >
            <img
              class="h-full w-full object-cover transform transition-transform duration-500 group-hover:scale-110"
              :src="document.couverture_url"
              :alt="document.titre"
            />
            <div class="absolute inset-0 bg-gradient-to-t from-black/80 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300 flex items-end justify-center p-4">
              <span class="text-white font-medium">Cliquer pour consulter</span>
            </div>
          </div>

          <!-- Informations du document -->
          <div class="p-4 bg-white">
            <h3 class="text-lg font-bold text-gray-800 line-clamp-1 mb-1">
              {{ document.titre }}
            </h3>

            <div class="flex items-center text-sm text-gray-600 mb-2">
              <font-awesome-icon icon="fa-solid fa-calendar" class="mr-1" />
              <span>{{ formatDate(document.date_heure_publication) }}</span>
            </div>

            <div class="flex items-center mb-2">
              <span
                :class="[
                  'px-2 py-1 text-xs rounded-full',
                  document.acces === 'Lecture'
                    ? 'bg-blue-100 text-blue-800'
                    : 'bg-green-100 text-green-800',
                ]"
              >
                {{ document.acces === 'Lecture' ? 'Lecture seule' : 'Téléchargeable' }}
              </span>
            </div>
          </div>
        </div>
      </TransitionGroup>

      <!-- État vide -->
      <div v-if="filteredDocuments.length === 0" class="text-center py-16">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-20 w-20 text-gray-300 mx-auto mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
        </svg>
        <p class="text-gray-500 text-lg">Aucun document trouvé</p>
      </div>
    </div>
  </div>

  <!-- Popup Ajouter un document - Formulaire complet -->
  <Transition name="modal-fade">
    <div v-if="showAddPopup" class="z-50 fixed inset-0 flex items-center justify-center">
      <div @click="showAddPopup = false" class="absolute inset-0 bg-black/60 backdrop-blur-sm"></div>

      <div class="relative w-full max-w-2xl mx-4 md:mx-auto animate-slideIn">
        <form
          @submit.prevent="submitDoc"
          class="bg-white rounded-xl overflow-hidden shadow-2xl transform transition-transform max-h-[90vh] overflow-y-auto"
        >
          <!-- En-tête -->
          <div class="bg-gradient-to-r from-custom-green to-emerald-700 py-4 sticky top-0 z-10">
            <h2 class="text-white text-center text-xl font-bold">
              Ajouter un document
            </h2>
          </div>

          <!-- Corps du formulaire -->
          <div class="p-6">
            <!-- Titre du document -->
            <div class="mb-4">
              <label class="block text-gray-700 text-sm font-bold mb-2">Titre du document</label>
              <input
                required
                v-model="documentForm.titre"
                type="text"
                class="w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-custom-green transition-all duration-200"
                placeholder="Saisissez le titre du document"
              />
            </div>

            <!-- Description -->
            <div class="mb-4">
              <label class="block text-gray-700 text-sm font-bold mb-2">Description</label>
              <textarea
                required
                v-model="documentForm.description"
                class="w-full px-3 py-2 border rounded-lg resize-none focus:outline-none focus:ring-2 focus:ring-custom-green transition-all duration-200"
                rows="3"
                placeholder="Décrivez le contenu du document"
              ></textarea>
            </div>

            <!-- Uploaders -->
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
              <!-- Image de couverture -->
              <div class="upload-container group">
                <label class="block p-4 border-2 border-dashed border-custom-green rounded-lg cursor-pointer hover:bg-custom-green/10 transition-colors duration-200">
                  <div class="flex flex-col items-center">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-10 w-10 text-custom-green mb-2 group-hover:scale-110 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
                    </svg>
                    <span class="text-custom-green font-medium">Image de couverture</span>
                    <span class="text-xs text-gray-500 mt-1">Cliquez pour sélectionner</span>
                    <input
                      required
                      class="hidden"
                      type="file"
                      accept="image/*"
                      @change="handleImageUpload"
                    />
                    <span v-if="docImage" class="mt-2 text-sm truncate max-w-full text-gray-700">
                      {{ docImage.name }}
                    </span>
                  </div>
                </label>
              </div>

              <!-- Document PDF -->
              <div class="upload-container group">
                <label class="block p-4 border-2 border-dashed border-custom-chocolat rounded-lg cursor-pointer hover:bg-custom-chocolat/10 transition-colors duration-200">
                  <div class="flex flex-col items-center">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-10 w-10 text-custom-chocolat mb-2 group-hover:scale-110 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                    </svg>
                    <span class="text-custom-chocolat font-medium">Document PDF</span>
                    <span class="text-xs text-gray-500 mt-1">Cliquez pour téléverser</span>
                    <input
                      required
                      class="hidden"
                      type="file"
                      accept=".pdf"
                      @change="handleDocUpload"
                    />
                    <span v-if="docFichier" class="mt-2 text-sm truncate max-w-full text-gray-700">
                      {{ docFichier.name }}
                    </span>
                  </div>
                </label>
              </div>
            </div>

            <!-- Type et Accès -->
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
              <div>
                <label class="block text-gray-700 text-sm font-bold mb-2">Type de document</label>
                <select
                  required
                  v-model="documentForm.type"
                  class="w-full px-3 py-2 border rounded-lg bg-white focus:outline-none focus:ring-2 focus:ring-custom-green transition-all duration-200"
                >
                  <option value="">Sélectionnez un type</option>
                  <option value="Roman">Roman</option>
                  <option value="Livre">Livre</option>
                  <option value="Thèse">Thèse</option>
                  <option value="Mémoire">Mémoire</option>
                  <option value="Rapport">Rapport</option>
                  <option value="Autre">Autre</option>
                </select>
              </div>

              <div>
                <label class="block text-gray-700 text-sm font-bold mb-2">Accès</label>
                <select
                  required
                  v-model="documentForm.acces"
                  class="w-full px-3 py-2 border rounded-lg bg-white focus:outline-none focus:ring-2 focus:ring-custom-green transition-all duration-200"
                >
                  <option value="">Sélectionnez un type d'accès</option>
                  <option value="Lecture">Lecture seule</option>
                  <option value="Téléchargeable">Téléchargeable</option>
                </select>
              </div>
            </div>

            <!-- Informations sur l'auteur -->
            <div class="mb-4">
              <label class="block text-gray-700 text-sm font-bold mb-2">Informations sur l'auteur</label>
              <textarea
                required
                v-model="documentForm.auteurBiblio"
                class="w-full px-3 py-2 border rounded-lg resize-none focus:outline-none focus:ring-2 focus:ring-custom-green transition-all duration-200"
                rows="3"
                placeholder="Biographie et informations sur l'auteur"
              ></textarea>
            </div>

            <!-- Date et Rapport -->
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
              <div>
                <label class="block text-gray-700 text-sm font-bold mb-2">Date de publication</label>
                <input
                  v-model="documentForm.datePublication"
                  required
                  type="date"
                  class="w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-custom-green transition-all duration-200"
                />
              </div>

              <div>
                <label class="block text-gray-700 text-sm font-bold mb-2">Votre rapport avec le document</label>
                <select
                  v-model="documentForm.rapport"
                  class="w-full px-3 py-2 border rounded-lg bg-white focus:outline-none focus:ring-2 focus:ring-custom-green transition-all duration-200"
                >
                  <option value="">Sélectionnez votre rapport</option>
                  <option value="Auteur">Auteur</option>
                  <option value="Co-auteur">Co-auteur</option>
                  <option value="Aucun">Aucun</option>
                </select>
              </div>
            </div>

            <!-- Consentement -->
            <div class="mb-6">
              <label class="flex items-center cursor-pointer">
                <input
                  class="w-5 h-5 text-custom-green border-2 focus:ring-green-500 rounded transition-colors duration-200"
                  type="checkbox"
                  v-model="documentForm.consent"
                  required
                />
                <span class="ml-2 text-sm text-gray-700">
                  Moi <span class="font-bold">{{ mockUser.prenom }} {{ mockUser.nom }}</span>, accepte la diffusion de cette publication
                </span>
              </label>
            </div>

            <!-- Barre de progression -->
            <div v-if="progress > 0" class="mb-4">
              <div class="w-full bg-gray-200 rounded-full h-2.5">
                <div
                  class="bg-custom-green h-2.5 rounded-full transition-all duration-300 ease-in-out"
                  :style="{ width: progress + '%' }"
                ></div>
              </div>
              <p class="text-sm text-center mt-1">{{ Math.round(progress) }}% terminé</p>
            </div>

            <!-- Boutons -->
            <div class="flex justify-end space-x-3">
              <button
                @click="showAddPopup = false"
                type="button"
                class="px-4 py-2 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300 transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-gray-400"
              >
                Annuler
              </button>
              <button
                type="submit"
                :disabled="isSubmitting"
                class="px-4 py-2 bg-gradient-to-r from-custom-green to-green-600 text-white rounded-lg hover:from-custom-green hover:to-green-700 transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-green-500 transform hover:scale-105 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {{ isSubmitting ? 'Envoi en cours...' : 'Soumettre' }}
              </button>
            </div>
          </div>
        </form>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { documentsNumeriques as initialDocuments, documentTypes, mockCurrentUser, type DocumentNumerique } from '~/mocks/bibliotheques'

useHead({
  title: 'Bibliothèque Numérique - UAfricas',
  meta: [
    { name: 'description', content: 'Accédez à des milliers de livres et documents africains' },
  ],
})

useAOS()

// Search and filter state
const searchQuery = ref('')
const selectedType = ref('Livre')

// Documents array (reactive to allow adding new documents)
const documents = ref<DocumentNumerique[]>([...initialDocuments])

// PDF viewer state
const pdfSelected = ref<{ url: string | null; acces: string | null }>({ url: null, acces: null })

// Add popup state
const showAddPopup = ref(false)
const progress = ref(0)
const isSubmitting = ref(false)

// File refs
const docImage = ref<File | null>(null)
const docFichier = ref<File | null>(null)

// Mock user for consent display
const mockUser = mockCurrentUser

// Document form state
const documentForm = ref({
  titre: '',
  description: '',
  type: '',
  acces: '',
  auteurBiblio: '',
  datePublication: '',
  rapport: '',
  consent: false
})

// Computed filtered documents
const filteredDocuments = computed(() => {
  let docs = documents.value

  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    docs = docs.filter(d =>
      d.titre.toLowerCase().includes(query) ||
      d.description.toLowerCase().includes(query)
    )
  }

  if (selectedType.value && selectedType.value !== 'Autre') {
    docs = docs.filter(d => d.type === selectedType.value)
  }

  return docs
})

// File upload handlers
function handleImageUpload(event: Event) {
  const target = event.target as HTMLInputElement
  if (target.files && target.files[0]) {
    docImage.value = target.files[0]
  }
}

function handleDocUpload(event: Event) {
  const target = event.target as HTMLInputElement
  if (target.files && target.files[0]) {
    docFichier.value = target.files[0]
  }
}

// Submit handler with simulated progress
async function submitDoc() {
  if (!documentForm.value.consent) {
    alert('Veuillez accepter les conditions')
    return
  }

  isSubmitting.value = true
  progress.value = 0

  // Simulate upload progress
  const interval = setInterval(() => {
    progress.value += 10
    if (progress.value >= 100) {
      clearInterval(interval)

      // Create new document with local file URLs or fallbacks
      const newDoc: DocumentNumerique = {
        id: String(Date.now()),
        titre: documentForm.value.titre,
        description: documentForm.value.description,
        couverture_url: docImage.value
          ? URL.createObjectURL(docImage.value)
          : 'https://images.unsplash.com/photo-1544716278-ca5e3f4abd8c?w=400',
        doc_url: docFichier.value
          ? URL.createObjectURL(docFichier.value)
          : '/documents/sample.pdf',
        acces: documentForm.value.acces as 'Lecture' | 'Téléchargeable',
        date_heure_publication: documentForm.value.datePublication,
        type: documentForm.value.type,
        auteur: { biblio: documentForm.value.auteurBiblio },
        user: { nom: mockUser.nom, prenom: mockUser.prenom }
      }

      // Add to beginning of array
      documents.value.unshift(newDoc)

      // Reset form and close popup
      resetForm()
      showAddPopup.value = false
      isSubmitting.value = false
    }
  }, 200) // 2 seconds total
}

function resetForm() {
  documentForm.value = {
    titre: '',
    description: '',
    type: '',
    acces: '',
    auteurBiblio: '',
    datePublication: '',
    rapport: '',
    consent: false
  }
  docImage.value = null
  docFichier.value = null
  progress.value = 0
}

function formatDate(dateString: string) {
  const date = new Date(dateString)
  return date.toLocaleDateString('fr-FR', { month: 'long', year: 'numeric' })
}
</script>

<style scoped>
@keyframes fadeIn {
  from { opacity: 0; transform: translateY(-20px); }
  to { opacity: 1; transform: translateY(0); }
}

@keyframes expandLine {
  from { width: 0; }
  to { width: 6rem; }
}

@keyframes slideIn {
  from { transform: translateY(-20px); opacity: 0; }
  to { transform: translateY(0); opacity: 1; }
}

.animate-title {
  animation: fadeIn 1s ease-out forwards;
}

.animate-subtitle {
  animation: fadeIn 1s ease-out 0.3s forwards;
  opacity: 0;
}

.animate-line {
  animation: expandLine 1.2s ease-out 0.1s forwards;
  width: 0;
}

.animate-slideIn {
  animation: slideIn 0.3s ease-out forwards;
}

.document-list-enter-active,
.document-list-leave-active {
  transition: all 0.5s ease;
}

.document-list-enter-from,
.document-list-leave-to {
  opacity: 0;
  transform: translateY(30px);
}

.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: opacity 0.3s ease;
}

.modal-fade-enter-from,
.modal-fade-leave-to {
  opacity: 0;
}
</style>
