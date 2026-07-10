<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section (compact, titre ↔ description au survol) -->
    <div class="group relative overflow-hidden bg-linear-to-br from-blue-900 via-blue-700 to-indigo-600">
      <!-- Motif décoratif -->
      <div class="absolute inset-0 opacity-10"
           style="background-image: repeating-linear-gradient(135deg, transparent, transparent 35px, rgba(255,255,255,0.1) 35px, rgba(255,255,255,0.1) 70px);"></div>
      <div class="absolute -top-20 -right-20 w-96 h-96 rounded-full bg-white/5 animate-pulse"></div>
      <div class="absolute -bottom-32 -left-16 w-80 h-80 rounded-full bg-white/5 animate-pulse" style="animation-delay: 1s;"></div>

      <div class="relative max-w-4xl mx-auto px-4 pt-16 pb-6 text-center select-none">
        <!-- Conteneur fixe : le titre et la description se superposent (crossfade au survol) -->
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12">
          <h1 class="absolute inset-0 flex items-center justify-center text-white text-2xl md:text-4xl font-display font-bold tracking-tight transition-opacity duration-300 group-hover:opacity-0">
            FactCheck
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Vérifier des idées reçues sur l'Afrique
          </p>
        </div>
      </div>
    </div>

    <!-- Contenu -->
    <div class="max-w-7xl mx-auto px-4 py-8 relative z-10">
      <!-- Barre de navigation -->
      <div class="bg-white rounded-xl shadow-lg p-5 mb-8 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
        <CommonBreadcrumbNav :custom-breadcrumbs="breadcrumbs" />
        <div class="flex flex-wrap items-center gap-2">
          <button
            type="button"
            class="inline-flex items-center gap-2 px-4 py-2 rounded-lg bg-linear-to-r from-blue-600 to-indigo-600 text-white hover:from-blue-700 hover:to-indigo-700 transition font-medium text-sm shadow-md"
            @click="ouvrirModalPublication"
          >
            <font-awesome-icon :icon="['fas', 'plus']" class="text-xs" />
            Publier un factcheck
          </button>
          <NuxtLink to="/universite/gouvernance"
                     class="inline-flex items-center gap-2 px-4 py-2 rounded-lg bg-blue-50 text-blue-700 hover:bg-blue-100 transition font-medium text-sm">
            <font-awesome-icon :icon="['fas', 'arrow-left']" class="text-xs" />
            Retour
          </NuxtLink>
        </div>
      </div>

      <UniversiteGouvernanceFactCheckCreateModal
        :open="modalOuvert"
        @close="modalOuvert = false"
        @created="apresPublicationRecharger"
      />

      <UniversiteGouvernancePartagerContributionModal
        ref="modalPartageRef"
        :is-open="modalPartageOuvert"
        :titre="contribAPartager?.titre ?? ''"
        @close="modalPartageOuvert = false"
        @submit="soumettrePartage"
      />

      <CommonImageViewer
        :images="viewerImages"
        :open="viewerOuvert"
        @close="viewerOuvert = false"
      />

      <div class="grid grid-cols-1 lg:grid-cols-4 gap-8">
        <!-- Filtres -->
        <div class="lg:col-span-1">
          <div class="bg-white rounded-xl shadow-lg overflow-hidden sticky top-4">
            <div class="bg-linear-to-r from-blue-600 to-blue-700 px-4 py-3">
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
                       class="w-full pl-10 pr-4 py-2 text-sm border border-gray-200 rounded-lg focus:ring-2 focus:ring-blue-500/30 focus:border-blue-500 transition">
              </div>

              <!-- Pays -->
              <div>
                <label class="block text-xs font-semibold text-gray-500 uppercase tracking-wide mb-2">Territoire</label>
                <select v-model="paysSelectionne" class="w-full px-3 py-2 text-sm border border-gray-200 rounded-lg focus:ring-2 focus:ring-blue-500/30 focus:border-blue-500 transition bg-white">
                  <option value="">Tous les territoires</option>
                  <option v-for="pays in paysDisponibles" :key="pays" :value="pays">{{ pays }}</option>
                </select>
              </div>

              <!-- Statut vérifié -->
              <div>
                <label class="flex items-center gap-3 px-3 py-2 rounded-lg border transition-all cursor-pointer"
                       :class="seulementVerifies
                         ? 'border-green-500 bg-green-50 text-green-700'
                         : 'border-gray-200 hover:border-gray-300 text-gray-600 hover:bg-gray-50'">
                  <input type="checkbox" v-model="seulementVerifies" class="sr-only">
                  <span class="w-5 h-5 rounded-md border-2 flex items-center justify-center shrink-0 transition-all"
                        :class="seulementVerifies ? 'border-green-500 bg-green-500' : 'border-gray-300'">
                    <font-awesome-icon v-if="seulementVerifies" :icon="['fas', 'check']" class="text-white text-xs" />
                  </span>
                  <span class="text-sm font-medium">Vérifiés uniquement</span>
                </label>
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
              <span v-if="recherche || paysSelectionne || seulementVerifies"> (filtré{{ contributionsFiltrees.length > 1 ? 's' : '' }})</span>
            </p>
          </div>

          <!-- Erreur (réaction / chargement / signalement) -->
          <div v-if="erreurReaction"
               class="mb-4 bg-red-50 border-l-4 border-red-500 p-3 rounded-lg text-sm text-red-700 flex items-center gap-2">
            <font-awesome-icon :icon="['fas', 'circle-exclamation']" />
            <span>{{ erreurReaction }}</span>
          </div>

          <!-- Info (signalement) -->
          <div v-if="messageInfo"
               class="mb-4 bg-orange-50 border-l-4 border-orange-500 p-3 rounded-lg text-sm text-orange-700 flex items-center gap-2">
            <font-awesome-icon :icon="['fas', 'flag']" />
            <span>{{ messageInfo }}</span>
          </div>

          <!-- Chargement -->
          <div v-if="chargement && contributions.length === 0"
               class="text-center py-20 bg-white rounded-xl shadow-lg">
            <font-awesome-icon :icon="['fas', 'spinner']" class="text-blue-500 text-2xl animate-spin" />
            <p class="text-gray-500 text-sm mt-3">Chargement des factchecks…</p>
          </div>

          <!-- État vide -->
          <div v-else-if="contributionsFiltrees.length === 0"
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
                 :class="pubCible === contribution.id ? 'border-blue-400 ring-2 ring-blue-400 ring-offset-2' : 'border-gray-100 hover:border-gray-200'">
              <!-- Bande bleue -->
              <div class="h-1.5 bg-linear-to-r from-blue-500 to-indigo-500"></div>

              <!-- Image illustrative (cliquable pour agrandir) -->
              <img
                v-if="contribution.imageUrl"
                :src="contribution.imageUrl"
                alt=""
                class="w-full h-44 object-cover cursor-zoom-in"
                @click.stop="ouvrirViewer([contribution.imageUrl!])"
              >

              <div class="p-6">
                <div class="flex items-start gap-4">
                  <!-- Icône -->
                  <div class="shrink-0 w-12 h-12 rounded-xl flex items-center justify-center"
                       :class="contribution.verified ? 'bg-green-100 text-green-600' : 'bg-blue-100 text-blue-600'">
                    <font-awesome-icon :icon="contribution.verified ? ['fas', 'circle-check'] : ['fas', 'eye']" class="text-lg" />
                  </div>

                  <div class="flex-1 min-w-0">
                    <!-- Titre -->
                    <h3 class="text-lg font-bold text-gray-900 mb-2 group-hover:text-blue-700 transition-colors line-clamp-2">
                      {{ contribution.titre }}
                    </h3>

                    <!-- Description -->
                    <p class="text-gray-500 text-sm leading-relaxed mb-4 line-clamp-2">
                      {{ contribution.description }}
                    </p>

                    <!-- Type de publication + preuve (fait vécu) -->
                    <div v-if="contribution.typePublication" class="flex flex-wrap items-center gap-2 mb-4">
                      <span
                        class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-semibold"
                        :class="typeBadge(contribution.typePublication).class"
                      >
                        <font-awesome-icon :icon="typeBadge(contribution.typePublication).icon" class="text-[10px]" />
                        {{ typeBadge(contribution.typePublication).label }}
                      </span>

                      <template v-if="contribution.typePublication === 'fait_vecu'">
                        <!-- Preuve image : ouvrable dans la visionneuse -->
                        <button
                          v-if="contribution.preuveUrl && contribution.preuveType === 'image'"
                          type="button"
                          class="inline-flex items-center gap-1.5 px-2.5 py-1 bg-green-50 text-green-700 border border-green-200 rounded-full text-xs font-medium hover:bg-green-100 transition"
                          @click.stop="ouvrirViewer([contribution.preuveUrl!])"
                        >
                          <font-awesome-icon :icon="['fas', 'image']" class="text-[10px]" />
                          Voir la preuve (photo)
                        </button>
                        <!-- Preuve PDF : lien -->
                        <a
                          v-else-if="contribution.preuveUrl"
                          :href="contribution.preuveUrl"
                          target="_blank"
                          rel="noopener noreferrer"
                          class="inline-flex items-center gap-1.5 px-2.5 py-1 bg-green-50 text-green-700 border border-green-200 rounded-full text-xs font-medium hover:bg-green-100 transition"
                          @click.stop
                        >
                          <font-awesome-icon :icon="['fas', 'file-pdf']" class="text-[10px]" />
                          Preuve (PDF)
                        </a>
                        <span
                          v-else
                          class="inline-flex items-center gap-1.5 px-2.5 py-1 bg-gray-100 text-gray-500 border border-gray-200 rounded-full text-xs font-medium"
                        >
                          <font-awesome-icon :icon="['fas', 'ban']" class="text-[10px]" />
                          Pas de preuve
                        </span>
                      </template>
                    </div>

                    <!-- FactCheck préjugé vs réalité -->
                    <div v-if="contribution.factcheck" class="grid grid-cols-1 md:grid-cols-2 gap-3 mb-4">
                      <div class="p-3 bg-red-50 rounded-lg border-l-4 border-red-400 flex flex-col">
                        <p class="text-xs font-bold text-red-600 uppercase tracking-wide mb-1">
                          <font-awesome-icon :icon="['fas', 'times']" class="mr-1" />Préjugé
                        </p>
                        <p class="text-red-800 text-sm line-clamp-2 flex-1">{{ contribution.factcheck.prejuge.titre }}</p>
                        <button
                          type="button"
                          class="mt-2 self-start inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-semibold transition"
                          :class="contribution.aLikePrejuge
                            ? 'bg-red-500 text-white shadow-sm'
                            : 'bg-white text-red-600 border border-red-200 hover:bg-red-100'"
                          @click.stop="reagirVolet(contribution, 'prejuge')"
                        >
                          <font-awesome-icon :icon="['fas', 'heart']" class="text-[11px]" />
                          {{ contribution.factcheck.prejuge.likes || 0 }}
                        </button>
                      </div>
                      <div class="p-3 bg-green-50 rounded-lg border-l-4 border-green-400 flex flex-col">
                        <p class="text-xs font-bold text-green-600 uppercase tracking-wide mb-1">
                          <font-awesome-icon :icon="['fas', 'check']" class="mr-1" />Réalité
                        </p>
                        <p class="text-green-800 text-sm line-clamp-2 flex-1">{{ contribution.factcheck.contrePrejuge.titre }}</p>
                        <button
                          type="button"
                          class="mt-2 self-start inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-semibold transition"
                          :class="contribution.aLikeRealite
                            ? 'bg-green-500 text-white shadow-sm'
                            : 'bg-white text-green-600 border border-green-200 hover:bg-green-100'"
                          @click.stop="reagirVolet(contribution, 'realite')"
                        >
                          <font-awesome-icon :icon="['fas', 'heart']" class="text-[11px]" />
                          {{ contribution.factcheck.contrePrejuge.likes || 0 }}
                        </button>
                      </div>
                    </div>

                    <!-- Sources (appuient la réalité) -->
                    <div v-if="contribution.sources?.length" class="flex flex-wrap items-center gap-2 mb-4">
                      <span class="flex items-center gap-1 text-xs font-bold text-green-600 uppercase tracking-wide">
                        <font-awesome-icon :icon="['fas', 'link']" />
                        Sources
                      </span>
                      <a
                        v-for="(source, index) in contribution.sources"
                        :key="index"
                        :href="source.url"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="inline-flex items-center gap-1.5 px-2.5 py-1 bg-green-50 text-green-700 border border-green-200 rounded-full text-xs font-medium hover:bg-green-100 hover:border-green-300 transition"
                        @click.stop
                      >
                        <font-awesome-icon :icon="['fas', 'circle-check']" class="text-[10px]" />
                        {{ source.titre }}
                      </a>
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

                <!-- Réactions globales (emojis) + signalement -->
                <div class="flex flex-wrap items-center gap-2 mt-5 pt-4 border-t border-gray-100 text-xs text-gray-400">
                  <button
                    v-for="r in reactionsConfig"
                    :key="r.type"
                    type="button"
                    :title="r.label"
                    class="flex items-center gap-1.5 px-2.5 py-1 rounded-full transition"
                    :class="contribution.reactions?.maReaction === r.type
                      ? `${r.actifClass} bg-gray-100 font-semibold`
                      : 'hover:bg-gray-100'"
                    @click.stop="reagirGlobal(contribution, r.type)"
                  >
                    <font-awesome-icon :icon="r.icon" />
                    {{ nombreReaction(contribution, r.type) }}
                  </button>

                  <!-- Signaler -->
                  <button
                    type="button"
                    :title="contribution.aSignale ? 'Vous avez signalé cette publication' : 'Signaler cette publication'"
                    :disabled="contribution.aSignale"
                    class="ml-auto flex items-center gap-1.5 px-2.5 py-1 rounded-full transition"
                    :class="contribution.aSignale
                      ? 'text-orange-600 font-semibold cursor-default'
                      : 'hover:bg-orange-50 hover:text-orange-600'"
                    @click.stop="signalerContribution(contribution)"
                  >
                    <font-awesome-icon :icon="['fas', 'flag']" />
                    {{ contribution.aSignale ? 'Signalé' : 'Signaler' }}
                  </button>

                  <button
                    type="button"
                    title="Partager sur le mur /publications"
                    class="flex items-center gap-1.5 px-2.5 py-1 rounded-full hover:bg-green-50 hover:text-custom-green transition"
                    @click.stop="ouvrirPartage(contribution)"
                  >
                    <font-awesome-icon :icon="['fas', 'share-nodes']" />
                    <span class="hidden sm:inline">Partager sur le mur</span>
                  </button>

                  <UniversiteGouvernancePartagePublication
                    class="px-2.5 py-1"
                    path="/universite/gouvernance/factcheck"
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
import type { ContributionCitoyenne, TypePublicationFactcheck, TypeReactionGlobale } from '~/types/gouvernance'

