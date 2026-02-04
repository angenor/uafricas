<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section -->
    <div
      class="relative h-80 bg-cover bg-center z-0"
      style="background-image: url('https://images.unsplash.com/photo-1507842217343-583bb7270b66?ixlib=rb-1.2.1&auto=format&fit=crop&w=1900&q=80');">
      <div class="absolute inset-0 bg-gradient-to-r from-custom-chocolat/90 to-black/70"></div>

      <div class="absolute inset-0 flex flex-col items-center justify-center mt-5">
        <h1 class="text-white text-4xl md:text-5xl font-bold mb-4 animate-title">
          Formations Disponibles
        </h1>
        <div class="h-1 w-24 bg-custom-green rounded animate-line"></div>
        <p class="text-white text-xl md:text-2xl mt-4 animate-subtitle">
          MOOC, CLOM, ateliers et concertations pour développer vos compétences
        </p>
      </div>
    </div>

    <!-- Contenu principal -->
    <div class="max-w-6xl mx-auto px-4 relative -top-10">
      <!-- Header avec breadcrumb -->
      <div class="bg-white shadow-xs rounded-t-lg">
        <div class="px-4 py-6">
          <CommonBreadcrumbNav class="mb-4" />

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
              <div class="bg-white rounded-lg shadow-md p-6 mb-6">
                <h3 class="text-lg font-bold mb-4">Rechercher</h3>
                <input v-model="recherche"
                       type="text"
                       placeholder="Rechercher une formation..."
                       class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-3 focus:ring-blue-500 focus:border-blue-500">
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
                {{ formationsFiltrees.length }} formation{{ formationsFiltrees.length > 1 ? 's' : '' }} trouvée{{ formationsFiltrees.length > 1 ? 's' : '' }}
              </p>
              <div class="flex items-center gap-4">
                <!-- Tri -->
                <select v-model="triSelectionne"
                        class="px-3 py-2 border border-gray-300 rounded-md text-sm">
                  <option value="date">Trier par date</option>
                  <option value="titre">Trier par titre</option>
                  <option value="popularite">Trier par popularité</option>
                  <option value="prix">Trier par prix</option>
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

            <div v-if="loading" class="text-center py-12">
              <div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
              <p class="mt-4 text-gray-600">Chargement des formations...</p>
            </div>

            <div v-else-if="formationsFiltrees.length === 0" class="text-center py-12">
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
                v-for="formation in formationsFiltrees"
                :key="formation.id"
                :formation="formation"
                @click="voirDetail"
                @inscrire="ouvrirInscription" />
            </div>

            <!-- Vue liste -->
            <div v-else class="space-y-4">
              <div v-for="formation in formationsFiltrees"
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
                      <span v-if="formation.tarification.gratuit" class="text-green-600 font-bold text-sm">
                        Gratuit
                      </span>
                      <span v-else class="text-gray-700 font-bold text-sm">
                        {{ formation.tarification.prix.toLocaleString() }} FCFA
                      </span>
                    </div>

                    <h3 class="text-xl font-bold text-gray-900 mb-2">{{ formation.titre }}</h3>
                    <p class="text-gray-600 mb-3 line-clamp-2">{{ formation.resume || formation.description }}</p>

                    <div class="flex items-center text-sm text-gray-500 gap-4 mb-4">
                      <span>{{ formation.formateurPrenom }} {{ formation.formateurNom }}</span>
                      <span>{{ formatDate(formation.dateDebut) }}</span>
                      <span>{{ formation.dureeEstimee.heures }}h</span>
                      <span class="px-2 py-1 bg-gray-100 text-gray-600 rounded">
                        {{ formation.modalites.langue.toUpperCase() }}
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
          <p class="text-sm text-gray-500 mb-4">
            Cette fonctionnalité sera disponible prochainement. Vous pourrez vous inscrire directement depuis la plateforme.
          </p>
          <button @click="formationSelectionnee = null"
                  class="w-full py-2 bg-custom-chocolat text-white rounded-md hover:bg-custom-chocolat/90 transition">
            Fermer
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { rechercherFormations, type Formation } from '~/mocks/inuda/formations'

