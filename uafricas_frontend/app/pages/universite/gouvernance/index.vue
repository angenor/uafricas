<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section avec image de fond -->
    <div
      class="relative h-80 bg-cover bg-center"
      style="background-image: url('https://images.unsplash.com/photo-1529107386315-e1a2ed48a620?ixlib=rb-4.0.3&auto=format&fit=crop&w=1920&q=80');">
      <div class="absolute inset-0 bg-gradient-to-r from-custom-green/90 to-black/70"></div>

      <!-- Texte avec animation d'entrée -->
      <div class="absolute inset-0 flex flex-col items-center justify-center pt-28">
        <h1 class="text-white text-4xl md:text-5xl font-bold mb-4 animate-fadeInUp">
          Novagouv
        </h1>
        <div class="h-1 w-32 bg-white rounded animate-expandWidth"></div>
        <p class="text-white text-xl md:text-2xl mt-4 text-center px-4 animate-fadeInUp animation-delay-200">
          Contribuez à l'amélioration de notre société
        </p>
        <div class="flex items-center space-x-6 my-6 text-white animate-fadeInUp animation-delay-400">
          <div class="flex items-center bg-white/20 backdrop-blur-xs px-4 py-2 rounded-full">
            <font-awesome-icon :icon="['fas', 'eye']" class="mr-2" />
            <span class="font-semibold">{{ stats.factcheck }}</span>
            <span class="ml-1 text-sm">FactCheck</span>
          </div>
          <div class="flex items-center bg-white/20 backdrop-blur-xs px-4 py-2 rounded-full">
            <font-awesome-icon :icon="['fas', 'lightbulb']" class="mr-2" />
            <span class="font-semibold">{{ stats.ideaforces }}</span>
            <span class="ml-1 text-sm">IdeaForces</span>
          </div>
          <div class="flex items-center bg-white/20 backdrop-blur-xs px-4 py-2 rounded-full">
            <font-awesome-icon :icon="['fas', 'exclamation-triangle']" class="mr-2" />
            <span class="font-semibold">{{ stats.badhabits }}</span>
            <span class="ml-1 text-sm">BadHabits</span>
          </div>
        </div>
        <p class="text-white/80 text-sm md:text-base mt-2 max-w-3xl text-center px-4 animate-fadeInUp animation-delay-400">
          Engager les citoyens africains et afro-descendants ainsi que la diaspora dans l'amélioration de gouvernance politique, sociale et économique de l'Afrique.
        </p>
      </div>
    </div>

    <!-- Contenu principal -->
    <div class="max-w-7xl mx-auto -mt-16 relative z-20 px-4 py-8">
      <!-- Navigation vers les pages spécialisées -->
      <div class="bg-white rounded-xl shadow-lg overflow-hidden">
        <div class="p-8">
          <h2 class="text-3xl font-bold text-gray-900 mb-4 text-center">Catégories de Gouvernance</h2>
          <p class="text-gray-600 mb-8 text-center text-lg">Explorez chaque catégorie en détail pour une expérience plus spécialisée</p>

          <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
            <!-- FactCheck Card -->
            <NuxtLink
              to="/universite/gouvernance/factcheck"
              class="group block p-8 bg-gradient-to-br from-blue-50 to-blue-100 rounded-xl border-2 border-blue-200 hover:border-blue-400 transition-all duration-300 transform hover:scale-105 hover:shadow-xl"
            >
              <div class="flex items-center justify-between mb-6">
                <div class="p-4 bg-blue-600 rounded-lg group-hover:bg-blue-700 transition-colors">
                  <font-awesome-icon :icon="['fas', 'search']" class="text-white text-2xl" />
                </div>
              </div>
              <h3 class="text-2xl font-bold text-blue-900 mb-3 group-hover:text-blue-700">FactCheck</h3>
              <p class="text-blue-700 mb-6">Vérification des faits et lutte contre la désinformation</p>
              <div class="flex items-center text-blue-600 group-hover:text-blue-700">
                <span class="font-medium">Explorer</span>
                <font-awesome-icon :icon="['fas', 'arrow-right']" class="ml-2 transform group-hover:translate-x-1 transition-transform" />
              </div>
            </NuxtLink>

            <!-- IdeaForces Card -->
            <NuxtLink
              to="/universite/gouvernance/ideaforces"
              class="group block p-8 bg-gradient-to-br from-yellow-50 to-orange-100 rounded-xl border-2 border-yellow-200 hover:border-yellow-400 transition-all duration-300 transform hover:scale-105 hover:shadow-xl"
            >
              <div class="flex items-center justify-between mb-6">
                <div class="p-4 bg-yellow-500 rounded-lg group-hover:bg-yellow-600 transition-colors">
                  <font-awesome-icon :icon="['fas', 'lightbulb']" class="text-white text-2xl" />
                </div>
              </div>
              <h3 class="text-2xl font-bold text-yellow-900 mb-3 group-hover:text-yellow-700">IdeaForces</h3>
              <p class="text-yellow-700 mb-6">Propositions d'idées et forces positives pour l'Afrique</p>
              <div class="flex items-center text-yellow-600 group-hover:text-yellow-700">
                <span class="font-medium">Explorer</span>
                <font-awesome-icon :icon="['fas', 'arrow-right']" class="ml-2 transform group-hover:translate-x-1 transition-transform" />
              </div>
            </NuxtLink>

            <!-- BadHabits Card -->
            <NuxtLink
              to="/universite/gouvernance/badhabits"
              class="group block p-8 bg-gradient-to-br from-red-50 to-red-100 rounded-xl border-2 border-red-200 hover:border-red-400 transition-all duration-300 transform hover:scale-105 hover:shadow-xl"
            >
              <div class="flex items-center justify-between mb-6">
                <div class="p-4 bg-red-600 rounded-lg group-hover:bg-red-700 transition-colors">
                  <font-awesome-icon :icon="['fas', 'exclamation-triangle']" class="text-white text-2xl" />
                </div>
              </div>
              <h3 class="text-2xl font-bold text-red-900 mb-3 group-hover:text-red-700">BadHabits</h3>
              <p class="text-red-700 mb-6">Signalement des mauvaises pratiques et habitudes néfastes</p>
              <div class="flex items-center text-red-600 group-hover:text-red-700">
                <span class="font-medium">Explorer</span>
                <font-awesome-icon :icon="['fas', 'arrow-right']" class="ml-2 transform group-hover:translate-x-1 transition-transform" />
              </div>
            </NuxtLink>
          </div>
        </div>
      </div>

      <!-- Dernières contributions -->
      <div class="mt-12">
        <h2 class="text-2xl font-bold mb-6">Dernières contributions</h2>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <UniversiteGouvernanceContributionCard
            v-for="contribution in dernieresContributions"
            :key="contribution.id"
            :contribution="contribution"
            @click="voirContribution"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ContributionCitoyenne } from '~/mocks/gouvernance/contributions'

