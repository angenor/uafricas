<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section (compact, titre ↔ description au survol) -->
    <div class="group relative overflow-hidden bg-linear-to-br from-custom-chocolat via-amber-700 to-custom-green">
      <div class="absolute inset-0 opacity-10"
           style="background-image: repeating-linear-gradient(135deg, transparent, transparent 35px, rgba(255,255,255,0.1) 35px, rgba(255,255,255,0.1) 70px);"></div>

      <div class="relative max-w-4xl mx-auto px-4 pt-16 pb-6 text-center select-none">
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12">
          <h1 class="absolute inset-0 flex items-center justify-center text-white text-2xl md:text-4xl font-display font-bold tracking-tight transition-opacity duration-300 group-hover:opacity-0">
            Publications
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Un flux unique pour toutes les voix d'Afrique
          </p>
        </div>
      </div>
    </div>

    <!-- Contenu -->
    <div class="max-w-7xl mx-auto px-4 py-8 relative z-10">
      <!-- Barre de navigation -->
      <div class="bg-white rounded-xl shadow-lg p-5 mb-8 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
        <CommonBreadcrumbNav :custom-breadcrumbs="breadcrumbs" />
        <div class="text-sm text-gray-500">
          <span class="font-semibold text-gray-900">{{ publications.length }}</span> publication{{ publications.length > 1 ? 's' : '' }} au total
        </div>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-4 gap-8">
        <!-- Sidebar — Filtres par catégorie -->
        <div class="lg:col-span-1">
          <div class="bg-white rounded-xl shadow-lg overflow-hidden sticky top-4">
            <div class="bg-linear-to-r from-custom-chocolat to-amber-700 px-6 py-4">
              <h3 class="text-white font-bold flex items-center gap-2">
                <font-awesome-icon :icon="['fas', 'filter']" />
                Filtres
              </h3>
            </div>

            <div class="p-6 space-y-5">
              <!-- Catégories -->
              <div>
                <label class="block text-xs font-semibold text-gray-500 uppercase tracking-wide mb-3">
                  Catégories
                </label>
                <div class="space-y-2">
                  <button
                    v-for="filtre in filtres"
                    :key="filtre.value"
                    @click="activeFilter = filtre.value"
                    class="w-full flex items-center justify-between gap-2 px-3 py-2.5 rounded-lg text-sm font-medium border transition-all duration-200"
                    :class="activeFilter === filtre.value
                      ? `${filtre.activeClasses} shadow-sm`
                      : 'bg-white text-gray-700 border-gray-200 hover:border-gray-300 hover:bg-gray-50'"
                  >
                    <span class="flex items-center gap-2">
                      <font-awesome-icon :icon="filtre.icon" class="text-xs" />
                      <span>{{ filtre.label }}</span>
                    </span>
                    <span
                      class="px-2 py-0.5 text-xs rounded-full"
                      :class="activeFilter === filtre.value ? 'bg-white/30' : 'bg-gray-100 text-gray-600'"
                    >
                      {{ compteurs[filtre.value] }}
                    </span>
                  </button>
                </div>
              </div>

              <!-- Recherche -->
              <div>
                <label class="block text-xs font-semibold text-gray-500 uppercase tracking-wide mb-2">
                  Recherche
                </label>
                <div class="relative">
                  <font-awesome-icon :icon="['fas', 'magnifying-glass']" class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 text-sm" />
                  <input v-model="recherche"
                         type="text"
                         placeholder="Mot-clé..."
                         class="w-full pl-10 pr-4 py-2.5 border border-gray-200 rounded-lg focus:ring-2 focus:ring-custom-green/30 focus:border-custom-green transition text-sm">
                </div>
              </div>

              <!-- Pays -->
              <div>
                <label class="block text-xs font-semibold text-gray-500 uppercase tracking-wide mb-2">Territoire</label>
                <select v-model="paysSelectionne" class="w-full px-3 py-2.5 border border-gray-200 rounded-lg focus:ring-2 focus:ring-custom-green/30 focus:border-custom-green transition bg-white text-sm">
                  <option value="">Tous les territoires</option>
                  <option v-for="pays in paysDisponibles" :key="pays" :value="pays">{{ pays }}</option>
                </select>
              </div>

              <!-- Reset -->
              <button
                v-if="hasFiltresActifs"
                @click="reinitialiser"
                class="w-full py-2.5 text-sm font-medium text-gray-500 border border-gray-200 rounded-lg hover:bg-gray-50 hover:text-gray-700 transition flex items-center justify-center gap-2"
              >
                <font-awesome-icon :icon="['fas', 'rotate-left']" class="text-xs" />
                Réinitialiser
              </button>
            </div>
          </div>
        </div>

        <!-- Liste principale -->
        <div class="lg:col-span-3">
          <!-- État de chargement -->
          <div v-if="loading" class="flex justify-center py-16 bg-white rounded-xl shadow-md">
            <div class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-custom-green"></div>
          </div>

          <!-- État d'erreur -->
          <div v-else-if="erreurChargement" class="py-16 text-center bg-white rounded-xl shadow-md">
            <div class="inline-flex items-center justify-center w-20 h-20 rounded-full bg-red-50 mb-4">
              <font-awesome-icon :icon="['fas', 'triangle-exclamation']" class="text-3xl text-red-400" />
            </div>
            <h3 class="text-lg font-semibold text-gray-600">Erreur de chargement</h3>
            <p class="text-gray-400 mt-1 text-sm">{{ erreurChargement }}</p>
            <button
              @click="chargerTout"
              class="mt-4 px-5 py-2.5 bg-linear-to-r from-custom-green to-green-600 text-white rounded-lg transition-all duration-300 transform hover:scale-105 text-sm font-medium"
            >
              <font-awesome-icon :icon="['fas', 'rotate-right']" class="mr-2" />
              Réessayer
            </button>
          </div>

          <template v-else>
            <!-- Résultats count + titre -->
            <div class="flex items-center justify-between mb-4">
              <p class="text-sm text-gray-500">
                <span class="font-semibold text-gray-900">{{ publicationsFiltrees.length }}</span> résultat{{ publicationsFiltrees.length > 1 ? 's' : '' }}
                <span v-if="hasFiltresActifs"> (filtré{{ publicationsFiltrees.length > 1 ? 's' : '' }})</span>
              </p>
            </div>

            <!-- État vide -->
            <div v-if="publicationsFiltrees.length === 0" class="text-center py-20 bg-white rounded-xl shadow-lg">
              <div class="w-20 h-20 mx-auto mb-6 rounded-full bg-gray-100 flex items-center justify-center">
                <font-awesome-icon :icon="['fas', 'magnifying-glass']" class="text-gray-400 text-2xl" />
              </div>
              <p class="text-gray-900 font-semibold text-lg mb-2">Aucun résultat trouvé</p>
              <p class="text-gray-500 text-sm">Essayez de modifier vos filtres de recherche</p>
            </div>

            <!-- Cartes unifiées (flux trié par date, toutes sources confondues) -->
            <TransitionGroup v-else name="feed" tag="div" class="space-y-5">
              <template v-for="pub in publicationsFiltrees" :key="pub.key">
              <!-- Carte « Territoire partagé » (composant dédié) -->
              <PublicationsTerritoirePartageCard
                v-if="pub.source === 'territoire_partage'"
                :partage="pub.data"
              />

              <!-- Carte « Découverte partagée » (secteur/recette/site/personnalité) -->
              <PublicationsElementPartageCard
                v-else-if="pub.source === 'element_partage'"
                :partage="pub.data"
              />

              <!-- Carte « Profil partagé » (composant dédié) -->
              <PublicationsProfilPartageCard
                v-else-if="pub.source === 'profil_partage'"
                :partage="pub.data"
              />

              <!-- Carte « Contribution partagée » (composant dédié) -->
              <PublicationsContributionPartageCard
                v-else-if="pub.source === 'contribution_partage'"
                :partage="pub.data"
              />

              <!-- Carte « Vidéo partagée » (composant dédié) -->
              <PublicationsVideoPartageCard
                v-else-if="pub.source === 'video_partage'"
                :partage="pub.data"
              />

              <!-- Cartes Codimoi / Gouvernance (markup inline) -->
              <div
                v-else
                class="group bg-white rounded-xl shadow-md hover:shadow-xl transition-all duration-300 overflow-hidden border border-gray-100 hover:border-gray-200"
                :class="{ 'cursor-pointer': pub.source === 'codimoi' }"
                @click="ouvrirPublication(pub)"
              >
                <!-- Bande colorée -->
                <div class="h-1.5" :class="pub.typeStyle.bande"></div>

                <div class="p-6">
                  <div class="flex items-start gap-4">
                    <!-- Icône -->
                    <div class="shrink-0 w-12 h-12 rounded-xl flex items-center justify-center"
                         :class="pub.typeStyle.iconeBg">
                      <font-awesome-icon :icon="pub.typeStyle.icone" class="text-lg" />
                    </div>

                    <div class="flex-1 min-w-0">
                      <!-- Badges -->
                      <div class="flex flex-wrap items-center gap-2 mb-2">
                        <span class="px-2.5 py-0.5 rounded-full text-xs font-bold uppercase tracking-wide"
                              :class="pub.typeStyle.badge">
                          {{ pub.typeStyle.label }}
                        </span>
                        <!-- Sous-badge spécifique -->
                        <span v-if="pub.source === 'gouvernance' && pub.data.verified"
                              class="flex items-center gap-1 px-2.5 py-0.5 bg-green-100 text-green-700 rounded-full text-xs font-semibold">
                          <font-awesome-icon :icon="['fas', 'circle-check']" class="text-[10px]" />
                          Vérifié
                        </span>
                        <span v-if="pub.source === 'gouvernance' && pub.data.problematique?.gravite"
                              class="px-2.5 py-0.5 rounded-full text-xs font-bold uppercase tracking-wide"
                              :class="getGraviteClasses(pub.data.problematique.gravite)">
                          {{ pub.data.problematique.gravite }}
                        </span>
                      </div>

                      <!-- Titre -->
                      <h3 class="text-lg font-bold text-gray-900 mb-2 transition-colors line-clamp-2"
                          :class="pub.typeStyle.titreHover">
                        {{ pub.source === 'codimoi' ? titreCodimoi(pub.data) : pub.data.titre }}
                      </h3>

                      <!-- Description -->
                      <p class="text-gray-500 text-sm leading-relaxed mb-4 line-clamp-2">
                        {{ pub.source === 'codimoi' ? (pub.data.explication || pub.data.contenu) : pub.data.description }}
                      </p>

                      <!-- Contenu spécifique FactCheck : préjugé vs réalité -->
                      <div v-if="pub.source === 'gouvernance' && pub.data.type === 'factcheck' && pub.data.factcheck"
                           class="grid grid-cols-1 md:grid-cols-2 gap-3 mb-4">
                        <div class="p-3 bg-red-50 rounded-lg border-l-4 border-red-400">
                          <p class="text-xs font-bold text-red-600 uppercase tracking-wide mb-1">
                            <font-awesome-icon :icon="['fas', 'xmark']" class="mr-1" />Préjugé
                          </p>
                          <p class="text-red-800 text-sm line-clamp-2">{{ pub.data.factcheck.prejuge.titre }}</p>
                        </div>
                        <div class="p-3 bg-green-50 rounded-lg border-l-4 border-green-400">
                          <p class="text-xs font-bold text-green-600 uppercase tracking-wide mb-1">
                            <font-awesome-icon :icon="['fas', 'check']" class="mr-1" />Réalité
                          </p>
                          <p class="text-green-800 text-sm line-clamp-2">{{ pub.data.factcheck.contrePrejuge.titre }}</p>
                        </div>
                      </div>

                      <!-- Contenu spécifique IdeaForces -->
                      <div v-if="pub.source === 'gouvernance' && pub.data.type === 'ideaforces' && pub.data.proposition"
                           class="p-3 bg-amber-50 rounded-lg border-l-4 border-amber-400 mb-4">
                        <p class="text-xs font-bold text-amber-700 uppercase tracking-wide mb-1">
                          <font-awesome-icon :icon="['fas', 'lightbulb']" class="mr-1" />Objectif
                        </p>
                        <p class="text-amber-900 text-sm line-clamp-2">{{ pub.data.proposition.objectif }}</p>
                      </div>

                      <!-- Contenu spécifique BadHabits -->
                      <div v-if="pub.source === 'gouvernance' && pub.data.type === 'badhabits' && pub.data.problematique"
                           class="p-3 bg-red-50 rounded-lg border-l-4 border-red-400 mb-4">
                        <p class="text-xs font-bold text-red-600 uppercase tracking-wide mb-1">
                          <font-awesome-icon :icon="['fas', 'tag']" class="mr-1" />{{ pub.data.problematique.categorie }}
                        </p>
                        <p v-if="pub.data.problematique.urgence" class="text-red-800 text-sm">
                          Urgence : {{ pub.data.problematique.urgence }}
                        </p>
                      </div>

                      <!-- Contenu spécifique Codimoi (proverbe/citation) -->
                      <div v-if="pub.source === 'codimoi' && isQuoteType(pub.data.type)"
                           class="p-4 rounded-lg text-white text-center mb-4"
                           :style="{ backgroundColor: pub.data.couleur_fond || '#2D5A27' }">
                        <p class="text-sm italic leading-relaxed line-clamp-3">
                          « {{ pub.data.contenu }} »
                        </p>
                        <p v-if="pub.data.nom_auteur_originel" class="text-xs opacity-90 mt-2">
                          — {{ pub.data.nom_auteur_originel }}
                        </p>
                      </div>

                      <!-- Hashtags Codimoi -->
                      <div v-if="pub.source === 'codimoi' && pub.data.hashtags && pub.data.hashtags.length"
                           class="flex flex-wrap gap-1.5 mb-4">
                        <span v-for="tag in pub.data.hashtags.slice(0, 3)"
                              :key="tag"
                              class="px-2 py-0.5 bg-custom-green/10 text-custom-green text-xs rounded-full font-medium">
                          #{{ tag }}
                        </span>
                      </div>

                      <!-- Tags Gouvernance -->
                      <div v-if="pub.source === 'gouvernance' && pub.data.tags && pub.data.tags.length"
                           class="flex flex-wrap gap-1.5 mb-4">
                        <span v-for="tag in pub.data.tags.slice(0, 3)"
                              :key="tag"
                              class="px-2 py-0.5 bg-gray-100 text-gray-600 text-xs rounded-full font-medium">
                          #{{ tag }}
                        </span>
                      </div>

                      <!-- Métadonnées -->
                      <div class="flex flex-wrap items-center gap-x-5 gap-y-2 text-xs text-gray-400">
                        <span class="flex items-center gap-1.5">
                          <font-awesome-icon :icon="['fas', 'user']" />
                          {{ nomAuteur(pub) }}
                        </span>
                        <span v-if="paysPub(pub)" class="flex items-center gap-1.5">
                          <font-awesome-icon :icon="['fas', 'location-dot']" />
                          {{ paysPub(pub) }}
                        </span>
                        <span class="flex items-center gap-1.5">
                          <font-awesome-icon :icon="['fas', 'calendar-alt']" />
                          {{ formatDate(pub.date) }}
                        </span>
                      </div>
                    </div>

                    <!-- Flèche -->
                    <div class="shrink-0 hidden sm:flex items-center">
                      <div class="w-8 h-8 rounded-full bg-gray-100 flex items-center justify-center transition-all group-hover:translate-x-1"
                           :class="pub.typeStyle.flecheHoverBg">
                        <font-awesome-icon :icon="['fas', 'chevron-right']" class="text-gray-400 text-xs transition-colors"
                                           :class="pub.typeStyle.flecheHoverColor" />
                      </div>
                    </div>
                  </div>

                  <!-- Stats footer -->
                  <div class="flex items-center gap-6 mt-5 pt-4 border-t border-gray-100 text-xs text-gray-400">
                    <span class="flex items-center gap-1.5 hover:text-gray-600 transition">
                      <font-awesome-icon :icon="['fas', 'eye']" />
                      {{ statsVues(pub) }} vues
                    </span>
                    <span class="flex items-center gap-1.5 hover:text-red-500 transition">
                      <font-awesome-icon :icon="['fas', 'heart']" />
                      {{ statsLikes(pub) }} likes
                    </span>
                    <span class="flex items-center gap-1.5 hover:text-blue-500 transition">
                      <font-awesome-icon :icon="['fas', 'comment']" />
                      {{ statsCommentaires(pub) }} commentaires
                    </span>
                  </div>
                </div>
              </div>
              </template>
            </TransitionGroup>
          </template>
        </div>
      </div>
    </div>

    <!-- Toast -->
    <Transition name="toast">
      <div
        v-if="showToast"
        class="fixed bottom-6 left-1/2 -translate-x-1/2 z-50 bg-gray-800 text-white px-5 py-3 rounded-lg shadow-lg text-sm flex items-center gap-2"
      >
        <font-awesome-icon :icon="['fas', 'circle-check']" class="text-custom-green" />
        {{ toastMessage }}
      </div>
    </Transition>

    <!-- Modale détail Codimoi -->
    <CodiMoiPostModal
      :post="selectedPost"
      :commentaires="selectedPostCommentaires"
      :chargement-commentaires="chargementCommentaires"
      @close="fermerPostCodimoi"
      @like="reagirModalCodimoi('like')"
      @dislike="reagirModalCodimoi('dislike')"
      @share="partagerModalCodimoi"
      @commenter="commenterModalCodimoi"
    />
  </div>
