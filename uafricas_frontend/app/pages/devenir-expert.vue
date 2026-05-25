<script setup lang="ts">
import {
  CATEGORIES_EXPERTISE,
  PROFILS_PROFESSIONNELS,
  type CandidatureExpertBody,
  type MaCandidatureAPI,
} from '~/composables/useExperts'

useHead({ title: 'Devenir expert - UAfricas' })
useAOS()

const router = useRouter()
const { isAuthenticated } = useAuth()
const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string

const profilComposable = useProfil()
const { creerCandidature, obtenirMaCandidature } = useExperts()

// ── Données de référence ──
interface PaysOptionAPI { id: string, nom: string }
const listePays = ref<PaysOptionAPI[]>([])
const domainesDisponibles = CATEGORIES_EXPERTISE.filter(d => d !== 'Tout')
const situationsDisponibles = PROFILS_PROFESSIONNELS.filter(s => s.id !== 'tous')

// ── État de chargement / soumission ──
const chargementInit = ref(true)
const soumission = ref(false)
const erreurForm = ref<string | null>(null)
const succes = ref(false)
const candidatureActive = ref<MaCandidatureAPI | null>(null)
const candidatureRefusee = ref<MaCandidatureAPI | null>(null)

// ── Formulaire ──
const form = reactive({
  fonction: '',
  paysId: '',
  domaine: '',
  biographie: '',
  nbAnnees: 0,
  situations: [] as string[],
  portfolio: '',
})

// ── Photo ──
const photoUrlActuelle = ref<string | null>(null)
const photoPreview = ref<string | null>(null)
const photoFile = ref<File | null>(null)
const inputPhoto = ref<HTMLInputElement | null>(null)

function declencherSelectionPhoto() {
  inputPhoto.value?.click()
}

function onPhotoChange(event: Event) {
  const fichier = (event.target as HTMLInputElement).files?.[0]
  if (!fichier) return
  photoFile.value = fichier
  photoPreview.value = URL.createObjectURL(fichier)
}

function toggleSituation(idSituation: string) {
  const idx = form.situations.indexOf(idSituation)
  if (idx === -1) form.situations.push(idSituation)
  else form.situations.splice(idx, 1)
}

// ── Validation client ──
const erreurs = reactive<Record<string, string>>({})

function valider(): boolean {
  Object.keys(erreurs).forEach(k => delete erreurs[k])

  if (!form.fonction.trim()) erreurs.fonction = 'Votre fonction est requise.'
  if (!form.paysId) erreurs.paysId = 'Sélectionnez votre pays de résidence.'
  if (!form.domaine) erreurs.domaine = 'Choisissez un domaine d\'expertise.'
  if (form.biographie.trim().length < 10) erreurs.biographie = 'La biographie doit contenir au moins 10 caractères.'
  else if (form.biographie.trim().length > 5000) erreurs.biographie = 'La biographie ne doit pas dépasser 5000 caractères.'
  if (form.nbAnnees < 0 || Number.isNaN(form.nbAnnees)) erreurs.nbAnnees = 'Le nombre d\'années doit être positif.'
  if (form.situations.length === 0) erreurs.situations = 'Sélectionnez au moins une situation professionnelle.'
  if (form.portfolio.trim() && !/^https?:\/\/.+/.test(form.portfolio.trim())) {
    erreurs.portfolio = 'Le lien doit commencer par http:// ou https://'
  }

  return Object.keys(erreurs).length === 0
}

// ── Soumission ──
async function soumettre() {
  erreurForm.value = null
  if (!valider()) {
    erreurForm.value = 'Veuillez corriger les champs indiqués.'
    return
  }

  soumission.value = true
  try {
    // 1. Photo (optionnelle)
    if (photoFile.value) {
      await profilComposable.changerPhoto(photoFile.value)
    }

    // 2. Profil de base (fonction + pays de résidence)
    await profilComposable.modifierProfil({
      fonction: form.fonction.trim(),
      pays_residence_id: form.paysId,
    })

    // 3. Candidature d'expertise
    const body: CandidatureExpertBody = {
      domaine: form.domaine,
      biographie: form.biographie.trim(),
      nb_annees_experience: form.nbAnnees,
      situations_professionnelles: form.situations,
    }
    if (form.portfolio.trim()) body.portfolio = form.portfolio.trim()

    const resultat = await creerCandidature(body)
    if (!resultat) {
      throw new Error('La demande n\'a pas pu être enregistrée.')
    }

    succes.value = true
    window.scrollTo({ top: 0, behavior: 'smooth' })
  }
  catch (e: any) {
    const message = e?.data?.error || e?.message || 'Une erreur est survenue lors de la soumission.'
    erreurForm.value = message
  }
  finally {
    soumission.value = false
  }
}

