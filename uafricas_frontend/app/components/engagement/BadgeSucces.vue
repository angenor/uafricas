<script setup lang="ts">
/**
 * Vignette d'un badge : Tailwind v4 pur, réutilisable dans l'espace membre et
 * sur le profil public.
 *
 * Deux états : obtenu (couleur pleine, date) et verrouillé (grisé, cadenas).
 * Le libellé, la description, la couleur et l'icône viennent tous de la base, 
 * aucun texte de badge n'est écrit dans ce composant.
 */
import { computed } from 'vue'

const props = withDefaults(defineProps<{
  libelle: string
  description?: string
  couleur?: string | null
  icone?: string | null
  /** Faux = badge verrouillé (« à débloquer ») */
  obtenu?: boolean
  /** Date d'obtention (ISO), affichée seulement si le badge est obtenu */
  obtenuAt?: string | null
  taille?: 'sm' | 'md'
}>(), {
  description: '',
  couleur: null,
  icone: null,
  obtenu: true,
  obtenuAt: null,
  taille: 'md',
})

/** Jetons de couleur (base) → classes Tailwind, alignés sur la palette du projet. */
const CLASSES: Record<string, { fond: string, texte: string, anneau: string }> = {
  green: { fond: 'bg-custom-green/10', texte: 'text-custom-green', anneau: 'ring-custom-green/30' },
  chocolat: { fond: 'bg-custom-chocolat/10', texte: 'text-custom-chocolat', anneau: 'ring-custom-chocolat/30' },
  amber: { fond: 'bg-amber-50', texte: 'text-amber-700', anneau: 'ring-amber-200' },
  sky: { fond: 'bg-sky-50', texte: 'text-sky-700', anneau: 'ring-sky-200' },
  violet: { fond: 'bg-violet-50', texte: 'text-violet-700', anneau: 'ring-violet-200' },
  rose: { fond: 'bg-rose-50', texte: 'text-rose-600', anneau: 'ring-rose-200' },
  slate: { fond: 'bg-slate-100', texte: 'text-slate-700', anneau: 'ring-slate-300' },
  gray: { fond: 'bg-gray-100', texte: 'text-gray-600', anneau: 'ring-gray-200' },
}

const VERROUILLE = { fond: 'bg-gray-50', texte: 'text-gray-400', anneau: 'ring-gray-200' }

const classes = computed(() =>
  props.obtenu ? (CLASSES[props.couleur || 'gray'] || CLASSES.gray) : VERROUILLE,
)

const dimensions = computed(() =>
  props.taille === 'sm'
    ? { pastille: 'size-9 text-sm', titre: 'text-xs', bloc: 'p-2.5 gap-2' }
    : { pastille: 'size-12 text-lg', titre: 'text-sm', bloc: 'p-3.5 gap-3' },
)

const dateFormatee = computed(() =>
  props.obtenuAt
    ? new Date(props.obtenuAt).toLocaleDateString('fr-FR', {
        day: '2-digit', month: 'long', year: 'numeric',
      })
    : null,
)
</script>

<template>
  <div
    class="flex items-start rounded-2xl ring-1 transition"
    :class="[classes.fond, classes.anneau, dimensions.bloc]"
    :title="description || libelle"
  >
    <span
      class="grid shrink-0 place-items-center rounded-full bg-white/70"
      :class="[dimensions.pastille, classes.texte]"
    >
      <font-awesome-icon
        :icon="obtenu ? `fa-solid fa-${icone || 'award'}` : 'fa-solid fa-lock'"
      />
    </span>

    <div class="min-w-0 flex-1">
      <p class="font-semibold" :class="[dimensions.titre, classes.texte]">{{ libelle }}</p>
      <p v-if="description" class="mt-0.5 text-xs leading-snug text-gray-500">
        {{ description }}
      </p>
      <p v-if="obtenu && dateFormatee" class="mt-1 text-[11px] text-gray-400">
        Obtenu le {{ dateFormatee }}
      </p>
      <slot />
    </div>
  </div>
</template>
