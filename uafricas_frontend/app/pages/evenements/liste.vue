<template>
  <div class="min-h-screen flex flex-col bg-gray-50">
    <!-- Hero Section (compact, titre ↔ description au survol) -->
    <div class="group relative">
      <div class="absolute inset-0 bg-linear-to-r from-custom-chocolat to-black/90"></div>

      <div class="relative max-w-4xl mx-auto px-4 pt-16 pb-6 text-center select-none">
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12">
          <h1 class="absolute inset-0 flex items-center justify-center text-white text-2xl md:text-4xl font-bold transition-opacity duration-300 group-hover:opacity-0">
            Africalive
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Découvrez nos événements
          </p>
        </div>
      </div>
    </div>

    <!-- Breadcrumb -->
    <div class="backdrop-blur-xs">
      <div class="mx-auto px-4 py-3">
        <CommonBreadcrumbNav :custom-breadcrumbs="breadcrumbs" />
      </div>
    </div>

    <!-- Contenu principal -->
    <div class="container mx-auto px-4 py-8">
      <!-- Filtres + toggle vue -->
      <div class="flex flex-col gap-4 mb-8 lg:flex-row lg:items-center lg:justify-between">
        <EvenementsEvenementFilters
          v-model:annee-selected="anneeSelected"
          v-model:filtre-type="filtreType"
          v-model:filtre-pays="filtrePays"
          v-model:filtre-zone="filtreZone"
          @open-modal="showModal = true"
          class="flex-1"
        />

        <!-- Toggle grille / carte (la carte Afrique n'a de sens que pour la zone Afrique) -->
        <div v-if="filtreZone === 'afrique'" class="flex items-center bg-gray-100 rounded-lg p-1 self-start lg:self-auto shrink-0">
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

      <!-- Chargement -->
      <div v-if="chargement" class="text-center py-16">
        <div class="text-4xl text-gray-300 mb-4 animate-spin inline-block">
          <font-awesome-icon icon="fa-solid fa-spinner" />
        </div>
        <p class="text-gray-500">Chargement des événements...</p>
      </div>

      <!-- Erreur -->
      <div v-else-if="erreur" class="text-center py-16">
        <div class="text-5xl text-red-300 mb-4">
          <font-awesome-icon icon="fa-solid fa-triangle-exclamation" />
        </div>
        <h3 class="text-xl font-semibold text-gray-500">
          Erreur de chargement
        </h3>
        <p class="text-gray-400 mt-2">{{ erreur }}</p>
        <button
          @click="chargerEvenements"
          class="mt-4 text-white bg-custom-green rounded-md py-2 px-4 hover:bg-custom-green/90 transition-colors"
        >
          Réessayer
        </button>
      </div>

      <!-- Grille d'événements -->
      <div v-else-if="viewMode === 'grille' && evenements.length" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <EvenementsEvenementCard
          v-for="evenement in evenements"
          :key="evenement.id"
          :evenement="evenement"
        />
      </div>

      <!-- État vide (grille) -->
      <div v-else-if="viewMode === 'grille'" class="text-center py-16">
        <div class="text-5xl text-gray-300 mb-4">
          <font-awesome-icon icon="fa-solid fa-calendar-xmark" />
        </div>
        <h3 class="text-xl font-semibold text-gray-500">
          Aucun événement trouvé
        </h3>
        <p class="text-gray-400 mt-2">
          Essayez de modifier vos filtres ou proposez un nouvel événement
        </p>
        <button
          @click="showModal = true"
          class="mt-6 text-white bg-custom-green rounded-md py-2 px-4 hover:bg-custom-green/90 transition-colors"
        >
          Proposer un événement
        </button>
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
                :class="{ 'cursor-pointer': evenementsParPays[location.id]?.length }"
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
                :class="{ 'map-tooltip-clickable': evenementsParPays[hoveredCountry.id]?.length }"
                :style="{ left: mousePos.x + 15 + 'px', top: mousePos.y - 10 + 'px' }"
              >
                <template v-if="evenementsParPays[hoveredCountry.id]?.length">
                  <span class="font-semibold">{{ nomsPaysFr[hoveredCountry.id] || hoveredCountry.name }}</span>
                  <span class="text-xs opacity-70">
                    {{ evenementsParPays[hoveredCountry.id]!.length }} événement{{ evenementsParPays[hoveredCountry.id]!.length > 1 ? 's' : '' }}
                  </span>
                </template>
                <template v-else>
                  {{ nomsPaysFr[hoveredCountry.id] || hoveredCountry.name }}
                </template>
              </div>
            </Transition>
          </div>

          <!-- Légende -->
          <div class="flex flex-wrap gap-4 px-4 pb-4 pt-2">
            <div class="flex items-center gap-2">
              <span class="w-3 h-3 rounded-full shrink-0 bg-custom-green"></span>
              <span class="text-xs text-gray-600">Événements disponibles</span>
            </div>
            <div class="flex items-center gap-2">
              <span class="w-3 h-3 rounded-full shrink-0" style="background:#FFD700"></span>
              <span class="text-xs text-gray-600">Sélectionné</span>
            </div>
            <div class="flex items-center gap-2">
              <span class="w-3 h-3 rounded-full shrink-0 bg-gray-200"></span>
              <span class="text-xs text-gray-600">Aucun événement</span>
            </div>
          </div>
        </div>

        <!-- Panneau des événements du pays sélectionné -->
        <Transition name="slide-in" mode="out-in">
          <div v-if="selectedPays" :key="selectedPays" class="lg:w-96 shrink-0">
            <div class="bg-white rounded-lg shadow-md p-4">
              <div class="flex items-center justify-between mb-4">
                <h3 class="text-lg font-bold text-gray-900">{{ nomsPaysFr[selectedPays] || selectedPays }}</h3>
                <button @click="selectedPays = null" class="text-gray-400 hover:text-gray-600" title="Fermer">
                  <font-awesome-icon :icon="['fas', 'xmark']" />
                </button>
              </div>

              <p class="text-sm text-gray-500 mb-4">
                {{ evenementsPaysSelectionne.length }} événement{{ evenementsPaysSelectionne.length > 1 ? 's' : '' }} disponible{{ evenementsPaysSelectionne.length > 1 ? 's' : '' }}
              </p>

              <div class="space-y-4">
                <EvenementsEvenementCard
                  v-for="evenement in evenementsPaysSelectionne"
                  :key="evenement.id"
                  :evenement="evenement"
                />
              </div>
            </div>
          </div>

          <!-- Invite par défaut -->
          <div v-else class="lg:w-96 shrink-0">
            <div class="bg-white rounded-lg shadow-md p-6 text-center text-gray-500">
              <span class="block lg:hidden">
                <font-awesome-icon :icon="['fas', 'hand-point-up']" class="h-8 mb-3 text-gray-300" />
              </span>
              <span class="hidden lg:block">
                <font-awesome-icon :icon="['fas', 'hand-point-left']" class="h-8 mb-3 text-gray-300" />
              </span>
              <p class="text-sm">Cliquez sur un territoire mis en évidence pour voir ses événements.</p>
            </div>
          </div>
        </Transition>
      </div>
    </div>

    <!-- Modal de création -->
    <EvenementsEvenementModal
      :show="showModal"
      @close="showModal = false"
      @submit="handleSubmit"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, nextTick } from 'vue'
