<script setup lang="ts">
import type { StatutExpertise } from '~/composables/useAdminExperts'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const router = useRouter()
const id = route.params.id as string

const {
  demandeDetail,
  chargement,
  erreur,
  obtenirDemande,
  validerDemande,
  rejeterDemande,
} = useAdminExperts()

useHead({ title: computed(() => demandeDetail.value ? `${demandeDetail.value.prenom} ${demandeDetail.value.nom} — Admin` : 'Chargement…') })

const showRejetModal = ref(false)
const commentaireRejet = ref('')
const actionLoading = ref(false)
const successMsg = ref<string | null>(null)

function statutBadge(statut: StatutExpertise) {
  const map: Record<StatutExpertise, string> = {
    en_attente: 'badge-warning',
    valide: 'badge-success',
    refuse: 'badge-error',
  }
  return map[statut]
}

function statutLabel(statut: StatutExpertise) {
  const map: Record<StatutExpertise, string> = {
    en_attente: 'En attente',
    valide: 'Validée',
    refuse: 'Refusée',
  }
  return map[statut]
}

function formatDate(iso: string | null) {
  if (!iso) return '—'
  return new Date(iso).toLocaleDateString('fr-FR', { day: '2-digit', month: 'long', year: 'numeric' })
}

const commentaireValide = computed(() => commentaireRejet.value.trim().length >= 10)

async function valider() {
  actionLoading.value = true
  try {
    await validerDemande(id)
    afficherSucces('Demande validée — l\'expert est maintenant visible sur /experts.')
  }
  catch { /* erreur déjà dans le composable */ }
  finally {
    actionLoading.value = false
  }
}

async function confirmerRejet() {
  if (!commentaireValide.value) return
  actionLoading.value = true
  try {
    await rejeterDemande(id, commentaireRejet.value.trim())
    afficherSucces('Demande refusée — le candidat a été notifié par email.')
    showRejetModal.value = false
  }
  catch { /* erreur déjà dans le composable */ }
  finally {
    actionLoading.value = false
  }
}

function afficherSucces(msg: string) {
  successMsg.value = msg
  setTimeout(() => { successMsg.value = null }, 4000)
}

onMounted(() => obtenirDemande(id))
</script>

