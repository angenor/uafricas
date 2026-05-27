<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section (compact, titre ↔ description au survol) -->
    <div
      class="group relative bg-cover bg-center z-0"
      style="background-image: url('https://images.unsplash.com/photo-1516026672322-bc52d61a55d5?ixlib=rb-1.2.1&auto=format&fit=crop&w=1900&q=80');">
      <div class="absolute inset-0 bg-gradient-to-r from-custom-green/90 to-yellow-600/70"></div>

      <div class="relative max-w-4xl mx-auto px-4 pt-16 pb-6 text-center select-none">
        <!-- Conteneur fixe : le titre et la description se superposent (crossfade au survol) -->
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12">
          <h1 class="absolute inset-0 flex items-center justify-center text-white text-2xl md:text-4xl font-bold transition-opacity duration-300 group-hover:opacity-0">
            Afripulse — Promouvons notre Afrique
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Rendre visible pour chaque territoire africain les opportunités touristiques, d'investissement et toute autre opportunité pertinente.
          </p>
        </div>
      </div>
    </div>

    <!-- Contenu principal -->
    <div class="max-w-6xl mx-auto px-4 relative mt-6">
      <!-- Header avec breadcrumb -->
      <div class="bg-white shadow-xs rounded-t-lg">
        <div class="px-4 py-6">
          <CommonBreadcrumbNav class="mb-4" :custom-breadcrumbs="[{ label: 'Opportunités en Afrique', to: undefined }]" />

          <div class="flex items-center justify-between">
            <div>
              <p class="text-gray-600">
                Explorez les territoires africains et leurs richesses
              </p>
            </div>
            <div class="flex items-center gap-3">
              <!-- Toggle grille / carte -->
              <div class="flex items-center bg-gray-100 rounded-lg p-1">
                <button
                  @click="viewMode = 'grille'"
                  class="p-2 rounded-md transition-colors"
                  :class="viewMode === 'grille' ? 'bg-custom-green text-white shadow-sm' : 'text-gray-500 hover:text-gray-700'"
                  title="Vue grille"
                >
                  <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z" />
                  </svg>
                </button>
                <button
                  @click="viewMode = 'carte'"
                  class="p-2 rounded-md transition-colors"
                  :class="viewMode === 'carte' ? 'bg-custom-green text-white shadow-sm' : 'text-gray-500 hover:text-gray-700'"
                  title="Vue carte"
                >
                  <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                  </svg>
                </button>
              </div>
              <NuxtLink to="/africa-culture"
                        class="text-custom-chocolat hover:text-custom-green font-medium hidden sm:inline">
                &#8592; AfricaCulture
              </NuxtLink>
            </div>
          </div>
        </div>
      </div>

      <div class="max-w-7xl mx-auto px-4 py-8">
        <div class="grid grid-cols-1 lg:grid-cols-4 gap-8">
          <!-- Filtres -->
          <div class="lg:col-span-1">
            <div class="bg-white rounded-lg shadow-md p-6 sticky top-4">
              <h3 class="text-lg font-bold mb-4">Filtrer les territoires</h3>

              <!-- Recherche -->
              <div class="mb-4">
                <input v-model="searchTerm"
                       @input="onSearchInput"
                       type="text"
                       placeholder="Rechercher un territoire..."
                       class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-3 focus:ring-custom-green focus:border-custom-green">
              </div>

              <!-- Regions -->
              <h4 class="text-sm font-medium text-gray-700 mb-2">Region</h4>
              <div class="space-y-2">
                <label class="flex items-center">
                  <input type="radio" v-model="selectedRegion" value="" class="mr-2 text-custom-green focus:ring-3 focus:ring-custom-green">
                  <span>Toutes les regions</span>
                </label>
                <label v-for="region in regions" :key="region" class="flex items-center">
                  <input type="radio" v-model="selectedRegion" :value="region" class="mr-2 text-custom-green focus:ring-3 focus:ring-custom-green">
                  <span class="text-sm">{{ region }}</span>
                </label>
              </div>

              <!-- Legende carte -->
              <div v-if="viewMode === 'carte'" class="mt-6 pt-6 border-t border-gray-200">
                <h4 class="text-sm font-medium text-gray-700 mb-3">Legende</h4>
                <div class="space-y-2">
                  <div v-for="(color, region) in REGION_COLORS" :key="region" class="flex items-center gap-2">
                    <span class="w-3 h-3 rounded-full shrink-0" :style="{ backgroundColor: color }"></span>
                    <span class="text-xs text-gray-600">{{ region }}</span>
                  </div>
                  <div class="flex items-center gap-2">
                    <span class="w-3 h-3 rounded-full shrink-0 bg-amber-400"></span>
                    <span class="text-xs text-gray-600">Selectionne</span>
                  </div>
                  <div class="flex items-center gap-2">
                    <span class="w-3 h-3 rounded-full shrink-0 bg-gray-200"></span>
                    <span class="text-xs text-gray-600">Fiche non disponible</span>
                  </div>
                </div>
              </div>

              <!-- Stats -->
              <div class="mt-6 pt-6 border-t border-gray-200">
                <div class="text-center">
                  <span class="text-2xl font-bold text-custom-green">{{ totalPays }}</span>
                  <span class="text-gray-600 ml-1">territoires</span>
                </div>
              </div>
            </div>
          </div>

          <!-- Liste des pays / Carte -->
          <div class="lg:col-span-3">
            <!-- Loading state -->
            <div v-if="chargement" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
              <div v-for="i in 6" :key="i" class="bg-white rounded-lg shadow-md overflow-hidden animate-pulse">
                <div class="h-40 bg-gray-200"></div>
                <div class="p-4">
                  <div class="h-4 bg-gray-200 rounded w-3/4 mb-2"></div>
                  <div class="h-3 bg-gray-200 rounded w-1/2 mb-4"></div>
                  <div class="h-3 bg-gray-200 rounded w-2/3"></div>
                </div>
              </div>
            </div>

            <!-- Empty state (mode grille uniquement) -->
            <div v-else-if="viewMode === 'grille' && paysList.length === 0" class="text-center py-12">
              <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
              </svg>
              <p class="mt-4 text-gray-600">Aucun territoire ne correspond a vos criteres</p>
              <button @click="resetFilters" class="mt-4 text-custom-green hover:underline">
                Reinitialiser les filtres
              </button>
            </div>

            <!-- Grid des pays -->
            <div v-else-if="viewMode === 'grille'" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
              <div v-for="pays in paysList" :key="pays.id"
                   @click="navigateToDetail(pays.id)"
                   class="bg-white rounded-lg shadow-md hover:shadow-lg transition-all duration-300 overflow-hidden cursor-pointer group">
                <!-- Image de couverture -->
                <div class="relative h-40 overflow-hidden">
                  <img :src="pays.image_couverture || ''"
                       :alt="pays.nom"
                       class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300">
                  <div class="absolute top-2 right-2">
                    <span class="px-2 py-1 bg-custom-green/90 text-white text-xs font-medium rounded-full">
                      {{ pays.region }}
                    </span>
                  </div>
                  <!-- Drapeau -->
                  <div class="absolute bottom-2 left-2">
                    <img :src="pays.drapeau_url || ''"
                         :alt="'Drapeau ' + pays.nom"
                         class="h-8 w-auto rounded shadow-md">
                  </div>
                </div>

                <!-- Contenu -->
                <div class="p-4">
                  <h3 class="text-lg font-bold text-gray-900 mb-1 group-hover:text-custom-green transition-colors">
                    {{ pays.nom }}
                  </h3>
                  <p v-if="pays.slogan" class="text-sm text-gray-500 italic mb-3 line-clamp-1">
                    {{ pays.slogan }}
                  </p>

                  <!-- Infos principales -->
                  <div class="space-y-1 text-sm text-gray-600">
                    <div class="flex items-center">
                      <svg class="w-4 h-4 mr-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"></path>
                      </svg>
                      <span>{{ pays.capitale }}</span>
                    </div>
                    <div class="flex items-center">
                      <svg class="w-4 h-4 mr-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"></path>
                      </svg>
                      <span>{{ pays.population }}</span>
                    </div>
                  </div>

                  <!-- Footer de la carte -->
                  <div class="mt-4 pt-3 border-t border-gray-100 flex items-center justify-between text-xs text-gray-500">
                    <span>{{ pays.nombre_contributions }} contributions</span>
                    <span>{{ formatDateShort(pays.updated_at) }}</span>
                  </div>
                </div>
              </div>
            </div>

            <!-- Mode carte -->
            <div v-else class="flex flex-col gap-6">
              <div class="flex flex-col lg:flex-row gap-6">
                <!-- Carte SVG d'Afrique -->
                <div class="flex-1 bg-white rounded-lg shadow-md overflow-hidden">
                  <div class="map-container relative p-2 sm:p-4" @mousemove="handleMapMouseMove">
                    <svg
                      :viewBox="AFRICA_VIEWBOX"
                      class="africa-map w-full h-auto"
                      xmlns="http://www.w3.org/2000/svg"
                    >
                      <path
                        v-for="location in africaLocations"
                        :key="location.id"
                        :d="location.path"
                        :fill="getMapColor(location.id)"
                        stroke="#fff"
                        stroke-width="0.5"
                        class="map-path"
                        :class="{ 'cursor-pointer': getFicheByCode(location.id) }"
                        @mouseenter="hoveredCountry = location"
                        @mouseleave="hoveredCountry = null"
                        @click="handleMapClick(location)"
                      />
                    </svg>

                    <!-- Tooltip -->
                    <Transition name="map-fade">
                      <div
                        v-if="hoveredCountry"
                        class="map-tooltip"
                        :class="{ 'map-tooltip-clickable': getFicheByCode(hoveredCountry.id) }"
                        :style="{ left: mousePos.x + 15 + 'px', top: mousePos.y - 10 + 'px' }"
                      >
                        <template v-if="getFicheByCode(hoveredCountry.id)">
                          <span class="font-semibold">{{ nomsPaysFr[hoveredCountry.id] || hoveredCountry.name }}</span>
                          <span class="text-xs opacity-70">Cliquer pour voir</span>
                        </template>
                        <template v-else>
                          {{ nomsPaysFr[hoveredCountry.id] || hoveredCountry.name }}
                        </template>
                      </div>
                    </Transition>
                  </div>
                </div>

                <!-- Fiche pays selectionne -->
                <Transition name="slide-in" mode="out-in">
                  <div v-if="selectedMapPays" :key="selectedMapPays.id" class="lg:w-80 shrink-0">
                    <div class="bg-white rounded-lg shadow-md overflow-hidden">
                      <!-- Image -->
                      <div class="relative h-44 overflow-hidden">
                        <img
                          :src="selectedMapPays.image_couverture || ''"
                          :alt="selectedMapPays.nom"
                          class="w-full h-full object-cover"
                        >
                        <div class="absolute inset-0 bg-linear-to-t from-black/50 via-transparent to-transparent"></div>
                        <div class="absolute top-2 right-2">
                          <span class="px-2 py-1 text-white text-xs font-medium rounded-full"
                                :style="{ backgroundColor: REGION_COLORS[selectedMapPays.region] || '#228B22' }">
                            {{ selectedMapPays.region }}
                          </span>
                        </div>
                        <div class="absolute bottom-2 left-2">
                          <img
                            :src="selectedMapPays.drapeau_url || ''"
                            :alt="'Drapeau ' + selectedMapPays.nom"
                            class="h-8 w-auto rounded shadow-md"
                          >
                        </div>
                      </div>

                      <!-- Contenu -->
                      <div class="p-4">
                        <h3 class="text-lg font-bold text-gray-900 mb-2">{{ selectedMapPays.nom }}</h3>
                        <p v-if="selectedMapPays.slogan" class="text-sm text-gray-500 italic mb-3">
                          {{ selectedMapPays.slogan }}
                        </p>

                        <div class="space-y-2 text-sm text-gray-600 mb-4">
                          <div v-if="selectedMapPays.capitale" class="flex items-center">
                            <svg class="w-4 h-4 mr-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4" />
                            </svg>
                            <span>{{ selectedMapPays.capitale }}</span>
                          </div>
                          <div v-if="selectedMapPays.population" class="flex items-center">
                            <svg class="w-4 h-4 mr-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
                            </svg>
                            <span>{{ selectedMapPays.population }}</span>
                          </div>
                        </div>

                        <div class="mt-3 pt-3 border-t border-gray-100 text-xs text-gray-500 mb-4">
                          {{ selectedMapPays.nombre_contributions }} contributions
                        </div>

                        <button
                          @click="navigateFromMap"
                          class="w-full px-4 py-2.5 bg-custom-green text-white font-medium rounded-lg hover:bg-green-700 transition-colors"
                        >
                          Voir la fiche territoire
                        </button>
                      </div>
                    </div>
                  </div>
                </Transition>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  useOpportuniteAfrique,
  formatDateShort,
  type FichePaysAPI,
} from '~/composables/useOpportuniteAfrique'
import World from '@svg-maps/world'
import { PAYS_AFRICAINS_ISO2 } from '~/constants/afripulsePaysAutorises'