// ── Initialisation ──
async function chargerPays() {
  try {
    const reponse = await $fetch<{ success: boolean, data: PaysOptionAPI[] | null }>(
      `${apiBase}/api/pays`,
    )
    if (reponse.success && reponse.data) {
      listePays.value = reponse.data
    }
  }
  catch {
    // non bloquant : la liste reste vide
  }
}

onMounted(async () => {
  if (!isAuthenticated.value) {
    router.replace({ path: '/login', query: { redirect: '/devenir-expert' } })
    return
  }

  await chargerPays()

  // Pré-remplir depuis le profil
  try {
    const profil = await profilComposable.chargerProfil()
    if (profil) {
      form.fonction = profil.fonction ?? ''
      form.paysId = profil.pays_residence_id ?? ''
      photoUrlActuelle.value = profil.photo_url
    }
  }
  catch {
    // non bloquant
  }

  // Vérifier une candidature active
  try {
    const candidature = await obtenirMaCandidature()
    if (candidature) {
      if (candidature.statut === 'refuse') {
        candidatureRefusee.value = candidature
      }
      else {
        candidatureActive.value = candidature
      }
    }
  }
  catch {
    // non bloquant
  }

  chargementInit.value = false
})
</script>

<template>
  <div class="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
    <div class="max-w-3xl mx-auto">
      <!-- En-tête -->
      <div class="text-center mb-8" data-aos="fade-up">
        <div class="inline-flex items-center justify-center w-16 h-16 rounded-2xl bg-gradient-to-br from-custom-chocolat to-custom-green mb-4">
          <font-awesome-icon icon="fa-solid fa-hand-holding-heart" class="text-white text-2xl" />
        </div>
        <h1 class="text-3xl font-bold text-gray-800 font-oswald">Apporter mon expertise</h1>
        <p class="text-gray-500 mt-2 max-w-xl mx-auto">
          Complétez votre profil et présentez votre expertise. Votre demande sera examinée par un administrateur avant publication.
        </p>
      </div>

      <!-- Chargement initial -->
      <div v-if="chargementInit" class="flex justify-center py-20">
        <font-awesome-icon icon="fa-solid fa-spinner" class="text-3xl text-custom-chocolat animate-spin" />
      </div>

      <!-- Confirmation de soumission -->
      <div
        v-else-if="succes"
        class="bg-white rounded-2xl shadow-sm border border-green-100 p-8 text-center"
        data-aos="fade-up"
      >
        <div class="inline-flex items-center justify-center w-16 h-16 rounded-full bg-green-100 mb-4">
          <font-awesome-icon icon="fa-solid fa-circle-check" class="text-green-600 text-3xl" />
        </div>
        <h2 class="text-xl font-bold text-gray-800">Demande envoyée !</h2>
        <p class="text-gray-500 mt-2">
          Votre demande sera examinée par un administrateur. Vous serez notifié(e) par email de la décision.
        </p>
        <div class="flex flex-wrap justify-center gap-3 mt-6">
          <NuxtLink
            to="/mon-compte/profil"
            class="inline-flex items-center gap-2 px-5 py-2.5 text-sm bg-gradient-to-r from-custom-chocolat to-custom-green text-white font-medium rounded-xl hover:shadow-lg transition-all duration-300"
          >
            <font-awesome-icon icon="fa-solid fa-user" />
            Suivre ma demande
          </NuxtLink>
          <NuxtLink
            to="/experts"
            class="inline-flex items-center gap-2 px-5 py-2.5 text-sm border border-gray-300 text-gray-600 font-medium rounded-xl hover:bg-gray-50 transition-all duration-300"
          >
            Voir les experts
          </NuxtLink>
        </div>
      </div>

      <!-- Demande déjà active (en attente ou validée) -->
      <div
        v-else-if="candidatureActive"
        class="bg-white rounded-2xl shadow-sm border border-gray-100 p-8 text-center"
        data-aos="fade-up"
      >
        <div
          class="inline-flex items-center justify-center w-16 h-16 rounded-full mb-4"
          :class="candidatureActive.statut === 'valide' ? 'bg-green-100' : 'bg-amber-100'"
        >
          <font-awesome-icon
            :icon="candidatureActive.statut === 'valide' ? 'fa-solid fa-circle-check' : 'fa-solid fa-clock'"
            class="text-3xl"
            :class="candidatureActive.statut === 'valide' ? 'text-green-600' : 'text-amber-500'"
          />
        </div>
        <h2 class="text-xl font-bold text-gray-800">
          {{ candidatureActive.statut === 'valide' ? 'Vous êtes déjà expert' : 'Demande en cours d\'examen' }}
        </h2>
        <p class="text-gray-500 mt-2">
          {{
            candidatureActive.statut === 'valide'
              ? 'Votre profil d\'expert est validé et visible publiquement.'
              : 'Vous avez déjà une demande en attente de validation. Vous ne pouvez pas en soumettre une nouvelle pour le moment.'
          }}
        </p>
        <div class="flex flex-wrap justify-center gap-3 mt-6">
          <NuxtLink
            to="/mon-compte/profil"
            class="inline-flex items-center gap-2 px-5 py-2.5 text-sm bg-gradient-to-r from-custom-chocolat to-custom-green text-white font-medium rounded-xl hover:shadow-lg transition-all duration-300"
          >
            <font-awesome-icon icon="fa-solid fa-user" />
            Voir le statut
          </NuxtLink>
        </div>
      </div>

      <!-- Formulaire -->
      <form
        v-else
        class="space-y-6"
        data-aos="fade-up"
        @submit.prevent="soumettre"
      >
        <!-- Bandeau re-soumission après refus -->
        <div v-if="candidatureRefusee" class="bg-red-50 border border-red-200 rounded-xl p-4">
          <p class="text-sm font-semibold text-red-700">Votre précédente demande a été refusée</p>
          <p v-if="candidatureRefusee.commentaireAdmin" class="text-sm text-red-600 mt-1">
            Motif : {{ candidatureRefusee.commentaireAdmin }}
          </p>
          <p class="text-xs text-red-500 mt-1">Corrigez votre dossier ci-dessous pour soumettre une nouvelle demande.</p>
        </div>

        <!-- Erreur globale -->
        <div v-if="erreurForm" class="bg-red-50 border border-red-200 rounded-xl p-4 flex items-start gap-2">
          <font-awesome-icon icon="fa-solid fa-circle-exclamation" class="text-red-500 mt-0.5" />
          <span class="text-sm text-red-600">{{ erreurForm }}</span>
        </div>

        <!-- Section profil de base -->
        <div class="bg-white rounded-2xl shadow-sm border border-gray-100 p-6">
          <h2 class="text-lg font-semibold text-gray-800 mb-4 flex items-center gap-2">
            <font-awesome-icon icon="fa-solid fa-user" class="text-custom-chocolat" />
            Mon profil
          </h2>

          <!-- Photo -->
          <div class="flex items-center gap-4 mb-5">
            <div class="w-20 h-20 rounded-full overflow-hidden bg-gray-100 border border-gray-200 flex items-center justify-center shrink-0">
              <img
                v-if="photoPreview || photoUrlActuelle"
                :src="photoPreview || photoUrlActuelle || ''"
                alt="Photo de profil"
                class="w-full h-full object-cover"
              />
              <font-awesome-icon v-else icon="fa-solid fa-user" class="text-gray-300 text-2xl" />
            </div>
            <div>
              <button
                type="button"
                class="inline-flex items-center gap-2 px-4 py-2 text-sm border border-gray-300 text-gray-700 font-medium rounded-lg hover:bg-gray-50 transition-colors"
                @click="declencherSelectionPhoto"
              >
                <font-awesome-icon icon="fa-solid fa-image" />
                Changer la photo
              </button>
              <p class="text-xs text-gray-400 mt-1">JPEG ou PNG, format carré recommandé.</p>
              <input
                ref="inputPhoto"
                type="file"
                accept="image/jpeg,image/png"
                class="hidden"
                @change="onPhotoChange"
              />
            </div>
          </div>

          <!-- Fonction -->
          <div class="mb-4">
            <label class="block text-sm font-medium text-gray-700 mb-1">Fonction <span class="text-red-500">*</span></label>
            <input
              v-model="form.fonction"
              type="text"
              placeholder="Ex : Ingénieure logiciel, Médecin, Agronome…"
              class="w-full px-4 py-2.5 border border-gray-300 rounded-xl focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all bg-gray-50 focus:bg-white"
            />
            <p v-if="erreurs.fonction" class="text-xs text-red-500 mt-1">{{ erreurs.fonction }}</p>
          </div>

          <!-- Pays de résidence -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Pays de résidence <span class="text-red-500">*</span></label>
            <select
              v-model="form.paysId"
              class="w-full px-4 py-2.5 border border-gray-300 rounded-xl focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all bg-gray-50 focus:bg-white"
            >
              <option value="">— Sélectionner —</option>
              <option v-for="p in listePays" :key="p.id" :value="p.id">{{ p.nom }}</option>
            </select>
            <p v-if="erreurs.paysId" class="text-xs text-red-500 mt-1">{{ erreurs.paysId }}</p>
          </div>
        </div>

        <!-- Section expertise -->
        <div class="bg-white rounded-2xl shadow-sm border border-gray-100 p-6">
          <h2 class="text-lg font-semibold text-gray-800 mb-4 flex items-center gap-2">
            <font-awesome-icon icon="fa-solid fa-user-tie" class="text-custom-chocolat" />
            Mon expertise
          </h2>

          <!-- Domaine -->
          <div class="mb-4">
            <label class="block text-sm font-medium text-gray-700 mb-1">Domaine d'expertise <span class="text-red-500">*</span></label>
            <select
              v-model="form.domaine"
              class="w-full px-4 py-2.5 border border-gray-300 rounded-xl focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all bg-gray-50 focus:bg-white"
            >
              <option value="">— Sélectionner —</option>
              <option v-for="d in domainesDisponibles" :key="d" :value="d">{{ d }}</option>
            </select>
            <p v-if="erreurs.domaine" class="text-xs text-red-500 mt-1">{{ erreurs.domaine }}</p>
          </div>

          <!-- Biographie -->
          <div class="mb-4">
            <label class="block text-sm font-medium text-gray-700 mb-1">Biographie <span class="text-red-500">*</span></label>
            <textarea
              v-model="form.biographie"
              rows="5"
              placeholder="Présentez votre parcours, vos réalisations et ce que vous pouvez apporter…"
              class="w-full px-4 py-2.5 border border-gray-300 rounded-xl focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all bg-gray-50 focus:bg-white resize-y"
            />
            <div class="flex justify-between mt-1">
              <p v-if="erreurs.biographie" class="text-xs text-red-500">{{ erreurs.biographie }}</p>
              <p class="text-xs text-gray-400 ml-auto">{{ form.biographie.length }} / 5000</p>
            </div>
          </div>

          <!-- Années d'expérience -->
          <div class="mb-4">
            <label class="block text-sm font-medium text-gray-700 mb-1">Années d'expérience <span class="text-red-500">*</span></label>
            <input
              v-model.number="form.nbAnnees"
              type="number"
              min="0"
              class="w-full px-4 py-2.5 border border-gray-300 rounded-xl focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all bg-gray-50 focus:bg-white"
            />
            <p v-if="erreurs.nbAnnees" class="text-xs text-red-500 mt-1">{{ erreurs.nbAnnees }}</p>
          </div>

          <!-- Situations professionnelles -->
          <div class="mb-4">
            <label class="block text-sm font-medium text-gray-700 mb-2">Situation(s) professionnelle(s) <span class="text-red-500">*</span></label>
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-2">
              <label
                v-for="s in situationsDisponibles"
                :key="s.id"
                class="flex items-center gap-2 px-3 py-2 rounded-xl border cursor-pointer transition-colors"
                :class="form.situations.includes(s.id)
                  ? 'border-custom-green bg-green-50 text-custom-green'
                  : 'border-gray-200 hover:bg-gray-50 text-gray-600'"
              >
                <input
                  type="checkbox"
                  :checked="form.situations.includes(s.id)"
                  class="accent-custom-green"
                  @change="toggleSituation(s.id)"
                />
                <span class="text-sm">{{ s.label }}</span>
              </label>
            </div>
            <p v-if="erreurs.situations" class="text-xs text-red-500 mt-1">{{ erreurs.situations }}</p>
          </div>

          <!-- Portfolio -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Portfolio / site web <span class="text-gray-400">(optionnel)</span></label>
            <input
              v-model="form.portfolio"
              type="url"
              placeholder="https://…"
              class="w-full px-4 py-2.5 border border-gray-300 rounded-xl focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all bg-gray-50 focus:bg-white"
            />
            <p v-if="erreurs.portfolio" class="text-xs text-red-500 mt-1">{{ erreurs.portfolio }}</p>
          </div>
        </div>

        <!-- Actions -->
        <div class="flex justify-end gap-3">
          <NuxtLink
            to="/experts"
            class="inline-flex items-center gap-2 px-5 py-2.5 text-sm border border-gray-300 text-gray-600 font-medium rounded-xl hover:bg-gray-50 transition-all"
          >
            Annuler
          </NuxtLink>
          <button
            type="submit"
            :disabled="soumission"
            class="inline-flex items-center gap-2 px-6 py-2.5 text-sm bg-gradient-to-r from-custom-chocolat to-custom-green text-white font-medium rounded-xl hover:shadow-lg transition-all duration-300 disabled:opacity-60 disabled:cursor-not-allowed"
          >
            <font-awesome-icon
              :icon="soumission ? 'fa-solid fa-spinner' : 'fa-solid fa-paper-plane'"
              :class="soumission ? 'animate-spin' : ''"
            />
            {{ soumission ? 'Envoi en cours…' : 'Soumettre ma demande' }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>
