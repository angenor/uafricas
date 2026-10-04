<template>
  <div class="flex flex-col gap-4">
    <div class="rounded-lg border border-af-bordure bg-af-fond/40 p-2">
      <CommonCarteAfriqueValeurs
        :comptes="{}"
        :selected-iso="solution ? null : choisi"
        :couleur="couleur"
        :libelle-bulle="bulle"
        :cliquable-a-zero="!verrouillee"
        @select="designer"
      />
    </div>

    <!-- La liste dit la même chose que la carte, sans pointeur : clavier,
         lecteur d'écran, et micro-États trop petits pour être visés au doigt. -->
    <label class="flex flex-col gap-1.5 text-[14px]/[1.4] font-bold text-af-encre">
      Ou choisissez dans la liste
      <select
        v-model="choisi"
        :disabled="verrouillee"
        class="w-full rounded-lg border border-af-bordure bg-af-surface px-3 py-2.5 text-[15px] font-normal text-af-encre focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-af-chocolat disabled:text-af-atone"
      >
        <option :value="null" disabled>Sélectionnez un pays</option>
        <option v-for="p in paysTries" :key="p.iso" :value="p.iso">{{ p.nom }}</option>
      </select>
    </label>

    <p v-if="solution?.bon_pays" class="text-[15px]/[1.5] text-af-encre">
      Le pays attendu : <strong class="text-af-vert">{{ solution.bon_pays.nom }}</strong>
      <template v-if="jouee?.pays && jouee.pays !== solution.bon_pays.iso">
        · vous avez désigné <strong class="text-af-live">{{ nomPays(jouee.pays) }}</strong>
      </template>
    </p>

    <div v-if="!solution" class="flex items-center justify-between gap-3">
      <p class="text-[14px]/[1.4] text-af-corps">
        <template v-if="choisi">Votre choix : <strong class="text-af-encre">{{ nomPays(choisi) }}</strong></template>
        <template v-else>Touchez un pays sur la carte.</template>
      </p>
      <AfricansBouton icone="fa-solid fa-check" :desactive="verrouillee || !choisi" @click="valider">
        Valider
      </AfricansBouton>
    </div>
  </div>
</template>

<script setup lang="ts">
import { PAYS_AFRICAINS_ISO2 } from '~/constants/afripulsePaysAutorises'
import { NOMS_PAYS_FR } from '~/utils/carteAfrique'
import type { ReponseJoueur, SolutionAPI } from '~/composables/useJeu'

/**
 * Épreuve « carte » (feature 014) : désigner un pays d'Afrique, sur la carte ou
 * dans la liste. Le choix n'est envoyé qu'à la validation : un clic hasardeux
 * sur la carte ne coûte pas l'épreuve.
 */
const props = defineProps<{
  verrouillee?: boolean
  /** Présente après la correction : colore le bon pays et le pays joué. */
  solution?: SolutionAPI | null
  jouee?: ReponseJoueur | null
}>()

const emit = defineEmits<{ repondre: [reponse: ReponseJoueur] }>()

const choisi = ref<string | null>(null)

const nomPays = (iso: string) => NOMS_PAYS_FR[iso] ?? iso.toUpperCase()

const paysTries = computed(() =>
  PAYS_AFRICAINS_ISO2
    .map(iso => ({ iso, nom: nomPays(iso) }))
    .sort((a, b) => a.nom.localeCompare(b.nom, 'fr')),
)

const designer = (iso: string) => {
  if (!props.verrouillee) choisi.value = iso
}

const valider = () => {
  if (choisi.value && !props.verrouillee) emit('repondre', { pays: choisi.value })
}

// Couleurs de la correction : vert pour le pays attendu, rouge pour un pays
// joué à tort. Avant la correction, la sélection est colorée par la carte.
const couleur = (_valeur: number, iso: string) => {
  const bon = props.solution?.bon_pays?.iso
  if (bon) {
    if (iso === bon) return '#228B22'
    if (iso === props.jouee?.pays) return '#dc2626'
  }
  return '#e5e7eb'
}

const bulle = (_valeur: number, iso: string) => {
  if (props.solution?.bon_pays?.iso === iso) return 'Le pays attendu'
  return props.verrouillee ? 'Épreuve close' : 'Toucher pour désigner'
}
</script>
