<template>
  <div class="flex flex-col gap-2">
    <!-- Libellé en italique au-dessus du champ : c'est le seul emploi de
         l'italique dans la maquette, avec l'horodatage des publications. -->
    <label :for="id" class="text-[14px]/[1.4] text-af-atone italic">{{ libelle }}</label>

    <select
      v-if="type === 'select'"
      :id="id"
      :value="modelValue"
      class="h-11 rounded-md border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:border-af-chocolat focus:outline-none"
      @change="$emit('update:modelValue', ($event.target as HTMLSelectElement).value)"
    >
      <slot />
    </select>

    <textarea
      v-else-if="type === 'textarea'"
      :id="id"
      :value="modelValue"
      :placeholder="placeholder"
      rows="4"
      class="rounded-md border border-af-bordure bg-white px-4 py-3 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:border-af-chocolat focus:outline-none"
      @input="$emit('update:modelValue', ($event.target as HTMLTextAreaElement).value)"
    />

    <!-- L'icône est posée EN INCRUSTATION, pas devant le champ : elle doit
         rester dans la boîte pour que le clic n'importe où la traverse et
         atteigne l'input. -->
    <div v-else class="relative">
      <font-awesome-icon
        v-if="icone"
        :icon="icone"
        class="pointer-events-none absolute top-1/2 left-4 -translate-y-1/2 text-af-atone-2"
      />
      <input
        :id="id"
        :type="type"
        :value="modelValue"
        :placeholder="placeholder"
        :required="obligatoire"
        :disabled="desactive"
        :autocomplete="autocomplete"
        class="h-11 w-full rounded-md border border-af-bordure bg-white pr-4 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:border-af-chocolat focus:outline-none disabled:opacity-50"
        :class="icone ? 'pl-11' : 'pl-4'"
        @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
      />
    </div>

    <p v-if="aide" class="text-[12px]/[1.4] text-af-atone">{{ aide }}</p>
  </div>
</template>

<script setup lang="ts">
withDefaults(defineProps<{
  libelle: string
  modelValue?: string
  type?: 'text' | 'email' | 'password' | 'tel' | 'url' | 'select' | 'textarea'
  placeholder?: string
  aide?: string
  /** Icône posée en tête du champ (types simples uniquement). */
  icone?: string
  obligatoire?: boolean
  desactive?: boolean
  autocomplete?: string
}>(), { type: 'text' })

defineEmits<{ 'update:modelValue': [string] }>()

// useId() garantit l'appariement label/champ même si le composant est monté
// plusieurs fois sur une page : le cas du panneau Filtres, répété par module.
const id = useId()
</script>