/** Badge d'affichage pour le type de publication */
function typeBadge(type: TypePublicationFactcheck | undefined): { label: string; icon: [string, string]; class: string } {
  switch (type) {
    case 'adage_legende':
      return { label: 'Adage / Légende', icon: ['fas', 'book-open'], class: 'bg-purple-50 text-purple-700 border border-purple-200' }
    case 'fait_vecu':
      return { label: 'Fait vécu ou observé', icon: ['fas', 'location-dot'], class: 'bg-amber-50 text-amber-700 border border-amber-200' }
    case 'on_dit':
    default:
      return { label: 'On dit / entendu', icon: ['fas', 'comments'], class: 'bg-blue-50 text-blue-700 border border-blue-200' }
  }
}

useHead({
  title: 'FactCheck - Gouvernance Citoyenne'
})

const { getContributions, reagir, signaler, partagerContribution } = useGouvernance()
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
    await partagerContribution('factcheck', contribAPartager.value.id, legende || undefined)
    modalPartageRef.value?.setSuccess()
  } catch (e) {
    modalPartageRef.value?.setError(e instanceof Error ? e.message : 'Erreur lors du partage.')
  }
}

const breadcrumbs = [
  { label: 'Université', to: '/universite' },
  { label: 'Gouvernance', to: '/universite/gouvernance' },
  { label: 'FactCheck', to: undefined }
]

