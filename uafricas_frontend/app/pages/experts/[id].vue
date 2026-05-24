<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Chargement -->
    <div v-if="chargement" class="min-h-screen flex items-center justify-center">
      <div class="text-center">
        <div class="text-5xl text-emerald-500 mb-4 animate-spin inline-block">
          <font-awesome-icon icon="fa-solid fa-spinner" />
        </div>
        <p class="text-gray-500 text-lg">Chargement du profil expert...</p>
      </div>
    </div>

    <!-- Contenu principal -->
    <template v-else-if="expert">
      <!-- Hero Section -->
      <div class="relative h-72 lg:h-96 w-full overflow-hidden">
        <img
          :src="expert.photoURL || '/images/default-avatar.jpg'"
          :alt="`Photo de ${expert.prenom} ${expert.nom}`"
          class="absolute inset-0 w-full h-full object-cover"
        />
        <div class="absolute inset-0 bg-gradient-to-t from-black/80 via-black/40 to-black/10" />

        <!-- Badge statut -->
        <div class="absolute top-28 right-4 flex gap-2">
          <span
            class="px-3 py-1 text-sm font-semibold rounded-full"
            :class="badgeStatutClasses"
          >
            {{ labelStatut }}
          </span>
        </div>

        <!-- Infos sur le hero -->
        <div class="absolute bottom-0 left-0 right-0 px-6 pb-14 pt-6 lg:px-16">
          <h1 class="text-white text-2xl lg:text-4xl font-bold font-display leading-tight drop-shadow-lg">
            {{ expert.prenom }} {{ expert.nom }}
          </h1>
          <p class="text-white/80 text-lg mt-2">{{ expert.expertiseInfo.domaine }}</p>
        </div>
      </div>

      <!-- Carte principale -->
      <div class="mx-4 md:mx-16 lg:mx-40 xl:mx-72 -mt-10 relative z-10 mb-12">
        <!-- Breadcrumb -->
        <div class="bg-white rounded-t-xl px-6 pt-4 pb-2 shadow-xl">
          <CommonBreadcrumbNav :custom-breadcrumbs="breadcrumbs" />
        </div>

        <!-- Contenu carte -->
        <div class="bg-white px-6 lg:px-8 pb-10 rounded-b-xl shadow-xl">
          <!-- Grille infos rapides -->
          <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3 py-5 border-b border-gray-100">
            <!-- Domaine -->
            <div class="flex items-center gap-3 p-3 bg-gray-50 rounded-lg">
              <div class="w-10 h-10 bg-emerald-500/10 rounded-lg flex items-center justify-center shrink-0">
                <font-awesome-icon icon="fa-solid fa-briefcase" class="text-emerald-500" />
              </div>
              <div>
                <div class="text-xs text-gray-400 uppercase font-semibold">Domaine</div>
                <div class="text-sm font-semibold text-gray-800">{{ expert.expertiseInfo.domaine }}</div>
              </div>
            </div>

            <!-- Experience -->
            <div class="flex items-center gap-3 p-3 bg-gray-50 rounded-lg">
              <div class="w-10 h-10 bg-blue-500/10 rounded-lg flex items-center justify-center shrink-0">
                <font-awesome-icon icon="fa-solid fa-award" class="text-blue-500" />
              </div>
              <div>
                <div class="text-xs text-gray-400 uppercase font-semibold">Experience</div>
                <div class="text-sm font-semibold text-gray-800">{{ expert.expertiseInfo.nbAnneesExperience }} ans</div>
              </div>
            </div>

            <!-- Note -->
            <div class="flex items-center gap-3 p-3 bg-gray-50 rounded-lg">
              <div class="w-10 h-10 bg-amber-500/10 rounded-lg flex items-center justify-center shrink-0">
                <font-awesome-icon icon="fa-solid fa-star" class="text-amber-500" />
              </div>
              <div>
                <div class="text-xs text-gray-400 uppercase font-semibold">Note</div>
                <div class="text-sm font-semibold text-gray-800">{{ expert.expertiseInfo.rating.toFixed(1) }} / 5</div>
              </div>
            </div>

            <!-- Localisation -->
            <div class="flex items-center gap-3 p-3 bg-gray-50 rounded-lg">
              <div class="w-10 h-10 bg-custom-chocolat/10 rounded-lg flex items-center justify-center shrink-0">
                <font-awesome-icon icon="fa-solid fa-location-dot" class="text-custom-chocolat" />
              </div>
              <div>
                <div class="text-xs text-gray-400 uppercase font-semibold">Localisation</div>
                <div class="text-sm font-semibold text-gray-800">
                  <span v-if="expert.ville">{{ expert.ville }}, </span>
                  {{ expert.pays }}
                </div>
              </div>
            </div>
          </div>

          <!-- Situations professionnelles -->
          <div v-if="expert.situationProfessionnelle.length > 0" class="mt-6">
            <h2 class="flex items-center gap-2 text-lg font-bold text-gray-800 mb-3">
              <font-awesome-icon icon="fa-solid fa-user-tag" class="text-emerald-500 text-base" />
              Situation professionnelle
            </h2>
            <div class="flex flex-wrap gap-2">
              <span
                v-for="situation in expert.situationProfessionnelle"
                :key="situation"
                class="px-4 py-2 rounded-full text-sm font-medium"
                :class="situationClasses(situation)"
              >
                {{ labelSituation(situation) }}
              </span>
            </div>
          </div>

          <!-- Biographie -->
          <div class="border-t border-gray-100 mt-6 pt-6">
            <h2 class="flex items-center gap-2 text-xl font-bold text-gray-800 mb-3">
              <font-awesome-icon icon="fa-solid fa-align-left" class="text-emerald-500 text-base" />
              Biographie
            </h2>
            <p class="text-gray-600 leading-relaxed whitespace-pre-line">{{ expert.expertiseInfo.biographie }}</p>
          </div>

          <!-- Portfolio -->
          <div v-if="expert.expertiseInfo.portfolio" class="border-t border-gray-100 mt-6 pt-6">
            <h2 class="flex items-center gap-2 text-xl font-bold text-gray-800 mb-3">
              <font-awesome-icon icon="fa-solid fa-link" class="text-blue-500 text-base" />
              Portfolio
            </h2>
            <a
              :href="expert.expertiseInfo.portfolio"
              target="_blank"
              rel="noopener noreferrer"
              class="inline-flex items-center gap-2 text-blue-600 hover:text-blue-800 hover:underline font-medium"
            >
              {{ expert.expertiseInfo.portfolio }}
              <font-awesome-icon icon="fa-solid fa-arrow-up-right-from-square" class="text-xs" />
            </a>
          </div>

          <!-- Contact -->
          <div class="border-t border-gray-100 mt-6 pt-6">
            <h2 class="flex items-center gap-2 text-xl font-bold text-gray-800 mb-4">
              <font-awesome-icon icon="fa-solid fa-envelope" class="text-custom-chocolat text-base" />
              Contact
            </h2>
            <div class="flex items-center gap-4 p-4 bg-gray-50 rounded-xl">
              <!-- Avatar -->
              <div v-if="expert.photoURL" class="shrink-0">
                <img
                  :src="expert.photoURL"
                  :alt="`${expert.prenom} ${expert.nom}`"
                  class="w-14 h-14 rounded-full object-cover ring-2 ring-emerald-500/20"
                />
              </div>
              <div v-else class="w-14 h-14 rounded-full bg-gradient-to-br from-emerald-500 to-teal-500 flex items-center justify-center shrink-0">
                <span class="text-white font-bold text-lg">
                  {{ (expert.prenom?.[0] || '').toUpperCase() }}{{ (expert.nom?.[0] || '').toUpperCase() }}
                </span>
              </div>
              <div class="flex-1">
                <div class="font-semibold text-gray-800 text-lg">
                  {{ expert.prenom }} {{ expert.nom }}
                </div>
                <div class="text-sm text-gray-500">{{ expert.email }}</div>
              </div>
              <a
                :href="`mailto:${expert.email}`"
                class="inline-flex items-center gap-2 px-5 py-2.5 bg-gradient-to-r from-emerald-500 to-teal-500 text-white rounded-lg font-semibold hover:shadow-lg hover:scale-[1.02] transition-all"
              >
                <font-awesome-icon icon="fa-solid fa-paper-plane" />
                Contacter
              </a>
            </div>
          </div>

          <!-- Dates -->
          <div class="border-t border-gray-100 mt-6 pt-6">
            <div class="flex flex-wrap gap-6 text-sm text-gray-400">
              <span>
                <font-awesome-icon icon="fa-regular fa-calendar" class="mr-1" />
                Membre depuis {{ formatDate(expert.dateInscription) }}
              </span>
              <span>
                <font-awesome-icon icon="fa-solid fa-clock-rotate-left" class="mr-1" />
                Mis a jour le {{ formatDate(expert.dateDerniereMiseAJour) }}
              </span>
            </div>
          </div>

          <!-- Retour -->
          <div class="mt-8 pt-6 border-t border-gray-100">
            <NuxtLink to="/experts" class="inline-flex items-center gap-2 text-emerald-600 hover:underline font-medium">
              <font-awesome-icon icon="fa-solid fa-arrow-left" />
              Retour a la liste des experts
            </NuxtLink>
          </div>
        </div>
      </div>
    </template>

    <!-- Etat non trouve -->
    <div v-else class="min-h-screen flex items-center justify-center">
      <div class="text-center">
        <div class="text-6xl text-gray-200 mb-6">
          <font-awesome-icon icon="fa-solid fa-user-slash" />
        </div>
        <h3 class="text-2xl font-semibold text-gray-500 mb-2">Expert non trouve</h3>
        <p class="text-gray-400 mb-6">Ce profil expert n'existe pas ou a ete supprime.</p>
        <NuxtLink
          to="/experts"
          class="inline-flex items-center gap-2 text-white bg-emerald-500 px-5 py-2.5 rounded-lg font-semibold hover:brightness-110 transition-all"
        >
          <font-awesome-icon icon="fa-solid fa-arrow-left" />
          Voir tous les experts
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useExperts, PROFILS_PROFESSIONNELS, type ExpertAPI } from '~/composables/useExperts'

