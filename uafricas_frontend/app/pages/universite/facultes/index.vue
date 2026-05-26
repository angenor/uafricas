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
            Nos Facultés Partenaires
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Découvrez les facultés qui collaborent avec l'INUDA pour votre réussite académique
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
                Explorez nos partenariats académiques d'excellence
              </p>
            </div>
            <NuxtLink to="/universite"
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
            <div class="bg-white rounded-lg shadow-md p-6 sticky top-4">
              <h3 class="text-lg font-bold mb-4">Filtrer par domaine</h3>

              <!-- Recherche -->
              <div class="mb-4 relative">
                <font-awesome-icon icon="fa-solid fa-magnifying-glass"
                                   class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 text-sm" />
                <input v-model="recherche"
                       type="text"
                       placeholder="Rechercher une faculté..."
                       class="w-full pl-9 pr-3 py-2 text-sm border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500">
              </div>

              <!-- Domaines -->
              <div class="space-y-2">
                <label class="flex items-center">
                  <input type="radio" v-model="domaineSelectionne" value="" class="mr-2">
                  <span>Tous les domaines</span>
                </label>
                <label v-for="domaine in domaines" :key="domaine" class="flex items-center">
                  <input type="radio" v-model="domaineSelectionne" :value="domaine" class="mr-2">
                  <span>{{ domaine }}</span>
                </label>
              </div>

              <!-- Type d'école -->
              <h4 class="text-sm font-medium text-gray-700 mt-6 mb-2">Type d'établissement</h4>
              <div class="space-y-2">
                <label class="flex items-center">
                  <input type="checkbox" v-model="typesEcole" value="publique" class="mr-2">
                  <span>Public</span>
                </label>
                <label class="flex items-center">
                  <input type="checkbox" v-model="typesEcole" value="privee" class="mr-2">
                  <span>Privé</span>
                </label>
              </div>

              <!-- Accepte nouveaux inscrits -->
              <div class="mt-6">
                <label class="flex items-center">
                  <input type="checkbox" v-model="seulementOuvertes" class="mr-2">
                  <span class="text-sm">Inscriptions ouvertes uniquement</span>
                </label>
              </div>
            </div>
          </div>

          <!-- Liste des facultés -->
          <div class="lg:col-span-3">
            <div v-if="loading" class="text-center py-12">
              <div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
              <p class="mt-4 text-gray-600">Chargement des facultés...</p>
            </div>

            <div v-else-if="facultesFiltrees.length === 0" class="text-center py-12">
              <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
              </svg>
              <p class="mt-4 text-gray-600">Aucune faculté ne correspond à vos critères</p>
            </div>

            <div v-else class="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div v-for="faculte in facultesFiltrees" :key="faculte.id"
                   class="bg-white rounded-lg shadow-md hover:shadow-lg transition-shadow overflow-hidden">
                <!-- Image de couverture -->
                <div class="h-48 bg-gradient-to-br from-blue-500 to-indigo-600 flex items-center justify-center">
                  <img v-if="faculte.imageCouverture"
                       :src="faculte.imageCouverture"
                       :alt="faculte.titre"
                       class="w-full h-full object-cover">
                  <svg v-else class="w-16 h-16 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 14l9-5-9-5-9 5 9 5z"></path>
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 14v7"></path>
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 9v7m18-7v7"></path>
                  </svg>
                </div>

                <!-- Contenu -->
                <div class="p-6">
                  <div class="flex items-start justify-between mb-3">
                    <div class="flex-1">
                      <h3 class="text-xl font-bold text-gray-900 mb-1">{{ faculte.titre }}</h3>
                      <p class="text-sm text-gray-600">{{ faculte.acronyme }}</p>
                    </div>
                    <span v-if="faculte.accepteNouveauxInscrits"
                          class="px-2 py-1 bg-green-100 text-green-800 text-xs font-medium rounded-full">
                      Inscriptions ouvertes
                    </span>
                  </div>

                  <p class="text-gray-600 mb-4 line-clamp-2">{{ faculte.description }}</p>

                  <!-- École partenaire -->
                  <div class="mb-4 text-sm text-gray-500">
                    <div class="flex items-center">
                      <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"></path>
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"></path>
                      </svg>
                      {{ faculte.ecolePartenaire?.nom }} - {{ faculte.ecolePartenaire?.ville }}, {{ faculte.ecolePartenaire?.pays }}
                    </div>
                  </div>

                  <!-- Domaines d'études -->
                  <div class="mb-4">
                    <div class="flex flex-wrap gap-1">
                      <span v-for="domaine in faculte.domainesEtudes?.slice(0, 3)" :key="domaine"
                            class="px-2 py-1 bg-blue-100 text-blue-800 text-xs rounded-full">
                        {{ domaine }}
                      </span>
                      <span v-if="faculte.domainesEtudes?.length > 3"
                            class="px-2 py-1 bg-gray-100 text-gray-600 text-xs rounded-full">
                        +{{ faculte.domainesEtudes.length - 3 }}
                      </span>
                    </div>
                  </div>

                  <!-- Boutons d'action -->
                  <div class="flex items-center gap-3">
                    <button @click="voirDetail(faculte)"
                            class="inline-flex items-center px-4 py-2 text-sm font-medium text-custom-chocolat bg-white border border-custom-chocolat rounded-md hover:bg-custom-chocolat hover:text-white transition-colors">
                      <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"></path>
                      </svg>
                      En savoir plus
                    </button>
                    <button @click="ouvrirFormulaireInteret(faculte)"
                            :disabled="!faculte.accepteNouveauxInscrits"
                            class="px-4 py-2 rounded-md font-medium transition"
                            :class="faculte.accepteNouveauxInscrits
                              ? 'bg-custom-green text-white hover:bg-green-700'
                              : 'bg-gray-300 text-gray-500 cursor-not-allowed'">
                      {{ faculte.accepteNouveauxInscrits ? 'Manifester intérêt' : 'Fermé' }}
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Modal Manifestation d'intérêt -->
    <div v-if="faculteSelectionnee" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
      <div class="bg-white rounded-lg max-w-lg w-full max-h-[90vh] overflow-y-auto">
        <div class="p-6">
          <div class="flex justify-between items-start mb-4">
            <h2 class="text-xl font-bold">Manifester mon intérêt</h2>
            <button @click="faculteSelectionnee = null" class="text-gray-500 hover:text-gray-700">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
              </svg>
            </button>
          </div>
          <p class="text-gray-600 mb-4">
            Vous manifestez votre intérêt pour : <strong>{{ faculteSelectionnee.titre }}</strong>
          </p>
          <p class="text-sm text-gray-500 mb-4">
            Cette fonctionnalité sera disponible prochainement. Vous pourrez soumettre votre candidature directement depuis la plateforme.
          </p>
          <button @click="faculteSelectionnee = null"
                  class="w-full py-2 bg-custom-chocolat text-white rounded-md hover:bg-custom-chocolat/90 transition">
            Fermer
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { getFacultesActives, getDomainesUniques, type Faculte } from '~/mocks/inuda/facultes'