</template>

<script setup lang="ts">
import { getCategoryLabel, type CodiMoiPostAPI, type CategoriePost, type CommentaireAPI } from '~/composables/useCodiMoi'
import type { PartageFicheAPI, PartageElementAPI } from '~/composables/useOpportuniteAfrique'
import type { PartageProfilAPI } from '~/composables/useMembres'
import type { PartageContributionAPI } from '~/composables/useGouvernance'
import type { PartageVideoAPI } from '~/composables/useVidafrica'
import type { ContributionCitoyenne } from '~/types/gouvernance'

useHead({
  title: 'Publications de la Communauté | UAfricas',
})

type FiltreValue = 'tous' | 'codimoi' | 'factcheck' | 'ideaforces' | 'badhabits' | 'territoire_partage' | 'element_partage' | 'profil_partage' | 'video_partage'

interface TypeStyle {
  label: string
  icone: string[]
  iconeBg: string
  badge: string
  bande: string
  titreHover: string
  flecheHoverBg: string
  flecheHoverColor: string
}

interface PublicationCodimoi {
  key: string
  source: 'codimoi'
  data: CodiMoiPostAPI
  date: Date
  typeFiltre: 'codimoi'
  typeStyle: TypeStyle
}

interface PublicationGouvernance {
  key: string
  source: 'gouvernance'
  data: ContributionCitoyenne
  date: Date
  typeFiltre: 'factcheck' | 'ideaforces' | 'badhabits'
  typeStyle: TypeStyle
}

