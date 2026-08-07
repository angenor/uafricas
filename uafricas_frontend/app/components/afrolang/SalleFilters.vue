<template>
  <div class="bg-white rounded-2xl shadow-xl p-6 sticky top-24">
    <!-- Header -->
    <div class="flex items-center justify-between mb-6">
      <h3 class="text-lg font-semibold text-gray-900 flex items-center gap-2">
        <font-awesome-icon :icon="['fas', 'filter']" class="w-5 h-5 text-blue-500" />
        Filtres
      </h3>
      <button
        class="text-sm text-gray-500 hover:text-blue-500 transition-colors"
        @click="$emit('reset')"
      >
        Réinitialiser
      </button>
    </div>

    <!-- Filtre par langue -->
    <div class="mb-6">
      <label class="block text-sm font-medium text-gray-700 mb-2">
        <font-awesome-icon :icon="['fas', 'language']" class="w-4 h-4 mr-2 text-gray-400" />
        Langue
      </label>
      <select
        v-model="localFiltres.langue"
        class="w-full p-3 bg-gray-50 border border-gray-200 rounded-xl focus:outline-hidden focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
        @change="emitChange"
      >
        <option value="">Toutes les langues</option>
        <option v-for="langue in langues" :key="langue" :value="langue">
          {{ langue }}
        </option>
      </select>
    </div>

    <!-- Filtre par pays d'origine (feature 001-afrolang-pays-origine) -->
    <div v-if="pays.length > 0" class="mb-6">
      <label class="block text-sm font-medium text-gray-700 mb-2">
        <font-awesome-icon :icon="['fas', 'globe']" class="w-4 h-4 mr-2 text-gray-400" />
        Territoire d'origine
      </label>

      <!-- Choix de la zone (radio) qui pilote le contenu du menu déroulant -->
      <div class="grid grid-cols-3 gap-2 mb-3">
        <label
          v-for="option in ZONES_TERRITOIRE"
          :key="option.value"
          :class="[
            'flex items-center justify-center px-2 py-2 rounded-xl text-xs font-medium whitespace-nowrap cursor-pointer transition-all',
            zoneTerritoire === option.value
              ? 'bg-blue-500 text-white shadow-sm'
              : 'bg-gray-100 text-gray-700 hover:bg-gray-200',
          ]"
        >
          <input
            v-model="zoneTerritoire"
            type="radio"
            :value="option.value"
            class="sr-only"
          />
          {{ option.label }}
        </label>
      </div>

      <select
        v-model="localFiltres.pays_id"
        class="w-full p-3 bg-gray-50 border border-gray-200 rounded-xl focus:outline-hidden focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
        @change="emitChange"
      >
        <option value="">Tous les territoires</option>
        <option v-for="p in territoiresDisponibles" :key="p.id" :value="p.id">
          {{ p.nom }}
        </option>
      </select>
    </div>

    <!-- Statistiques -->
    <div class="pt-6 border-t border-gray-200">
      <h4 class="text-sm font-medium text-gray-500 uppercase tracking-wider mb-4">Statistiques</h4>
      <div class="space-y-3">
        <div class="flex justify-between items-center p-3 bg-gray-50 rounded-lg">
          <span class="text-sm text-gray-600">Total salles</span>
          <span class="font-bold text-gray-900 text-lg">{{ totalSalles }}</span>
        </div>
        <div class="flex justify-between items-center p-3 bg-blue-50 rounded-lg">
          <span class="text-sm text-blue-700">Résultats filtrés</span>
          <span class="font-bold text-blue-600 text-lg">{{ filteredCount }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { PaysOrigineLight, SalleFiltres } from '~/composables/useAfrolang'
import { PAYS_AFRICAINS_ISO2 } from '~/constants/afripulsePaysAutorises'

const props = defineProps<{
  totalSalles: number
  filteredCount: number
  modelValue: SalleFiltres
  langues: string[]
  pays: PaysOrigineLight[]
}>()

const emit = defineEmits<{
  'update:modelValue': [filtres: SalleFiltres]
  reset: []
}>()

const localFiltres = ref<SalleFiltres>({ ...props.modelValue })

// Zone géographique qui pilote le contenu du menu déroulant des territoires
const ZONES_TERRITOIRE = [
  { value: 'tout' as const, label: 'Mondial' },
  { value: 'afrique' as const, label: 'Afrique' },
  { value: 'hors_afrique' as const, label: 'Hors Afrique' },
]
type ZoneTerritoire = (typeof ZONES_TERRITOIRE)[number]['value']

// La zone est portée par le modèle (`filtres.zone`) : elle filtre aussi la
// liste des salles, pas seulement le contenu du menu déroulant des territoires.
const zoneTerritoire = computed<ZoneTerritoire>({
  get: () => localFiltres.value.zone ?? 'tout',
  set: (valeur) => {
    localFiltres.value.zone = valeur
    // Contenus disjoints : changer de zone réinitialise le territoire choisi.
    if (localFiltres.value.pays_id) localFiltres.value.pays_id = ''
    emitChange()
  },
})

const PAYS_AFRICAINS_SET = new Set<string>(PAYS_AFRICAINS_ISO2)

const estAfricain = (p: PaysOrigineLight): boolean =>
  !!p.code_iso2 && PAYS_AFRICAINS_SET.has(p.code_iso2.toLowerCase())

const territoiresDisponibles = computed(() => {
  if (zoneTerritoire.value === 'tout') return props.pays
  return props.pays.filter(p => (zoneTerritoire.value === 'afrique' ? estAfricain(p) : !estAfricain(p)))
})

watch(
  () => props.modelValue,
  (newValue) => {
    localFiltres.value = { ...newValue }
    // Aligner la zone sur le territoire sélectionné (ex. reset externe), sans
    // repasser par le setter de la computed (qui émettrait / viderait pays_id).
    // En zone « Tout » on ne réaligne pas : le choix englobe les deux zones.
    const selection = newValue.zone === 'tout'
      ? undefined
      : props.pays.find(p => p.id === newValue.pays_id)
    if (selection) {
      localFiltres.value.zone = estAfricain(selection) ? 'afrique' : 'hors_afrique'
    }
  },
  { deep: true },
)

const emitChange = () => {
  emit('update:modelValue', { ...localFiltres.value })
}
</script>
