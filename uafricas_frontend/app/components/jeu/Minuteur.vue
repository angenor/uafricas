<template>
  <div
    class="flex items-center gap-3"
    role="timer"
    :aria-label="`${secondes} seconde${secondes > 1 ? 's' : ''} restante${secondes > 1 ? 's' : ''}`"
  >
    <div class="h-2 flex-1 overflow-hidden rounded-full bg-af-bordure">
      <div
        class="h-full rounded-full transition-[width] duration-200 ease-linear"
        :class="urgent ? 'bg-af-live' : 'bg-af-chocolat'"
        :style="{ width: `${fraction * 100}%` }"
      />
    </div>
    <span
      class="w-10 shrink-0 text-right text-[14px]/[1.4] font-bold tabular-nums"
      :class="urgent ? 'text-af-live' : 'text-af-encre'"
    >{{ secondes }} s</span>
  </div>
</template>

<script setup lang="ts">
/**
 * Compte à rebours d'une épreuve.
 *
 * Il se cale sur l'horloge du SERVEUR : `maintenant` est l'heure du serveur à
 * l'instant où il a délivré l'épreuve, `expireA` l'instant où le temps
 * s'achève. On en déduit le décalage avec l'horloge locale une fois pour
 * toutes ; un appareil dont l'horloge est fausse affiche quand même le bon
 * temps restant. C'est le serveur qui tranche de toute façon : ce minuteur
 * n'est qu'un affichage.
 */
const props = defineProps<{
  expireA: string
  maintenant: string
  /** Durée totale de l'épreuve, pour la jauge. */
  dureeS: number
  /** Fige l'affichage (réponse envoyée, correction affichée). */
  arrete?: boolean
}>()

const emit = defineEmits<{ expire: [] }>()

const restantMs = ref(0)
let minuterie: ReturnType<typeof setInterval> | null = null
let decalageMs = 0
let dejaExpire = false

const calculer = () => {
  const reste = new Date(props.expireA).getTime() - (Date.now() + decalageMs)
  restantMs.value = Math.max(0, reste)
  if (reste <= 0 && !dejaExpire && !props.arrete) {
    dejaExpire = true
    arreter()
    emit('expire')
  }
}

const arreter = () => {
  if (minuterie) {
    clearInterval(minuterie)
    minuterie = null
  }
}

const demarrer = () => {
  arreter()
  dejaExpire = false
  decalageMs = new Date(props.maintenant).getTime() - Date.now()
  calculer()
  if (!props.arrete) minuterie = setInterval(calculer, 200)
}

watch(() => [props.expireA, props.maintenant], demarrer)
watch(() => props.arrete, (arrete) => {
  if (arrete) arreter()
})

onMounted(demarrer)
onBeforeUnmount(arreter)

const secondes = computed(() => Math.ceil(restantMs.value / 1000))
// La jauge se mesure sur le temps réellement accordé (une épreuve à média a une
// marge de chargement en plus), sans jamais dépasser 100 %.
const totalMs = computed(() =>
  Math.max(props.dureeS * 1000, new Date(props.expireA).getTime() - new Date(props.maintenant).getTime()),
)
const fraction = computed(() => Math.min(1, restantMs.value / totalMs.value))
const urgent = computed(() => secondes.value <= 5)
</script>
