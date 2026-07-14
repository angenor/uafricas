<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section (compact, titre ↔ description au survol) -->
    <div
      class="group relative overflow-hidden transition-colors duration-500"
      :class="vueActive === 'bonne'
        ? 'bg-linear-to-br from-emerald-900 via-emerald-700 to-green-600'
        : 'bg-linear-to-br from-red-900 via-red-700 to-orange-600'"
    >
      <!-- Motif géométrique animé -->
      <div class="absolute inset-0 opacity-10">
        <div class="absolute top-0 left-0 w-full h-full"
             style="background-image: repeating-linear-gradient(45deg, transparent, transparent 35px, rgba(255,255,255,0.1) 35px, rgba(255,255,255,0.1) 70px);">
        </div>
      </div>
      <!-- Cercles décoratifs -->
      <div class="absolute -top-20 -right-20 w-96 h-96 rounded-full bg-white/5 animate-pulse"></div>
      <div class="absolute -bottom-32 -left-16 w-80 h-80 rounded-full bg-white/5 animate-pulse" style="animation-delay: 1s;"></div>

      <div class="relative max-w-4xl mx-auto px-4 pt-16 pb-6 text-center select-none">
        <!-- Conteneur fixe : le titre et la description se superposent (crossfade au survol) -->
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12">
          <h1 class="absolute inset-0 flex items-center justify-center text-white text-2xl md:text-4xl font-display font-bold tracking-tight transition-opacity duration-300 group-hover:opacity-0">
            {{ heroTitre }}
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            {{ heroDescription }}
          </p>
        </div>

        <!-- Bouton d'aide : ouvre la présentation de BadGoodhabits -->
        <div class="mt-4 flex flex-wrap items-center justify-center gap-3">
          <button
            type="button"
            class="inline-flex items-center gap-2 rounded-full bg-white/15 hover:bg-white/25 text-white font-medium text-sm px-4 py-2.5 backdrop-blur-xs ring-1 ring-white/25 transition-colors"
            aria-label="En savoir plus sur BadGoodhabits"
            @click="presentationOuverte = true"
          >
            <font-awesome-icon :icon="['fas', 'circle-question']" class="w-4 h-4" />
            C'est quoi BadGoodhabits&nbsp;?
          </button>
        </div>
      </div>
    </div>

    <!-- Modale de présentation « C'est quoi BadGoodhabits ? » -->
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
                <font-awesome-icon :icon="['fas', 'scale-balanced']" class="w-6 h-6" />
              </div>
              <div>
                <h2 class="text-xl md:text-2xl font-bold leading-tight">BadGoodhabits</h2>
                <p class="text-white/90 text-sm">Valoriser le meilleur, dénoncer ce qui nuit</p>
              </div>
            </div>
          </div>

          <!-- Corps défilant -->
          <div class="overflow-y-auto px-6 py-6 space-y-8">
            <!-- L'accroche -->
            <p class="text-gray-700 leading-relaxed">
              Nos gestes du quotidien façonnent l'Afrique de demain.
              <strong class="text-gray-900">BadGoodhabits</strong> est un espace citoyen pour
              <strong class="text-gray-900">mettre en lumière les bonnes pratiques</strong> et
              <strong class="text-gray-900">signaler ce qui nuit</strong> à l'intérêt commun — sur
              la base de faits vérifiables — afin de cultiver ensemble responsabilité, transparence
              et citoyenneté active.
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
                Renforcer le civisme et le sens des responsabilités, promouvoir la transparence et
                la redevabilité, et diffuser des modèles positifs qui inspirent le plus grand nombre.
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
            class="inline-flex items-center gap-2 px-4 py-2 rounded-lg text-white font-medium text-sm shadow-md transition"
            :class="vueActive === 'bonne'
              ? 'bg-linear-to-r from-emerald-600 to-green-600 hover:from-emerald-700 hover:to-green-700'
              : 'bg-linear-to-r from-red-600 to-orange-600 hover:from-red-700 hover:to-orange-700'"
            @click="ouvrirModalPublication(vueActive === 'bonne' ? 'bonne' : 'mauvaise')"
          >
            <font-awesome-icon :icon="['fas', 'plus']" class="text-xs" />
            {{ vueActive === 'bonne' ? 'Ajouter un Goodhabits' : 'Signaler un Badhabits' }}
          </button>
          <NuxtLink to="/universite/gouvernance"
                     class="inline-flex items-center gap-2 px-4 py-2 rounded-lg bg-gray-50 text-gray-700 hover:bg-gray-100 transition font-medium text-sm">
            <font-awesome-icon :icon="['fas', 'arrow-left']" class="text-xs" />
            Retour
          </NuxtLink>
        </div>
      </div>

      <!-- Onglets -->
      <div class="bg-white rounded-xl shadow-sm p-1.5 mb-6 inline-flex flex-wrap gap-1">
        <button
          v-for="onglet in onglets"
          :key="onglet.value"
          type="button"
          class="px-4 py-2 rounded-lg text-sm font-semibold transition-all flex items-center gap-2"
          :class="vueActive === onglet.value
            ? onglet.activeClass
            : 'text-gray-500 hover:text-gray-700 hover:bg-gray-50'"
          @click="vueActive = onglet.value"
        >
          <font-awesome-icon :icon="onglet.icon" class="text-xs" />
          {{ onglet.label }}
          <span class="ml-1 text-xs px-2 py-0.5 rounded-full"
                :class="vueActive === onglet.value ? 'bg-white/30' : 'bg-gray-100 text-gray-500'">
            {{ onglet.compte }}
          </span>
        </button>
      </div>

      <UniversiteGouvernanceBadHabitsCreateModal
        :open="modalOuvert"
        :type-pratique-initial="typePratiqueInitial"
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

      <CommonImageViewer
        :images="viewerImages"
        :open="viewerOuvert"
        :index="viewerIndex"
        @close="viewerOuvert = false"
      />

      <div class="grid grid-cols-1 lg:grid-cols-4 gap-8">
        <!-- Filtres -->
        <div class="lg:col-span-1">
          <div class="bg-white rounded-xl shadow-lg overflow-hidden sticky top-4">
            <div class="px-4 py-3 transition-colors duration-300"
                 :class="vueActive === 'bonne'
                   ? 'bg-linear-to-r from-emerald-600 to-emerald-700'
                   : 'bg-linear-to-r from-red-600 to-red-700'">
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
                       class="w-full pl-10 pr-4 py-2 text-sm border border-gray-200 rounded-lg transition"
                       :class="vueActive === 'bonne'
                         ? 'focus:ring-2 focus:ring-emerald-500/30 focus:border-emerald-500'
                         : 'focus:ring-2 focus:ring-red-500/30 focus:border-red-500'">
              </div>

              <!-- Pays -->
              <div>
                <label class="block text-xs font-semibold text-gray-500 uppercase tracking-wide mb-2">Territoire</label>
                <select v-model="paysSelectionne"
                        class="w-full px-3 py-2 text-sm border border-gray-200 rounded-lg transition bg-white"
                        :class="vueActive === 'bonne'
                          ? 'focus:ring-2 focus:ring-emerald-500/30 focus:border-emerald-500'
                          : 'focus:ring-2 focus:ring-red-500/30 focus:border-red-500'">
                  <option value="">Tous les territoires</option>
                  <option v-for="pays in paysDisponibles" :key="pays" :value="pays">{{ pays }}</option>
                </select>
              </div>

              <!-- Gravité (mauvaises pratiques) -->
              <div v-if="vueActive !== 'bonne'">
                <label class="block text-xs font-semibold text-gray-500 uppercase tracking-wide mb-2">Gravité</label>
                <div class="space-y-2">
                  <button v-for="g in niveauxGravite" :key="g.valeur"
                          @click="graviteSelectionnee = graviteSelectionnee === g.valeur ? '' : g.valeur"
                          class="w-full flex items-center gap-3 px-3 py-1.5 rounded-lg border transition-all text-sm"
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

              <!-- Impact (bonnes pratiques) -->
              <div v-if="vueActive !== 'mauvaise'">
                <label class="block text-xs font-semibold text-gray-500 uppercase tracking-wide mb-2">Impact</label>
                <div class="space-y-2">
                  <button v-for="i in niveauxImpact" :key="i.valeur"
                          @click="impactSelectionne = impactSelectionne === i.valeur ? '' : i.valeur"
                          class="w-full flex items-center gap-3 px-3 py-1.5 rounded-lg border transition-all text-sm"
                          :class="impactSelectionne === i.valeur
                            ? 'border-emerald-500 bg-emerald-50 text-emerald-700 shadow-sm'
                            : 'border-gray-200 hover:border-gray-300 text-gray-600 hover:bg-gray-50'">
                    <span class="w-3 h-3 rounded-full" :class="i.couleurPastille"></span>
                    <span class="font-medium">{{ i.label }}</span>
                    <span class="ml-auto text-xs px-2 py-0.5 rounded-full bg-gray-100 text-gray-500">
                      {{ compterParImpact(i.valeur) }}
                    </span>
                  </button>
                </div>
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
              <span v-if="recherche || paysSelectionne || graviteSelectionnee || impactSelectionne"> (filtré{{ contributionsFiltrees.length > 1 ? 's' : '' }})</span>
            </p>
          </div>

          <!-- Chargement -->
          <div v-if="chargement" class="text-center py-20 bg-white rounded-xl shadow-lg">
            <div class="w-16 h-16 mx-auto mb-4 rounded-full border-4 border-gray-200 border-t-gray-600 animate-spin"></div>
            <p class="text-gray-500 text-sm">Chargement des contributions…</p>
          </div>

          <!-- Erreur réseau -->
          <div v-else-if="erreurChargement"
               class="text-center py-20 bg-white rounded-xl shadow-lg border border-red-100">
            <div class="w-20 h-20 mx-auto mb-6 rounded-full bg-red-50 flex items-center justify-center">
              <font-awesome-icon :icon="['fas', 'triangle-exclamation']" class="text-red-400 text-2xl" />
            </div>
            <p class="text-gray-900 font-semibold text-lg mb-2">Impossible de charger les contributions</p>
            <p class="text-gray-500 text-sm mb-4">{{ erreurChargement }}</p>
            <button @click="chargerContributions"
                    class="px-5 py-2 bg-gray-900 text-white rounded-lg hover:bg-gray-800 transition text-sm font-medium">
              Réessayer
            </button>
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
            <div v-for="(contribution, index) in contributionsFiltrees" :key="contribution.id"
                 :id="`contrib-${contribution.id}`"
                 class="group bg-white rounded-xl shadow-md hover:shadow-xl transition-all duration-300 overflow-hidden border scroll-mt-24"
                 :class="pubCible === contribution.id ? 'border-amber-400 ring-2 ring-amber-400 ring-offset-2' : 'border-gray-100 hover:border-gray-200'"
                 :style="{ animationDelay: `${index * 80}ms` }">
              <!-- Bande de gravité / impact -->
              <div class="h-1.5" :class="getBandeClass(contribution)"></div>

              <div class="p-6">
                <div class="flex items-start gap-4">
                  <!-- Icône gravité / impact -->
                  <div class="shrink-0 w-12 h-12 rounded-xl flex items-center justify-center"
                       :class="getIconeWrapperClass(contribution)">
                    <font-awesome-icon :icon="getIconeContribution(contribution)" class="text-lg" />
                  </div>

                  <div class="flex-1 min-w-0">
                    <!-- Badges -->
                    <div class="flex flex-wrap items-center gap-2 mb-2">
                      <span v-if="contribution.typePratique === 'bonne'"
                            class="px-2.5 py-0.5 rounded-full text-xs font-bold uppercase tracking-wide bg-emerald-100 text-emerald-700">
                        <font-awesome-icon :icon="['fas', 'thumbs-up']" class="text-xs mr-1" />
                        Goodhabits
                      </span>
                      <span v-if="contribution.typePratique !== 'bonne' && contribution.problematique?.gravite"
                            class="px-2.5 py-0.5 rounded-full text-xs font-bold uppercase tracking-wide"
                            :class="getGraviteClass(contribution.problematique.gravite)">
                        {{ contribution.problematique.gravite }}
                      </span>
                      <span v-if="contribution.typePratique === 'bonne' && contribution.bonnePratique?.impact"
                            class="px-2.5 py-0.5 rounded-full text-xs font-bold uppercase tracking-wide"
                            :class="getImpactClass(contribution.bonnePratique.impact)">
                        Impact {{ contribution.bonnePratique.impact }}
                      </span>
                      <span v-if="getCategorieAffichee(contribution)"
                            class="px-2.5 py-0.5 bg-gray-100 text-gray-600 rounded-full text-xs font-medium">
                        {{ getCategorieAffichee(contribution) }}
                      </span>
                    </div>

                    <!-- Titre -->
                    <h3 class="text-lg font-bold text-gray-900 mb-2 transition-colors line-clamp-2"
                        :class="contribution.typePratique === 'bonne'
                          ? 'group-hover:text-emerald-700'
                          : 'group-hover:text-red-700'">
                      {{ contribution.titre }}
                    </h3>

                    <!-- Description -->
                    <p class="text-gray-500 text-sm leading-relaxed mb-4 line-clamp-2">
                      {{ contribution.description }}
                    </p>

                    <!-- Preuves (images + PDF) -->
                    <div v-if="contribution.images?.length" class="flex flex-wrap gap-2 mb-4">
                      <button
                        v-for="(img, idx) in imagesPreuves(contribution)"
                        :key="img"
                        type="button"
                        class="relative w-20 h-20 rounded-lg overflow-hidden border border-gray-200 cursor-zoom-in group/img"
                        @click.stop="ouvrirViewer(imagesPreuves(contribution), idx)"
                      >
                        <img :src="img" alt="Preuve" class="w-full h-full object-cover transition group-hover/img:scale-105">
                      </button>
                      <a
                        v-for="pdf in pdfsPreuves(contribution)"
                        :key="pdf"
                        :href="pdf"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="relative w-20 h-20 rounded-lg border border-gray-200 bg-gray-50 flex flex-col items-center justify-center gap-1 text-red-500 hover:text-red-600 hover:bg-red-50 transition"
                        @click.stop
                      >
                        <font-awesome-icon :icon="['fas', 'file-pdf']" class="text-2xl" />
                        <span class="text-[10px] font-semibold">PDF</span>
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
                        {{ contribution.localisation.ville }}, {{ contribution.localisation.pays }}
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
                  <span class="flex items-center gap-1.5 transition"
                        :class="contribution.typePratique === 'bonne'
                          ? 'hover:text-emerald-500'
                          : 'hover:text-red-500'">
                    <font-awesome-icon :icon="['fas', contribution.typePratique === 'bonne' ? 'hands-clapping' : 'hand-fist']" />
                    {{ contribution.stats.soutiens || 0 }}
                    {{ contribution.typePratique === 'bonne' ? 'félicitations' : 'soutiens' }}
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
                    path="/universite/gouvernance/bad-good-habits"
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
import type { TypePratique } from '~/composables/useGouvernance'

