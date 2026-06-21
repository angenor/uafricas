<template>
  <Teleport to="body">
    <div
      v-if="open"
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 px-4 py-8 overflow-y-auto"
      @click.self="emit('close')"
    >
      <div class="bg-white rounded-lg shadow-2xl max-w-2xl w-full my-auto">
        <!-- En-tête -->
        <div class="flex items-center justify-between px-6 py-4 border-b">
          <h2 class="text-lg font-bold text-custom-chocolat">
            Je candidate à ce programme
          </h2>
          <button
            type="button"
            class="text-gray-400 hover:text-gray-600"
            @click="emit('close')"
          >
            <font-awesome-icon :icon="['fas', 'xmark']" class="text-xl" />
          </button>
        </div>

        <!-- Corps -->
        <form @submit.prevent="soumettre" class="px-6 py-5 space-y-5">
          <!-- Condition d'éligibilité -->
          <div class="bg-amber-50 border border-amber-200 rounded-md p-3 text-sm text-amber-800">
            <font-awesome-icon :icon="['fas', 'circle-info']" class="mr-1" />
            Condition : vous devez être <strong>en emploi</strong> ou <strong>retraité(e)</strong>.
            Les personnes sans emploi ne sont pas éligibles.
          </div>

          <!-- Statut emploi -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">
              Votre situation <span class="text-red-500">*</span>
            </label>
            <div class="flex gap-3">
              <label
                v-for="statut in STATUTS_EMPLOI"
                :key="statut.value"
                class="flex-1 cursor-pointer"
              >
                <input
                  type="radio"
                  name="statut-emploi"
                  :value="statut.value"
                  v-model="form.statutEmploi"
                  class="hidden"
                />
                <div
                  class="text-center py-2.5 px-3 rounded-md border-2 transition-all text-sm font-medium"
                  :class="form.statutEmploi === statut.value
                    ? 'border-custom-green bg-custom-green/10 text-custom-green'
                    : 'border-gray-200 text-gray-500 hover:border-gray-300'"
                >
                  {{ statut.label }}
                </div>
              </label>
            </div>
          </div>

          <!-- Nom état civil -->
          <div>
            <label for="cand-nom" class="block text-sm font-medium text-gray-700 mb-1">
              Nom et prénoms à l'état civil <span class="text-red-500">*</span>
            </label>
            <input
              id="cand-nom"
              v-model="form.nomEtatCivil"
              type="text"
              class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green"
              placeholder="Ex: Aminata Diallo"
            />
          </div>

          <!-- Fonction & lieu -->
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label for="cand-fonction" class="block text-sm font-medium text-gray-700 mb-1">
                Fonction actuelle <span class="text-red-500">*</span>
              </label>
              <input
                id="cand-fonction"
                v-model="form.fonctionActuelle"
                type="text"
                class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green"
                placeholder="Ex: Ingénieure agronome"
              />
            </div>
            <div>
              <label for="cand-lieu" class="block text-sm font-medium text-gray-700 mb-1">
                Lieu de résidence ou de fonction <span class="text-red-500">*</span>
              </label>
              <input
                id="cand-lieu"
                v-model="form.lieuResidence"
                type="text"
                class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green"
                placeholder="Ex: Dakar, Sénégal"
              />
            </div>
          </div>

          <!-- Adéquation au profil -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">
              Répondez-vous entièrement au profil et à la spécialisation requis par le projet ?
              <span class="text-red-500">*</span>
            </label>
            <div class="flex gap-3">
              <label class="flex-1 cursor-pointer">
                <input type="radio" name="repond-profil" :value="true" v-model="repondProfil" class="hidden" />
                <div
                  class="text-center py-2.5 px-3 rounded-md border-2 transition-all text-sm font-medium"
                  :class="repondProfil === true
                    ? 'border-custom-green bg-custom-green/10 text-custom-green'
                    : 'border-gray-200 text-gray-500 hover:border-gray-300'"
                >
                  Oui, entièrement
                </div>
              </label>
              <label class="flex-1 cursor-pointer">
                <input type="radio" name="repond-profil" :value="false" v-model="repondProfil" class="hidden" />
                <div
                  class="text-center py-2.5 px-3 rounded-md border-2 transition-all text-sm font-medium"
                  :class="repondProfil === false
                    ? 'border-custom-chocolat bg-custom-chocolat/10 text-custom-chocolat'
                    : 'border-gray-200 text-gray-500 hover:border-gray-300'"
                >
                  Partiellement
                </div>
              </label>
            </div>
          </div>

          <!-- CV ou compte expertise -->
          <div class="p-3 bg-custom-green/10 border border-custom-green/40 rounded-md space-y-3">
            <p class="text-sm font-medium text-custom-green">
              Justificatif <span class="text-red-500">*</span>
              <span class="font-normal text-gray-500">(CV ou lien vers votre compte expertise)</span>
            </p>
            <div>
              <label for="cand-cv" class="block text-xs text-gray-600 mb-1">CV (PDF)</label>
              <input
                id="cand-cv"
                type="file"
                accept=".pdf"
                @change="handleCvChange"
                class="w-full text-sm text-gray-500 file:mr-3 file:py-1.5 file:px-3 file:rounded-md file:border-0 file:text-sm file:bg-custom-green file:text-white hover:file:bg-custom-green/90"
              />
            </div>
            <div>
              <label for="cand-expertise" class="block text-xs text-gray-600 mb-1">
                Ou lien vers votre compte expertise
              </label>
              <input
                id="cand-expertise"
                v-model="form.lienExpertise"
                type="url"
                class="w-full border-2 rounded-md p-2 border-custom-green/40 focus:outline-hidden focus:border-custom-green text-sm"
                placeholder="https://africans-world.org/profil/..."
              />
            </div>
          </div>

          <!-- Lettre de motivation (obligatoire) -->
          <div>
            <label for="cand-lettre" class="block text-sm font-medium text-gray-700 mb-1">
              Lettre de motivation <span class="text-red-500">*</span>
            </label>
            <textarea
              id="cand-lettre"
              v-model="form.lettreMotivation"
              rows="4"
              class="w-full border-2 rounded-md p-2 border-custom-green/70 focus:outline-hidden focus:border-custom-green text-sm"
              placeholder="Présentez votre motivation pour ce programme..."
            ></textarea>
          </div>

          <!-- Erreur -->
          <div
            v-if="erreur"
            class="bg-red-50 border border-red-200 text-red-600 px-4 py-2.5 rounded-md text-sm flex items-center gap-2"
          >
            <font-awesome-icon :icon="['fas', 'circle-exclamation']" />
            {{ erreur }}
          </div>

          <!-- Boutons -->
          <div class="flex flex-col sm:flex-row gap-3 justify-end pt-2">
            <button
              type="button"
              class="px-5 py-2.5 bg-gray-200 text-gray-700 rounded-md hover:bg-gray-300 transition-colors"
              @click="emit('close')"
            >
              Annuler
            </button>
            <button
              type="submit"
              :disabled="!isValid || loading"
              class="px-5 py-2.5 bg-custom-green text-white rounded-md hover:bg-custom-green/90 transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
            >
              <font-awesome-icon v-if="loading" :icon="['fas', 'spinner']" class="animate-spin" />
              {{ loading ? 'Envoi en cours...' : 'Envoyer ma candidature' }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { reactive, ref, computed, watch } from 'vue'
import {
  useSabbatiques,
  STATUTS_EMPLOI,
  type CandidatureForm,
} from '~/composables/useSabbatiques'
import { useExperts } from '~/composables/useExperts'
import { useUserStore } from '~/stores/user'

const props = defineProps<{
  programmeId: string
  open: boolean
}>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'success'): void
}>()

