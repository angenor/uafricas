<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Chargement -->
    <div v-if="chargement" class="min-h-screen flex items-center justify-center">
      <div class="text-center">
        <div class="text-5xl text-custom-green mb-4 animate-spin inline-block">
          <font-awesome-icon icon="fa-solid fa-spinner" />
        </div>
        <p class="text-gray-500 text-lg">Chargement de l'événement...</p>
      </div>
    </div>

    <!-- Contenu principal -->
    <template v-else-if="evenement">
      <!-- Hero Section -->
      <div class="relative h-72 lg:h-96 w-full overflow-hidden">
        <img
          :src="evenement.couverture_url || '/images/placeholder-evenement.jpg'"
          :alt="evenement.titre"
          class="absolute inset-0 w-full h-full object-cover"
        />
        <div class="absolute inset-0 bg-gradient-to-t from-black/70 via-black/30 to-black/10" />

        <!-- Badges sur le hero (top-28 pour passer sous la navbar h-24) -->
        <div class="absolute top-28 left-4 flex flex-wrap gap-2">
          <span class="px-3 py-1 text-sm font-semibold rounded-full text-white" :class="badgeFormatClasses">
            <font-awesome-icon :icon="formatIcon" class="mr-1" />
            {{ evenement.type }}
          </span>
          <span class="px-3 py-1 text-sm font-semibold rounded-full" :class="badgeStatutClasses">
            {{ labelStatut }}
          </span>
        </div>

        <!-- Titre sur le hero (pb-14 pour rester visible au-dessus de la carte -mt-10) -->
        <div class="absolute bottom-0 left-0 right-0 px-6 pb-14 pt-6 lg:px-16">
          <h1 class="text-white text-2xl lg:text-4xl font-bold font-display leading-tight drop-shadow-lg">
            {{ evenement.titre }}
          </h1>
        </div>
      </div>

      <!-- Carte principale (negative margin sur le hero) -->
      <div class="mx-4 md:mx-16 lg:mx-40 xl:mx-72 -mt-10 relative z-10 mb-12">
        <!-- Breadcrumb -->
        <div class="bg-white rounded-t-xl px-6 pt-4 pb-2 shadow-xl">
          <CommonBreadcrumbNav :custom-breadcrumbs="breadcrumbs" />
        </div>

        <!-- Contenu carte -->
        <div class="bg-white px-6 lg:px-8 pb-10 rounded-b-xl shadow-xl">
          <!-- Grille infos rapides -->
          <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3 py-5 border-b border-gray-100">
            <!-- Date -->
            <div class="flex items-center gap-3 p-3 bg-gray-50 rounded-lg">
              <div class="w-10 h-10 bg-custom-green/10 rounded-lg flex items-center justify-center shrink-0">
                <font-awesome-icon icon="fa-regular fa-calendar" class="text-custom-green" />
              </div>
              <div>
                <div class="text-xs text-gray-400 uppercase font-semibold">Date</div>
                <div class="text-sm font-semibold text-gray-800">{{ formatDateShort(evenement.date_heure_debut) }}</div>
              </div>
            </div>

            <!-- Horaire -->
            <div class="flex items-center gap-3 p-3 bg-gray-50 rounded-lg">
              <div class="w-10 h-10 bg-custom-green/10 rounded-lg flex items-center justify-center shrink-0">
                <font-awesome-icon icon="fa-regular fa-clock" class="text-custom-green" />
              </div>
              <div>
                <div class="text-xs text-gray-400 uppercase font-semibold">Horaire</div>
                <div class="text-sm font-semibold text-gray-800">
                  {{ getHeure(evenement.date_heure_debut) }}
                  <span v-if="evenement.date_heure_fin"> - {{ getHeure(evenement.date_heure_fin) }}</span>
                </div>
              </div>
            </div>

            <!-- Lieu -->
            <div class="flex items-center gap-3 p-3 bg-gray-50 rounded-lg">
              <div class="w-10 h-10 bg-custom-chocolat/10 rounded-lg flex items-center justify-center shrink-0">
                <font-awesome-icon icon="fa-solid fa-location-dot" class="text-custom-chocolat" />
              </div>
              <div>
                <div class="text-xs text-gray-400 uppercase font-semibold">Lieu</div>
                <div class="text-sm font-semibold text-gray-800">
                  <span v-if="evenement.ville">{{ evenement.ville }}, </span>
                  {{ evenement.pays || 'Non précisé' }}
                </div>
              </div>
            </div>

            <!-- Places -->
            <div class="flex items-center gap-3 p-3 bg-gray-50 rounded-lg">
              <div class="w-10 h-10 bg-custom-green/10 rounded-lg flex items-center justify-center shrink-0">
                <font-awesome-icon icon="fa-solid fa-users" class="text-custom-green" />
              </div>
              <div>
                <div class="text-xs text-gray-400 uppercase font-semibold">Inscrits</div>
                <div class="text-sm font-semibold text-gray-800">
                  {{ evenement.nombre_inscrits }}
                  <span v-if="evenement.nombre_places" class="text-gray-400 font-normal">/ {{ evenement.nombre_places }}</span>
                </div>
              </div>
            </div>
          </div>

          <!-- Adresse détaillée -->
          <div v-if="evenement.adresse" class="flex items-start gap-2 mt-4 p-3 bg-amber-50 rounded-lg border border-amber-100">
            <font-awesome-icon icon="fa-solid fa-map-pin" class="text-amber-600 mt-0.5" />
            <span class="text-sm text-amber-800">{{ evenement.adresse }}</span>
          </div>

          <!-- Lien en ligne -->
          <div v-if="evenement.lien_en_ligne" class="flex items-center gap-2 mt-3 p-3 bg-blue-50 rounded-lg border border-blue-100">
            <font-awesome-icon icon="fa-solid fa-video" class="text-blue-600" />
            <a :href="evenement.lien_en_ligne" target="_blank" class="text-sm text-blue-700 hover:underline font-medium">
              Rejoindre en ligne
              <font-awesome-icon icon="fa-solid fa-arrow-up-right-from-square" class="ml-1 text-xs" />
            </a>
          </div>

          <!-- Inscription -->
          <div class="mt-6 mb-6">
            <div v-if="isAuthenticated">
              <button
                v-if="isInscrit"
                class="inline-flex items-center gap-2 rounded-lg text-custom-green border-2 border-custom-green px-6 py-2.5 font-semibold cursor-default"
              >
                <font-awesome-icon icon="fa-solid fa-circle-check" />
                Inscription confirmée
              </button>
              <button
                v-else
                @click="handleInscription"
                class="inline-flex items-center gap-2 text-white rounded-lg bg-custom-green px-6 py-2.5 font-semibold hover:brightness-110 hover:scale-[1.02] transition-all shadow-md"
              >
                <font-awesome-icon icon="fa-solid fa-user-plus" />
                S'inscrire à cet événement
              </button>
            </div>
            <div v-else class="flex items-center gap-4 p-4 bg-red-50 border border-red-200 rounded-lg">
              <font-awesome-icon icon="fa-solid fa-lock" class="text-red-400 text-lg" />
              <div class="flex-1">
                <div class="text-sm text-red-700 font-medium">Connectez-vous pour vous inscrire à cet événement</div>
              </div>
              <NuxtLink
                to="/login"
                class="text-white rounded-lg bg-custom-green px-4 py-2 text-sm font-semibold hover:brightness-110 transition-all shrink-0"
              >
                Connexion
              </NuxtLink>
            </div>
          </div>

          <!-- Description -->
          <div class="border-t border-gray-100 pt-6">
            <h2 class="flex items-center gap-2 text-xl font-bold text-gray-800 mb-3">
              <font-awesome-icon icon="fa-solid fa-align-left" class="text-custom-green text-base" />
              Description
            </h2>
            <div class="text-gray-600 leading-relaxed prose prose-sm max-w-none" v-html="evenement.description" />
          </div>

          <!-- Organisateur -->
          <div class="border-t border-gray-100 mt-8 pt-6">
            <h2 class="flex items-center gap-2 text-xl font-bold text-gray-800 mb-4">
              <font-awesome-icon icon="fa-solid fa-user-tie" class="text-custom-chocolat text-base" />
              Organisateur
            </h2>
            <div class="flex items-center gap-4 p-4 bg-gray-50 rounded-xl">
              <!-- Avatar avec initiales en fallback -->
              <div v-if="evenement.user.photo_url" class="shrink-0">
                <img
                  :src="evenement.user.photo_url"
                  :alt="`${evenement.user.prenom} ${evenement.user.nom}`"
                  class="w-14 h-14 rounded-full object-cover ring-2 ring-custom-green/20"
                />
              </div>
              <div v-else class="w-14 h-14 rounded-full bg-gradient-to-br from-custom-green to-custom-chocolat flex items-center justify-center shrink-0">
                <span class="text-white font-bold text-lg">
                  {{ (evenement.user.prenom?.[0] || '').toUpperCase() }}{{ (evenement.user.nom?.[0] || '').toUpperCase() }}
                </span>
              </div>
              <div>
                <div class="font-semibold text-gray-800 text-lg">
                  {{ evenement.user.prenom }} {{ evenement.user.nom }}
                </div>
                <div class="text-sm text-gray-500">{{ evenement.user.email }}</div>
              </div>
            </div>
          </div>

          <!-- Retour -->
          <div class="mt-8 pt-6 border-t border-gray-100">
            <NuxtLink to="/evenements/liste" class="inline-flex items-center gap-2 text-custom-green hover:underline font-medium">
              <font-awesome-icon icon="fa-solid fa-arrow-left" />
              Retour à la liste des événements
            </NuxtLink>
          </div>
        </div>
      </div>
    </template>

    <!-- État non trouvé -->
    <div v-else class="min-h-screen flex items-center justify-center">
      <div class="text-center">
        <div class="text-6xl text-gray-200 mb-6">
          <font-awesome-icon icon="fa-solid fa-calendar-xmark" />
        </div>
        <h3 class="text-2xl font-semibold text-gray-500 mb-2">Événement non trouvé</h3>
        <p class="text-gray-400 mb-6">Cet événement n'existe pas ou a été supprimé.</p>
        <NuxtLink
          to="/evenements/liste"
          class="inline-flex items-center gap-2 text-white bg-custom-green px-5 py-2.5 rounded-lg font-semibold hover:brightness-110 transition-all"
        >
          <font-awesome-icon icon="fa-solid fa-arrow-left" />
          Voir tous les événements
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useEvenements, formatDateShort, getHeure, type EvenementDetailAPI } from '~/composables/useEvenements'
import { useUserStore } from '~/stores/user'