interface PublicationTerritoirePartage {
  key: string
  source: 'territoire_partage'
  data: PartageFicheAPI
  date: Date
  typeFiltre: 'territoire_partage'
  typeStyle: TypeStyle
}

interface PublicationElementPartage {
  key: string
  source: 'element_partage'
  data: PartageElementAPI
  date: Date
  typeFiltre: 'element_partage'
  typeStyle: TypeStyle
}

interface PublicationProfilPartage {
  key: string
  source: 'profil_partage'
  data: PartageProfilAPI
  date: Date
  typeFiltre: 'profil_partage'
  typeStyle: TypeStyle
}

interface PublicationContributionPartage {
  key: string
  source: 'contribution_partage'
  data: PartageContributionAPI
  date: Date
  typeFiltre: 'factcheck' | 'ideaforces' | 'badhabits'
  typeStyle: TypeStyle
}

interface PublicationVideoPartage {
  key: string
  source: 'video_partage'
  data: PartageVideoAPI
  date: Date
  typeFiltre: 'video_partage'
  typeStyle: TypeStyle
}

type Publication = PublicationCodimoi | PublicationGouvernance | PublicationTerritoirePartage | PublicationElementPartage | PublicationProfilPartage | PublicationContributionPartage | PublicationVideoPartage

const breadcrumbs = [
  { label: 'Publications', to: undefined },
]

