<script setup lang="ts">
/**
 * La réponse attendue d'une épreuve, quel que soit son type (feature 014) :
 * de quoi trancher en revue sans ouvrir l'épreuve.
 */
import { LIBELLES_TYPE_REPONSE, type ElementOrdre, type PaireSaisie } from '~/composables/useAdminJeu'
import type { TypeReponse } from '~/composables/useJeu'

defineProps<{
  typeReponse: TypeReponse
  propositions: string[]
  bonneReponse: number | null
  paysNom?: string | null
  elements?: ElementOrdre[] | null
  paires?: PaireSaisie[] | null
}>()
</script>

<template>
  <div class="mt-2">
    <span v-if="typeReponse !== 'choix'" class="badge badge-outline badge-sm mb-2">{{ LIBELLES_TYPE_REPONSE[typeReponse] }}</span>

    <ul v-if="typeReponse === 'choix'" class="flex flex-wrap gap-2">
      <li
        v-for="(proposition, index) in propositions"
        :key="index"
        class="badge"
        :class="index + 1 === bonneReponse ? 'badge-success' : 'badge-ghost'"
      >
        {{ proposition }}
      </li>
    </ul>

    <p v-else-if="typeReponse === 'carte'" class="text-sm">
      Pays attendu : <span class="badge badge-success">{{ paysNom ?? '—' }}</span>
    </p>

    <ol v-else-if="typeReponse === 'ordre' && elements" class="list-inside list-decimal text-sm">
      <li v-for="(e, i) in elements" :key="i">
        {{ e.texte }}<span v-if="e.valeur" class="text-base-content/60"> — {{ e.valeur }}</span>
      </li>
    </ol>

    <ul v-else-if="typeReponse === 'paires' && paires" class="text-sm">
      <li v-for="(p, i) in paires" :key="i">{{ p.gauche }} <span class="text-base-content/50">→</span> {{ p.droite }}</li>
    </ul>
  </div>
</template>
