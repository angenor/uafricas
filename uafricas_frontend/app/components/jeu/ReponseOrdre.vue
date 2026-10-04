<template>
  <div class="flex flex-col gap-4">
    <!-- Après la correction : l'ordre ATTENDU, avec ce qui le justifie, et le
         rang où le membre avait mis chaque élément. -->
    <ol v-if="solution?.solution" class="flex flex-col gap-2" role="list">
      <li
        v-for="(cle, i) in solution.solution"
        :key="cle"
        class="flex items-center gap-3 rounded-lg border px-4 py-3 text-[15px]/[1.4]"
        :class="bienPlace(cle, i) ? 'border-af-vert/50 bg-af-vert/5' : 'border-af-live/30 bg-af-live/5'"
      >
        <span class="grid size-7 shrink-0 place-items-center rounded-full bg-af-surface text-[13px] font-bold text-af-encre">
          {{ i + 1 }}
        </span>
        <span class="flex-1 font-bold text-af-encre">{{ texte(cle) }}</span>
        <span v-if="solution.valeurs?.[cle]" class="text-[13px] text-af-corps">{{ solution.valeurs[cle] }}</span>
        <font-awesome-icon
          :icon="bienPlace(cle, i) ? 'fa-solid fa-circle-check' : 'fa-solid fa-circle-xmark'"
          :class="bienPlace(cle, i) ? 'text-af-vert' : 'text-af-live'"
        />
      </li>
    </ol>

    <template v-else>
      <p class="text-[13px]/[1.4] text-af-atone">
        Faites glisser les éléments, ou utilisez les flèches de chaque ligne.
      </p>
      <ol class="flex flex-col gap-2" role="list">
        <li
          v-for="(element, i) in ordre"
          :key="element.cle"
          class="flex items-center gap-2 rounded-lg border bg-af-surface px-3 py-2.5 text-[15px]/[1.4] text-af-encre transition"
          :class="survol === i ? 'border-af-chocolat bg-af-chocolat/[0.05]' : 'border-af-bordure'"
          :draggable="!verrouillee"
          @dragstart="debutGlisser(i)"
          @dragover.prevent="survol = i"
          @dragleave="survol = null"
          @drop.prevent="deposer(i)"
          @dragend="survol = null"
        >
          <font-awesome-icon icon="fa-solid fa-grip-vertical" class="cursor-grab text-af-atone" aria-hidden="true" />
          <span class="grid size-7 shrink-0 place-items-center rounded-full bg-af-fond text-[13px] font-bold">{{ i + 1 }}</span>
          <span class="flex-1">{{ element.texte }}</span>
          <button
            type="button"
            class="grid size-9 place-items-center rounded-md text-af-corps hover:bg-af-fond disabled:opacity-30 focus-visible:outline-2 focus-visible:outline-af-chocolat"
            :disabled="verrouillee || i === 0"
            :aria-label="`Monter ${element.texte}`"
            @click="deplacer(i, i - 1)"
          >
            <font-awesome-icon icon="fa-solid fa-chevron-up" />
          </button>
          <button
            type="button"
            class="grid size-9 place-items-center rounded-md text-af-corps hover:bg-af-fond disabled:opacity-30 focus-visible:outline-2 focus-visible:outline-af-chocolat"
            :disabled="verrouillee || i === ordre.length - 1"
            :aria-label="`Descendre ${element.texte}`"
            @click="deplacer(i, i + 1)"
          >
            <font-awesome-icon icon="fa-solid fa-chevron-down" />
          </button>
        </li>
      </ol>
      <div class="flex justify-end">
        <AfricansBouton icone="fa-solid fa-check" :desactive="verrouillee" @click="valider">
          Valider cet ordre
        </AfricansBouton>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import type { PropositionServieAPI, ReponseJoueur, SolutionAPI } from '~/composables/useJeu'

/**
 * Épreuve « ordre » (feature 014) : remettre de 3 à 6 éléments dans l'ordre que
 * dit l'énoncé. Tout ou rien : la correction dit quels éléments étaient à leur
 * place, mais l'épreuve n'est juste que si tous le sont.
 */
const props = defineProps<{
  elements: PropositionServieAPI[]
  verrouillee?: boolean
  solution?: SolutionAPI | null
  jouee?: ReponseJoueur | null
}>()

const emit = defineEmits<{ repondre: [reponse: ReponseJoueur] }>()

const ordre = ref<PropositionServieAPI[]>([...props.elements])
// On surveille les CLÉS, pas le tableau : en duel direct l'état est relu toutes
// les trois secondes et l'épreuve arrive chaque fois dans un objet neuf ; suivre
// l'objet effacerait l'ordre en cours de construction.
watch(() => props.elements.map(e => e.cle).join(','), () => { ordre.value = [...props.elements] })

const deplacer = (de: number, vers: number) => {
  if (props.verrouillee || vers < 0 || vers >= ordre.value.length) return
  const copie = [...ordre.value]
  const [element] = copie.splice(de, 1)
  copie.splice(vers, 0, element!)
  ordre.value = copie
}

// Glisser-déposer natif, pour la souris. Au doigt, les flèches font le travail :
// l'API de glisser-déposer du navigateur ne répond pas au toucher.
const source = ref<number | null>(null)
const survol = ref<number | null>(null)
const debutGlisser = (i: number) => { source.value = i }
const deposer = (i: number) => {
  if (source.value != null) deplacer(source.value, i)
  source.value = null
  survol.value = null
}

const valider = () => {
  if (!props.verrouillee) emit('repondre', { ordre: ordre.value.map(e => e.cle) })
}

const texte = (cle: number) => props.elements.find(e => e.cle === cle)?.texte ?? ''
const bienPlace = (cle: number, rang: number) => props.jouee?.ordre?.[rang] === cle
</script>
