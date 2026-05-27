<template>
  <div class="min-h-screen bg-linear-to-br from-gray-50 via-white to-gray-50 pt-28 pb-16">
    <div class="max-w-7xl mx-auto px-4">

      <!-- En-tête -->
      <div class="text-center mb-10" data-aos="fade-up">
        <h1 class="text-3xl sm:text-4xl font-bold text-gray-800 font-display">
          Annuaire des membres
        </h1>
        <p class="text-gray-500 mt-3 max-w-2xl mx-auto">
          Découvrez les membres de la communauté UAfricas : experts, bibliothèques humaines et passionnés du développement durable africain.
        </p>
        <p v-if="total > 0" class="text-sm text-custom-chocolat font-medium mt-2">
          {{ total }} membre{{ total > 1 ? 's' : '' }} inscrit{{ total > 1 ? 's' : '' }}
        </p>
      </div>

      <!-- Barre de recherche + filtres -->
      <div class="bg-white rounded-2xl shadow-lg p-5 mb-8" data-aos="fade-up" data-aos-delay="100">
        <div class="flex flex-col lg:flex-row gap-4 lg:items-center lg:justify-between">
          <!-- Recherche -->
          <div class="relative flex-1 max-w-md">
            <font-awesome-icon
              icon="fa-solid fa-magnifying-glass"
              class="absolute left-4 top-1/2 -translate-y-1/2 text-gray-400 text-sm"
            />
            <input
              v-model="recherche"
              type="text"
              placeholder="Rechercher par nom, fonction, ville..."
              class="w-full pl-11 pr-4 py-3 border border-gray-200 rounded-xl text-sm bg-gray-50 focus:bg-white focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all"
              @keyup.enter="appliquerRecherche"
            />
          </div>

          <!-- Filtres par type -->
          <div class="flex flex-wrap gap-2">
            <button
              v-for="t in typesMembres"
              :key="t.value"
              class="px-4 py-2 rounded-full text-sm font-medium transition-all"
              :class="typeActif === t.value
                ? 'bg-linear-to-r from-custom-chocolat to-custom-green text-white shadow'
                : 'bg-gray-100 text-gray-600 hover:bg-gray-200'"
              @click="typeActif = t.value"
            >
              <font-awesome-icon :icon="t.icon" class="mr-1.5 text-xs" />
              {{ t.label }}
            </button>
          </div>
        </div>
      </div>

      <!-- Chargement -->
      <div v-if="chargement" class="flex justify-center py-24">
        <font-awesome-icon icon="fa-solid fa-spinner" class="text-4xl text-custom-chocolat animate-spin" />
      </div>

      <!-- Grille de membres -->
      <div
        v-else-if="membres.length > 0"
        class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6"
      >
        <NuxtLink
          v-for="membre in membres"
          :key="membre.id"
          :to="`/profil/${membre.id}`"
          class="group bg-white rounded-2xl shadow-md hover:shadow-xl overflow-hidden transition-all duration-300 hover:-translate-y-1"
        >
          <!-- Bandeau -->
          <div class="h-20 bg-linear-to-r from-custom-chocolat via-amber-700 to-custom-green relative"></div>

          <div class="px-5 pb-5 -mt-10">
            <!-- Avatar -->
            <div class="flex justify-center">
              <img
                v-if="photoComplete(membre.photoUrl)"
                :src="photoComplete(membre.photoUrl)!"
                :alt="`${membre.prenom} ${membre.nom}`"
                class="w-20 h-20 rounded-full object-cover border-4 border-white shadow-md"
              />
              <div
                v-else
                class="w-20 h-20 rounded-full bg-custom-chocolat text-white flex items-center justify-center text-2xl font-bold border-4 border-white shadow-md"
              >
                {{ initiaux(membre) }}
              </div>
            </div>

            <!-- Nom -->
            <div class="text-center mt-3">
              <h3 class="font-semibold text-gray-800 group-hover:text-custom-chocolat transition-colors truncate">
                {{ membre.prenom }} {{ membre.nom }}
              </h3>
              <p v-if="membre.fonction" class="text-sm text-gray-500 mt-0.5 truncate">
                {{ membre.fonction }}
              </p>
            </div>

            <!-- Badges -->
            <div class="flex flex-wrap justify-center gap-1.5 mt-3 min-h-6">
              <span
                v-if="membre.estExpert"
                class="inline-flex items-center gap-1 px-2.5 py-0.5 bg-emerald-100 text-emerald-700 text-xs font-medium rounded-full"
              >
                <font-awesome-icon icon="fa-solid fa-briefcase" class="text-[10px]" />
                Expert
              </span>
              <span
                v-if="membre.estBiblio"
                class="inline-flex items-center gap-1 px-2.5 py-0.5 bg-custom-chocolat/10 text-custom-chocolat text-xs font-medium rounded-full"
              >
                <font-awesome-icon icon="fa-solid fa-book-open" class="text-[10px]" />
                Bibliothèque
              </span>
            </div>

            <!-- Localisation -->
            <div
              v-if="membre.ville || membre.pays"
              class="flex items-center justify-center gap-1.5 text-xs text-gray-400 mt-3"
            >
              <font-awesome-icon icon="fa-solid fa-location-dot" />
              <span class="truncate">
                {{ [membre.ville, membre.pays].filter(Boolean).join(', ') }}
              </span>
            </div>

            <!-- Bouton d'amitié (membres connectés, masqué sur sa propre carte) -->
            <div
              v-if="estConnecte"
              class="flex justify-center mt-4"
              @click.stop.prevent
            >
              <SocialBoutonAmitie
                :utilisateur-id="membre.id"
                :etat="etats[membre.id] || 'aucune'"
                taille="sm"
                @update="(e) => etats[membre.id] = e"
              />
            </div>
          </div>
        </NuxtLink>
      </div>

      <!-- État vide -->
      <div v-else class="bg-white rounded-2xl shadow-lg p-12 text-center">
        <div class="w-20 h-20 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-5">
          <font-awesome-icon icon="fa-solid fa-users-slash" class="text-3xl text-gray-400" />
        </div>
        <h3 class="text-lg font-semibold text-gray-800 mb-2">Aucun membre trouvé</h3>
        <p class="text-gray-500 text-sm max-w-md mx-auto">
          Essayez de modifier votre recherche ou vos filtres.
        </p>
        <button
          class="mt-5 px-5 py-2.5 bg-linear-to-r from-custom-chocolat to-custom-green text-white text-sm rounded-xl font-medium hover:shadow-lg transition-all"
          @click="reinitialiser"
        >
          Réinitialiser
        </button>
      </div>

      <!-- Pagination -->
      <div v-if="!chargement && membres.length > 0 && totalPages > 1" class="flex justify-center mt-10">
        <nav class="flex items-center gap-2">
          <button
            :disabled="page === 1"
            class="p-2.5 rounded-xl bg-white border border-gray-200 text-gray-600 hover:bg-gray-50 transition-all disabled:opacity-40 disabled:cursor-not-allowed"
            @click="page--"
          >
            <font-awesome-icon icon="fa-solid fa-chevron-left" class="text-sm" />
          </button>

          <div class="flex gap-1">
            <button
              v-for="p in pagesVisibles"
              :key="p"
              class="w-10 h-10 rounded-xl font-medium text-sm transition-all"
              :class="p === page
                ? 'bg-linear-to-r from-custom-chocolat to-custom-green text-white'
                : 'bg-white border border-gray-200 text-gray-600 hover:bg-gray-50'"
              @click="page = p"
            >
              {{ p }}
            </button>
          </div>

          <button
            :disabled="page === totalPages"
            class="p-2.5 rounded-xl bg-white border border-gray-200 text-gray-600 hover:bg-gray-50 transition-all disabled:opacity-40 disabled:cursor-not-allowed"
            @click="page++"
          >
            <font-awesome-icon icon="fa-solid fa-chevron-right" class="text-sm" />
          </button>
        </nav>
      </div>

    </div>
  </div>