const userStore = useUserStore()

// Visionneuse d'images
const viewerOuvert = ref(false)
const viewerImages = ref<string[]>([])
function ouvrirViewer(images: string[]) {
  viewerImages.value = images
  viewerOuvert.value = true
}

const contributions = ref<ContributionCitoyenne[]>([])
const recherche = ref('')
const paysSelectionne = ref('')
const seulementVerifies = ref(false)
const modalOuvert = ref(false)
const chargement = ref(false)
const erreurReaction = ref<string | null>(null)

function ouvrirModalPublication() {
  if (!userStore.isAuthenticated) {
    redirigerVersConnexion()
    return
  }
  modalOuvert.value = true
}

function apresPublication(_id: string) {
  modalOuvert.value = false
}

const paysDisponibles = computed(() => {
  const pays = new Set(contributions.value.map(c => c.localisation.pays))
  return Array.from(pays).sort()
})

const contributionsFiltrees = computed(() => {
  return contributions.value.filter(c => {
    if (recherche.value) {
      const search = recherche.value.toLowerCase()
      if (!c.titre.toLowerCase().includes(search) &&
          !c.description.toLowerCase().includes(search)) {
        return false
      }
    }
    if (paysSelectionne.value && c.localisation.pays !== paysSelectionne.value) {
      return false
    }
    if (seulementVerifies.value && !c.verified) {
      return false
    }
    return true
  })
})

