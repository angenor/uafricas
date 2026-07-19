<script setup lang="ts">
/**
 * « En ce moment » et « À suivre » d'un support programmé (FR-039).
 *
 * Les deux créneaux sont résolus par le serveur à l'instant de la requête : ce
 * composant n'entretient aucune minuterie et ne recalcule rien. Un support sans
 * grille active ne renvoie ni l'un ni l'autre — le bandeau disparaît alors
 * entièrement et la section retombe sur son contenu mis en évidence (FR-041).
 */
import type { CreneauAPI } from '~/composables/useMediaProgrammation'

const props = defineProps<{
  enCours: CreneauAPI | null
  suivant: CreneauAPI | null
  /** Préfixe des liens de contenu : `programmes-tele` ou `programmes-radio`. */
  baseLienContenu: string
}>()

const lien = (creneau: CreneauAPI) =>
  creneau.contenu_slug ? `/medias/${props.baseLienContenu}/${creneau.contenu_slug}` : null

/** Le fuseau n'est rappelé que lorsque les deux créneaux ne partagent pas le même. */
const fuseauxDistincts = computed(() =>
  Boolean(props.enCours && props.suivant && props.enCours.fuseau !== props.suivant.fuseau),
)
</script>

<template>
  <div
    v-if="enCours || suivant"
    class="flex flex-wrap items-center gap-x-6 gap-y-3 rounded-xl bg-neutral-900/70 border border-white/10 px-4 py-3 mb-6"
  >
    <div v-if="enCours" class="flex items-center gap-3 min-w-0">
      <span
        class="shrink-0 rounded-full bg-red-600 text-white text-[10px] font-bold px-2 py-0.5 uppercase tracking-wide"
      >
        En ce moment
      </span>
      <div class="min-w-0">
        <NuxtLink
          v-if="lien(enCours)"
          :to="lien(enCours)!"
          class="block truncate text-white text-sm font-semibold hover:text-yellow-400 transition-colors"
        >
          {{ enCours.contenu_nom }}
        </NuxtLink>
        <span v-else class="block truncate text-white text-sm font-semibold">
          {{ enCours.contenu_nom }}
        </span>
        <span class="text-gray-400 text-xs">
          {{ enCours.heure_debut }} – {{ heureFin(enCours.heure_debut, enCours.duree_minutes) }}
          <span v-if="fuseauxDistincts"> ({{ enCours.fuseau }})</span>
        </span>
      </div>
    </div>

    <div v-if="suivant" class="flex items-center gap-3 min-w-0">
      <span
        class="shrink-0 rounded-full border border-yellow-400 text-yellow-400 text-[10px] font-bold px-2 py-0.5 uppercase tracking-wide"
      >
        À suivre
      </span>
      <div class="min-w-0">
        <NuxtLink
          v-if="lien(suivant)"
          :to="lien(suivant)!"
          class="block truncate text-white text-sm hover:text-yellow-400 transition-colors"
        >
          {{ suivant.contenu_nom }}
        </NuxtLink>
        <span v-else class="block truncate text-white text-sm">{{ suivant.contenu_nom }}</span>
        <span class="text-gray-400 text-xs">
          à {{ suivant.heure_debut }}
          <span v-if="fuseauxDistincts"> ({{ suivant.fuseau }})</span>
        </span>
      </div>
    </div>
  </div>
</template>
