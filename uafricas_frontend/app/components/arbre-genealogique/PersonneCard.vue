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

// `urlMedia` : le backend renvoie un chemin relatif servi sur SON port.
const photo = computed(() => urlMedia(props.personne.photo_url))

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
    class="group w-full rounded-[10px] border border-af-bordure bg-white p-4 text-left transition hover:border-af-chocolat"
    @click="emit('click', personne.id)"
  >
    <div class="flex items-center gap-3">
      <!-- Photo ou initiales -->
      <img
        v-if="photo"
        :src="photo"
        :alt="nomComplet"
        class="size-12 shrink-0 rounded-full object-cover"
      />
      <span
        v-else
        class="grid size-12 shrink-0 place-items-center rounded-full bg-af-chocolat/15 text-[14px]/[1.4] font-bold text-af-chocolat"
      >{{ initiales }}</span>

      <!-- Infos -->
      <div class="min-w-0 flex-1">
        <p class="truncate text-[14px]/[1.4] font-bold text-af-encre transition group-hover:text-af-chocolat">
          {{ nomComplet }}
        </p>
        <p v-if="anneesVie" class="mt-0.5 text-[12px]/[1.4] text-af-corps">{{ anneesVie }}</p>
        <p v-if="personne.naissance_lieu" class="mt-0.5 truncate text-[12px]/[1.4] text-af-atone">
          {{ personne.naissance_lieu }}
        </p>
      </div>

      <font-awesome-icon
        icon="fa-solid fa-chevron-right"
        class="shrink-0 text-af-atone-2 transition group-hover:translate-x-1 group-hover:text-af-chocolat"
      />
    </div>
  </button>
</template>