</template>

<script setup lang="ts">
import type { MembreAPI } from '~/composables/useMembres'
import type { EtatRelation } from '~/composables/useAmis'
import { useUserStore } from '~/stores/user'

useHead({
  title: 'Annuaire des membres - UAfricas',
  meta: [
    {
      name: 'description',
      content: 'Découvrez tous les membres inscrits sur la plateforme UAfricas.',
    },
  ],
})

useAOS()

const { listerMembres, chargement } = useMembres()
const { obtenirEtatsRelationLot } = useAmis()
const userStore = useUserStore()
const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string

const estConnecte = computed(() => userStore.isAuthenticated)

// ── État ──
const membres = ref<MembreAPI[]>([])
const etats = ref<Record<string, EtatRelation>>({})
const total = ref(0)
const totalPages = ref(1)
const page = ref(1)
const recherche = ref('')
const rechercheActive = ref('')
const typeActif = ref('')
const parPage = 12

const typesMembres = [
  { value: '', label: 'Tous', icon: 'fa-solid fa-users' },
  { value: 'expert', label: 'Experts', icon: 'fa-solid fa-briefcase' },
  { value: 'biblio', label: 'Bibliothèque', icon: 'fa-solid fa-book-open' },
]

// ── Helpers ──
const photoComplete = (url: string | null): string | null => {
  if (!url) return null
  return url.startsWith('http') ? url : `${apiBase}${url}`
}

