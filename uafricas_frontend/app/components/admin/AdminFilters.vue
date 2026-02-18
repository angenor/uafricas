<script setup lang="ts">
import type { FilterDefinition } from '~/types/admin'

const props = defineProps<{
  /** Definitions des filtres a afficher */
  filtres: FilterDefinition[]
  /** Valeurs actuelles des filtres */
  modelValue: Record<string, string>
}>()

const emit = defineEmits<{
  'update:modelValue': [value: Record<string, string>]
  rechercher: []
  reinitialiser: []
}>()

const valeurs = computed({
  get: () => props.modelValue,
  set: (v) => emit('update:modelValue', v),
})

const mettreAJour = (key: string, value: string) => {
  emit('update:modelValue', { ...valeurs.value, [key]: value })
}

const reinitialiser = () => {
  const vide: Record<string, string> = {}
  for (const filtre of props.filtres) {
    vide[filtre.key] = filtre.defaultValue || ''
  }
  emit('update:modelValue', vide)
  emit('reinitialiser')
}
</script>

<template>
  <div class="card bg-base-100 shadow-sm mb-4">
    <div class="card-body p-4">
      <div class="flex flex-wrap gap-3 items-end">
        <div
          v-for="filtre in filtres"
          :key="filtre.key"
          class="form-control w-full sm:w-auto sm:min-w-48"
        >
          <label class="label pb-1">
            <span class="label-text text-xs">{{ filtre.label }}</span>
          </label>

          <!-- Filtre texte -->
          <input
            v-if="filtre.type === 'text'"
            type="text"
            class="input input-bordered input-sm w-full"
            :placeholder="filtre.placeholder || 'Rechercher...'"
            :value="valeurs[filtre.key] || ''"
            @input="mettreAJour(filtre.key, ($event.target as HTMLInputElement).value)"
            @keydown.enter="emit('rechercher')"
          >

          <!-- Filtre select -->
          <select
            v-else-if="filtre.type === 'select'"
            class="select select-bordered select-sm w-full"
            :value="valeurs[filtre.key] || ''"
            @change="mettreAJour(filtre.key, ($event.target as HTMLSelectElement).value)"
          >
            <option value="">
              {{ filtre.placeholder || 'Tous' }}
            </option>
            <option
              v-for="opt in filtre.options"
              :key="opt.value"
              :value="opt.value"
            >
              {{ opt.label }}
            </option>
          </select>

          <!-- Filtre date -->
          <input
            v-else-if="filtre.type === 'date'"
            type="date"
            class="input input-bordered input-sm w-full"
            :value="valeurs[filtre.key] || ''"
            @input="mettreAJour(filtre.key, ($event.target as HTMLInputElement).value)"
          >
        </div>

        <!-- Boutons -->
        <div class="flex gap-2 ml-auto">
          <button
            class="btn btn-sm btn-ghost"
            @click="reinitialiser"
          >
            <font-awesome-icon icon="rotate-left" />
          </button>
          <button
            class="btn btn-sm btn-primary"
            @click="emit('rechercher')"
          >
            <font-awesome-icon icon="magnifying-glass" class="mr-1" />
            Filtrer
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