const filtres: { value: FiltreValue; label: string; icon: string[]; activeClasses: string }[] = [
  { value: 'tous', label: 'Toutes', icon: ['fas', 'layer-group'], activeClasses: 'bg-gray-800 text-white border-gray-800' },
  { value: 'codimoi', label: 'Codimoi', icon: ['fas', 'quote-left'], activeClasses: 'bg-custom-chocolat text-white border-custom-chocolat' },
  { value: 'factcheck', label: 'FactCheck', icon: ['fas', 'magnifying-glass'], activeClasses: 'bg-blue-600 text-white border-blue-600' },
  { value: 'ideaforces', label: 'IdeaForces', icon: ['fas', 'lightbulb'], activeClasses: 'bg-amber-500 text-white border-amber-500' },
  { value: 'badhabits', label: 'BadHabits', icon: ['fas', 'triangle-exclamation'], activeClasses: 'bg-red-600 text-white border-red-600' },
  { value: 'territoire_partage', label: 'Territoires partagés', icon: ['fas', 'earth-africa'], activeClasses: 'bg-custom-green text-white border-custom-green' },
  { value: 'element_partage', label: 'Découvertes partagées', icon: ['fas', 'share-nodes'], activeClasses: 'bg-emerald-600 text-white border-emerald-600' },
  { value: 'profil_partage', label: 'Profils partagés', icon: ['fas', 'user'], activeClasses: 'bg-custom-chocolat text-white border-custom-chocolat' },
  { value: 'video_partage', label: 'Vidéos partagées', icon: ['fas', 'video'], activeClasses: 'bg-purple-600 text-white border-purple-600' },
]