const initiaux = (membre: MembreAPI): string => {
  return `${membre.prenom?.charAt(0)?.toUpperCase() || ''}${membre.nom?.charAt(0)?.toUpperCase() || ''}`
}

const pagesVisibles = computed(() => {
  const pages: number[] = []
  const t = totalPages.value
  const c = page.value
  if (t <= 5) {
    for (let i = 1; i <= t; i++) pages.push(i)
  } else if (c <= 3) {
    pages.push(1, 2, 3, 4, 5)
  } else if (c >= t - 2) {
    pages.push(t - 4, t - 3, t - 2, t - 1, t)
  } else {
    pages.push(c - 2, c - 1, c, c + 1, c + 2)
  }
  return pages.filter((p) => p >= 1 && p <= t)
})

// ── Chargement ──
const charger = async () => {
  const result = await listerMembres({
    recherche: rechercheActive.value || undefined,
    type: typeActif.value || undefined,
    page: page.value,
    par_page: parPage,
  })
  if (result) {
    membres.value = result.membres
    total.value = result.total
    totalPages.value = result.total_pages
    await chargerEtats()
  }
}

// Charger les états de relation des cartes visibles en un seul appel (anti N+1)
const chargerEtats = async () => {
  if (!estConnecte.value || membres.value.length === 0) {
    etats.value = {}
    return
  }
  const ids = membres.value
    .map(m => m.id)
    .filter(id => id !== userStore.user?.id)
  etats.value = await obtenirEtatsRelationLot(ids)
}

const appliquerRecherche = () => {
  rechercheActive.value = recherche.value.trim()
  page.value = 1
}

const reinitialiser = () => {
  recherche.value = ''
  rechercheActive.value = ''
  typeActif.value = ''
  page.value = 1
}

// Recharger quand filtres/recherche changent (reset page)
watch([rechercheActive, typeActif], () => {
  page.value = 1
  charger()
})

// Recharger quand la page change
watch(page, charger)

onMounted(charger)
</script>

<style scoped>
@reference "~/assets/css/main.css";

input:focus {
  box-shadow: 0 0 0 3px rgba(34, 139, 34, 0.1);
}
</style>
