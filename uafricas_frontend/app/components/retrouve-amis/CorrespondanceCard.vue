<template>
  <div
    class="bg-white rounded-2xl shadow-md hover:shadow-lg transition-all p-5 cursor-pointer border border-af-bordure"
    @click="$emit('voir', correspondance.id)"
  >
    <!-- En-tete : initiales + score + etat -->
    <div class="flex items-start justify-between mb-4">
      <div class="w-12 h-12 rounded-full bg-af-chocolat/10 text-af-chocolat font-bold flex items-center justify-center text-lg">
        {{ correspondance.resume_anonymise.initiales }}
      </div>
      <div class="flex items-center gap-2">
        <span
          class="text-xs px-2 py-0.5 rounded-full font-medium"
          :class="etatClasses"
        >
          {{ etatLabel }}
        </span>
        <RetrouveAmisScoreBadge :score="correspondance.score" taille="md" />
      </div>
    </div>

    <!-- Informations -->
    <div class="space-y-2 mb-4">
      <!-- Ville -->
      <div v-if="correspondance.resume_anonymise.ville" class="flex items-center text-sm text-af-corps">
        <font-awesome-icon :icon="['fas', 'location-dot']" class="w-3.5 h-3.5 mr-2 text-af-vert" />
        <span>{{ correspondance.resume_anonymise.ville }}</span>
      </div>

      <!-- Periode -->
      <div v-if="correspondance.resume_anonymise.periode" class="flex items-center text-sm text-af-corps">
        <font-awesome-icon :icon="['fas', 'calendar']" class="w-3.5 h-3.5 mr-2 text-af-chocolat" />
        <span>{{ correspondance.resume_anonymise.periode }}</span>
      </div>

      <!-- Badges type_cible et mon_role -->
      <div class="flex items-center gap-2 mt-1">
        <span
          class="text-xs px-2 py-0.5 rounded-full font-medium"
          :class="correspondance.type_cible === 'avis'
            ? 'bg-af-chocolat/5 text-af-chocolat'
            : 'bg-violet-50 text-violet-700'"
        >
          {{ correspondance.type_cible === 'avis' ? 'Avis' : 'Profil' }}
        </span>
        <span
          class="text-xs px-2 py-0.5 rounded-full font-medium"
          :class="correspondance.mon_role === 'auteur'
            ? 'bg-af-chocolat/10 text-af-chocolat'
            : 'bg-sky-50 text-sky-700'"
        >
          {{ correspondance.mon_role === 'auteur' ? 'Auteur' : 'Cible' }}
        </span>
      </div>
    </div>

    <!-- Criteres communs -->
    <div
      v-if="correspondance.resume_anonymise.criteres_communs?.length"
      class="flex flex-wrap gap-1.5 mb-4"
    >
      <span
        v-for="critere in correspondance.resume_anonymise.criteres_communs"
        :key="critere"
        class="bg-af-vert/10 text-af-vert text-xs px-2 py-0.5 rounded-full"
      >
        {{ critere }}
      </span>
    </div>

    <!-- Pied : expiration -->
    <div
      v-if="correspondance.expire_at"
      class="pt-3 border-t border-af-bordure flex items-center text-xs text-af-atone-2"
    >
      <font-awesome-icon :icon="['fas', 'clock']" class="w-3 h-3 mr-1.5" />
      Expire le {{ formaterDate(correspondance.expire_at) }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface ResumeAnonymise {
  initiales: string
  ville: string | null
  periode: string | null
  criteres_communs: string[]
}

interface Correspondance {
  id: string
  avis_id: string
  score: number
  etat: string
  type_cible: string
  resume_anonymise: ResumeAnonymise
  mon_role: string
  created_at: string
  expire_at: string | null
}

const props = defineProps<{
  correspondance: Correspondance
}>()

defineEmits<{
  voir: [id: string]
}>()

const etatClasses = computed(() => {
  const classes: Record<string, string> = {
    en_attente: 'bg-af-chocolat/10 text-af-chocolat',
    acceptee_a: 'bg-af-chocolat/10 text-af-chocolat',
    acceptee_b: 'bg-af-chocolat/10 text-af-chocolat',
    mutuelle: 'bg-af-vert/10 text-af-vert',
    declinee: 'bg-af-live/10 text-af-live',
    archivee: 'bg-af-fond text-af-atone',
  }
  return classes[props.correspondance.etat] || 'bg-af-fond text-af-atone'
})

const etatLabel = computed(() => {
  const labels: Record<string, string> = {
    en_attente: 'En attente',
    acceptee_a: 'Acceptee',
    acceptee_b: 'Acceptee',
    mutuelle: 'Mutuelle',
    declinee: 'Declinee',
    archivee: 'Archivee',
  }
  return labels[props.correspondance.etat] || props.correspondance.etat
})

const formaterDate = (iso: string): string => {
  const date = new Date(iso)
  return date.toLocaleDateString('fr-FR', { day: 'numeric', month: 'long', year: 'numeric' })
}
</script>