import World from '@svg-maps/world'
import { PAYS_AFRICAINS_ISO2 } from '~/constants/afripulsePaysAutorises'
import { useEvenements, type EvenementAPI, type EvenementFiltres } from '~/composables/useEvenements'

useHead({
  title: 'Africalive - Événements & Ateliers | AfricanS'
})

const breadcrumbs = [
  { label: 'Événements', to: '/evenements' },
  { label: 'Liste', to: undefined }
]

const { listerEvenements, creerEvenement, chargement, erreur } = useEvenements()

const showModal = ref(false)
const viewMode = ref<'grille' | 'carte'>('grille')
const anneeSelected = ref(new Date().getFullYear().toString())
const filtreType = ref('')
const filtrePays = ref('')
const filtreZone = ref<'afrique' | 'hors_afrique'>('afrique')
const evenements = ref<EvenementAPI[]>([])

const chargerEvenements = async () => {
  const filtres: EvenementFiltres = {
    annee: parseInt(anneeSelected.value),
    format: filtreType.value || undefined,
    pays: filtrePays.value || undefined,
    zone: filtreZone.value,
    par_page: 50,
  }
  const data = await listerEvenements(filtres)
  evenements.value = data?.evenements ?? []
}

// Changer de zone : la carte Afrique n'a de sens que pour l'Afrique ; hors
// Afrique on force la grille et on réinitialise le territoire (liste africaine).
watch(filtreZone, (zone) => {
  if (zone === 'hors_afrique') {
    viewMode.value = 'grille'
    filtrePays.value = ''
  }
})

watch([anneeSelected, filtreType, filtrePays, filtreZone], chargerEvenements)
onMounted(async () => {
  await chargerEvenements()
  if (viewMode.value === 'carte') {
    await nextTick()
    calculerTransformsIles()
  }
})

const handleSubmit = async (data: any) => {
  const result = await creerEvenement(
    {
      titre: data.titre,
      description: data.description,
      type: data.type,
      thematique: data.thematique,
      pays: data.pays,
      ville: data.ville,
      date_heure_debut: data.date_heure_debut,
      date_heure_fin: data.date_heure_fin,
      adresse: data.adresse,
      lien_en_ligne: data.lien_en_ligne,
      nombre_places: data.nombre_places,
      type_organisateur: data.type_organisateur,
      contact_nom: data.contact_nom,
      contact_email: data.contact_email,
      contact_telephone: data.contact_telephone,
      contact_site_web: data.contact_site_web,
    },
    data.couverture_file,
  )
  if (result) {
    showModal.value = false
    await chargerEvenements()
  }
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

// Regroupe les événements par code ISO2 du pays
const evenementsParPays = computed<Record<string, EvenementAPI[]>>(() => {
  const groupes: Record<string, EvenementAPI[]> = {}
  for (const e of evenements.value) {
    if (!e.pays) continue
    const iso = isoParNom[normaliserNom(e.pays)]
    if (!iso) continue
    ;(groupes[iso] ||= []).push(e)
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

const evenementsPaysSelectionne = computed(() =>
  selectedPays.value ? (evenementsParPays.value[selectedPays.value] || []) : []
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
  const aEvenements = !!evenementsParPays.value[id]?.length

  if (aEvenements) {
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
  if (evenementsParPays.value[location.id]?.length) {
    selectedPays.value = location.id
  }
}
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
