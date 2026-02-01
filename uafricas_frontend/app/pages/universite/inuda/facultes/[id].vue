<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Loading -->
    <div v-if="loading" class="flex items-center justify-center h-screen">
      <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
    </div>

    <!-- Faculté non trouvée -->
    <div v-else-if="!faculte" class="flex flex-col items-center justify-center h-screen">
      <svg class="w-16 h-16 text-gray-400 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
      </svg>
      <h1 class="text-2xl font-bold text-gray-700 mb-2">Faculté non trouvée</h1>
      <p class="text-gray-500 mb-4">La faculté que vous recherchez n'existe pas.</p>
      <NuxtLink to="/universite/inuda/facultes" class="text-blue-600 hover:underline">
        ← Retour à la liste des facultés
      </NuxtLink>
    </div>

    <!-- Contenu -->
    <template v-else>
      <!-- Hero Section -->
      <div class="relative h-64 md:h-80 bg-cover bg-center"
           :style="{ backgroundImage: faculte.imageCouverture ? `url(${faculte.imageCouverture})` : 'linear-gradient(to right, #3B82F6, #6366F1)' }">
        <div class="absolute inset-0 bg-gradient-to-r from-black/70 to-black/50"></div>
        <div class="absolute inset-0 flex flex-col justify-center px-4 md:px-8">
          <div class="max-w-6xl mx-auto w-full">
            <CommonBreadcrumbNav class="mb-4 text-white" />
            <h1 class="text-3xl md:text-4xl font-bold text-white mb-2">{{ faculte.titre }}</h1>
            <p class="text-xl text-white/80">{{ faculte.acronyme }}</p>
            <div v-if="faculte.accepteNouveauxInscrits" class="mt-4">
              <span class="px-4 py-2 bg-green-500 text-white rounded-full text-sm font-medium">
                Inscriptions ouvertes
              </span>
            </div>
          </div>
        </div>
      </div>

      <!-- Contenu principal -->
      <div class="max-w-6xl mx-auto px-4 py-8">
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
          <!-- Colonne principale -->
          <div class="lg:col-span-2 space-y-8">
            <!-- Description -->
            <div class="bg-white rounded-lg shadow-md p-6">
              <h2 class="text-2xl font-bold mb-4">À propos</h2>
              <p class="text-gray-700 leading-relaxed">{{ faculte.description }}</p>
            </div>

            <!-- Programmes -->
            <div class="bg-white rounded-lg shadow-md p-6">
              <h2 class="text-2xl font-bold mb-4">Programmes proposés</h2>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div v-if="faculte.programmesResume.licence.length > 0">
                  <h3 class="font-semibold text-blue-600 mb-2">Licences</h3>
                  <ul class="list-disc list-inside text-gray-600 space-y-1">
                    <li v-for="prog in faculte.programmesResume.licence" :key="prog">{{ prog }}</li>
                  </ul>
                </div>
                <div v-if="faculte.programmesResume.master.length > 0">
                  <h3 class="font-semibold text-purple-600 mb-2">Masters</h3>
                  <ul class="list-disc list-inside text-gray-600 space-y-1">
                    <li v-for="prog in faculte.programmesResume.master" :key="prog">{{ prog }}</li>
                  </ul>
                </div>
                <div v-if="faculte.programmesResume.doctorat.length > 0">
                  <h3 class="font-semibold text-red-600 mb-2">Doctorats</h3>
                  <ul class="list-disc list-inside text-gray-600 space-y-1">
                    <li v-for="prog in faculte.programmesResume.doctorat" :key="prog">{{ prog }}</li>
                  </ul>
                </div>
                <div v-if="faculte.programmesResume.certificats.length > 0">
                  <h3 class="font-semibold text-green-600 mb-2">Certificats</h3>
                  <ul class="list-disc list-inside text-gray-600 space-y-1">
                    <li v-for="prog in faculte.programmesResume.certificats" :key="prog">{{ prog }}</li>
                  </ul>
                </div>
              </div>
            </div>

            <!-- Points forts -->
            <div v-if="faculte.pointsForts.length > 0" class="bg-white rounded-lg shadow-md p-6">
              <h2 class="text-2xl font-bold mb-4">Points forts</h2>
              <ul class="space-y-2">
                <li v-for="point in faculte.pointsForts" :key="point" class="flex items-start">
                  <svg class="w-5 h-5 text-green-500 mr-2 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"></path>
                  </svg>
                  <span class="text-gray-700">{{ point }}</span>
                </li>
              </ul>
            </div>

            <!-- Domaines d'études -->
            <div class="bg-white rounded-lg shadow-md p-6">
              <h2 class="text-2xl font-bold mb-4">Domaines d'études</h2>
              <div class="flex flex-wrap gap-2">
                <span v-for="domaine in faculte.domainesEtudes" :key="domaine"
                      class="px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-sm">
                  {{ domaine }}
                </span>
              </div>
            </div>
          </div>

          <!-- Sidebar -->
          <div class="space-y-6">
            <!-- École partenaire -->
            <div class="bg-white rounded-lg shadow-md p-6">
              <h3 class="text-lg font-bold mb-4">École partenaire</h3>
              <div class="space-y-3">
                <p class="font-medium">{{ faculte.ecolePartenaire.nom }}</p>
                <p class="text-gray-600">
                  {{ faculte.ecolePartenaire.ville }}, {{ faculte.ecolePartenaire.pays }}
                </p>
                <p class="text-sm">
                  <span class="px-2 py-1 rounded text-xs"
                        :class="faculte.ecolePartenaire.type === 'publique' ? 'bg-blue-100 text-blue-800' : 'bg-purple-100 text-purple-800'">
                    {{ faculte.ecolePartenaire.type === 'publique' ? 'Public' : 'Privé' }}
                  </span>
                </p>
                <div v-if="faculte.ecolePartenaire.siteWeb" class="pt-2">
                  <a :href="faculte.ecolePartenaire.siteWeb" target="_blank" rel="noopener"
                     class="text-blue-600 hover:underline text-sm flex items-center">
                    <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"></path>
                    </svg>
                    Visiter le site web
                  </a>
                </div>
              </div>
            </div>

            <!-- Conditions d'admission -->
            <div class="bg-white rounded-lg shadow-md p-6">
              <h3 class="text-lg font-bold mb-4">Conditions d'admission</h3>
              <div class="space-y-3 text-sm">
                <div>
                  <p class="text-gray-500">Diplôme minimum</p>
                  <p class="font-medium">{{ faculte.conditionsAdmission.diplomeMinimum }}</p>
                </div>
                <div>
                  <p class="text-gray-500">Langues d'enseignement</p>
                  <p class="font-medium">{{ faculte.conditionsAdmission.languesEnseignement.join(', ') }}</p>
                </div>
                <div>
                  <p class="text-gray-500">Frais de scolarité annuels</p>
                  <p class="font-medium">
                    {{ faculte.conditionsAdmission.fraisScolariteAnnuels.min.toLocaleString() }} -
                    {{ faculte.conditionsAdmission.fraisScolariteAnnuels.max.toLocaleString() }} FCFA
                  </p>
                  <p v-if="faculte.conditionsAdmission.fraisScolariteAnnuels.boursesPossibles" class="text-green-600 text-xs mt-1">
                    Bourses disponibles
                  </p>
                </div>
                <div>
                  <p class="text-gray-500">Périodes d'inscription</p>
                  <p class="font-medium">{{ faculte.conditionsAdmission.periodesInscription }}</p>
                </div>
              </div>
            </div>

            <!-- Statistiques -->
            <div class="bg-white rounded-lg shadow-md p-6">
              <h3 class="text-lg font-bold mb-4">Statistiques</h3>
              <div class="grid grid-cols-2 gap-4 text-center">
                <div class="bg-blue-50 rounded-lg p-4">
                  <p class="text-2xl font-bold text-blue-600">{{ faculte.stats.nombreInscritsTotal }}</p>
                  <p class="text-xs text-gray-600">Intéressés total</p>
                </div>
                <div class="bg-green-50 rounded-lg p-4">
                  <p class="text-2xl font-bold text-green-600">{{ faculte.stats.nombreInscritsAnneeEnCours }}</p>
                  <p class="text-xs text-gray-600">Cette année</p>
                </div>
              </div>
            </div>

            <!-- Action -->
            <button v-if="faculte.accepteNouveauxInscrits"
                    @click="manifesterInteret"
                    class="w-full py-3 bg-custom-green text-white rounded-lg font-medium hover:bg-green-700 transition">
              Manifester mon intérêt
            </button>
            <div v-else class="bg-gray-100 rounded-lg p-4 text-center">
              <p class="text-gray-600">Les inscriptions sont actuellement fermées</p>
            </div>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { getFaculteById, type Faculte } from '~/mocks/inuda/facultes'

const route = useRoute()
const loading = ref(true)
const faculte = ref<Faculte | null>(null)

const manifesterInteret = () => {
  alert('Cette fonctionnalité sera disponible prochainement.')
}

onMounted(() => {
  loading.value = true
  const id = route.params.id as string
  faculte.value = getFaculteById(id) || null
  loading.value = false

  if (faculte.value) {
    useHead({
      title: `${faculte.value.titre} - INUDA`
    })
  }
})
</script>
