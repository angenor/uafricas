<script setup lang="ts">
/**
 * « En ce moment » et « À suivre » d'un support programmé (FR-039, FR-020).
 *
 * Les deux créneaux sont résolus par le serveur à l'instant de la requête : ce
 * composant n'entretient aucune minuterie et ne recalcule rien. Un support sans
 * grille active ne renvoie ni l'un ni l'autre, le bandeau disparaît alors
 * entièrement et la section retombe sur son contenu mis en évidence (FR-041).
 *
 * Depuis 09q le créneau désigne un **programme** et l'épisode se déduit de la
 * rotation : le bandeau nomme donc les deux. Le lien pointe vers l'épisode : 
 * c'est lui qu'on regarde ; le nom du programme le surmonte pour situer la série.
 */
import type { CreneauAPI } from '~/composables/useMediaProgrammation'

const props = defineProps<{
  enCours: CreneauAPI | null
  suivant: CreneauAPI | null
  /** Préfixe des liens d'épisode : `programmes-tele` ou `programmes-radio`.
   * Ces adresses sont celles d'avant 09q, volontairement conservées (FR-056). */
  baseLienContenu: string
}>()

/** Le lien vise l'ÉPISODE ; à défaut, le programme. */
const lien = (creneau: CreneauAPI): string | null => {
  if (creneau.episode?.slug) return `/medias/${props.baseLienContenu}/${creneau.episode.slug}`
  if (creneau.emission?.slug) {
    const base = props.baseLienContenu === 'programmes-radio' ? 'emissions-radio' : 'emissions-tele'
    return `/medias/${base}/${creneau.emission.slug}`
  }
  return null
}

const titre = (creneau: CreneauAPI): string =>
  creneau.episode?.titre ?? creneau.emission?.titre ?? 'Programme'

/** Nom de la série, affiché seulement s'il ajoute quelque chose au titre. */
const serie = (creneau: CreneauAPI): string | null =>
  creneau.episode && creneau.emission ? creneau.emission.titre : null

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
        <span v-if="serie(enCours)" class="block truncate text-gray-400 text-[11px] uppercase tracking-wide">
          {{ serie(enCours) }}
        </span>
        <NuxtLink
          v-if="lien(enCours)"
          :to="lien(enCours)!"
          class="block truncate text-white text-sm font-semibold hover:text-yellow-400 transition-colors"
        >
          {{ titre(enCours) }}
        </NuxtLink>
        <span v-else class="block truncate text-white text-sm font-semibold">
          {{ titre(enCours) }}
        </span>
        <span class="text-gray-400 text-xs">
          {{ enCours.heure_debut }} – {{ heureFin(enCours.heure_debut, enCours.duree_minutes) }}
          <span v-if="fuseauxDistincts"> ({{ enCours.fuseau }})</span>
          <!-- La rotation a bouclé : le public a droit à l'information. -->
          <span
            v-if="enCours.est_rediffusion"
            class="ml-1.5 rounded-full border border-white/20 text-gray-300 text-[10px] px-1.5 py-0.5 uppercase tracking-wide"
          >
            Rediffusion
          </span>
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
        <span v-if="serie(suivant)" class="block truncate text-gray-400 text-[11px] uppercase tracking-wide">
          {{ serie(suivant) }}
        </span>
        <NuxtLink
          v-if="lien(suivant)"
          :to="lien(suivant)!"
          class="block truncate text-white text-sm hover:text-yellow-400 transition-colors"
        >
          {{ titre(suivant) }}
        </NuxtLink>
        <span v-else class="block truncate text-white text-sm">{{ titre(suivant) }}</span>
        <span class="text-gray-400 text-xs">
          à {{ suivant.heure_debut }}
          <span v-if="fuseauxDistincts"> ({{ suivant.fuseau }})</span>
          <span
            v-if="suivant.est_rediffusion"
            class="ml-1.5 rounded-full border border-white/20 text-gray-300 text-[10px] px-1.5 py-0.5 uppercase tracking-wide"
          >
            Rediffusion
          </span>
        </span>
      </div>
    </div>
  </div>
</template>