const STYLES_PAR_TYPE: Record<'codimoi' | 'factcheck' | 'ideaforces' | 'badhabits' | 'territoire_partage' | 'element_partage' | 'profil_partage' | 'video_partage', TypeStyle> = {
  codimoi: {
    label: 'Codimoi',
    icone: ['fas', 'quote-left'],
    iconeBg: 'bg-amber-100 text-custom-chocolat',
    badge: 'bg-amber-100 text-custom-chocolat',
    bande: 'bg-linear-to-r from-custom-chocolat to-amber-600',
    titreHover: 'group-hover:text-custom-chocolat',
    flecheHoverBg: 'group-hover:bg-amber-100',
    flecheHoverColor: 'group-hover:text-custom-chocolat',
  },
  factcheck: {
    label: 'FactCheck',
    icone: ['fas', 'magnifying-glass'],
    iconeBg: 'bg-blue-100 text-blue-600',
    badge: 'bg-blue-100 text-blue-700',
    bande: 'bg-linear-to-r from-blue-500 to-indigo-500',
    titreHover: 'group-hover:text-blue-700',
    flecheHoverBg: 'group-hover:bg-blue-100',
    flecheHoverColor: 'group-hover:text-blue-600',
  },
  ideaforces: {
    label: 'IdeaForces',
    icone: ['fas', 'lightbulb'],
    iconeBg: 'bg-amber-100 text-amber-600',
    badge: 'bg-amber-100 text-amber-700',
    bande: 'bg-linear-to-r from-amber-400 to-orange-500',
    titreHover: 'group-hover:text-amber-700',
    flecheHoverBg: 'group-hover:bg-amber-100',
    flecheHoverColor: 'group-hover:text-amber-600',
  },
  badhabits: {
    label: 'BadHabits',
    icone: ['fas', 'triangle-exclamation'],
    iconeBg: 'bg-red-100 text-red-600',
    badge: 'bg-red-100 text-red-700',
    bande: 'bg-linear-to-r from-red-500 to-rose-500',
    titreHover: 'group-hover:text-red-700',
    flecheHoverBg: 'group-hover:bg-red-100',
    flecheHoverColor: 'group-hover:text-red-600',
  },
  territoire_partage: {
    label: 'Territoire',
    icone: ['fas', 'earth-africa'],
    iconeBg: 'bg-green-100 text-custom-green',
    badge: 'bg-green-100 text-custom-green',
    bande: 'bg-linear-to-r from-custom-green to-emerald-600',
    titreHover: 'group-hover:text-custom-green',
    flecheHoverBg: 'group-hover:bg-green-100',
    flecheHoverColor: 'group-hover:text-custom-green',
  },
  element_partage: {
    label: 'Découverte',
    icone: ['fas', 'share-nodes'],
    iconeBg: 'bg-emerald-100 text-emerald-600',
    badge: 'bg-emerald-100 text-emerald-700',
    bande: 'bg-linear-to-r from-emerald-500 to-green-600',
    titreHover: 'group-hover:text-emerald-700',
    flecheHoverBg: 'group-hover:bg-emerald-100',
    flecheHoverColor: 'group-hover:text-emerald-600',
  },
  profil_partage: {
    label: 'Profil',
    icone: ['fas', 'user'],
    iconeBg: 'bg-amber-100 text-custom-chocolat',
    badge: 'bg-amber-100 text-custom-chocolat',
    bande: 'bg-linear-to-r from-custom-chocolat to-amber-700',
    titreHover: 'group-hover:text-custom-chocolat',
    flecheHoverBg: 'group-hover:bg-amber-100',
    flecheHoverColor: 'group-hover:text-custom-chocolat',
  },
  video_partage: {
    label: 'Vidéo',
    icone: ['fas', 'video'],
    iconeBg: 'bg-purple-100 text-purple-600',
    badge: 'bg-purple-100 text-purple-700',
    bande: 'bg-linear-to-r from-purple-600 to-fuchsia-600',
    titreHover: 'group-hover:text-purple-700',
    flecheHoverBg: 'group-hover:bg-purple-100',
    flecheHoverColor: 'group-hover:text-purple-600',
  },
}