const { getContributions, partagerContribution } = useGouvernance()
const { pubCible, cibler } = usePartagePublication()
const { redirigerVersConnexion } = useAuth()
const chargement = ref(false)
const erreurChargement = ref<string | null>(null)

useHead({
  title: 'BadGoodhabits - Gouvernance Citoyenne'
})

const breadcrumbs = [
  { label: 'Université', to: '/universite' },
  { label: 'Gouvernance', to: '/universite/gouvernance' },
  { label: 'BadGoodhabits', to: undefined }
]

const userStore = useUserStore()

// Visionneuse d'images (preuves photos)
const viewerOuvert = ref(false)
const viewerImages = ref<string[]>([])
const viewerIndex = ref(0)
function ouvrirViewer(images: string[], index = 0) {
  viewerImages.value = images
  viewerIndex.value = index
  viewerOuvert.value = true
}

// Séparation des preuves : images (visionneuse) vs documents PDF (lien)
const estPdf = (url: string) => /\.pdf(\?|#|$)/i.test(url)
const imagesPreuves = (c: ContributionCitoyenne) => (c.images ?? []).filter(u => !estPdf(u))
const pdfsPreuves = (c: ContributionCitoyenne) => (c.images ?? []).filter(estPdf)

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
    await partagerContribution('badhabits', contribAPartager.value.id, legende || undefined)
    modalPartageRef.value?.setSuccess()
  } catch (e) {
    modalPartageRef.value?.setError(e instanceof Error ? e.message : 'Erreur lors du partage.')
  }
}

