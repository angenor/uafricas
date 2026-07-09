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
      <NuxtLink to="/universite/formations" class="text-blue-600 hover:underline">
        ← Retour à la liste des formations
      </NuxtLink>
    </div>

    <!-- Contenu -->
    <template v-else>
      <!-- Bannière (image de couverture) -->
      <div class="relative h-64 md:h-80 lg:h-96 w-full overflow-hidden"
           :class="!formation.couverture_url ? getTypeGradient(formation.type) : 'bg-gray-800'">
        <img v-if="formation.couverture_url"
             :src="formation.couverture_url"
             :alt="formation.titre"
             class="absolute inset-0 w-full h-full object-cover">
        <!-- Voile léger, concentré sur le bas pour la lisibilité du titre -->
        <div class="absolute inset-0 bg-gradient-to-t from-black/75 via-black/15 to-transparent"></div>

        <div class="relative h-full flex flex-col justify-end px-4 md:px-8 pb-6">
          <div class="max-w-6xl mx-auto w-full">
            <CommonBreadcrumbNav :custom-breadcrumbs="breadcrumbs" class="mb-3 text-white" />
            <div class="flex flex-wrap items-center gap-2 mb-3">
              <span class="px-3 py-1 bg-white/20 backdrop-blur-xs text-white rounded-full text-sm font-medium">
                {{ getTypeLabel(formation.type) }}
              </span>
              <span :class="getStatutBadgeClass(formation.statut)">
                {{ getStatutLabel(formation.statut) }}
              </span>
              <span v-if="formation.est_certifiante"
                    class="inline-flex items-center gap-1 px-3 py-1 bg-amber-500/90 text-white rounded-full text-sm font-medium">
                <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                  <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.286 3.955a1 1 0 00.95.69h4.162c.969 0 1.371 1.24.588 1.81l-3.367 2.446a1 1 0 00-.364 1.118l1.287 3.955c.3.922-.755 1.688-1.54 1.118l-3.366-2.446a1 1 0 00-1.175 0l-3.367 2.446c-.783.57-1.838-.196-1.539-1.118l1.286-3.955a1 1 0 00-.363-1.118L2.98 9.382c-.784-.57-.38-1.81.588-1.81h4.16a1 1 0 00.951-.69l1.286-3.955z"/>
                </svg>
                Certifiante
              </span>
              <span v-if="formation.a_evaluation"
                    class="inline-flex items-center gap-1 px-3 py-1 bg-blue-600/90 text-white rounded-full text-sm font-medium">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01"/>
                </svg>
                Évaluation finale
              </span>
            </div>
            <h1 class="text-2xl md:text-3xl lg:text-4xl font-bold text-white drop-shadow-lg">{{ formation.titre }}</h1>
          </div>
        </div>
      </div>

      <!-- Image de couverture (rappel après le hero) -->
      <div v-if="formation.couverture_url" class="max-w-6xl mx-auto px-4 pt-8">
        <img :src="formation.couverture_url"
             :alt="formation.titre"
             class="w-full h-56 md:h-72 object-cover rounded-lg shadow-md">
      </div>

      <!-- Contenu principal -->
      <div class="max-w-6xl mx-auto px-4 py-8">
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
          <!-- Colonne principale : panneau à onglets fusionnés + programme permanent -->
          <div class="lg:col-span-2 space-y-8">
            <!-- Panneau à onglets verticaux : menu fusionné avec le contenu -->
            <div class="bg-white rounded-lg shadow-md overflow-hidden flex flex-col md:flex-row">
              <!-- Onglets (fusionnés à gauche du contenu) -->
              <nav class="md:w-52 shrink-0 border-b md:border-b-0 md:border-r border-gray-100 bg-gray-50/70 p-3">
                <ul class="flex md:flex-col gap-1">
                  <li v-for="section in sections" :key="section.id" class="flex-1 md:flex-none">
                    <button type="button"
                            @click="sectionActive = section.id"
                            class="w-full flex items-center justify-center md:justify-start gap-2 px-3 py-2.5 rounded-md text-sm font-medium transition"
                            :class="sectionActive === section.id
                              ? 'bg-custom-green text-white shadow-sm'
                              : 'text-gray-600 hover:bg-gray-200/60'">
                      <span class="hidden md:inline w-1.5 h-1.5 rounded-full"
                            :class="sectionActive === section.id ? 'bg-white' : 'bg-gray-300'"></span>
                      {{ section.label }}
                    </button>
                  </li>
                </ul>
              </nav>

              <!-- Contenu de l'onglet actif -->
              <div class="flex-1 min-w-0 p-6">
                <!-- Objectif -->
                <div v-show="sectionActive === 'objectif'">
                  <h2 class="text-2xl font-bold mb-4">Objectif</h2>
                  <p v-if="formation.objectif" class="text-gray-700 leading-relaxed whitespace-pre-line">{{ formation.objectif }}</p>
                  <p v-else class="text-gray-400 italic">Les objectifs de cette formation seront précisés prochainement.</p>
                </div>

                <!-- Présentation -->
                <div v-show="sectionActive === 'presentation'">
                  <h2 class="text-2xl font-bold mb-4">Présentation</h2>
                  <p class="text-gray-700 leading-relaxed whitespace-pre-line">
                    {{ formation.presentation || formation.description }}
                  </p>
                  <div v-if="formation.prerequis" class="mt-6 flex items-start gap-2 rounded-md bg-blue-50 p-4">
                    <svg class="w-5 h-5 text-blue-500 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"></path>
                    </svg>
                    <div>
                      <p class="font-semibold text-gray-800">Prérequis</p>
                      <p class="text-gray-700">{{ formation.prerequis }}</p>
                    </div>
                  </div>
                </div>

                <!-- Intervenants -->
                <div v-show="sectionActive === 'intervenants'">
                  <h2 class="text-2xl font-bold mb-6">Intervenants</h2>
                  <div class="grid grid-cols-1 sm:grid-cols-2 gap-6">
                    <!-- Formateur principal (responsable) -->
                    <div class="flex items-start gap-4">
                      <img v-if="formation.formateur.photo_url"
                           :src="formation.formateur.photo_url"
                           :alt="`${formation.formateur.prenom} ${formation.formateur.nom}`"
                           class="w-16 h-16 rounded-full object-cover flex-shrink-0">
                      <div v-else class="w-16 h-16 rounded-full bg-gray-200 flex items-center justify-center flex-shrink-0">
                        <svg class="w-9 h-9 text-gray-400" fill="currentColor" viewBox="0 0 20 20">
                          <path fill-rule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" clip-rule="evenodd"></path>
                        </svg>
                      </div>
                      <div>
                        <h3 class="font-semibold text-gray-900">{{ formation.formateur.prenom }} {{ formation.formateur.nom }}</h3>
                        <p class="text-sm text-custom-green">Responsable de la formation</p>
                        <p class="text-sm text-gray-500">{{ formation.formateur.email }}</p>
                      </div>
                    </div>

                    <!-- Placeholders intervenants (à venir) -->
                    <div v-for="n in 2" :key="`interv-${n}`" class="flex items-start gap-4">
                      <div class="w-16 h-16 rounded-full bg-gray-100 flex items-center justify-center flex-shrink-0 ring-2 ring-dashed ring-gray-200">
                        <svg class="w-9 h-9 text-gray-300" fill="currentColor" viewBox="0 0 20 20">
                          <path fill-rule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" clip-rule="evenodd"></path>
                        </svg>
                      </div>
                      <div>
                        <h3 class="font-semibold text-gray-400">Intervenant à confirmer</h3>
                        <p class="text-sm text-gray-400">Profil bientôt disponible</p>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- Programme : affiché en permanence -->
            <UniversiteInudaFormationCurriculum
              :formation-id="formation.id"
              :refresh-token="refreshContenu"
              @require-inscription="surRequireInscription" />
          </div>

          <!-- Colonne inscription (sticky) -->
          <aside class="lg:col-span-1">
            <div ref="inscriptionCard" class="lg:sticky lg:top-24 bg-white rounded-lg shadow-md p-6 scroll-mt-24">
              <div class="space-y-3 text-sm">
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
                  <span class="font-medium">{{ formation.nombre_inscrits }}/{{ formation.nombre_places }}</span>
                </div>
                <div class="flex justify-between py-2 border-b">
                  <span class="text-gray-500">Certifiante</span>
                  <span class="font-medium" :class="formation.est_certifiante ? 'text-custom-green' : 'text-gray-400'">
                    {{ formation.est_certifiante ? 'Oui' : 'Non' }}
                  </span>
                </div>
                <div class="flex justify-between py-2 border-b">
                  <span class="text-gray-500">Évaluation finale</span>
                  <span class="font-medium" :class="formation.a_evaluation ? 'text-custom-green' : 'text-gray-400'">
                    {{ formation.a_evaluation ? 'Oui' : 'Non' }}
                  </span>
                </div>
                <div v-if="formation.lien_en_ligne" class="flex justify-between py-2 border-b">
                  <span class="text-gray-500">Accès</span>
                  <a :href="formation.lien_en_ligne" target="_blank"
                     class="text-blue-600 hover:underline font-medium">Lien en ligne</a>
                </div>
              </div>

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
          </aside>
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
const refreshContenu = ref(0)
const inscriptionCard = ref<HTMLElement | null>(null)

// Onglets éditoriaux (le Programme est affiché en permanence, hors onglets)
const sections = [
  { id: 'objectif', label: 'Objectif' },
  { id: 'presentation', label: 'Présentation' },
  { id: 'intervenants', label: 'Intervenants' },
]
const sectionActive = ref('objectif')

const surRequireInscription = () => {
  inscriptionCard.value?.scrollIntoView({ behavior: 'smooth', block: 'center' })
}

const breadcrumbs = computed(() => [
  { label: 'Université', to: '/universite' },
  { label: 'INUDA', to: '/universite' },
  { label: 'Formations', to: '/universite/formations' },
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
    const updated = await obtenirFormation(formation.value.id)
    if (updated) formation.value = updated
    refreshContenu.value++
  } else {
    alert('Erreur lors de l\'inscription. Vérifiez que vous êtes connecté.')
  }
}

onMounted(async () => {
  const id = route.params.id as string
  formation.value = await obtenirFormation(id)
  if (formation.value) {
    useHead({ title: `${formation.value.titre} - INUDA` })
  }
})
</script>
