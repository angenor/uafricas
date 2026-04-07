<script setup lang="ts">
import type { AvisPublicResume, PaginationInfo, TypeRelationRecherche } from '~/composables/useRetrouvAmis'
import { TYPES_RELATION } from '~/composables/useRetrouvAmis'

definePageMeta({ layout: 'default' })

const userStore = useUserStore()
const { tableauDeBord, basculerTrouvable, rechercherAvisPublics } = useRetrouvAmis()

const estConnecte = computed(() => userStore.isAuthenticated)

// ── Filtres ───────────────────────────────────────────────
const filtreRelation = ref<TypeRelationRecherche | ''>('')
const filtreRecherche = ref('')
const filtresActifs = computed(() => filtreRelation.value !== '' || filtreRecherche.value.trim() !== '')

// ── Avis publics ──────────────────────────────────────────
const avisPublics = ref<AvisPublicResume[]>([])
const pagination = ref<PaginationInfo | null>(null)
const chargementAvis = ref(false)
const pageActuelle = ref(1)

const chargerAvisPublics = async (page: number = 1) => {
  chargementAvis.value = true
  try {
    const params: Record<string, any> = { page, par_page: 12 }
    if (filtreRelation.value) params.type_relation = filtreRelation.value
    if (filtreRecherche.value.trim()) params.recherche = filtreRecherche.value.trim()

    const res = await rechercherAvisPublics(params)
    if (res) {
      avisPublics.value = res.avis
      pagination.value = res.pagination
      pageActuelle.value = page
    }
  } finally {
    chargementAvis.value = false
  }
}

const appliquerFiltres = () => {
  chargerAvisPublics(1)
}

const reinitialiserFiltres = () => {
  filtreRelation.value = ''
  filtreRecherche.value = ''
  chargerAvisPublics(1)
}

// ── Dashboard utilisateur ─────────────────────────────────
const dashboard = ref<{ avis_actifs: number; correspondances_en_attente: number; notifications_non_lues: number } | null>(null)
const estTrouvable = ref(false)
const chargementTrouvable = ref(false)

const chargerTableauDeBord = async () => {
  if (!estConnecte.value) return
  try {
    const res = await tableauDeBord()
    dashboard.value = res
    estTrouvable.value = res.est_trouvable ?? false
  } catch {
    // silencieux sur page publique
  }
}

const onCreerAvis = () => {
  navigateTo('/retrouve-amis/nouveau')
}

const onActiverTrouvable = async () => {
  chargementTrouvable.value = true
  try {
    const res = await basculerTrouvable(!estTrouvable.value)
    if (res) {
      estTrouvable.value = res.est_trouvable
    }
  } finally {
    chargementTrouvable.value = false
  }
}

const etapes = [
  {
    icone: 'fa-pen-to-square',
    titre: 'Deposez un avis',
    description: 'Decrivez la personne que vous recherchez : nom, lieu de derniere rencontre, epoque, details physiques ou anecdotes.'
  },
  {
    icone: 'fa-magnifying-glass',
    titre: 'Le systeme compare',
    description: 'Notre algorithme croise votre avis avec les profils et autres avis de recherche pour identifier des correspondances potentielles.'
  },
  {
    icone: 'fa-handshake',
    titre: 'Acceptez le contact',
    description: 'Quand une correspondance est trouvee, les deux parties doivent accepter avant que les coordonnees ne soient partagees.'
  }
]

onMounted(() => {
  chargerAvisPublics()
  chargerTableauDeBord()
})
</script>

