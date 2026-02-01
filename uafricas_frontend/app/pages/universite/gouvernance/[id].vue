<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Loading -->
    <div v-if="loading" class="flex items-center justify-center h-screen">
      <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
    </div>

    <!-- Non trouvé -->
    <div v-else-if="!contribution" class="flex flex-col items-center justify-center h-screen">
      <svg class="w-16 h-16 text-gray-400 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
      </svg>
      <h1 class="text-2xl font-bold text-gray-700 mb-2">Contribution non trouvée</h1>
      <NuxtLink to="/universite/gouvernance" class="text-blue-600 hover:underline">
        ← Retour à la gouvernance
      </NuxtLink>
    </div>

    <!-- Contenu -->
    <template v-else>
      <!-- Hero -->
      <div class="relative h-48 md:h-56"
           :class="getTypeGradient(contribution.type)">
        <div class="absolute inset-0 bg-black/30"></div>
        <div class="absolute inset-0 flex flex-col justify-center px-4 md:px-8">
          <div class="max-w-6xl mx-auto w-full">
            <CommonBreadcrumbNav class="mb-4 text-white" />
            <div class="flex items-center gap-3 mb-2">
              <span class="px-3 py-1 bg-white/20 text-white rounded-full text-sm font-medium">
                {{ getTypeLabel(contribution.type) }}
              </span>
              <span v-if="contribution.verified" class="flex items-center text-green-300 text-sm">
                <svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"></path>
                </svg>
                Vérifié
              </span>
            </div>
            <h1 class="text-2xl md:text-3xl font-bold text-white">{{ contribution.titre }}</h1>
          </div>
        </div>
      </div>

      <!-- Contenu principal -->
      <div class="max-w-6xl mx-auto px-4 py-8">
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
          <!-- Colonne principale -->
          <div class="lg:col-span-2 space-y-6">
            <!-- Description -->
            <div class="bg-white rounded-lg shadow-md p-6">
              <h2 class="text-xl font-bold mb-4">Description</h2>
              <p class="text-gray-700 leading-relaxed">{{ contribution.description }}</p>
            </div>

            <!-- Contenu FactCheck -->
            <div v-if="contribution.type === 'factcheck' && contribution.factcheck" class="space-y-4">
              <div class="bg-red-50 rounded-lg p-6 border-l-4 border-red-500">
                <h3 class="font-bold text-red-800 mb-2">Préjugé</h3>
                <p class="font-medium text-red-900 mb-2">{{ contribution.factcheck.prejuge.titre }}</p>
                <p class="text-red-700">{{ contribution.factcheck.prejuge.description }}</p>
              </div>
              <div class="bg-green-50 rounded-lg p-6 border-l-4 border-green-500">
                <h3 class="font-bold text-green-800 mb-2">Réalité / Contre-argument</h3>
                <p class="font-medium text-green-900 mb-2">{{ contribution.factcheck.contrePrejuge.titre }}</p>
                <p class="text-green-700">{{ contribution.factcheck.contrePrejuge.description }}</p>
              </div>
            </div>

            <!-- Contenu IdeaForces -->
            <div v-if="contribution.type === 'ideaforces' && contribution.proposition" class="bg-yellow-50 rounded-lg p-6">
              <h3 class="font-bold text-yellow-800 mb-4">Proposition</h3>
              <div class="space-y-4">
                <div>
                  <p class="text-sm font-medium text-yellow-600">Objectif</p>
                  <p class="text-yellow-900">{{ contribution.proposition.objectif }}</p>
                </div>
                <div v-if="contribution.proposition.moyens?.length">
                  <p class="text-sm font-medium text-yellow-600">Moyens</p>
                  <ul class="list-disc list-inside text-yellow-800">
                    <li v-for="moyen in contribution.proposition.moyens" :key="moyen">{{ moyen }}</li>
                  </ul>
                </div>
                <div v-if="contribution.proposition.beneficiaires?.length">
                  <p class="text-sm font-medium text-yellow-600">Bénéficiaires</p>
                  <div class="flex flex-wrap gap-2">
                    <span v-for="b in contribution.proposition.beneficiaires" :key="b"
                          class="px-2 py-1 bg-yellow-200 text-yellow-800 rounded text-sm">
                      {{ b }}
                    </span>
                  </div>
                </div>
                <div v-if="contribution.proposition.impact">
                  <p class="text-sm font-medium text-yellow-600">Impact attendu</p>
                  <p class="text-yellow-900">{{ contribution.proposition.impact }}</p>
                </div>
              </div>
            </div>

            <!-- Contenu BadHabits -->
            <div v-if="contribution.type === 'badhabits' && contribution.problematique" class="bg-red-50 rounded-lg p-6">
              <h3 class="font-bold text-red-800 mb-4">Problématique signalée</h3>
              <div class="space-y-3">
                <div class="flex items-center gap-4">
                  <span class="px-3 py-1 bg-red-200 text-red-800 rounded-full text-sm">
                    {{ contribution.problematique.categorie }}
                  </span>
                  <span v-if="contribution.problematique.gravite"
                        class="px-3 py-1 rounded-full text-sm font-medium"
                        :class="getGraviteClass(contribution.problematique.gravite)">
                    Gravité: {{ contribution.problematique.gravite }}
                  </span>
                </div>
              </div>
            </div>

            <!-- Sources -->
            <div v-if="contribution.sources?.length" class="bg-white rounded-lg shadow-md p-6">
              <h3 class="font-bold mb-4">Sources</h3>
              <ul class="space-y-2">
                <li v-for="source in contribution.sources" :key="source.url">
                  <a :href="source.url" target="_blank" rel="noopener"
                     class="text-blue-600 hover:underline flex items-center">
                    <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"></path>
                    </svg>
                    {{ source.titre }}
                  </a>
                </li>
              </ul>
            </div>

            <!-- Tags -->
            <div v-if="contribution.tags?.length" class="bg-white rounded-lg shadow-md p-6">
              <h3 class="font-bold mb-4">Tags</h3>
              <div class="flex flex-wrap gap-2">
                <span v-for="tag in contribution.tags" :key="tag"
                      class="px-3 py-1 bg-gray-100 text-gray-600 rounded-full text-sm">
                  #{{ tag }}
                </span>
              </div>
            </div>
          </div>

          <!-- Sidebar -->
          <div class="space-y-6">
            <!-- Auteur -->
            <div class="bg-white rounded-lg shadow-md p-6">
              <h3 class="font-bold mb-4">Auteur</h3>
              <div class="flex items-center gap-3">
                <img v-if="contribution.auteur.photoURL"
                     :src="contribution.auteur.photoURL"
                     :alt="`${contribution.auteur.prenom} ${contribution.auteur.nom}`"
                     class="w-12 h-12 rounded-full object-cover">
                <div v-else class="w-12 h-12 rounded-full bg-gray-200 flex items-center justify-center">
                  <svg class="w-6 h-6 text-gray-400" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" clip-rule="evenodd"></path>
                  </svg>
                </div>
                <div>
                  <p class="font-medium">{{ contribution.auteur.prenom }} {{ contribution.auteur.nom }}</p>
                  <p class="text-sm text-gray-500">{{ formatDate(contribution.dateCreation) }}</p>
                </div>
              </div>
            </div>

            <!-- Localisation -->
            <div class="bg-white rounded-lg shadow-md p-6">
              <h3 class="font-bold mb-4">Localisation</h3>
              <div class="flex items-start gap-2">
                <svg class="w-5 h-5 text-gray-400 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"></path>
                </svg>
                <div>
                  <p class="font-medium">{{ contribution.localisation.pays }}</p>
                  <p v-if="contribution.localisation.ville" class="text-sm text-gray-600">
                    {{ contribution.localisation.ville }}
                    <span v-if="contribution.localisation.region">, {{ contribution.localisation.region }}</span>
                  </p>
                </div>
              </div>
            </div>

            <!-- Statistiques -->
            <div class="bg-white rounded-lg shadow-md p-6">
              <h3 class="font-bold mb-4">Statistiques</h3>
              <div class="grid grid-cols-2 gap-4">
                <div class="text-center p-3 bg-gray-50 rounded-lg">
                  <p class="text-2xl font-bold text-blue-600">{{ contribution.stats.vues }}</p>
                  <p class="text-xs text-gray-500">Vues</p>
                </div>
                <div class="text-center p-3 bg-gray-50 rounded-lg">
                  <p class="text-2xl font-bold text-red-600">{{ contribution.stats.likes }}</p>
                  <p class="text-xs text-gray-500">Likes</p>
                </div>
                <div class="text-center p-3 bg-gray-50 rounded-lg">
                  <p class="text-2xl font-bold text-green-600">{{ contribution.stats.commentaires }}</p>
                  <p class="text-xs text-gray-500">Commentaires</p>
                </div>
                <div class="text-center p-3 bg-gray-50 rounded-lg">
                  <p class="text-2xl font-bold text-purple-600">{{ contribution.stats.partages }}</p>
                  <p class="text-xs text-gray-500">Partages</p>
                </div>
              </div>
            </div>

            <!-- Actions -->
            <div class="bg-white rounded-lg shadow-md p-6">
              <div class="space-y-3">
                <button class="w-full py-2 px-4 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition flex items-center justify-center gap-2">
                  <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"></path>
                  </svg>
                  J'aime
                </button>
                <button class="w-full py-2 px-4 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 transition flex items-center justify-center gap-2">
                  <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z"></path>
                  </svg>
                  Partager
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { getContributionById, type ContributionCitoyenne } from '~/mocks/gouvernance/contributions'

