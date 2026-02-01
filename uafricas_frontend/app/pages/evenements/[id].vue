<template>
  <div class="min-h-screen">
    <div class="w-full h-full">
      <!-- Breadcrumb -->
      <div class="backdrop-blur-sm py-3 px-4 md:px-72 mb-4">
        <CommonBreadcrumbNav :custom-breadcrumbs="breadcrumbs" />
      </div>

      <!-- Contenu -->
      <div v-if="evenement" class="bg-white mx-4 md:mx-72 pt-6 px-7 pb-20 rounded-b-md shadow-md">
        <!-- Info lieu et date -->
        <div class="p-3 bg-slate-100 rounded-r-md shadow-md border border-l-4 border-l-custom-green">
          <div class="flex flex-wrap text-custom-chocolat gap-2">
            <div>
              {{ evenement.pays }} -
              <span class="font-bold underline">{{ evenement.ville }}</span>
            </div>
            <div class="mx-2 hidden md:block">|</div>
            <div class="text-custom-green">
              {{ formatDate(evenement.created_at) }}
            </div>
          </div>
        </div>

        <!-- Titre -->
        <h1 class="font-bold text-4xl mb-4 mt-2">
          {{ evenement.titre }}
        </h1>

        <!-- Image de couverture -->
        <img
          class="w-full mb-3 h-64 md:h-96 object-cover rounded-xl"
          :src="evenement.couverture_url"
          :alt="evenement.titre"
        />

        <!-- Infos supplémentaires -->
        <div class="flex flex-wrap gap-4 mb-4">
          <div class="flex items-center text-gray-600">
            <font-awesome-icon icon="fa-regular fa-calendar" class="mr-2 text-custom-green" />
            {{ formatDateShort(evenement.date_heure_debut) }}
          </div>
          <div class="flex items-center text-gray-600">
            <font-awesome-icon icon="fa-regular fa-clock" class="mr-2 text-custom-green" />
            {{ getHeure(evenement.date_heure_debut) }} - {{ getHeure(evenement.date_heure_fin) }}
          </div>
          <div class="flex items-center">
            <span class="px-3 py-1 bg-custom-chocolat text-white text-sm rounded-full">
              {{ evenement.type }}
            </span>
          </div>
        </div>

        <!-- Bouton inscription -->
        <div v-if="isAuthenticated" class="mb-4">
          <button
            v-if="isInscrit"
            class="rounded-md text-custom-green border-2 border-custom-green px-4 py-1 hover:scale-105 transition-all italic"
          >
            Inscription envoyée
          </button>
          <button
            v-else
            @click="handleInscription"
            class="text-white rounded-md bg-custom-green px-4 py-1 hover:scale-105 transition-all"
          >
            S'inscrire
          </button>
        </div>
        <div
          v-else
          class="border p-3 text-red-600 bg-red-600 border-red-600 bg-opacity-10 rounded-md mb-4"
        >
          <div class="mb-2">Connectez-vous pour pouvoir vous inscrire</div>
          <NuxtLink
            to="/login"
            class="text-white rounded-md bg-custom-green px-4 py-1 hover:scale-105 transition-all inline-block"
          >
            Connexion
          </NuxtLink>
        </div>

        <!-- Description -->
        <div class="mt-4 relative -bottom-px p-2 inline-flex font-bold text-lg border-t-4 border-l-4 border-r-4 rounded-tl-md rounded-tr-md border-custom-green">
          Description
        </div>
        <div class="bg-slate-100 rounded-r-md shadow-md border border-l-4 border-l-custom-green p-3">
          {{ evenement.description }}
        </div>

        <!-- Organisateur -->
        <div class="mt-6 p-4 bg-gray-50 rounded-lg">
          <h3 class="font-bold text-lg mb-3">Organisateur</h3>
          <div class="flex items-center">
            <img
              :src="evenement.user.photo_url || 'https://via.placeholder.com/48'"
              :alt="`${evenement.user.prenom} ${evenement.user.nom}`"
              class="w-12 h-12 rounded-full object-cover mr-3"
            />
            <div>
              <div class="font-semibold">{{ evenement.user.prenom }} {{ evenement.user.nom }}</div>
              <div class="text-sm text-gray-500">{{ evenement.user.email }}</div>
            </div>
          </div>
        </div>
      </div>

      <!-- État non trouvé -->
      <div v-else class="text-center py-16">
        <div class="text-5xl text-gray-300 mb-4">
          <font-awesome-icon icon="fa-solid fa-calendar-xmark" />
        </div>
        <h3 class="text-xl font-semibold text-gray-500">
          Événement non trouvé
        </h3>
        <NuxtLink to="/evenements/liste" class="mt-4 inline-block text-custom-green hover:underline">
          Retour à la liste des événements
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { getEvenementById, formatDate, formatDateShort, getHeure, type Evenement } from '~/mocks/evenements'

const route = useRoute()
const evenementId = route.params.id as string

const evenement = ref<Evenement | undefined>(undefined)
const isInscrit = ref(false)
const isAuthenticated = ref(false) // En mode mock, l'utilisateur n'est pas connecté

onMounted(() => {
  evenement.value = getEvenementById(evenementId)
})

const breadcrumbs = computed(() => [
  { label: 'Centre Culturel', to: '/africa-culture' },
  { label: 'Promotion des Valeurs', to: '/promotion-valeur' },
  { label: 'Événements', to: '/evenements' },
  { label: 'Liste', to: '/evenements/liste' },
  { label: evenement.value?.titre || 'Détail', to: null }
])

useHead({
  title: computed(() => evenement.value ? `${evenement.value.titre} | UAfricas` : 'Événement | UAfricas')
})

const handleInscription = () => {
  isInscrit.value = true
  alert('Inscription envoyée avec succès ! (Mode démo)')
}
</script>
