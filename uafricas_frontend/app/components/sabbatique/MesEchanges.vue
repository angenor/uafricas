<template>
  <div class="space-y-4">
    <!-- Chargement -->
    <div v-if="chargement" class="flex justify-center py-8">
      <font-awesome-icon icon="fa-solid fa-spinner" class="text-2xl text-custom-chocolat animate-spin" />
    </div>

    <!-- Aucun programme soumis -->
    <div v-else-if="!mesProgrammes.length" class="text-center py-10">
      <font-awesome-icon icon="fa-solid fa-paper-plane" class="text-4xl text-gray-300 mb-3" />
      <p class="text-gray-500 text-sm mb-4">
        Vous n'avez pas encore proposé de projet d'échange sabbatique.
      </p>
      <NuxtLink
        to="/echanges-sabbatiques/proposer"
        class="inline-flex items-center gap-2 px-5 py-2.5 text-sm bg-custom-green text-white font-medium rounded-xl hover:bg-custom-green/90 transition-all"
      >
        <font-awesome-icon icon="fa-solid fa-plus" />
        Proposer un projet
      </NuxtLink>
    </div>

    <!-- Liste des programmes soumis -->
    <div
      v-for="prog in mesProgrammes"
      :key="prog.id"
      class="border border-gray-200 rounded-xl overflow-hidden"
    >
      <!-- En-tête programme -->
      <div class="p-4 flex flex-wrap items-start justify-between gap-3">
        <div class="min-w-0">
          <NuxtLink
            :to="`/echanges-sabbatiques/${prog.id}`"
            class="font-semibold text-gray-900 hover:text-custom-green transition-colors"
          >
            {{ prog.titre }}
          </NuxtLink>
          <div class="flex flex-wrap items-center gap-2 mt-1 text-xs">
            <span class="px-2 py-0.5 rounded-full" :class="statutClasses(prog.statut)">
              {{ statutLabel(prog.statut) }}
            </span>
            <span v-if="prog.type_organisation_label" class="text-gray-500">
              {{ prog.type_organisation_label }}
            </span>
            <span class="text-gray-400">·</span>
            <span class="text-gray-600">
              {{ prog.nombre_candidatures }} candidature{{ prog.nombre_candidatures > 1 ? 's' : '' }}
            </span>
          </div>
          <p v-if="prog.candidat_retenu" class="text-sm text-custom-green mt-1 flex items-center gap-1">
            <font-awesome-icon icon="fa-solid fa-award" />
            Candidat retenu :
            {{ prog.candidat_retenu.prenom ? `${prog.candidat_retenu.prenom} ` : '' }}{{ prog.candidat_retenu.nom }}
          </p>
        </div>
        <button
          type="button"
          class="shrink-0 px-3 py-1.5 rounded-md text-sm font-medium transition-colors"
          :class="programmeOuvert === prog.id
            ? 'bg-gray-200 text-gray-700 hover:bg-gray-300'
            : 'bg-custom-chocolat text-white hover:bg-custom-chocolat/90'"
          @click="basculerCandidatures(prog.id)"
        >
          <font-awesome-icon :icon="programmeOuvert === prog.id ? 'fa-solid fa-chevron-up' : 'fa-solid fa-users'" class="mr-1" />
          {{ programmeOuvert === prog.id ? 'Masquer' : 'Gérer les candidatures' }}
        </button>
      </div>

      <!-- Candidatures (déplié) -->
      <div v-if="programmeOuvert === prog.id" class="border-t border-gray-100 bg-gray-50 p-4">
        <div v-if="chargementCandidatures" class="flex justify-center py-6">
          <font-awesome-icon icon="fa-solid fa-spinner" class="text-xl text-custom-chocolat animate-spin" />
        </div>

        <p v-else-if="!candidatures.length" class="text-gray-500 text-sm text-center py-6">
          Aucune candidature pour ce programme.
        </p>

        <div
          v-for="c in candidatures"
          v-else
          :key="c.id"
          class="bg-white border rounded-lg p-3 mb-2"
          :class="c.est_retenu ? 'border-custom-green' : 'border-gray-200'"
        >
          <div class="flex items-start justify-between gap-3">
            <div class="min-w-0">
              <p class="font-semibold text-gray-900 text-sm">
                {{ c.nom_etat_civil || `${c.candidat.prenom || ''} ${c.candidat.nom}`.trim() }}
              </p>
              <p class="text-xs text-gray-600">{{ c.fonction_actuelle }}</p>
              <p class="text-xs text-gray-400">{{ c.lieu_residence }}</p>
              <a
                v-if="c.candidat.email"
                :href="`mailto:${c.candidat.email}`"
                class="inline-flex items-center gap-1 text-xs text-custom-chocolat hover:underline mt-0.5"
              >
                <font-awesome-icon icon="fa-solid fa-envelope" />
                {{ c.candidat.email }}
              </a>
              <div class="flex flex-wrap gap-2 mt-1.5 text-xs">
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
              <div class="flex flex-wrap gap-3 mt-1.5 text-sm">
                <a v-if="c.cv_url" :href="c.cv_url" target="_blank" class="text-custom-green hover:underline">
                  <font-awesome-icon icon="fa-solid fa-file-pdf" class="mr-1" />CV
                </a>
                <a v-if="c.lien_expertise" :href="c.lien_expertise" target="_blank" class="text-custom-green hover:underline">
                  <font-awesome-icon icon="fa-solid fa-link" class="mr-1" />Compte expertise
                </a>
              </div>
              <p v-if="c.lettre_motivation" class="text-xs text-gray-600 mt-1.5 italic">
                « {{ c.lettre_motivation }} »
              </p>
            </div>
            <div class="shrink-0">
              <span v-if="c.est_retenu" class="inline-flex items-center gap-1 text-custom-green font-medium text-sm">
                <font-awesome-icon icon="fa-solid fa-award" /> Retenu
              </span>
              <button
                v-else
                class="bg-custom-green text-white px-3 py-1.5 rounded-md text-sm hover:bg-custom-green/90 transition-colors disabled:opacity-50"
                :disabled="selectionEnCours"
                @click="retenir(prog.id, c.id)"
              >
                Retenir
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import {
  useSabbatiques,
  type SabbatiqueAPI,
  type CandidatureAPI,
} from '~/composables/useSabbatiques'

