<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Loading -->
    <div v-if="chargement" class="flex items-center justify-center h-screen">
      <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
    </div>

    <!-- Formation non trouvee -->
    <div v-else-if="!formation" class="flex flex-col items-center justify-center h-screen">
      <svg class="w-16 h-16 text-gray-400 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
      </svg>
      <h1 class="text-2xl font-bold text-gray-700 mb-2">Formation non trouvée</h1>
      <p class="text-gray-500 mb-4">La formation que vous recherchez n'existe pas.</p>
      <NuxtLink to="/universite/inuda/formations" class="text-blue-600 hover:underline">
        ← Retour à la liste des formations
      </NuxtLink>
    </div>

    <!-- Contenu -->
    <template v-else>
      <!-- Hero Section -->
      <div class="relative bg-gradient-to-r"
           :class="getTypeGradient(formation.type)">
        <div class="absolute inset-0 bg-black/30"></div>
        <div class="relative px-4 md:px-8 pt-16 pb-6">
          <div class="max-w-6xl mx-auto w-full">
            <CommonBreadcrumbNav :custom-breadcrumbs="breadcrumbs" class="mb-4 text-white" />
            <div class="flex items-center gap-3 mb-3">
              <span class="px-3 py-1 bg-white/20 backdrop-blur-xs text-white rounded-full text-sm font-medium">
                {{ getTypeLabel(formation.type) }}
              </span>
              <span :class="getStatutBadgeClass(formation.statut)">
                {{ getStatutLabel(formation.statut) }}
              </span>
            </div>
            <h1 class="text-3xl md:text-4xl font-bold text-white mb-2">{{ formation.titre }}</h1>
          </div>
        </div>
      </div>

      <!-- Contenu principal -->
      <div class="max-w-6xl mx-auto px-4 py-8">
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
          <!-- Colonne principale -->
          <div class="lg:col-span-2 space-y-8">
            <!-- Description -->
            <div class="bg-white rounded-lg shadow-md p-6">
              <h2 class="text-2xl font-bold mb-4">À propos de cette formation</h2>
              <p class="text-gray-700 leading-relaxed whitespace-pre-line">{{ formation.description }}</p>
            </div>

            <!-- Formateur -->
            <div class="bg-white rounded-lg shadow-md p-6">
              <h2 class="text-2xl font-bold mb-4">Votre formateur</h2>
              <div class="flex items-start gap-4">
                <img v-if="formation.formateur.photo_url"
                     :src="formation.formateur.photo_url"
                     :alt="`${formation.formateur.prenom} ${formation.formateur.nom}`"
                     class="w-20 h-20 rounded-full object-cover">
                <div v-else class="w-20 h-20 rounded-full bg-gray-200 flex items-center justify-center">
                  <svg class="w-10 h-10 text-gray-400" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" clip-rule="evenodd"></path>
                  </svg>
                </div>
                <div>
                  <h3 class="text-xl font-semibold">{{ formation.formateur.prenom }} {{ formation.formateur.nom }}</h3>
                  <p class="text-gray-500 text-sm">{{ formation.formateur.email }}</p>
                </div>
              </div>
            </div>

            <!-- Prerequis -->
            <div v-if="formation.prerequis" class="bg-white rounded-lg shadow-md p-6">
              <h2 class="text-2xl font-bold mb-4">Prérequis</h2>
              <div class="flex items-start">
                <svg class="w-5 h-5 text-blue-500 mr-2 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"></path>
                </svg>
                <span class="text-gray-700">{{ formation.prerequis }}</span>
              </div>
            </div>
          </div>

          <!-- Sidebar -->
          <div class="space-y-6">
            <!-- Carte principale -->
            <div class="bg-white rounded-lg shadow-md p-6">
              <!-- Infos cles -->
              <div class="space-y-4 text-sm">
                <div class="flex justify-between py-2 border-b">
                  <span class="text-gray-500">Début</span>
                  <span class="font-medium">{{ formatDateFormation(formation.date_heure_debut) }}</span>
                </div>
                <div v-if="formation.date_heure_fin" class="flex justify-between py-2 border-b">
                  <span class="text-gray-500">Fin</span>
                  <span class="font-medium">{{ formatDateFormation(formation.date_heure_fin) }}</span>
                </div>
                <div class="flex justify-between py-2 border-b">
                  <span class="text-gray-500">Langue</span>
                  <span class="font-medium">{{ formation.langue }}</span>
                </div>
                <div class="flex justify-between py-2 border-b">
                  <span class="text-gray-500">Format</span>
                  <span class="font-medium">{{ mapperFormatFrontend(formation.format) }}</span>
                </div>
                <div v-if="formation.pays" class="flex justify-between py-2 border-b">
                  <span class="text-gray-500">Territoire</span>
                  <span class="font-medium">{{ formation.pays }}</span>
                </div>
                <div v-if="formation.ville" class="flex justify-between py-2 border-b">
                  <span class="text-gray-500">Ville</span>
                  <span class="font-medium">{{ formation.ville }}</span>
                </div>
                <div v-if="formation.nombre_places" class="flex justify-between py-2 border-b">
                  <span class="text-gray-500">Places</span>
                  <span class="font-medium">
                    {{ formation.nombre_inscrits }}/{{ formation.nombre_places }}
                  </span>
                </div>
                <div v-if="formation.lien_en_ligne" class="flex justify-between py-2 border-b">
                  <span class="text-gray-500">Accès</span>
                  <a :href="formation.lien_en_ligne" target="_blank"
                     class="text-blue-600 hover:underline font-medium">
                    Lien en ligne
                  </a>
                </div>
              </div>

              <!-- Bouton d'action -->
              <button v-if="!formation.est_inscrit"
                      @click="sInscrire"
                      :disabled="!canInscribe"
                      class="w-full mt-6 py-3 rounded-lg font-medium transition"
                      :class="canInscribe
                        ? 'bg-custom-green text-white hover:bg-green-700'
                        : 'bg-gray-300 text-gray-500 cursor-not-allowed'">
                {{ actionLabel }}
              </button>
              <div v-else class="w-full mt-6 py-3 rounded-lg font-medium text-center bg-blue-100 text-blue-800">
                Vous êtes inscrit(e)
              </div>
            </div>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import {
  useFormations,
  type FormationDetailAPI,
  getTypeLabel,
  getTypeGradient,
  getStatutLabel,
  getStatutBadgeClass,
  getActionLabel,
  peutSInscrire,
  formatDateFormation,
  mapperFormatFrontend,
} from '~/composables/useFormations'

const route = useRoute()
const { chargement, obtenirFormation, inscrireFormation } = useFormations()

const formation = ref<FormationDetailAPI | null>(null)

const breadcrumbs = computed(() => [
  { label: 'Université', to: '/universite' },
  { label: 'INUDA', to: '/universite/inuda' },
  { label: 'Formations', to: '/universite/inuda/formations' },
  { label: formation.value?.titre || 'Formation', to: undefined }
])

const canInscribe = computed(() => {
  if (!formation.value) return false
  return peutSInscrire(formation.value)
})

const actionLabel = computed(() => {
  if (!formation.value) return ''
  return getActionLabel(formation.value)
})

const sInscrire = async () => {
  if (!formation.value) return
  const succes = await inscrireFormation(formation.value.id)
  if (succes) {
    // Recharger pour mettre a jour est_inscrit et le compteur
    const updated = await obtenirFormation(formation.value.id)
    if (updated) formation.value = updated
  } else {
    alert('Erreur lors de l\'inscription. Vérifiez que vous êtes connecté.')
  }
}

onMounted(async () => {
  const id = route.params.id as string
  formation.value = await obtenirFormation(id)

  if (formation.value) {
    useHead({
      title: `${formation.value.titre} - INUDA`
    })
  }
})
</script>
