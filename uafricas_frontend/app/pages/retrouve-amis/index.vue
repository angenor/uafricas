<script setup lang="ts">
import type { AvisPublicResume, PaginationInfo, PaysInfo, TypeRelationRecherche } from '~/composables/useRetrouvAmis'
import { TYPES_RELATION } from '~/composables/useRetrouvAmis'
import { NOMS_PAYS_FR, normaliserNomPays } from '~/utils/carteAfrique'

definePageMeta({ layout: 'default' })

const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string
const userStore = useUserStore()
const { tableauDeBord, basculerTrouvable, rechercherAvisPublics } = useRetrouvAmis()

const estConnecte = computed(() => userStore.isAuthenticated)

// ── Mode d'affichage : liste ou carte ─────────────────────
const viewMode = ref<'liste' | 'carte'>('liste')

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

// ── Statut « trouvable » utilisateur ──────────────────────
const estTrouvable = ref(false)
const chargementTrouvable = ref(false)

const chargerStatutTrouvable = async () => {
  if (!estConnecte.value) return
  try {
    const res = await tableauDeBord()
    estTrouvable.value = res?.est_trouvable ?? false
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
    titre: 'Déposez un avis',
    description: 'Décrivez la personne que vous recherchez : nom, lieu de dernière rencontre, époque, détails physiques ou anecdotes.'
  },
  {
    icone: 'fa-magnifying-glass',
    titre: 'Le système compare',
    description: 'Notre algorithme croise votre avis avec les profils et autres avis de recherche pour identifier des correspondances potentielles.'
  },
  {
    icone: 'fa-handshake',
    titre: 'Acceptez le contact',
    description: 'Quand une correspondance est trouvée, les deux parties doivent accepter avant que les coordonnées ne soient partagées.'
  }
]

// ── Mode carte : répartition géographique des avis ────────
const chargementCarte = ref(false)
const carteChargee = ref(false)
const avisCarteBrut = ref<AvisPublicResume[]>([])

// Index de résolution pays (id / nom normalisé → code ISO2, et ISO2 → pays_id)
const isoParPaysId = ref<Record<string, string>>({})
const isoParNomNorm = ref<Record<string, string>>({})
const paysIdParIso = ref<Record<string, string>>({})

/** Résout le code ISO2 (minuscule) d'un pays d'avis via son id puis son nom. */
const resoudreIso = (pays?: PaysInfo): string | null => {
  if (!pays) return null
  const parId = isoParPaysId.value[pays.id]
  if (parId) return parId
  return isoParNomNorm.value[normaliserNomPays(pays.nom)] ?? null
}

/** Nombre d'avis par code ISO2 (recalculé quand les avis/index changent). */
const comptesParIso = computed<Record<string, number>>(() => {
  const map: Record<string, number> = {}
  for (const avis of avisCarteBrut.value) {
    const iso = resoudreIso(avis.pays)
    if (iso) map[iso] = (map[iso] || 0) + 1
  }
  return map
})

/** Charge le référentiel des pays (id, nom, code ISO2) une seule fois. */
const chargerReferentielPays = async () => {
  if (Object.keys(isoParPaysId.value).length > 0) return
  try {
    const rep = await $fetch<{ success: boolean; data: { id: string; nom: string; code_iso2: string | null }[] | null }>(
      `${apiBase}/api/pays`,
    )
    const parId: Record<string, string> = {}
    const parNom: Record<string, string> = {}
    const idParIso: Record<string, string> = {}
    for (const p of rep.data ?? []) {
      const iso = p.code_iso2?.toLowerCase()
      if (!iso) continue
      parId[p.id] = iso
      parNom[normaliserNomPays(p.nom)] = iso
      idParIso[iso] = p.id
    }
    isoParPaysId.value = parId
    isoParNomNorm.value = parNom
    paysIdParIso.value = idParIso
  } catch (e) {
    console.error('Erreur chargement référentiel pays:', e)
  }
}

