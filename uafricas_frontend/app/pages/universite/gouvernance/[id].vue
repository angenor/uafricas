<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Loading -->
    <div v-if="loading" class="flex items-center justify-center h-screen">
      <div class="flex flex-col items-center gap-4">
        <div class="w-16 h-16 rounded-full border-4 border-gray-200 border-t-custom-green animate-spin"></div>
        <p class="text-gray-500 text-sm font-medium">Chargement...</p>
      </div>
    </div>

    <!-- Non trouvé -->
    <div v-else-if="!contribution" class="flex flex-col items-center justify-center h-screen px-4">
      <div class="w-24 h-24 rounded-full bg-gray-100 flex items-center justify-center mb-6">
        <font-awesome-icon :icon="['fas', 'search']" class="text-gray-400 text-3xl" />
      </div>
      <h1 class="text-2xl font-bold text-gray-800 mb-2">Contribution non trouvée</h1>
      <p class="text-gray-500 mb-6">Cette contribution n'existe pas ou a été supprimée.</p>
      <NuxtLink to="/universite/gouvernance"
                 class="inline-flex items-center gap-2 px-5 py-2.5 bg-custom-green text-white rounded-lg hover:bg-custom-green/90 transition font-medium">
        <font-awesome-icon :icon="['fas', 'arrow-left']" class="text-sm" />
        Retour à la gouvernance
      </NuxtLink>
    </div>

    <!-- Contenu -->
    <template v-else>
      <!-- Hero -->
      <div class="relative h-72 md:h-80 overflow-hidden"
           :class="getTypeGradient(contribution.type)">
        <!-- Motif décoratif -->
        <div class="absolute inset-0 opacity-10"
             style="background-image: repeating-linear-gradient(45deg, transparent, transparent 35px, rgba(255,255,255,0.1) 35px, rgba(255,255,255,0.1) 70px);"></div>
        <div class="absolute -top-20 -right-20 w-80 h-80 rounded-full bg-white/5 animate-pulse"></div>
        <div class="absolute -bottom-24 -left-12 w-64 h-64 rounded-full bg-white/5 animate-pulse" style="animation-delay: 1s;"></div>

        <div class="absolute inset-0 bg-black/20"></div>
        <div class="absolute inset-0 flex flex-col justify-end px-4 md:px-8 pb-10">
          <div class="max-w-6xl mx-auto w-full">
            <CommonBreadcrumbNav class="mb-5 text-white/70" />
            <div class="flex flex-wrap items-center gap-3 mb-3">
              <span class="px-4 py-1.5 bg-white/20 backdrop-blur-sm text-white rounded-full text-sm font-semibold border border-white/20">
                <font-awesome-icon :icon="getTypeIcon(contribution.type)" class="mr-1.5" />
                {{ getTypeLabel(contribution.type) }}
              </span>
              <span v-if="contribution.problematique?.gravite"
                    class="px-3 py-1.5 rounded-full text-sm font-bold uppercase tracking-wide"
                    :class="getGraviteHeroClass(contribution.problematique.gravite)">
                {{ contribution.problematique.gravite }}
              </span>
              <span v-if="contribution.verified"
                    class="flex items-center gap-1.5 px-3 py-1.5 bg-green-500/20 backdrop-blur-sm text-green-300 rounded-full text-sm font-medium border border-green-400/30">
                <font-awesome-icon :icon="['fas', 'circle-check']" />
                Vérifié
              </span>
            </div>
            <h1 class="text-3xl md:text-4xl font-display font-bold text-white leading-tight">
              {{ contribution.titre }}
            </h1>
          </div>
        </div>
      </div>

      <!-- Contenu principal -->
      <div class="max-w-6xl mx-auto px-4 py-8 -mt-6 relative z-10">
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
          <!-- Colonne principale -->
          <div class="lg:col-span-2 space-y-6">
            <!-- Description -->
            <div class="bg-white rounded-xl shadow-lg p-8">
              <h2 class="text-xl font-bold text-gray-900 mb-4 flex items-center gap-2">
                <font-awesome-icon :icon="['fas', 'align-left']" class="text-gray-400" />
                Description
              </h2>
              <p class="text-gray-600 leading-relaxed text-[15px]">{{ contribution.description }}</p>
            </div>

            <!-- Contenu FactCheck -->
            <div v-if="contribution.type === 'factcheck' && contribution.factcheck" class="space-y-4">
              <div class="bg-white rounded-xl shadow-lg overflow-hidden">
                <div class="h-1.5 bg-linear-to-r from-red-500 to-red-600"></div>
                <div class="p-8">
                  <div class="flex items-start gap-4">
                    <div class="shrink-0 w-12 h-12 rounded-xl bg-red-100 text-red-600 flex items-center justify-center">
                      <font-awesome-icon :icon="['fas', 'times']" class="text-xl" />
                    </div>
                    <div>
                      <h3 class="font-bold text-red-800 mb-2 text-lg">Préjugé</h3>
                      <p class="font-semibold text-red-900 mb-2">{{ contribution.factcheck.prejuge.titre }}</p>
                      <p class="text-red-700/80 leading-relaxed">{{ contribution.factcheck.prejuge.description }}</p>
                    </div>
                  </div>
                </div>
              </div>
              <div class="bg-white rounded-xl shadow-lg overflow-hidden">
                <div class="h-1.5 bg-linear-to-r from-green-500 to-green-600"></div>
                <div class="p-8">
                  <div class="flex items-start gap-4">
                    <div class="shrink-0 w-12 h-12 rounded-xl bg-green-100 text-green-600 flex items-center justify-center">
                      <font-awesome-icon :icon="['fas', 'check']" class="text-xl" />
                    </div>
                    <div>
                      <h3 class="font-bold text-green-800 mb-2 text-lg">Réalité / Contre-argument</h3>
                      <p class="font-semibold text-green-900 mb-2">{{ contribution.factcheck.contrePrejuge.titre }}</p>
                      <p class="text-green-700/80 leading-relaxed">{{ contribution.factcheck.contrePrejuge.description }}</p>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- Contenu IdeaForces -->
            <div v-if="contribution.type === 'ideaforces' && contribution.proposition" class="bg-white rounded-xl shadow-lg overflow-hidden">
              <div class="h-1.5 bg-linear-to-r from-yellow-400 to-orange-500"></div>
              <div class="p-8">
                <h3 class="font-bold text-orange-800 mb-6 text-lg flex items-center gap-2">
                  <font-awesome-icon :icon="['fas', 'lightbulb']" class="text-orange-500" />
                  Proposition
                </h3>
                <div class="space-y-6">
                  <div>
                    <p class="text-xs font-semibold text-orange-500 uppercase tracking-wide mb-1">Objectif</p>
                    <p class="text-gray-800 leading-relaxed">{{ contribution.proposition.objectif }}</p>
                  </div>
                  <div v-if="contribution.proposition.moyens?.length">
                    <p class="text-xs font-semibold text-orange-500 uppercase tracking-wide mb-2">Moyens</p>
                    <ul class="space-y-2">
                      <li v-for="moyen in contribution.proposition.moyens" :key="moyen"
                          class="flex items-start gap-2 text-gray-700">
                        <font-awesome-icon :icon="['fas', 'chevron-right']" class="text-orange-400 text-xs mt-1.5 shrink-0" />
                        {{ moyen }}
                      </li>
                    </ul>
                  </div>
                  <div v-if="contribution.proposition.beneficiaires?.length">
                    <p class="text-xs font-semibold text-orange-500 uppercase tracking-wide mb-2">Bénéficiaires</p>
                    <div class="flex flex-wrap gap-2">
                      <span v-for="b in contribution.proposition.beneficiaires" :key="b"
                            class="px-3 py-1 bg-orange-100 text-orange-700 rounded-full text-sm font-medium">
                        {{ b }}
                      </span>
                    </div>
                  </div>
                  <div v-if="contribution.proposition.impact">
                    <p class="text-xs font-semibold text-orange-500 uppercase tracking-wide mb-1">Impact attendu</p>
                    <p class="text-gray-800 leading-relaxed">{{ contribution.proposition.impact }}</p>
                  </div>
                </div>
              </div>
            </div>

            <!-- Contenu BadHabits -->
            <div v-if="contribution.type === 'badhabits' && contribution.problematique" class="bg-white rounded-xl shadow-lg overflow-hidden">
              <div class="h-1.5" :class="getBandeGraviteClass(contribution.problematique.gravite)"></div>
              <div class="p-8">
                <h3 class="font-bold text-red-800 mb-6 text-lg flex items-center gap-2">
                  <font-awesome-icon :icon="['fas', 'exclamation-triangle']" class="text-red-500" />
                  Problématique signalée
                </h3>
                <div class="flex flex-wrap items-center gap-3">
                  <span class="px-4 py-2 bg-red-100 text-red-800 rounded-lg text-sm font-semibold flex items-center gap-2">
                    <font-awesome-icon :icon="['fas', 'tag']" class="text-red-400" />
                    {{ contribution.problematique.categorie }}
                  </span>
                  <span v-if="contribution.problematique.gravite"
                        class="px-4 py-2 rounded-lg text-sm font-bold flex items-center gap-2"
                        :class="getGraviteClass(contribution.problematique.gravite)">
                    <font-awesome-icon :icon="getGraviteIcon(contribution.problematique.gravite)" />
                    Gravité : {{ contribution.problematique.gravite }}
                  </span>
                </div>
              </div>
            </div>

            <!-- Sources -->
            <div v-if="contribution.sources?.length" class="bg-white rounded-xl shadow-lg p-8">
              <h3 class="font-bold text-gray-900 mb-4 flex items-center gap-2">
                <font-awesome-icon :icon="['fas', 'link']" class="text-gray-400" />
                Sources
              </h3>
              <ul class="space-y-3">
                <li v-for="source in contribution.sources" :key="source.url">
                  <a :href="source.url" target="_blank" rel="noopener"
                     class="flex items-center gap-3 px-4 py-3 rounded-lg bg-gray-50 hover:bg-blue-50 border border-gray-100 hover:border-blue-200 transition group">
                    <font-awesome-icon :icon="['fas', 'arrow-up-right-from-square']" class="text-gray-400 group-hover:text-blue-500 transition text-sm" />
                    <span class="text-gray-700 group-hover:text-blue-700 transition font-medium">{{ source.titre }}</span>
                  </a>
                </li>
              </ul>
            </div>

            <!-- Tags -->
            <div v-if="contribution.tags?.length" class="bg-white rounded-xl shadow-lg p-8">
              <h3 class="font-bold text-gray-900 mb-4 flex items-center gap-2">
                <font-awesome-icon :icon="['fas', 'tags']" class="text-gray-400" />
                Tags
              </h3>
              <div class="flex flex-wrap gap-2">
                <span v-for="tag in contribution.tags" :key="tag"
                      class="px-3 py-1.5 bg-gray-100 text-gray-600 rounded-full text-sm font-medium hover:bg-gray-200 transition cursor-default">
                  #{{ tag }}
                </span>
              </div>
            </div>
          </div>

          <!-- Sidebar -->
          <div class="space-y-6">
            <!-- Auteur -->
            <div class="bg-white rounded-xl shadow-lg overflow-hidden">
              <div class="h-20 bg-linear-to-r from-gray-700 to-gray-800 relative">
                <div class="absolute -bottom-8 left-6">
                  <img v-if="contribution.auteur.photoURL"
                       :src="contribution.auteur.photoURL"
                       :alt="`${contribution.auteur.prenom} ${contribution.auteur.nom}`"
                       class="w-16 h-16 rounded-xl object-cover border-4 border-white shadow-md">
                  <div v-else class="w-16 h-16 rounded-xl bg-gray-200 border-4 border-white shadow-md flex items-center justify-center">
                    <font-awesome-icon :icon="['fas', 'user']" class="text-gray-400 text-xl" />
                  </div>
                </div>
              </div>
              <div class="pt-12 pb-6 px-6">
                <p class="font-bold text-gray-900">{{ contribution.auteur.prenom }} {{ contribution.auteur.nom }}</p>
                <p class="text-sm text-gray-500 flex items-center gap-1.5 mt-1">
                  <font-awesome-icon :icon="['fas', 'calendar-alt']" class="text-xs" />
                  {{ formatDate(contribution.dateCreation) }}
                </p>
              </div>
            </div>

            <!-- Localisation -->
            <div class="bg-white rounded-xl shadow-lg p-6">
              <h3 class="font-bold text-gray-900 mb-4 flex items-center gap-2 text-sm">
                <font-awesome-icon :icon="['fas', 'map-marker-alt']" class="text-red-400" />
                Localisation
              </h3>
              <div class="flex items-center gap-3 px-4 py-3 bg-gray-50 rounded-lg">
                <font-awesome-icon :icon="['fas', 'earth-africa']" class="text-custom-green text-lg" />
                <div>
                  <p class="font-semibold text-gray-800">{{ contribution.localisation.pays }}</p>
                  <p v-if="contribution.localisation.ville" class="text-sm text-gray-500">
                    {{ contribution.localisation.ville }}
                    <span v-if="contribution.localisation.region">, {{ contribution.localisation.region }}</span>
                  </p>
                </div>
              </div>
            </div>

            <!-- Statistiques -->
            <div class="bg-white rounded-xl shadow-lg p-6">
              <h3 class="font-bold text-gray-900 mb-4 flex items-center gap-2 text-sm">
                <font-awesome-icon :icon="['fas', 'chart-line']" class="text-blue-400" />
                Statistiques
              </h3>
              <div class="grid grid-cols-2 gap-3">
                <div class="text-center p-4 rounded-xl bg-blue-50 border border-blue-100">
                  <p class="text-2xl font-bold text-blue-600">{{ contribution.stats.vues }}</p>
                  <p class="text-xs text-blue-500 font-medium mt-1">Vues</p>
                </div>
                <div class="text-center p-4 rounded-xl bg-red-50 border border-red-100">
                  <p class="text-2xl font-bold text-red-600">{{ contribution.stats.likes }}</p>
                  <p class="text-xs text-red-500 font-medium mt-1">Likes</p>
                </div>
                <div class="text-center p-4 rounded-xl bg-green-50 border border-green-100">
                  <p class="text-2xl font-bold text-green-600">{{ contribution.stats.commentaires }}</p>
                  <p class="text-xs text-green-500 font-medium mt-1">Commentaires</p>
                </div>
                <div class="text-center p-4 rounded-xl bg-purple-50 border border-purple-100">
                  <p class="text-2xl font-bold text-purple-600">{{ contribution.stats.partages }}</p>
                  <p class="text-xs text-purple-500 font-medium mt-1">Partages</p>
                </div>
              </div>
            </div>

            <!-- Actions -->
            <div class="bg-white rounded-xl shadow-lg p-6 space-y-3">
              <button class="w-full py-3 px-4 bg-linear-to-r from-red-500 to-red-600 text-white rounded-xl hover:from-red-600 hover:to-red-700 transition-all shadow-md hover:shadow-lg font-semibold flex items-center justify-center gap-2">
                <font-awesome-icon :icon="['fas', 'heart']" />
                J'aime
              </button>
              <button class="w-full py-3 px-4 border-2 border-gray-200 text-gray-700 rounded-xl hover:bg-gray-50 hover:border-gray-300 transition-all font-semibold flex items-center justify-center gap-2">
                <font-awesome-icon :icon="['fas', 'share-nodes']" />
                Partager
              </button>
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

