<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Loading -->
    <div v-if="chargement" class="flex items-center justify-center h-screen">
      <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-custom-green"></div>
    </div>

    <!-- Pays non trouve -->
    <div v-else-if="!pays" class="flex flex-col items-center justify-center h-screen">
      <svg class="w-16 h-16 text-gray-400 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
      </svg>
      <h1 class="text-2xl font-bold text-gray-700 mb-2">Pays non trouve</h1>
      <p class="text-gray-500 mb-4">Le pays que vous recherchez n'existe pas dans notre base.</p>
      <NuxtLink to="/opportunite-afrique" class="text-custom-green hover:underline">
        &#8592; Retour a la liste des pays
      </NuxtLink>
    </div>

    <!-- Contenu -->
    <template v-else>
      <!-- Hero Section avec image de couverture -->
      <div class="relative h-72 md:h-96 bg-cover bg-center"
           :style="{ backgroundImage: `url(${pays.image_couverture})` }">
        <div class="absolute inset-0 bg-gradient-to-t from-black/80 via-black/40 to-transparent"></div>

        <!-- Contenu du hero -->
        <div class="absolute inset-0 flex flex-col justify-end px-4 md:px-8 pb-8">
          <div class="max-w-6xl mx-auto w-full">
            <CommonBreadcrumbNav class="mb-4" />

            <div class="flex items-center gap-4">
              <!-- Drapeau -->
              <img :src="pays.drapeau_url || ''"
                   :alt="'Drapeau ' + pays.nom"
                   class="h-16 md:h-20 w-auto rounded shadow-lg">

              <div>
                <h1 class="text-3xl md:text-5xl font-bold text-white mb-2">{{ pays.nom }}</h1>
                <p v-if="pays.slogan" class="text-xl text-white/80 italic">{{ pays.slogan }}</p>
              </div>
            </div>

            <!-- Badge region -->
            <div class="mt-4">
              <span class="px-4 py-2 bg-custom-green text-white rounded-full text-sm font-medium">
                {{ pays.region }}
              </span>
            </div>
          </div>
        </div>
      </div>

      <!-- Contenu principal -->
      <div class="max-w-6xl mx-auto px-4 py-8">
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
          <!-- Colonne principale -->
          <div class="lg:col-span-2 space-y-6">
            <!-- Image de couverture -->
            <div class="rounded-lg overflow-hidden shadow-md">
              <img :src="pays.image_couverture"
                   :alt="pays.nom"
                   class="w-full h-64 md:h-80 object-cover" />
            </div>

            <!-- Informations generales -->
            <div class="bg-white rounded-lg shadow-md p-6">
              <h2 class="text-2xl font-bold mb-6 flex items-center">
                <svg class="w-6 h-6 mr-2 text-custom-green" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                </svg>
                Informations generales
              </h2>

              <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div class="flex items-start space-x-3">
                  <div class="flex-shrink-0 w-10 h-10 bg-blue-100 rounded-lg flex items-center justify-center">
                    <svg class="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"></path>
                    </svg>
                  </div>
                  <div>
                    <p class="text-sm text-gray-500">Capitale</p>
                    <p class="font-semibold text-gray-900">{{ pays.capitale }}</p>
                  </div>
                </div>

                <div class="flex items-start space-x-3">
                  <div class="flex-shrink-0 w-10 h-10 bg-green-100 rounded-lg flex items-center justify-center">
                    <svg class="w-5 h-5 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z"></path>
                    </svg>
                  </div>
                  <div>
                    <p class="text-sm text-gray-500">Population</p>
                    <p class="font-semibold text-gray-900">{{ pays.population }}</p>
                  </div>
                </div>

                <div class="flex items-start space-x-3">
                  <div class="flex-shrink-0 w-10 h-10 bg-purple-100 rounded-lg flex items-center justify-center">
                    <svg class="w-5 h-5 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 20l-5.447-2.724A1 1 0 013 16.382V5.618a1 1 0 011.447-.894L9 7m0 13l6-3m-6 3V7m6 10l4.553 2.276A1 1 0 0021 18.382V7.618a1 1 0 00-.553-.894L15 4m0 13V4m0 0L9 7"></path>
                    </svg>
                  </div>
                  <div>
                    <p class="text-sm text-gray-500">Superficie</p>
                    <p class="font-semibold text-gray-900">{{ pays.superficie }}</p>
                  </div>
                </div>

                <div class="flex items-start space-x-3">
                  <div class="flex-shrink-0 w-10 h-10 bg-yellow-100 rounded-lg flex items-center justify-center">
                    <svg class="w-5 h-5 text-yellow-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                    </svg>
                  </div>
                  <div>
                    <p class="text-sm text-gray-500">Monnaie</p>
                    <p class="font-semibold text-gray-900">{{ pays.monnaie }}</p>
                  </div>
                </div>
              </div>
            </div>

            <!-- Culture et Langues -->
            <div class="bg-white rounded-lg shadow-md p-6">
              <h2 class="text-2xl font-bold mb-6 flex items-center">
                <svg class="w-6 h-6 mr-2 text-custom-chocolat" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9"></path>
                </svg>
                Culture et Langues
              </h2>

              <div class="space-y-6">
                <!-- Langues -->
                <div>
                  <h3 class="text-sm font-medium text-gray-500 mb-3">Langues parlees</h3>
                  <div class="flex flex-wrap gap-2">
                    <span v-for="langue in pays.langues" :key="langue"
                          class="px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-sm">
                      {{ langue }}
                    </span>
                  </div>
                </div>

                <!-- Ethnies -->
                <div>
                  <h3 class="text-sm font-medium text-gray-500 mb-3">Principales ethnies</h3>
                  <div class="flex flex-wrap gap-2">
                    <span v-for="ethnie in pays.ethnies" :key="ethnie"
                          class="px-3 py-1 bg-green-100 text-green-800 rounded-full text-sm">
                      {{ ethnie }}
                    </span>
                  </div>
                </div>
              </div>
            </div>

            <!-- Symboles nationaux -->
            <div class="bg-white rounded-lg shadow-md p-6">
              <h2 class="text-2xl font-bold mb-6 flex items-center">
                <svg class="w-6 h-6 mr-2 text-yellow-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z"></path>
                </svg>
                Symboles nationaux
              </h2>

              <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <!-- Devise -->
                <div v-if="pays.devise" class="bg-gray-50 rounded-lg p-4">
                  <p class="text-sm text-gray-500 mb-1">Devise nationale</p>
                  <p class="text-lg font-semibold text-gray-900 italic">"{{ pays.devise }}"</p>
                </div>

                <!-- Drapeau -->
                <div class="bg-gray-50 rounded-lg p-4 flex items-center space-x-4">
                  <img :src="pays.drapeau_url || ''"
                       :alt="'Drapeau ' + pays.nom"
                       class="h-12 w-auto rounded shadow">
                  <div>
                    <p class="text-sm text-gray-500">Drapeau</p>
                    <p class="font-medium text-gray-900">{{ pays.nom }}</p>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- Sidebar -->
          <div class="space-y-6">
            <!-- Statistiques -->
            <div class="bg-white rounded-lg shadow-md p-6">
              <h3 class="text-lg font-bold mb-4">Statistiques</h3>
              <div class="space-y-4">
                <div class="flex items-center justify-between py-2 border-b border-gray-100">
                  <span class="text-gray-600">Region</span>
                  <span class="font-medium text-custom-green">{{ pays.region }}</span>
                </div>
                <div class="flex items-center justify-between py-2 border-b border-gray-100">
                  <span class="text-gray-600">Contributions</span>
                  <span class="font-medium">{{ pays.nombre_contributions }}</span>
                </div>
                <div class="flex items-center justify-between py-2">
                  <span class="text-gray-600">Derniere mise a jour</span>
                  <span class="font-medium text-sm">{{ formatDate(pays.updated_at) }}</span>
                </div>
              </div>
            </div>

            <!-- Contributeurs -->
            <OpportuniteAfriqueContributeursSection :contributeurs="contributeurs" />

            <!-- Actions -->
            <div class="bg-white rounded-lg shadow-md p-6">
              <h3 class="text-lg font-bold mb-4">Actions</h3>
              <div class="space-y-3">
                <button @click="proposerModification"
                        class="w-full py-2 px-4 bg-custom-chocolat text-white rounded-lg font-medium hover:bg-custom-chocolat/90 transition flex items-center justify-center">
                  <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path>
                  </svg>
                  Proposer une modification
                </button>

                <button @click="signalerProbleme"
                        class="w-full py-2 px-4 bg-gray-100 text-gray-700 rounded-lg font-medium hover:bg-gray-200 transition flex items-center justify-center">
                  <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path>
                  </svg>
                  Signaler un probleme
                </button>

                <NuxtLink to="/opportunite-afrique"
                          class="w-full py-2 px-4 border border-gray-300 text-gray-700 rounded-lg font-medium hover:bg-gray-50 transition flex items-center justify-center">
                  <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18"></path>
                  </svg>
                  Retour a la liste
                </NuxtLink>
              </div>
            </div>
          </div>
        </div>
      </div>
    </template>

    <!-- Modal contribution -->
    <OpportuniteAfriqueContributionModal
      ref="contributionModalRef"
      :is-open="showContributionModal"
      :fiche-id="pays?.id || ''"
      :pays-nom="pays?.nom || ''"
      @close="showContributionModal = false"
      @submit="handleContributionSubmit"
    />
  </div>
