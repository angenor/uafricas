<script setup lang="ts">
import { VueFlow, useVueFlow } from '@vue-flow/core'
import { Controls } from '@vue-flow/controls'
import { MiniMap } from '@vue-flow/minimap'
import type { Node, Edge } from '@vue-flow/core'
import NoeudPersonne from './NoeudPersonne.vue'

const props = defineProps<{
  nodes: Node[]
  edges: Edge[]
  selectedId?: string | null
}>()

const emit = defineEmits<{
  'node-click': [nodeId: string]
  'pane-click': []
}>()

const { fitView, setCenter } = useVueFlow()

const surClicNoeud = (_event: MouseEvent, node: Node) => {
  emit('node-click', node.id)
  // Recentrer la vue avec animation
  const { x, y } = node.position
  setCenter(x + 100, y + 50, { duration: 800, zoom: 1 })
}

const surClicFond = () => {
  emit('pane-click')
}

defineExpose({ fitView, setCenter })
</script>

<template>
  <div class="arbre-canevas relative h-full w-full overflow-hidden">
    <!-- Décor organique : feuillage en haut, terre/racines en bas -->
    <div class="pointer-events-none absolute inset-0 z-0" aria-hidden="true">
      <!-- Texture feuilles très subtile -->
      <div class="arbre-feuillage absolute inset-0" />
      <!-- Silhouette d'arbre / branches stylisées en bas -->
      <svg
        class="absolute bottom-0 left-1/2 h-[55%] w-[140%] -translate-x-1/2 text-[var(--color-custom-chocolat)] opacity-[0.06]"
        viewBox="0 0 800 400"
        preserveAspectRatio="xMidYMax meet"
        fill="none"
      >
        <path
          d="M400 400 V240 M400 300 C320 280 280 220 240 160 M400 300 C480 280 520 220 560 160 M400 250 C350 230 330 190 300 150 M400 250 C450 230 470 190 500 150 M240 160 C210 130 200 100 210 70 M560 160 C590 130 600 100 590 70 M300 150 C280 120 285 95 300 75 M500 150 C520 120 515 95 500 75"
          stroke="currentColor"
          stroke-width="10"
          stroke-linecap="round"
        />
      </svg>
    </div>

    <VueFlow
      :nodes="props.nodes"
      :edges="props.edges"
      :default-viewport="{ x: 0, y: 0, zoom: 0.8 }"
      :min-zoom="0.1"
      :max-zoom="2"
      :pan-on-drag="true"
      :zoom-on-scroll="true"
      :zoom-on-pinch="true"
      :fit-view-on-init="true"
      :fit-view-on-init-options="{ padding: 0.3, maxZoom: 0.8 }"
      :nodes-draggable="false"
      class="relative z-10 h-full w-full"
      @node-click="surClicNoeud"
      @pane-click="surClicFond"
    >
      <template #node-personne="{ data }">
        <NoeudPersonne
          :data="data"
          :selected="data.id === props.selectedId"
          @click.stop="emit('node-click', data.id)"
        />
      </template>

      <Controls position="bottom-right" />
      <MiniMap position="bottom-left" class="max-sm:hidden" />
    </VueFlow>
  </div>
</template>

<style>
@import '@vue-flow/core/dist/style.css';
@import '@vue-flow/core/dist/theme-default.css';
@import '@vue-flow/controls/dist/style.css';
@import '@vue-flow/minimap/dist/style.css';

/* Fond organique : canopée verte en haut → terre chocolat en bas */
.arbre-canevas {
  background:
    radial-gradient(120% 80% at 50% -10%, rgba(34, 139, 34, 0.10) 0%, transparent 55%),
    linear-gradient(180deg, #eef5e6 0%, #f7f3ea 48%, #efe2d2 100%);
}

/* Le canevas Vue Flow laisse voir le décor */
.arbre-canevas .vue-flow,
.arbre-canevas .vue-flow__pane,
.arbre-canevas .vue-flow__background {
  background: transparent !important;
}

/* Texture feuillage : pastilles vertes diffuses très légères */
.arbre-feuillage {
  background-image:
    radial-gradient(circle at 20% 25%, rgba(34, 139, 34, 0.06) 0 14px, transparent 15px),
    radial-gradient(circle at 78% 18%, rgba(34, 139, 34, 0.05) 0 18px, transparent 19px),
    radial-gradient(circle at 45% 12%, rgba(34, 139, 34, 0.05) 0 10px, transparent 11px),
    radial-gradient(circle at 88% 40%, rgba(34, 139, 34, 0.04) 0 12px, transparent 13px),
    radial-gradient(circle at 8% 55%, rgba(34, 139, 34, 0.04) 0 16px, transparent 17px);
  background-repeat: no-repeat;
}

.vue-flow__handle {
  width: 6px;
  height: 6px;
  opacity: 0;
}

/* Branches : tracé brun arrondi, union conjugale verte pointillée */
.vue-flow__edge-path {
  stroke-linecap: round;
  stroke-linejoin: round;
}
</style>