const route = useRoute()
const expertId = route.params.id as string

const { obtenirExpert, chargement } = useExperts()

const expert = ref<ExpertAPI | null>(null)

onMounted(async () => {
  const data = await obtenirExpert(expertId)
  if (data) {
    expert.value = data
  }
})

const breadcrumbs = computed(() => [
  { label: 'Experts', to: '/experts' },
  { label: expert.value ? `${expert.value.prenom} ${expert.value.nom}` : 'Détail', to: undefined },
])

useHead({
  title: computed(() =>
    expert.value
      ? `${expert.value.prenom} ${expert.value.nom} — Expert ${expert.value.expertiseInfo.domaine} | UAfricas`
      : 'Expert | UAfricas',
  ),
})

// Badge statut expertise
const badgeStatutClasses = computed(() => {
  switch (expert.value?.expertiseInfo.statut) {
    case 'valide': return 'bg-green-100 text-green-700'
    case 'en_attente': return 'bg-amber-100 text-amber-700'
    case 'refuse': return 'bg-red-100 text-red-700'
    default: return 'bg-gray-200 text-gray-600'
  }
})

const labelStatut = computed(() => {
  switch (expert.value?.expertiseInfo.statut) {
    case 'valide': return 'Expert valide'
    case 'en_attente': return 'En attente de validation'
    case 'refuse': return 'Candidature refusee'
    default: return ''
  }
})

// Labels et styles pour les situations professionnelles
const labelSituation = (situation: string): string => {
  const profil = PROFILS_PROFESSIONNELS.find(p => p.id === situation)
  return profil?.label || situation
}

const situationClasses = (situation: string): string => {
  switch (situation) {
    case 'recherche_emploi': return 'bg-red-100 text-red-700'
    case 'en_emploi': return 'bg-green-100 text-green-700'
    case 'consultance': return 'bg-blue-100 text-blue-700'
    case 'volontariat_expertise': return 'bg-purple-100 text-purple-700'
    case 'recherche_nouvelles_opportunites': return 'bg-orange-100 text-orange-700'
    default: return 'bg-gray-100 text-gray-700'
  }
}

// Formatage date
const formatDate = (dateStr: string): string => {
  try {
    const date = new Date(dateStr)
    return date.toLocaleDateString('fr-FR', {
      day: 'numeric',
      month: 'long',
      year: 'numeric',
    })
  }
  catch {
    return dateStr
  }
}
</script>
