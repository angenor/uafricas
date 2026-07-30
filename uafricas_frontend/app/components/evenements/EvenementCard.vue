<template>
  <div class="bg-white rounded-lg overflow-hidden shadow-md hover:shadow-lg transition-all duration-300">
    <!-- Image -->
    <div class="h-48 relative overflow-hidden">
      <img
        :src="evenement.couverture_url || 'https://via.placeholder.com/400x250?text=Image+non+disponible'"
        :alt="evenement.titre"
        class="w-full h-full object-cover transition-transform hover:scale-105 duration-500"
      />
      <div class="absolute top-0 right-0 bg-custom-chocolat text-white text-xs px-2 py-1 rounded-bl-md">
        {{ evenement.type }}
      </div>
      <!-- Partage réseaux sociaux -->
      <div class="absolute top-2 left-2">
        <EvenementsEvenementPartage
          variant="compact"
          :path="`/evenements/${evenement.id}`"
          :evenement-id="evenement.id"
          :titre="evenement.titre"
        />
      </div>
    </div>

    <!-- Contenu -->
    <div class="p-4">
      <h3 class="text-xl font-bold text-custom-chocolat mb-2 line-clamp-2">
        {{ evenement.titre }}
      </h3>
      <span
        v-if="evenement.thematique"
        class="inline-block bg-custom-green/10 text-custom-green text-xs font-medium px-2 py-1 rounded-full mb-2"
      >
        {{ evenement.thematique }}
      </span>
      <p class="text-sm text-gray-600 mb-3 line-clamp-2">
        {{ evenement.description }}
      </p>

      <!-- Lieu -->
      <div class="flex items-center text-sm text-gray-500 mb-2">
        <font-awesome-icon icon="fa-solid fa-location-dot" class="mr-2 text-custom-green" />
        <span>{{ evenement.pays }}, {{ evenement.ville }}</span>
      </div>

      <!-- Date -->
      <div class="flex items-center text-sm text-gray-500 mb-2">
        <font-awesome-icon icon="fa-regular fa-calendar" class="mr-2 text-custom-green" />
        <span>{{ formatDateShort(evenement.date_heure_debut) }}</span>
      </div>

      <!-- Horaire -->
      <div class="flex items-center text-sm text-gray-500 mb-4">
        <font-awesome-icon icon="fa-regular fa-clock" class="mr-2 text-custom-green" />
        <span>{{ getHeure(evenement.date_heure_debut) }} - {{ getHeure(evenement.date_heure_fin) }}</span>
      </div>

      <!-- Footer: Organisateur + Bouton -->
      <div class="flex justify-between items-center mt-4 pt-4 border-t border-gray-100">
        <div class="flex items-center">
          <img
            :src="evenement.user?.photo_url || 'https://via.placeholder.com/40?text=U'"
            :alt="`${evenement.user?.prenom} ${evenement.user?.nom}`"
            class="w-8 h-8 rounded-full mr-2 object-cover"
          />
          <span class="text-xs text-gray-600">
            {{ evenement.user?.prenom }} {{ evenement.user?.nom }}
          </span>
        </div>
        <NuxtLink :to="`/evenements/${evenement.id}`">
          <button class="text-white text-sm whitespace-nowrap rounded-full bg-custom-green px-3 py-1 hover:scale-105 transition-all">
            Détails
            <font-awesome-icon icon="fa-solid fa-arrow-right" class="ml-1" />
          </button>
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { formatDateShort, getHeure, type EvenementAPI } from '~/composables/useEvenements'

defineProps<{
  evenement: EvenementAPI
}>()
</script>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
