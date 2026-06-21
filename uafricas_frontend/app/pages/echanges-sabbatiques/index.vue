<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section (compact, titre ↔ description au survol) -->
    <div class="group relative bg-cover bg-center">
      <div class="absolute inset-0 bg-linear-to-r from-custom-chocolat to-black/90"></div>

      <div class="relative max-w-4xl mx-auto px-4 pt-16 pb-6 text-center select-none">
        <!-- Conteneur fixe : le titre et la description se superposent (crossfade au survol) -->
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12">
          <h1 class="absolute inset-0 flex items-center justify-center text-white text-2xl md:text-4xl font-bold transition-opacity duration-300 group-hover:opacity-0">
            Échanges Sabbafrica
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Offrir des canaux aux entreprises et organisations en Afrique de bénéficier de l'expérience pointue de la diaspora dans des domaines clés de développement.
          </p>
        </div>
      </div>
    </div>

    <!-- Contenu principal -->
    <div class="max-w-7xl mx-auto px-4 relative mt-6">
      <!-- Header avec breadcrumb + boutons de proposition + toggle vue -->
      <div class="bg-white shadow-xs rounded-t-lg">
        <div class="px-4 py-6">
          <CommonBreadcrumbNav class="mb-4" :custom-breadcrumbs="[{ label: 'Échanges Sabbafrica' }]" />

          <div class="flex flex-col gap-4 lg:flex-row lg:items-center lg:justify-between">
            <!-- Boutons de proposition (en haut à gauche) -->
            <div class="flex flex-wrap items-center gap-3">
              <NuxtLink to="/echanges-sabbatiques/proposer?type=interafricain">
                <button
                  class="text-white whitespace-nowrap shadow-md hover:shadow-none h-10 rounded-full bg-custom-green px-4 hover:scale-105 transition-all flex items-center gap-2"
                >
                  <font-awesome-icon :icon="['fas', 'plus']" />
                  Proposer un échange interafricain
                </button>
              </NuxtLink>
              <NuxtLink to="/echanges-sabbatiques/proposer?type=hors_afrique">
                <button
                  class="text-custom-chocolat whitespace-nowrap shadow-md hover:shadow-none h-10 rounded-full border border-custom-chocolat px-4 hover:scale-105 transition-all flex items-center gap-2"
                >
                  <font-awesome-icon :icon="['fas', 'plus']" />
                  Proposer un échange hors Afrique
                </button>
              </NuxtLink>
            </div>

            <!-- Toggle grille / carte -->
            <div class="flex items-center bg-gray-100 rounded-lg p-1 self-start lg:self-auto">
              <button
                @click="viewMode = 'grille'"
                class="flex items-center gap-2 px-3 py-2 rounded-md text-sm font-medium transition-colors"
                :class="viewMode === 'grille' ? 'bg-custom-green text-white shadow-sm' : 'text-gray-500 hover:text-gray-700'"
                title="Vue grille"
              >
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z" />
                </svg>
                <span>Grille</span>
              </button>
              <button
                @click="viewMode = 'carte'"
                class="flex items-center gap-2 px-3 py-2 rounded-md text-sm font-medium transition-colors"
                :class="viewMode === 'carte' ? 'bg-custom-green text-white shadow-sm' : 'text-gray-500 hover:text-gray-700'"
                title="Vue carte"
              >
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                <span>Carte</span>
              </button>
            </div>
          </div>
        </div>
      </div>

      <div class="py-8">
        <div class="grid grid-cols-1 lg:grid-cols-4 gap-8">
          <!-- Filtres -->
          <div class="lg:col-span-1">
            <div class="bg-white rounded-lg shadow-md p-6 sticky top-4">
              <h3 class="text-lg font-bold mb-4">Filtrer les programmes</h3>

              <!-- Recherche -->
              <div class="mb-4">
                <input
                  v-model="filtres.recherche"
                  @input="onSearchInput"
                  type="text"
                  placeholder="Rechercher un programme..."
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-3 focus:ring-custom-green focus:border-custom-green"
                />
              </div>

              <!-- Type de programme -->
              <h4 class="text-sm font-medium text-gray-700 mb-2">Type de programme</h4>
              <div class="space-y-2 mb-6">
                <label v-for="type in TYPES_PROGRAMME" :key="type.value" class="flex items-center">
                  <input
                    type="radio"
                    v-model="filtres.type"
                    :value="type.value"
                    class="mr-2 text-custom-green focus:ring-3 focus:ring-custom-green"
                  />
                  <span class="text-sm">{{ type.label }}</span>
                </label>
              </div>

              <!-- Territoire -->
              <h4 class="text-sm font-medium text-gray-700 mb-2">Territoire</h4>
              <select
                v-model="filtres.pays"
                class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-3 focus:ring-custom-green focus:border-custom-green mb-6"
              >
                <option v-for="pays in PAYS_AFRICAINS" :key="pays.value" :value="pays.value">
                  {{ pays.label }}
                </option>
              </select>

              <!-- Domaine -->
              <h4 class="text-sm font-medium text-gray-700 mb-2">Domaine</h4>
              <select
                v-model="filtres.domaine"
                class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-3 focus:ring-custom-green focus:border-custom-green"
              >
                <option v-for="domaine in DOMAINES" :key="domaine.value" :value="domaine.value">
                  {{ domaine.label }}
                </option>
              </select>

              <!-- Légende carte -->
              <div v-if="viewMode === 'carte'" class="mt-6 pt-6 border-t border-gray-200">
                <h4 class="text-sm font-medium text-gray-700 mb-3">Légende</h4>
                <div class="space-y-2">
                  <div class="flex items-center gap-2">
                    <span class="w-3 h-3 rounded-full shrink-0 bg-custom-green"></span>
                    <span class="text-xs text-gray-600">Programmes disponibles</span>
                  </div>
                  <div class="flex items-center gap-2">
                    <span class="w-3 h-3 rounded-full shrink-0 bg-amber-400"></span>
                    <span class="text-xs text-gray-600">Sélectionné</span>
                  </div>
                  <div class="flex items-center gap-2">
                    <span class="w-3 h-3 rounded-full shrink-0 bg-gray-200"></span>
                    <span class="text-xs text-gray-600">Aucun programme</span>
                  </div>
                </div>
              </div>

              <!-- Réinitialiser -->
              <button
                v-if="filtresActifs"
                @click="reinitialiserFiltres"
                class="mt-4 w-full text-sm text-custom-green hover:underline"
              >
                Réinitialiser les filtres
              </button>

              <!-- Stats -->
              <div class="mt-6 pt-6 border-t border-gray-200">
                <div class="text-center">
                  <span class="text-2xl font-bold text-custom-green">{{ total }}</span>
                  <span class="text-gray-600 ml-1">programme{{ total > 1 ? 's' : '' }}</span>
                </div>
              </div>
            </div>
          </div>

          <!-- Liste / Carte -->
          <div class="lg:col-span-3">
            <!-- Chargement -->
            <div v-if="chargement" class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 gap-6">
              <div v-for="i in 6" :key="i" class="bg-white rounded-lg shadow-md overflow-hidden animate-pulse">
                <div class="h-40 bg-gray-200"></div>
                <div class="p-4">
                  <div class="h-3 bg-gray-200 rounded w-1/2 mb-2"></div>
                  <div class="h-4 bg-gray-200 rounded w-3/4 mb-4"></div>
                  <div class="h-3 bg-gray-200 rounded w-1/3"></div>
                </div>
              </div>
            </div>

            <!-- État vide (grille uniquement) -->
            <div v-else-if="viewMode === 'grille' && programmes.length === 0" class="text-center py-16 text-gray-500">
              <font-awesome-icon :icon="['fas', 'search']" class="h-12 mb-4 text-gray-300" />
              <p class="text-lg">Aucun programme ne correspond à vos critères de recherche.</p>
              <button
                v-if="filtresActifs"
                class="mt-4 text-custom-green underline hover:no-underline"
                @click="reinitialiserFiltres"
              >
                Réinitialiser les filtres
              </button>
            </div>

            <!-- Grille des programmes -->
            <div v-else-if="viewMode === 'grille'" class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 gap-6 justify-items-center">
              <SabbatiqueCard
                v-for="programme in programmes"
                :key="programme.id"
                :programme="programme"
                @click="voirDetail"
              />
            </div>

            <!-- Mode carte -->
            <div v-else class="flex flex-col lg:flex-row gap-6">
              <!-- Carte SVG d'Afrique -->
              <div class="flex-1 bg-white rounded-lg shadow-md">
                <div class="map-container relative p-0 sm:p-1" @mousemove="handleMapMouseMove">
                  <svg
                    ref="svgRef"
                    :viewBox="AFRICA_VIEWBOX"
                    class="africa-map w-full h-auto"
                    xmlns="http://www.w3.org/2000/svg"
                  >
                    <path
                      v-for="location in africaLocations"
                      :key="location.id"
                      :data-id="location.id"
                      :d="location.path"
                      :fill="getMapColor(location.id)"
                      stroke="#fff"
                      :stroke-width="strokeWidth(location.id)"
                      class="map-path"
                      :class="{ 'cursor-pointer': programmesParPays[location.id]?.length }"
                      :transform="mapTransforms[location.id]"
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
                      :class="{ 'map-tooltip-clickable': programmesParPays[hoveredCountry.id]?.length }"
                      :style="{ left: mousePos.x + 15 + 'px', top: mousePos.y - 10 + 'px' }"
                    >
                      <template v-if="programmesParPays[hoveredCountry.id]?.length">
                        <span class="font-semibold">{{ nomsPaysFr[hoveredCountry.id] || hoveredCountry.name }}</span>
                        <span class="text-xs opacity-70">
                          {{ programmesParPays[hoveredCountry.id]!.length }} programme{{ programmesParPays[hoveredCountry.id]!.length > 1 ? 's' : '' }}
                        </span>
                      </template>
                      <template v-else>
                        {{ nomsPaysFr[hoveredCountry.id] || hoveredCountry.name }}
                      </template>
                    </div>
                  </Transition>
                </div>
              </div>

              <!-- Panneau des programmes du pays sélectionné -->
              <Transition name="slide-in" mode="out-in">
                <div v-if="selectedPays" :key="selectedPays" class="lg:w-80 shrink-0">
                  <div class="bg-white rounded-lg shadow-md p-4">
                    <div class="flex items-center justify-between mb-4">
                      <h3 class="text-lg font-bold text-gray-900">{{ nomsPaysFr[selectedPays] || selectedPays }}</h3>
                      <button @click="selectedPays = null" class="text-gray-400 hover:text-gray-600" title="Fermer">
                        <font-awesome-icon :icon="['fas', 'xmark']" />
                      </button>
                    </div>

                    <p class="text-sm text-gray-500 mb-4">
                      {{ programmesPaysSelectionne.length }} programme{{ programmesPaysSelectionne.length > 1 ? 's' : '' }} disponible{{ programmesPaysSelectionne.length > 1 ? 's' : '' }}
                    </p>

                    <div class="space-y-4">
                      <SabbatiqueCard
                        v-for="programme in programmesPaysSelectionne"
                        :key="programme.id"
                        :programme="programme"
                        @click="voirDetail"
                      />
                    </div>
                  </div>
                </div>

                <!-- Invite par défaut -->
                <div v-else class="lg:w-80 shrink-0">
                  <div class="bg-white rounded-lg shadow-md p-6 text-center text-gray-500">
                    <font-awesome-icon :icon="['fas', 'hand-pointer']" class="h-8 mb-3 text-gray-300" />
                    <p class="text-sm">Cliquez sur un territoire mis en évidence pour voir ses programmes d'échange.</p>
                  </div>
                </div>
              </Transition>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, nextTick } from 'vue'
