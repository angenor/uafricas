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
    class="h-full w-full"
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
</template>

<style>
@import '@vue-flow/core/dist/style.css';
@import '@vue-flow/core/dist/theme-default.css';
@import '@vue-flow/controls/dist/style.css';
@import '@vue-flow/minimap/dist/style.css';

.vue-flow__handle {
  width: 6px;
  height: 6px;
  opacity: 0;
}

.vue-flow__edge-path {
  stroke-linecap: round;
}
</style>
