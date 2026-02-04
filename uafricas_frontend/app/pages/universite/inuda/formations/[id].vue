<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Loading -->
    <div v-if="loading" class="flex items-center justify-center h-screen">
      <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
    </div>

    <!-- Formation non trouvée -->
    <div v-else-if="!formation" class="flex flex-col items-center justify-center h-screen">
      <svg class="w-16 h-16 text-gray-400 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
      </svg>
      <h1 class="text-2xl font-bold text-gray-700 mb-2">Formation non trouvée</h1>
      <p class="text-gray-500 mb-4">La formation que vous recherchez n'existe pas.</p>
      <NuxtLink to="/universite/inuda/formations" class="text-blue-600 hover:underline">
        ← Retour à la liste des formations
      </NuxtLink>
    </div>

    <!-- Contenu -->
    <template v-else>
      <!-- Hero Section -->
      <div class="relative h-64 md:h-72 bg-gradient-to-r"
           :class="getTypeGradient(formation.type)">
        <div class="absolute inset-0 bg-black/30"></div>
        <div class="absolute inset-0 flex flex-col justify-center px-4 md:px-8">
          <div class="max-w-6xl mx-auto w-full">
            <CommonBreadcrumbNav class="mb-4 text-white" />
            <div class="flex items-center gap-3 mb-3">
              <span class="px-3 py-1 bg-white/20 backdrop-blur-xs text-white rounded-full text-sm font-medium">
                {{ getTypeLabel(formation.type) }}
              </span>
              <span :class="getStatutBadgeClass(formation.statut)">
                {{ getStatutLabel(formation.statut) }}
              </span>
            </div>
            <h1 class="text-3xl md:text-4xl font-bold text-white mb-2">{{ formation.titre }}</h1>
          </div>
        </div>
      </div>

      <!-- Contenu principal -->
      <div class="max-w-6xl mx-auto px-4 py-8">
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
          <!-- Colonne principale -->
          <div class="lg:col-span-2 space-y-8">
            <!-- Résumé -->
            <div class="bg-white rounded-lg shadow-md p-6">
              <h2 class="text-2xl font-bold mb-4">À propos de cette formation</h2>
              <p class="text-gray-700 leading-relaxed">{{ formation.description }}</p>
            </div>

            <!-- Formateur -->
            <div class="bg-white rounded-lg shadow-md p-6">
              <h2 class="text-2xl font-bold mb-4">Votre formateur</h2>
              <div class="flex items-start gap-4">
                <img v-if="formation.formateurPhotoURL"
                     :src="formation.formateurPhotoURL"
                     :alt="`${formation.formateurPrenom} ${formation.formateurNom}`"
                     class="w-20 h-20 rounded-full object-cover">
                <div v-else class="w-20 h-20 rounded-full bg-gray-200 flex items-center justify-center">
                  <svg class="w-10 h-10 text-gray-400" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" clip-rule="evenodd"></path>
                  </svg>
                </div>
                <div>
                  <h3 class="text-xl font-semibold">{{ formation.formateurPrenom }} {{ formation.formateurNom }}</h3>
                  <p v-if="formation.formateurBio" class="text-gray-600 mt-2">{{ formation.formateurBio }}</p>
                  <div v-if="formation.formateurQualifications" class="mt-3 flex flex-wrap gap-2">
                    <span v-for="qual in formation.formateurQualifications" :key="qual"
                          class="px-2 py-1 bg-blue-100 text-blue-800 text-xs rounded">
                      {{ qual }}
                    </span>
                  </div>
                </div>
              </div>
            </div>

            <!-- Objectifs -->
            <div v-if="formation.objectifs" class="bg-white rounded-lg shadow-md p-6">
              <h2 class="text-2xl font-bold mb-4">Objectifs de la formation</h2>
              <div class="space-y-4">
                <div v-if="formation.objectifs.generaux?.length">
                  <h3 class="font-semibold text-gray-800 mb-2">Objectifs généraux</h3>
                  <ul class="list-disc list-inside text-gray-600 space-y-1">
                    <li v-for="obj in formation.objectifs.generaux" :key="obj">{{ obj }}</li>
                  </ul>
                </div>
                <div v-if="formation.objectifs.competencesAcquises?.length">
                  <h3 class="font-semibold text-gray-800 mb-2">Compétences acquises</h3>
                  <div class="flex flex-wrap gap-2">
                    <span v-for="comp in formation.objectifs.competencesAcquises" :key="comp"
                          class="px-3 py-1 bg-green-100 text-green-800 rounded-full text-sm">
                      {{ comp }}
                    </span>
                  </div>
                </div>
              </div>
            </div>

            <!-- Modules -->
            <div v-if="formation.modalites.modulesCours?.length" class="bg-white rounded-lg shadow-md p-6">
              <h2 class="text-2xl font-bold mb-4">Programme</h2>
              <div class="space-y-3">
                <div v-for="(module, index) in formation.modalites.modulesCours" :key="module.id"
                     class="flex items-start gap-4 p-4 bg-gray-50 rounded-lg">
                  <div class="w-8 h-8 rounded-full bg-blue-600 text-white flex items-center justify-center font-bold text-sm flex-shrink-0">
                    {{ index + 1 }}
                  </div>
                  <div>
                    <h3 class="font-semibold">{{ module.titre }}</h3>
                    <p class="text-gray-600 text-sm">{{ module.description }}</p>
                    <p class="text-gray-500 text-xs mt-1">Durée: {{ module.duree }} min</p>
                  </div>
                </div>
              </div>
            </div>

            <!-- Prérequis -->
            <div v-if="formation.modalites.prerequis?.length" class="bg-white rounded-lg shadow-md p-6">
              <h2 class="text-2xl font-bold mb-4">Prérequis</h2>
              <ul class="space-y-2">
                <li v-for="prereq in formation.modalites.prerequis" :key="prereq" class="flex items-start">
                  <svg class="w-5 h-5 text-blue-500 mr-2 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"></path>
                  </svg>
                  <span class="text-gray-700">{{ prereq }}</span>
                </li>
              </ul>
            </div>
          </div>

          <!-- Sidebar -->
          <div class="space-y-6">
            <!-- Carte principale -->
            <div class="bg-white rounded-lg shadow-md p-6">
              <!-- Prix -->
              <div class="text-center mb-6">
                <p v-if="formation.tarification.gratuit" class="text-3xl font-bold text-green-600">
                  Gratuit
                </p>
                <template v-else>
                  <p class="text-3xl font-bold text-gray-900">
                    {{ formation.tarification.prix.toLocaleString() }} FCFA
                  </p>
                  <div v-if="formation.tarification.prixReduit?.length" class="mt-2">
                    <p v-for="reduc in formation.tarification.prixReduit" :key="reduc.conditions"
                       class="text-sm text-green-600">
                      {{ reduc.montant.toLocaleString() }} FCFA - {{ reduc.conditions }}
                    </p>
                  </div>
                </template>
              </div>

              <!-- Infos clés -->
              <div class="space-y-4 text-sm">
                <div class="flex justify-between py-2 border-b">
                  <span class="text-gray-500">Début</span>
                  <span class="font-medium">{{ formatDate(formation.dateDebut) }}</span>
                </div>
                <div class="flex justify-between py-2 border-b">
                  <span class="text-gray-500">Fin</span>
                  <span class="font-medium">{{ formatDate(formation.dateFin) }}</span>
                </div>
                <div class="flex justify-between py-2 border-b">
                  <span class="text-gray-500">Durée</span>
                  <span class="font-medium">
                    {{ formation.dureeEstimee.heures }}h
                    <span v-if="formation.dureeEstimee.semaines">
                      ({{ formation.dureeEstimee.semaines }} semaines)
                    </span>
                  </span>
                </div>
                <div class="flex justify-between py-2 border-b">
                  <span class="text-gray-500">Langue</span>
                  <span class="font-medium">{{ formation.modalites.langue.toUpperCase() }}</span>
                </div>
                <div class="flex justify-between py-2 border-b">
                  <span class="text-gray-500">Niveau</span>
                  <span class="font-medium">{{ getNiveauLabel(formation.modalites.niveauRequis) }}</span>
                </div>
                <div v-if="formation.capacite.maximum" class="flex justify-between py-2 border-b">
                  <span class="text-gray-500">Places</span>
                  <span class="font-medium">
                    {{ formation.capacite.inscritsActuels }}/{{ formation.capacite.maximum }}
                  </span>
                </div>
                <div v-if="formation.modalites.certificationDisponible" class="flex items-center py-2">
                  <svg class="w-5 h-5 text-yellow-500 mr-2" fill="currentColor" viewBox="0 0 20 20">
                    <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"></path>
                  </svg>
                  <span class="font-medium text-yellow-700">Certification incluse</span>
                </div>
              </div>

              <!-- Bouton d'action -->
              <button @click="sInscrire"
                      :disabled="!peutSInscrire"
                      class="w-full mt-6 py-3 rounded-lg font-medium transition"
                      :class="peutSInscrire
                        ? 'bg-custom-green text-white hover:bg-green-700'
                        : 'bg-gray-300 text-gray-500 cursor-not-allowed'">
                {{ actionLabel }}
              </button>
            </div>

            <!-- Tags -->
            <div v-if="formation.tags?.length" class="bg-white rounded-lg shadow-md p-6">
              <h3 class="font-bold mb-3">Tags</h3>
              <div class="flex flex-wrap gap-2">
                <span v-for="tag in formation.tags" :key="tag"
                      class="px-3 py-1 bg-gray-100 text-gray-600 rounded-full text-sm">
                  #{{ tag }}
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { getFormationById, type Formation } from '~/mocks/inuda/formations'

