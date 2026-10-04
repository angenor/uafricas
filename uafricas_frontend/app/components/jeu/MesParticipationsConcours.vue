<template>
  <section v-if="participations.length" class="rounded-[10px] border border-af-bordure bg-af-surface p-6">
    <h2 class="text-[17px]/[1.4] font-bold text-af-encre">Mes photos de concours</h2>
    <ul class="mt-4 flex flex-col divide-y divide-af-bordure" role="list">
      <li v-for="p in participations" :key="p.id" class="flex items-center gap-4 py-3">
        <img :src="urlMedia(p.media_url) ?? undefined" alt="" class="size-14 shrink-0 rounded-lg object-cover">
        <div class="min-w-0 flex-1">
          <NuxtLink :to="`/activites/concours/${p.concours_id}`" class="font-bold text-af-encre hover:text-af-chocolat">{{ p.concours_titre }}</NuxtLink>
          <p class="text-[13px]/[1.4] text-af-corps">
            {{ LIBELLES_MA_PARTICIPATION[p.etat] }}
            <template v-if="p.phase"> · {{ libellePhase(p.phase) }}</template>
            <template v-if="p.motif_rejet"> · motif : {{ p.motif_rejet }}</template>
          </p>
        </div>
        <div class="shrink-0 text-right text-[13px]/[1.4]">
          <p v-if="p.rang" class="font-bold text-af-encre">{{ p.rang }}<sup>{{ p.rang === 1 ? 're' : 'e' }}</sup> place</p>
          <p v-if="p.taux != null" class="text-af-atone">{{ p.taux }} % de duels gagnés</p>
          <p v-if="p.gain" class="font-bold text-af-vert">+{{ p.gain }}</p>
        </div>
      </li>
    </ul>
  </section>
</template>

<script setup lang="ts">
import {
  LIBELLES_MA_PARTICIPATION,
  LIBELLES_PHASE_PUBLIQUE,
  type ParticipationHistoriqueAPI,
} from '~/composables/useConcours'

/** Les participations du membre à des concours, dans « Mes activités » (FR-059). */
const { mesParticipations } = useConcours()
const participations = ref<ParticipationHistoriqueAPI[]>([])

const libellePhase = (phase: string) =>
  LIBELLES_PHASE_PUBLIQUE[phase as keyof typeof LIBELLES_PHASE_PUBLIQUE] ?? ''

onMounted(async () => {
  try {
    participations.value = (await mesParticipations()) ?? []
  }
  catch {
    participations.value = []
  }
})
</script>
