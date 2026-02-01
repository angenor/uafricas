<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section -->
    <div class="relative h-64 bg-gradient-to-r from-blue-600 to-blue-800">
      <div class="absolute inset-0 flex flex-col items-center justify-center">
        <h1 class="text-white text-4xl font-bold mb-2">FactCheck</h1>
        <p class="text-white/80 text-lg">Vérification des faits et lutte contre la désinformation</p>
      </div>
    </div>

    <!-- Contenu -->
    <div class="max-w-7xl mx-auto px-4 py-8 -mt-8">
      <!-- Header -->
      <div class="bg-white rounded-lg shadow-md p-6 mb-6">
        <CommonBreadcrumbNav class="mb-4" />
        <div class="flex items-center justify-between">
          <div>
            <p class="text-gray-600">{{ contributions.length }} vérifications publiées</p>
          </div>
          <NuxtLink to="/universite/gouvernance" class="text-blue-600 hover:text-blue-800">
            ← Retour à la gouvernance
          </NuxtLink>
        </div>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-4 gap-8">
        <!-- Filtres -->
        <div class="lg:col-span-1">
          <div class="bg-white rounded-lg shadow-md p-6 sticky top-4">
            <h3 class="text-lg font-bold mb-4">Filtres</h3>

            <!-- Recherche -->
            <div class="mb-4">
              <input v-model="recherche"
                     type="text"
                     placeholder="Rechercher..."
                     class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500">
            </div>

            <!-- Pays -->
            <div class="mb-4">
              <label class="block text-sm font-medium text-gray-700 mb-2">Pays</label>
              <select v-model="paysSelectionne" class="w-full px-3 py-2 border border-gray-300 rounded-md">
                <option value="">Tous les pays</option>
                <option v-for="pays in paysDisponibles" :key="pays" :value="pays">{{ pays }}</option>
              </select>
            </div>

            <!-- Statut vérifié -->
            <div class="mb-4">
              <label class="flex items-center">
                <input type="checkbox" v-model="seulementVerifies" class="mr-2">
                <span class="text-sm">Vérifiés uniquement</span>
              </label>
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
                    <span class="px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-sm font-medium">
                      FactCheck
                    </span>
                    <span v-if="contribution.verified" class="flex items-center text-green-600 text-sm">
                      <svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"></path>
                      </svg>
                      Vérifié
                    </span>
                  </div>
                  <h3 class="text-xl font-bold text-gray-900 mb-2">{{ contribution.titre }}</h3>
                  <p class="text-gray-600 mb-4">{{ contribution.description }}</p>

                  <!-- FactCheck spécifique -->
                  <div v-if="contribution.factcheck" class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
                    <div class="p-4 bg-red-50 rounded-lg border-l-4 border-red-500">
                      <p class="text-sm font-medium text-red-800 mb-1">Préjugé</p>
                      <p class="text-red-700">{{ contribution.factcheck.prejuge.titre }}</p>
                    </div>
                    <div class="p-4 bg-green-50 rounded-lg border-l-4 border-green-500">
                      <p class="text-sm font-medium text-green-800 mb-1">Réalité</p>
                      <p class="text-green-700">{{ contribution.factcheck.contrePrejuge.titre }}</p>
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
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                  </svg>
                  {{ contribution.stats.validations || 0 }} validations
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
import { getContributionsByType, paysAfricains, type ContributionCitoyenne } from '~/mocks/gouvernance/contributions'

useHead({
  title: 'FactCheck - Gouvernance Citoyenne'
})

const contributions = ref<ContributionCitoyenne[]>([])
const recherche = ref('')
const paysSelectionne = ref('')
const seulementVerifies = ref(false)

const paysDisponibles = computed(() => {
  const pays = new Set(contributions.value.map(c => c.localisation.pays))
  return Array.from(pays).sort()
})

const contributionsFiltrees = computed(() => {
  return contributions.value.filter(c => {
    if (recherche.value) {
      const search = recherche.value.toLowerCase()
      if (!c.titre.toLowerCase().includes(search) &&
          !c.description.toLowerCase().includes(search)) {
        return false
      }
    }
    if (paysSelectionne.value && c.localisation.pays !== paysSelectionne.value) {
      return false
    }
    if (seulementVerifies.value && !c.verified) {
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
  seulementVerifies.value = false
}

const formatDate = (date: Date) => {
  return new Intl.DateTimeFormat('fr-FR', {
    day: 'numeric',
    month: 'short',
    year: 'numeric'
  }).format(new Date(date))
}

onMounted(() => {
  contributions.value = getContributionsByType('factcheck')
})
</script>
