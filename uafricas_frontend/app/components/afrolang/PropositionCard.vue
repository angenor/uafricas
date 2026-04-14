<template>
  <article
    class="bg-white rounded-xl border border-gray-200 shadow-sm hover:shadow-md transition p-5"
  >
    <header class="flex items-start justify-between gap-3 mb-3">
      <div class="min-w-0">
        <h3 class="text-lg font-bold text-gray-900 truncate">
          {{ proposition.nom_groupe_ethnique }}
        </h3>
        <p v-if="proposition.langue_cible" class="text-sm text-gray-600 mt-0.5">
          Langue : <span class="font-medium text-gray-800">{{ proposition.langue_cible }}</span>
        </p>
      </div>
      <span
        :class="[
          'inline-flex items-center gap-1 px-2.5 py-0.5 rounded-full text-xs font-medium whitespace-nowrap',
          badgeClasses,
        ]"
      >
        <font-awesome-icon :icon="badgeIcon" class="text-[10px]" />
        {{ badgeLabel }}
      </span>
    </header>

    <p v-if="proposition.description" class="text-sm text-gray-700 mb-3 line-clamp-3">
      {{ proposition.description }}
    </p>

    <div
      v-if="proposition.etat === 'refusee' && proposition.motif_refus"
      class="text-sm border border-red-200 bg-red-50 text-red-800 rounded-lg p-2.5 mb-3"
    >
      <p class="font-semibold mb-0.5">Motif du refus</p>
      <p>{{ proposition.motif_refus }}</p>
    </div>

    <footer class="flex items-center justify-between text-xs text-gray-500">
      <span>Soumise le {{ formatDate(proposition.created_at) }}</span>
      <NuxtLink
        v-if="proposition.etat === 'approuvee' && proposition.salle_id_creee"
        :to="`/afrolang/${proposition.salle_id_creee}`"
        class="inline-flex items-center gap-1 text-custom-chocolat hover:underline font-semibold"
      >
        Accéder à la salle
        <font-awesome-icon icon="arrow-right" class="text-[10px]" />
      </NuxtLink>
    </footer>
  </article>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { PropositionSalleAPI } from '~/composables/useAfrolang'
import { formatDate } from '~/composables/useAfrolang'

interface Props {
  proposition: PropositionSalleAPI
}

const props = defineProps<Props>()

const badgeClasses = computed(() => {
  switch (props.proposition.etat) {
    case 'en_attente':
      return 'bg-amber-100 text-amber-900 border border-amber-200'
    case 'approuvee':
      return 'bg-green-100 text-green-900 border border-green-200'
    case 'refusee':
      return 'bg-red-100 text-red-900 border border-red-200'
    default:
      return 'bg-gray-100 text-gray-800 border border-gray-200'
  }
})

const badgeLabel = computed(() => {
  switch (props.proposition.etat) {
    case 'en_attente':
      return 'En attente'
    case 'approuvee':
      return 'Validée'
    case 'refusee':
      return 'Refusée'
    default:
      return props.proposition.etat
  }
})

const badgeIcon = computed(() => {
  switch (props.proposition.etat) {
    case 'en_attente':
      return 'clock'
    case 'approuvee':
      return 'circle-check'
    case 'refusee':
      return 'circle-xmark'
    default:
      return 'circle-info'
  }
})
</script>