const route = useRoute()
const loading = ref(true)
const contribution = ref<ContributionCitoyenne | null>(null)

const getTypeLabel = (type: string) => {
  const labels: Record<string, string> = {
    factcheck: 'FactCheck',
    badhabits: 'BadHabits',
    ideaforces: 'IdeaForces'
  }
  return labels[type] || type
}

const getTypeGradient = (type: string) => {
  const gradients: Record<string, string> = {
    factcheck: 'bg-gradient-to-r from-blue-600 to-blue-800',
    badhabits: 'bg-gradient-to-r from-red-600 to-red-800',
    ideaforces: 'bg-gradient-to-r from-yellow-500 to-orange-600'
  }
  return gradients[type] || 'bg-gradient-to-r from-gray-600 to-gray-800'
}

const getGraviteClass = (gravite: string) => {
  const classes: Record<string, string> = {
    faible: 'bg-yellow-100 text-yellow-800',
    moyenne: 'bg-orange-100 text-orange-800',
    grave: 'bg-red-100 text-red-800',
    critique: 'bg-red-600 text-white'
  }
  return classes[gravite] || 'bg-gray-100 text-gray-800'
}

const formatDate = (date: Date) => {
  return new Intl.DateTimeFormat('fr-FR', {
    day: 'numeric',
    month: 'long',
    year: 'numeric'
  }).format(new Date(date))
}

onMounted(() => {
  loading.value = true
  const id = route.params.id as string
  contribution.value = getContributionById(id) || null
  loading.value = false

  if (contribution.value) {
    useHead({
      title: `${contribution.value.titre} - Gouvernance`
    })
  }
})
</script>
