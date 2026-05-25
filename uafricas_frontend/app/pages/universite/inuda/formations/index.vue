<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section (compact, titre ↔ description au survol) -->
    <div
      class="group relative bg-cover bg-center"
      style="background-image: url('https://images.unsplash.com/photo-1507842217343-583bb7270b66?ixlib=rb-1.2.1&auto=format&fit=crop&w=1900&q=80')">
      <div class="absolute inset-0 bg-gradient-to-r from-custom-chocolat/90 to-black/70"></div>

      <div class="relative max-w-4xl mx-auto px-4 pt-16 pb-6 text-center select-none">
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12">
          <h1 class="absolute inset-0 flex items-center justify-center text-white text-2xl md:text-4xl font-bold transition-opacity duration-300 group-hover:opacity-0">
            Formations Disponibles
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            MOOC, CLOM, ateliers et concertations pour développer vos compétences
          </p>
        </div>
      </div>
    </div>

    <!-- Contenu principal -->
    <div class="max-w-6xl mx-auto px-4 mt-6">
      <!-- Header avec breadcrumb -->
      <div class="bg-white shadow-xs rounded-t-lg">
        <div class="px-4 py-6">
          <CommonBreadcrumbNav :custom-breadcrumbs="breadcrumbs" class="mb-4" />

          <div class="flex items-center justify-between">
            <div>
              <p class="text-gray-600">
                Découvrez nos formations pour développer vos compétences
              </p>
            </div>
            <NuxtLink to="/universite/inuda"
                      class="text-custom-chocolat hover:text-custom-green font-medium">
              ← Retour
            </NuxtLink>
          </div>
        </div>
      </div>

      <div class="max-w-7xl mx-auto px-4 py-8">
        <div class="grid grid-cols-1 lg:grid-cols-4 gap-8">
          <!-- Filtres -->
          <div class="lg:col-span-1">
            <div class="sticky top-4">
              <!-- Barre de recherche -->
              <div class="bg-white rounded-lg shadow-md p-3 mb-6">
                <div class="relative">
                  <font-awesome-icon icon="fa-solid fa-magnifying-glass"
                                     class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 text-sm" />
                  <input v-model="recherche"
                         type="text"
                         placeholder="Rechercher une formation..."
                         class="w-full pl-9 pr-3 py-2 text-sm border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500">
                </div>
              </div>

              <!-- Composant de filtres -->
              <UniversiteInudaFiltresFormations @filtres-changes="appliquerFiltres" />
            </div>
          </div>

          <!-- Liste des formations -->
          <div class="lg:col-span-3">
            <!-- Résumé des résultats -->
            <div class="flex items-center justify-between mb-6">
              <p class="text-gray-600">
                {{ formations.length }} formation{{ formations.length > 1 ? 's' : '' }} trouvée{{ formations.length > 1 ? 's' : '' }}
              </p>
              <div class="flex items-center gap-4">
                <!-- Tri -->
                <select v-model="triSelectionne"
                        class="px-3 py-2 border border-gray-300 rounded-md text-sm">
                  <option value="date">Trier par date</option>
                  <option value="titre">Trier par titre</option>
                </select>

                <!-- Vue -->
                <div class="flex border border-gray-300 rounded-md">
                  <button @click="vueMode = 'grid'"
                          :class="vueMode === 'grid' ? 'bg-blue-600 text-white' : 'bg-white text-gray-600'"
                          class="px-3 py-2 text-sm rounded-l-md border-r border-gray-300">
                    <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                      <path d="M5 3a2 2 0 00-2 2v2a2 2 0 002 2h2a2 2 0 002-2V5a2 2 0 00-2-2H5zM5 11a2 2 0 00-2 2v2a2 2 0 002 2h2a2 2 0 002-2v-2a2 2 0 00-2-2H5zM11 5a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V5zM11 13a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"></path>
                    </svg>
                  </button>
                  <button @click="vueMode = 'list'"
                          :class="vueMode === 'list' ? 'bg-blue-600 text-white' : 'bg-white text-gray-600'"
                          class="px-3 py-2 text-sm rounded-r-md">
                    <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M3 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd"></path>
                    </svg>
                  </button>
                </div>
              </div>
            </div>

            <!-- Erreur -->
            <div v-if="erreur" class="bg-red-50 border border-red-200 rounded-lg p-4 mb-6">
              <p class="text-red-700">{{ erreur }}</p>
              <button @click="chargerFormations" class="mt-2 text-red-600 underline text-sm">Réessayer</button>
            </div>

            <div v-if="chargement" class="text-center py-12">
              <div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
              <p class="mt-4 text-gray-600">Chargement des formations...</p>
            </div>

            <div v-else-if="formationsTriees.length === 0" class="text-center py-12">
              <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
              </svg>
              <p class="mt-4 text-gray-600">Aucune formation ne correspond à vos critères</p>
              <button @click="reinitialiserFiltres"
                      class="mt-4 px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700">
                Réinitialiser les filtres
              </button>
            </div>

            <!-- Vue grille -->
            <div v-else-if="vueMode === 'grid'"
                 class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6">
              <UniversiteInudaFormationCard
                v-for="formation in formationsTriees"
                :key="formation.id"
                :formation="formation"
                @click="voirDetail"
                @inscrire="ouvrirInscription" />
            </div>

            <!-- Vue liste -->
            <div v-else class="space-y-4">
              <div v-for="formation in formationsTriees"
                   :key="formation.id"
                   class="bg-white rounded-lg shadow-md p-6 hover:shadow-lg transition-shadow cursor-pointer"
                   @click="voirDetail(formation)">
                <div class="flex items-start justify-between">
                  <div class="flex-1">
                    <div class="flex items-center gap-3 mb-2">
                      <span class="px-3 py-1 rounded-full text-sm font-medium"
                            :class="getTypeClasses(formation.type)">
                        {{ getTypeLabel(formation.type) }}
                      </span>
                      <span :class="getStatutColor(formation.statut)"
                            class="w-2 h-2 rounded-full inline-block"></span>
                      <span class="text-gray-500 text-sm">{{ getStatutLabel(formation.statut) }}</span>
                    </div>

                    <h3 class="text-xl font-bold text-gray-900 mb-2">{{ formation.titre }}</h3>
                    <p class="text-gray-600 mb-3 line-clamp-2">{{ formation.description }}</p>

                    <div class="flex items-center text-sm text-gray-500 gap-4 mb-4">
                      <span>{{ formation.formateur.prenom }} {{ formation.formateur.nom }}</span>
                      <span>{{ formatDateCourt(formation.date_heure_debut) }}</span>
                      <span class="px-2 py-1 bg-gray-100 text-gray-600 rounded">
                        {{ formation.langue }}
                      </span>
                    </div>

                    <!-- Boutons d'action -->
                    <div class="flex items-center gap-3">
                      <button @click.stop="voirDetail(formation)"
                              class="inline-flex items-center px-4 py-2 text-sm font-medium text-custom-chocolat bg-white border border-custom-chocolat rounded-md hover:bg-custom-chocolat hover:text-white transition-colors">
                        En savoir plus
                      </button>
                      <button @click.stop="ouvrirInscription(formation)"
                              :disabled="!peutSInscrire(formation)"
                              class="px-4 py-2 rounded-md font-medium transition"
                              :class="peutSInscrire(formation)
                                ? 'bg-custom-green text-white hover:bg-green-700'
                                : 'bg-gray-300 text-gray-500 cursor-not-allowed'">
                        {{ getActionLabel(formation) }}
                      </button>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- Pagination -->
            <div v-if="totalPages > 1" class="flex justify-center mt-8 gap-2">
              <button v-for="p in totalPages" :key="p"
                      @click="allerPage(p)"
                      :class="p === pageActuelle ? 'bg-blue-600 text-white' : 'bg-white text-gray-700 hover:bg-gray-100'"
                      class="px-4 py-2 border border-gray-300 rounded-md text-sm">
                {{ p }}
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Modal d'inscription -->
    <div v-if="formationSelectionnee" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
      <div class="bg-white rounded-lg max-w-lg w-full max-h-[90vh] overflow-y-auto">
        <div class="p-6">
          <div class="flex justify-between items-start mb-4">
            <h2 class="text-xl font-bold">S'inscrire à la formation</h2>
            <button @click="formationSelectionnee = null" class="text-gray-500 hover:text-gray-700">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
              </svg>
            </button>
          </div>
          <p class="text-gray-600 mb-4">
            Vous souhaitez vous inscrire à : <strong>{{ formationSelectionnee.titre }}</strong>
          </p>
          <div class="flex gap-3">
            <button @click="confirmerInscription"
                    :disabled="inscriptionEnCours"
                    class="flex-1 py-2 bg-custom-green text-white rounded-md hover:bg-green-700 transition disabled:opacity-50">
              {{ inscriptionEnCours ? 'Inscription...' : 'Confirmer' }}
            </button>
            <button @click="formationSelectionnee = null"
                    class="flex-1 py-2 bg-gray-200 text-gray-700 rounded-md hover:bg-gray-300 transition">
              Annuler
            </button>
          </div>
          <p v-if="messageInscription" class="mt-3 text-sm" :class="inscriptionReussie ? 'text-green-600' : 'text-red-600'">
            {{ messageInscription }}
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  useFormations,
  type FormationAPI,
  type FormationFiltres,
  getTypeLabel,
  getTypeClasses,
  getStatutLabel,
  getStatutColor,
  getActionLabel,
  peutSInscrire,
  formatDateCourt,
} from '~/composables/useFormations'

