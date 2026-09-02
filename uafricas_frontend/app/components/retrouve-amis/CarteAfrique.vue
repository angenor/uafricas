<script setup lang="ts">
import World from '@svg-maps/world'
import { PAYS_AFRICAINS_ISO2 } from '~/constants/afripulsePaysAutorises'
import { NOMS_PAYS_FR, AFRICA_VIEWBOX, PETITES_ILES, couleurChaleurAvis } from '~/utils/carteAfrique'

interface MapLocation {
  id: string
  name: string
  path: string
}

const props = defineProps<{
  /** Nombre d'avis de recherche par code ISO2 (minuscule). */
  comptes: Record<string, number>
  /** Code ISO2 du pays sélectionné (minuscule) ou null. */
  selectedIso: string | null
}>()

const emit = defineEmits<{ (e: 'select', iso: string): void }>()

// ── Locations africaines filtrées depuis la carte du monde ────
const PAYS_AFRICAINS = new Set<string>(PAYS_AFRICAINS_ISO2)
const africaLocations = computed<MapLocation[]>(() =>
  (World.locations as MapLocation[]).filter(loc => PAYS_AFRICAINS.has(loc.id.toLowerCase())),
)

// ── Agrandissement des petites îles ───────────────────────────
const svgRef = ref<SVGSVGElement | null>(null)
const mapTransforms = ref<Record<string, string>>({})

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

onMounted(async () => {
  await nextTick()
  calculerTransformsIles()
})
watch(africaLocations, async () => {
  await nextTick()
  calculerTransformsIles()
})

// ── Survol / tooltip ──────────────────────────────────────────
const hoveredCountry = ref<MapLocation | null>(null)
const mousePos = ref({ x: 0, y: 0 })

const handleMapMouseMove = (event: MouseEvent) => {
  const container = event.currentTarget as HTMLElement
  const rect = container.getBoundingClientRect()
  mousePos.value = { x: event.clientX - rect.left, y: event.clientY - rect.top }
}

// ── Comptes / couleurs ────────────────────────────────────────
const compteFor = (iso: string): number => props.comptes[iso.toLowerCase()] || 0

/** Assombrit une couleur hex d'un pourcentage donné (négatif = plus foncé). */
const adjustBrightness = (hex: string, percent: number): string => {
  const num = parseInt(hex.replace('#', ''), 16)
  const amt = Math.round(2.55 * percent)
  const R = Math.min(255, Math.max(0, (num >> 16) + amt))
  const G = Math.min(255, Math.max(0, ((num >> 8) & 0x00ff) + amt))
  const B = Math.min(255, Math.max(0, (num & 0x0000ff) + amt))
  return `#${((1 << 24) + (R << 16) + (G << 8) + B).toString(16).slice(1)}`
}

/** Échelle de chaleur ambre selon le nombre d'avis. */
// L'échelle vit dans `utils/carteAfrique.ts` : la légende du rail la lit aussi,
// et deux tables séparées finiraient par diverger.
const couleurChaleur = couleurChaleurAvis

const getMapColor = (id: string): string => {
  const iso = id.toLowerCase()
  const compte = compteFor(iso)
  const isHovered = hoveredCountry.value?.id.toLowerCase() === iso
  const isSelected = props.selectedIso === iso

  if (isSelected) return '#A54A1C' // af-chocolat
  if (compte > 0) {
    const base = couleurChaleur(compte)
    return isHovered ? adjustBrightness(base, -12) : base
  }
  return isHovered ? '#d1d5db' : '#e5e7eb'
}

const handleMapClick = (location: MapLocation) => {
  if (compteFor(location.id) > 0) emit('select', location.id.toLowerCase())
}
</script>

<template>
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
        :stroke-width="PETITES_ILES[location.id] ? 0.5 / PETITES_ILES[location.id] : 0.5"
        class="map-path"
        :class="{ 'cursor-pointer': compteFor(location.id) > 0 }"
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
        :class="{ 'map-tooltip-clickable': compteFor(hoveredCountry.id) > 0 }"
        :style="{ left: mousePos.x + 15 + 'px', top: mousePos.y - 10 + 'px' }"
      >
        <span class="font-semibold">
          {{ NOMS_PAYS_FR[hoveredCountry.id.toLowerCase()] || hoveredCountry.name }}
        </span>
        <template v-if="compteFor(hoveredCountry.id) > 0">
          <span class="text-xs opacity-80">
            {{ compteFor(hoveredCountry.id) }}
            avis · cliquer pour voir
          </span>
        </template>
        <span v-else class="text-xs opacity-70">Aucun avis</span>
      </div>
    </Transition>
  </div>
</template>

<style scoped>
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
  opacity: 0.9;
}

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
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.map-tooltip-clickable {
  pointer-events: auto;
  cursor: pointer;
  padding: 10px 16px;
  transition: all 0.2s ease;
}

.map-tooltip-clickable:hover {
  background: rgba(165, 74, 28, 0.95);
  transform: translateY(-50%) scale(1.05);
}

.map-fade-enter-active,
.map-fade-leave-active {
  transition: opacity 0.2s ease;
}

.map-fade-enter-from,
.map-fade-leave-to {
  opacity: 0;
}
</style>