useHead({
  title: 'Afripulse - Opportunités en Afrique | UAfricas',
  meta: [
    { name: 'description', content: 'Decouvrez les fiches territoires africains et leurs opportunites economiques, culturelles et sociales.' }
  ]
})

const { chargement, listerFiches, listerRegions } = useOpportuniteAfrique()

const paysList = ref<FichePaysAPI[]>([])
const searchTerm = ref('')
const selectedRegion = ref('')
const regions = ref<string[]>([])
const totalPays = ref(0)
const viewMode = ref<'grille' | 'carte'>('grille')

// Debounce search
let searchTimeout: ReturnType<typeof setTimeout> | null = null
const onSearchInput = () => {
  if (searchTimeout) clearTimeout(searchTimeout)
  searchTimeout = setTimeout(() => {
    chargerFiches()
  }, 500)
}

// Charger les fiches depuis l'API
const chargerFiches = async () => {
  const result = await listerFiches({
    recherche: searchTerm.value || undefined,
    region: selectedRegion.value || undefined,
    par_page: 60,
  })

  if (result) {
    paysList.value = result.fiches
    totalPays.value = result.total
  }
}

// Watch sur la region selectionnee
watch(selectedRegion, () => {
  chargerFiches()
})

// Navigation vers le detail
const navigateToDetail = (id: string) => {
  navigateTo(`/opportunite-afrique/${id}`)
}

