<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section (compact, titre ↔ description au survol) -->
    <div
      class="group relative bg-cover bg-center"
      style="background-image: url('https://images.unsplash.com/photo-1507842217343-583bb7270b66?ixlib=rb-1.2.1&auto=format&fit=crop&w=1900&q=80')">
      <div class="absolute inset-0 bg-gradient-to-r from-custom-chocolat/90 to-black/70"></div>

      <div class="relative max-w-4xl mx-auto px-4 pt-16 pb-6 text-center select-none">
        <!-- Conteneur fixe : le titre et la description se superposent (crossfade au survol) -->
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12">
          <h1 class="absolute inset-0 flex items-center justify-center text-white text-2xl md:text-4xl font-bold transition-opacity duration-300 group-hover:opacity-0">
            Mindshiftlab
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Institut universitaire pour le développement de l'Afrique — vulgariser des formations de masse sur des enjeux d'intérêt pour le développement et le renforcement de gouvernance en Afrique.
          </p>
        </div>

        <div class="mt-4 flex flex-wrap items-center justify-center gap-3">
          <!-- Bouton d'aide : ouvre la présentation de Muniversa -->
          <button
            type="button"
            class="inline-flex items-center gap-2 rounded-full bg-white/15 hover:bg-white/25 text-white font-medium text-sm px-4 py-2.5 backdrop-blur-xs ring-1 ring-white/25 transition-colors"
            aria-label="En savoir plus sur Muniversa"
            @click="presentationOuverte = true"
          >
            <font-awesome-icon :icon="['fas', 'circle-question']" class="w-4 h-4" />
            C'est quoi Muniversa&nbsp;?
          </button>
        </div>
      </div>
    </div>

    <!-- Modale de présentation « C'est quoi Muniversa ? » -->
    <UniversitePresentationModal
      :open="presentationOuverte"
      @close="presentationOuverte = false"
    />

    <!-- Statistiques -->
    <div class="max-w-6xl mx-auto px-4 mt-6">
      <div class="bg-white rounded-lg shadow-lg p-8">
        <div class="grid grid-cols-1 md:grid-cols-4 gap-6">
          <div class="text-center">
            <div class="text-4xl font-bold text-blue-600">{{ stats.nombreFacultes }}</div>
            <div class="text-gray-600 mt-2">Facultés partenaires</div>
          </div>
          <div class="text-center">
            <div class="text-4xl font-bold text-green-600">{{ stats.nombreFormationsOuvertes }}</div>
            <div class="text-gray-600 mt-2">Formations disponibles</div>
          </div>
          <div class="text-center">
            <div class="text-4xl font-bold text-purple-600">{{ stats.nombreInscritsTotal }}+</div>
            <div class="text-gray-600 mt-2">Apprenants inscrits</div>
          </div>
          <div class="text-center">
            <div class="text-4xl font-bold text-orange-600">{{ stats.nombrePays }}+</div>
            <div class="text-gray-600 mt-2">Territoires représentés</div>
          </div>
        </div>
      </div>
    </div>

    <!-- Sections principales -->
    <div class="max-w-7xl mx-auto px-4 py-16">
      <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
        <!-- Facultés -->
        <div class="bg-white rounded-lg shadow-lg overflow-hidden hover:shadow-xl transition-shadow cursor-pointer"
             @click="naviguerVers('facultes')">
          <div class="h-48 bg-gradient-to-br from-blue-500 to-blue-700 flex items-center justify-center">
            <svg class="w-24 h-24 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 14l9-5-9-5-9 5 9 5z"></path>
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 14v7"></path>
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 9v7m18-7v7"></path>
            </svg>
          </div>
          <div class="p-6">
            <h3 class="text-2xl font-bold mb-3">Facultés</h3>
            <p class="text-gray-600 mb-4">
              Découvrez nos facultés partenaires et leurs programmes d'excellence
            </p>
            <div class="flex items-center text-blue-600 font-semibold">
              Explorer les facultés
              <svg class="w-5 h-5 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
              </svg>
            </div>
          </div>
        </div>

        <!-- Formations -->
        <div class="bg-white rounded-lg shadow-lg overflow-hidden hover:shadow-xl transition-shadow cursor-pointer"
             @click="naviguerVers('formations')">
          <div class="h-48 bg-gradient-to-br from-green-500 to-green-700 flex items-center justify-center">
            <svg class="w-24 h-24 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"></path>
            </svg>
          </div>
          <div class="p-6">
            <h3 class="text-2xl font-bold mb-3">Formations</h3>
            <p class="text-gray-600 mb-4">
              MOOC, ateliers et concertations pour développer vos compétences
            </p>
            <div class="flex items-center text-green-600 font-semibold">
              Voir les formations
              <svg class="w-5 h-5 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
              </svg>
            </div>
          </div>
        </div>

        <!-- À propos -->
        <div class="bg-white rounded-lg shadow-lg overflow-hidden hover:shadow-xl transition-shadow cursor-pointer"
             @click="afficherAPropos = true">
          <div class="h-48 bg-gradient-to-br from-purple-500 to-purple-700 flex items-center justify-center">
            <svg class="w-24 h-24 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
            </svg>
          </div>
          <div class="p-6">
            <h3 class="text-2xl font-bold mb-3">À propos</h3>
            <p class="text-gray-600 mb-4">
              Notre mission, vision et impact sur l'éducation en Afrique
            </p>
            <div class="flex items-center text-purple-600 font-semibold">
              En savoir plus
              <svg class="w-5 h-5 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
              </svg>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Formations récentes -->
    <div class="bg-white py-16">
      <div class="max-w-7xl mx-auto px-4">
        <h2 class="text-3xl font-bold mb-8">Formations à venir</h2>
        <div v-if="loading" class="text-center py-8">
          <div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
        </div>
        <div v-else-if="formationsRecentes.length > 0" class="grid grid-cols-1 md:grid-cols-3 gap-6">
          <div v-for="formation in formationsRecentes" :key="formation.id"
               class="border rounded-lg overflow-hidden hover:shadow-lg transition-shadow cursor-pointer"
               @click="voirFormation(formation.id)">
            <div class="h-40 bg-gradient-to-br from-green-500 to-green-700 flex items-center justify-center overflow-hidden">
              <img v-if="formation.couverture_url"
                   :src="formation.couverture_url"
                   :alt="formation.titre"
                   class="w-full h-full object-cover">
              <svg v-else class="w-14 h-14 text-white/80" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"></path>
              </svg>
            </div>
            <div class="p-6">
              <div class="flex items-center justify-between mb-4">
                <span class="px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-sm font-medium">
                  {{ getTypeLabel(formation.type) }}
                </span>
                <span class="text-sm text-gray-500">
                  {{ formation.nombre_inscrits }} inscrit{{ formation.nombre_inscrits > 1 ? 's' : '' }}
                </span>
              </div>
              <h3 class="text-xl font-bold mb-2">{{ formation.titre }}</h3>
              <p class="text-gray-600 mb-4 line-clamp-2">{{ formation.description }}</p>
              <div class="flex items-center text-sm text-gray-500">
                <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path>
                </svg>
                {{ formatDateFormation(formation.date_heure_debut) }}
              </div>
            </div>
          </div>
        </div>
        <div v-else class="text-center py-8 text-gray-500">
          Aucune formation disponible pour le moment
        </div>
      </div>
    </div>

    <!-- Modal À propos -->
    <div v-if="afficherAPropos" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
      <div class="bg-white rounded-lg max-w-3xl w-full max-h-[90vh] overflow-y-auto">
        <div class="p-8">
          <div class="flex justify-between items-start mb-6">
            <h2 class="text-3xl font-bold">À propos de Mindshiftlab</h2>
            <button @click="afficherAPropos = false" class="text-gray-500 hover:text-gray-700">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
              </svg>
            </button>
          </div>

          <div class="space-y-6">
            <div>
              <h3 class="text-xl font-bold mb-3 text-blue-600">Notre Mission</h3>
              <p class="text-gray-700">
                L'Institut universitaire pour le développement de l'Afrique (INUDA) a pour mission de
                démocratiser l'accès à l'enseignement supérieur de qualité pour tous les Africains,
                où qu'ils se trouvent dans le monde, en utilisant les technologies numériques les plus avancées.
              </p>
            </div>

            <div>
              <h3 class="text-xl font-bold mb-3 text-green-600">Notre Vision</h3>
              <p class="text-gray-700">
                Devenir la référence en matière d'éducation numérique en Afrique, en créant un pont entre
                les meilleures institutions académiques du continent et les apprenants de la diaspora,
                tout en favorisant l'excellence, l'innovation et le développement durable.
              </p>
            </div>

            <div>
              <h3 class="text-xl font-bold mb-3 text-purple-600">Nos Valeurs</h3>
              <ul class="list-disc list-inside space-y-2 text-gray-700">
                <li>Excellence académique et professionnelle</li>
                <li>Accessibilité et inclusion</li>
                <li>Innovation pédagogique</li>
                <li>Collaboration panafricaine</li>
                <li>Développement durable</li>
                <li>Intégrité et transparence</li>
              </ul>
            </div>

            <div>
              <h3 class="text-xl font-bold mb-3 text-orange-600">Notre Impact</h3>
              <p class="text-gray-700">
                Depuis notre création, nous avons permis à des milliers d'apprenants d'accéder à
                une éducation de qualité, contribuant ainsi au développement des compétences et
                à la transformation numérique de l'Afrique. Nos diplômés occupent aujourd'hui
                des positions clés dans divers secteurs à travers le continent.
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { type FormationAPI, getTypeLabel, formatDateFormation } from '~/composables/useFormations'

