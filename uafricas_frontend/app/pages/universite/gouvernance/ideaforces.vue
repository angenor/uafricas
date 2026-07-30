<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section (compact, titre ↔ description au survol) -->
    <div class="group relative overflow-hidden bg-linear-to-br from-amber-700 via-orange-500 to-yellow-400">
      <!-- Motif décoratif -->
      <div class="absolute inset-0 opacity-10"
           style="background-image: repeating-linear-gradient(-45deg, transparent, transparent 35px, rgba(255,255,255,0.1) 35px, rgba(255,255,255,0.1) 70px);"></div>
      <div class="absolute -top-20 -right-20 w-96 h-96 rounded-full bg-white/5 animate-pulse"></div>
      <div class="absolute -bottom-32 -left-16 w-80 h-80 rounded-full bg-white/5 animate-pulse" style="animation-delay: 1s;"></div>

      <div class="relative max-w-4xl mx-auto px-4 pt-16 pb-6 text-center select-none">
        <!-- Conteneur fixe : le titre et la description se superposent (crossfade au survol) -->
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12">
          <h1 class="absolute inset-0 flex items-center justify-center text-white text-2xl md:text-4xl font-display font-bold tracking-tight transition-opacity duration-300 group-hover:opacity-0">
            IdeaForces
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Partager des idées et des orientations sur des enjeux de développement
          </p>
        </div>

        <!-- Bouton d'aide : ouvre la présentation d'IdeaForces -->
        <div class="mt-4 flex flex-wrap items-center justify-center gap-3">
          <button
            type="button"
            class="inline-flex items-center gap-2 rounded-full bg-white/15 hover:bg-white/25 text-white font-medium text-sm px-4 py-2.5 backdrop-blur-xs ring-1 ring-white/25 transition-colors"
            aria-label="En savoir plus sur IdeaForces"
            @click="presentationOuverte = true"
          >
            <font-awesome-icon :icon="['fas', 'circle-question']" class="w-4 h-4" />
            C'est quoi IdeaForces&nbsp;?
          </button>
        </div>
      </div>
    </div>

    <!-- Modale de présentation « C'est quoi IdeaForces ? » -->
    <Transition name="modal-fade">
      <div
        v-if="presentationOuverte"
        class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/75 backdrop-blur-xs"
        @click.self="presentationOuverte = false"
      >
        <div
          class="relative w-full max-w-2xl max-h-[90vh] flex flex-col bg-white shadow-2xl rounded-3xl overflow-hidden"
          @click.stop
        >
          <!-- En-tête -->
          <div class="relative shrink-0 bg-linear-to-r from-custom-chocolat to-custom-chocolat/80 px-6 py-6 text-white">
            <button
              type="button"
              class="absolute top-4 right-4 text-white/80 hover:text-white transition-colors"
              aria-label="Fermer"
              @click="presentationOuverte = false"
            >
              <font-awesome-icon :icon="['fas', 'xmark']" class="w-5 h-5" />
            </button>

            <div class="flex items-center gap-3">
              <div class="w-12 h-12 rounded-2xl bg-white/15 flex items-center justify-center shrink-0">
                <font-awesome-icon :icon="['fas', 'lightbulb']" class="w-6 h-6" />
              </div>
              <div>
                <h2 class="text-xl md:text-2xl font-bold leading-tight">IdeaForces</h2>
                <p class="text-white/90 text-sm">Un labo d'idées pour relever les défis du continent</p>
              </div>
            </div>
          </div>

          <!-- Corps défilant -->
          <div class="overflow-y-auto px-6 py-6 space-y-8">
            <!-- L'accroche -->
            <p class="text-gray-700 leading-relaxed">
              L'Afrique regorge d'idées et de talents. <strong class="text-gray-900">IdeaForces</strong>
              est un laboratoire d'intelligence collective : un espace où l'on réunit idées,
              expertises et expériences pour imaginer, ensemble, des
              <strong class="text-gray-900">réponses concrètes</strong> aux défis du continent, des
              Afro-descendants et de la diaspora.
            </p>

            <!-- Ce que vous pouvez faire -->
            <div>
              <h3 class="text-sm font-bold uppercase tracking-wide text-custom-chocolat mb-4">
                Ce que vous pouvez y faire
              </h3>
              <div class="grid sm:grid-cols-2 gap-3">
                <div
                  v-for="item in presentationCartes"
                  :key="item.titre"
                  class="flex gap-3 rounded-2xl border border-gray-100 bg-gray-50/60 p-4"
                >
                  <div class="w-10 h-10 rounded-xl bg-custom-green/10 text-custom-green flex items-center justify-center shrink-0">
                    <font-awesome-icon :icon="['fas', item.icone]" class="w-5 h-5" />
                  </div>
                  <div class="min-w-0">
                    <p class="font-semibold text-gray-900 text-sm">{{ item.titre }}</p>
                    <p class="text-gray-500 text-xs mt-0.5 leading-relaxed">{{ item.texte }}</p>
                  </div>
                </div>
              </div>
            </div>

            <!-- Les outils -->
            <div>
              <h3 class="text-sm font-bold uppercase tracking-wide text-custom-chocolat mb-4">
                Les outils à votre disposition
              </h3>
              <div class="flex flex-wrap gap-2">
                <span
                  v-for="outil in presentationOutils"
                  :key="outil"
                  class="inline-flex items-center gap-1.5 rounded-full bg-custom-chocolat/5 text-custom-chocolat text-xs font-medium px-3 py-1.5"
                >
                  <font-awesome-icon :icon="['fas', 'circle-check']" class="w-3 h-3 text-custom-green" />
                  {{ outil }}
                </span>
              </div>
            </div>

            <!-- Notre objectif -->
            <div class="rounded-2xl bg-custom-green/5 border border-custom-green/15 p-5">
              <h3 class="flex items-center gap-2 text-sm font-bold text-custom-green mb-2">
                <font-awesome-icon :icon="['fas', 'seedling']" class="w-4 h-4" />
                Notre objectif
              </h3>
              <p class="text-gray-700 text-sm leading-relaxed">
                Faire émerger des solutions innovantes pour le développement africain, élargir la
                participation citoyenne aux grandes réflexions et rapprocher experts, institutions
                et citoyens.
              </p>
            </div>
          </div>

          <!-- Pied -->
          <div class="shrink-0 border-t border-gray-100 px-6 py-4 bg-gray-50/50">
            <button
              type="button"
              class="w-full sm:w-auto sm:ml-auto sm:block px-6 py-2.5 rounded-full bg-custom-chocolat text-white font-semibold text-sm hover:bg-custom-chocolat/90 transition-colors"
              @click="presentationOuverte = false"
            >
              J'ai compris
            </button>
          </div>
        </div>
      </div>
    </Transition>

    <!-- Contenu -->
    <div class="max-w-7xl mx-auto px-4 py-8 relative z-10">
      <!-- Barre de navigation -->
      <div class="bg-white rounded-xl shadow-lg p-5 mb-8 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
        <CommonBreadcrumbNav :custom-breadcrumbs="breadcrumbs" />
        <div class="flex flex-wrap items-center gap-2">
          <button
            type="button"
            class="inline-flex items-center gap-2 px-4 py-2 rounded-lg bg-linear-to-r from-amber-500 to-orange-600 text-white hover:from-amber-600 hover:to-orange-700 transition font-medium text-sm shadow-md"
            @click="ouvrirModalPublication"
          >
            <font-awesome-icon :icon="['fas', 'plus']" class="text-xs" />
            Proposer une idée
          </button>
          <NuxtLink to="/universite/gouvernance"
                     class="inline-flex items-center gap-2 px-4 py-2 rounded-lg bg-orange-50 text-orange-700 hover:bg-orange-100 transition font-medium text-sm">
            <font-awesome-icon :icon="['fas', 'arrow-left']" class="text-xs" />
            Retour
          </NuxtLink>
        </div>
      </div>

      <UniversiteGouvernanceIdeaForcesCreateModal
        :open="modalOuvert"
        @close="modalOuvert = false"
        @created="apresPublication"
      />

      <UniversiteGouvernancePartagerContributionModal
        ref="modalPartageRef"
        :is-open="modalPartageOuvert"
        :titre="contribAPartager?.titre ?? ''"
        @close="modalPartageOuvert = false"
        @submit="soumettrePartage"
      />

      <div class="grid grid-cols-1 lg:grid-cols-4 gap-8">
        <!-- Filtres -->
        <div class="lg:col-span-1">
          <div class="bg-white rounded-xl shadow-lg overflow-hidden sticky top-4">
            <div class="bg-linear-to-r from-orange-500 to-amber-500 px-4 py-3">
              <h3 class="text-white font-bold text-sm flex items-center gap-2">
                <font-awesome-icon :icon="['fas', 'filter']" />
                Filtres
              </h3>
            </div>

            <div class="p-4 space-y-4">
              <!-- Recherche -->
              <div class="relative">
                <font-awesome-icon :icon="['fas', 'search']" class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 text-sm" />
                <input v-model="recherche"
                       type="text"
                       placeholder="Rechercher..."
                       class="w-full pl-10 pr-4 py-2 text-sm border border-gray-200 rounded-lg focus:ring-2 focus:ring-orange-500/30 focus:border-orange-500 transition">
              </div>

              <!-- Pays -->
              <div>
                <label class="block text-xs font-semibold text-gray-500 uppercase tracking-wide mb-2">Territoire</label>
                <select v-model="paysSelectionne" class="w-full px-3 py-2 text-sm border border-gray-200 rounded-lg focus:ring-2 focus:ring-orange-500/30 focus:border-orange-500 transition bg-white">
                  <option value="">Tous les territoires</option>
                  <option v-for="pays in paysDisponibles" :key="pays" :value="pays">{{ pays }}</option>
                </select>
              </div>

              <button @click="reinitialiser"
                      class="w-full py-2 text-sm font-medium text-gray-500 border border-gray-200 rounded-lg hover:bg-gray-50 hover:text-gray-700 transition flex items-center justify-center gap-2">
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
                 :id="`contrib-${contribution.id}`"
                 class="group bg-white rounded-xl shadow-md hover:shadow-xl transition-all duration-300 overflow-hidden border scroll-mt-24"
                 :class="pubCible === contribution.id ? 'border-orange-400 ring-2 ring-orange-400 ring-offset-2' : 'border-gray-100 hover:border-gray-200'">
              <!-- Bande orange -->
              <div class="h-1.5 bg-linear-to-r from-amber-400 to-orange-500"></div>

              <div class="p-6">
                <div class="flex items-start gap-4">
                  <!-- Icône -->
                  <div class="shrink-0 w-12 h-12 rounded-xl bg-amber-100 text-amber-600 flex items-center justify-center">
                    <font-awesome-icon :icon="['fas', 'lightbulb']" class="text-lg" />
                  </div>

                  <div class="flex-1 min-w-0">
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
                  <button
                    type="button"
                    title="Partager sur le mur /publications"
                    class="ml-auto flex items-center gap-1.5 hover:text-custom-green transition"
                    @click.stop="ouvrirPartage(contribution)"
                  >
                    <font-awesome-icon :icon="['fas', 'share-nodes']" />
                    <span class="hidden sm:inline">Partager sur le mur</span>
                  </button>
                  <UniversiteGouvernancePartagePublication
                    path="/universite/gouvernance/ideaforces"
                    type-objet="idea_force"
                    :id="contribution.id"
                    :titre="contribution.titre"
                  />
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
import type { ContributionCitoyenne } from '~/types/gouvernance'

