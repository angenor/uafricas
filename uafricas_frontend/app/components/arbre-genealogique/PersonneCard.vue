<script setup lang="ts">
import type { Personne } from '~/mocks/arbre-genealogique'
import { formaterDate, getInitiales } from '~/mocks/arbre-genealogique'

interface Props {
  personne: Personne
}

const props = defineProps<Props>()
const emit = defineEmits<{ click: [id: string] }>()

const nomComplet = computed(() => {
  if (props.personne.prenoms) return `${props.personne.prenoms} ${props.personne.nom}`
  return props.personne.nom
})

const initiales = computed(() => getInitiales(props.personne))

const anneesVie = computed(() => {
  const n = props.personne.naissance?.annee
  const d = props.personne.deces?.annee
  if (n && d) return `${n} – ${d}`
  if (n) return `né(e) en ${n}`
  if (d) return `décédé(e) en ${d}`
  return null
})
</script>

<template>
  <button
    type="button"
    class="w-full text-left bg-white border border-stone-200 rounded-xl p-4 hover:border-custom-chocolat/40 hover:shadow-md transition-all group"
    @click="emit('click', personne.id)"
  >
    <div class="flex items-center gap-3">
      <!-- Photo ou initiales -->
      <div class="shrink-0">
        <img
          v-if="personne.photo_url"
          :src="personne.photo_url"
          :alt="nomComplet"
          class="w-12 h-12 rounded-full object-cover border-2 border-stone-200 group-hover:border-custom-chocolat/40 transition-colors"
        />
        <div
          v-else
          class="w-12 h-12 rounded-full bg-custom-chocolat/10 border-2 border-custom-chocolat/20 flex items-center justify-center group-hover:bg-custom-chocolat/20 transition-colors"
        >
          <span class="text-sm font-bold text-custom-chocolat">{{ initiales }}</span>
        </div>
      </div>

      <!-- Infos -->
      <div class="min-w-0 flex-1">
        <p class="font-semibold text-stone-800 truncate group-hover:text-custom-chocolat transition-colors">
          {{ nomComplet }}
        </p>
        <p v-if="anneesVie" class="text-xs text-stone-500 mt-0.5">{{ anneesVie }}</p>
        <p v-if="personne.naissance_lieu" class="text-xs text-stone-400 truncate mt-0.5">
          {{ personne.naissance_lieu }}
        </p>
      </div>

      <!-- Flèche -->
      <svg class="shrink-0 w-4 h-4 text-stone-400 group-hover:text-custom-chocolat transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
      </svg>
    </div>
  </button>
</template>