type VueActive = 'toutes' | 'mauvaise' | 'bonne'

// Hero adaptatif selon la vue active
const heroTitre = computed(() => {
  if (vueActive.value === 'bonne') return 'Goodhabits'
  if (vueActive.value === 'mauvaise') return 'Badhabits'
  return 'BadGoodhabits'
})
const heroDescription = computed(() => {
  if (vueActive.value === 'bonne') return 'Féliciter les bonnes actions et pratiques exemplaires.'
  if (vueActive.value === 'mauvaise') return 'Dénoncer les mauvaises pratiques de gouvernance.'
  return 'Dénoncer les mauvaises pratiques et féliciter les bonnes actions.'
})

const contributions = ref<ContributionCitoyenne[]>([])
const vueActive = ref<VueActive>('toutes')
const recherche = ref('')
const paysSelectionne = ref('')
const graviteSelectionnee = ref('')
const impactSelectionne = ref('')
const modalOuvert = ref(false)
const typePratiqueInitial = ref<TypePratique>('mauvaise')

// Modale de présentation « C'est quoi BadGoodhabits ? »
const presentationOuverte = ref(false)
const presentationCartes = [
  {
    icone: 'flag',
    titre: 'Signaler ce qui nuit',
    texte: 'Documentez les mauvaises pratiques avec preuves (photos, documents PDF).',
  },
  {
    icone: 'thumbs-up',
    titre: 'Valoriser le meilleur',
    texte: 'Mettez en avant des initiatives exemplaires et inspirantes.',
  },
  {
    icone: 'share-nodes',
    titre: 'Partager largement',
    texte: 'Diffusez les contributions sur le mur communautaire pour les faire connaître.',
  },
  {
    icone: 'book-open',
    titre: 'Capitaliser les initiatives',
    texte: 'Constituez un répertoire de bonnes pratiques reproductibles.',
  },
]
const presentationOutils = [
  'Signalements avec preuves',
  'Photos & documents (PDF)',
  'Gravité & impact',
  'Partage communautaire',
]