const route = useRoute()
const evenementId = route.params.id as string
const userStore = useUserStore()

const { obtenirEvenement, inscrireEvenement, chargement, erreur } = useEvenements()

const evenement = ref<EvenementDetailAPI | null>(null)
const isInscrit = ref(false)
const isAuthenticated = computed(() => !!userStore.accessToken)

onMounted(async () => {
  const data = await obtenirEvenement(evenementId)
  if (data) {
    evenement.value = data
    isInscrit.value = data.est_inscrit
  }
})

const breadcrumbs = computed(() => [
  { label: 'Centre Culturel', to: '/africa-culture' },
  { label: 'Événements', to: '/evenements' },
  { label: 'Liste', to: '/evenements/liste' },
  { label: evenement.value?.titre || 'Détail', to: undefined }
])

useHead({
  title: computed(() => evenement.value ? `${evenement.value.titre} | UAfricas` : 'Événement | UAfricas')
})

// --- Computed pour les badges ---

const badgeFormatClasses = computed(() => {
  const type = evenement.value?.type || ''
  if (type.includes('ligne')) return 'bg-blue-600'
  if (type.includes('Hybride') || type.includes('hybride')) return 'bg-purple-600'
  return 'bg-custom-chocolat'
})

const formatIcon = computed(() => {
  const type = evenement.value?.type || ''
  if (type.includes('ligne')) return 'fa-solid fa-video'
  if (type.includes('Hybride') || type.includes('hybride')) return 'fa-solid fa-shuffle'
  return 'fa-solid fa-map-marker-alt'
})

const badgeStatutClasses = computed(() => {
  switch (evenement.value?.statut) {
    case 'a_venir': return 'bg-green-100 text-green-700'
    case 'en_cours': return 'bg-blue-100 text-blue-700'
    case 'termine': return 'bg-gray-200 text-gray-600'
    case 'annule': return 'bg-red-100 text-red-700'
    default: return 'bg-gray-200 text-gray-600'
  }
})

const labelStatut = computed(() => {
  switch (evenement.value?.statut) {
    case 'a_venir': return 'À venir'
    case 'en_cours': return 'En cours'
    case 'termine': return 'Terminé'
    case 'annule': return 'Annulé'
    default: return evenement.value?.statut || ''
  }
})

const handleInscription = async () => {
  const success = await inscrireEvenement(evenementId)
  if (success) {
    isInscrit.value = true
  }
}
</script>