useHead({
  title: 'Formations - INUDA'
})

const loading = ref(true)
const formations = ref<Formation[]>([])
const recherche = ref('')
const filtres = ref<{ types?: string[]; statuts?: string[]; gratuit?: boolean | null }>({})
const triSelectionne = ref('date')
const vueMode = ref('grid')
const formationSelectionnee = ref<Formation | null>(null)

// Formations filtrées et triées
const formationsFiltrees = computed(() => {
  let resultats = [...formations.value]

  // Appliquer le tri
  switch (triSelectionne.value) {
    case 'titre':
      resultats.sort((a, b) => a.titre.localeCompare(b.titre))
      break
    case 'popularite':
      resultats.sort((a, b) => (b.stats?.inscriptions || 0) - (a.stats?.inscriptions || 0))
      break
    case 'prix':
      resultats.sort((a, b) => {
        const prixA = a.tarification.gratuit ? 0 : a.tarification.prix
        const prixB = b.tarification.gratuit ? 0 : b.tarification.prix
        return prixA - prixB
      })
      break
    case 'date':
    default:
      resultats.sort((a, b) => new Date(a.dateDebut).getTime() - new Date(b.dateDebut).getTime())
  }

  return resultats
})

const voirDetail = (formation: Formation) => {
  navigateTo(`/universite/inuda/formations/${formation.id}`)
}

const ouvrirInscription = (formation: Formation) => {
  formationSelectionnee.value = formation
}

const appliquerFiltres = (nouveauxFiltres: typeof filtres.value) => {
  filtres.value = nouveauxFiltres
  chargerFormations()
}

const reinitialiserFiltres = () => {
  recherche.value = ''
  filtres.value = {}
  chargerFormations()
}

const chargerFormations = () => {
  loading.value = true
  try {
    formations.value = rechercherFormations(recherche.value, filtres.value)
  } catch (error) {
    console.error('Erreur lors du chargement des formations:', error)
  } finally {
    loading.value = false
  }
}

// Fonctions utilitaires
const getTypeLabel = (type: string) => {
  const labels: Record<string, string> = { mooc: 'MOOC', clom: 'CLOM', atelier: 'Atelier', concertation: 'Concertation' }
  return labels[type] || type.toUpperCase()
}

const getTypeClasses = (type: string) => {
  const classes: Record<string, string> = {
    mooc: 'bg-blue-100 text-blue-800',
    clom: 'bg-purple-100 text-purple-800',
    atelier: 'bg-green-100 text-green-800',
    concertation: 'bg-orange-100 text-orange-800'
  }
  return classes[type] || 'bg-gray-100 text-gray-800'
}

const peutSInscrire = (formation: Formation) => {
  return formation.statut === 'inscriptions_ouvertes' &&
         (!formation.capacite.maximum || formation.capacite.inscritsActuels < formation.capacite.maximum)
}

const getActionLabel = (formation: Formation) => {
  if (formation.statut === 'complet') return 'Complet'
  if (formation.statut === 'termine') return 'Terminé'
  if (formation.statut === 'en_cours') return 'En cours'
  if (formation.statut === 'inscriptions_ouvertes') return "S'inscrire"
  return 'Prochainement'
}

const formatDate = (date: Date) => {
  return new Intl.DateTimeFormat('fr-FR', {
    day: 'numeric',
    month: 'short',
    year: 'numeric'
  }).format(new Date(date))
}

// Recherche réactive
watch(recherche, () => {
  chargerFormations()
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

@keyframes fadeInUp {
  from { opacity: 0; transform: translateY(30px); }
  to { opacity: 1; transform: translateY(0); }
}

@keyframes expandWidth {
  from { width: 0; }
  to { width: 6rem; }
}

@keyframes fadeInDelay {
  0%, 40% { opacity: 0; transform: translateY(20px); }
  100% { opacity: 1; transform: translateY(0); }
}

.animate-title { animation: fadeInUp 1s ease-out 0.3s both; }
.animate-line { animation: expandWidth 1s ease-out 1s both; }
.animate-subtitle { animation: fadeInDelay 1.5s ease-out 0.8s both; }
</style>
