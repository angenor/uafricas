<script setup lang="ts">
import { ref, watch } from 'vue'
import { useRechercheArbre, type ResultatRecherche } from '~/composables/useRechercheArbre'
import { useDecouvertes } from '~/composables/useDecouvertes'
import type { NoeudArbre } from '~/composables/useLayoutArbre'

const props = defineProps<{
  graphe: Map<string, NoeudArbre>
}>()

const emit = defineEmits<{
  'resultat-selectionne': [rattachementId: string]
}>()

const { rechercherLocal } = useRechercheArbre()
const { rechercherPublique } = useDecouvertes()

const terme = ref('')
const modePublic = ref(false)
const resultats = ref<ResultatRecherche[]>([])
const chargement = ref(false)
const ouvert = ref(false)

let timerId: ReturnType<typeof setTimeout> | null = null

watch(terme, (val) => {
  if (timerId) clearTimeout(timerId)
  if (!val || val.length < 2) {
    resultats.value = []
    ouvert.value = false
    return
  }

  const delai = modePublic.value ? 500 : 300
  timerId = setTimeout(async () => {
    if (modePublic.value) {
      chargement.value = true
      try {
        const rep = await rechercherPublique(val)
        if (rep.success && rep.data) {
          resultats.value = rep.data.resultats.map((r: any) => ({
            id: '',
            personne_id: '',
            nom: r.nom,
            prenoms: r.prenoms,
            naissance_annee: r.naissance_annee,
            naissance_lieu: r.naissance_lieu,
            genre: r.genre,
            source: 'autre_arbre' as const,
          }))
        }
      } finally {
        chargement.value = false
      }
    } else {
      resultats.value = rechercherLocal(props.graphe, val)
    }
    ouvert.value = resultats.value.length > 0
  }, delai)
})

watch(modePublic, () => {
  if (terme.value.length >= 2) {
    // Re-trigger search when mode changes
    const val = terme.value
    terme.value = ''
    nextTick(() => { terme.value = val })
  }
})

const surSelection = (resultat: ResultatRecherche) => {
  if (resultat.source === 'mon_arbre' && resultat.id) {
    emit('resultat-selectionne', resultat.id)
  }
  ouvert.value = false
  terme.value = ''
}

const fermer = () => {
  ouvert.value = false
}
</script>

<template>
  <div class="relative">
    <!-- Toggle + Champ -->
    <div class="flex items-center gap-1 rounded-lg border border-stone-300 bg-white px-2 py-1 focus-within:ring-2 focus-within:ring-[var(--color-custom-chocolat)]/30">
      <!-- Toggle Mon arbre / Tous -->
      <div class="flex shrink-0 items-center gap-0.5 rounded bg-stone-100 p-0.5 text-[10px]">
        <button
          :class="[
            'rounded px-1.5 py-0.5 font-medium transition-all',
            !modePublic ? 'bg-white text-stone-800 shadow-sm' : 'text-stone-500',
          ]"
          @click="modePublic = false"
        >
          Mon arbre
        </button>
        <button
          :class="[
            'rounded px-1.5 py-0.5 font-medium transition-all',
            modePublic ? 'bg-[var(--color-custom-green)] text-white shadow-sm' : 'text-stone-500',
          ]"
          @click="modePublic = true"
        >
          Tous
        </button>
      </div>

      <!-- Input -->
      <input
        v-model="terme"
        type="text"
        placeholder="Rechercher..."
        class="w-32 border-none bg-transparent text-sm text-stone-800 placeholder-stone-400 outline-none focus:w-48 transition-all max-sm:w-24"
        @focus="ouvert = resultats.length > 0"
        @blur="setTimeout(fermer, 200)"
      />

      <!-- Spinner -->
      <div v-if="chargement" class="h-4 w-4 animate-spin rounded-full border-2 border-stone-200 border-t-stone-500" />
    </div>

    <!-- Dropdown résultats -->
    <div
      v-if="ouvert && resultats.length > 0"
      class="absolute left-0 top-full z-50 mt-1 max-h-64 w-80 overflow-y-auto rounded-xl border border-stone-200 bg-white shadow-lg"
    >
      <button
        v-for="(r, i) in resultats"
        :key="i"
        class="flex w-full items-center gap-3 border-b border-stone-50 px-3 py-2 text-left transition-colors last:border-b-0 hover:bg-stone-50"
        @mousedown.prevent="surSelection(r)"
      >
        <div class="min-w-0 flex-1">
          <p class="truncate text-sm font-semibold text-stone-800">
            {{ r.prenoms }} {{ r.nom }}
          </p>
          <p class="truncate text-xs text-stone-500">
            <span v-if="r.naissance_annee">{{ r.naissance_annee }}</span>
            <span v-if="r.naissance_lieu"> — {{ r.naissance_lieu }}</span>
          </p>
        </div>
        <span
          :class="[
            'shrink-0 rounded-full px-2 py-0.5 text-[10px] font-medium',
            r.source === 'mon_arbre' ? 'bg-stone-100 text-stone-600' : 'bg-[var(--color-custom-green)]/10 text-[var(--color-custom-green)]',
          ]"
        >
          {{ r.source === 'mon_arbre' ? 'Votre arbre' : 'Autre arbre' }}
        </span>
      </button>
    </div>

    <!-- Aucun résultat -->
    <div
      v-if="ouvert && resultats.length === 0 && terme.length >= 2 && !chargement"
      class="absolute left-0 top-full z-50 mt-1 w-80 rounded-xl border border-stone-200 bg-white p-4 text-center text-sm text-stone-500 shadow-lg"
    >
      Aucun résultat pour « {{ terme }} »
    </div>
  </div>
</template>