import AOS from 'aos'
import World from '@svg-maps/world'
import { PAYS_AFRICAINS_ISO2 } from '~/constants/afripulsePaysAutorises'
import {
  useSabbatiques,
  TYPES_PROGRAMME,
  PAYS_AFRICAINS,
  DOMAINES,
  type SabbatiqueAPI,
  type SabbatiqueFiltres,
} from '~/composables/useSabbatiques'

const { listerProgrammes, chargement } = useSabbatiques()

useHead({
  title: 'Échanges Sabbafrica - UAfricas',
  meta: [
    {
      name: 'description',
      content: 'Découvrez les programmes d\'échanges sabbatiques pour partager votre expertise en Afrique'
    }
  ]
})

const viewMode = ref<'grille' | 'carte'>('grille')

const filtres = ref<SabbatiqueFiltres>({
  type: 'tous',
  pays: '',
  domaine: '',
  recherche: ''
})

const programmes = ref<SabbatiqueAPI[]>([])
const total = ref(0)

const filtresActifs = computed(() =>
  filtres.value.type !== 'tous'
  || !!filtres.value.pays
  || !!filtres.value.domaine
  || !!filtres.value.recherche
)

const chargerProgrammes = async () => {
  const result = await listerProgrammes({ ...filtres.value, par_page: 60 })
  if (result) {
    programmes.value = result.programmes
    total.value = result.total
  }
}

