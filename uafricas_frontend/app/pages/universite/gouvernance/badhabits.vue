<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section -->
    <div class="relative h-96 overflow-hidden bg-linear-to-br from-red-900 via-red-700 to-orange-600">
      <!-- Motif géométrique animé -->
      <div class="absolute inset-0 opacity-10">
        <div class="absolute top-0 left-0 w-full h-full"
             style="background-image: repeating-linear-gradient(45deg, transparent, transparent 35px, rgba(255,255,255,0.1) 35px, rgba(255,255,255,0.1) 70px);">
        </div>
      </div>
      <!-- Cercles décoratifs -->
      <div class="absolute -top-20 -right-20 w-96 h-96 rounded-full bg-white/5 animate-pulse"></div>
      <div class="absolute -bottom-32 -left-16 w-80 h-80 rounded-full bg-white/5 animate-pulse" style="animation-delay: 1s;"></div>

      <div class="absolute inset-0 flex flex-col items-center justify-center px-4">
        <div class="flex items-center gap-3 mb-4 animate-fadeInUp">
          <div class="w-12 h-12 rounded-full bg-white/20 backdrop-blur-sm flex items-center justify-center">
            <font-awesome-icon :icon="['fas', 'exclamation-triangle']" class="text-white text-xl" />
          </div>
          <span class="text-white/60 text-sm font-semibold tracking-widest uppercase">Gouvernance Citoyenne</span>
        </div>
        <h1 class="text-white text-5xl md:text-6xl font-display font-bold mb-4 animate-fadeInUp tracking-tight">
          BadHabits
        </h1>
        <div class="h-1 w-24 bg-linear-to-r from-orange-400 to-yellow-300 rounded-full mb-4 animate-expandWidth"></div>
        <p class="text-white/80 text-lg md:text-xl text-center max-w-2xl animate-fadeInUp animation-delay-200">
          Signalement des mauvaises pratiques et habitudes néfastes
        </p>

        <!-- Stats Hero -->
        <div class="flex flex-wrap items-center justify-center gap-4 mt-8 animate-fadeInUp animation-delay-400">
          <div class="flex items-center bg-white/15 backdrop-blur-sm px-5 py-2.5 rounded-full border border-white/20">
            <font-awesome-icon :icon="['fas', 'file-lines']" class="text-orange-300 mr-2" />
            <span class="text-white font-bold text-lg">{{ contributions.length }}</span>
            <span class="text-white/70 ml-2 text-sm">signalements</span>
          </div>
          <div class="flex items-center bg-white/15 backdrop-blur-sm px-5 py-2.5 rounded-full border border-white/20">
            <font-awesome-icon :icon="['fas', 'earth-africa']" class="text-orange-300 mr-2" />
            <span class="text-white font-bold text-lg">{{ paysDisponibles.length }}</span>
            <span class="text-white/70 ml-2 text-sm">pays concernés</span>
          </div>
          <div class="flex items-center bg-white/15 backdrop-blur-sm px-5 py-2.5 rounded-full border border-white/20">
            <font-awesome-icon :icon="['fas', 'exclamation-circle']" class="text-red-300 mr-2" />
            <span class="text-white font-bold text-lg">{{ nombreCritiques }}</span>
            <span class="text-white/70 ml-2 text-sm">critiques</span>
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
                   class="inline-flex items-center gap-2 px-4 py-2 rounded-lg bg-red-50 text-red-700 hover:bg-red-100 transition-colors font-medium text-sm">
          <font-awesome-icon :icon="['fas', 'arrow-left']" class="text-xs" />
          Retour à la gouvernance
        </NuxtLink>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-4 gap-8">
        <!-- Filtres -->
        <div class="lg:col-span-1">
          <div class="bg-white rounded-xl shadow-lg overflow-hidden sticky top-4">
            <div class="bg-linear-to-r from-red-600 to-red-700 px-6 py-4">
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
                       class="w-full pl-10 pr-4 py-2.5 border border-gray-200 rounded-lg focus:ring-2 focus:ring-red-500/30 focus:border-red-500 transition">
              </div>

              <!-- Pays -->
              <div>
                <label class="block text-xs font-semibold text-gray-500 uppercase tracking-wide mb-2">Pays</label>
                <select v-model="paysSelectionne" class="w-full px-3 py-2.5 border border-gray-200 rounded-lg focus:ring-2 focus:ring-red-500/30 focus:border-red-500 transition bg-white">
                  <option value="">Tous les pays</option>
                  <option v-for="pays in paysDisponibles" :key="pays" :value="pays">{{ pays }}</option>
                </select>
              </div>

              <!-- Gravité -->
              <div>
                <label class="block text-xs font-semibold text-gray-500 uppercase tracking-wide mb-2">Gravité</label>
                <div class="space-y-2">
                  <button v-for="g in niveauxGravite" :key="g.valeur"
                          @click="graviteSelectionnee = graviteSelectionnee === g.valeur ? '' : g.valeur"
                          class="w-full flex items-center gap-3 px-3 py-2 rounded-lg border transition-all text-sm"
                          :class="graviteSelectionnee === g.valeur
                            ? 'border-red-500 bg-red-50 text-red-700 shadow-sm'
                            : 'border-gray-200 hover:border-gray-300 text-gray-600 hover:bg-gray-50'">
                    <span class="w-3 h-3 rounded-full" :class="g.couleurPastille"></span>
                    <span class="font-medium">{{ g.label }}</span>
                    <span class="ml-auto text-xs px-2 py-0.5 rounded-full bg-gray-100 text-gray-500">
                      {{ compterParGravite(g.valeur) }}
                    </span>
                  </button>
                </div>
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
              <span v-if="recherche || paysSelectionne || graviteSelectionnee"> (filtré{{ contributionsFiltrees.length > 1 ? 's' : '' }})</span>
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
            <div v-for="(contribution, index) in contributionsFiltrees" :key="contribution.id"
                 class="group bg-white rounded-xl shadow-md hover:shadow-xl transition-all duration-300 overflow-hidden cursor-pointer border border-gray-100 hover:border-gray-200"
                 :style="{ animationDelay: `${index * 80}ms` }"
                 @click="voirDetail(contribution)">
              <!-- Bande de gravité -->
              <div class="h-1.5"
                   :class="getBandeGraviteClass(contribution.problematique?.gravite)"></div>

              <div class="p-6">
                <div class="flex items-start gap-4">
                  <!-- Icône gravité -->
                  <div class="shrink-0 w-12 h-12 rounded-xl flex items-center justify-center"
                       :class="getIconeGraviteClass(contribution.problematique?.gravite)">
                    <font-awesome-icon :icon="getIconeGravite(contribution.problematique?.gravite)" class="text-lg" />
                  </div>

                  <div class="flex-1 min-w-0">
                    <!-- Badges -->
                    <div class="flex flex-wrap items-center gap-2 mb-2">
                      <span v-if="contribution.problematique?.gravite"
                            class="px-2.5 py-0.5 rounded-full text-xs font-bold uppercase tracking-wide"
                            :class="getGraviteClass(contribution.problematique.gravite)">
                        {{ contribution.problematique.gravite }}
                      </span>
                      <span v-if="contribution.problematique?.categorie"
                            class="px-2.5 py-0.5 bg-gray-100 text-gray-600 rounded-full text-xs font-medium">
                        {{ contribution.problematique.categorie }}
                      </span>
                    </div>

                    <!-- Titre -->
                    <h3 class="text-lg font-bold text-gray-900 mb-2 group-hover:text-red-700 transition-colors line-clamp-2">
                      {{ contribution.titre }}
                    </h3>

                    <!-- Description -->
                    <p class="text-gray-500 text-sm leading-relaxed mb-4 line-clamp-2">
                      {{ contribution.description }}
                    </p>

                    <!-- Métadonnées -->
                    <div class="flex flex-wrap items-center gap-x-5 gap-y-2 text-xs text-gray-400">
                      <span class="flex items-center gap-1.5">
                        <font-awesome-icon :icon="['fas', 'user']" />
                        {{ contribution.auteur.prenom }} {{ contribution.auteur.nom }}
                      </span>
                      <span class="flex items-center gap-1.5">
                        <font-awesome-icon :icon="['fas', 'map-marker-alt']" />
                        {{ contribution.localisation.ville }}, {{ contribution.localisation.pays }}
                      </span>
                      <span class="flex items-center gap-1.5">
                        <font-awesome-icon :icon="['fas', 'calendar-alt']" />
                        {{ formatDate(contribution.dateCreation) }}
                      </span>
                    </div>
                  </div>

                  <!-- Flèche -->
                  <div class="shrink-0 hidden sm:flex items-center">
                    <div class="w-8 h-8 rounded-full bg-gray-100 group-hover:bg-red-100 flex items-center justify-center transition-all group-hover:translate-x-1">
                      <font-awesome-icon :icon="['fas', 'chevron-right']" class="text-gray-400 group-hover:text-red-600 text-xs transition-colors" />
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
  title: 'BadHabits - Gouvernance Citoyenne'
})

