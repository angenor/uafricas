<template>
  <!-- Accordéon de fiche pays. Ouvert, le fond passe par défaut au pêche
       (chocolat à 15 %) : c'est le même signal que l'item de navigation actif,
       et c'est voulu : dans les deux cas il marque « vous êtes ici ».
       `fond="blanc"` le laisse blanc, pour les sections que la maquette ne
       teinte pas. -->
  <section
    class="overflow-hidden rounded-[10px] border transition-colors"
    :class="[
      ouvert ? 'border-af-chocolat/30' : 'border-af-bordure',
      ouvert && fond === 'peche' ? 'bg-af-chocolat/15' : 'bg-white']"
  >
    <h3>
      <button
        type="button"
        class="flex w-full items-center gap-3 px-5 py-6 text-left focus-visible:outline-2 focus-visible:-outline-offset-2 focus-visible:outline-af-chocolat"
        :aria-expanded="ouvert"
        :aria-controls="id"
        @click="basculer"
      >
        <font-awesome-icon v-if="icone" :icon="icone" class="size-6 shrink-0 text-af-chocolat" />
        <span class="text-[20px]/[1.4] font-bold text-af-chocolat">{{ titre }}</span>
        <font-awesome-icon
          icon="fa-solid fa-chevron-down"
          class="ml-auto size-6 shrink-0 text-af-chocolat transition-transform"
          :class="ouvert && 'rotate-180'"
        />
      </button>
    </h3>

    <div v-show="ouvert" :id="id" class="px-5 pb-6">
      <slot />
    </div>
  </section>
</template>

<script setup lang="ts">
const props = withDefaults(defineProps<{
  titre: string
  icone?: string
  /** Non contrôlé par défaut ; passer `modelValue` pour piloter de l'extérieur. */
  modelValue?: boolean
  parDefautOuvert?: boolean
  /**
   * Fond une fois ouvert. La maquette n'en donne pas qu'un : « Informations
   * générales » reste blanche là où « Cultures et langues » passe au pêche.
   * Le pêche est le défaut : c'est le cas le plus fréquent, et c'est le même
   * signal que l'item de navigation actif.
   */
  fond?: 'peche' | 'blanc'
}>(), {
  // `modelValue: undefined` n'est PAS décoratif. Vue convertit toute prop de
  // type Boolean absente en `false` : sauf si une valeur par défaut est
  // déclarée. Sans cette ligne, `props.modelValue` valait `false` au lieu de
  // `undefined`, le `??` ci-dessous ne se repliait jamais sur l'état interne,
  // et l'accordéon non contrôlé restait fermé pour toujours : `parDefautOuvert`
  // était ignoré et le clic n'ouvrait rien.
  modelValue: undefined,
  parDefautOuvert: false,
  fond: 'peche',
})

const emit = defineEmits<{ 'update:modelValue': [boolean] }>()

const id = useId()

// v-show et non v-if : le contenu des fiches pays est indexable, et le retirer
// du DOM le soustrairait aussi à la recherche du navigateur (Cmd+F).
const interne = ref(props.parDefautOuvert)
const ouvert = computed(() => props.modelValue ?? interne.value)

function basculer() {
  const suivant = !ouvert.value
  interne.value = suivant
  emit('update:modelValue', suivant)
}
</script>