const activeFilter = ref<FiltreValue>('tous')
const recherche = ref('')
const paysSelectionne = ref('')
const publications = ref<Publication[]>([])
const loading = ref(false)
const erreurChargement = ref<string | null>(null)

// APIs
const { erreur: erreurCodimoi, listerPosts, reagir, listerCommentaires, creerCommentaire } = useCodiMoi()
const { getContributions, listerPartagesContributions } = useGouvernance()
const { listerPartagesFiches, listerPartagesElements } = useOpportuniteAfrique()
const { listerPartagesProfils } = useMembres()
const { listerPartagesVideos } = useVidafrica()

// Toast
const showToast = ref(false)
const toastMessage = ref('')

const notifier = (message: string) => {
  toastMessage.value = message
  showToast.value = true
  setTimeout(() => { showToast.value = false }, 2500)
}

// Modale codimoi
const selectedPost = ref<CodiMoiPostAPI | null>(null)
const selectedPostCommentaires = ref<CommentaireAPI[]>([])
const chargementCommentaires = ref(false)

// Compteurs par type
const compteurs = computed<Record<FiltreValue, number>>(() => {
  const c: Record<FiltreValue, number> = {
    tous: publications.value.length,
    codimoi: 0,
    factcheck: 0,
    ideaforces: 0,
    badhabits: 0,
    territoire_partage: 0,
    element_partage: 0,
    profil_partage: 0,
    video_partage: 0,
  }
  for (const p of publications.value) {
    c[p.typeFiltre]++
  }
  return c
})

const paysDisponibles = computed(() => {
  const pays = new Set<string>()
  for (const p of publications.value) {
    const pays_ = paysPub(p)
    if (pays_) pays.add(pays_)
  }
  return Array.from(pays).sort()
})

const hasFiltresActifs = computed(() =>
  activeFilter.value !== 'tous' || !!recherche.value || !!paysSelectionne.value
)

const publicationsFiltrees = computed<Publication[]>(() => {
  return publications.value.filter(p => {
    // Filtre par catégorie
    if (activeFilter.value !== 'tous' && p.typeFiltre !== activeFilter.value) return false

    // Filtre par recherche
    if (recherche.value) {
      const q = recherche.value.toLowerCase()
      let titre: string
      let desc: string
      if (p.source === 'codimoi') {
        titre = titreCodimoi(p.data).toLowerCase()
        desc = (p.data.explication || p.data.contenu).toLowerCase()
      }
      else if (p.source === 'territoire_partage') {
        titre = p.data.fiche.nom.toLowerCase()
        desc = `${p.data.legende ?? ''} ${p.data.fiche.slogan ?? ''} ${p.data.fiche.region ?? ''} ${p.data.fiche.capitale ?? ''}`.toLowerCase()
      }
      else if (p.source === 'element_partage') {
        titre = p.data.element.titre.toLowerCase()
        desc = `${p.data.legende ?? ''} ${p.data.element.territoire_nom ?? ''}`.toLowerCase()
      }
      else if (p.source === 'profil_partage') {
        titre = `${p.data.profil.prenom} ${p.data.profil.nom}`.toLowerCase()
        desc = `${p.data.legende ?? ''} ${p.data.profil.fonction ?? ''} ${p.data.profil.pays ?? ''} ${p.data.profil.ville ?? ''}`.toLowerCase()
      }
      else if (p.source === 'contribution_partage') {
        titre = p.data.contribution.titre.toLowerCase()
        desc = `${p.data.legende ?? ''} ${p.data.contribution.description ?? ''} ${p.data.contribution.categorie ?? ''}`.toLowerCase()
      }
      else if (p.source === 'video_partage') {
        titre = p.data.video.titre.toLowerCase()
        desc = `${p.data.legende ?? ''}`.toLowerCase()
      }
      else {
        titre = p.data.titre.toLowerCase()
        desc = p.data.description.toLowerCase()
      }
      if (!titre.includes(q) && !desc.includes(q)) return false
    }

    // Filtre par pays
    if (paysSelectionne.value && paysPub(p) !== paysSelectionne.value) return false

    return true
  })
})

// Helpers données
function titreCodimoi(post: CodiMoiPostAPI): string {
  if (isQuoteType(post.type)) {
    return getCategoryLabel(post.type as CategoriePost)
  }
  return post.contenu.length > 80 ? post.contenu.slice(0, 80) + '…' : post.contenu
}

function isQuoteType(type: string): boolean {
  return type === 'proverbe_adage' || type === 'citation'
}

function nomAuteur(pub: Publication): string {
  if (pub.source === 'codimoi') {
    const { prenom, nom } = pub.data.auteur
    return `${prenom ?? ''} ${nom}`.trim() || 'Anonyme'
  }
  if (pub.source === 'territoire_partage' || pub.source === 'element_partage' || pub.source === 'profil_partage' || pub.source === 'contribution_partage' || pub.source === 'video_partage') {
    const { prenom, nom } = pub.data.auteur
    return `${prenom ?? ''} ${nom ?? ''}`.trim() || 'Anonyme'
  }
  return `${pub.data.auteur.prenom} ${pub.data.auteur.nom}`
}