useHead({
  title: 'Formations - INUDA'
})

const breadcrumbs = [
  { label: 'Université', to: '/universite' },
  { label: 'INUDA', to: '/universite/inuda' },
  { label: 'Formations', to: undefined }
]

const { chargement, erreur, listerFormations, inscrireFormation } = useFormations()

const formations = ref<FormationAPI[]>([])
const recherche = ref('')
const filtresActifs = ref<{ types?: string[]; statuts?: string[] }>({})
const triSelectionne = ref('date')
const vueMode = ref('grid')
const formationSelectionnee = ref<FormationAPI | null>(null)
const pageActuelle = ref(1)
const totalPages = ref(1)
const inscriptionEnCours = ref(false)
const inscriptionReussie = ref(false)
const messageInscription = ref('')

// Formations triees cote client
const formationsTriees = computed(() => {
  let resultats = [...formations.value]

  // Filtrer par type (cote client si plusieurs types selectionnes)
  if (filtresActifs.value.types?.length) {
    resultats = resultats.filter(f => f.type && filtresActifs.value.types!.includes(f.type))
  }

  // Filtrer par statut
  if (filtresActifs.value.statuts?.length) {
    resultats = resultats.filter(f => filtresActifs.value.statuts!.includes(f.statut))
  }

  // Appliquer le tri
  switch (triSelectionne.value) {
    case 'titre':
      resultats.sort((a, b) => a.titre.localeCompare(b.titre))
      break
    case 'date':
    default:
      resultats.sort((a, b) => new Date(a.date_heure_debut).getTime() - new Date(b.date_heure_debut).getTime())
  }

  return resultats
})