function ouvrirModalPublication(type: TypePratique = 'mauvaise') {
  if (!userStore.isAuthenticated) {
    redirigerVersConnexion()
    return
  }
  typePratiqueInitial.value = type
  modalOuvert.value = true
}

function apresPublication(_id: string) {
  modalOuvert.value = false
}

const niveauxGravite = [
  { valeur: 'critique', label: 'Critique', couleurPastille: 'bg-red-600' },
  { valeur: 'grave', label: 'Grave', couleurPastille: 'bg-red-400' },
  { valeur: 'moyenne', label: 'Moyenne', couleurPastille: 'bg-orange-400' },
  { valeur: 'faible', label: 'Faible', couleurPastille: 'bg-yellow-400' },
]

const niveauxImpact = [
  { valeur: 'exemplaire', label: 'Exemplaire', couleurPastille: 'bg-green-600' },
  { valeur: 'fort', label: 'Fort', couleurPastille: 'bg-emerald-500' },
  { valeur: 'moyen', label: 'Moyen', couleurPastille: 'bg-lime-400' },
  { valeur: 'faible', label: 'Modeste', couleurPastille: 'bg-lime-300' },
]

const contributionsMauvaises = computed(() =>
  contributions.value.filter(c => c.typePratique !== 'bonne')
)
const contributionsBonnes = computed(() =>
  contributions.value.filter(c => c.typePratique === 'bonne')
)

