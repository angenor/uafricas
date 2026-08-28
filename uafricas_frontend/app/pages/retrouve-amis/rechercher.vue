<script setup lang="ts">
import type { AvisPublicResume, PaysInfo, RecherchePubliqueParams, PaginationInfo } from '~/composables/useRetrouvAmis'

definePageMeta({ layout: false })

const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string
const { rechercherAvisPublics, chargement, erreur } = useRetrouvAmis()

// SEO
useSeoMeta({
  title: 'Rechercher un avis de recherche | AfricanS',
  description: 'Parcourez les avis de recherche publics sur AfricanS. Aidez les gens a retrouver des amis perdus de vue en Afrique et dans le monde.',
  ogTitle: 'Rechercher un avis de recherche | AfricanS',
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
  <NuxtLayout name="africans">
    <template #bandeau>
      <!-- L'image était hotlinkée sur Unsplash ; le hero local du module
           existait déjà. -->
      <AfricansBandeauModule
        titre="Rechercher un avis"
        sous-titre="Parcourez les avis publics et aidez des personnes à retrouver leurs amis perdus de vue."
        image="/images/africans/heros/hero-africonnect.jpg"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[
          { libelle: 'Opafrica', vers: '/retrouve-amis' },
          { libelle: 'Africonnect', vers: '/retrouve-amis' },
          { libelle: 'Rechercher' },
        ]"
      />
    </template>

    <div class="min-w-0">
      <!-- Filtres -->
      <div class="bg-white rounded-lg shadow-sm border border-af-bordure p-3 mb-8">
        <form @submit.prevent="soumettreRecherche" class="space-y-3">
          <!-- Barre de recherche -->
          <div class="relative">
            <font-awesome-icon
              :icon="['fas', 'magnifying-glass']"
              class="absolute left-3 top-1/2 -translate-y-1/2 text-af-atone-2 text-sm"
            />
            <input
              v-model="recherche"
              type="text"
              placeholder="Rechercher par nom, prenom, ecole, ville..."
              class="w-full pl-9 pr-3 py-2 rounded-lg border border-af-bordure text-sm text-af-encre placeholder:text-af-atone-2 focus:border-af-chocolat focus:ring-2 focus:border-af-chocolat/20 focus:outline-none transition-colors"
            />
          </div>

          <!-- Filtres avances -->
          <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3">
            <!-- Pays -->
            <div>
              <label for="filtre-pays" class="block text-xs text-af-atone uppercase tracking-wide font-medium mb-1">
                Territoire
              </label>
              <select
                id="filtre-pays"
                v-model="paysId"
                class="w-full rounded-lg border border-af-bordure px-3 py-2 text-sm text-af-encre focus:border-af-chocolat focus:ring-2 focus:border-af-chocolat/20 focus:outline-none transition-colors bg-white"
              >
                <option value="">
                  Tous les territoires
                </option>
                <option v-for="p in paysListe" :key="p.id" :value="p.id">
                  {{ p.nom }}
                </option>
              </select>
            </div>

            <!-- Ville -->
            <div>
              <label for="filtre-ville" class="block text-xs text-af-atone uppercase tracking-wide font-medium mb-1">
                Ville
              </label>
              <input
                id="filtre-ville"
                v-model="ville"
                type="text"
                placeholder="Ville..."
                class="w-full rounded-lg border border-af-bordure px-3 py-2 text-sm text-af-encre placeholder:text-af-atone-2 focus:border-af-chocolat focus:ring-2 focus:border-af-chocolat/20 focus:outline-none transition-colors"
              />
            </div>

            <!-- Ecole -->
            <div>
              <label for="filtre-ecole" class="block text-xs text-af-atone uppercase tracking-wide font-medium mb-1">
                École
              </label>
              <input
                id="filtre-ecole"
                v-model="ecole"
                type="text"
                placeholder="École..."
                class="w-full rounded-lg border border-af-bordure px-3 py-2 text-sm text-af-encre placeholder:text-af-atone-2 focus:border-af-chocolat focus:ring-2 focus:border-af-chocolat/20 focus:outline-none transition-colors"
              />
            </div>

            <!-- Tri -->
            <div>
              <label for="filtre-tri" class="block text-xs text-af-atone uppercase tracking-wide font-medium mb-1">
                Trier par
              </label>
              <select
                id="filtre-tri"
                v-model="tri"
                class="w-full rounded-lg border border-af-bordure px-3 py-2 text-sm text-af-encre focus:border-af-chocolat focus:ring-2 focus:border-af-chocolat/20 focus:outline-none transition-colors bg-white"
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
              class="px-5 py-2 bg-af-chocolat text-white text-sm font-medium rounded-lg hover:opacity-90 transition-colors flex items-center gap-2"
              :disabled="chargement"
            >
              <font-awesome-icon v-if="chargement" :icon="['fas', 'spinner']" class="animate-spin" />
              <font-awesome-icon v-else :icon="['fas', 'magnifying-glass']" />
              Rechercher
            </button>
            <button
              v-if="aFiltresActifs"
              type="button"
              class="px-4 py-2 text-sm text-af-corps hover:text-af-encre transition-colors"
              @click="reinitialiser"
            >
              Réinitialiser
            </button>
          </div>
        </form>
      </div>

      <!-- Resultats -->
      <div v-if="chargement" class="text-center py-12">
        <font-awesome-icon :icon="['fas', 'spinner']" class="animate-spin text-2xl text-af-chocolat" />
        <p class="text-af-atone mt-3 text-sm">Chargement des avis...</p>
      </div>

      <div v-else-if="erreur" class="bg-af-live/5 border border-af-live/30 rounded-lg p-6 text-center">
        <font-awesome-icon :icon="['fas', 'circle-exclamation']" class="text-af-live text-xl mb-2" />
        <p class="text-af-live text-sm">{{ erreur }}</p>
      </div>

      <div v-else-if="avis.length === 0" class="bg-white rounded-lg shadow-sm border border-af-bordure p-12 text-center">
        <div class="w-16 h-16 mx-auto mb-4 bg-af-fond text-af-atone-2 rounded-full flex items-center justify-center">
          <font-awesome-icon :icon="['fas', 'magnifying-glass']" class="text-xl" />
        </div>
        <h2 class="text-lg font-semibold text-af-corps mb-2">
          Aucun avis ne correspond
        </h2>
        <p class="text-af-atone text-sm">
          Essayez de modifier vos criteres de recherche ou de reinitialiser les filtres.
        </p>
      </div>

      <template v-else>
        <!-- Compteur resultats -->
        <div class="flex items-center justify-between mb-4">
          <p class="text-sm text-af-atone">
            {{ pagination.total }} avis{{ pagination.total !== 1 ? '' : '' }} trouve{{ pagination.total !== 1 ? 's' : '' }}
          </p>
        </div>

        <!-- Grille de cartes -->
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 mb-8">
          <RetrouveAmisCarteAvisPublic
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
              ? 'border-af-bordure text-af-atone-2 cursor-not-allowed'
              : 'border-af-bordure text-af-corps hover:bg-af-fond'"
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
              ? 'bg-af-chocolat text-white border-af-chocolat'
              : 'border-af-bordure text-af-corps hover:bg-af-fond'"
            @click="allerPage(p)"
          >
            {{ p }}
          </button>

          <button
            type="button"
            class="px-3 py-2 text-sm rounded-lg border transition-colors"
            :class="pagination.page === pagination.pages
              ? 'border-af-bordure text-af-atone-2 cursor-not-allowed'
              : 'border-af-bordure text-af-corps hover:bg-af-fond'"
            :disabled="pagination.page === pagination.pages"
            @click="allerPage(pagination.page + 1)"
          >
            <font-awesome-icon :icon="['fas', 'chevron-right']" />
          </button>
        </div>
      </template>
    </div>
  </NuxtLayout>
</template>