// Réactions globales (emojis) du post
const reactionsConfig: { type: TypeReactionGlobale; icon: [string, string]; label: string; actifClass: string }[] = [
  { type: 'coeur', icon: ['fas', 'heart'], label: 'Cœur', actifClass: 'text-red-500' },
  { type: 'pouce', icon: ['fas', 'thumbs-up'], label: "J'aime", actifClass: 'text-blue-500' },
  { type: 'rire', icon: ['fas', 'face-laugh-squint'], label: 'Ça me fait rire', actifClass: 'text-amber-500' },
  { type: 'jaime_pas', icon: ['fas', 'thumbs-down'], label: "Je n'aime pas", actifClass: 'text-gray-700' },
]

function nombreReaction(c: ContributionCitoyenne, type: TypeReactionGlobale): number {
  const r = c.reactions
  if (!r) return 0
  switch (type) {
    case 'coeur': return r.coeur
    case 'pouce': return r.pouce
    case 'rire': return r.rire
    case 'jaime_pas': return r.jaimePas
  }
}

/** Remplace immutablement la contribution dans la liste avec le nouvel état de réaction */
function appliquerEtat(id: string, etat: Awaited<ReturnType<typeof reagir>>) {
  const index = contributions.value.findIndex(c => c.id === id)
  if (index === -1) return
  const actuelle = contributions.value[index]!
  const maj: ContributionCitoyenne = {
    ...actuelle,
    reactions: etat.reactions,
    aLikePrejuge: etat.aLikePrejuge,
    aLikeRealite: etat.aLikeRealite,
    factcheck: actuelle.factcheck
      ? {
          prejuge: { ...actuelle.factcheck.prejuge, likes: etat.prejugeLikes },
          contrePrejuge: { ...actuelle.factcheck.contrePrejuge, likes: etat.realiteLikes },
        }
      : undefined,
  }
  const copie = [...contributions.value]
  copie[index] = maj
  contributions.value = copie
}

