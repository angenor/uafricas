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
  if (n) return `n. ${n}`
  return ''
})

const estDecede = computed(() => Boolean(props.data.deces?.annee))

// Anneau dégradé du médaillon selon le genre (ou chocolat si sélectionné)
const anneauGenre = computed(() => {
  if (props.selected) return 'from-[var(--color-custom-chocolat)] to-amber-500'
  if (props.data.genre === 'masculin') return 'from-sky-300 to-sky-500'
  if (props.data.genre === 'feminin') return 'from-pink-300 to-rose-500'
  return 'from-stone-300 to-stone-400'
})

const fondInitiales = computed(() => {
  if (props.data.genre === 'masculin') return 'bg-sky-50 text-sky-700'
  if (props.data.genre === 'feminin') return 'bg-pink-50 text-rose-700'
  return 'bg-stone-50 text-stone-600'
})

const surClicBadge = (e: MouseEvent) => {
  e.stopPropagation()
  emit('ajout-parent-demande', props.data.id)
}
</script>

<template>
  <div class="group relative flex w-[150px] cursor-pointer flex-col items-center">
    <!-- Badge incomplétude -->
    <button
      v-if="data.incompletude?.estIncomplet"
      class="absolute right-7 top-0 z-20 flex h-5 w-5 items-center justify-center rounded-full bg-amber-400 text-[11px] font-bold leading-none text-white shadow ring-2 ring-white transition-transform hover:scale-110"
      :title="data.incompletude.messageManquant || 'Parents manquants'"
      @click="surClicBadge"
    >
      +
    </button>

    <!-- Médaillon photo (anneau dégradé) -->
    <div
      :class="[
        'relative rounded-full bg-gradient-to-br p-[3px] shadow-md transition-all duration-200',
        anneauGenre,
        selected
          ? 'scale-105 shadow-lg ring-4 ring-[var(--color-custom-chocolat)]/25'
          : 'group-hover:scale-105 group-hover:shadow-lg',
      ]"
    >
      <img
        v-if="data.photo_url"
        :src="data.photo_url"
        :alt="nomComplet"
        :class="[
          'h-[76px] w-[76px] rounded-full object-cover ring-2 ring-white',
          estDecede ? 'opacity-80 grayscale-[35%]' : '',
        ]"
      />
      <div
        v-else
        :class="[
          'flex h-[76px] w-[76px] items-center justify-center rounded-full text-xl font-bold ring-2 ring-white',
          fondInitiales,
        ]"
      >
        {{ initiales }}
      </div>

      <!-- Marqueur personne décédée -->
      <span
        v-if="estDecede"
        class="absolute -bottom-0.5 -right-0.5 flex h-5 w-5 items-center justify-center rounded-full bg-stone-700 text-[9px] text-white shadow ring-2 ring-white"
        title="Personne décédée"
      >
        ✝
      </span>
    </div>

    <!-- Nom + années sur pilule translucide (lisible sur le fond dégradé) -->
    <div class="mt-2 max-w-[150px] rounded-xl bg-white/85 px-2.5 py-1 text-center shadow-sm ring-1 ring-black/5 backdrop-blur-sm">
      <p class="truncate text-[13px] font-semibold leading-tight text-stone-800">{{ nomComplet }}</p>
      <p v-if="anneesVie" class="truncate text-[10px] leading-tight text-stone-500">{{ anneesVie }}</p>
    </div>

    <!-- Handles (connexions branches) -->
    <Handle type="target" :position="Position.Top" class="!h-1.5 !w-1.5 !border-0 !bg-[#8a5a2b] !opacity-0" />
    <Handle type="source" :position="Position.Bottom" class="!h-1.5 !w-1.5 !border-0 !bg-[#8a5a2b] !opacity-0" />
    <Handle id="conjoint-left" type="target" :position="Position.Left" class="!h-1.5 !w-1.5 !border-0 !bg-[var(--color-custom-green)] !opacity-0" />
    <Handle id="conjoint-right" type="source" :position="Position.Right" class="!h-1.5 !w-1.5 !border-0 !bg-[var(--color-custom-green)] !opacity-0" />
  </div>
</template>
