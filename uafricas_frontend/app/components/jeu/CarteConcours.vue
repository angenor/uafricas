<template>
  <NuxtLink
    :to="`/activites/concours/${concours.id}`"
    class="group flex flex-col overflow-hidden rounded-[10px] border border-af-bordure bg-af-surface transition hover:border-af-chocolat focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-af-chocolat"
  >
    <div class="relative grid aspect-[16/7] place-items-center bg-af-chocolat/[0.08]">
      <img v-if="visuel" :src="visuel" alt="" class="absolute inset-0 size-full object-cover">
      <font-awesome-icon v-else icon="fa-solid fa-images" class="text-4xl text-af-chocolat/50" />
      <span class="absolute left-3 top-3 rounded-full px-3 py-1 text-[12px] font-bold" :class="classePhase">
        {{ LIBELLES_PHASE_PUBLIQUE[concours.phase] }}
      </span>
    </div>
    <div class="flex flex-1 flex-col gap-2 p-4">
      <p class="text-[12px]/[1.3] font-bold uppercase tracking-wide text-af-atone">
        Bataille de photos<template v-if="concours.rattachement"> · {{ libelleModulePlateforme(concours.rattachement) }}</template>
      </p>
      <h3 class="text-[17px]/[1.35] font-bold text-af-encre group-hover:text-af-chocolat">{{ concours.titre }}</h3>
      <p class="line-clamp-2 text-[14px]/[1.5] text-af-corps">{{ concours.theme }}</p>
      <p class="mt-auto flex items-center gap-2 pt-2 text-[13px]/[1.4] text-af-corps">
        <font-awesome-icon icon="fa-solid fa-hourglass-half" class="text-af-atone" />
        {{ echeance }}
        <span class="ml-auto">{{ concours.participations_publiees }} photo{{ concours.participations_publiees > 1 ? 's' : '' }}</span>
      </p>
    </div>
  </NuxtLink>
</template>

<script setup lang="ts">
import { LIBELLES_PHASE_PUBLIQUE, type ConcoursPublicAPI } from '~/composables/useConcours'
import { libelleModulePlateforme } from '~/utils/modulesPlateforme'

/** Un concours dans une liste (feature 014) : sa phase et sa prochaine échéance. */
const props = defineProps<{ concours: ConcoursPublicAPI }>()

const visuel = computed(() => urlMedia(props.concours.image_url))

const classePhase = computed(() => ({
  appel: 'bg-af-orange text-white',
  vote: 'bg-af-vert text-white',
  deliberation: 'bg-af-chocolat text-white',
  resultats: 'bg-af-encre text-white',
  annule: 'bg-af-bordure text-af-corps',
}[props.concours.phase]))

const dans = (iso: string) => {
  const ms = new Date(iso).getTime() - Date.now()
  const jours = Math.floor(ms / 86400000)
  if (jours >= 2) return `dans ${jours} jours`
  const heures = Math.max(1, Math.round(ms / 3600000))
  return `dans ${heures} h`
}

const echeance = computed(() => {
  const c = props.concours
  if (c.phase === 'appel') return `Dépôts jusqu'au vote, ${dans(c.vote_debut)}`
  if (c.phase === 'vote') return `Résultats ${dans(c.vote_fin)}`
  if (c.phase === 'deliberation') return 'Le jury délibère'
  if (c.phase === 'resultats') return 'Résultats publiés'
  return 'Concours annulé'
})
</script>
