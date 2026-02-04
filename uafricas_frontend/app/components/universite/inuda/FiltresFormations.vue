<template>
  <div class="bg-white rounded-lg shadow-md p-6">
    <h3 class="text-lg font-bold mb-4">Filtrer les formations</h3>

    <!-- Type de formation -->
    <div class="mb-6">
      <h4 class="text-sm font-medium text-gray-700 mb-2">Type de formation</h4>
      <div class="space-y-2">
        <label v-for="type in typesFormation" :key="type.value" class="flex items-center">
          <input type="checkbox"
                 v-model="filtres.types"
                 :value="type.value"
                 class="mr-2 rounded text-blue-600 focus:ring-3 focus:ring-blue-500">
          <span class="text-sm">{{ type.label }}</span>
        </label>
      </div>
    </div>

    <!-- Statut -->
    <div class="mb-6">
      <h4 class="text-sm font-medium text-gray-700 mb-2">Statut</h4>
      <div class="space-y-2">
        <label v-for="statut in statutsFormation" :key="statut.value" class="flex items-center">
          <input type="checkbox"
                 v-model="filtres.statuts"
                 :value="statut.value"
                 class="mr-2 rounded text-blue-600 focus:ring-3 focus:ring-blue-500">
          <span class="text-sm">{{ statut.label }}</span>
        </label>
      </div>
    </div>

    <!-- Tarification -->
    <div class="mb-6">
      <h4 class="text-sm font-medium text-gray-700 mb-2">Tarification</h4>
      <div class="space-y-2">
        <label class="flex items-center">
          <input type="radio"
                 v-model="filtres.gratuit"
                 :value="null"
                 class="mr-2 text-blue-600 focus:ring-3 focus:ring-blue-500">
          <span class="text-sm">Tous</span>
        </label>
        <label class="flex items-center">
          <input type="radio"
                 v-model="filtres.gratuit"
                 :value="true"
                 class="mr-2 text-blue-600 focus:ring-3 focus:ring-blue-500">
          <span class="text-sm">Gratuit uniquement</span>
        </label>
        <label class="flex items-center">
          <input type="radio"
                 v-model="filtres.gratuit"
                 :value="false"
                 class="mr-2 text-blue-600 focus:ring-3 focus:ring-blue-500">
          <span class="text-sm">Payant uniquement</span>
        </label>
      </div>
    </div>

    <!-- Bouton réinitialiser -->
    <button @click="reinitialiser"
            class="w-full py-2 text-sm text-gray-600 hover:text-gray-800 border border-gray-300 rounded-md hover:bg-gray-50 transition">
      Réinitialiser les filtres
    </button>
  </div>
</template>

<script setup lang="ts">
interface Filtres {
  types: string[]
  statuts: string[]
  gratuit: boolean | null
}

const emit = defineEmits<{
  (e: 'filtres-changes', filtres: Filtres): void
}>()

const filtres = reactive<Filtres>({
  types: [],
  statuts: [],
  gratuit: null
})

const typesFormation = [
  { value: 'mooc', label: 'MOOC' },
  { value: 'clom', label: 'CLOM' },
  { value: 'atelier', label: 'Atelier' },
  { value: 'concertation', label: 'Concertation' }
]

const statutsFormation = [
  { value: 'inscriptions_ouvertes', label: 'Inscriptions ouvertes' },
  { value: 'programme', label: 'Programmé' },
  { value: 'en_cours', label: 'En cours' },
  { value: 'termine', label: 'Terminé' }
]

const reinitialiser = () => {
  filtres.types = []
  filtres.statuts = []
  filtres.gratuit = null
}

// Émettre les changements de filtres
watch(filtres, (newFiltres) => {
  emit('filtres-changes', { ...newFiltres })
}, { deep: true })
</script>