<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section -->
    <div
      class="relative h-96 bg-cover bg-center"
      style="background-image: url('https://images.unsplash.com/photo-1529156069898-49953e39b3ac?ixlib=rb-4.0.3&auto=format&fit=crop&w=1900&q=80')"
    >
      <div class="absolute inset-0 bg-linear-to-r from-custom-chocolat/90 to-black/70" />
      <div class="absolute inset-0 flex flex-col items-center justify-center mt-14">
        <h1 class="text-white text-4xl md:text-5xl font-bold mb-4 animate-title">
          Retrouvez vos proches
        </h1>
        <div class="h-1 w-24 bg-custom-green rounded animate-line" />
        <p class="text-white/80 text-sm md:text-base mt-3 max-w-3xl text-center px-4 animate-subtitle">
          Retrouvez vos amis, proches et connaissances perdus de vue grace a la communaute panafricaine.
        </p>
        <div class="flex flex-wrap justify-center gap-4 mt-8">
          <button
            v-if="estConnecte"
            class="px-6 py-3 bg-white text-custom-chocolat font-semibold rounded-lg hover:bg-amber-50 transition-colors cursor-pointer"
            @click="onCreerAvis"
          >
            <font-awesome-icon :icon="['fas', 'plus']" class="mr-2" />
            Creer un avis de recherche
          </button>
          <button
            v-if="estConnecte"
            class="px-6 py-3 border-2 border-white rounded-lg font-semibold transition-colors cursor-pointer"
            :class="estTrouvable ? 'bg-white/20 text-white' : 'bg-transparent text-white hover:bg-white/10'"
            :disabled="chargementTrouvable"
            @click="onActiverTrouvable"
          >
            <font-awesome-icon :icon="['fas', estTrouvable ? 'eye' : 'eye-slash']" class="mr-2" />
            {{ estTrouvable ? 'Vous etes trouvable' : 'Devenir trouvable' }}
          </button>
          <NuxtLink
            v-if="!estConnecte"
            to="/login"
            class="px-6 py-3 bg-white text-custom-chocolat font-semibold rounded-lg hover:bg-amber-50 transition-colors"
          >
            Se connecter pour commencer
          </NuxtLink>
        </div>
      </div>
    </div>

    <!-- Avis publics : listing principal -->
    <section class="py-12 px-4">
      <div class="max-w-6xl mx-auto">
        <h2 class="text-3xl font-bold text-gray-800 mb-8 font-[Oswald]">
          Avis de recherche
        </h2>

        <!-- Filtres -->
        <div class="flex flex-wrap items-end gap-3 mb-8">
          <div class="flex-1 min-w-50">
            <label class="block text-sm font-medium text-gray-700 mb-1">Rechercher</label>
            <div class="relative">
              <font-awesome-icon :icon="['fas', 'magnifying-glass']" class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 text-sm" />
              <input
                v-model="filtreRecherche"
                type="text"
                placeholder="Nom, lieu, ecole..."
                class="w-full pl-9 pr-3 py-2.5 border border-gray-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-amber-500 focus:border-transparent"
                @keyup.enter="appliquerFiltres"
              >
            </div>
          </div>
          <div class="min-w-45">
            <label class="block text-sm font-medium text-gray-700 mb-1">Type de relation</label>
            <select
              v-model="filtreRelation"
              class="w-full px-3 py-2.5 border border-gray-300 rounded-lg text-sm bg-white focus:outline-none focus:ring-2 focus:ring-amber-500 focus:border-transparent"
              @change="appliquerFiltres"
            >
              <option value="">Toutes les relations</option>
              <option v-for="t in TYPES_RELATION" :key="t.value" :value="t.value">
                {{ t.label }}
              </option>
            </select>
          </div>
          <button
            class="px-4 py-2.5 bg-amber-700 text-white text-sm font-medium rounded-lg hover:bg-amber-800 transition-colors cursor-pointer"
            @click="appliquerFiltres"
          >
            <font-awesome-icon :icon="['fas', 'magnifying-glass']" class="mr-1.5" />
            Rechercher
          </button>
          <button
            v-if="filtresActifs"
            class="px-4 py-2.5 border border-gray-300 text-gray-700 text-sm font-medium rounded-lg hover:bg-gray-100 transition-colors cursor-pointer"
            @click="reinitialiserFiltres"
          >
            <font-awesome-icon :icon="['fas', 'xmark']" class="mr-1.5" />
            Reinitialiser
          </button>
        </div>

        <!-- Chargement -->
        <div v-if="chargementAvis" class="flex justify-center py-16">
          <div class="animate-spin rounded-full h-10 w-10 border-b-2 border-amber-700" />
        </div>

        <!-- Etat vide : aucun resultat avec filtres actifs -->
        <div v-else-if="avisPublics.length === 0 && filtresActifs" class="text-center py-16">
          <div class="w-20 h-20 mx-auto mb-6 bg-gray-100 text-gray-400 rounded-full flex items-center justify-center">
            <font-awesome-icon :icon="['fas', 'filter-circle-xmark']" class="text-3xl" />
          </div>
          <h3 class="text-xl font-semibold text-gray-700 mb-3">
            Aucun resultat pour ces criteres
          </h3>
          <p class="text-gray-500 max-w-md mx-auto mb-6">
            Essayez de modifier vos criteres de recherche ou de reinitialiser les filtres.
          </p>
          <button
            class="inline-block px-6 py-3 bg-amber-700 text-white font-semibold rounded-lg hover:bg-amber-800 transition-colors cursor-pointer"
            @click="reinitialiserFiltres"
          >
            <font-awesome-icon :icon="['fas', 'rotate-left']" class="mr-2" />
            Reinitialiser les filtres
          </button>
        </div>

        <!-- Etat vide : aucun avis disponible -->
        <div v-else-if="avisPublics.length === 0" class="text-center py-16">
          <div class="w-20 h-20 mx-auto mb-6 bg-amber-100 text-amber-600 rounded-full flex items-center justify-center">
            <font-awesome-icon :icon="['fas', 'users']" class="text-3xl" />
          </div>
          <h3 class="text-xl font-semibold text-gray-700 mb-3">
            Aucun avis de recherche pour le moment
          </h3>
          <p class="text-gray-500 max-w-md mx-auto mb-6">
            Soyez le premier a publier un avis de recherche et aidez a reunir des proches separes.
          </p>
          <NuxtLink
            v-if="estConnecte"
            to="/retrouve-amis/nouveau"
            class="inline-block px-6 py-3 bg-amber-700 text-white font-semibold rounded-lg hover:bg-amber-800 transition-colors"
          >
            <font-awesome-icon :icon="['fas', 'plus']" class="mr-2" />
            Creer le premier avis
          </NuxtLink>
          <NuxtLink
            v-else
            to="/login"
            class="inline-block px-6 py-3 bg-amber-700 text-white font-semibold rounded-lg hover:bg-amber-800 transition-colors"
          >
            Se connecter pour creer un avis
          </NuxtLink>
        </div>

        <!-- Grille des avis -->
        <template v-else>
          <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
            <RetrouveAmisCarteAvisPublic
              v-for="avis in avisPublics"
              :key="avis.id"
              :avis="avis"
            />
          </div>

          <!-- Pagination -->
          <div v-if="pagination && pagination.pages > 1" class="flex justify-center items-center gap-2 mt-10">
            <button
              class="px-4 py-2 text-sm rounded-lg border border-gray-300 hover:bg-gray-100 transition-colors disabled:opacity-40 disabled:cursor-not-allowed"
              :disabled="pageActuelle <= 1"
              @click="chargerAvisPublics(pageActuelle - 1)"
            >
              Precedent
            </button>
            <span class="text-sm text-gray-600 px-3">
              Page {{ pageActuelle }} / {{ pagination.pages }}
            </span>
            <button
              class="px-4 py-2 text-sm rounded-lg border border-gray-300 hover:bg-gray-100 transition-colors disabled:opacity-40 disabled:cursor-not-allowed"
              :disabled="pageActuelle >= pagination.pages"
              @click="chargerAvisPublics(pageActuelle + 1)"
            >
              Suivant
            </button>
          </div>
        </template>
      </div>
    </section>

    <!-- Comment ca marche -->
    <section class="py-16 px-4 bg-white">
      <div class="max-w-5xl mx-auto">
        <h2 class="text-3xl font-bold text-center text-gray-800 mb-12 font-[Oswald]">
          Comment ca marche ?
        </h2>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
          <div
            v-for="(etape, index) in etapes"
            :key="index"
            class="bg-gray-50 rounded-xl shadow-sm border border-gray-200 p-8 text-center hover:shadow-md transition-shadow"
          >
            <div class="w-16 h-16 mx-auto mb-5 bg-amber-100 text-amber-700 rounded-full flex items-center justify-center">
              <font-awesome-icon :icon="['fas', etape.icone]" class="text-2xl" />
            </div>
            <span class="inline-block w-8 h-8 bg-amber-700 text-white rounded-full text-sm font-bold leading-8 mb-3">
              {{ index + 1 }}
            </span>
            <h3 class="text-lg font-semibold text-gray-800 mb-2">{{ etape.titre }}</h3>
            <p class="text-gray-600 text-sm leading-relaxed">{{ etape.description }}</p>
          </div>
        </div>
      </div>
    </section>

    <!-- Tableau de bord (connecte) -->
    <section v-if="estConnecte && dashboard" class="py-12 px-4">
      <div class="max-w-5xl mx-auto">
        <h2 class="text-2xl font-bold text-gray-800 mb-8 font-[Oswald]">
          Votre tableau de bord
        </h2>
        <div class="grid grid-cols-1 sm:grid-cols-3 gap-6">
          <NuxtLink
            to="/retrouve-amis/mes-recherches"
            class="bg-amber-50 border border-amber-200 rounded-xl p-6 text-center hover:shadow-md hover:border-amber-300 transition-all"
          >
            <p class="text-3xl font-bold text-amber-800">{{ dashboard.avis_actifs }}</p>
            <p class="text-sm text-amber-700 mt-1">Avis actifs</p>
            <p class="text-xs text-amber-500 mt-2">
              <font-awesome-icon :icon="['fas', 'arrow-right']" class="mr-1" />
              Voir mes recherches
            </p>
          </NuxtLink>
          <NuxtLink
            to="/retrouve-amis/correspondances"
            class="bg-green-50 border border-green-200 rounded-xl p-6 text-center hover:shadow-md hover:border-green-300 transition-all"
          >
            <p class="text-3xl font-bold text-green-800">{{ dashboard.correspondances_en_attente }}</p>
            <p class="text-sm text-green-700 mt-1">Correspondances en attente</p>
            <p class="text-xs text-green-500 mt-2">
              <font-awesome-icon :icon="['fas', 'arrow-right']" class="mr-1" />
              Voir les correspondances
            </p>
          </NuxtLink>
          <NuxtLink
            to="/retrouve-amis/correspondances"
            class="bg-blue-50 border border-blue-200 rounded-xl p-6 text-center hover:shadow-md hover:border-blue-300 transition-all"
          >
            <p class="text-3xl font-bold text-blue-800">{{ dashboard.notifications_non_lues }}</p>
            <p class="text-sm text-blue-700 mt-1">Notifications non lues</p>
            <p class="text-xs text-blue-500 mt-2">
              <font-awesome-icon :icon="['fas', 'arrow-right']" class="mr-1" />
              Consulter
            </p>
          </NuxtLink>
        </div>
      </div>
    </section>

    <!-- CTA inscription -->
    <section v-if="!estConnecte" class="py-16 px-4 bg-amber-50">
      <div class="max-w-3xl mx-auto text-center">
        <h2 class="text-2xl font-bold text-gray-800 mb-4 font-[Oswald]">
          Rejoignez la communaute
        </h2>
        <p class="text-gray-600 mb-8">
          Inscrivez-vous gratuitement pour deposer un avis de recherche et retrouver vos proches perdus de vue.
        </p>
        <NuxtLink
          to="/login?mode=inscription"
          class="inline-block px-8 py-3 bg-amber-700 text-white font-semibold rounded-lg hover:bg-amber-800 transition-colors"
        >
          Creer un compte gratuitement
        </NuxtLink>
      </div>
    </section>
  </div>
</template>