// Reinitialiser les filtres
const resetFilters = () => {
  searchTerm.value = ''
  selectedRegion.value = ''
  chargerFiches()
}

// === MODE CARTE ===

// Codes ISO des pays africains (source unique partagée avec le backend)
const PAYS_AFRICAINS = new Set<string>(PAYS_AFRICAINS_ISO2)

// Noms francais des pays africains
const nomsPaysFr: Record<string, string> = {
  dz: 'Algerie', ao: 'Angola', bj: 'Benin', bw: 'Botswana', bf: 'Burkina Faso',
  bi: 'Burundi', cv: 'Cap-Vert', cm: 'Cameroun', cf: 'Centrafrique',
  td: 'Tchad', km: 'Comores', cg: 'Congo', cd: 'RD Congo', ci: "Cote d'Ivoire",
  dj: 'Djibouti', eg: 'Egypte', gq: 'Guinee equatoriale', er: 'Erythree',
  sz: 'Eswatini', et: 'Ethiopie', ga: 'Gabon', gm: 'Gambie', gh: 'Ghana',
  gn: 'Guinee', gw: 'Guinee-Bissau', ke: 'Kenya', ls: 'Lesotho', lr: 'Liberia',
  ly: 'Libye', mg: 'Madagascar', mw: 'Malawi', ml: 'Mali', mr: 'Mauritanie',
  mu: 'Maurice', ma: 'Maroc', mz: 'Mozambique', na: 'Namibie', ne: 'Niger',
  ng: 'Nigeria', rw: 'Rwanda', st: 'Sao Tome-et-Principe', sn: 'Senegal',
  sc: 'Seychelles', sl: 'Sierra Leone', so: 'Somalie', za: 'Afrique du Sud',
  ss: 'Soudan du Sud', sd: 'Soudan', tz: 'Tanzanie', tg: 'Togo', tn: 'Tunisie',
  ug: 'Ouganda', zm: 'Zambie', zw: 'Zimbabwe', eh: 'Sahara occidental',
}