const onglets = computed(() => [
  {
    value: 'toutes' as VueActive,
    label: 'Toutes',
    icon: ['fas', 'layer-group'],
    compte: contributions.value.length,
    activeClass: 'bg-gray-900 text-white shadow-sm',
  },
  {
    value: 'mauvaise' as VueActive,
    label: 'Badhabits',
    icon: ['fas', 'triangle-exclamation'],
    compte: contributionsMauvaises.value.length,
    activeClass: 'bg-linear-to-r from-red-600 to-orange-600 text-white shadow-sm',
  },
  {
    value: 'bonne' as VueActive,
    label: 'Goodhabits',
    icon: ['fas', 'thumbs-up'],
    compte: contributionsBonnes.value.length,
    activeClass: 'bg-linear-to-r from-emerald-600 to-green-600 text-white shadow-sm',
  },
])

const paysDisponibles = computed(() => {
  const pays = new Set(contributions.value.map(c => c.localisation.pays))
  return Array.from(pays).sort()
})

const compterParGravite = (gravite: string) =>
  contributionsMauvaises.value.filter(c => c.problematique?.gravite === gravite).length

const compterParImpact = (impact: string) =>
  contributionsBonnes.value.filter(c => c.bonnePratique?.impact === impact).length

const contributionsFiltrees = computed(() => {
  return contributions.value.filter(c => {
    if (vueActive.value === 'mauvaise' && c.typePratique === 'bonne') return false
    if (vueActive.value === 'bonne' && c.typePratique !== 'bonne') return false

    if (recherche.value) {
      const search = recherche.value.toLowerCase()
      if (!c.titre.toLowerCase().includes(search) && !c.description.toLowerCase().includes(search)) {
        return false
      }
    }
    if (paysSelectionne.value && c.localisation.pays !== paysSelectionne.value) {
      return false
    }
    if (vueActive.value !== 'bonne' && graviteSelectionnee.value && c.problematique?.gravite !== graviteSelectionnee.value) {
      return false
    }
    if (vueActive.value !== 'mauvaise' && impactSelectionne.value && c.bonnePratique?.impact !== impactSelectionne.value) {
      return false
    }
    return true
  })
})

