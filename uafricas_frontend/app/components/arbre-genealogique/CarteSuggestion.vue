<script setup lang="ts">
import { computed } from 'vue'
import { formaterScore, couleurScore, libelleStatut } from '~/mocks/matching'
import type { SuggestionCorrespondance } from '~/mocks/matching'

const props = defineProps<{
  suggestion: SuggestionCorrespondance
}>()

const emit = defineEmits<{
  confirmer: [id: string]
  rejeter: [id: string]
}>()

const scorePercent = computed(() => Math.round(props.suggestion.score * 100))
const scoreCouleur = computed(() => couleurScore(props.suggestion.score))

const criteresResume = computed(() => {
  const d = props.suggestion.details_score
  const criteres = []
  if (d.nom >= 0.8) criteres.push('Nom identique')
  else if (d.nom >= 0.5) criteres.push('Nom similaire')
  if (d.lieu >= 0.8) criteres.push('Même lieu')
  else if (d.lieu >= 0.5) criteres.push('Lieu proche')
  if (d.date >= 0.8) criteres.push('Dates compatibles')
  if (d.genre >= 1.0) criteres.push('Même genre')
  return criteres
})

const peutAgir = computed(() => {
  return props.suggestion.statut === 'en_attente'
})
</script>

<template>
  <div class="rounded-xl border border-stone-200 bg-white p-4 shadow-sm transition-shadow hover:shadow-md">
    <!-- En-tête : score + statut -->
    <div class="mb-3 flex items-center justify-between">
      <div class="flex items-center gap-2">
        <div
          :class="['flex h-10 w-10 items-center justify-center rounded-full text-sm font-bold', scoreCouleur]"
          :style="{ background: `conic-gradient(currentColor ${scorePercent * 3.6}deg, #e7e5e4 0deg)` }"
        >
          <span class="flex h-8 w-8 items-center justify-center rounded-full bg-white text-xs">
            {{ scorePercent }}%
          </span>
        </div>
        <span class="text-xs text-stone-500">{{ suggestion.membre_id_anonymise }}</span>
      </div>
      <span
        :class="[
          'rounded-full px-2.5 py-0.5 text-xs font-medium',
          suggestion.statut === 'confirmee' ? 'bg-green-100 text-green-700' :
          suggestion.statut === 'confirmee_de_mon_cote' ? 'bg-amber-100 text-amber-700' :
          'bg-stone-100 text-stone-600'
        ]"
      >
        {{ libelleStatut(suggestion.statut) }}
      </span>
    </div>

    <!-- Personnes comparées -->
    <div class="mb-3 flex items-center gap-3">
      <!-- Ma personne -->
      <div class="flex-1 rounded-lg bg-stone-50 p-2.5">
        <p class="text-xs font-semibold text-stone-400">Mon arbre</p>
        <p class="text-sm font-bold text-stone-800">
          {{ suggestion.ma_personne.prenoms }} {{ suggestion.ma_personne.nom }}
        </p>
        <p v-if="suggestion.ma_personne.naissance_annee" class="text-xs text-stone-500">
          {{ suggestion.ma_personne.naissance_annee }}
          <span v-if="suggestion.ma_personne.naissance_lieu"> — {{ suggestion.ma_personne.naissance_lieu }}</span>
        </p>
      </div>

      <!-- Icône lien -->
      <div class="shrink-0 text-stone-300">
        <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4" />
        </svg>
      </div>

      <!-- Personne matchée -->
      <div class="flex-1 rounded-lg bg-[var(--color-custom-green)]/5 p-2.5">
        <p class="text-xs font-semibold text-[var(--color-custom-green)]">Autre arbre</p>
        <p class="text-sm font-bold text-stone-800">
          {{ suggestion.personne_matchee.prenoms }} {{ suggestion.personne_matchee.nom }}
        </p>
        <p v-if="suggestion.personne_matchee.naissance_annee" class="text-xs text-stone-500">
          {{ suggestion.personne_matchee.naissance_annee }}
          <span v-if="suggestion.personne_matchee.naissance_lieu"> — {{ suggestion.personne_matchee.naissance_lieu }}</span>
        </p>
      </div>
    </div>

    <!-- Critères -->
    <div class="mb-3 flex flex-wrap gap-1.5">
      <span
        v-for="critere in criteresResume"
        :key="critere"
        class="rounded-full bg-stone-100 px-2 py-0.5 text-xs text-stone-600"
      >
        {{ critere }}
      </span>
    </div>

    <!-- Actions -->
    <div v-if="peutAgir" class="flex gap-2">
      <button
        class="flex-1 rounded-lg bg-[var(--color-custom-green)] px-4 py-2 text-sm font-semibold text-white transition-colors hover:bg-[var(--color-custom-green)]/90"
        @click="emit('confirmer', suggestion.id)"
      >
        Confirmer
      </button>
      <button
        class="flex-1 rounded-lg border border-stone-300 px-4 py-2 text-sm font-semibold text-stone-600 transition-colors hover:bg-stone-50"
        @click="emit('rejeter', suggestion.id)"
      >
        Rejeter
      </button>
    </div>

    <!-- Message en attente -->
    <p v-else-if="suggestion.statut === 'confirmee_de_mon_cote'" class="text-center text-xs text-amber-600">
      En attente de confirmation de l'autre membre
    </p>
  </div>
</template>
