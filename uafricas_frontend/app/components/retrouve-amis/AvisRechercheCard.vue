<script setup lang="ts">
interface Pays {
  id: string
  nom: string
}

interface AvisRecherche {
  id: string
  nom_recherche: string
  prenom_recherche: string | null
  surnom: string | null
  ecole: string | null
  ville: string | null
  pays: Pays | null
  periode_debut: number | null
  periode_fin: number | null
  etat: 'actif' | 'cloture' | 'suspendu'
  nb_correspondances: number
  created_at: string
  updated_at: string
}

const props = defineProps<{
  avis: AvisRecherche
}>()

const emit = defineEmits<{
  modifier: [id: string]
  cloturer: [id: string]
  voir: [id: string]
}>()

const nomComplet = computed(() => {
  const parties = [props.avis.prenom_recherche, props.avis.nom_recherche].filter(Boolean)
  return parties.join(' ')
})

const periodeLabel = computed(() => {
  const { periode_debut, periode_fin } = props.avis
  if (periode_debut && periode_fin) return `${periode_debut} - ${periode_fin}`
  if (periode_debut) return `Depuis ${periode_debut}`
  if (periode_fin) return `Jusqu'en ${periode_fin}`
  return null
})

const etatConfig = computed(() => {
  const configs: Record<string, { label: string; classes: string }> = {
    actif: { label: 'Actif', classes: 'bg-af-vert/10 text-af-vert' },
    cloture: { label: 'Cloture', classes: 'bg-af-fond text-af-atone' },
    suspendu: { label: 'Suspendu', classes: 'bg-af-live/10 text-af-live' },
  }
  return configs[props.avis.etat] ?? configs.actif
})

const dateCreation = computed(() => {
  return new Date(props.avis.created_at).toLocaleDateString('fr-FR', {
    day: 'numeric',
    month: 'long',
    year: 'numeric',
  })
})

const estActif = computed(() => props.avis.etat === 'actif')

function onModifier(event: Event) {
  event.stopPropagation()
  emit('modifier', props.avis.id)
}

function onCloturer(event: Event) {
  event.stopPropagation()
  emit('cloturer', props.avis.id)
}
</script>

<template>
  <article
    class="bg-white rounded-2xl shadow-md hover:shadow-lg transition-all duration-300 p-5 cursor-pointer border border-af-bordure"
    @click="emit('voir', avis.id)"
  >
    <!-- En-tete : nom + badge etat -->
    <div class="flex items-start justify-between gap-3 mb-3">
      <h3 class="text-lg font-bold text-af-encre leading-tight">
        {{ nomComplet }}
        <span v-if="avis.surnom" class="text-sm font-normal text-af-atone-2 ml-1">
          ({{ avis.surnom }})
        </span>
      </h3>
      <span
        class="shrink-0 text-xs font-semibold px-2.5 py-1 rounded-full"
        :class="etatConfig.classes"
      >
        {{ etatConfig.label }}
      </span>
    </div>

    <!-- Criteres (chips) -->
    <div class="flex flex-wrap gap-2 mb-4">
      <span v-if="avis.ville" class="inline-flex items-center gap-1.5 text-xs text-af-corps bg-af-fond rounded-full px-3 py-1">
        <font-awesome-icon icon="location-dot" class="text-af-chocolat/70" />
        {{ avis.ville }}
      </span>
      <span v-if="avis.ecole" class="inline-flex items-center gap-1.5 text-xs text-af-corps bg-af-fond rounded-full px-3 py-1">
        <font-awesome-icon icon="graduation-cap" class="text-af-chocolat/70" />
        {{ avis.ecole }}
      </span>
      <span v-if="avis.pays" class="inline-flex items-center gap-1.5 text-xs text-af-corps bg-af-fond rounded-full px-3 py-1">
        <font-awesome-icon icon="earth-africa" class="text-af-chocolat/70" />
        {{ avis.pays.nom }}
      </span>
      <span v-if="periodeLabel" class="inline-flex items-center gap-1.5 text-xs text-af-corps bg-af-fond rounded-full px-3 py-1">
        <font-awesome-icon icon="calendar" class="text-af-chocolat/70" />
        {{ periodeLabel }}
      </span>
    </div>

    <!-- Pied : correspondances + date + actions -->
    <div class="flex items-center justify-between gap-3">
      <div class="flex items-center gap-3">
        <span class="text-xs font-semibold text-af-chocolat bg-af-chocolat/10 rounded-full px-2.5 py-1">
          {{ avis.nb_correspondances }} correspondance{{ avis.nb_correspondances > 1 ? 's' : '' }}
        </span>
        <span class="text-xs text-af-atone-2">{{ dateCreation }}</span>
      </div>

      <div v-if="estActif" class="flex items-center gap-1">
        <button
          class="text-af-chocolat hover:bg-af-chocolat/10 rounded-lg px-3 py-1 text-sm transition-colors"
          @click="onModifier"
        >
          Modifier
        </button>
        <button
          class="text-af-live hover:bg-af-live/5 rounded-lg px-3 py-1 text-sm transition-colors"
          @click="onCloturer"
        >
          Cloturer
        </button>
      </div>
    </div>
  </article>
</template>