const getTypeIcon = (type: string): string[] => {
  const icons: Record<string, string[]> = {
    factcheck: ['fas', 'eye'],
    badhabits: ['fas', 'exclamation-triangle'],
    ideaforces: ['fas', 'lightbulb']
  }
  return icons[type] || ['fas', 'file-lines']
}

const getTypeGradient = (type: string) => {
  const gradients: Record<string, string> = {
    factcheck: 'bg-linear-to-br from-blue-700 via-blue-600 to-indigo-700',
    badhabits: 'bg-linear-to-br from-red-900 via-red-700 to-orange-600',
    ideaforces: 'bg-linear-to-br from-amber-600 via-orange-500 to-yellow-500'
  }
  return gradients[type] || 'bg-linear-to-br from-gray-700 to-gray-900'
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

const getGraviteHeroClass = (gravite: string) => {
  const classes: Record<string, string> = {
    faible: 'bg-yellow-500/20 text-yellow-200 border border-yellow-400/30',
    moyenne: 'bg-orange-500/20 text-orange-200 border border-orange-400/30',
    grave: 'bg-red-500/30 text-red-200 border border-red-400/30',
    critique: 'bg-red-600 text-white border border-red-400/50'
  }
  return classes[gravite] || ''
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

const getGraviteIcon = (gravite: string): string[] => {
  const icons: Record<string, string[]> = {
    faible: ['fas', 'info-circle'],
    moyenne: ['fas', 'exclamation-circle'],
    grave: ['fas', 'exclamation-triangle'],
    critique: ['fas', 'skull-crossbones']
  }
  return icons[gravite] || ['fas', 'question-circle']
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