useHead({
  title: 'Novagouv - Gouvernance Citoyenne | UAfricas'
})

const { getStats, getContributions } = useGouvernance()

const stats = ref({ total: 0, factcheck: 0, badhabits: 0, ideaforces: 0, totalLikes: 0 })
const dernieresContributions = ref<ContributionCitoyenne[]>([])

const voirContribution = (contribution: ContributionCitoyenne) => {
  navigateTo(`/universite/gouvernance/${contribution.id}`)
}

onMounted(async () => {
  try {
    const [statsData, contributionsData] = await Promise.all([
      getStats(),
      getContributions({ parPage: 6 }),
    ])
    stats.value = statsData
    dernieresContributions.value = contributionsData.contributions
  } catch (err) {
    console.error('Erreur chargement gouvernance:', err)
  }
})
</script>

<style scoped>
/* Animations personnalisées */
@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(30px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes expandWidth {
  from {
    width: 0;
  }
  to {
    width: 8rem;
  }
}

.animate-fadeInUp {
  animation: fadeInUp 0.6s ease-out forwards;
}

.animate-expandWidth {
  animation: expandWidth 0.8s ease-out forwards;
}

.animation-delay-200 {
  animation-delay: 200ms;
}

.animation-delay-400 {
  animation-delay: 400ms;
}
</style>