function paysPub(pub: Publication): string | null {
  if (pub.source === 'codimoi') return pub.data.pays || null
  if (pub.source === 'territoire_partage') return pub.data.fiche.nom || null
  if (pub.source === 'element_partage') return pub.data.element.territoire_nom || null
  if (pub.source === 'profil_partage') return pub.data.profil.pays || null
  if (pub.source === 'contribution_partage' || pub.source === 'video_partage') return null
  return pub.data.localisation.pays || null
}

function statsVues(pub: Publication): number {
  if (pub.source === 'codimoi') return pub.data.nombre_vues
  if (pub.source === 'territoire_partage' || pub.source === 'element_partage' || pub.source === 'profil_partage' || pub.source === 'contribution_partage' || pub.source === 'video_partage') return 0
  return pub.data.stats.vues
}

function statsLikes(pub: Publication): number {
  if (pub.source === 'codimoi') return pub.data.nombre_likes
  if (pub.source === 'territoire_partage' || pub.source === 'element_partage' || pub.source === 'profil_partage' || pub.source === 'contribution_partage' || pub.source === 'video_partage') return 0
  return pub.data.stats.likes
}

function statsCommentaires(pub: Publication): number {
  if (pub.source === 'codimoi') return pub.data.nombre_commentaires
  if (pub.source === 'territoire_partage' || pub.source === 'element_partage' || pub.source === 'profil_partage' || pub.source === 'contribution_partage' || pub.source === 'video_partage') return 0
  return pub.data.stats.commentaires
}

function formatDate(date: Date): string {
  return new Intl.DateTimeFormat('fr-FR', {
    day: 'numeric',
    month: 'short',
    year: 'numeric',
  }).format(date)
}

function getGraviteClasses(gravite: string): string {
  const classes: Record<string, string> = {
    faible: 'bg-yellow-100 text-yellow-800',
    moyenne: 'bg-orange-100 text-orange-800',
    grave: 'bg-red-100 text-red-800',
    critique: 'bg-red-600 text-white',
  }
  return classes[gravite] || 'bg-gray-100 text-gray-800'
}

const chargerTout = async () => {
  loading.value = true
  erreurChargement.value = null

  const [resCodimoi, resGouv, resPartages, resPartagesElements, resPartagesProfils, resPartagesContrib, resPartagesVideos] = await Promise.allSettled([
    listerPosts({ page: 1, par_page: 30 }),
    getContributions({ page: 1, parPage: 30 }),
    listerPartagesFiches(1, 30),
    listerPartagesElements(1, 30),
    listerPartagesProfils(1, 30),
    listerPartagesContributions(1, 30),
    listerPartagesVideos(1, 30),
  ])

  const items: Publication[] = []

  if (resCodimoi.status === 'fulfilled' && resCodimoi.value?.posts) {
    for (const p of resCodimoi.value.posts) {
      items.push({
        key: `codimoi-${p.id}`,
        source: 'codimoi',
        data: p,
        date: new Date(p.created_at),
        typeFiltre: 'codimoi',
        typeStyle: STYLES_PAR_TYPE.codimoi,
      })
    }
  }
  else if (resCodimoi.status === 'rejected') {
    console.error('Erreur chargement Codimoi:', resCodimoi.reason)
  }

  if (resGouv.status === 'fulfilled' && resGouv.value?.contributions) {
    for (const c of resGouv.value.contributions) {
      items.push({
        key: `gouv-${c.id}`,
        source: 'gouvernance',
        data: c,
        date: c.dateCreation instanceof Date ? c.dateCreation : new Date(c.dateCreation),
        typeFiltre: c.type,
        typeStyle: STYLES_PAR_TYPE[c.type],
      })
    }
  }
  else if (resGouv.status === 'rejected') {
    console.error('Erreur chargement Gouvernance:', resGouv.reason)
  }

  if (resPartages.status === 'fulfilled' && resPartages.value?.partages) {
    for (const partage of resPartages.value.partages) {
      items.push({
        key: `partage-${partage.id}`,
        source: 'territoire_partage',
        data: partage,
        date: new Date(partage.created_at),
        typeFiltre: 'territoire_partage',
        typeStyle: STYLES_PAR_TYPE.territoire_partage,
      })
    }
  }
  else if (resPartages.status === 'rejected') {
    console.error('Erreur chargement Partages:', resPartages.reason)
  }

  if (resPartagesElements.status === 'fulfilled' && resPartagesElements.value?.partages) {
    for (const partage of resPartagesElements.value.partages) {
      items.push({
        key: `partage-element-${partage.id}`,
        source: 'element_partage',
        data: partage,
        date: new Date(partage.created_at),
        typeFiltre: 'element_partage',
        typeStyle: STYLES_PAR_TYPE.element_partage,
      })
    }
  }
  else if (resPartagesElements.status === 'rejected') {
    console.error('Erreur chargement Partages éléments:', resPartagesElements.reason)
  }

  if (resPartagesProfils.status === 'fulfilled' && resPartagesProfils.value?.partages) {
    for (const partage of resPartagesProfils.value.partages) {
      items.push({
        key: `partage-profil-${partage.id}`,
        source: 'profil_partage',
        data: partage,
        date: new Date(partage.created_at),
        typeFiltre: 'profil_partage',
        typeStyle: STYLES_PAR_TYPE.profil_partage,
      })
    }
  }
  else if (resPartagesProfils.status === 'rejected') {
    console.error('Erreur chargement Partages profils:', resPartagesProfils.reason)
  }

  if (resPartagesContrib.status === 'fulfilled' && resPartagesContrib.value?.partages) {
    for (const partage of resPartagesContrib.value.partages) {
      items.push({
        key: `partage-contrib-${partage.id}`,
        source: 'contribution_partage',
        data: partage,
        date: new Date(partage.created_at),
        typeFiltre: partage.contribution.type_contribution,
        typeStyle: STYLES_PAR_TYPE[partage.contribution.type_contribution],
      })
    }
  }
  else if (resPartagesContrib.status === 'rejected') {
    console.error('Erreur chargement Partages contributions:', resPartagesContrib.reason)
  }

  if (resPartagesVideos.status === 'fulfilled' && resPartagesVideos.value?.partages) {
    for (const partage of resPartagesVideos.value.partages) {
      items.push({
        key: `partage-video-${partage.id}`,
        source: 'video_partage',
        data: partage,
        date: new Date(partage.created_at),
        typeFiltre: 'video_partage',
        typeStyle: STYLES_PAR_TYPE.video_partage,
      })
    }
  }
  else if (resPartagesVideos.status === 'rejected') {
    console.error('Erreur chargement Partages vidéos:', resPartagesVideos.reason)
  }

  items.sort((a, b) => b.date.getTime() - a.date.getTime())
  publications.value = items

  if (items.length === 0 && resCodimoi.status === 'rejected' && resGouv.status === 'rejected') {
    erreurChargement.value = 'Impossible de charger les publications'
  }

  loading.value = false
}