async function reagirGlobal(c: ContributionCitoyenne, type: TypeReactionGlobale) {
  if (!userStore.isAuthenticated) {
    redirigerVersConnexion()
    return
  }
  erreurReaction.value = null
  try {
    appliquerEtat(c.id, await reagir(c.id, 'general', type))
  } catch (err) {
    erreurReaction.value = err instanceof Error ? err.message : 'Erreur lors de la réaction'
  }
}

async function reagirVolet(c: ContributionCitoyenne, cible: 'prejuge' | 'realite') {
  if (!userStore.isAuthenticated) {
    redirigerVersConnexion()
    return
  }
  erreurReaction.value = null
  try {
    appliquerEtat(c.id, await reagir(c.id, cible))
  } catch (err) {
    erreurReaction.value = err instanceof Error ? err.message : 'Erreur lors de la réaction'
  }
}

const messageInfo = ref<string | null>(null)

async function signalerContribution(c: ContributionCitoyenne) {
  if (!userStore.isAuthenticated) {
    redirigerVersConnexion()
    return
  }
  if (c.aSignale) return
  if (!confirm('Signaler cette publication comme inappropriée ou trompeuse ?')) return
  erreurReaction.value = null
  try {
    const etat = await signaler(c.id)
    if (etat.suspendu) {
      // Suspendue : elle quitte la liste publique
      contributions.value = contributions.value.filter(x => x.id !== c.id)
      messageInfo.value = 'Publication suspendue : elle a dépassé le seuil de signalements.'
    } else {
      // Marquer comme signalée (immutable)
      const index = contributions.value.findIndex(x => x.id === c.id)
      if (index !== -1) {
        const copie = [...contributions.value]
        copie[index] = { ...contributions.value[index]!, aSignale: true }
        contributions.value = copie
      }
      messageInfo.value = etat.dejaSignale ? 'Vous aviez déjà signalé cette publication.' : 'Merci, votre signalement a été pris en compte.'
    }
    setTimeout(() => { messageInfo.value = null }, 4000)
  } catch (err) {
    erreurReaction.value = err instanceof Error ? err.message : 'Erreur lors du signalement'
  }
}

