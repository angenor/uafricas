<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Loading state -->
    <div v-if="chargement" class="flex items-center justify-center min-h-screen">
      <font-awesome-icon
        :icon="['fas', 'spinner']"
        class="h-12 text-custom-green animate-spin"
      />
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
          :src="programme.couverture_url || '/images/carte-afrique.jpg'"
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
        <CommonBreadcrumbNav
          class="px-7 pt-6"
          :custom-breadcrumbs="[
            { label: 'Échanges Sabbatiques', to: '/echanges-sabbatiques' },
            { label: programme.titre },
          ]"
        />

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
            {{ formatDateSabbatique(programme.date_debut) }}
          </span>
          <span class="border-l border-gray-300 pl-4 flex items-center gap-1">
            <font-awesome-icon :icon="['fas', 'clock']" class="text-custom-green" />
            {{ programme.duree_label }}
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
            :src="programme.couverture_url || '/images/carte-afrique.jpg'"
            :alt="programme.titre"
            class="w-full h-64 lg:h-100 object-cover rounded-lg shadow-md"
          />
        </div>

        <!-- Candidat retenu (affichage public, transparence) -->
        <div v-if="programme.candidat_retenu" class="px-7 pt-6">
          <div class="bg-custom-green/10 border border-custom-green rounded-lg p-4 flex items-center gap-3">
            <font-awesome-icon :icon="['fas', 'award']" class="text-2xl text-custom-green" />
            <div>
              <p class="text-sm text-gray-500">Candidat retenu pour ce programme</p>
              <p class="font-semibold text-custom-chocolat">
                {{ programme.candidat_retenu.prenom ? `${programme.candidat_retenu.prenom} ` : '' }}{{ programme.candidat_retenu.nom }}
              </p>
            </div>
          </div>
        </div>

        <!-- Action candidature -->
        <div class="px-7 pt-6">
          <!-- Organisateur : gestion des candidatures -->
          <div v-if="programme.est_organisateur" class="bg-gray-50 p-4 rounded-lg">
            <p class="text-gray-700 mb-3 flex items-center gap-2">
              <font-awesome-icon :icon="['fas', 'user-tie']" class="text-custom-green" />
              Vous êtes l'organisateur de ce programme.
            </p>
            <button
              class="bg-custom-chocolat text-white px-5 py-2.5 rounded-lg font-medium hover:bg-custom-chocolat/90 transition-all flex items-center gap-2"
              @click="ouvrirCandidatures"
            >
              <font-awesome-icon :icon="['fas', 'users']" />
              Gérer les candidatures ({{ programme.nombre_candidatures }})
            </button>
          </div>

          <!-- Candidat déjà inscrit -->
          <div v-else-if="programme.a_deja_candidate" class="bg-green-50 border border-green-200 p-4 rounded-lg">
            <p class="text-green-700 flex items-center gap-2">
              <font-awesome-icon :icon="['fas', 'circle-check']" />
              Vous avez déjà candidaté à ce programme.
            </p>
            <button
              class="mt-3 text-sm text-custom-green hover:underline"
              @click="modaleCandidature = true"
            >
              Modifier ma candidature
            </button>
          </div>

          <!-- Bouton candidater -->
          <div v-else-if="isAuthenticated" class="bg-gray-50 p-4 rounded-lg">
            <button
              class="w-full lg:w-auto bg-custom-green text-white px-6 py-3 rounded-lg font-medium hover:bg-custom-green/90 transition-all flex items-center justify-center gap-2"
              @click="modaleCandidature = true"
            >
              <font-awesome-icon :icon="['fas', 'paper-plane']" />
              Je candidate
            </button>
            <p class="text-sm text-gray-500 mt-2">
              Réservé aux personnes en emploi ou retraitées.
            </p>
          </div>

          <!-- Non connecté -->
          <div v-else class="bg-yellow-50 border border-yellow-200 p-4 rounded-lg">
            <p class="text-gray-700 mb-3">
              Connectez-vous pour candidater à ce programme.
            </p>
            <NuxtLink
              to="/login"
              class="inline-flex items-center gap-2 bg-custom-chocolat text-white px-4 py-2 rounded-lg hover:bg-custom-chocolat/90 transition-all"
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
          <!-- eslint-disable-next-line vue/no-v-html -->
          <div
            class="text-gray-700 leading-relaxed prose prose-sm max-w-none"
            v-html="programme.description"
          />
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
              <p class="font-medium text-gray-900">{{ programme.domaine || 'Non précisé' }}</p>
            </div>

            <!-- Durée -->
            <div class="bg-gray-50 p-4 rounded-lg">
              <p class="text-sm text-gray-500 mb-1">Durée du programme</p>
              <p class="font-medium text-gray-900">{{ programme.duree_label }}</p>
            </div>

            <!-- Dates -->
            <div class="bg-gray-50 p-4 rounded-lg">
              <p class="text-sm text-gray-500 mb-1">Période</p>
              <p class="font-medium text-gray-900">
                Du {{ formatDateCourte(programme.date_debut) }}
                <template v-if="programme.date_fin">
                  au {{ formatDateCourte(programme.date_fin) }}
                </template>
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
            <div v-if="programme.prise_en_charge.length > 0" class="bg-gray-50 p-4 rounded-lg md:col-span-2">
              <p class="text-sm text-gray-500 mb-2">Prise en charge par l'organisation</p>
              <div class="flex flex-wrap gap-2">
                <span
                  v-for="prise in programme.prise_en_charge"
                  :key="prise"
                  class="px-3 py-1 bg-custom-green/10 text-custom-green rounded-full text-sm font-medium"
                >
                  <font-awesome-icon :icon="['fas', 'check']" class="mr-1" />
                  {{ getPriseLabel(prise) }}
                </span>
              </div>
            </div>

            <!-- Prérequis -->
            <div v-if="programme.prerequis" class="bg-gray-50 p-4 rounded-lg md:col-span-2">
              <p class="text-sm text-gray-500 mb-1">Prérequis</p>
              <p class="font-medium text-gray-900 whitespace-pre-line">{{ programme.prerequis }}</p>
            </div>

            <!-- Langues requises -->
            <div v-if="programme.langues_requises" class="bg-gray-50 p-4 rounded-lg md:col-span-2">
              <p class="text-sm text-gray-500 mb-1">Langues requises</p>
              <p class="font-medium text-gray-900">{{ programme.langues_requises }}</p>
            </div>

            <!-- Type d'organisation -->
            <div v-if="programme.type_organisation_label" class="bg-gray-50 p-4 rounded-lg">
              <p class="text-sm text-gray-500 mb-1">Type d'organisation</p>
              <p class="font-medium text-gray-900">{{ programme.type_organisation_label }}</p>
            </div>

            <!-- Statut légal -->
            <div v-if="programme.statut_legal" class="bg-gray-50 p-4 rounded-lg">
              <p class="text-sm text-gray-500 mb-1">Statut légal de l'organisation</p>
              <p class="font-medium text-gray-900">{{ programme.statut_legal }}</p>
            </div>

            <!-- Organisateur -->
            <div v-if="programme.user" class="bg-gray-50 p-4 rounded-lg md:col-span-2">
              <p class="text-sm text-gray-500 mb-1">Organisateur</p>
              <p class="font-medium text-gray-900">
                {{ programme.user.prenom ? `${programme.user.prenom} ` : '' }}{{ programme.user.nom }}
              </p>
              <p class="text-sm text-gray-600">{{ programme.user.email }}</p>
            </div>

            <!-- Document -->
            <div v-if="programme.document_url" class="bg-gray-50 p-4 rounded-lg md:col-span-2">
              <p class="text-sm text-gray-500 mb-1">Document associé</p>
              <a
                :href="programme.document_url"
                target="_blank"
                class="inline-flex items-center gap-2 text-custom-green hover:underline font-medium"
              >
                <font-awesome-icon :icon="['fas', 'file-pdf']" />
                Télécharger le document
              </a>
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

      <!-- Modale de candidature -->
      <SabbatiqueCandidatureModal
        :programme-id="programme.id"
        :open="modaleCandidature"
        @close="modaleCandidature = false"
        @success="onCandidatureSuccess"
      />

      <!-- Modale gestion des candidatures (organisateur) -->
      <Teleport to="body">
        <div
          v-if="modaleGestion"
          class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 px-4 py-8 overflow-y-auto"
          @click.self="modaleGestion = false"
        >
          <div class="bg-white rounded-lg shadow-2xl max-w-2xl w-full my-auto">
            <div class="flex items-center justify-between px-6 py-4 border-b">
              <h2 class="text-lg font-bold text-custom-chocolat">
                Candidatures reçues
              </h2>
              <button type="button" class="text-gray-400 hover:text-gray-600" @click="modaleGestion = false">
                <font-awesome-icon :icon="['fas', 'xmark']" class="text-xl" />
              </button>
            </div>

            <div class="px-6 py-5 max-h-[70vh] overflow-y-auto">
              <p v-if="!candidatures.length" class="text-gray-500 text-sm text-center py-8">
                Aucune candidature pour le moment.
              </p>

              <div
                v-for="c in candidatures"
                :key="c.id"
                class="border rounded-lg p-4 mb-3"
                :class="c.est_retenu ? 'border-custom-green bg-custom-green/5' : 'border-gray-200'"
              >
                <div class="flex items-start justify-between gap-3">
                  <div class="min-w-0">
                    <p class="font-semibold text-gray-900">
                      {{ c.nom_etat_civil || `${c.candidat.prenom || ''} ${c.candidat.nom}`.trim() }}
                    </p>
                    <p class="text-sm text-gray-600">{{ c.fonction_actuelle }}</p>
                    <p class="text-xs text-gray-400">{{ c.lieu_residence }}</p>
                    <div class="flex flex-wrap gap-2 mt-2 text-xs">
                      <span v-if="c.statut_emploi_label" class="px-2 py-0.5 bg-gray-100 rounded-full text-gray-600">
                        {{ c.statut_emploi_label }}
                      </span>
                      <span
                        class="px-2 py-0.5 rounded-full"
                        :class="c.repond_profil ? 'bg-green-100 text-green-700' : 'bg-amber-100 text-amber-700'"
                      >
                        {{ c.repond_profil ? 'Répond au profil' : 'Profil partiel' }}
                      </span>
                    </div>
                    <div class="flex flex-wrap gap-3 mt-2 text-sm">
                      <a v-if="c.cv_url" :href="c.cv_url" target="_blank" class="text-custom-green hover:underline">
                        <font-awesome-icon :icon="['fas', 'file-pdf']" class="mr-1" />CV
                      </a>
                      <a v-if="c.lien_expertise" :href="c.lien_expertise" target="_blank" class="text-custom-green hover:underline">
                        <font-awesome-icon :icon="['fas', 'link']" class="mr-1" />Compte expertise
                      </a>
                    </div>
                    <p v-if="c.lettre_motivation" class="text-sm text-gray-600 mt-2 italic">
                      « {{ c.lettre_motivation }} »
                    </p>
                  </div>
                  <div class="shrink-0">
                    <span v-if="c.est_retenu" class="inline-flex items-center gap-1 text-custom-green font-medium text-sm">
                      <font-awesome-icon :icon="['fas', 'award']" /> Retenu
                    </span>
                    <button
                      v-else
                      class="bg-custom-green text-white px-3 py-1.5 rounded-md text-sm hover:bg-custom-green/90 transition-colors disabled:opacity-50"
                      :disabled="selectionEnCours"
                      @click="retenir(c.id)"
                    >
                      Retenir
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </Teleport>

      <!-- Toast succès candidature -->
      <Teleport to="body">
        <div
          v-if="toastSucces"
          class="fixed bottom-6 left-1/2 -translate-x-1/2 z-50 bg-custom-green text-white px-5 py-3 rounded-lg shadow-lg flex items-center gap-2"
        >
          <font-awesome-icon :icon="['fas', 'circle-check']" />
          Votre candidature a bien été envoyée !
        </div>
      </Teleport>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import {
  useSabbatiques,
  formatDateSabbatique,
  formatDateCourte,
  PRISES_EN_CHARGE,
  type SabbatiqueDetailAPI,
  type CandidatureAPI,
} from '~/composables/useSabbatiques'
import { useUserStore } from '~/stores/user'

