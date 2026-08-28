<script setup lang="ts">
/**
 * Couverture territoriale d'un support (feature 009, US4).
 *
 * Deux modes **exclusifs** (FR-034) : « toute l'Afrique » ou une liste de
 * territoires. La bascule continentale neutralise la liste à l'écran, comme la
 * transaction serveur la vide en base, laisser les deux cochables aurait
 * produit une fiche affichant deux couvertures contradictoires jusqu'à la
 * prochaine écriture, que le trigger rejetterait.
 *
 * Terminologie : « **territoire** » à l'écran, `pays` dans l'API et la base.
 *
 * Tailwind v4 pur, sans daisyUI : le composant sert l'espace membre comme le
 * back-office.
 */
import type { TerritoirePublic } from '~/composables/useMediaSupport'

const props = withDefaults(defineProps<{
  /** `true` : couverture continentale : la liste est alors ignorée. */
  continentale: boolean
  /** Identifiants des territoires couverts. */
  territoires: string[]
  options?: TerritoirePublic[]
  /** Rend l'absence de couverture fautive à l'écran (support publié, FR-035). */
  requis?: boolean
  disabled?: boolean
  sombre?: boolean
}>(), {
  options: undefined,
  requis: false,
  disabled: false,
  sombre: false,
})

const emit = defineEmits<{
  'update:continentale': [boolean]
  'update:territoires': [string[]]
}>()

const { listerReferentielsEdition } = useMediaSupport()

const catalogue = ref<TerritoirePublic[]>(props.options ?? [])
const recherche = ref('')

onMounted(async () => {
  if (props.options === undefined && !catalogue.value.length) {
    const ref_ = await listerReferentielsEdition()
    catalogue.value = ref_.territoires
  }
})

watch(() => props.options, (valeur) => {
  if (valeur) catalogue.value = valeur
})

const filtres = computed(() => {
  const q = recherche.value.trim().toLowerCase()
  if (!q) return catalogue.value
  return catalogue.value.filter(t => t.nom.toLowerCase().includes(q))
})

const estSelectionne = (id: string) => props.territoires.includes(id)

const basculerTerritoire = (id: string) => {
  if (props.disabled || props.continentale) return
  emit(
    'update:territoires',
    estSelectionne(id)
      ? props.territoires.filter(x => x !== id)
      : [...props.territoires, id],
  )
}

/**
 * Passer en continental **vide** la liste immédiatement : la conserver
 * masquée laisserait croire qu'un retour au mode territoires la restituerait,
 * alors que l'écriture serveur l'a déjà supprimée.
 */
const basculerContinentale = (valeur: boolean) => {
  if (props.disabled) return
  emit('update:continentale', valeur)
  if (valeur && props.territoires.length) emit('update:territoires', [])
}

const enDefaut = computed(
  () => props.requis && !props.continentale && props.territoires.length === 0,
)
</script>

<template>
  <div>
    <p :class="sombre ? 'text-sm text-gray-300 mb-2' : 'text-sm text-af-corps mb-2'">
      Couverture territoriale
      <span v-if="requis" class="text-af-live">*</span>
    </p>

    <!-- Les deux modes sont présentés comme un choix, pas comme deux cases -->
    <div class="grid gap-2 sm:grid-cols-2 mb-4">
      <button
        type="button"
        :disabled="disabled"
        class="text-left rounded-lg border px-4 py-3 transition-colors disabled:opacity-50"
        :class="continentale
          ? (sombre ? 'bg-af-chocolat/10 border-af-chocolat' : 'bg-gray-900/5 border-af-encre')
          : (sombre ? 'bg-white/5 border-white/15 hover:border-white/40' : 'bg-white border-af-bordure hover:border-af-bordure')"
        @click="basculerContinentale(true)"
      >
        <span class="block font-semibold" :class="sombre ? 'text-white' : 'text-af-encre'">
          Toute l'Afrique
        </span>
        <span class="block text-xs mt-0.5" :class="sombre ? 'text-gray-400' : 'text-af-atone'">
          Le support remonte sur chaque territoire, quel que soit le filtre.
        </span>
      </button>

      <button
        type="button"
        :disabled="disabled"
        class="text-left rounded-lg border px-4 py-3 transition-colors disabled:opacity-50"
        :class="!continentale
          ? (sombre ? 'bg-af-chocolat/10 border-af-chocolat' : 'bg-gray-900/5 border-af-encre')
          : (sombre ? 'bg-white/5 border-white/15 hover:border-white/40' : 'bg-white border-af-bordure hover:border-af-bordure')"
        @click="basculerContinentale(false)"
      >
        <span class="block font-semibold" :class="sombre ? 'text-white' : 'text-af-encre'">
          Territoires choisis
        </span>
        <span class="block text-xs mt-0.5" :class="sombre ? 'text-gray-400' : 'text-af-atone'">
          {{ territoires.length }} territoire{{ territoires.length > 1 ? 's' : '' }} sélectionné{{ territoires.length > 1 ? 's' : '' }}.
        </span>
      </button>
    </div>

    <div v-if="!continentale">
      <input
        v-model="recherche"
        type="search"
        placeholder="Filtrer les territoires…"
        :disabled="disabled"
        class="w-full rounded-lg px-3 py-2 text-sm mb-3 border outline-none transition-colors disabled:opacity-50"
        :class="sombre
          ? 'bg-white/5 border-white/15 text-white placeholder-af-atone-2 focus:border-af-chocolat'
          : 'bg-white border-af-bordure text-af-encre placeholder-af-atone-2 focus:border-af-encre'"
      >

      <div
        class="flex flex-wrap gap-2 max-h-64 overflow-y-auto p-1 rounded-lg"
        :class="enDefaut ? 'ring-1 ring-af-live/60' : ''"
      >
        <button
          v-for="territoire in filtres"
          :key="territoire.id"
          type="button"
          :disabled="disabled"
          class="rounded-full px-3 py-1.5 text-sm border transition-colors disabled:opacity-50"
          :class="estSelectionne(territoire.id)
            ? (sombre
              ? 'bg-af-chocolat border-af-chocolat text-af-encre font-semibold'
              : 'bg-gray-900 border-af-encre text-white font-semibold')
            : (sombre
              ? 'bg-white/5 border-white/15 text-gray-300 hover:border-af-chocolat'
              : 'bg-white border-af-bordure text-af-corps hover:border-af-encre')"
          @click="basculerTerritoire(territoire.id)"
        >
          {{ territoire.nom }}
        </button>

        <p
          v-if="!filtres.length"
          :class="sombre ? 'text-sm text-af-atone' : 'text-sm text-gray-400'"
        >
          Aucun territoire ne correspond à cette recherche.
        </p>
      </div>

      <p v-if="enDefaut" class="text-xs text-af-live mt-2">
        Un support publié doit déclarer au moins un territoire, ou couvrir toute l'Afrique.
      </p>
    </div>
  </div>
</template>