const reinitialiser = () => {
  recherche.value = ''
  paysSelectionne.value = ''
  seulementVerifies.value = false
}

const formatDate = (date: Date) => {
  return new Intl.DateTimeFormat('fr-FR', {
    day: 'numeric',
    month: 'short',
    year: 'numeric'
  }).format(new Date(date))
}

// Garde de séquence : le chargement anonyme (onMounted) et le rechargement
// authentifié (après restauration du token) peuvent se courser. Seule la
// requête la plus récente applique son résultat, pour ne pas écraser l'état
// personnalisé (ma_reaction / a_like_*) par une réponse anonyme tardive.
let chargementSeq = 0
async function chargerContributions() {
  const seq = ++chargementSeq
  chargement.value = true
  try {
    const { contributions: liste } = await getContributions({ type: 'factcheck', parPage: 50 })
    if (seq !== chargementSeq) return
    contributions.value = liste
    cibler(liste.map(c => c.id))
  } catch (err) {
    if (seq === chargementSeq) {
      erreurReaction.value = err instanceof Error ? err.message : 'Erreur lors du chargement'
    }
  } finally {
    if (seq === chargementSeq) chargement.value = false
  }
}

function apresPublicationRecharger(id: string) {
  apresPublication(id)
  chargerContributions()
}

onMounted(chargerContributions)

// Le token d'accès est restauré en mémoire de façon asynchrone après le montage.
// Dès qu'il devient disponible, on recharge pour récupérer l'état de réaction
// personnalisé (ma_reaction / a_like_*) sans lequel les surbrillances manquent.
watch(() => userStore.accessToken, (token, ancien) => {
  if (token && !ancien) chargerContributions()
})
</script>
