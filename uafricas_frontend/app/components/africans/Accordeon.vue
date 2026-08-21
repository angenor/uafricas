<template>
  <!-- Accordéon de fiche pays. Ouvert, le fond passe au pêche (chocolat à 15 %)
       — c'est le même signal que l'item de navigation actif, et c'est voulu :
       dans les deux cas il marque « vous êtes ici ». -->
  <section
    class="overflow-hidden rounded-[10px] border transition-colors"
    :class="ouvert ? 'border-af-chocolat/30 bg-af-chocolat/15' : 'border-af-bordure bg-white'"
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
}>(), { parDefautOuvert: false })

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