useHead({
  title: 'IdeaForces - Gouvernance Citoyenne'
})

const breadcrumbs = [
  { label: 'Université', to: '/universite' },
  { label: 'Gouvernance', to: '/universite/gouvernance' },
  { label: 'IdeaForces', to: undefined }
]

const userStore = useUserStore()
const { getContributions, partagerContribution } = useGouvernance()
const { pubCible, cibler } = usePartagePublication()
const { redirigerVersConnexion } = useAuth()

// Partage vers le mur /publications
const modalPartageOuvert = ref(false)
const contribAPartager = ref<ContributionCitoyenne | null>(null)
const modalPartageRef = ref<{ setLoading: (v: boolean) => void; setError: (m: string) => void; setSuccess: () => void } | null>(null)

function ouvrirPartage(c: ContributionCitoyenne) {
  if (!userStore.isAuthenticated) {
    redirigerVersConnexion()
    return
  }
  contribAPartager.value = c
  modalPartageOuvert.value = true
}

async function soumettrePartage(legende: string) {
  if (!contribAPartager.value) return
  modalPartageRef.value?.setLoading(true)
  try {
    await partagerContribution('ideaforces', contribAPartager.value.id, legende || undefined)
    modalPartageRef.value?.setSuccess()
  } catch (e) {
    modalPartageRef.value?.setError(e instanceof Error ? e.message : 'Erreur lors du partage.')
  }
}

