<template>
  <article class="rounded-lg border border-af-bordure bg-white p-5 transition hover:border-af-chocolat">
    <header class="flex items-start justify-between gap-3">
      <div class="min-w-0">
        <h3 class="line-clamp-2 text-base font-bold text-af-encre">{{ proposition.titre }}</h3>
        <p class="mt-1 text-[12px]/[1.4] text-af-atone">
          {{ proposition.langue_cible }}<span v-if="proposition.langue_code"> · {{ proposition.langue_code }}</span>
          · {{ proposition.groupe_ethnique?.nom ?? proposition.groupe_ethnique_libre }}
        </p>
      </div>
      <span
        class="inline-flex shrink-0 items-center gap-1 rounded-full border px-2.5 py-1 text-[11px] font-bold"
        :class="badge.classes"
      >
        <font-awesome-icon :icon="badge.icone" />
        {{ badge.label }}
      </span>
    </header>

    <p v-if="proposition.description" class="mt-3 line-clamp-3 text-[14px]/[1.6] text-af-corps">
      {{ proposition.description }}
    </p>

    <div v-if="proposition.pays_origine.length" class="mt-3 flex flex-wrap gap-1.5">
      <span
        v-for="pays in proposition.pays_origine"
        :key="pays.id"
        class="inline-flex items-center gap-1 rounded-full bg-af-fond px-2 py-0.5 text-[11px] text-af-corps"
      >
        <span v-if="pays.code_iso2">{{ drapeauEmoji(pays.code_iso2) }}</span>
        {{ pays.nom }}
      </span>
    </div>

    <div
      v-if="proposition.commentaire_decision"
      class="mt-3 rounded-lg border px-3 py-2 text-[12px]/[1.6]"
      :class="proposition.statut === 'rejetee'
        ? 'border-af-live/30 bg-af-live/5 text-af-live'
        : 'border-af-vert/30 bg-af-vert/5 text-af-vert'"
    >
      <p class="mb-0.5 font-bold">Commentaire de l'administrateur</p>
      <p>{{ proposition.commentaire_decision }}</p>
    </div>

    <footer class="mt-4 flex flex-wrap items-center justify-between gap-3">
      <p class="text-[11px] text-af-atone">
        Soumise le {{ formatDate(proposition.created_at) }}
        <span v-if="proposition.decide_at">· Décidée le {{ formatDate(proposition.decide_at) }}</span>
      </p>
      <button
        v-if="proposition.statut === 'en_attente'"
        type="button"
        :disabled="enRetrait"
        class="inline-flex items-center gap-1.5 rounded-md border border-af-bordure bg-white px-3 py-1.5 text-[12px] font-bold text-af-corps transition hover:border-af-chocolat hover:text-af-chocolat disabled:opacity-50"
        @click="retirer"
      >
        <font-awesome-icon
          :icon="enRetrait ? 'fa-solid fa-spinner' : 'fa-solid fa-rotate-left'"
          :class="enRetrait && 'animate-spin'"
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
      return { label: 'En attente', icone: 'fa-solid fa-clock', classes: 'border-af-chocolat/30 bg-af-chocolat/5 text-af-chocolat' }
    case 'validee':
      return { label: 'Validée', icone: 'fa-solid fa-circle-check', classes: 'border-af-vert/30 bg-af-vert/5 text-af-vert' }
    case 'rejetee':
      return { label: 'Rejetée', icone: 'fa-solid fa-circle-xmark', classes: 'border-af-live/30 bg-af-live/5 text-af-live' }
    case 'retiree':
      return { label: 'Retirée', icone: 'fa-solid fa-rotate-left', classes: 'border-af-bordure bg-af-fond text-af-corps' }
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