/** Prépare la vue carte : référentiel pays + lot d'avis à géolocaliser. */
const chargerCarte = async () => {
  if (carteChargee.value) return
  chargementCarte.value = true
  try {
    await chargerReferentielPays()
    const res = await rechercherAvisPublics({ par_page: 300 })
    if (res) avisCarteBrut.value = res.avis
    carteChargee.value = true
  } finally {
    chargementCarte.value = false
  }
}

watch(viewMode, (mode) => {
  if (mode === 'carte') chargerCarte()
})

// ── Territoire sélectionné sur la carte ───────────────────
const paysSelectionneIso = ref<string | null>(null)
const avisPaysSelectionne = ref<AvisPublicResume[]>([])
const chargementPaysSel = ref(false)

const nomPaysSelectionne = computed(() =>
  paysSelectionneIso.value ? (NOMS_PAYS_FR[paysSelectionneIso.value] ?? paysSelectionneIso.value) : '',
)

const onSelectPays = async (iso: string) => {
  paysSelectionneIso.value = iso
  const paysId = paysIdParIso.value[iso]
  if (!paysId) {
    avisPaysSelectionne.value = []
    return
  }
  chargementPaysSel.value = true
  try {
    const res = await rechercherAvisPublics({ pays_id: paysId, par_page: 60 })
    avisPaysSelectionne.value = res?.avis ?? []
  } finally {
    chargementPaysSel.value = false
  }
}

onMounted(() => {
  chargerAvisPublics()
  chargerStatutTrouvable()
})
</script>