const reinitialiser = () => {
  activeFilter.value = 'tous'
  recherche.value = ''
  paysSelectionne.value = ''
}

// Clic sur une publication
const ouvrirPublication = async (pub: Publication) => {
  // Seules les publications Codimoi ouvrent une vue détaillée (modale).
  // Les contributions de gouvernance n'ont pas de page détail.
  if (pub.source === 'codimoi') {
    await ouvrirPostCodimoi(pub.data)
  }
}

// Actions Codimoi (modale)
const ouvrirPostCodimoi = async (post: CodiMoiPostAPI) => {
  selectedPost.value = post
  chargementCommentaires.value = true
  selectedPostCommentaires.value = []

  const resultat = await listerCommentaires(post.id)
  if (resultat) {
    selectedPostCommentaires.value = resultat.commentaires
  }
  chargementCommentaires.value = false
}

const fermerPostCodimoi = () => {
  selectedPost.value = null
  selectedPostCommentaires.value = []
}

const mettreAJourPostCodimoi = (updated: CodiMoiPostAPI) => {
  const index = publications.value.findIndex(
    p => p.source === 'codimoi' && p.data.id === updated.id
  )
  if (index !== -1) {
    const current = publications.value[index] as PublicationCodimoi
    publications.value[index] = { ...current, data: updated }
  }
  if (selectedPost.value?.id === updated.id) {
    selectedPost.value = updated
  }
}

const reagirModalCodimoi = async (type: 'like' | 'dislike') => {
  if (!selectedPost.value) return
  const updated = await reagir(selectedPost.value.id, type)
  if (updated) mettreAJourPostCodimoi(updated)
}

const commenterModalCodimoi = async (contenu: string) => {
  if (!selectedPost.value) return
  const commentaire = await creerCommentaire(selectedPost.value.id, contenu)
  if (commentaire) {
    selectedPostCommentaires.value.unshift(commentaire)
    if (selectedPost.value) {
      selectedPost.value.nombre_commentaires++
    }
    const index = publications.value.findIndex(
      p => p.source === 'codimoi' && p.data.id === selectedPost.value?.id
    )
    if (index !== -1) {
      const current = publications.value[index] as PublicationCodimoi
      current.data.nombre_commentaires++
    }
  }
  else {
    notifier(erreurCodimoi.value || 'Erreur lors de la publication du commentaire')
  }
}

const partagerModalCodimoi = () => {
  if (!selectedPost.value) return
  if (import.meta.client && navigator.clipboard) {
    navigator.clipboard.writeText(`${window.location.origin}/codi-moi/${selectedPost.value.id}`)
    notifier('Lien copié dans le presse-papiers !')
  }
}

onMounted(async () => {
  const { initAuth } = useAuth()
  await initAuth()
  await chargerTout()
})
</script>

<style scoped>
.feed-enter-active,
.feed-leave-active {
  transition: all 0.35s ease;
}
.feed-enter-from,
.feed-leave-to {
  opacity: 0;
  transform: translateY(20px);
}

.toast-enter-active { transition: all 0.3s ease-out; }
.toast-leave-active { transition: all 0.2s ease-in; }
.toast-enter-from,
.toast-leave-to {
  opacity: 0;
  transform: translate(-50%, 20px);
}
</style>