useHead({
  title: 'Facultés partenaires - INUDA'
})

const breadcrumbs = [
  { label: 'Université', to: '/universite' },
  { label: 'INUDA', to: '/universite' },
  { label: 'Facultés', to: undefined }
]

const { loading, erreur, listerFacultes } = useFacultes()

const facultes = ref<Faculte[]>([])
const domaines = ref<string[]>([])
const recherche = ref('')
const domaineSelectionne = ref('')
const typesEcole = ref(['publique', 'privee'])
const seulementOuvertes = ref(false)
const faculteSelectionnee = ref<Faculte | null>(null)

// Filtrer les facultes (le filtrage type_ecole se fait cote client
// car le backend filtre deja par domaine/recherche/ouvertes)
const facultesFiltrees = computed(() => {
  return facultes.value.filter(faculte => {
    if (faculte.ecolePartenaire?.type && !typesEcole.value.includes(faculte.ecolePartenaire.type)) {
      return false
    }
    return true
  })
})

const voirDetail = (faculte: Faculte) => {
  navigateTo(`/universite/facultes/${faculte.id}`)
}

const ouvrirFormulaireInteret = (faculte: Faculte) => {
  faculteSelectionnee.value = faculte
}

// Debounce pour la recherche
let rechercheTimer: ReturnType<typeof setTimeout> | null = null

const chargerFacultes = async () => {
  try {
    const result = await listerFacultes({
      recherche: recherche.value || undefined,
      domaine: domaineSelectionne.value || undefined,
      ouvertes: seulementOuvertes.value || undefined,
    })
    facultes.value = result.facultes
    domaines.value = result.domaines
  } catch (e) {
    // Fallback sur les mocks si le backend n'est pas disponible
    console.warn('Fallback sur les données mock:', e)
    facultes.value = getFacultesActives()
    domaines.value = getDomainesUniques()
  }
}

// Recharger quand les filtres changent
watch([domaineSelectionne, seulementOuvertes], () => {
  chargerFacultes()
})

watch(recherche, () => {
  if (rechercheTimer) clearTimeout(rechercheTimer)
  rechercheTimer = setTimeout(() => {
    chargerFacultes()
  }, 400)
})

onMounted(() => {
  chargerFacultes()
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