const route = useRoute()
const loading = ref(true)
const formation = ref<Formation | null>(null)

const peutSInscrire = computed(() => {
  if (!formation.value) return false
  return formation.value.statut === 'inscriptions_ouvertes' &&
         (!formation.value.capacite.maximum ||
          formation.value.capacite.inscritsActuels < formation.value.capacite.maximum)
})

const actionLabel = computed(() => {
  if (!formation.value) return ''
  if (formation.value.statut === 'complet') return 'Complet'
  if (formation.value.statut === 'termine') return 'Terminé'
  if (formation.value.statut === 'en_cours') return 'Formation en cours'
  if (formation.value.statut === 'inscriptions_ouvertes') return "S'inscrire maintenant"
  return 'Prochainement'
})

const sInscrire = () => {
  alert('Cette fonctionnalité sera disponible prochainement.')
}

const getTypeLabel = (type: string) => {
  const labels: Record<string, string> = { mooc: 'MOOC', clom: 'CLOM', atelier: 'Atelier', concertation: 'Concertation' }
  return labels[type] || type.toUpperCase()
}

const getTypeGradient = (type: string) => {
  const gradients: Record<string, string> = {
    mooc: 'from-blue-600 to-blue-800',
    clom: 'from-purple-600 to-purple-800',
    atelier: 'from-green-600 to-green-800',
    concertation: 'from-orange-600 to-orange-800'
  }
  return gradients[type] || 'from-gray-600 to-gray-800'
}