</template>

<script setup lang="ts">
import {
  useOpportuniteAfrique,
  formatDate,
  type FichePaysDetailAPI,
  type ContributeurAPI,
} from '~/composables/useOpportuniteAfrique'
import { useUserStore } from '~/stores/user'

const route = useRoute()
const userStore = useUserStore()
const { chargement, obtenirFiche, soumettreContribution, listerContributeurs } = useOpportuniteAfrique()

const pays = ref<FichePaysDetailAPI | null>(null)
const contributeurs = ref<ContributeurAPI[]>([])
const showContributionModal = ref(false)
const contributionModalRef = ref<{ setLoading: (val: boolean) => void; setError: (msg: string) => void; setSuccess: () => void } | null>(null)

const proposerModification = () => {
  if (!userStore.isAuthenticated) {
    navigateTo('/login')
    return
  }
  showContributionModal.value = true
}

const signalerProbleme = () => {
  alert('Cette fonctionnalite sera disponible prochainement. Vous pourrez signaler des erreurs ou des problemes.')
}

const handleContributionSubmit = async (data: {
  section: string
  type_contribution: string
  nouvelle_valeur: string
  justification: string
}) => {
  if (!pays.value) return
  contributionModalRef.value?.setLoading(true)

  const result = await soumettreContribution(pays.value.id, {
    section: data.section,
    type_contribution: data.type_contribution,
    nouvelle_valeur: data.nouvelle_valeur,
    justification: data.justification || undefined,
  })

  if (result) {
    contributionModalRef.value?.setSuccess()
    // Rafraichir la liste des contributeurs
    contributeurs.value = await listerContributeurs(pays.value.id)
  } else {
    contributionModalRef.value?.setError('Erreur lors de la soumission de votre contribution. Veuillez reessayer.')
  }
}

onMounted(async () => {
  const id = route.params.id as string
  pays.value = await obtenirFiche(id)

  if (pays.value) {
    // Charger les contributeurs
    contributeurs.value = await listerContributeurs(pays.value.id)

    useHead({
      title: `${pays.value.nom} - Opportunites en Afrique - UAfricas`,
      meta: [
        { name: 'description', content: `Decouvrez le ${pays.value.nom}: capitale ${pays.value.capitale}, population ${pays.value.population}, langues et culture.` }
      ]
    })
  }
})
</script>