// Debounce de la recherche textuelle
let searchTimeout: ReturnType<typeof setTimeout> | null = null
const onSearchInput = () => {
  if (searchTimeout) clearTimeout(searchTimeout)
  searchTimeout = setTimeout(() => {
    chargerProgrammes()
  }, 500)
}

// Rechargement immédiat sur les filtres select / radio
watch(
  () => [filtres.value.type, filtres.value.pays, filtres.value.domaine],
  () => {
    chargerProgrammes()
  }
)

const voirDetail = (programme: SabbatiqueAPI) => {
  navigateTo(`/echanges-sabbatiques/${programme.id}`)
}

const reinitialiserFiltres = () => {
  filtres.value = {
    type: 'tous',
    pays: '',
    domaine: '',
    recherche: ''
  }
  chargerProgrammes()
}

// === MODE CARTE ===

const PAYS_AFRICAINS_SET = new Set<string>(PAYS_AFRICAINS_ISO2)

// Noms français des pays africains (code ISO2 → nom)
const nomsPaysFr: Record<string, string> = {
  dz: 'Algérie', ao: 'Angola', bj: 'Bénin', bw: 'Botswana', bf: 'Burkina Faso',
  bi: 'Burundi', cv: 'Cap-Vert', cm: 'Cameroun', cf: 'Centrafrique',
  td: 'Tchad', km: 'Comores', cg: 'Congo', cd: 'RD Congo', ci: "Côte d'Ivoire",
  dj: 'Djibouti', eg: 'Égypte', gq: 'Guinée équatoriale', er: 'Érythrée',
  sz: 'Eswatini', et: 'Éthiopie', ga: 'Gabon', gm: 'Gambie', gh: 'Ghana',
  gn: 'Guinée', gw: 'Guinée-Bissau', ke: 'Kenya', ls: 'Lesotho', lr: 'Liberia',
  ly: 'Libye', mg: 'Madagascar', mw: 'Malawi', ml: 'Mali', mr: 'Mauritanie',
  mu: 'Maurice', ma: 'Maroc', mz: 'Mozambique', na: 'Namibie', ne: 'Niger',
  ng: 'Nigeria', rw: 'Rwanda', st: 'Sao Tomé-et-Principe', sn: 'Sénégal',
  sc: 'Seychelles', sl: 'Sierra Leone', so: 'Somalie', za: 'Afrique du Sud',
  ss: 'Soudan du Sud', sd: 'Soudan', tz: 'Tanzanie', tg: 'Togo', tn: 'Tunisie',
  ug: 'Ouganda', zm: 'Zambie', zw: 'Zimbabwe', eh: 'Sahara occidental',
}