useHead({
  title: 'Mindshiftlab - Institut universitaire pour le développement de l\'Afrique'
})

const { listerFormations, obtenirStatsUniversite } = useFormations()

const loading = ref(true)
const afficherAPropos = ref(false)

// Modale de présentation « C'est quoi Muniversa ? »
const presentationOuverte = ref(false)

const stats = ref({
  nombreFacultes: 0,
  nombreFormationsOuvertes: 0,
  nombreInscritsTotal: 0,
  nombrePays: 0,
})
const formationsRecentes = ref<FormationAPI[]>([])

const naviguerVers = (route: string) => {
  navigateTo(`/universite/${route}`)
}

const voirFormation = (formationId: string) => {
  navigateTo(`/universite/formations/${formationId}`)
}

const chargerDonnees = async () => {
  loading.value = true
  try {
    // Charger les statistiques agrégées et les formations récentes en parallèle
    const [resStats, resFormations] = await Promise.all([
      obtenirStatsUniversite(),
      listerFormations({ par_page: 3 }),
    ])

    // Stats depuis l'endpoint dédié (données réelles agrégées côté backend)
    if (resStats) {
      stats.value.nombreFacultes = resStats.nombre_facultes
      stats.value.nombreFormationsOuvertes = resStats.nombre_formations
      stats.value.nombreInscritsTotal = resStats.nombre_inscrits
      stats.value.nombrePays = resStats.nombre_pays
    }

    if (resFormations) {
      formationsRecentes.value = resFormations.formations
    }
  } catch (error) {
    console.error('Erreur lors du chargement des données:', error)
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  chargerDonnees()
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
