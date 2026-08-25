<script setup lang="ts">
/**
 * Sélection **multiple** des thématiques d'un support (feature 009, US3).
 *
 * Tailwind v4 pur, sans daisyUI : le composant sert les deux mondes, l'espace
 * membre `/mon-compte/mes-supports` (page publique, principe VI) et le
 * back-office. Une variante daisyUI aurait dupliqué la logique de sélection
 * pour n'en changer que la peinture.
 *
 * Le composant ne persiste rien : il émet `update:modelValue`. L'écriture
 * (`PUT …/thematiques`) appartient à l'écran qui l'accueille, seul à savoir
 * s'il enregistre en membre ou en administration.
 */
import type { ThematiquePublique } from '~/composables/useMediaSupport'

const props = withDefaults(defineProps<{
  /** Identifiants sélectionnés. */
  modelValue: string[]
  /** Catalogue complet ; chargé par l'appelant, ou ici à défaut. */
  options?: ThematiquePublique[]
  /** Rend l'absence de sélection fautive à l'écran (support publié, FR-029). */
  requis?: boolean
  disabled?: boolean
  /** Palette sombre pour les pages publiques médias. */
  sombre?: boolean
}>(), {
  options: undefined,
  requis: false,
  disabled: false,
  sombre: false,
})

const emit = defineEmits<{ 'update:modelValue': [string[]] }>()

const { listerReferentielsEdition } = useMediaSupport()

const catalogue = ref<ThematiquePublique[]>(props.options ?? [])
const recherche = ref('')

// Chargement autonome seulement si l'appelant n'a rien fourni : deux écrans
// voisins ne doivent pas retélécharger les 44 thèmes chacun de leur côté.
onMounted(async () => {
  if (props.options === undefined && !catalogue.value.length) {
    const ref_ = await listerReferentielsEdition()
    catalogue.value = ref_.thematiques
  }
})

watch(() => props.options, (valeur) => {
  if (valeur) catalogue.value = valeur
})

const filtrees = computed(() => {
  const q = recherche.value.trim().toLowerCase()
  if (!q) return catalogue.value
  return catalogue.value.filter(t => t.nom.toLowerCase().includes(q))
})

const estSelectionne = (id: string) => props.modelValue.includes(id)

const basculer = (id: string) => {
  if (props.disabled) return
  emit(
    'update:modelValue',
    estSelectionne(id)
      ? props.modelValue.filter(x => x !== id)
      : [...props.modelValue, id],
  )
}

const viderSelection = () => emit('update:modelValue', [])

/** Faute affichée seulement quand elle est réelle : requis ET vide. */
const enDefaut = computed(() => props.requis && props.modelValue.length === 0)
</script>

<template>
  <div>
    <div class="flex flex-wrap items-baseline justify-between gap-2 mb-2">
      <p :class="sombre ? 'text-sm text-gray-300' : 'text-sm text-gray-700'">
        Thématiques
        <span v-if="requis" class="text-red-500">*</span>
        <span :class="sombre ? 'text-gray-500' : 'text-gray-400'">
          {{ modelValue.length }} sélectionnée{{ modelValue.length > 1 ? 's' : '' }}
        </span>
      </p>
      <button
        v-if="modelValue.length && !disabled"
        type="button"
        class="text-xs underline"
        :class="sombre ? 'text-gray-400 hover:text-white' : 'text-gray-500 hover:text-gray-900'"
        @click="viderSelection"
      >
        Tout décocher
      </button>
    </div>

    <input
      v-model="recherche"
      type="search"
      placeholder="Filtrer les thématiques…"
      :disabled="disabled"
      class="w-full rounded-lg px-3 py-2 text-sm mb-3 border outline-none transition-colors disabled:opacity-50"
      :class="sombre
        ? 'bg-white/5 border-white/15 text-white placeholder-gray-500 focus:border-yellow-400'
        : 'bg-white border-gray-300 text-gray-900 placeholder-gray-400 focus:border-gray-900'"
    >

    <div
      class="flex flex-wrap gap-2 max-h-64 overflow-y-auto p-1 rounded-lg"
      :class="enDefaut ? 'ring-1 ring-red-500/60' : ''"
    >
      <button
        v-for="theme in filtrees"
        :key="theme.id"
        type="button"
        :disabled="disabled"
        class="rounded-full px-3 py-1.5 text-sm border transition-colors disabled:opacity-50"
        :class="estSelectionne(theme.id)
          ? (sombre
            ? 'bg-yellow-400 border-yellow-400 text-neutral-900 font-semibold'
            : 'bg-gray-900 border-gray-900 text-white font-semibold')
          : (sombre
            ? 'bg-white/5 border-white/15 text-gray-300 hover:border-yellow-400'
            : 'bg-white border-gray-300 text-gray-700 hover:border-gray-900')"
        @click="basculer(theme.id)"
      >
        {{ theme.nom }}
      </button>

      <p
        v-if="!filtrees.length"
        :class="sombre ? 'text-sm text-gray-500' : 'text-sm text-gray-400'"
      >
        Aucune thématique ne correspond à cette recherche.
      </p>
    </div>

    <p v-if="enDefaut" class="text-xs text-red-500 mt-2">
      Un support publié doit porter au moins une thématique.
    </p>
  </div>
</template>
