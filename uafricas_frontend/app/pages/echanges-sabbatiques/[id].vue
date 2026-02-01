<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Loading state -->
    <div v-if="loading" class="flex items-center justify-center min-h-screen">
      <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-custom-green"></div>
    </div>

    <!-- Not found -->
    <div v-else-if="!programme" class="flex flex-col items-center justify-center min-h-screen">
      <font-awesome-icon
        :icon="['fas', 'exclamation-triangle']"
        class="h-16 text-gray-300 mb-4"
      />
      <p class="text-xl text-gray-600 mb-4">Programme non trouvé</p>
      <NuxtLink
        to="/echanges-sabbatiques"
        class="text-custom-green hover:underline flex items-center gap-2"
      >
        <font-awesome-icon :icon="['fas', 'arrow-left']" />
        Retour à la liste
      </NuxtLink>
    </div>

    <!-- Contenu -->
    <template v-else>
      <!-- Hero avec image -->
      <div class="relative h-64 lg:h-80 w-full overflow-hidden">
        <img
          :src="programme.couvertureUrl"
          :alt="programme.titre"
          class="w-full h-full object-cover"
        />
        <div class="absolute inset-0 bg-gradient-to-t from-black/60 to-transparent"></div>

        <!-- Badge type -->
        <span
          class="absolute top-4 right-4 px-4 py-2 rounded-full text-sm font-medium"
          :class="programme.interafricain ? 'bg-custom-green text-white' : 'bg-custom-chocolat text-white'"
        >
          {{ programme.interafricain ? 'Programme Interafricain' : 'Hors Afrique vers Afrique' }}
        </span>
      </div>

      <!-- Contenu principal -->
      <div class="bg-white mx-4 md:mx-16 lg:mx-72 -mt-16 relative z-10 rounded-t-lg shadow-xl">
        <!-- Breadcrumb -->
        <CommonBreadcrumbNav class="px-7 pt-6" />

        <!-- Info bar -->
        <div
          class="mx-7 mt-4 p-3 bg-slate-100 rounded-r-md shadow-md border-l-4 border-l-custom-green flex flex-wrap gap-4 text-sm text-gray-600"
        >
          <span class="flex items-center gap-1">
            <font-awesome-icon :icon="['fas', 'location-dot']" class="text-custom-green" />
            {{ programme.pays }}
            <span v-if="programme.ville">({{ programme.ville }})</span>
          </span>
          <span class="border-l border-gray-300 pl-4 flex items-center gap-1">
            <font-awesome-icon :icon="['fas', 'calendar-days']" class="text-custom-green" />
            {{ formatDateSabbatique(programme.dateHeureDebut) }}
          </span>
          <span class="border-l border-gray-300 pl-4 flex items-center gap-1">
            <font-awesome-icon :icon="['fas', 'clock']" class="text-custom-green" />
            {{ getDureeLabel(programme.dureeProgramme) }}
          </span>
        </div>

        <!-- Titre -->
        <h1 class="px-7 pt-6 text-2xl lg:text-3xl font-bold text-gray-900">
          {{ programme.titre }}
        </h1>

        <!-- Statut -->
        <div class="px-7 pt-2">
          <span
            class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium"
            :class="getStatutClasses(programme.statut)"
          >
            {{ getStatutLabel(programme.statut) }}
          </span>
        </div>

        <!-- Image principale -->
        <div class="px-7 pt-6">
          <img
            :src="programme.couvertureUrl"
            :alt="programme.titre"
            class="w-full h-64 lg:h-100 object-cover rounded-lg shadow-md"
          />
        </div>

        <!-- Bouton intérêt -->
        <div class="px-7 pt-6">
          <div v-if="isAuthenticated" class="bg-gray-50 p-4 rounded-lg">
            <button
              class="w-full lg:w-auto bg-custom-green text-white px-6 py-3 rounded-lg font-medium hover:bg-opacity-90 transition-all flex items-center justify-center gap-2"
              @click="envoyerInteret"
            >
              <font-awesome-icon :icon="['fas', 'heart']" />
              Je suis intéressé(e) par ce programme
            </button>
            <p class="text-sm text-gray-500 mt-2">
              Vous serez contacté(e) par l'organisateur pour plus d'informations.
            </p>
          </div>
          <div v-else class="bg-yellow-50 border border-yellow-200 p-4 rounded-lg">
            <p class="text-gray-700 mb-3">
              Connectez-vous pour manifester votre intérêt pour ce programme.
            </p>
            <NuxtLink
              to="/login"
              class="inline-flex items-center gap-2 bg-custom-chocolat text-white px-4 py-2 rounded-lg hover:bg-opacity-90 transition-all"
            >
              <font-awesome-icon :icon="['fas', 'sign-in-alt']" />
              Se connecter
            </NuxtLink>
          </div>
        </div>

        <!-- Description -->
        <section class="px-7 pt-8">
          <h2 class="text-xl font-bold text-gray-900 mb-4 flex items-center gap-2">
            <font-awesome-icon :icon="['fas', 'info-circle']" class="text-custom-green" />
            Description du programme
          </h2>
          <p class="text-gray-700 leading-relaxed whitespace-pre-line">
            {{ programme.description }}
          </p>
        </section>

        <!-- Informations détaillées -->
        <section class="px-7 pt-8 pb-12">
          <h2 class="text-xl font-bold text-gray-900 mb-4 flex items-center gap-2">
            <font-awesome-icon :icon="['fas', 'list-check']" class="text-custom-green" />
            Informations pratiques
          </h2>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Domaine -->
            <div class="bg-gray-50 p-4 rounded-lg">
              <p class="text-sm text-gray-500 mb-1">Domaine d'intervention</p>
              <p class="font-medium text-gray-900">{{ getDomaineLabel(programme.domaine) }}</p>
            </div>

            <!-- Durée -->
            <div class="bg-gray-50 p-4 rounded-lg">
              <p class="text-sm text-gray-500 mb-1">Durée du programme</p>
              <p class="font-medium text-gray-900">{{ getDureeLabel(programme.dureeProgramme) }}</p>
            </div>

            <!-- Dates -->
            <div class="bg-gray-50 p-4 rounded-lg">
              <p class="text-sm text-gray-500 mb-1">Période</p>
              <p class="font-medium text-gray-900">
                Du {{ formatDateCourteSabbatique(programme.dateHeureDebut) }}
                au {{ formatDateCourteSabbatique(programme.dateHeureFin) }}
              </p>
            </div>

            <!-- Lieu -->
            <div class="bg-gray-50 p-4 rounded-lg">
              <p class="text-sm text-gray-500 mb-1">Lieu</p>
              <p class="font-medium text-gray-900">
                {{ programme.ville ? `${programme.ville}, ` : '' }}{{ programme.pays }}
              </p>
            </div>

            <!-- Prise en charge -->
            <div class="bg-gray-50 p-4 rounded-lg md:col-span-2">
              <p class="text-sm text-gray-500 mb-2">Prise en charge par l'organisation</p>
              <div class="flex flex-wrap gap-2">
                <span
                  v-for="prise in getPriseEnChargeLabels(programme.priseEnCharge)"
                  :key="prise"
                  class="px-3 py-1 bg-custom-green/10 text-custom-green rounded-full text-sm font-medium"
                >
                  <font-awesome-icon :icon="['fas', 'check']" class="mr-1" />
                  {{ prise }}
                </span>
              </div>
            </div>

            <!-- Organisateur -->
            <div v-if="programme.organisateurNom" class="bg-gray-50 p-4 rounded-lg md:col-span-2">
              <p class="text-sm text-gray-500 mb-1">Organisateur</p>
              <p class="font-medium text-gray-900">{{ programme.organisateurNom }}</p>
              <p v-if="programme.organisateurEmail" class="text-sm text-gray-600">
                {{ programme.organisateurEmail }}
              </p>
            </div>
          </div>
        </section>

        <!-- Bouton retour -->
        <div class="px-7 pb-12">
          <NuxtLink
            to="/echanges-sabbatiques"
            class="inline-flex items-center gap-2 text-custom-chocolat hover:underline"
          >
            <font-awesome-icon :icon="['fas', 'arrow-left']" />
            Retour à la liste des programmes
          </NuxtLink>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import {
  getSabbatiqueById,
  formatDateSabbatique,
  formatDateCourteSabbatique,
  getDureeLabel,
  getDomaineLabel,
  getPriseEnChargeLabels,
  type ProgrammeSabbatique
} from '~/mocks/sabbatiques'
import { useUserStore } from '~/stores/user'

