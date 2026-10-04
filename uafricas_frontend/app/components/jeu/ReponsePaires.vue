<template>
  <div class="flex flex-col gap-4">
    <!-- Après la correction : chaque élément de gauche avec son bon
         correspondant, et ce que le membre avait relié s'il s'est trompé. -->
    <ul v-if="solution?.solution" class="flex flex-col gap-2" role="list">
      <li
        v-for="(g, i) in gaucheTriee"
        :key="g.cle"
        class="flex flex-wrap items-center gap-x-3 gap-y-1 rounded-lg border px-4 py-3 text-[15px]/[1.4]"
        :class="juste(i) ? 'border-af-vert/50 bg-af-vert/5' : 'border-af-live/30 bg-af-live/5'"
      >
        <span class="font-bold text-af-encre">{{ g.texte }}</span>
        <font-awesome-icon icon="fa-solid fa-arrow-right" class="text-af-atone" />
        <span class="font-bold" :class="juste(i) ? 'text-af-vert' : 'text-af-encre'">{{ texteDroite(solution.solution[i]!) }}</span>
        <span v-if="!juste(i) && jouee?.paires?.[i]" class="text-[13px] text-af-live">
          (vous : {{ texteDroite(jouee.paires[i]!) }})
        </span>
      </li>
    </ul>

    <template v-else>
      <p class="text-[13px]/[1.4] text-af-atone">
        Touchez un élément à gauche, puis son correspondant à droite. Touchez une paire pour la défaire.
      </p>
      <div class="grid grid-cols-2 gap-3">
        <ul class="flex flex-col gap-2" role="list" aria-label="Éléments à associer">
          <li v-for="g in gauche" :key="g.cle">
            <button
              type="button"
              class="flex min-h-12 w-full items-center gap-2 rounded-lg border px-3 py-2.5 text-left text-[15px]/[1.35] transition focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-af-chocolat"
              :class="classeGauche(g.cle)"
              :disabled="verrouillee"
              :aria-pressed="actif === g.cle"
              @click="toucherGauche(g.cle)"
            >
              <span v-if="liens[g.cle] != null" class="size-3 shrink-0 rounded-full" :class="couleurPaire(g.cle)" aria-hidden="true" />
              <span class="flex-1">{{ g.texte }}</span>
            </button>
          </li>
        </ul>
        <ul class="flex flex-col gap-2" role="list" aria-label="Correspondants">
          <li v-for="d in droite" :key="d.cle">
            <button
              type="button"
              class="flex min-h-12 w-full items-center gap-2 rounded-lg border px-3 py-2.5 text-left text-[15px]/[1.35] transition focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-af-chocolat"
              :class="classeDroite(d.cle)"
              :disabled="verrouillee || actif == null"
              @click="toucherDroite(d.cle)"
            >
              <span v-if="gaucheDe(d.cle) != null" class="size-3 shrink-0 rounded-full" :class="couleurPaire(gaucheDe(d.cle)!)" aria-hidden="true" />
              <span class="flex-1">{{ d.texte }}</span>
            </button>
          </li>
        </ul>
      </div>
      <div class="flex items-center justify-between gap-3">
        <p class="text-[13px]/[1.4] text-af-corps">{{ nbLiens }} / {{ gauche.length }} paires</p>
        <AfricansBouton icone="fa-solid fa-check" :desactive="verrouillee || nbLiens < gauche.length" @click="valider">
          Valider les paires
        </AfricansBouton>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import type { PropositionServieAPI, ReponseJoueur, SolutionAPI } from '~/composables/useJeu'

/**
 * Épreuve « paires » (feature 014) : relier 3 à 5 éléments à leur
 * correspondant. La réponse envoyée est, pour chaque clé de gauche PAR ORDRE
 * CROISSANT, la clé de droite qui lui a été associée.
 */
const props = defineProps<{
  gauche: PropositionServieAPI[]
  droite: PropositionServieAPI[]
  verrouillee?: boolean
  solution?: SolutionAPI | null
  jouee?: ReponseJoueur | null
}>()

const emit = defineEmits<{ repondre: [reponse: ReponseJoueur] }>()

/** clé de gauche → clé de droite */
const liens = ref<Record<number, number>>({})
const actif = ref<number | null>(null)
// Les clés, pas le tableau : voir ReponseOrdre (relecture du duel direct).
watch(() => props.gauche.map(g => g.cle).join(','), () => { liens.value = {}; actif.value = null })

const nbLiens = computed(() => Object.keys(liens.value).length)
const gaucheDe = (cleDroite: number): number | null => {
  const trouve = Object.entries(liens.value).find(([, d]) => d === cleDroite)
  return trouve ? Number(trouve[0]) : null
}

const toucherGauche = (cle: number) => {
  if (props.verrouillee) return
  if (liens.value[cle] != null) {
    // Toucher une paire la défait.
    const { [cle]: _, ...reste } = liens.value
    liens.value = reste
    actif.value = cle
    return
  }
  // Pas de bascule : après une paire, l'élément suivant est déjà actif, et le
  // toucher (geste naturel) ne doit pas le désactiver.
  actif.value = cle
}

const toucherDroite = (cle: number) => {
  if (props.verrouillee || actif.value == null) return
  const reste = Object.fromEntries(Object.entries(liens.value).filter(([, d]) => d !== cle))
  liens.value = { ...reste, [actif.value]: cle }
  // On enchaîne sur le prochain élément de gauche encore libre.
  actif.value = props.gauche.find(g => liens.value[g.cle] == null)?.cle ?? null
}

const valider = () => {
  if (props.verrouillee || nbLiens.value < props.gauche.length) return
  const paires = [...props.gauche].sort((a, b) => a.cle - b.cle).map(g => liens.value[g.cle]!)
  emit('repondre', { paires })
}

// Une couleur par paire, attribuée par l'ordre d'affichage de la gauche.
const PALETTE = ['bg-amber-500', 'bg-sky-500', 'bg-emerald-500', 'bg-rose-500', 'bg-violet-500']
const couleurPaire = (cleGauche: number) =>
  PALETTE[props.gauche.findIndex(g => g.cle === cleGauche) % PALETTE.length]

const classeGauche = (cle: number) => {
  if (actif.value === cle) return 'border-af-chocolat bg-af-chocolat/[0.08] font-bold text-af-encre'
  if (liens.value[cle] != null) return 'border-af-bordure bg-af-fond/60 text-af-encre'
  return props.verrouillee ? 'border-af-bordure text-af-atone' : 'border-af-bordure bg-af-surface text-af-encre hover:border-af-chocolat'
}
const classeDroite = (cle: number) => {
  if (gaucheDe(cle) != null) return 'border-af-bordure bg-af-fond/60 text-af-encre'
  if (actif.value == null || props.verrouillee) return 'border-af-bordure bg-af-surface text-af-atone'
  return 'border-af-bordure bg-af-surface text-af-encre hover:border-af-chocolat'
}

// Correction : la gauche dans l'ordre des clés, comme la solution.
const gaucheTriee = computed(() => [...props.gauche].sort((a, b) => a.cle - b.cle))
const texteDroite = (cle: number) => props.droite.find(d => d.cle === cle)?.texte ?? ''
const juste = (i: number) => props.jouee?.paires?.[i] === props.solution?.solution?.[i]
</script>