const route = useRoute()
const userStore = useUserStore()
const {
  obtenirProgramme,
  chargement,
  listerCandidatures,
  selectionnerCandidat,
} = useSabbatiques()

const programme = ref<SabbatiqueDetailAPI | null>(null)

const isAuthenticated = computed(() => userStore.isAuthenticated)

// État des modales et candidatures (organisateur)
const modaleCandidature = ref(false)
const modaleGestion = ref(false)
const toastSucces = ref(false)
const candidatures = ref<CandidatureAPI[]>([])
const selectionEnCours = ref(false)

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

const getPriseLabel = (value: string) => {
  const found = PRISES_EN_CHARGE.find(p => p.value === value)
  return found ? found.label : value
}

const rechargerProgramme = async () => {
  const id = route.params.id as string
  const result = await obtenirProgramme(id)
  if (result) programme.value = result
}

const onCandidatureSuccess = async () => {
  modaleCandidature.value = false
  toastSucces.value = true
  setTimeout(() => { toastSucces.value = false }, 4000)
  await rechargerProgramme()
}

const ouvrirCandidatures = async () => {
  if (!programme.value) return
  const liste = await listerCandidatures(programme.value.id)
  candidatures.value = liste || []
  modaleGestion.value = true
}

const retenir = async (candidatureId: string) => {
  if (!programme.value) return
  selectionEnCours.value = true
  const ok = await selectionnerCandidat(programme.value.id, candidatureId)
  selectionEnCours.value = false
  if (ok) {
    await Promise.all([rechargerProgramme(), ouvrirCandidatures()])
  }
}

onMounted(async () => {
  const id = route.params.id as string
  const result = await obtenirProgramme(id)
  programme.value = result

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
})
</script>
