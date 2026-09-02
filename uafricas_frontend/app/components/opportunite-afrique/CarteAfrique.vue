<template>
  <div class="rounded-[10px] border border-af-bordure bg-white p-2">
    <div class="relative" @mousemove="surDeplacement">
      <svg ref="svgRef" :viewBox="AFRICA_VIEWBOX" class="af-carte block h-auto w-full" xmlns="http://www.w3.org/2000/svg">
        <path
          v-for="location in territoires"
          :key="location.id"
          :data-id="location.id"
          :d="location.path"
          :fill="couleurDe(location.id)"
          stroke="#fff"
          :stroke-width="PETITES_ILES[location.id] ? 0.5 / PETITES_ILES[location.id]! : 0.5"
          class="af-tuile"
          :class="ficheParCode(location.id) && 'cursor-pointer'"
          :transform="transforms[location.id]"
          @mouseenter="survole = location"
          @mouseleave="survole = null"
          @click="surClic(location)"
        />
      </svg>

      <Transition name="af-fondu">
        <div
          v-if="survole"
          class="af-bulle"
          :class="ficheParCode(survole.id) && 'af-bulle-active'"
          :style="{ left: `${souris.x + 15}px`, top: `${souris.y - 10}px` }"
        >
          <template v-if="ficheParCode(survole.id)">
            <span class="font-bold">{{ NOMS_PAYS_FR[survole.id] || survole.name }}</span>
            <span class="text-[12px]/[1.4] opacity-70">Cliquer pour voir</span>
          </template>
          <template v-else>
            {{ NOMS_PAYS_FR[survole.id] || survole.name }}
          </template>
        </div>
      </Transition>
    </div>
  </div>
</template>

<script setup lang="ts">
import World from '@svg-maps/world'
// `AFRICA_VIEWBOX`, `PETITES_ILES` et `NOMS_PAYS_FR` viennent de
// `utils/carteAfrique.ts` (auto-importé), partagé avec la carte de Retrouv'Amis.
import type { FichePaysAPI } from '~/composables/useOpportuniteAfrique'
import { PAYS_AFRICAINS_ISO2 } from '~/constants/afripulsePaysAutorises'

/**
 * Carte du continent. Extraite telle quelle de la page : le calcul des
 * agrandissements d'îles passe par `getBBox()`, donc par le DOM, et n'a de
 * sens qu'auprès du SVG qu'il mesure.
 */
const props = defineProps<{
  fiches: FichePaysAPI[]
  selection: FichePaysAPI | null
}>()

const emit = defineEmits<{ selectionner: [fiche: FichePaysAPI] }>()

const PAYS_AFRICAINS = new Set<string>(PAYS_AFRICAINS_ISO2)

const territoires = computed(() =>
  World.locations.filter(loc => PAYS_AFRICAINS.has(loc.id.toLowerCase())),
)

const svgRef = ref<SVGSVGElement | null>(null)
const transforms = ref<Record<string, string>>({})

/** Agrandissement centré sur le centroïde : appliqué au coin, l'île sortirait
 *  du cadre au lieu de grossir sur place. */
const calculerTransforms = () => {
  const svg = svgRef.value
  if (!svg) return
  const resultat: Record<string, string> = {}
  for (const [id, facteur] of Object.entries(PETITES_ILES)) {
    const path = svg.querySelector<SVGPathElement>(`path[data-id="${id}"]`)
    if (!path) continue
    const bbox = path.getBBox()
    const cx = bbox.x + bbox.width / 2
    const cy = bbox.y + bbox.height / 2
    resultat[id] = `translate(${cx} ${cy}) scale(${facteur}) translate(${-cx} ${-cy})`
  }
  transforms.value = resultat
}

onMounted(async () => {
  await nextTick()
  calculerTransforms()
})

watch(territoires, async () => {
  await nextTick()
  calculerTransforms()
})

const survole = ref<{ id: string, name: string } | null>(null)
const souris = ref({ x: 0, y: 0 })

const ficheParCode = (code: string): FichePaysAPI | undefined =>
  props.fiches.find(f => f.code?.toLowerCase() === code.toLowerCase())

/** Assombrit une couleur hexadécimale d'un pourcentage donné. */
const assombrir = (hex: string, pourcent: number): string => {
  const num = Number.parseInt(hex.replace('#', ''), 16)
  const delta = Math.round(2.55 * pourcent)
  const r = Math.min(255, Math.max(0, (num >> 16) + delta))
  const v = Math.min(255, Math.max(0, ((num >> 8) & 0x00FF) + delta))
  const b = Math.min(255, Math.max(0, (num & 0x0000FF) + delta))
  return `#${((1 << 24) + (r << 16) + (v << 8) + b).toString(16).slice(1)}`
}

const couleurDe = (id: string): string => {
  const estSurvole = survole.value?.id === id
  const estSelectionne = props.selection?.code?.toLowerCase() === id
  const fiche = ficheParCode(id)

  if (fiche) {
    const couleur = COULEURS_REGION[fiche.region] || '#9ca3af'
    if (estSelectionne) return COULEUR_SELECTION
    if (estSurvole) return assombrir(couleur, -15)
    return couleur
  }

  return estSurvole ? COULEUR_SANS_FICHE_SURVOL : COULEUR_SANS_FICHE
}

const surDeplacement = (evenement: MouseEvent) => {
  const conteneur = evenement.currentTarget as HTMLElement
  const rect = conteneur.getBoundingClientRect()
  souris.value = { x: evenement.clientX - rect.left, y: evenement.clientY - rect.top }
}

const surClic = (location: { id: string }) => {
  const fiche = ficheParCode(location.id)
  if (fiche) emit('selectionner', fiche)
}
</script>

<style scoped>
.af-carte {
  max-height: 92vh;
  margin: 0 auto;
}

.af-tuile {
  transition: fill 0.2s ease, opacity 0.2s ease;
}
.af-tuile:hover {
  opacity: 0.85;
}

.af-bulle {
  position: absolute;
  z-index: 50;
  transform: translateY(-50%);
  border-radius: 6px;
  background: rgba(0, 0, 0, 0.85);
  padding: 8px 14px;
  font-size: 13px;
  font-weight: 500;
  color: #fff;
  white-space: nowrap;
  pointer-events: none;
}

.af-bulle-active {
  display: flex;
  flex-direction: column;
  gap: 2px;
  padding: 10px 16px;
  pointer-events: auto;
  cursor: pointer;
  transition: all 0.2s ease;
}
.af-bulle-active:hover {
  background: rgba(167, 73, 22, 0.95);
  transform: translateY(-50%) scale(1.05);
}

.af-fondu-enter-active,
.af-fondu-leave-active {
  transition: opacity 0.2s ease;
}
.af-fondu-enter-from,
.af-fondu-leave-to {
  opacity: 0;
}
</style>