const reinitialiser = () => {
  recherche.value = ''
  paysSelectionne.value = ''
  graviteSelectionnee.value = ''
  impactSelectionne.value = ''
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

const getImpactClass = (impact: string) => {
  const classes: Record<string, string> = {
    faible: 'bg-lime-100 text-lime-800',
    moyen: 'bg-lime-200 text-lime-900',
    fort: 'bg-emerald-100 text-emerald-800',
    exemplaire: 'bg-green-600 text-white'
  }
  return classes[impact] || 'bg-gray-100 text-gray-800'
}

const getBandeClass = (c: ContributionCitoyenne) => {
  if (c.typePratique === 'bonne') {
    const classes: Record<string, string> = {
      faible: 'bg-linear-to-r from-lime-300 to-lime-400',
      moyen: 'bg-linear-to-r from-lime-400 to-emerald-400',
      fort: 'bg-linear-to-r from-emerald-400 to-emerald-500',
      exemplaire: 'bg-linear-to-r from-emerald-600 to-green-600'
    }
    return classes[c.bonnePratique?.impact || ''] || 'bg-emerald-400'
  }
  const classes: Record<string, string> = {
    faible: 'bg-linear-to-r from-yellow-300 to-yellow-400',
    moyenne: 'bg-linear-to-r from-orange-400 to-orange-500',
    grave: 'bg-linear-to-r from-red-400 to-red-500',
    critique: 'bg-linear-to-r from-red-600 to-red-700'
  }
  return classes[c.problematique?.gravite || ''] || 'bg-gray-200'
}

const getIconeWrapperClass = (c: ContributionCitoyenne) => {
  if (c.typePratique === 'bonne') {
    const classes: Record<string, string> = {
      faible: 'bg-lime-100 text-lime-700',
      moyen: 'bg-lime-100 text-lime-700',
      fort: 'bg-emerald-100 text-emerald-700',
      exemplaire: 'bg-green-600 text-white'
    }
    return classes[c.bonnePratique?.impact || ''] || 'bg-emerald-100 text-emerald-700'
  }
  const classes: Record<string, string> = {
    faible: 'bg-yellow-100 text-yellow-600',
    moyenne: 'bg-orange-100 text-orange-600',
    grave: 'bg-red-100 text-red-600',
    critique: 'bg-red-600 text-white'
  }
  return classes[c.problematique?.gravite || ''] || 'bg-gray-100 text-gray-500'
}

const getIconeContribution = (c: ContributionCitoyenne): string[] => {
  if (c.typePratique === 'bonne') {
    const icones: Record<string, string[]> = {
      faible: ['fas', 'seedling'],
      moyen: ['fas', 'leaf'],
      fort: ['fas', 'hands-clapping'],
      exemplaire: ['fas', 'star']
    }
    return icones[c.bonnePratique?.impact || ''] || ['fas', 'thumbs-up']
  }
  const icones: Record<string, string[]> = {
    faible: ['fas', 'info-circle'],
    moyenne: ['fas', 'exclamation-circle'],
    grave: ['fas', 'exclamation-triangle'],
    critique: ['fas', 'skull-crossbones']
  }
  return icones[c.problematique?.gravite || ''] || ['fas', 'question-circle']
}

const getCategorieAffichee = (c: ContributionCitoyenne): string | undefined => {
  return c.typePratique === 'bonne'
    ? c.bonnePratique?.categorie
    : c.problematique?.categorie
}

const formatDate = (date: Date) => {
  return new Intl.DateTimeFormat('fr-FR', { day: 'numeric', month: 'short', year: 'numeric' }).format(new Date(date))
}

async function chargerContributions() {
  chargement.value = true
  erreurChargement.value = null
  try {
    const resultat = await getContributions({ type: 'badhabits', parPage: 50 })
    contributions.value = resultat.contributions
    cibler(resultat.contributions.map(c => c.id))
  } catch (e: unknown) {
    erreurChargement.value = e instanceof Error ? e.message : 'Erreur inconnue'
    contributions.value = []
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