<template>
  <div>
    <!-- Fil d'ariane -->
    <div class="text-sm breadcrumbs mb-4">
      <ul>
        <li><NuxtLink to="/admin/experts">Demandes d'expertise</NuxtLink></li>
        <li>{{ demandeDetail?.prenom }} {{ demandeDetail?.nom }}</li>
      </ul>
    </div>

    <!-- Chargement -->
    <div v-if="chargement" class="flex justify-center py-20">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <!-- Erreur -->
    <div v-else-if="erreur" class="alert alert-error">
      <font-awesome-icon icon="circle-exclamation" />
      <span>{{ erreur }}</span>
      <button class="btn btn-sm btn-ghost ml-auto" @click="router.back()">Retour</button>
    </div>

    <template v-else-if="demandeDetail">
      <!-- Succès -->
      <div v-if="successMsg" class="alert alert-success mb-4">
        <font-awesome-icon icon="circle-check" />
        <span>{{ successMsg }}</span>
      </div>

      <!-- En-tête profil -->
      <div class="card bg-base-100 shadow-sm mb-4">
        <div class="card-body">
          <div class="flex flex-col sm:flex-row items-start sm:items-center gap-4">
            <div class="avatar">
              <div class="w-20 rounded-full bg-primary text-primary-content flex items-center justify-center">
                <img
                  v-if="demandeDetail.photoUrl"
                  :src="demandeDetail.photoUrl"
                  :alt="demandeDetail.prenom"
                  class="w-full h-full object-cover rounded-full"
                />
                <span v-else class="text-2xl font-bold">
                  {{ demandeDetail.prenom[0] }}{{ demandeDetail.nom[0] }}
                </span>
              </div>
            </div>
            <div class="flex-1">
              <h1 class="text-2xl font-bold">{{ demandeDetail.prenom }} {{ demandeDetail.nom }}</h1>
              <p class="text-base-content/70 text-sm mt-0.5">{{ demandeDetail.email }}</p>
              <div class="flex flex-wrap gap-2 mt-2">
                <span :class="['badge', statutBadge(demandeDetail.statut)]">
                  {{ statutLabel(demandeDetail.statut) }}
                </span>
                <span class="badge badge-ghost">
                  <font-awesome-icon icon="user-tie" class="mr-1 text-xs" />
                  {{ demandeDetail.domaine }}
                </span>
                <span v-if="demandeDetail.pays" class="badge badge-ghost">
                  <font-awesome-icon icon="location-dot" class="mr-1 text-xs" />
                  {{ demandeDetail.pays }}
                </span>
              </div>
            </div>
            <!-- Actions (seulement si en attente) -->
            <div v-if="demandeDetail.statut === 'en_attente'" class="flex gap-2 flex-shrink-0">
              <button
                class="btn btn-error btn-sm"
                :disabled="actionLoading"
                @click="showRejetModal = true"
              >
                <font-awesome-icon icon="xmark" class="mr-1" />
                Refuser
              </button>
              <button
                class="btn btn-success btn-sm"
                :disabled="actionLoading"
                @click="valider"
              >
                <span v-if="actionLoading" class="loading loading-spinner loading-sm" />
                <font-awesome-icon v-else icon="check" class="mr-1" />
                Valider
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Détails -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-4">
        <!-- Biographie + situations (2/3) -->
        <div class="lg:col-span-2 space-y-4">
          <div class="card bg-base-100 shadow-sm">
            <div class="card-body">
              <h2 class="card-title text-base">Biographie</h2>
              <p class="text-base-content/80 leading-relaxed text-sm whitespace-pre-wrap">
                {{ demandeDetail.biographie }}
              </p>
            </div>
          </div>

          <div class="card bg-base-100 shadow-sm">
            <div class="card-body">
              <h2 class="card-title text-base">Situations professionnelles</h2>
              <div class="flex flex-wrap gap-2">
                <span
                  v-for="s in demandeDetail.situationsProfessionnelles"
                  :key="s"
                  class="badge badge-outline"
                >{{ s }}</span>
                <span v-if="demandeDetail.situationsProfessionnelles.length === 0" class="text-base-content/50 text-sm">
                  Aucune situation renseignée
                </span>
              </div>
            </div>
          </div>

          <!-- Commentaire admin si refusé -->
          <div v-if="demandeDetail.commentaireAdmin" class="alert alert-error">
            <font-awesome-icon icon="comment" class="shrink-0" />
            <div>
              <p class="font-semibold text-sm">Motif du refus</p>
              <p class="text-sm mt-0.5">{{ demandeDetail.commentaireAdmin }}</p>
            </div>
          </div>
        </div>

        <!-- Infos latérales (1/3) -->
        <div class="space-y-4">
          <div class="card bg-base-100 shadow-sm">
            <div class="card-body gap-3">
              <h2 class="card-title text-base">Informations</h2>
              <div>
                <p class="text-xs text-base-content/50 uppercase tracking-wide font-semibold mb-0.5">Domaine</p>
                <p class="text-sm">{{ demandeDetail.domaine }}</p>
              </div>
              <div>
                <p class="text-xs text-base-content/50 uppercase tracking-wide font-semibold mb-0.5">Années d'expérience</p>
                <p class="text-sm">{{ demandeDetail.nbAnneesExperience }}</p>
              </div>
              <div v-if="demandeDetail.portfolio">
                <p class="text-xs text-base-content/50 uppercase tracking-wide font-semibold mb-0.5">Portfolio</p>
                <a :href="demandeDetail.portfolio" target="_blank" rel="noopener" class="link link-primary text-sm break-all">
                  {{ demandeDetail.portfolio }}
                </a>
              </div>
              <div>
                <p class="text-xs text-base-content/50 uppercase tracking-wide font-semibold mb-0.5">Soumis le</p>
                <p class="text-sm">{{ formatDate(demandeDetail.createdAt) }}</p>
              </div>
              <div v-if="demandeDetail.dateValidation">
                <p class="text-xs text-base-content/50 uppercase tracking-wide font-semibold mb-0.5">Décision le</p>
                <p class="text-sm">{{ formatDate(demandeDetail.dateValidation) }}</p>
              </div>
              <div v-if="demandeDetail.valideParNom">
                <p class="text-xs text-base-content/50 uppercase tracking-wide font-semibold mb-0.5">Traité par</p>
                <p class="text-sm">{{ demandeDetail.valideParNom }}</p>
              </div>
            </div>
          </div>

          <!-- Fiche publique si validée -->
          <NuxtLink
            v-if="demandeDetail.statut === 'valide'"
            :to="`/experts/${demandeDetail.utilisateurId}`"
            class="btn btn-outline btn-block btn-sm"
          >
            <font-awesome-icon icon="arrow-up-right-from-square" class="mr-1" />
            Voir la fiche publique
          </NuxtLink>
        </div>
      </div>
    </template>

    <!-- Modal refus (commentaire obligatoire) -->
    <Teleport to="body">
      <div v-if="showRejetModal" class="modal modal-open">
        <div class="modal-box max-w-md">
          <h3 class="font-bold text-lg mb-1">Refuser la demande</h3>
          <p class="text-base-content/70 text-sm mb-4">
            Indiquez le motif du refus. Il sera envoyé au candidat par email et
            consultable depuis son espace personnel.
          </p>
          <textarea
            v-model="commentaireRejet"
            rows="4"
            class="textarea textarea-bordered w-full text-sm"
            placeholder="Motif du refus (obligatoire, min. 10 caractères)…"
          />
          <p v-if="commentaireRejet.length > 0 && !commentaireValide" class="text-error text-xs mt-1">
            Le commentaire doit contenir au moins 10 caractères.
          </p>
          <div class="modal-action">
            <button class="btn btn-ghost" @click="showRejetModal = false">Annuler</button>
            <button
              class="btn btn-error"
              :disabled="actionLoading || !commentaireValide"
              @click="confirmerRejet"
            >
              <span v-if="actionLoading" class="loading loading-spinner loading-sm" />
              Confirmer le refus
            </button>
          </div>
        </div>
        <div class="modal-backdrop" @click="showRejetModal = false" />
      </div>
    </Teleport>
  </div>
</template>
