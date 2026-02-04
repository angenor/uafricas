<template>
  <div class="flex flex-wrap gap-3 items-center mb-6">
    <!-- Filtre par type -->
    <select
      v-model="localFiltres.type"
      class="font-medium px-3 py-2 rounded-xl cursor-pointer bg-white border border-custom-chocolat text-custom-chocolat"
      @change="emitUpdate"
    >
      <option
        v-for="type in TYPES_PROGRAMME"
        :key="type.value"
        :value="type.value"
      >
        {{ type.label }}
      </option>
    </select>

    <!-- Filtre par pays -->
    <select
      v-model="localFiltres.pays"
      class="font-medium px-3 py-2 rounded-xl cursor-pointer bg-white border border-custom-chocolat text-custom-chocolat"
      @change="emitUpdate"
    >
      <option
        v-for="pays in PAYS_AFRICAINS"
        :key="pays.value"
        :value="pays.value"
      >
        {{ pays.label }}
      </option>
    </select>

    <!-- Filtre par domaine -->
    <select
      v-model="localFiltres.domaine"
      class="font-medium px-3 py-2 rounded-xl cursor-pointer bg-white border border-custom-chocolat text-custom-chocolat"
      @change="emitUpdate"
    >
      <option
        v-for="domaine in DOMAINES"
        :key="domaine.value"
        :value="domaine.value"
      >
        {{ domaine.label }}
      </option>
    </select>

    <!-- Recherche textuelle -->
    <div class="relative">
      <input
        v-model="localFiltres.recherche"
        type="text"
        placeholder="Rechercher..."
        class="pl-10 pr-4 py-2 rounded-xl border border-gray-300 focus:border-custom-green focus:outline-hidden"
        @input="emitUpdate"
      />
      <font-awesome-icon
        class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 h-4"
        :icon="['fas', 'search']"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, watch } from 'vue'
import { TYPES_PROGRAMME, PAYS_AFRICAINS, DOMAINES, type FiltresSabbatique } from '~/mocks/sabbatiques'

const props = defineProps<{
  modelValue: FiltresSabbatique
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', filtres: FiltresSabbatique): void
}>()

const localFiltres = reactive<FiltresSabbatique>({
  type: props.modelValue.type,
  pays: props.modelValue.pays,
  domaine: props.modelValue.domaine,
  recherche: props.modelValue.recherche
})

watch(() => props.modelValue, (newVal) => {
  localFiltres.type = newVal.type
  localFiltres.pays = newVal.pays
  localFiltres.domaine = newVal.domaine
  localFiltres.recherche = newVal.recherche
}, { deep: true })

const emitUpdate = () => {
  emit('update:modelValue', { ...localFiltres })
}
</script>