const contributions = ref<ContributionCitoyenne[]>([])
const recherche = ref('')
const paysSelectionne = ref('')
const graviteSelectionnee = ref('')

const niveauxGravite = [
  { valeur: 'critique', label: 'Critique', couleurPastille: 'bg-red-600' },
  { valeur: 'grave', label: 'Grave', couleurPastille: 'bg-red-400' },
  { valeur: 'moyenne', label: 'Moyenne', couleurPastille: 'bg-orange-400' },
  { valeur: 'faible', label: 'Faible', couleurPastille: 'bg-yellow-400' },
]

const paysDisponibles = computed(() => {
  const pays = new Set(contributions.value.map(c => c.localisation.pays))
  return Array.from(pays).sort()
})

const nombreCritiques = computed(() =>
  contributions.value.filter(c => c.problematique?.gravite === 'critique').length
)

const compterParGravite = (gravite: string) =>
  contributions.value.filter(c => c.problematique?.gravite === gravite).length

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
    if (graviteSelectionnee.value && c.problematique?.gravite !== graviteSelectionnee.value) {
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
  graviteSelectionnee.value = ''
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

const getBandeGraviteClass = (gravite?: string) => {
  const classes: Record<string, string> = {
    faible: 'bg-linear-to-r from-yellow-300 to-yellow-400',
    moyenne: 'bg-linear-to-r from-orange-400 to-orange-500',
    grave: 'bg-linear-to-r from-red-400 to-red-500',
    critique: 'bg-linear-to-r from-red-600 to-red-700'
  }
  return classes[gravite || ''] || 'bg-gray-200'
}

const getIconeGraviteClass = (gravite?: string) => {
  const classes: Record<string, string> = {
    faible: 'bg-yellow-100 text-yellow-600',
    moyenne: 'bg-orange-100 text-orange-600',
    grave: 'bg-red-100 text-red-600',
    critique: 'bg-red-600 text-white'
  }
  return classes[gravite || ''] || 'bg-gray-100 text-gray-500'
}

const getIconeGravite = (gravite?: string): string[] => {
  const icones: Record<string, string[]> = {
    faible: ['fas', 'info-circle'],
    moyenne: ['fas', 'exclamation-circle'],
    grave: ['fas', 'exclamation-triangle'],
    critique: ['fas', 'skull-crossbones']
  }
  return icones[gravite || ''] || ['fas', 'question-circle']
}

const formatDate = (date: Date) => {
  return new Intl.DateTimeFormat('fr-FR', { day: 'numeric', month: 'short', year: 'numeric' }).format(new Date(date))
}

onMounted(() => {
  contributions.value = getContributionsByType('badhabits')
})
</script>
