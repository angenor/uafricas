<template>
  <!-- PDF Viewer -->
  <CommonPdfViewer
    v-if="pdfSelected.url"
    :url="pdfSelected.url"
    :acces="pdfSelected.acces"
    @close="pdfSelected.url = null"
  />

  <div class="min-h-screen pb-10 bg-gray-50">
    <!-- Hero Section (compact, titre ↔ description au survol) -->
    <div
      class="group relative bg-cover bg-center"
      style="background-image: url('https://images.unsplash.com/photo-1507842217343-583bb7270b66?ixlib=rb-1.2.1&auto=format&fit=crop&w=1900&q=80')"
    >
      <div class="absolute inset-0 bg-gradient-to-r from-custom-chocolat/90 to-black/70"></div>

      <div class="relative max-w-4xl mx-auto px-4 pt-16 pb-6 text-center select-none">
        <!-- Conteneur fixe : le titre et la description se superposent (crossfade au survol) -->
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12">
          <h1 class="absolute inset-0 flex items-center justify-center text-white text-2xl md:text-4xl font-bold transition-opacity duration-300 group-hover:opacity-0">
            Numetech
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Permettre à des africains ou à des écoles de consulter vos publications ou ouvrages.
          </p>
        </div>
      </div>
    </div>

    <!-- Barre de recherche -->
    <div class="max-w-4xl mx-auto mt-6 relative z-10 px-4">
      <div class="bg-white rounded-xl shadow-lg p-3 transform transition-all hover:shadow-xl">
        <div class="relative">
          <font-awesome-icon icon="fa-solid fa-search" class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400" />
          <input
            v-model="searchQuery"
            type="text"
            class="w-full pl-10 pr-3 py-2 text-sm border border-gray-300 rounded-lg focus:outline-hidden focus:ring-2 focus:ring-custom-green transition-all"
            placeholder="Rechercher un document..."
          />
        </div>

        <!-- Filtres -->
        <div class="flex flex-wrap mt-2 gap-1.5">
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
              class="px-3 py-1.5 rounded-full text-xs cursor-pointer transition-all duration-200"
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

    <!-- Etat de chargement -->
    <div v-if="chargement" class="flex justify-center py-16">
      <div class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-custom-green"></div>
    </div>

    <!-- Erreur -->
    <div v-if="erreur" class="max-w-7xl mx-auto px-4 mb-4">
      <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded-lg flex justify-between items-center">
        <p>{{ erreur }}</p>
        <button @click="chargerLivres" class="text-sm underline hover:text-red-900">Reessayer</button>
      </div>
    </div>

    <!-- Grille de documents -->
    <div v-if="!chargement" class="max-w-7xl mx-auto px-4">
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
            @click="pdfSelected = { url: document.document_pdf_url, acces: document.acces }"
            class="relative h-72 overflow-hidden cursor-pointer group"
          >
            <img
              class="h-full w-full object-cover transform transition-transform duration-500 group-hover:scale-110"
              :src="document.image_couverture_url || 'https://images.unsplash.com/photo-1544716278-ca5e3f4abd8c?w=400'"
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
              <span>{{ formatDate(document.date_publication) }}</span>
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

      <!-- Etat vide -->
      <div v-if="filteredDocuments.length === 0 && !chargement" class="text-center py-16">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-20 w-20 text-gray-300 mx-auto mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
        </svg>
        <p class="text-gray-500 text-lg">Aucun document trouve</p>
      </div>
    </div>
  </div>

  <!-- Popup Ajouter un document - Formulaire complet -->
  <Transition name="modal-fade">
    <div v-if="showAddPopup" class="z-50 fixed inset-0 flex items-center justify-center">
      <div @click="showAddPopup = false" class="absolute inset-0 bg-black/60 backdrop-blur-xs"></div>

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
                class="w-full px-3 py-2 border rounded-lg focus:outline-hidden focus:ring-2 focus:ring-custom-green transition-all duration-200"
                placeholder="Saisissez le titre du document"
              />
            </div>

            <!-- Description -->
            <div class="mb-4">
              <label class="block text-gray-700 text-sm font-bold mb-2">Description</label>
              <textarea
                required
                v-model="documentForm.description"
                class="w-full px-3 py-2 border rounded-lg resize-none focus:outline-hidden focus:ring-2 focus:ring-custom-green transition-all duration-200"
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
                  class="w-full px-3 py-2 border rounded-lg bg-white focus:outline-hidden focus:ring-2 focus:ring-custom-green transition-all duration-200"
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
                  class="w-full px-3 py-2 border rounded-lg bg-white focus:outline-hidden focus:ring-2 focus:ring-custom-green transition-all duration-200"
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
                class="w-full px-3 py-2 border rounded-lg resize-none focus:outline-hidden focus:ring-2 focus:ring-custom-green transition-all duration-200"
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
                  class="w-full px-3 py-2 border rounded-lg focus:outline-hidden focus:ring-2 focus:ring-custom-green transition-all duration-200"
                />
              </div>

              <div>
                <label class="block text-gray-700 text-sm font-bold mb-2">Votre rapport avec le document</label>
                <select
                  v-model="documentForm.rapport"
                  class="w-full px-3 py-2 border rounded-lg bg-white focus:outline-hidden focus:ring-2 focus:ring-custom-green transition-all duration-200"
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
                  class="w-5 h-5 text-custom-green border-2 focus:ring-3 focus:ring-green-500 rounded transition-colors duration-200"
                  type="checkbox"
                  v-model="documentForm.consent"
                  required
                />
                <span class="ml-2 text-sm text-gray-700">
                  Moi <span class="font-bold">{{ utilisateurNom }}</span>, accepte la diffusion de cette publication
                </span>
              </label>
            </div>

            <!-- Message d'erreur soumission -->
            <div v-if="erreurSoumission" class="mb-4 bg-red-100 border border-red-400 text-red-700 px-4 py-2 rounded-lg text-sm">
              {{ erreurSoumission }}
            </div>

            <!-- Boutons -->
            <div class="flex justify-end space-x-3">
              <button
                @click="showAddPopup = false"
                type="button"
                class="px-4 py-2 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300 transition-colors duration-200 focus:outline-hidden focus:ring-2 focus:ring-gray-400"
              >
                Annuler
              </button>
              <button
                type="submit"
                :disabled="isSubmitting"
                class="px-4 py-2 bg-gradient-to-r from-custom-green to-green-600 text-white rounded-lg hover:from-custom-green hover:to-green-700 transition-all duration-200 focus:outline-hidden focus:ring-2 focus:ring-green-500 transform hover:scale-105 disabled:opacity-50 disabled:cursor-not-allowed"
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
import { documentTypes as typesBase } from '~/mocks/bibliotheques'