const { candidater } = useSabbatiques()
const { obtenirMaCandidature } = useExperts()
const userStore = useUserStore()

const form = reactive<Omit<CandidatureForm, 'repondProfil'>>({
  nomEtatCivil: '',
  fonctionActuelle: '',
  lieuResidence: '',
  statutEmploi: '',
  lettreMotivation: '',
  lienExpertise: '',
})
const repondProfil = ref<boolean | null>(null)
const cvFile = ref<File | null>(null)
const loading = ref(false)
const erreur = ref<string | null>(null)

const handleCvChange = (event: Event) => {
  const target = event.target as HTMLInputElement
  cvFile.value = target.files && target.files[0] ? target.files[0] : null
}

// Pré-remplir le lien d'expertise si l'utilisateur connecté possède un
// compte d'expertise validé (lien vers son profil unifié).
const prechargerLienExpertise = async () => {
  if (!userStore.isAuthenticated || !userStore.user) return
  // Ne pas écraser une saisie manuelle existante
  if (form.lienExpertise?.trim()) return

  const candidature = await obtenirMaCandidature()
  if (candidature?.statut === 'valide') {
    const origine = import.meta.client ? window.location.origin : ''
    form.lienExpertise = `${origine}/profil/${userStore.user.id}`
  }
}

// Charger automatiquement à l'ouverture de la modale
watch(
  () => props.open,
  (ouvert) => {
    if (ouvert) prechargerLienExpertise()
  },
  { immediate: true },
)

const isValid = computed(() => {
  return !!form.statutEmploi &&
    form.nomEtatCivil.trim() &&
    form.fonctionActuelle.trim() &&
    form.lieuResidence.trim() &&
    !!form.lettreMotivation?.trim() &&
    repondProfil.value !== null &&
    (cvFile.value !== null || !!form.lienExpertise?.trim())
})

const soumettre = async () => {
  if (!isValid.value) return
  loading.value = true
  erreur.value = null

  const ok = await candidater(
    props.programmeId,
    { ...form, repondProfil: repondProfil.value === true },
    cvFile.value,
  )

  loading.value = false
  if (ok) {
    emit('success')
  } else {
    erreur.value = 'Impossible d\'envoyer votre candidature. Vérifiez les champs et réessayez.'
  }
}
</script>
