<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section -->
    <div class="relative h-64 bg-gradient-to-r from-yellow-500 to-orange-600">
      <div class="absolute inset-0 flex flex-col items-center justify-center">
        <h1 class="text-white text-4xl font-bold mb-2">IdeaForces</h1>
        <p class="text-white/80 text-lg">Propositions d'idées et forces positives pour l'Afrique</p>
      </div>
    </div>

    <!-- Contenu -->
    <div class="max-w-7xl mx-auto px-4 py-8 -mt-8">
      <!-- Header -->
      <div class="bg-white rounded-lg shadow-md p-6 mb-6">
        <CommonBreadcrumbNav class="mb-4" />
        <div class="flex items-center justify-between">
          <div>
            <p class="text-gray-600">{{ contributions.length }} idées publiées</p>
          </div>
          <NuxtLink to="/universite/gouvernance" class="text-yellow-600 hover:text-yellow-800">
            ← Retour à la gouvernance
          </NuxtLink>
        </div>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-4 gap-8">
        <!-- Filtres -->
        <div class="lg:col-span-1">
          <div class="bg-white rounded-lg shadow-md p-6 sticky top-4">
            <h3 class="text-lg font-bold mb-4">Filtres</h3>

            <div class="mb-4">
              <input v-model="recherche"
                     type="text"
                     placeholder="Rechercher..."
                     class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-3 focus:ring-yellow-500 focus:border-yellow-500">
            </div>

            <div class="mb-4">
              <label class="block text-sm font-medium text-gray-700 mb-2">Pays</label>
              <select v-model="paysSelectionne" class="w-full px-3 py-2 border border-gray-300 rounded-md">
                <option value="">Tous les pays</option>
                <option v-for="pays in paysDisponibles" :key="pays" :value="pays">{{ pays }}</option>
              </select>
            </div>

            <button @click="reinitialiser" class="w-full py-2 text-sm text-gray-600 border border-gray-300 rounded-md hover:bg-gray-50">
              Réinitialiser
            </button>
          </div>
        </div>

        <!-- Liste -->
        <div class="lg:col-span-3">
          <div v-if="contributionsFiltrees.length === 0" class="text-center py-12 bg-white rounded-lg shadow-md">
            <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
            </svg>
            <p class="mt-4 text-gray-600">Aucun résultat trouvé</p>
          </div>

          <div v-else class="space-y-6">
            <div v-for="contribution in contributionsFiltrees" :key="contribution.id"
                 class="bg-white rounded-lg shadow-md p-6 hover:shadow-lg transition cursor-pointer"
                 @click="voirDetail(contribution)">
              <div class="flex items-start justify-between mb-4">
                <div class="flex-1">
                  <div class="flex items-center gap-2 mb-2">
                    <span class="px-3 py-1 bg-yellow-100 text-yellow-800 rounded-full text-sm font-medium">
                      IdeaForces
                    </span>
                  </div>
                  <h3 class="text-xl font-bold text-gray-900 mb-2">{{ contribution.titre }}</h3>
                  <p class="text-gray-600 mb-4">{{ contribution.description }}</p>

                  <!-- Proposition -->
                  <div v-if="contribution.proposition" class="mb-4 p-4 bg-yellow-50 rounded-lg">
                    <p class="text-sm font-medium text-yellow-800 mb-2">Objectif</p>
                    <p class="text-yellow-700 mb-3">{{ contribution.proposition.objectif }}</p>

                    <div v-if="contribution.proposition.beneficiaires?.length" class="mt-2">
                      <p class="text-xs text-yellow-600 mb-1">Bénéficiaires :</p>
                      <div class="flex flex-wrap gap-1">
                        <span v-for="b in contribution.proposition.beneficiaires" :key="b"
                              class="px-2 py-0.5 bg-yellow-200 text-yellow-800 rounded text-xs">
                          {{ b }}
                        </span>
                      </div>
                    </div>
                  </div>

                  <!-- Métadonnées -->
                  <div class="flex items-center gap-4 text-sm text-gray-500">
                    <span class="flex items-center">
                      <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
                      </svg>
                      {{ contribution.auteur.prenom }} {{ contribution.auteur.nom }}
                    </span>
                    <span class="flex items-center">
                      <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"></path>
                      </svg>
                      {{ contribution.localisation.pays }}
                    </span>
                    <span>{{ formatDate(contribution.dateCreation) }}</span>
                  </div>
                </div>
              </div>

              <!-- Stats -->
              <div class="flex items-center gap-6 pt-4 border-t border-gray-100 text-sm text-gray-500">
                <span class="flex items-center">
                  <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                  </svg>
                  {{ contribution.stats.vues }} vues
                </span>
                <span class="flex items-center">
                  <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"></path>
                  </svg>
                  {{ contribution.stats.likes }} likes
                </span>
                <span class="flex items-center">
                  <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 10h4.764a2 2 0 011.789 2.894l-3.5 7A2 2 0 0115.263 21h-4.017c-.163 0-.326-.02-.485-.06L7 20m7-10V5a2 2 0 00-2-2h-.095c-.5 0-.905.405-.905.905 0 .714-.211 1.412-.608 2.006L7 11v9m7-10h-2M7 20H5a2 2 0 01-2-2v-6a2 2 0 012-2h2.5"></path>
                  </svg>
                  {{ contribution.stats.soutiens || 0 }} soutiens
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { getContributionsByType, type ContributionCitoyenne } from '~/mocks/gouvernance/contributions'

useHead({
  title: 'IdeaForces - Gouvernance Citoyenne'
})

const contributions = ref<ContributionCitoyenne[]>([])
const recherche = ref('')
const paysSelectionne = ref('')

const paysDisponibles = computed(() => {
  const pays = new Set(contributions.value.map(c => c.localisation.pays))
  return Array.from(pays).sort()
})

const contributionsFiltrees = computed(() => {
  return contributions.value.filter(c => {
    if (recherche.value) {
      const search = recherche.value.toLowerCase()
      if (!c.titre.toLowerCase().includes(search) && !c.description.toLowerCase().includes(search)) {
        return false
      }
    }
    if (paysSelectionne.value && c.localisation.pays !== paysSelectionne.value) {
      return false
    }
    return true
  })
})

const voirDetail = (contribution: ContributionCitoyenne) => {
  navigateTo(`/universite/gouvernance/${contribution.id}`)
}

const reinitialiser = () => {
  recherche.value = ''
  paysSelectionne.value = ''
}

const formatDate = (date: Date) => {
  return new Intl.DateTimeFormat('fr-FR', { day: 'numeric', month: 'short', year: 'numeric' }).format(new Date(date))
}

onMounted(() => {
  contributions.value = getContributionsByType('ideaforces')
})
</script>