const documentTypes = ['Tous', ...typesBase]
import type { LivreAPI } from '~/composables/useBibliotheque'

useHead({
  title: 'Bibliotheque Numerique - UAfricas',
  meta: [
    { name: 'description', content: 'Accedez a des milliers de livres et documents africains' },
  ],
})

useAOS()

// Composable API bibliotheque
const { chargement, erreur, listerLivres, creerLivre } = useBibliotheque()

// Etat de recherche et filtrage
const searchQuery = ref('')
const selectedType = ref('Tous')

// Liste des livres charges depuis l'API
const livres = ref<LivreAPI[]>([])

// PDF viewer state
const pdfSelected = ref<{ url: string | null; acces: string | null }>({ url: null, acces: null })

// Popup et formulaire
const showAddPopup = ref(false)
const isSubmitting = ref(false)
const erreurSoumission = ref<string | null>(null)
const docImage = ref<File | null>(null)
const docFichier = ref<File | null>(null)

// Nom utilisateur pour le consentement
const utilisateurNom = ref('Utilisateur')

const documentForm = ref({
  titre: '',
  description: '',
  type: '',
  acces: '',
  auteurBiblio: '',
  datePublication: '',
  rapport: '',
  consent: false,
})

// Documents affiches (directement depuis l'API, le filtrage est cote serveur)
const filteredDocuments = computed(() => livres.value)

// Charger les livres depuis l'API
async function chargerLivres() {
  const filtres: Record<string, any> = {
    par_page: 20,
  }

  if (searchQuery.value.trim()) {
    filtres.recherche = searchQuery.value.trim()
  }

  if (selectedType.value && selectedType.value !== 'Tous' && selectedType.value !== 'Autre') {
    filtres.type_document = selectedType.value
  }

  const resultat = await listerLivres(filtres)
  if (resultat) {
    livres.value = resultat.livres
  }
}

// Surveiller les changements de recherche et de type pour recharger
let debounceTimer: ReturnType<typeof setTimeout> | null = null
watch([searchQuery, selectedType], () => {
  if (debounceTimer) clearTimeout(debounceTimer)
  debounceTimer = setTimeout(() => {
    chargerLivres()
  }, 300)
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

// Soumission reelle du formulaire via l'API
async function submitDoc() {
  if (!documentForm.value.consent) {
    alert('Veuillez accepter les conditions')
    return
  }
  if (!docFichier.value) {
    alert('Veuillez selectionner un document PDF')
    return
  }

  isSubmitting.value = true
  erreurSoumission.value = null

  const nouveauLivre = await creerLivre(
    documentForm.value,
    docImage.value,
    docFichier.value,
  )

  if (nouveauLivre) {
    livres.value.unshift(nouveauLivre)
    resetForm()
    showAddPopup.value = false
  } else {
    erreurSoumission.value = erreur.value || 'Erreur lors de la creation du document'
  }

  isSubmitting.value = false
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
    consent: false,
  }
  docImage.value = null
  docFichier.value = null
  erreurSoumission.value = null
}

function formatDate(dateString: string | null) {
  if (!dateString) return ''
  const date = new Date(dateString)
  return date.toLocaleDateString('fr-FR', { month: 'long', year: 'numeric' })
}

// Chargement initial
onMounted(() => {
  chargerLivres()
})
</script>

<style scoped>
@keyframes slideIn {
  from { transform: translateY(-20px); opacity: 0; }
  to { transform: translateY(0); opacity: 1; }
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
