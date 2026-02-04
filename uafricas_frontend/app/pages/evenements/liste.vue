<template>
  <div class="min-h-screen flex flex-col bg-gray-50">
    <!-- Hero -->
    <EvenementsEvenementHero titre="Événements & Ateliers" />

    <!-- Breadcrumb -->
    <div class="backdrop-blur-xs">
      <div class="mx-auto px-4 py-3">
        <CommonBreadcrumbNav :custom-breadcrumbs="breadcrumbs" />
      </div>
    </div>

    <!-- Contenu principal -->
    <div class="container mx-auto px-4 py-8">
      <!-- Filtres -->
      <EvenementsEvenementFilters
        v-model:annee-selected="anneeSelected"
        v-model:filtre-type="filtreType"
        v-model:filtre-pays="filtrePays"
        @open-modal="showModal = true"
        class="mb-8"
      />

      <!-- Grille d'événements -->
      <div v-if="filteredEvents.length" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <EvenementsEvenementCard
          v-for="evenement in filteredEvents"
          :key="evenement.id"
          :evenement="evenement"
        />
      </div>

      <!-- État vide -->
      <div v-else class="text-center py-16">
        <div class="text-5xl text-gray-300 mb-4">
          <font-awesome-icon icon="fa-solid fa-calendar-xmark" />
        </div>
        <h3 class="text-xl font-semibold text-gray-500">
          Aucun événement trouvé
        </h3>
        <p class="text-gray-400 mt-2">
          Essayez de modifier vos filtres ou proposez un nouvel événement
        </p>
        <button
          @click="showModal = true"
          class="mt-6 text-white bg-custom-green rounded-md py-2 px-4 hover:bg-custom-green/90 transition-colors"
        >
          Proposer un événement
        </button>
      </div>
    </div>

    <!-- Modal de création -->
    <EvenementsEvenementModal
      :show="showModal"
      @close="showModal = false"
      @submit="handleSubmit"
    />
  </div>
</template>

<script setup lang="ts">
import { evenementsMock, filterEvenements, type Evenement, type TypeEvenement } from '~/mocks/evenements'

useHead({
  title: 'Liste des Événements | UAfricas'
})

const breadcrumbs = [
  { label: 'Centre Culturel', to: '/africa-culture' },
  { label: 'Promotion des Valeurs', to: '/promotion-valeur' },
  { label: 'Événements', to: '/evenements' },
  { label: 'Liste', to: null }
]

const showModal = ref(false)
const anneeSelected = ref('2025')
const filtreType = ref('')
const filtrePays = ref('')

const filteredEvents = computed(() => {
  return filterEvenements(
    anneeSelected.value,
    filtreType.value as TypeEvenement | '',
    filtrePays.value
  )
})

const handleSubmit = (data: any) => {
  console.log('Nouvel événement:', data)
  // En mode mock, on affiche juste un message
  alert('Événement soumis avec succès ! (Mode démo)')
  showModal.value = false
}
</script>
