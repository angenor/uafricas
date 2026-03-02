<script setup lang="ts">
import type { AvisPublicResume, PaysInfo, RecherchePubliqueParams, PaginationInfo } from '~/composables/useRetrouvAmis'

definePageMeta({ layout: 'default' })

const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string
const { rechercherAvisPublics, chargement, erreur } = useRetrouvAmis()

// SEO
useSeoMeta({
  title: 'Rechercher un avis de recherche — UAfricas',
  description: 'Parcourez les avis de recherche publics sur UAfricas. Aidez les gens a retrouver des amis perdus de vue en Afrique et dans le monde.',
  ogTitle: 'Rechercher un avis de recherche — UAfricas',
  ogDescription: 'Parcourez les avis de recherche publics et aidez a retrouver des amis perdus de vue.',
  ogType: 'website',
  ogUrl: 'https://www.africans-world.org/retrouve-amis/rechercher',
})

// ── Etat local ──────────────────────────────────────────
const avis = ref<AvisPublicResume[]>([])
const pagination = ref<PaginationInfo>({ page: 1, par_page: 12, total: 0, pages: 0 })
const recherche = ref('')
const paysId = ref('')
const ville = ref('')
const ecole = ref('')
const tri = ref<'created_at' | 'compteur_partages'>('created_at')
const ordre = ref<'asc' | 'desc'>('desc')
const paysListe = ref<PaysInfo[]>([])

// ── Charger la liste des pays ───────────────────────────
const chargerPays = async () => {
  try {
    const reponse = await $fetch<{ success: boolean; data: PaysInfo[] }>(
      `${apiBase}/api/retrouve-amis/pays`,
    )
    if (reponse.success && reponse.data) {
      paysListe.value = reponse.data
    }
  }
  catch {
    // Silencieux
  }
}

// ── Rechercher les avis ──────────────────────────────────
const chargerAvis = async (page = 1) => {
  const params: RecherchePubliqueParams = {
    page,
    par_page: 12,
    tri: tri.value,
    ordre: ordre.value,
  }
  if (recherche.value.trim()) params.recherche = recherche.value.trim()
  if (paysId.value) params.pays_id = paysId.value
  if (ville.value.trim()) params.ville = ville.value.trim()
  if (ecole.value.trim()) params.ecole = ecole.value.trim()

  const resultat = await rechercherAvisPublics(params)
  if (resultat) {
    avis.value = resultat.avis
    pagination.value = resultat.pagination
  }
}

// ── Navigation pagination ────────────────────────────────
const allerPage = (page: number) => {
  if (page < 1 || page > pagination.value.pages) return
  chargerAvis(page)
  window.scrollTo({ top: 0, behavior: 'smooth' })
}

// ── Reinitialiser les filtres ────────────────────────────
const reinitialiser = () => {
  recherche.value = ''
  paysId.value = ''
  ville.value = ''
  ecole.value = ''
  tri.value = 'created_at'
  ordre.value = 'desc'
  chargerAvis(1)
}

// ── Soumettre la recherche ───────────────────────────────
const soumettreRecherche = () => {
  chargerAvis(1)
}

// ── Pages visibles pour la pagination ────────────────────
const pagesVisibles = computed(() => {
  const total = pagination.value.pages
  const current = pagination.value.page
  const pages: number[] = []
  const debut = Math.max(1, current - 2)
  const fin = Math.min(total, current + 2)
  for (let i = debut; i <= fin; i++) {
    pages.push(i)
  }
  return pages
})

const aFiltresActifs = computed(() => {
  return recherche.value.trim() || paysId.value || ville.value.trim() || ecole.value.trim()
})

// ── Chargement initial ───────────────────────────────────
chargerPays()
chargerAvis()
</script>

