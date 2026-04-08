<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section -->
    <div class="relative h-96 overflow-hidden bg-linear-to-br from-amber-700 via-orange-500 to-yellow-400">
      <!-- Motif décoratif -->
      <div class="absolute inset-0 opacity-10"
           style="background-image: repeating-linear-gradient(-45deg, transparent, transparent 35px, rgba(255,255,255,0.1) 35px, rgba(255,255,255,0.1) 70px);"></div>
      <div class="absolute -top-20 -right-20 w-96 h-96 rounded-full bg-white/5 animate-pulse"></div>
      <div class="absolute -bottom-32 -left-16 w-80 h-80 rounded-full bg-white/5 animate-pulse" style="animation-delay: 1s;"></div>

      <div class="absolute inset-0 flex flex-col items-center justify-center px-4">
        <div class="flex items-center gap-3 mb-4 animate-fadeInUp">
          <div class="w-12 h-12 rounded-full bg-white/20 backdrop-blur-sm flex items-center justify-center">
            <font-awesome-icon :icon="['fas', 'lightbulb']" class="text-white text-xl" />
          </div>
          <span class="text-white/60 text-sm font-semibold tracking-widest uppercase">Gouvernance Citoyenne</span>
        </div>
        <h1 class="text-white text-5xl md:text-6xl font-display font-bold mb-4 animate-fadeInUp tracking-tight">
          IdeaForces
        </h1>
        <div class="h-1 w-24 bg-linear-to-r from-yellow-200 to-white rounded-full mb-4 animate-expandWidth"></div>
        <p class="text-white/80 text-lg md:text-xl text-center max-w-2xl animate-fadeInUp animation-delay-200">
          Propositions d'idées et forces positives pour l'Afrique
        </p>

        <!-- Stats Hero -->
        <div class="flex flex-wrap items-center justify-center gap-4 mt-8 animate-fadeInUp animation-delay-400">
          <div class="flex items-center bg-white/15 backdrop-blur-sm px-5 py-2.5 rounded-full border border-white/20">
            <font-awesome-icon :icon="['fas', 'lightbulb']" class="text-yellow-200 mr-2" />
            <span class="text-white font-bold text-lg">{{ contributions.length }}</span>
            <span class="text-white/70 ml-2 text-sm">idées</span>
          </div>
          <div class="flex items-center bg-white/15 backdrop-blur-sm px-5 py-2.5 rounded-full border border-white/20">
            <font-awesome-icon :icon="['fas', 'earth-africa']" class="text-yellow-200 mr-2" />
            <span class="text-white font-bold text-lg">{{ paysDisponibles.length }}</span>
            <span class="text-white/70 ml-2 text-sm">pays concernés</span>
          </div>
          <div class="flex items-center bg-white/15 backdrop-blur-sm px-5 py-2.5 rounded-full border border-white/20">
            <font-awesome-icon :icon="['fas', 'thumbs-up']" class="text-yellow-200 mr-2" />
            <span class="text-white font-bold text-lg">{{ totalSoutiens }}</span>
            <span class="text-white/70 ml-2 text-sm">soutiens</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Contenu -->
    <div class="max-w-7xl mx-auto px-4 py-8 -mt-16 relative z-10">
      <!-- Barre de navigation -->
      <div class="bg-white rounded-xl shadow-lg p-5 mb-8 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
        <CommonBreadcrumbNav />
        <NuxtLink to="/universite/gouvernance"
                   class="inline-flex items-center gap-2 px-4 py-2 rounded-lg bg-orange-50 text-orange-700 hover:bg-orange-100 transition-colors font-medium text-sm">
          <font-awesome-icon :icon="['fas', 'arrow-left']" class="text-xs" />
          Retour à la gouvernance
        </NuxtLink>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-4 gap-8">
        <!-- Filtres -->
        <div class="lg:col-span-1">
          <div class="bg-white rounded-xl shadow-lg overflow-hidden sticky top-4">
            <div class="bg-linear-to-r from-orange-500 to-amber-500 px-6 py-4">
              <h3 class="text-white font-bold flex items-center gap-2">
                <font-awesome-icon :icon="['fas', 'filter']" />
                Filtres
              </h3>
            </div>

            <div class="p-6 space-y-5">
              <!-- Recherche -->
              <div class="relative">
                <font-awesome-icon :icon="['fas', 'search']" class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 text-sm" />
                <input v-model="recherche"
                       type="text"
                       placeholder="Rechercher..."
                       class="w-full pl-10 pr-4 py-2.5 border border-gray-200 rounded-lg focus:ring-2 focus:ring-orange-500/30 focus:border-orange-500 transition">
              </div>

              <!-- Pays -->
              <div>
                <label class="block text-xs font-semibold text-gray-500 uppercase tracking-wide mb-2">Pays</label>
                <select v-model="paysSelectionne" class="w-full px-3 py-2.5 border border-gray-200 rounded-lg focus:ring-2 focus:ring-orange-500/30 focus:border-orange-500 transition bg-white">
                  <option value="">Tous les pays</option>
                  <option v-for="pays in paysDisponibles" :key="pays" :value="pays">{{ pays }}</option>
                </select>
              </div>

              <button @click="reinitialiser"
                      class="w-full py-2.5 text-sm font-medium text-gray-500 border border-gray-200 rounded-lg hover:bg-gray-50 hover:text-gray-700 transition flex items-center justify-center gap-2">
                <font-awesome-icon :icon="['fas', 'undo']" class="text-xs" />
                Réinitialiser
              </button>
            </div>
          </div>
        </div>

        <!-- Liste -->
        <div class="lg:col-span-3">
          <!-- Résultats count -->
          <div class="flex items-center justify-between mb-4">
            <p class="text-sm text-gray-500">
              <span class="font-semibold text-gray-900">{{ contributionsFiltrees.length }}</span> résultat{{ contributionsFiltrees.length > 1 ? 's' : '' }}
              <span v-if="recherche || paysSelectionne"> (filtré{{ contributionsFiltrees.length > 1 ? 's' : '' }})</span>
            </p>
          </div>

          <!-- État vide -->
          <div v-if="contributionsFiltrees.length === 0"
               class="text-center py-20 bg-white rounded-xl shadow-lg">
            <div class="w-20 h-20 mx-auto mb-6 rounded-full bg-gray-100 flex items-center justify-center">
              <font-awesome-icon :icon="['fas', 'search']" class="text-gray-400 text-2xl" />
            </div>
            <p class="text-gray-900 font-semibold text-lg mb-2">Aucun résultat trouvé</p>
            <p class="text-gray-500 text-sm">Essayez de modifier vos filtres de recherche</p>
          </div>

          <!-- Cartes -->
          <div v-else class="space-y-5">
            <div v-for="contribution in contributionsFiltrees" :key="contribution.id"
                 class="group bg-white rounded-xl shadow-md hover:shadow-xl transition-all duration-300 overflow-hidden cursor-pointer border border-gray-100 hover:border-gray-200"
                 @click="voirDetail(contribution)">
              <!-- Bande orange -->
              <div class="h-1.5 bg-linear-to-r from-amber-400 to-orange-500"></div>

              <div class="p-6">
                <div class="flex items-start gap-4">
                  <!-- Icône -->
                  <div class="shrink-0 w-12 h-12 rounded-xl bg-amber-100 text-amber-600 flex items-center justify-center">
                    <font-awesome-icon :icon="['fas', 'lightbulb']" class="text-lg" />
                  </div>

                  <div class="flex-1 min-w-0">
                    <!-- Badge -->
                    <div class="flex flex-wrap items-center gap-2 mb-2">
                      <span class="px-2.5 py-0.5 bg-amber-100 text-amber-700 rounded-full text-xs font-bold uppercase tracking-wide">
                        IdeaForces
                      </span>
                    </div>

                    <!-- Titre -->
                    <h3 class="text-lg font-bold text-gray-900 mb-2 group-hover:text-orange-600 transition-colors line-clamp-2">
                      {{ contribution.titre }}
                    </h3>

                    <!-- Description -->
                    <p class="text-gray-500 text-sm leading-relaxed mb-4 line-clamp-2">
                      {{ contribution.description }}
                    </p>

                    <!-- Proposition -->
                    <div v-if="contribution.proposition" class="mb-4 p-4 bg-amber-50 rounded-lg border border-amber-100">
                      <p class="text-xs font-bold text-amber-600 uppercase tracking-wide mb-2">
                        <font-awesome-icon :icon="['fas', 'rocket']" class="mr-1" />
                        Objectif
                      </p>
                      <p class="text-amber-900 text-sm leading-relaxed line-clamp-2">{{ contribution.proposition.objectif }}</p>

                      <div v-if="contribution.proposition.beneficiaires?.length" class="mt-3 flex flex-wrap gap-1.5">
                        <span v-for="b in contribution.proposition.beneficiaires" :key="b"
                              class="px-2 py-0.5 bg-amber-200/60 text-amber-800 rounded-full text-xs font-medium">
                          {{ b }}
                        </span>
                      </div>
                    </div>

                    <!-- Métadonnées -->
                    <div class="flex flex-wrap items-center gap-x-5 gap-y-2 text-xs text-gray-400">
                      <span class="flex items-center gap-1.5">
                        <font-awesome-icon :icon="['fas', 'user']" />
                        {{ contribution.auteur.prenom }} {{ contribution.auteur.nom }}
                      </span>
                      <span class="flex items-center gap-1.5">
                        <font-awesome-icon :icon="['fas', 'map-marker-alt']" />
                        {{ contribution.localisation.pays }}
                      </span>
                      <span class="flex items-center gap-1.5">
                        <font-awesome-icon :icon="['fas', 'calendar-alt']" />
                        {{ formatDate(contribution.dateCreation) }}
                      </span>
                    </div>
                  </div>

                  <!-- Flèche -->
                  <div class="shrink-0 hidden sm:flex items-center">
                    <div class="w-8 h-8 rounded-full bg-gray-100 group-hover:bg-orange-100 flex items-center justify-center transition-all group-hover:translate-x-1">
                      <font-awesome-icon :icon="['fas', 'chevron-right']" class="text-gray-400 group-hover:text-orange-600 text-xs transition-colors" />
                    </div>
                  </div>
                </div>

                <!-- Stats footer -->
                <div class="flex items-center gap-6 mt-5 pt-4 border-t border-gray-100 text-xs text-gray-400">
                  <span class="flex items-center gap-1.5 hover:text-gray-600 transition">
                    <font-awesome-icon :icon="['fas', 'eye']" />
                    {{ contribution.stats.vues }} vues
                  </span>
                  <span class="flex items-center gap-1.5 hover:text-red-500 transition">
                    <font-awesome-icon :icon="['fas', 'heart']" />
                    {{ contribution.stats.likes }} likes
                  </span>
                  <span class="flex items-center gap-1.5 hover:text-orange-500 transition">
                    <font-awesome-icon :icon="['fas', 'hand-fist']" />
                    {{ contribution.stats.soutiens || 0 }} soutiens
                  </span>
                </div>
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

const totalSoutiens = computed(() =>
  contributions.value.reduce((acc, c) => acc + (c.stats.soutiens || 0), 0)
)

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