// ViewBox calé au plus près du continent
const AFRICA_VIEWBOX = '401 347 239 267'

// Normalise un nom de pays (minuscule, sans accents) pour la correspondance
const normaliserNom = (nom: string): string =>
  nom.toLowerCase().normalize('NFD').replace(/[̀-ͯ]/g, '').trim()

// Index nom-français-normalisé → code ISO2
const isoParNom: Record<string, string> = Object.fromEntries(
  Object.entries(nomsPaysFr).map(([iso, nom]) => [normaliserNom(nom), iso])
)

// Locations africaines filtrées depuis la carte du monde
const africaLocations = computed(() =>
  World.locations.filter(loc => PAYS_AFRICAINS_SET.has(loc.id.toLowerCase()))
)

// Regroupe les programmes par code ISO2 du pays
const programmesParPays = computed<Record<string, SabbatiqueAPI[]>>(() => {
  const groupes: Record<string, SabbatiqueAPI[]> = {}
  for (const p of programmes.value) {
    if (!p.pays) continue
    const iso = isoParNom[normaliserNom(p.pays)]
    if (!iso) continue
    ;(groupes[iso] ||= []).push(p)
  }
  return groupes
})

// Petites îles trop petites pour être visibles : facteur d'agrandissement par code ISO
const PETITES_ILES: Record<string, number> = {
  cv: 5, st: 6, km: 5, mu: 6, sc: 7,
}

