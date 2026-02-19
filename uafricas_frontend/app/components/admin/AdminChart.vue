<script setup lang="ts">
export interface ChartSegment {
  label: string
  valeur: number
  couleur: string
}

const props = defineProps<{
  titre: string
  type: 'barres' | 'donut'
  segments: ChartSegment[]
}>()

const total = computed(() => props.segments.reduce((s, seg) => s + seg.valeur, 0))

const maxValeur = computed(() => Math.max(...props.segments.map(s => s.valeur), 1))

// Donut : calcul des arcs SVG
const donutArcs = computed(() => {
  if (total.value === 0) return []
  let cumul = 0
  return props.segments
    .filter(s => s.valeur > 0)
    .map((seg) => {
      const pct = seg.valeur / total.value
      const start = cumul
      cumul += pct
      return { ...seg, start, pct }
    })
})

const describeArc = (start: number, pct: number) => {
  const r = 40
  const cx = 50
  const cy = 50
  const startAngle = start * 2 * Math.PI - Math.PI / 2
  const endAngle = (start + pct) * 2 * Math.PI - Math.PI / 2
  const largeArc = pct > 0.5 ? 1 : 0
  const x1 = cx + r * Math.cos(startAngle)
  const y1 = cy + r * Math.sin(startAngle)
  const x2 = cx + r * Math.cos(endAngle)
  const y2 = cy + r * Math.sin(endAngle)
  return `M ${cx} ${cy} L ${x1} ${y1} A ${r} ${r} 0 ${largeArc} 1 ${x2} ${y2} Z`
}
</script>

<template>
  <div class="card bg-base-100 border border-base-200 shadow-sm">
    <div class="card-body p-5">
      <h2 class="card-title text-sm font-display mb-3">{{ titre }}</h2>

      <!-- Barres horizontales -->
      <div v-if="type === 'barres'" class="space-y-2">
        <div v-for="seg in segments" :key="seg.label" class="flex items-center gap-2">
          <span class="text-xs text-base-content/60 w-28 truncate" :title="seg.label">{{ seg.label }}</span>
          <div class="flex-1 bg-base-200 rounded-full h-5 overflow-hidden">
            <div
              class="h-full rounded-full transition-all duration-500 flex items-center justify-end pr-1.5"
              :class="seg.couleur"
              :style="{ width: maxValeur > 0 ? `${Math.max((seg.valeur / maxValeur) * 100, seg.valeur > 0 ? 8 : 0)}%` : '0%' }"
            >
              <span v-if="seg.valeur > 0" class="text-[10px] font-bold text-white">{{ seg.valeur }}</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Donut SVG -->
      <div v-else-if="type === 'donut'" class="flex items-center gap-4">
        <div class="w-28 h-28 flex-shrink-0">
          <svg v-if="total > 0" viewBox="0 0 100 100" class="w-full h-full">
            <path
              v-for="(arc, i) in donutArcs"
              :key="i"
              :d="describeArc(arc.start, arc.pct)"
              :class="arc.couleur"
              class="transition-all duration-500"
              fill="currentColor"
            />
            <circle cx="50" cy="50" r="22" class="fill-base-100" />
            <text x="50" y="50" text-anchor="middle" dominant-baseline="central" class="fill-base-content text-[11px] font-bold">
              {{ total }}
            </text>
          </svg>
          <div v-else class="w-full h-full flex items-center justify-center text-base-content/30">
            <font-awesome-icon icon="chart-pie" class="w-10 h-10" />
          </div>
        </div>
        <ul class="space-y-1 flex-1">
          <li v-for="seg in segments" :key="seg.label" class="flex items-center gap-2 text-xs">
            <span class="w-2.5 h-2.5 rounded-full flex-shrink-0" :class="seg.couleur.replace('text-', 'bg-')" />
            <span class="text-base-content/60 flex-1">{{ seg.label }}</span>
            <span class="font-medium">{{ seg.valeur }}</span>
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>
