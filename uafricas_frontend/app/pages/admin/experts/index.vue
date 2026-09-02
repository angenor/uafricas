<script setup lang="ts">
import type { FiltresExpertise, StatutExpertise } from '~/composables/useAdminExperts'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

useHead({ title: 'Demandes d\'expertise, Admin' })

const {
  demandes,
  nbEnAttente,
  chargement,
  erreur,
  total,
  page,
  totalPages,
  listerDemandes,
  validerDemande,
  rejeterDemande,
} = useAdminExperts()

const filtreStatut = ref<FiltresExpertise['statut']>('')
const filtreRecherche = ref('')
const successMsg = ref<string | null>(null)

const showRejetModal = ref(false)
const commentaireRejet = ref('')
const actionLoading = ref(false)
const cibleId = ref<string | null>(null)

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

function formatDate(iso: string) {
  return new Date(iso).toLocaleDateString('fr-FR', { day: '2-digit', month: '2-digit', year: 'numeric' })
}

async function charger(p = page.value) {
  await listerDemandes({
    statut: filtreStatut.value,
    recherche: filtreRecherche.value,
    page: p,
    par_page: 20,
  })
}

async function confirmerApprobation(id: string) {
  actionLoading.value = true
  try {
    await validerDemande(id)
    afficherSucces('Demande validée : l\'expert est maintenant visible sur /experts.')
    await charger(page.value)
  }
  catch { /* erreur déjà dans le composable */ }
  finally {
    actionLoading.value = false
  }
}

function ouvrirRejet(id: string) {
  cibleId.value = id
  commentaireRejet.value = ''
  showRejetModal.value = true
}

const commentaireValide = computed(() => commentaireRejet.value.trim().length >= 10)

async function confirmerRejet() {
  if (!cibleId.value || !commentaireValide.value) return
  actionLoading.value = true
  try {
    await rejeterDemande(cibleId.value, commentaireRejet.value.trim())
    afficherSucces('Demande refusée : le candidat a été notifié par email.')
    showRejetModal.value = false
    await charger(page.value)
  }
  catch { /* erreur déjà dans le composable */ }
  finally {
    actionLoading.value = false
    cibleId.value = null
  }
}

function afficherSucces(msg: string) {
  successMsg.value = msg
  setTimeout(() => { successMsg.value = null }, 4000)
}

let rechercheTimeout: ReturnType<typeof setTimeout> | null = null
watch(filtreRecherche, () => {
  if (rechercheTimeout) clearTimeout(rechercheTimeout)
  rechercheTimeout = setTimeout(() => charger(1), 350)
})

watch(filtreStatut, () => charger(1))

onMounted(() => charger(1))
</script>

