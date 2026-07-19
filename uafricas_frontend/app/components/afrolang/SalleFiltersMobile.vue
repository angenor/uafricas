<template>
  <!-- Overlay -->
  <Transition
    enter-active-class="transition-opacity duration-300"
    enter-from-class="opacity-0"
    enter-to-class="opacity-100"
    leave-active-class="transition-opacity duration-300"
    leave-from-class="opacity-100"
    leave-to-class="opacity-0"
  >
    <div
      v-if="isOpen"
      class="fixed inset-0 bg-black/50 z-40"
      @click="$emit('close')"
    />
  </Transition>

  <!-- Drawer -->
  <Transition
    enter-active-class="transition-transform duration-300"
    enter-from-class="-translate-x-full"
    enter-to-class="translate-x-0"
    leave-active-class="transition-transform duration-300"
    leave-from-class="translate-x-0"
    leave-to-class="-translate-x-full"
  >
    <div
      v-if="isOpen"
      class="fixed top-0 left-0 h-full w-80 max-w-[85vw] bg-white z-50 shadow-2xl overflow-y-auto"
    >
      <!-- Header -->
      <div class="flex items-center justify-between p-4 border-b border-gray-200">
        <h3 class="text-lg font-semibold text-gray-900 flex items-center gap-2">
          <font-awesome-icon :icon="['fas', 'filter']" class="w-5 h-5 text-blue-500" />
          Filtres
        </h3>
        <button
          class="p-2 text-gray-500 hover:text-gray-700 hover:bg-gray-100 rounded-lg transition-colors"
          @click="$emit('close')"
        >
          <font-awesome-icon :icon="['fas', 'times']" class="w-5 h-5" />
        </button>
      </div>

      <!-- Filtres -->
      <div class="p-4 space-y-6">
        <!-- Filtre par langue -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">
            <font-awesome-icon :icon="['fas', 'language']" class="w-4 h-4 mr-2 text-gray-400" />
            Langue
          </label>
          <select
            v-model="localFiltres.langue"
            class="w-full p-3 bg-gray-50 border border-gray-200 rounded-xl focus:outline-hidden focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            @change="emitChange"
          >
            <option value="">Toutes les langues</option>
            <option v-for="langue in langues" :key="langue" :value="langue">
              {{ langue }}
            </option>
          </select>
        </div>

        <!-- Filtre par pays d'origine (feature 001-afrolang-pays-origine) -->
        <div v-if="pays.length > 0">
          <label class="block text-sm font-medium text-gray-700 mb-2">
            <font-awesome-icon :icon="['fas', 'globe']" class="w-4 h-4 mr-2 text-gray-400" />
            Territoire d'origine
          </label>

          <!-- Choix de la zone (radio) qui pilote le contenu du menu déroulant -->
          <div class="grid grid-cols-2 gap-2 mb-3">
            <label
              v-for="option in ZONES_TERRITOIRE"
              :key="option.value"
              :class="[
                'flex items-center justify-center px-3 py-2 rounded-xl text-sm font-medium cursor-pointer transition-all',
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
            class="w-full p-3 bg-gray-50 border border-gray-200 rounded-xl focus:outline-hidden focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            @change="emitChange"
          >
            <option value="">Tous les territoires</option>
            <option v-for="p in territoiresDisponibles" :key="p.id" :value="p.id">
              {{ p.nom }}
            </option>
          </select>
        </div>

        <!-- Statistiques -->
        <div class="pt-4 border-t border-gray-200">
          <div class="space-y-3">
            <div class="flex justify-between items-center p-3 bg-gray-50 rounded-lg">
              <span class="text-sm text-gray-600">Total salles</span>
              <span class="font-bold text-gray-900">{{ totalSalles }}</span>
            </div>
            <div class="flex justify-between items-center p-3 bg-blue-50 rounded-lg">
              <span class="text-sm text-blue-700">Résultats</span>
              <span class="font-bold text-blue-600">{{ filteredCount }}</span>
            </div>
          </div>
        </div>

        <!-- Boutons d'action -->
        <div class="space-y-3 pt-4">
          <button
            class="w-full p-3 bg-gradient-to-r from-blue-500 to-cyan-500 text-white rounded-xl font-medium hover:shadow-lg transition-all"
            @click="$emit('close')"
          >
            Appliquer les filtres
          </button>
          <button
            class="w-full p-3 bg-gray-100 text-gray-700 rounded-xl font-medium hover:bg-gray-200 transition-all"
            @click="resetAndClose"
          >
            Réinitialiser
          </button>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import type { PaysOrigineLight, SalleFiltres } from '~/composables/useAfrolang'
import { PAYS_AFRICAINS_ISO2 } from '~/constants/afripulsePaysAutorises'

const props = defineProps<{
  isOpen: boolean
  totalSalles: number
  filteredCount: number
  modelValue: SalleFiltres
  langues: string[]
  pays: PaysOrigineLight[]
}>()

const emit = defineEmits<{
  'update:modelValue': [filtres: SalleFiltres]
  close: []
  reset: []
}>()

const localFiltres = ref<SalleFiltres>({ ...props.modelValue })

// Zone géographique qui pilote le contenu du menu déroulant des territoires
const ZONES_TERRITOIRE = [
  { value: 'afrique' as const, label: 'Afrique' },
  { value: 'hors_afrique' as const, label: 'Hors Afrique' },
]
// La zone est portée par le modèle (`filtres.zone`) : elle filtre aussi la
// liste des salles, pas seulement le contenu du menu déroulant des territoires.
const zoneTerritoire = computed<'afrique' | 'hors_afrique'>({
  get: () => localFiltres.value.zone ?? 'afrique',
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

const territoiresDisponibles = computed(() =>
  props.pays.filter(p => (zoneTerritoire.value === 'afrique' ? estAfricain(p) : !estAfricain(p))),
)

watch(
  () => props.modelValue,
  (newValue) => {
    localFiltres.value = { ...newValue }
    // Aligner la zone sur le territoire sélectionné (ex. reset externe), sans
    // repasser par le setter de la computed (qui émettrait / viderait pays_id).
    const selection = props.pays.find(p => p.id === newValue.pays_id)
    if (selection) {
      localFiltres.value.zone = estAfricain(selection) ? 'afrique' : 'hors_afrique'
    }
  },
  { deep: true },
)

const emitChange = () => {
  emit('update:modelValue', { ...localFiltres.value })
}

const resetAndClose = () => {
  emit('reset')
  emit('close')
}
</script>
