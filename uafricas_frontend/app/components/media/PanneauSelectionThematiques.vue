<script setup lang="ts">
/**
 * Feuille de sélection multiple de thématiques, TÉLÉPORTÉE dans `<body>`.
 *
 * Factorisée hors de `BarreFiltresTele.vue` : la pastille « Africans
 * Thématique » (22 genres de grille) et la pastille « Africans Télé
 * International » (44 lignes éditoriales, 09u) ouvrent chacune une instance
 * de cette même feuille, seuls le titre et le référentiel diffèrent — les
 * dupliquer aurait fait diverger tôt ou tard l'une des deux copies.
 *
 * Elle ne peut pas vivre dans la barre : celle-ci est collée au bas d'une
 * vedette qui occupe tout l'écran, donc un panneau ouvert vers le bas sort du
 * champ ; et la rangée de pastilles porte `overflow-x-auto`, qui le rogne. Un
 * simple `fixed` ne suffirait pas non plus, le `backdrop-blur` de la barre
 * crée un bloc conteneur qui capture jusqu'aux éléments fixes. Sortir du
 * sous-arbre est le seul remède qui tienne dans les trois cas.
 *
 * La forme retenue est une feuille ancrée en bas : c'est là que se trouve le
 * bouton, et c'est ce qui reste atteignable au pouce sur mobile.
 */
import type { ThematiqueDecompte } from '~/composables/useMediaSupport'

const props = withDefaults(defineProps<{
  ouvert: boolean
  titre: string
  sousTitre: string
  items: ThematiqueDecompte[]
  selection: string[]
  /** Nombre de résultats de la page, tous filtres confondus. */
  nombreResultats?: number
}>(), {
  selection: () => [],
})

const emit = defineEmits<{
  fermer: []
  'update:selection': [valeur: string[]]
}>()

/**
 * Échap ferme la feuille. Elle recouvre le bas de l'écran et n'a pas d'autre
 * sortie au clavier ; sans cela, une navigation sans souris s'y trouve coincée.
 * L'écouteur n'existe que pendant l'ouverture.
 */
const fermerSurEchap = (evenement: KeyboardEvent) => {
  if (evenement.key === 'Escape') emit('fermer')
}

watch(() => props.ouvert, (ouvert) => {
  if (!import.meta.client) return
  if (ouvert) window.addEventListener('keydown', fermerSurEchap)
  else window.removeEventListener('keydown', fermerSurEchap)
})

onBeforeUnmount(() => {
  if (import.meta.client) window.removeEventListener('keydown', fermerSurEchap)
})

const basculer = (id: string) => {
  emit(
    'update:selection',
    props.selection.includes(id)
      ? props.selection.filter(x => x !== id)
      : [...props.selection, id],
  )
}
</script>

<template>
  <Teleport to="body">
    <div v-if="ouvert" class="fixed inset-0 z-60">
      <div class="absolute inset-0 bg-black/60" @click="emit('fermer')" />

      <div
        role="dialog"
        :aria-label="`${titre}, filtrer par thématique`"
        class="absolute inset-x-3 bottom-3 sm:left-1/2 sm:right-auto sm:-translate-x-1/2 sm:w-[36rem] max-h-[65vh] flex flex-col rounded-2xl bg-black/80 ring-1 ring-white/15 shadow-2xl"
      >
        <div class="flex items-center justify-between gap-3 px-4 pt-4 pb-3 border-b border-white/10">
          <div class="min-w-0">
            <p class="text-white font-semibold truncate">
              {{ titre }}
              <span v-if="selection.length" class="text-af-orange">({{ selection.length }})</span>
            </p>
            <p class="text-xs text-white/70">{{ sousTitre }}</p>
          </div>
          <div class="flex items-center gap-3">
            <button
              v-if="selection.length"
              type="button"
              class="text-xs text-white/70 underline hover:text-white"
              @click="emit('update:selection', [])"
            >
              Tout décocher
            </button>
            <button
              type="button"
              class="text-white/70 hover:text-white"
              aria-label="Fermer"
              @click="emit('fermer')"
            >
              <font-awesome-icon :icon="['fas', 'xmark']" class="w-4 h-4" />
            </button>
          </div>
        </div>

        <!-- Le défilement vit ici, pas sur la feuille : l'en-tête et le pied
             doivent rester visibles quand la liste est longue. -->
        <div class="flex flex-wrap gap-2 overflow-y-auto px-4 py-4">
          <!-- Les thèmes sans support restent proposés et cliquables : leur
               `(0)` est une information : il dit que le thème existe et
               n'attend qu'un contenu, là où les masquer laisserait croire à
               un catalogue plus étroit qu'il n'est. Ils sont simplement
               estompés pour ne pas concurrencer les thèmes servis. Le `title`
               natif porte la description longue, quand il y en a une. -->
          <button
            v-for="t in items"
            :key="t.id"
            type="button"
            :title="t.description || undefined"
            class="rounded-full border px-3 py-1.5 text-xs transition-colors"
            :class="selection.includes(t.id)
              ? 'bg-af-orange border-af-orange text-black font-semibold'
              : t.nombre_supports > 0
                ? 'bg-white/5 border-white/15 text-white/80 hover:border-af-orange'
                : 'bg-transparent border-white/10 text-white/60 hover:border-af-orange/60 hover:text-white/80'"
            @click="basculer(t.id)"
          >
            {{ t.nom }} ({{ t.nombre_supports }})
          </button>
        </div>

        <div class="px-4 pb-4 pt-1 border-t border-white/10">
          <button
            type="button"
            class="w-full rounded-full bg-af-orange text-black font-semibold py-2.5 text-sm hover:bg-af-orange transition-colors"
            @click="emit('fermer')"
          >
            Voir les résultats
            <span v-if="nombreResultats !== undefined">({{ nombreResultats }})</span>
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>
