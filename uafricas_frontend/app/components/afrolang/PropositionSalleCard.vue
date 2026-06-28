<template>
  <article
    class="bg-white rounded-xl border border-gray-200 shadow-sm hover:shadow-md transition-shadow p-5"
  >
    <header class="flex items-start justify-between gap-3">
      <div>
        <h3 class="text-base font-semibold text-gray-900 line-clamp-2">{{ proposition.titre }}</h3>
        <p class="mt-1 text-xs text-gray-500">
          {{ proposition.langue_cible }}<span v-if="proposition.langue_code"> · {{ proposition.langue_code }}</span>
          · {{ proposition.groupe_ethnique?.nom ?? proposition.groupe_ethnique_libre }}
        </p>
      </div>
      <span
        class="shrink-0 inline-flex items-center gap-1 px-2.5 py-1 rounded-full text-[11px] font-medium border"
        :class="badge.classes"
      >
        <font-awesome-icon :icon="['fas', badge.icone]" class="w-3 h-3" />
        {{ badge.label }}
      </span>
    </header>

    <p v-if="proposition.description" class="mt-3 text-sm text-gray-600 line-clamp-3">
      {{ proposition.description }}
    </p>

    <div v-if="proposition.pays_origine.length" class="mt-3 flex flex-wrap gap-1.5">
      <span
        v-for="pays in proposition.pays_origine"
        :key="pays.id"
        class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full bg-gray-100 text-gray-700 text-[11px]"
      >
        <span v-if="pays.code_iso2">{{ drapeauEmoji(pays.code_iso2) }}</span>
        {{ pays.nom }}
      </span>
    </div>

    <div
      v-if="proposition.commentaire_decision"
      class="mt-3 rounded-lg border px-3 py-2 text-xs"
      :class="proposition.statut === 'rejetee'
        ? 'bg-red-50 border-red-200 text-red-800'
        : 'bg-emerald-50 border-emerald-200 text-emerald-800'"
    >
      <p class="font-medium mb-0.5">Commentaire de l'administrateur</p>
      <p>{{ proposition.commentaire_decision }}</p>
    </div>

    <footer class="mt-4 flex items-center justify-between gap-3">
      <p class="text-[11px] text-gray-500">
        Soumise le {{ formatDate(proposition.created_at) }}
        <span v-if="proposition.decide_at">
          · Décidée le {{ formatDate(proposition.decide_at) }}
        </span>
      </p>
      <button
        v-if="proposition.statut === 'en_attente'"
        type="button"
        :disabled="enRetrait"
        class="inline-flex items-center gap-1.5 rounded-lg border border-gray-300 bg-white px-3 py-1.5 text-xs font-medium text-gray-700 hover:bg-gray-50 disabled:opacity-50"
        @click="retirer"
      >
        <font-awesome-icon
          :icon="['fas', enRetrait ? 'spinner' : 'rotate-left']"
          :class="enRetrait ? 'animate-spin' : ''"
          class="w-3 h-3"
        />
        {{ enRetrait ? 'Retrait…' : 'Retirer' }}
      </button>
    </footer>
  </article>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import type { PropositionSalle, StatutProposition } from '~/composables/useAfrolang'

const props = defineProps<{
  proposition: PropositionSalle
}>()

const emit = defineEmits<{
  (e: 'retiree', proposition: PropositionSalle): void
}>()

const { retirerProposition } = useAfrolang()
const enRetrait = ref(false)

const badge = computed(() => badgePourStatut(props.proposition.statut))

function badgePourStatut(statut: StatutProposition) {
  switch (statut) {
    case 'en_attente':
      return { label: 'En attente', icone: 'clock', classes: 'bg-amber-50 text-amber-800 border-amber-200' }
    case 'validee':
      return { label: 'Validée', icone: 'circle-check', classes: 'bg-emerald-50 text-emerald-800 border-emerald-200' }
    case 'rejetee':
      return { label: 'Rejetée', icone: 'circle-xmark', classes: 'bg-red-50 text-red-800 border-red-200' }
    case 'retiree':
      return { label: 'Retirée', icone: 'rotate-left', classes: 'bg-gray-100 text-gray-700 border-gray-200' }
  }
}

const formatDate = (iso: string) => new Date(iso).toLocaleDateString('fr-FR', {
  day: 'numeric', month: 'long', year: 'numeric',
})

const drapeauEmoji = (code: string): string => {
  if (!code || code.length !== 2) return ''
  const upper = code.toUpperCase()
  const base = 0x1F1E6 - 65
  return String.fromCodePoint(base + upper.charCodeAt(0)) +
         String.fromCodePoint(base + upper.charCodeAt(1))
}

const retirer = async () => {
  if (!confirm('Voulez-vous vraiment retirer cette proposition ?')) return
  enRetrait.value = true
  try {
    const proposition = await retirerProposition(props.proposition.id)
    if (proposition) emit('retiree', proposition)
  }
  finally {
    enRetrait.value = false
  }
}
</script>