<template>
  <div>
    <AdminPageHeader
      titre="Demandes d'expertise"
      sous-titre="Valider ou refuser les demandes pour devenir expert"
    >
      <template #actions>
        <span v-if="nbEnAttente > 0" class="badge badge-warning badge-lg gap-1">
          <font-awesome-icon icon="clock" />
          {{ nbEnAttente }} en attente
        </span>
      </template>
    </AdminPageHeader>

    <!-- Alertes -->
    <div v-if="successMsg" class="alert alert-success mb-4">
      <font-awesome-icon icon="circle-check" />
      <span>{{ successMsg }}</span>
    </div>
    <div v-if="erreur" class="alert alert-error mb-4">
      <font-awesome-icon icon="circle-exclamation" />
      <span>{{ erreur }}</span>
    </div>

    <!-- Filtres -->
    <div class="card bg-base-100 shadow-sm mb-4">
      <div class="card-body py-3">
        <div class="flex flex-wrap gap-3 items-end">
          <div class="flex-1 min-w-48">
            <label class="label py-0 pb-1">
              <span class="label-text text-xs">Recherche</span>
            </label>
            <input
              v-model="filtreRecherche"
              type="text"
              placeholder="Nom, prénom, domaine…"
              class="input input-bordered input-sm w-full"
            />
          </div>
          <div>
            <label class="label py-0 pb-1">
              <span class="label-text text-xs">Statut</span>
            </label>
            <select v-model="filtreStatut" class="select select-bordered select-sm">
              <option value="">Toutes</option>
              <option value="en_attente">En attente</option>
              <option value="valide">Validées</option>
              <option value="refuse">Refusées</option>
            </select>
          </div>
          <button
            class="btn btn-ghost btn-sm"
            @click="() => { filtreRecherche = ''; filtreStatut = ''; charger(1) }"
          >
            Réinitialiser
          </button>
        </div>
      </div>
    </div>

    <!-- Tableau -->
    <div class="card bg-base-100 shadow-sm">
      <div class="card-body p-0">
        <div v-if="chargement" class="flex justify-center py-12">
          <span class="loading loading-spinner loading-lg" />
        </div>

        <template v-else>
          <div class="overflow-x-auto">
            <table class="table table-zebra table-sm">
              <thead>
                <tr>
                  <th>Candidat</th>
                  <th>Domaine</th>
                  <th class="text-center">Expérience</th>
                  <th class="text-center">Statut</th>
                  <th>Soumis le</th>
                  <th class="text-center">Actions</th>
                </tr>
              </thead>
              <tbody>
                <tr v-if="demandes.length === 0">
                  <td colspan="6" class="text-center text-base-content/50 py-8">
                    Aucune demande trouvée
                  </td>
                </tr>
                <tr v-for="d in demandes" :key="d.id">
                  <td>
                    <div class="flex items-center gap-2">
                      <div class="avatar">
                        <div class="w-8 rounded-full bg-primary text-primary-content flex items-center justify-center">
                          <img
                            v-if="d.photoUrl"
                            :src="d.photoUrl"
                            :alt="d.prenom + ' ' + d.nom"
                            class="w-full h-full object-cover rounded-full"
                          />
                          <span v-else class="text-xs font-bold">{{ d.prenom[0] }}{{ d.nom[0] }}</span>
                        </div>
                      </div>
                      <NuxtLink
                        :to="`/admin/experts/${d.id}`"
                        class="font-semibold text-sm link link-hover"
                      >
                        {{ d.prenom }} {{ d.nom }}
                      </NuxtLink>
                    </div>
                  </td>
                  <td class="text-sm">{{ d.domaine }}</td>
                  <td class="text-center text-sm">{{ d.nbAnneesExperience }} an{{ d.nbAnneesExperience > 1 ? 's' : '' }}</td>
                  <td class="text-center">
                    <span :class="['badge badge-sm', statutBadge(d.statut)]">
                      {{ statutLabel(d.statut) }}
                    </span>
                  </td>
                  <td class="text-sm text-base-content/70">{{ formatDate(d.createdAt) }}</td>
                  <td>
                    <div class="flex gap-1 justify-center">
                      <NuxtLink
                        :to="`/admin/experts/${d.id}`"
                        class="btn btn-ghost btn-xs"
                        title="Voir le détail"
                      >
                        <font-awesome-icon icon="eye" />
                      </NuxtLink>
                      <button
                        v-if="d.statut === 'en_attente'"
                        class="btn btn-success btn-xs"
                        :disabled="actionLoading"
                        title="Valider"
                        @click="confirmerApprobation(d.id)"
                      >
                        <font-awesome-icon icon="check" />
                      </button>
                      <button
                        v-if="d.statut === 'en_attente'"
                        class="btn btn-error btn-xs"
                        :disabled="actionLoading"
                        title="Refuser"
                        @click="ouvrirRejet(d.id)"
                      >
                        <font-awesome-icon icon="xmark" />
                      </button>
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>

          <!-- Pagination -->
          <div v-if="totalPages > 1" class="flex justify-center gap-1 py-4">
            <button
              class="btn btn-sm"
              :disabled="page <= 1"
              @click="charger(page - 1)"
            >
              «
            </button>
            <button
              v-for="p in totalPages"
              :key="p"
              :class="['btn btn-sm', p === page ? 'btn-primary' : 'btn-ghost']"
              @click="charger(p)"
            >
              {{ p }}
            </button>
            <button
              class="btn btn-sm"
              :disabled="page >= totalPages"
              @click="charger(page + 1)"
            >
              »
            </button>
          </div>

          <div class="px-4 py-2 text-xs text-base-content/50 border-t">
            {{ total }} demande{{ total > 1 ? 's' : '' }} au total
          </div>
        </template>
      </div>
    </div>

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
