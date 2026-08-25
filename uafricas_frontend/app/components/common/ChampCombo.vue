<script setup lang="ts">
/**
 * Champ de saisie libre assisté par une liste de suggestions, feature 010.
 *
 * **La valeur hors liste est acceptée** (FR-015) : les suggestions proposent, la
 * saisie décide. C'est ce qui distingue ce composant d'un `<select>` et de
 * `arbre-genealogique/ChampRecherche.vue`, qui vide le champ à la sélection.
 * Le contournement jusqu'ici en usage, un `<select>` doublé d'une option
 * « AUTRE » qui révèle un `<input>` : demandait deux gestes pour une valeur
 * inédite, et rendait la nouveauté visiblement moins légitime que le catalogue.
 *
 * Tailwind v4 pur (Principe VI) : monté sur des surfaces membres et
 * back-office.
 */

const props = withDefaults(defineProps<{
  modelValue: string
  suggestions?: string[]
  placeholder?: string
  /** Nombre maximum de suggestions listées à la fois. */
  maxSuggestions?: number
  disabled?: boolean
  ariaLabel?: string
}>(), {
  suggestions: () => [],
  placeholder: '',
  maxSuggestions: 8,
  disabled: false,
  ariaLabel: '',
})

const emit = defineEmits<{ 'update:modelValue': [valeur: string] }>()

const ouvert = ref(false)
const indexActif = ref(-1)
const racine = ref<HTMLElement | null>(null)

/**
 * Filtrage insensible à la casse et aux accents : « realisateur » doit trouver
 * « Réalisateur », sans quoi le rédacteur recréerait une entrée quasi-doublon
 * que le référentiel afficherait ensuite à côté de l'originale.
 */
const aplatir = (valeur: string): string =>
  valeur
    .normalize('NFD')
    .replace(/[\u0300-\u036f]/g, '')
    .toLowerCase()
    .trim()

const filtrees = computed(() => {
  const saisie = aplatir(props.modelValue)
  const liste = props.suggestions.filter((s) => {
    if (!saisie) return true
    return aplatir(s).includes(saisie)
  })
  return liste.slice(0, props.maxSuggestions)
})

const auChangement = (evenement: Event) => {
  emit('update:modelValue', (evenement.target as HTMLInputElement).value)
  ouvert.value = true
  indexActif.value = -1
}

const choisir = (valeur: string) => {
  emit('update:modelValue', valeur)
  ouvert.value = false
  indexActif.value = -1
}

const auClavier = (evenement: KeyboardEvent) => {
  if (evenement.key === 'Escape') {
    ouvert.value = false
    return
  }
  if (!filtrees.value.length) return

  if (evenement.key === 'ArrowDown') {
    evenement.preventDefault()
    ouvert.value = true
    indexActif.value = (indexActif.value + 1) % filtrees.value.length
  }
  else if (evenement.key === 'ArrowUp') {
    evenement.preventDefault()
    ouvert.value = true
    indexActif.value = indexActif.value <= 0 ? filtrees.value.length - 1 : indexActif.value - 1
  }
  else if (evenement.key === 'Enter' && ouvert.value && indexActif.value >= 0) {
    // Entrée ne valide que ce qui est SURLIGNÉ. Sans cette garde, valider un
    // formulaire au clavier écraserait la saisie libre par la première
    // suggestion venue.
    evenement.preventDefault()
    choisir(filtrees.value[indexActif.value] as string)
  }
}

const auClicExterieur = (evenement: MouseEvent) => {
  if (racine.value && !racine.value.contains(evenement.target as Node)) {
    ouvert.value = false
  }
}

onMounted(() => document.addEventListener('mousedown', auClicExterieur))
onBeforeUnmount(() => document.removeEventListener('mousedown', auClicExterieur))
</script>

<template>
  <div ref="racine" class="relative">
    <input
      :value="modelValue"
      type="text"
      role="combobox"
      autocomplete="off"
      :aria-expanded="ouvert"
      :aria-label="ariaLabel || placeholder"
      :placeholder="placeholder"
      :disabled="disabled"
      class="w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm text-gray-900 placeholder-gray-400 transition-colors focus:border-custom-chocolat focus:outline-none focus:ring-1 focus:ring-custom-chocolat disabled:cursor-not-allowed disabled:bg-gray-100"
      @input="auChangement"
      @focus="ouvert = true"
      @keydown="auClavier"
    >

    <ul
      v-if="ouvert && filtrees.length"
      class="absolute z-30 mt-1 max-h-56 w-full overflow-y-auto rounded-lg border border-gray-200 bg-white py-1 shadow-lg"
    >
      <li
        v-for="(suggestion, index) in filtrees"
        :key="suggestion"
        class="cursor-pointer px-3 py-2 text-sm text-gray-700 transition-colors"
        :class="index === indexActif ? 'bg-custom-chocolat/10 text-custom-chocolat' : 'hover:bg-gray-50'"
        @mousedown.prevent="choisir(suggestion)"
        @mouseenter="indexActif = index"
      >
        {{ suggestion }}
      </li>
    </ul>
  </div>
</template>