const getStatutLabel = (statut: string) => {
  const labels: Record<string, string> = {
    brouillon: 'Brouillon', programme: 'Programmé', inscriptions_ouvertes: 'Inscriptions ouvertes',
    complet: 'Complet', en_cours: 'En cours', termine: 'Terminé', annule: 'Annulé', archive: 'Archivé'
  }
  return labels[statut] || statut
}

const getStatutBadgeClass = (statut: string) => {
  const classes: Record<string, string> = {
    inscriptions_ouvertes: 'px-3 py-1 bg-green-500 text-white rounded-full text-sm font-medium',
    en_cours: 'px-3 py-1 bg-blue-500 text-white rounded-full text-sm font-medium',
    termine: 'px-3 py-1 bg-gray-500 text-white rounded-full text-sm font-medium',
    complet: 'px-3 py-1 bg-red-500 text-white rounded-full text-sm font-medium'
  }
  return classes[statut] || 'px-3 py-1 bg-gray-500 text-white rounded-full text-sm font-medium'
}

const getNiveauLabel = (niveau: string) => {
  const labels: Record<string, string> = {
    debutant: 'Débutant', intermediaire: 'Intermédiaire', avance: 'Avancé', tous_niveaux: 'Tous niveaux'
  }
  return labels[niveau] || niveau
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
  formation.value = getFormationById(id) || null
  loading.value = false

  if (formation.value) {
    useHead({
      title: `${formation.value.titre} - INUDA`
    })
  }
})
</script>