<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section (compact, titre ↔ description au survol) -->
    <div
      class="group relative bg-cover bg-center"
      style="background-image: url('https://images.unsplash.com/photo-1529156069898-49953e39b3ac?ixlib=rb-4.0.3&auto=format&fit=crop&w=1900&q=80')"
    >
      <div class="absolute inset-0 bg-linear-to-r from-custom-chocolat/90 to-black/70" />
      <div class="relative max-w-4xl mx-auto px-4 pt-16 pb-6 text-center select-none">
        <!-- Conteneur fixe : titre et description se superposent (crossfade au survol) -->
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12">
          <h1 class="absolute inset-0 flex items-center justify-center text-white text-2xl md:text-4xl font-bold transition-opacity duration-300 group-hover:opacity-0">
            Retrouver une personne perdue de vue
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Retrouvez vos amis, proches et connaissances perdus de vue grâce à la communauté panafricaine.
          </p>
        </div>
        <div class="flex flex-wrap justify-center gap-4 mt-6">
          <button
            v-if="estConnecte"
            class="px-6 py-3 bg-white text-custom-chocolat font-semibold rounded-lg hover:bg-amber-50 transition-colors cursor-pointer"
            @click="onCreerAvis"
          >
            <font-awesome-icon :icon="['fas', 'plus']" class="mr-2" />
            Créer un avis de recherche
          </button>
          <button
            v-if="estConnecte"
            class="px-6 py-3 border-2 border-white rounded-lg font-semibold transition-colors cursor-pointer"
            :class="estTrouvable ? 'bg-white/20 text-white' : 'bg-transparent text-white hover:bg-white/10'"
            :disabled="chargementTrouvable"
            @click="onActiverTrouvable"
          >
            <font-awesome-icon :icon="['fas', estTrouvable ? 'eye' : 'eye-slash']" class="mr-2" />
            {{ estTrouvable ? 'Vous êtes trouvable' : 'Devenir trouvable' }}
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

    <!-- Comment ça marche (placé avant le listing : comprendre avant de commencer) -->
    <section class="py-10 px-4 bg-white border-b border-gray-100">
      <div class="max-w-5xl mx-auto">
        <h2 class="text-xl font-bold text-center text-gray-800 mb-8 font-[Oswald]">
          Comment ça marche ?
        </h2>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-x-8 gap-y-6">
          <div
            v-for="(etape, index) in etapes"
            :key="index"
            class="flex items-start gap-3"
          >
            <span class="shrink-0 w-7 h-7 bg-amber-100 text-amber-700 rounded-full flex items-center justify-center text-sm font-bold">
              {{ index + 1 }}
            </span>
            <div>
              <h3 class="text-sm font-semibold text-gray-800 mb-0.5">
                <font-awesome-icon :icon="['fas', etape.icone]" class="mr-1.5 text-amber-600" />
                {{ etape.titre }}
              </h3>
              <p class="text-gray-500 text-xs leading-relaxed">{{ etape.description }}</p>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Avis publics : listing principal (+ sidebar si connecté) -->
    <section class="py-16 px-4">
      <div class="max-w-7xl mx-auto lg:flex lg:gap-8">
        <RetrouveAmisSideBar v-if="estConnecte" />
        <div class="flex-1 min-w-0">
        <!-- Barre de filtres compacte -->
        <div class="mb-10 rounded-2xl bg-white p-3 shadow-sm ring-1 ring-gray-200/60">
          <div class="flex flex-wrap items-center gap-2">
            <!-- Toggle liste / carte -->
            <div class="flex items-center rounded-xl bg-gray-100 p-1">
              <button
                class="flex items-center gap-2 rounded-lg px-3 py-1.5 text-sm font-medium transition-colors cursor-pointer"
                :class="viewMode === 'liste' ? 'bg-custom-chocolat text-white shadow-sm' : 'text-gray-500 hover:text-gray-700'"
                title="Vue liste"
                @click="viewMode = 'liste'"
              >
                <font-awesome-icon :icon="['fas', 'list-check']" />
                <span class="hidden sm:inline">Liste</span>
              </button>
              <button
                class="flex items-center gap-2 rounded-lg px-3 py-1.5 text-sm font-medium transition-colors cursor-pointer"
                :class="viewMode === 'carte' ? 'bg-custom-chocolat text-white shadow-sm' : 'text-gray-500 hover:text-gray-700'"
                title="Vue carte"
                @click="viewMode = 'carte'"
              >
                <font-awesome-icon :icon="['fas', 'earth-africa']" />
                <span class="hidden sm:inline">Carte</span>
              </button>
            </div>

            <!-- Recherche -->
            <div v-if="viewMode === 'liste'" class="relative flex-1 min-w-56">
              <font-awesome-icon :icon="['fas', 'magnifying-glass']" class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 text-sm" />
              <input
                v-model="filtreRecherche"
                type="text"
                placeholder="Rechercher un nom, un lieu, une école..."
                class="w-full rounded-xl bg-gray-50 pl-9 pr-3 py-2 text-sm text-gray-700 placeholder-gray-400 ring-1 ring-gray-200 focus:outline-none focus:ring-2 focus:ring-amber-500 focus:bg-white transition-all"
                @keyup.enter="appliquerFiltres"
              >
            </div>
            <!-- Filtre relation -->
            <div v-if="viewMode === 'liste'" class="min-w-44">
              <select
                v-model="filtreRelation"
                class="w-full rounded-xl bg-gray-50 px-3 py-2 text-sm text-gray-700 ring-1 ring-gray-200 focus:outline-none focus:ring-2 focus:ring-amber-500 focus:bg-white transition-all appearance-none cursor-pointer"
                @change="appliquerFiltres"
              >
                <option value="">Toutes les relations</option>
                <option v-for="t in TYPES_RELATION" :key="t.value" :value="t.value">
                  {{ t.label }}
                </option>
              </select>
            </div>
            <!-- Bouton recherche (input texte non réactif) -->
            <button
              v-if="viewMode === 'liste'"
              class="flex items-center gap-2 rounded-xl bg-custom-chocolat px-4 py-2 text-sm font-semibold text-white shadow-sm transition-all hover:bg-amber-800 hover:shadow-md active:scale-95 cursor-pointer"
              @click="appliquerFiltres"
            >
              <font-awesome-icon :icon="['fas', 'magnifying-glass']" />
              Rechercher
            </button>
            <button
              v-if="viewMode === 'liste' && filtresActifs"
              class="flex items-center gap-2 rounded-xl bg-gray-100 px-3 py-2 text-sm font-medium text-gray-600 transition-colors hover:bg-gray-200 cursor-pointer"
              @click="reinitialiserFiltres"
            >
              <font-awesome-icon :icon="['fas', 'xmark']" />
              Réinitialiser
            </button>

            <!-- Aide mode carte -->
            <p v-if="viewMode === 'carte'" class="flex items-center gap-2 px-2 text-sm text-gray-500">
              <font-awesome-icon :icon="['fas', 'location-dot']" class="text-custom-chocolat" />
              Cliquez sur un territoire pour voir les avis de recherche qui y sont rattachés.
            </p>
          </div>
        </div>

        <!-- Mode carte : répartition géographique des avis -->
        <div v-if="viewMode === 'carte'" class="grid grid-cols-1 lg:grid-cols-5 gap-6">
          <!-- Carte SVG d'Afrique -->
          <div class="lg:col-span-3">
            <div class="rounded-2xl bg-white p-4 shadow-sm ring-1 ring-gray-200/60">
              <!-- Chargement carte -->
              <div v-if="chargementCarte" class="flex flex-col items-center justify-center py-20">
                <div class="h-12 w-12 animate-spin rounded-full border-4 border-amber-200 border-t-amber-600" />
                <p class="mt-4 text-sm text-gray-500">Chargement de la carte...</p>
              </div>

              <template v-else>
                <RetrouveAmisCarteAfrique
                  :comptes="comptesParIso"
                  :selected-iso="paysSelectionneIso"
                  @select="onSelectPays"
                />

                <!-- Légende (échelle de chaleur) -->
                <div class="mt-4 flex flex-wrap items-center justify-center gap-x-4 gap-y-2 border-t border-gray-100 pt-4">
                  <span class="text-xs font-medium text-gray-500">Nombre d'avis :</span>
                  <div class="flex items-center gap-1.5">
                    <span class="h-3 w-3 rounded-full" style="background-color: #fbbf24" />
                    <span class="text-xs text-gray-600">1-2</span>
                  </div>
                  <div class="flex items-center gap-1.5">
                    <span class="h-3 w-3 rounded-full" style="background-color: #f59e0b" />
                    <span class="text-xs text-gray-600">3-5</span>
                  </div>
                  <div class="flex items-center gap-1.5">
                    <span class="h-3 w-3 rounded-full" style="background-color: #d97706" />
                    <span class="text-xs text-gray-600">6-9</span>
                  </div>
                  <div class="flex items-center gap-1.5">
                    <span class="h-3 w-3 rounded-full" style="background-color: #b45309" />
                    <span class="text-xs text-gray-600">10+</span>
                  </div>
                  <div class="flex items-center gap-1.5">
                    <span class="h-3 w-3 rounded-full" style="background-color: #e5e7eb" />
                    <span class="text-xs text-gray-600">Aucun</span>
                  </div>
                </div>
              </template>
            </div>
          </div>

          <!-- Panneau du territoire sélectionné -->
          <div class="lg:col-span-2">
            <div class="sticky top-4">
              <!-- Aucun territoire sélectionné -->
              <div
                v-if="!paysSelectionneIso"
                class="flex flex-col items-center justify-center rounded-2xl bg-white px-6 py-16 text-center shadow-sm ring-1 ring-gray-200/60"
              >
                <div class="mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-amber-50">
                  <font-awesome-icon :icon="['fas', 'hand-pointer']" class="text-2xl text-amber-400" />
                </div>
                <h3 class="text-base font-semibold text-gray-700">Explorez la carte</h3>
                <p class="mt-2 text-sm text-gray-400">
                  Sélectionnez un territoire coloré pour afficher les avis de recherche qui y sont rattachés.
                </p>
              </div>

              <!-- Territoire sélectionné -->
              <div v-else>
                <div class="mb-4 flex items-center justify-between">
                  <h3 class="flex items-center gap-2 text-lg font-bold text-gray-800">
                    <font-awesome-icon :icon="['fas', 'location-dot']" class="text-custom-chocolat" />
                    {{ nomPaysSelectionne }}
                  </h3>
                  <button
                    class="flex h-8 w-8 items-center justify-center rounded-full text-gray-400 transition-colors hover:bg-gray-100 hover:text-gray-600 cursor-pointer"
                    title="Fermer"
                    @click="paysSelectionneIso = null"
                  >
                    <font-awesome-icon :icon="['fas', 'xmark']" />
                  </button>
                </div>

                <!-- Chargement des avis du territoire -->
                <div v-if="chargementPaysSel" class="flex flex-col items-center justify-center py-16">
                  <div class="h-10 w-10 animate-spin rounded-full border-4 border-amber-200 border-t-amber-600" />
                </div>

                <!-- Aucun avis (ne devrait pas arriver si sélectionnable) -->
                <div
                  v-else-if="avisPaysSelectionne.length === 0"
                  class="rounded-2xl bg-white px-6 py-12 text-center text-sm text-gray-400 shadow-sm ring-1 ring-gray-200/60"
                >
                  Aucun avis de recherche pour ce territoire.
                </div>

                <!-- Liste des avis du territoire -->
                <div v-else class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-1 gap-5">
                  <RetrouveAmisCarteAvisPublic
                    v-for="avis in avisPaysSelectionne"
                    :key="avis.id"
                    :avis="avis"
                  />
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Mode liste -->
        <template v-else>
        <!-- Chargement -->
        <div v-if="chargementAvis" class="flex flex-col items-center justify-center py-20">
          <div class="h-12 w-12 animate-spin rounded-full border-4 border-amber-200 border-t-amber-600" />
          <p class="mt-4 text-sm text-gray-500">Chargement des avis...</p>
        </div>

        <!-- État vide : aucun résultat avec filtres actifs -->
        <div v-else-if="avisPublics.length === 0 && filtresActifs" class="text-center py-20">
          <div class="mx-auto mb-6 flex h-24 w-24 items-center justify-center rounded-full bg-gray-100">
            <font-awesome-icon :icon="['fas', 'filter-circle-xmark']" class="text-4xl text-gray-300" />
          </div>
          <h3 class="text-xl font-semibold text-gray-700 mb-2">
            Aucun résultat pour ces critères
          </h3>
          <p class="text-gray-400 max-w-md mx-auto mb-8 text-sm">
            Essayez de modifier vos critères de recherche ou de réinitialiser les filtres.
          </p>
          <button
            class="inline-flex items-center gap-2 rounded-xl bg-custom-chocolat px-6 py-3 text-sm font-semibold text-white shadow-sm hover:bg-amber-800 transition-colors cursor-pointer"
            @click="reinitialiserFiltres"
          >
            <font-awesome-icon :icon="['fas', 'rotate-left']" />
            Réinitialiser les filtres
          </button>
        </div>

        <!-- État vide : aucun avis disponible -->
        <div v-else-if="avisPublics.length === 0" class="text-center py-20">
          <div class="mx-auto mb-6 flex h-24 w-24 items-center justify-center rounded-full bg-amber-50">
            <font-awesome-icon :icon="['fas', 'users']" class="text-4xl text-amber-300" />
          </div>
          <h3 class="text-xl font-semibold text-gray-700 mb-2">
            Aucun avis de recherche pour le moment
          </h3>
          <p class="text-gray-400 max-w-md mx-auto mb-8 text-sm">
            Soyez le premier à publier un avis de recherche et aidez à réunir des proches séparés.
          </p>
          <NuxtLink
            v-if="estConnecte"
            to="/retrouve-amis/nouveau"
            class="inline-flex items-center gap-2 rounded-xl bg-custom-chocolat px-6 py-3 text-sm font-semibold text-white shadow-sm hover:bg-amber-800 transition-colors"
          >
            <font-awesome-icon :icon="['fas', 'plus']" />
            Créer le premier avis
          </NuxtLink>
          <NuxtLink
            v-else
            to="/login"
            class="inline-flex items-center gap-2 rounded-xl bg-custom-chocolat px-6 py-3 text-sm font-semibold text-white shadow-sm hover:bg-amber-800 transition-colors"
          >
            Se connecter pour créer un avis
          </NuxtLink>
        </div>

        <!-- Grille des avis -->
        <template v-else>
          <!-- Compteur de résultats -->
          <p v-if="pagination" class="mb-6 text-sm text-gray-500">
            <span class="font-semibold text-gray-700">{{ pagination.total }}</span> avis de recherche
            <span v-if="filtresActifs"> correspondant à vos critères</span>
          </p>

          <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
            <RetrouveAmisCarteAvisPublic
              v-for="avis in avisPublics"
              :key="avis.id"
              :avis="avis"
            />
          </div>

          <!-- Pagination -->
          <div v-if="pagination && pagination.pages > 1" class="mt-12 flex items-center justify-center gap-1">
            <button
              class="flex items-center gap-1.5 rounded-lg px-4 py-2.5 text-sm font-medium text-gray-600 transition-colors hover:bg-gray-100 disabled:opacity-30 disabled:cursor-not-allowed cursor-pointer"
              :disabled="pageActuelle <= 1"
              @click="chargerAvisPublics(pageActuelle - 1)"
            >
              <font-awesome-icon :icon="['fas', 'chevron-left']" class="text-xs" />
              Précédent
            </button>
            <!-- Numéros de pages -->
            <template v-for="p in pagination.pages" :key="p">
              <button
                v-if="p === 1 || p === pagination.pages || (p >= pageActuelle - 1 && p <= pageActuelle + 1)"
                class="h-10 w-10 rounded-lg text-sm font-medium transition-colors cursor-pointer"
                :class="p === pageActuelle ? 'bg-custom-chocolat text-white shadow-sm' : 'text-gray-600 hover:bg-gray-100'"
                @click="chargerAvisPublics(p)"
              >
                {{ p }}
              </button>
              <span
                v-else-if="p === pageActuelle - 2 || p === pageActuelle + 2"
                class="px-1 text-gray-400"
              >...</span>
            </template>
            <button
              class="flex items-center gap-1.5 rounded-lg px-4 py-2.5 text-sm font-medium text-gray-600 transition-colors hover:bg-gray-100 disabled:opacity-30 disabled:cursor-not-allowed cursor-pointer"
              :disabled="pageActuelle >= pagination.pages"
              @click="chargerAvisPublics(pageActuelle + 1)"
            >
              Suivant
              <font-awesome-icon :icon="['fas', 'chevron-right']" class="text-xs" />
            </button>
          </div>
        </template>
        </template>
        </div>
      </div>
    </section>

    <!-- CTA inscription -->
    <section v-if="!estConnecte" class="py-16 px-4 bg-amber-50">
      <div class="max-w-3xl mx-auto text-center">
        <h2 class="text-2xl font-bold text-gray-800 mb-4 font-[Oswald]">
          Rejoignez la communauté
        </h2>
        <p class="text-gray-600 mb-8">
          Inscrivez-vous gratuitement pour déposer un avis de recherche et retrouver vos proches perdus de vue.
        </p>
        <NuxtLink
          to="/login?mode=inscription"
          class="inline-block px-8 py-3 bg-amber-700 text-white font-semibold rounded-lg hover:bg-amber-800 transition-colors"
        >
          Créer un compte gratuitement
        </NuxtLink>
      </div>
    </section>
  </div>
</template>