const { listerMesProgrammes, listerCandidatures, selectionnerCandidat } = useSabbatiques()

const mesProgrammes = ref<SabbatiqueAPI[]>([])
const chargement = ref(true)
const programmeOuvert = ref<string | null>(null)
const candidatures = ref<CandidatureAPI[]>([])
const chargementCandidatures = ref(false)
const selectionEnCours = ref(false)

const statutLabel = (statut: string): string => {
  const labels: Record<string, string> = {
    ouvert: 'Ouvert',
    en_cours: 'En cours',
    termine: 'Terminé',
    en_attente: 'En attente',
    annule: 'Annulé',
    suspendu: 'Suspendu',
  }
  return labels[statut] || statut
}

const statutClasses = (statut: string): string => {
  const classes: Record<string, string> = {
    ouvert: 'bg-green-100 text-green-700',
    en_cours: 'bg-blue-100 text-blue-700',
    termine: 'bg-gray-100 text-gray-700',
    en_attente: 'bg-amber-100 text-amber-700',
    annule: 'bg-red-100 text-red-700',
    suspendu: 'bg-orange-100 text-orange-700',
  }
  return classes[statut] || 'bg-gray-100 text-gray-700'
}

const chargerMesProgrammes = async () => {
  chargement.value = true
  const liste = await listerMesProgrammes()
  mesProgrammes.value = liste || []
  chargement.value = false
}

const basculerCandidatures = async (programmeId: string) => {
  if (programmeOuvert.value === programmeId) {
    programmeOuvert.value = null
    return
  }
  programmeOuvert.value = programmeId
  chargementCandidatures.value = true
  candidatures.value = (await listerCandidatures(programmeId)) || []
  chargementCandidatures.value = false
}

const retenir = async (programmeId: string, candidatureId: string) => {
  selectionEnCours.value = true
  const ok = await selectionnerCandidat(programmeId, candidatureId)
  selectionEnCours.value = false
  if (ok) {
    candidatures.value = (await listerCandidatures(programmeId)) || []
    await chargerMesProgrammes()
    programmeOuvert.value = programmeId
  }
}

onMounted(chargerMesProgrammes)
</script>