// Épaisseur de trait : réduite pour les petites îles agrandies
const strokeWidth = (id: string): number => {
  const facteur = PETITES_ILES[id]
  return facteur ? 0.5 / facteur : 0.5
}

const svgRef = ref<SVGSVGElement | null>(null)
const mapTransforms = ref<Record<string, string>>({})

// Calcule un scale centré sur le centroïde de chaque petite île
const calculerTransformsIles = () => {
  const svg = svgRef.value
  if (!svg) return
  const transforms: Record<string, string> = {}
  for (const [id, facteur] of Object.entries(PETITES_ILES)) {
    const path = svg.querySelector<SVGPathElement>(`path[data-id="${id}"]`)
    if (!path) continue
    const bbox = path.getBBox()
    const cx = bbox.x + bbox.width / 2
    const cy = bbox.y + bbox.height / 2
    transforms[id] = `translate(${cx} ${cy}) scale(${facteur}) translate(${-cx} ${-cy})`
  }
  mapTransforms.value = transforms
}

watch([viewMode, africaLocations], async () => {
  if (viewMode.value === 'carte') {
    await nextTick()
    calculerTransformsIles()
  }
})

const hoveredCountry = ref<{ id: string; name: string } | null>(null)
const mousePos = ref({ x: 0, y: 0 })
const selectedPays = ref<string | null>(null)

const programmesPaysSelectionne = computed(() =>
  selectedPays.value ? (programmesParPays.value[selectedPays.value] || []) : []
)

const adjustBrightness = (hex: string, percent: number): string => {
  const num = parseInt(hex.replace('#', ''), 16)
  const amt = Math.round(2.55 * percent)
  const R = Math.min(255, Math.max(0, (num >> 16) + amt))
  const G = Math.min(255, Math.max(0, ((num >> 8) & 0x00ff) + amt))
  const B = Math.min(255, Math.max(0, (num & 0x0000ff) + amt))
  return `#${((1 << 24) + (R << 16) + (G << 8) + B).toString(16).slice(1)}`
}

const getMapColor = (id: string): string => {
  const isHovered = hoveredCountry.value?.id === id
  const isSelected = selectedPays.value === id
  const aProgrammes = !!programmesParPays.value[id]?.length

  if (aProgrammes) {
    if (isSelected) return '#FFD700'
    if (isHovered) return adjustBrightness('#228B22', -15)
    return '#228B22'
  }

  if (isHovered) return '#bdbdbd'
  return '#e5e7eb'
}

const handleMapMouseMove = (event: MouseEvent) => {
  const container = event.currentTarget as HTMLElement
  const rect = container.getBoundingClientRect()
  mousePos.value = {
    x: event.clientX - rect.left,
    y: event.clientY - rect.top,
  }
}

const handleMapClick = (location: { id: string }) => {
  if (programmesParPays.value[location.id]?.length) {
    selectedPays.value = location.id
  }
}

onMounted(async () => {
  AOS.init({
    duration: 800,
    easing: 'ease-out-cubic',
    once: true
  })
  await chargerProgrammes()

  if (viewMode.value === 'carte') {
    await nextTick()
    calculerTransformsIles()
  }
})
</script>

<style scoped>
/* Carte SVG */
.map-container {
  position: relative;
  width: 100%;
}

.africa-map {
  display: block;
  width: 100%;
  height: auto;
  max-height: 92vh;
  margin: 0 auto;
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
