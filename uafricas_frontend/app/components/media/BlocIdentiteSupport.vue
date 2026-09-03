<script setup lang="ts">
/**
 * Thématiques déclarées et couverture territoriale d'un support (US3, US4).
 *
 * La couverture est annoncée en clair : « Toute l'Afrique » n'est pas la même
 * chose qu'une liste vide, et une fiche muette laisserait croire qu'un support
 * panafricain ne rayonne nulle part. Terminologie « territoire » à l'écran,
 * `pays` dans l'API : convention du projet.
 *
 * Tailwind v4 pur : page publique (principe VI).
 */
import type { ThematiquePublique, CouverturePublique } from '~/composables/useMediaSupport'

const props = withDefaults(defineProps<{
  thematiques?: ThematiquePublique[]
  couverture?: CouverturePublique | null
}>(), {
  thematiques: () => [],
  couverture: null,
})

/** Rien de déclaré : le bloc entier disparaît plutôt que d'afficher deux titres
 * vides sur la fiche d'un support ancien. */
const aQuelqueChose = computed(() =>
  props.thematiques.length > 0
  || props.couverture?.couverture_continentale
  || (props.couverture?.territoires.length ?? 0) > 0,
)

/**
 * Deux référentiels distincts arrivent dans la même liste : les genres de
 * grille, que tout support déclare, et les lignes éditoriales, propres aux
 * chaînes de la plateforme. Les empiler sous un seul titre donnait dix-sept
 * pastilles indifférenciées sur une fiche comme Africans Télé International,
 * alors que la barre de filtres les sépare justement en deux entrées.
 */
const genres = computed(() => props.thematiques.filter(t => !t.est_ligne_editoriale))
const lignesEditoriales = computed(() => props.thematiques.filter(t => t.est_ligne_editoriale))
</script>

<template>
  <div v-if="aQuelqueChose" class="mb-10 space-y-5">
    <div v-if="genres.length">
      <p class="text-xs uppercase tracking-wide text-af-atone mb-2">Thématiques</p>
      <ul class="flex flex-wrap gap-2">
        <li
          v-for="theme in genres"
          :key="theme.id"
          class="rounded-full border border-af-bordure bg-af-fond text-af-corps px-3 py-1 text-sm"
        >
          {{ theme.nom }}
        </li>
      </ul>
    </div>

    <!-- Réservées aux chaînes de la plateforme : le bloc n'existe pas ailleurs. -->
    <div v-if="lignesEditoriales.length">
      <p class="text-xs uppercase tracking-wide text-af-atone mb-2">Lignes éditoriales</p>
      <ul class="flex flex-wrap gap-2">
        <li
          v-for="ligne in lignesEditoriales"
          :key="ligne.id"
          class="rounded-full border border-af-chocolat/40 bg-af-chocolat/5 text-af-chocolat px-3 py-1 text-sm"
        >
          {{ ligne.nom }}
        </li>
      </ul>
    </div>

    <div v-if="couverture">
      <p class="text-xs uppercase tracking-wide text-af-atone mb-2">Couverture</p>
      <p v-if="couverture.couverture_continentale" class="inline-flex items-center gap-2 text-sm text-af-chocolat">
        <font-awesome-icon :icon="['fas', 'globe']" class="w-4 h-4" />
        Toute l'Afrique
      </p>
      <ul v-else-if="couverture.territoires.length" class="flex flex-wrap gap-2">
        <li
          v-for="territoire in couverture.territoires"
          :key="territoire.id"
          class="rounded-full border border-af-bordure bg-af-fond text-af-corps px-3 py-1 text-sm"
        >
          {{ territoire.nom }}
        </li>
      </ul>
    </div>
  </div>
</template>