const contributions = ref<ContributionCitoyenne[]>([])
const chargement = ref(false)
const erreurChargement = ref<string | null>(null)
const recherche = ref('')
const paysSelectionne = ref('')
const modalOuvert = ref(false)

// Modale de présentation « C'est quoi IdeaForces ? »
const presentationOuverte = ref(false)
const presentationCartes = [
  {
    icone: 'lightbulb',
    titre: 'Proposer une idée',
    texte: 'Publiez une proposition structurée : objectif, modalités concrètes et bénéficiaires.',
  },
  {
    icone: 'layer-group',
    titre: 'Cibler un enjeu',
    texte: 'Rattachez votre idée à une thématique : gouvernance, éducation, santé, environnement…',
  },
  {
    icone: 'magnifying-glass',
    titre: 'Explorer les propositions',
    texte: 'Parcourez et filtrez les idées de la communauté par thématique et territoire.',
  },
  {
    icone: 'share-nodes',
    titre: 'Diffuser à la communauté',
    texte: 'Partagez les propositions qui vous inspirent sur le mur communautaire.',
  },
]
const presentationOutils = [
  'Propositions structurées',
  'Catégories thématiques',
  'Recherche & filtres',
  'Partage communautaire',
]

function ouvrirModalPublication() {
  if (!userStore.isAuthenticated) {
    redirigerVersConnexion()
    return
  }
  modalOuvert.value = true
}

function apresPublication(_id: string) {
  modalOuvert.value = false
  chargerContributions()
}

const paysDisponibles = computed(() => {
  const pays = new Set(contributions.value.map(c => c.localisation.pays))
  return Array.from(pays).sort()
})

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

const reinitialiser = () => {
  recherche.value = ''
  paysSelectionne.value = ''
}

const formatDate = (date: Date) => {
  return new Intl.DateTimeFormat('fr-FR', { day: 'numeric', month: 'short', year: 'numeric' }).format(new Date(date))
}

async function chargerContributions() {
  chargement.value = true
  erreurChargement.value = null
  try {
    const { contributions: liste } = await getContributions({ type: 'ideaforces', parPage: 50 })
    contributions.value = liste
    cibler(liste.map(c => c.id))
  } catch (err) {
    erreurChargement.value = err instanceof Error ? err.message : 'Erreur lors du chargement'
  } finally {
    chargement.value = false
  }
}

onMounted(chargerContributions)
</script>

<style scoped>
.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: opacity 0.25s ease;
}
.modal-fade-enter-from,
.modal-fade-leave-to {
  opacity: 0;
}
</style>
