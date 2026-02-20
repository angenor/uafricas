<template>
  <div class="min-h-screen flex flex-col bg-gray-50">
    <!-- Hero Section -->
    <div class="relative h-80">
      <div class="absolute inset-0 bg-linear-to-r from-custom-chocolat to-black/90"></div>

      <div class="absolute inset-0 flex flex-col items-center justify-center mt-14">
        <h1 class="text-white text-4xl md:text-5xl font-bold mb-4 animate-title">
          Événements & Ateliers
        </h1>
        <div class="h-1 w-24 bg-custom-green rounded animate-line"></div>
        <p class="text-white text-xl md:text-2xl mt-4 animate-subtitle">
          Découvrez nos événements
        </p>
      </div>
    </div>

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

      <!-- Chargement -->
      <div v-if="chargement" class="text-center py-16">
        <div class="text-4xl text-gray-300 mb-4 animate-spin inline-block">
          <font-awesome-icon icon="fa-solid fa-spinner" />
        </div>
        <p class="text-gray-500">Chargement des événements...</p>
      </div>

      <!-- Erreur -->
      <div v-else-if="erreur" class="text-center py-16">
        <div class="text-5xl text-red-300 mb-4">
          <font-awesome-icon icon="fa-solid fa-triangle-exclamation" />
        </div>
        <h3 class="text-xl font-semibold text-gray-500">
          Erreur de chargement
        </h3>
        <p class="text-gray-400 mt-2">{{ erreur }}</p>
        <button
          @click="chargerEvenements"
          class="mt-4 text-white bg-custom-green rounded-md py-2 px-4 hover:bg-custom-green/90 transition-colors"
        >
          Réessayer
        </button>
      </div>

      <!-- Grille d'événements -->
      <div v-else-if="evenements.length" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <EvenementsEvenementCard
          v-for="evenement in evenements"
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
import { useEvenements, type EvenementAPI, type EvenementFiltres } from '~/composables/useEvenements'

useHead({
  title: 'Liste des Événements | UAfricas'
})

const breadcrumbs = [
  { label: 'Centre Culturel', to: '/africa-culture' },
  { label: 'Promotion des Valeurs', to: '/promotion-valeur' },
  { label: 'Événements', to: '/evenements' },
  { label: 'Liste', to: undefined }
]

const { listerEvenements, creerEvenement, chargement, erreur } = useEvenements()

const showModal = ref(false)
const anneeSelected = ref(new Date().getFullYear().toString())
const filtreType = ref('')
const filtrePays = ref('')
const evenements = ref<EvenementAPI[]>([])

const chargerEvenements = async () => {
  const filtres: EvenementFiltres = {
    annee: parseInt(anneeSelected.value),
    format: filtreType.value || undefined,
    pays: filtrePays.value || undefined,
    par_page: 50,
  }
  const data = await listerEvenements(filtres)
  evenements.value = data?.evenements ?? []
}

watch([anneeSelected, filtreType, filtrePays], chargerEvenements)
onMounted(chargerEvenements)

const handleSubmit = async (data: any) => {
  const result = await creerEvenement(
    {
      titre: data.titre,
      description: data.description,
      type: data.type,
      pays: data.pays,
      ville: data.ville,
      date_heure_debut: data.date_heure_debut,
      date_heure_fin: data.date_heure_fin,
    },
    data.couverture_file,
  )
  if (result) {
    showModal.value = false
    await chargerEvenements()
  }
}
</script>

<style scoped>
@reference "~/assets/css/main.css";

@keyframes fadeIn {
  from { opacity: 0; transform: translateY(-20px); }
  to { opacity: 1; transform: translateY(0); }
}

@keyframes expandLine {
  from { width: 0; }
  to { width: 6rem; }
}

.animate-title {
  animation: fadeIn 1s ease-out forwards;
}

.animate-subtitle {
  animation: fadeIn 1s ease-out 0.3s forwards;
  opacity: 0;
}

.animate-line {
  animation: expandLine 1.2s ease-out 0.1s forwards;
  width: 0;
}
</style>