const voirDetail = (formation: FormationAPI) => {
  navigateTo(`/universite/inuda/formations/${formation.id}`)
}

const ouvrirInscription = (formation: FormationAPI) => {
  formationSelectionnee.value = formation
  messageInscription.value = ''
  inscriptionReussie.value = false
}

const confirmerInscription = async () => {
  if (!formationSelectionnee.value) return
  inscriptionEnCours.value = true
  messageInscription.value = ''

  const succes = await inscrireFormation(formationSelectionnee.value.id)

  if (succes) {
    inscriptionReussie.value = true
    messageInscription.value = 'Inscription réussie !'
    // Recharger pour mettre a jour le compteur
    await chargerFormations()
  } else {
    messageInscription.value = 'Erreur lors de l\'inscription. Vérifiez que vous êtes connecté.'
  }

  inscriptionEnCours.value = false
}

const appliquerFiltres = (nouveauxFiltres: { types: string[]; statuts: string[]; gratuit: boolean | null }) => {
  filtresActifs.value = {
    types: nouveauxFiltres.types,
    statuts: nouveauxFiltres.statuts,
  }
}

const reinitialiserFiltres = () => {
  recherche.value = ''
  filtresActifs.value = {}
  chargerFormations()
}

const allerPage = (page: number) => {
  pageActuelle.value = page
  chargerFormations()
}

const chargerFormations = async () => {
  const filtres: FormationFiltres = {
    page: pageActuelle.value,
    par_page: 50,
  }
  if (recherche.value) filtres.recherche = recherche.value

  const data = await listerFormations(filtres)
  if (data) {
    formations.value = data.formations
    totalPages.value = data.total_pages
  }
}

// Recherche reactive avec debounce
let debounceTimer: ReturnType<typeof setTimeout>
watch(recherche, () => {
  clearTimeout(debounceTimer)
  debounceTimer = setTimeout(() => {
    pageActuelle.value = 1
    chargerFormations()
  }, 300)
})

onMounted(() => {
  chargerFormations()
})
</script>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
