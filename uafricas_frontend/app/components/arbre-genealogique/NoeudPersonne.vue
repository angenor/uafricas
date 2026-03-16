<script setup lang="ts">
import { computed } from 'vue'
import { Handle, Position } from '@vue-flow/core'
import type { NoeudArbre, InfoIncompletude } from '~/composables/useLayoutArbre'

const props = defineProps<{
  data: NoeudArbre & { incompletude?: InfoIncompletude }
  selected?: boolean
}>()

const emit = defineEmits<{
  'ajout-parent-demande': [nodeId: string]
}>()

const nomComplet = computed(() => {
  return props.data.prenoms
    ? `${props.data.prenoms} ${props.data.nom}`
    : props.data.nom
})

const initiales = computed(() => {
  const premierPrenom = props.data.prenoms?.split(' ')[0] ?? ''
  return `${premierPrenom.charAt(0)}${props.data.nom.charAt(0)}`.toUpperCase()
})

const anneesVie = computed(() => {
  const n = props.data.naissance?.annee
  const d = props.data.deces?.annee
  if (n && d) return `${n} – ${d}`
  if (n) return `né(e) en ${n}`
  return ''
})

const bordureCouleur = computed(() => {
  if (props.selected) return 'border-[var(--color-custom-chocolat)]'
  if (props.data.genre === 'masculin') return 'border-sky-400'
  if (props.data.genre === 'feminin') return 'border-pink-400'
  return 'border-stone-300'
})

const fondInitiales = computed(() => {
  if (props.data.genre === 'masculin') return 'bg-sky-100 text-sky-700'
  if (props.data.genre === 'feminin') return 'bg-pink-100 text-pink-700'
  return 'bg-stone-100 text-stone-600'
})

const surClicBadge = (e: MouseEvent) => {
  e.stopPropagation()
  emit('ajout-parent-demande', props.data.id)
}
</script>

<template>
  <div
    :class="[
      'relative flex items-center gap-2 rounded-xl border-2 bg-white px-3 py-2 shadow-sm transition-all',
      'min-w-[160px] max-w-[220px] cursor-pointer',
      bordureCouleur,
      selected ? 'shadow-md ring-2 ring-[var(--color-custom-chocolat)]/30' : 'hover:shadow-md',
    ]"
  >
    <!-- Badge incomplétude -->
    <button
      v-if="data.incompletude?.estIncomplet"
      class="absolute -right-1.5 -top-1.5 flex h-5 w-5 items-center justify-center rounded-full bg-amber-400 text-[9px] text-white shadow-sm transition-transform hover:scale-110"
      :title="data.incompletude.messageManquant || 'Parents manquants'"
      @click="surClicBadge"
    >
      +
    </button>

    <!-- Photo ou initiales -->
    <div class="shrink-0">
      <img
        v-if="data.photo_url"
        :src="data.photo_url"
        :alt="nomComplet"
        class="h-10 w-10 rounded-full object-cover"
      />
      <div
        v-else
        :class="[
          'flex h-10 w-10 items-center justify-center rounded-full text-xs font-bold',
          fondInitiales,
        ]"
      >
        {{ initiales }}
      </div>
    </div>

    <!-- Infos -->
    <div class="min-w-0 flex-1">
      <p class="truncate text-sm font-semibold text-stone-800">{{ nomComplet }}</p>
      <p v-if="anneesVie" class="truncate text-xs text-stone-500">{{ anneesVie }}</p>
    </div>

    <!-- Handles -->
    <Handle type="target" :position="Position.Top" class="!bg-stone-400" />
    <Handle type="source" :position="Position.Bottom" class="!bg-stone-400" />
    <Handle id="conjoint-left" type="target" :position="Position.Left" class="!bg-[var(--color-custom-chocolat)]" />
    <Handle id="conjoint-right" type="source" :position="Position.Right" class="!bg-[var(--color-custom-chocolat)]" />
  </div>
</template>