// ViewBox pour afficher uniquement l'Afrique
const AFRICA_VIEWBOX = '375 290 275 325'

// Couleurs par region
const REGION_COLORS: Record<string, string> = {
  "Afrique de l'Ouest": '#228B22',
  'Afrique Centrale': '#0E7C4A',
  "Afrique de l'Est": '#1565C0',
  'Afrique du Nord': '#E65100',
  'Afrique Australe': '#7B1FA2',
}

// Locations africaines filtrees depuis la carte du monde
const africaLocations = computed(() => {
  return World.locations.filter(loc => PAYS_AFRICAINS.has(loc.id.toLowerCase()))
})

// Etat du survol
const hoveredCountry = ref<{ id: string; name: string } | null>(null)
const mousePos = ref({ x: 0, y: 0 })

// Pays selectionne sur la carte
const selectedMapPays = ref<FichePaysAPI | null>(null)

// Trouver la fiche d'un pays par code ISO
const getFicheByCode = (isoCode: string): FichePaysAPI | undefined => {
  return paysList.value.find(p => p.code?.toLowerCase() === isoCode.toLowerCase())
}

// Ajuster la luminosite d'une couleur hex
const adjustBrightness = (hex: string, percent: number): string => {
  const num = parseInt(hex.replace('#', ''), 16)
  const amt = Math.round(2.55 * percent)
  const R = Math.min(255, Math.max(0, (num >> 16) + amt))
  const G = Math.min(255, Math.max(0, ((num >> 8) & 0x00ff) + amt))
  const B = Math.min(255, Math.max(0, (num & 0x0000ff) + amt))
  return `#${((1 << 24) + (R << 16) + (G << 8) + B).toString(16).slice(1)}`
}