const route = useRoute()
const userStore = useUserStore()

const loading = ref(true)
const programme = ref<ProgrammeSabbatique | null>(null)

const isAuthenticated = computed(() => userStore.isAuthenticated)

const getStatutLabel = (statut: string) => {
  const labels: Record<string, string> = {
    ouvert: 'Inscriptions ouvertes',
    en_cours: 'En cours',
    termine: 'Terminé',
    complet: 'Complet'
  }
  return labels[statut] || statut
}

const getStatutClasses = (statut: string) => {
  const classes: Record<string, string> = {
    ouvert: 'bg-green-100 text-green-800',
    en_cours: 'bg-blue-100 text-blue-800',
    termine: 'bg-gray-100 text-gray-800',
    complet: 'bg-red-100 text-red-800'
  }
  return classes[statut] || 'bg-gray-100 text-gray-800'
}

const envoyerInteret = () => {
  alert('Votre intérêt a été enregistré ! L\'organisateur vous contactera prochainement.')
}

onMounted(() => {
  const id = route.params.id as string
  programme.value = getSabbatiqueById(id) || null

  if (programme.value) {
    useHead({
      title: `${programme.value.titre} - Échanges Sabbatiques - UAfricas`,
      meta: [
        {
          name: 'description',
          content: programme.value.description.substring(0, 160)
        }
      ]
    })
  }

  loading.value = false
})
</script>
