<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section -->
    <div class="relative h-48 bg-gradient-to-r from-purple-600 to-indigo-700">
      <div class="absolute inset-0 flex flex-col items-center justify-center">
        <h1 class="text-white text-3xl md:text-4xl font-bold mb-2">Mon Espace INUDA</h1>
        <p class="text-white/80">Suivez votre parcours de formation</p>
      </div>
    </div>

    <!-- Contenu -->
    <div class="max-w-7xl mx-auto px-4 py-8 -mt-8">
      <!-- Header -->
      <div class="bg-white rounded-lg shadow-md p-6 mb-6">
        <CommonBreadcrumbNav class="mb-4" />
        <div class="flex items-center justify-between">
          <div>
            <h2 class="text-xl font-bold">Mes inscriptions</h2>
            <p class="text-gray-600">Gérez vos formations et suivez votre progression</p>
          </div>
          <NuxtLink to="/universite/inuda" class="text-purple-600 hover:text-purple-800">
            ← Retour à INUDA
          </NuxtLink>
        </div>
      </div>

      <!-- Statistiques personnelles -->
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-8">
        <div class="bg-white rounded-lg shadow-md p-6 text-center">
          <div class="text-3xl font-bold text-blue-600">{{ stats.total }}</div>
          <div class="text-gray-600 text-sm">Formations suivies</div>
        </div>
        <div class="bg-white rounded-lg shadow-md p-6 text-center">
          <div class="text-3xl font-bold text-yellow-600">{{ stats.enCours }}</div>
          <div class="text-gray-600 text-sm">En cours</div>
        </div>
        <div class="bg-white rounded-lg shadow-md p-6 text-center">
          <div class="text-3xl font-bold text-green-600">{{ stats.terminees }}</div>
          <div class="text-gray-600 text-sm">Terminées</div>
        </div>
        <div class="bg-white rounded-lg shadow-md p-6 text-center">
          <div class="text-3xl font-bold text-purple-600">{{ stats.certificats }}</div>
          <div class="text-gray-600 text-sm">Certificats</div>
        </div>
      </div>

      <!-- Message d'information (pas d'auth) -->
      <div class="bg-yellow-50 border-l-4 border-yellow-400 p-6 rounded-lg mb-8">
        <div class="flex items-start">
          <svg class="w-6 h-6 text-yellow-400 mr-3 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
          </svg>
          <div>
            <h3 class="text-yellow-800 font-medium">Mode démonstration</h3>
            <p class="text-yellow-700 text-sm mt-1">
              Cette page affiche des données de démonstration. Connectez-vous pour voir vos vraies inscriptions et suivre votre progression.
            </p>
          </div>
        </div>
      </div>

      <!-- Filtres -->
      <div class="bg-white rounded-lg shadow-md p-4 mb-6">
        <div class="flex flex-wrap items-center gap-4">
          <div class="flex-1 min-w-[200px]">
            <input v-model="recherche"
                   type="text"
                   placeholder="Rechercher une formation..."
                   class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-3 focus:ring-purple-500 focus:border-purple-500">
          </div>
          <select v-model="filtreStatut" class="px-3 py-2 border border-gray-300 rounded-md">
            <option value="">Tous les statuts</option>
            <option value="en_cours">En cours</option>
            <option value="terminee">Terminée</option>
            <option value="non_commencee">Non commencée</option>
          </select>
          <select v-model="filtreType" class="px-3 py-2 border border-gray-300 rounded-md">
            <option value="">Tous les types</option>
            <option value="mooc">MOOC</option>
            <option value="clom">CLOM</option>
            <option value="atelier">Atelier</option>
            <option value="concertation">Concertation</option>
          </select>
        </div>
      </div>

      <!-- Liste des inscriptions -->
      <div class="space-y-4">
        <div v-for="inscription in inscriptionsFiltrees" :key="inscription.id"
             class="bg-white rounded-lg shadow-md overflow-hidden hover:shadow-lg transition">
          <div class="p-6">
            <div class="flex items-start justify-between">
              <div class="flex-1">
                <div class="flex items-center gap-2 mb-2">
                  <span class="px-3 py-1 rounded-full text-sm font-medium"
                        :class="getTypeClasses(inscription.formation.type)">
                    {{ getTypeLabel(inscription.formation.type) }}
                  </span>
                  <span class="px-3 py-1 rounded-full text-sm font-medium"
                        :class="getStatutClasses(inscription.statut)">
                    {{ getStatutLabel(inscription.statut) }}
                  </span>
                </div>
                <h3 class="text-xl font-bold text-gray-900 mb-2">{{ inscription.formation.titre }}</h3>
                <p class="text-gray-600 text-sm mb-4">{{ inscription.formation.formateurPrenom }} {{ inscription.formation.formateurNom }}</p>

                <!-- Progression -->
                <div class="mb-4">
                  <div class="flex items-center justify-between text-sm mb-1">
                    <span class="text-gray-600">Progression</span>
                    <span class="font-medium">{{ inscription.progression }}%</span>
                  </div>
                  <div class="w-full bg-gray-200 rounded-full h-2">
                    <div class="bg-purple-600 h-2 rounded-full transition-all duration-300"
                         :style="{ width: inscription.progression + '%' }"></div>
                  </div>
                </div>

                <!-- Informations -->
                <div class="flex items-center gap-4 text-sm text-gray-500">
                  <span class="flex items-center">
                    <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path>
                    </svg>
                    Inscrit le {{ formatDate(inscription.dateInscription) }}
                  </span>
                  <span class="flex items-center">
                    <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                    </svg>
                    {{ inscription.formation.dureeEstimee }}h estimées
                  </span>
                </div>
              </div>

              <!-- Actions -->
              <div class="ml-4 flex flex-col gap-2">
                <button v-if="inscription.statut !== 'terminee'"
                        class="px-4 py-2 bg-purple-600 text-white rounded-md hover:bg-purple-700 transition text-sm">
                  Continuer
                </button>
                <button v-if="inscription.statut === 'terminee' && inscription.formation.certificationDisponible"
                        class="px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 transition text-sm">
                  Certificat
                </button>
                <button class="px-4 py-2 border border-gray-300 text-gray-700 rounded-md hover:bg-gray-50 transition text-sm">
                  Détails
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Message si aucune inscription -->
      <div v-if="inscriptionsFiltrees.length === 0" class="text-center py-12 bg-white rounded-lg shadow-md">
        <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"></path>
        </svg>
        <p class="mt-4 text-gray-600">Aucune inscription trouvée</p>
        <NuxtLink to="/universite/inuda/formations"
                  class="mt-4 inline-block px-6 py-2 bg-purple-600 text-white rounded-md hover:bg-purple-700 transition">
          Découvrir les formations
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
useHead({
  title: 'Mon Espace - INUDA'
})

