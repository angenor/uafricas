<template>
  <div class="bg-white rounded-lg shadow-md hover:shadow-xl transition-all duration-300 overflow-hidden cursor-pointer"
       @click="$emit('click', formation)">
    <!-- Header avec type et tarif -->
    <div class="px-6 py-4 border-b border-gray-100">
      <div class="flex items-center justify-between">
        <span class="px-3 py-1 rounded-full text-sm font-medium"
              :class="getTypeClasses(formation.type)">
          {{ getTypeLabel(formation.type) }}
        </span>
        <div class="text-right">
          <span v-if="formation.tarification.gratuit" class="text-green-600 font-bold">
            Gratuit
          </span>
          <span v-else class="text-gray-700 font-bold">
            {{ formation.tarification.prix.toLocaleString() }} FCFA
          </span>
          <div v-if="formation.tarification.prixReduit?.length" class="text-xs text-gray-500">
            Réductions disponibles
          </div>
        </div>
      </div>
    </div>

    <!-- Contenu principal -->
    <div class="p-6">
      <h3 class="text-xl font-bold text-gray-900 mb-2">{{ formation.titre }}</h3>
      <p class="text-gray-600 mb-4 line-clamp-2">{{ formation.resume || formation.description }}</p>

      <!-- Formateur -->
      <div class="flex items-center mb-4">
        <img v-if="formation.formateurPhotoURL"
             :src="formation.formateurPhotoURL"
             :alt="`${formation.formateurPrenom} ${formation.formateurNom}`"
             class="w-10 h-10 rounded-full mr-3 object-cover">
        <div v-else class="w-10 h-10 rounded-full mr-3 bg-gray-300 flex items-center justify-center">
          <svg class="w-6 h-6 text-gray-600" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" clip-rule="evenodd"></path>
          </svg>
        </div>
        <div>
          <p class="font-medium text-sm">{{ formation.formateurPrenom }} {{ formation.formateurNom }}</p>
          <p class="text-xs text-gray-500">Formateur</p>
        </div>
      </div>

      <!-- Informations clés -->
      <div class="grid grid-cols-2 gap-4 mb-4">
        <!-- Dates -->
        <div>
          <p class="text-xs text-gray-500 mb-1">Début</p>
          <p class="text-sm font-medium">{{ formatDate(formation.dateDebut) }}</p>
        </div>

        <!-- Durée -->
        <div>
          <p class="text-xs text-gray-500 mb-1">Durée</p>
          <p class="text-sm font-medium">
            {{ formation.dureeEstimee.heures }}h
            <span v-if="formation.dureeEstimee.semaines">
              ({{ formation.dureeEstimee.semaines }} sem.)
            </span>
          </p>
        </div>
      </div>

      <!-- Tags -->
      <div class="flex flex-wrap gap-2 mb-4">
        <span class="px-2 py-1 bg-gray-100 text-gray-600 text-xs rounded">
          {{ formation.modalites.langue.toUpperCase() }}
        </span>
        <span class="px-2 py-1 bg-gray-100 text-gray-600 text-xs rounded">
          {{ getNiveauLabel(formation.modalites.niveauRequis) }}
        </span>
        <span v-if="formation.modalites.certificationDisponible"
              class="px-2 py-1 bg-yellow-100 text-yellow-800 text-xs rounded">
          Certification
        </span>
      </div>

      <!-- Statut et capacité -->
      <div class="flex items-center justify-between text-sm">
        <div class="flex items-center">
          <span class="w-2 h-2 rounded-full mr-2"
                :class="getStatutColor(formation.statut)"></span>
          <span class="text-gray-600">{{ getStatutLabel(formation.statut) }}</span>
        </div>
        <div v-if="formation.capacite.maximum" class="text-gray-500">
          {{ formation.capacite.inscritsActuels }}/{{ formation.capacite.maximum }} inscrits
        </div>
      </div>
    </div>

    <!-- Action footer -->
    <div class="px-6 py-3 bg-gray-50 border-t border-gray-100">
      <button @click.stop="$emit('inscrire', formation)"
              :disabled="!peutSInscrire(formation)"
              class="w-full py-2 rounded-md font-medium transition"
              :class="peutSInscrire(formation)
                ? 'bg-blue-600 text-white hover:bg-blue-700'
                : 'bg-gray-300 text-gray-500 cursor-not-allowed'">
        {{ getActionLabel(formation) }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Formation } from '~/mocks/inuda/formations'

defineProps<{
  formation: Formation
}>()

defineEmits<{
  (e: 'click', formation: Formation): void
  (e: 'inscrire', formation: Formation): void
}>()

const getTypeLabel = (type: string) => {
  const labels: Record<string, string> = {
    mooc: 'MOOC',
    clom: 'CLOM',
    atelier: 'Atelier',
    concertation: 'Concertation'
  }
  return labels[type] || type.toUpperCase()
}

const getTypeClasses = (type: string) => {
  const classes: Record<string, string> = {
    mooc: 'bg-blue-100 text-blue-800',
    clom: 'bg-purple-100 text-purple-800',
    atelier: 'bg-green-100 text-green-800',
    concertation: 'bg-orange-100 text-orange-800'
  }
  return classes[type] || 'bg-gray-100 text-gray-800'
}

const getNiveauLabel = (niveau: string) => {
  const labels: Record<string, string> = {
    debutant: 'Débutant',
    intermediaire: 'Intermédiaire',
    avance: 'Avancé',
    tous_niveaux: 'Tous niveaux'
  }
  return labels[niveau] || niveau
}

const getStatutLabel = (statut: string) => {
  const labels: Record<string, string> = {
    brouillon: 'Brouillon',
    programme: 'Programmé',
    inscriptions_ouvertes: 'Inscriptions ouvertes',
    complet: 'Complet',
    en_cours: 'En cours',
    termine: 'Terminé',
    annule: 'Annulé',
    archive: 'Archivé'
  }
  return labels[statut] || statut
}

const getStatutColor = (statut: string) => {
  const colors: Record<string, string> = {
    programme: 'bg-yellow-500',
    inscriptions_ouvertes: 'bg-green-500',
    complet: 'bg-red-500',
    en_cours: 'bg-blue-500',
    termine: 'bg-gray-500',
    annule: 'bg-red-500',
    archive: 'bg-gray-500'
  }
  return colors[statut] || 'bg-gray-500'
}

const peutSInscrire = (formation: Formation) => {
  return formation.statut === 'inscriptions_ouvertes' &&
         (!formation.capacite.maximum || formation.capacite.inscritsActuels < formation.capacite.maximum)
}

const getActionLabel = (formation: Formation) => {
  if (formation.statut === 'complet') return 'Complet'
  if (formation.statut === 'termine') return 'Terminé'
  if (formation.statut === 'en_cours') return 'En cours'
  if (formation.statut === 'inscriptions_ouvertes') return "S'inscrire"
  return 'Prochainement'
}

const formatDate = (date: Date) => {
  return new Intl.DateTimeFormat('fr-FR', {
    day: 'numeric',
    month: 'short',
    year: 'numeric'
  }).format(new Date(date))
}
</script>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