<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero -->
    <div
      class="relative h-56 bg-cover bg-center"
      style="background-image: url('https://images.unsplash.com/photo-1529156069898-49953e39b3ac?ixlib=rb-4.0.3&auto=format&fit=crop&w=1900&q=80')"
    >
      <div class="absolute inset-0 bg-gradient-to-r from-custom-chocolat/90 to-black/70" />
      <div class="absolute inset-0 flex flex-col items-center justify-center mt-10">
        <h1 class="text-white text-2xl md:text-3xl font-bold mb-2">
          Rechercher un avis de recherche
        </h1>
        <p class="text-white/80 text-sm md:text-base max-w-xl text-center px-4">
          Parcourez les avis publics et aidez des personnes a retrouver leurs amis perdus de vue
        </p>
        <div class="h-1 w-16 bg-custom-green rounded mt-4" />
      </div>
    </div>

    <div class="max-w-6xl mx-auto px-4 py-8">
      <!-- Filtres -->
      <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-6 mb-8">
        <form @submit.prevent="soumettreRecherche" class="space-y-4">
          <!-- Barre de recherche -->
          <div class="relative">
            <font-awesome-icon
              :icon="['fas', 'magnifying-glass']"
              class="absolute left-4 top-1/2 -translate-y-1/2 text-gray-400"
            />
            <input
              v-model="recherche"
              type="text"
              placeholder="Rechercher par nom, prenom, ecole, ville..."
              class="w-full pl-11 pr-4 py-3 rounded-lg border border-gray-300 text-sm text-gray-800 placeholder:text-gray-400 focus:border-amber-500 focus:ring-2 focus:ring-amber-500/20 focus:outline-none transition-colors"
            />
          </div>

          <!-- Filtres avances -->
          <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
            <!-- Pays -->
            <div>
              <label for="filtre-pays" class="block text-xs text-gray-500 uppercase tracking-wide font-medium mb-1">
                Pays
              </label>
              <select
                id="filtre-pays"
                v-model="paysId"
                class="w-full rounded-lg border border-gray-300 px-3 py-2.5 text-sm text-gray-800 focus:border-amber-500 focus:ring-2 focus:ring-amber-500/20 focus:outline-none transition-colors bg-white"
              >
                <option value="">
                  Tous les pays
                </option>
                <option v-for="p in paysListe" :key="p.id" :value="p.id">
                  {{ p.nom }}
                </option>
              </select>
            </div>

            <!-- Ville -->
            <div>
              <label for="filtre-ville" class="block text-xs text-gray-500 uppercase tracking-wide font-medium mb-1">
                Ville
              </label>
              <input
                id="filtre-ville"
                v-model="ville"
                type="text"
                placeholder="Ville..."
                class="w-full rounded-lg border border-gray-300 px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-amber-500 focus:ring-2 focus:ring-amber-500/20 focus:outline-none transition-colors"
              />
            </div>

            <!-- Ecole -->
            <div>
              <label for="filtre-ecole" class="block text-xs text-gray-500 uppercase tracking-wide font-medium mb-1">
                Ecole
              </label>
              <input
                id="filtre-ecole"
                v-model="ecole"
                type="text"
                placeholder="Ecole..."
                class="w-full rounded-lg border border-gray-300 px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-amber-500 focus:ring-2 focus:ring-amber-500/20 focus:outline-none transition-colors"
              />
            </div>

            <!-- Tri -->
            <div>
              <label for="filtre-tri" class="block text-xs text-gray-500 uppercase tracking-wide font-medium mb-1">
                Trier par
              </label>
              <select
                id="filtre-tri"
                v-model="tri"
                class="w-full rounded-lg border border-gray-300 px-3 py-2.5 text-sm text-gray-800 focus:border-amber-500 focus:ring-2 focus:ring-amber-500/20 focus:outline-none transition-colors bg-white"
              >
                <option value="created_at">
                  Plus recents
                </option>
                <option value="compteur_partages">
                  Plus partages
                </option>
              </select>
            </div>
          </div>

          <!-- Boutons -->
          <div class="flex items-center gap-3">
            <button
              type="submit"
              class="px-6 py-2.5 bg-amber-700 text-white text-sm font-medium rounded-lg hover:bg-amber-800 transition-colors flex items-center gap-2"
              :disabled="chargement"
            >
              <font-awesome-icon v-if="chargement" :icon="['fas', 'spinner']" class="animate-spin" />
              <font-awesome-icon v-else :icon="['fas', 'magnifying-glass']" />
              Rechercher
            </button>
            <button
              v-if="aFiltresActifs"
              type="button"
              class="px-4 py-2.5 text-sm text-gray-600 hover:text-gray-800 transition-colors"
              @click="reinitialiser"
            >
              Reinitialiser
            </button>
          </div>
        </form>
      </div>

      <!-- Resultats -->
      <div v-if="chargement" class="text-center py-12">
        <font-awesome-icon :icon="['fas', 'spinner']" class="animate-spin text-2xl text-amber-700" />
        <p class="text-gray-500 mt-3 text-sm">Chargement des avis...</p>
      </div>

      <div v-else-if="erreur" class="bg-red-50 border border-red-200 rounded-lg p-6 text-center">
        <font-awesome-icon :icon="['fas', 'circle-exclamation']" class="text-red-500 text-xl mb-2" />
        <p class="text-red-700 text-sm">{{ erreur }}</p>
      </div>

      <div v-else-if="avis.length === 0" class="bg-white rounded-xl shadow-sm border border-gray-200 p-12 text-center">
        <div class="w-16 h-16 mx-auto mb-4 bg-gray-100 text-gray-400 rounded-full flex items-center justify-center">
          <font-awesome-icon :icon="['fas', 'magnifying-glass']" class="text-xl" />
        </div>
        <h2 class="text-lg font-semibold text-gray-700 mb-2">
          Aucun avis ne correspond
        </h2>
        <p class="text-gray-500 text-sm">
          Essayez de modifier vos criteres de recherche ou de reinitialiser les filtres.
        </p>
      </div>

      <template v-else>
        <!-- Compteur resultats -->
        <div class="flex items-center justify-between mb-4">
          <p class="text-sm text-gray-500">
            {{ pagination.total }} avis{{ pagination.total !== 1 ? '' : '' }} trouve{{ pagination.total !== 1 ? 's' : '' }}
          </p>
        </div>

        <!-- Grille de cartes -->
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 mb-8">
          <RetrouvAmisCarteAvisPublic
            v-for="a in avis"
            :key="a.slug"
            :avis="a"
          />
        </div>

        <!-- Pagination -->
        <div v-if="pagination.pages > 1" class="flex items-center justify-center gap-2">
          <button
            type="button"
            class="px-3 py-2 text-sm rounded-lg border transition-colors"
            :class="pagination.page === 1
              ? 'border-gray-200 text-gray-400 cursor-not-allowed'
              : 'border-gray-300 text-gray-700 hover:bg-gray-50'"
            :disabled="pagination.page === 1"
            @click="allerPage(pagination.page - 1)"
          >
            <font-awesome-icon :icon="['fas', 'chevron-left']" />
          </button>

          <button
            v-for="p in pagesVisibles"
            :key="p"
            type="button"
            class="px-3.5 py-2 text-sm rounded-lg border transition-colors"
            :class="p === pagination.page
              ? 'bg-amber-700 text-white border-amber-700'
              : 'border-gray-300 text-gray-700 hover:bg-gray-50'"
            @click="allerPage(p)"
          >
            {{ p }}
          </button>

          <button
            type="button"
            class="px-3 py-2 text-sm rounded-lg border transition-colors"
            :class="pagination.page === pagination.pages
              ? 'border-gray-200 text-gray-400 cursor-not-allowed'
              : 'border-gray-300 text-gray-700 hover:bg-gray-50'"
            :disabled="pagination.page === pagination.pages"
            @click="allerPage(pagination.page + 1)"
          >
            <font-awesome-icon :icon="['fas', 'chevron-right']" />
          </button>
        </div>
      </template>
    </div>
  </div>
</template>