interface InscriptionMock {
  id: string
  formation: {
    id: string
    titre: string
    type: string
    formateurPrenom: string
    formateurNom: string
    dureeEstimee: number
    certificationDisponible: boolean
  }
  statut: 'non_commencee' | 'en_cours' | 'terminee'
  progression: number
  dateInscription: Date
}

const recherche = ref('')
const filtreStatut = ref('')
const filtreType = ref('')

// Données mock pour les inscriptions
const inscriptions = ref<InscriptionMock[]>([
  {
    id: 'insc-1',
    formation: {
      id: 'form-1',
      titre: 'Introduction à l\'économie africaine',
      type: 'mooc',
      formateurPrenom: 'Fatou',
      formateurNom: 'Dieng',
      dureeEstimee: 24,
      certificationDisponible: true
    },
    statut: 'en_cours',
    progression: 65,
    dateInscription: new Date('2025-01-05')
  },
  {
    id: 'insc-2',
    formation: {
      id: 'form-2',
      titre: 'Leadership et gouvernance en Afrique',
      type: 'clom',
      formateurPrenom: 'Kwame',
      formateurNom: 'Asante',
      dureeEstimee: 40,
      certificationDisponible: true
    },
    statut: 'non_commencee',
    progression: 0,
    dateInscription: new Date('2025-01-20')
  },
  {
    id: 'insc-3',
    formation: {
      id: 'form-5',
      titre: 'Développement web moderne',
      type: 'mooc',
      formateurPrenom: 'Ousmane',
      formateurNom: 'Traoré',
      dureeEstimee: 60,
      certificationDisponible: true
    },
    statut: 'terminee',
    progression: 100,
    dateInscription: new Date('2024-11-15')
  }
])

const stats = computed(() => {
  return {
    total: inscriptions.value.length,
    enCours: inscriptions.value.filter(i => i.statut === 'en_cours').length,
    terminees: inscriptions.value.filter(i => i.statut === 'terminee').length,
    certificats: inscriptions.value.filter(i => i.statut === 'terminee' && i.formation.certificationDisponible).length
  }
})

const inscriptionsFiltrees = computed(() => {
  return inscriptions.value.filter(inscription => {
    if (recherche.value) {
      const search = recherche.value.toLowerCase()
      if (!inscription.formation.titre.toLowerCase().includes(search)) {
        return false
      }
    }
    if (filtreStatut.value && inscription.statut !== filtreStatut.value) {
      return false
    }
    if (filtreType.value && inscription.formation.type !== filtreType.value) {
      return false
    }
    return true
  })
})

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

const getStatutLabel = (statut: string) => {
  const labels: Record<string, string> = {
    non_commencee: 'Non commencée',
    en_cours: 'En cours',
    terminee: 'Terminée'
  }
  return labels[statut] || statut
}

const getStatutClasses = (statut: string) => {
  const classes: Record<string, string> = {
    non_commencee: 'bg-gray-100 text-gray-800',
    en_cours: 'bg-yellow-100 text-yellow-800',
    terminee: 'bg-green-100 text-green-800'
  }
  return classes[statut] || 'bg-gray-100 text-gray-800'
}

const formatDate = (date: Date) => {
  return new Intl.DateTimeFormat('fr-FR', { day: 'numeric', month: 'short', year: 'numeric' }).format(new Date(date))
}
</script>