// Couleur d'un pays sur la carte
const getMapColor = (id: string): string => {
  const isHovered = hoveredCountry.value?.id === id
  const isSelected = selectedMapPays.value?.code?.toLowerCase() === id
  const fiche = getFicheByCode(id)

  if (fiche) {
    const regionColor = REGION_COLORS[fiche.region] || '#228B22'
    if (isSelected) return '#FFD700'
    if (isHovered) return adjustBrightness(regionColor, -15)
    return regionColor
  }

  if (isHovered) return '#bdbdbd'
  return '#e5e7eb'
}

// Gestion du survol de la carte
const handleMapMouseMove = (event: MouseEvent) => {
  const container = event.currentTarget as HTMLElement
  const rect = container.getBoundingClientRect()
  mousePos.value = {
    x: event.clientX - rect.left,
    y: event.clientY - rect.top,
  }
}

// Clic sur un pays de la carte
const handleMapClick = (location: { id: string }) => {
  const fiche = getFicheByCode(location.id)
  if (fiche) {
    selectedMapPays.value = fiche
  }
}

// Naviguer vers le detail depuis la carte
const navigateFromMap = () => {
  if (selectedMapPays.value) {
    navigateToDetail(selectedMapPays.value.id)
  }
}

// Charger les donnees au montage
onMounted(async () => {
  const regionsResult = await listerRegions()
  if (regionsResult) {
    regions.value = regionsResult
  }
  await chargerFiches()
})
</script>

<style scoped>
.line-clamp-1 {
  display: -webkit-box;
  -webkit-line-clamp: 1;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

/* Carte SVG */
.map-container {
  position: relative;
  width: 100%;
}

.africa-map {
  width: 100%;
  height: auto;
  max-height: 600px;
}

.map-path {
  transition: fill 0.2s ease, opacity 0.2s ease;
}

.map-path:hover {
  opacity: 0.85;
}

/* Tooltip carte */
.map-tooltip {
  position: absolute;
  background: rgba(0, 0, 0, 0.85);
  color: #fff;
  padding: 8px 14px;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  pointer-events: none;
  z-index: 50;
  white-space: nowrap;
  transform: translateY(-50%);
}

.map-tooltip-clickable {
  pointer-events: auto;
  cursor: pointer;
  display: flex;
  flex-direction: column;
  gap: 2px;
  padding: 10px 16px;
  transition: all 0.2s ease;
}

.map-tooltip-clickable:hover {
  background: rgba(34, 139, 34, 0.95);
  transform: translateY(-50%) scale(1.05);
}

/* Transitions */
.map-fade-enter-active,
.map-fade-leave-active {
  transition: opacity 0.2s ease;
}

.map-fade-enter-from,
.map-fade-leave-to {
  opacity: 0;
}

.slide-in-enter-active {
  transition: all 0.3s ease-out;
}

.slide-in-leave-active {
  transition: all 0.2s ease-in;
}

.slide-in-enter-from {
  opacity: 0;
  transform: translateX(30px);
}

.slide-in-leave-to {
  opacity: 0;
  transform: translateX(30px);
}
</style>
