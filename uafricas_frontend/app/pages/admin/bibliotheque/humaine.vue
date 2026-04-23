<script setup lang="ts">
import type { DemandeBiblioHumaine, StatutDemande } from '~/mocks/bibliotheques-humaines'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

useHead({ title: 'Bibliothèques Humaines — Admin' })

const {
  demandes,
  demandeDetail,
  nbEnAttente,
  chargement,
  erreur,
  listerDemandes,
  obtenirDemande,
  approuverDemande,
  rejeterDemande,
} = useAdminBibliothequeHumaine()

const filtreStatut = ref<StatutDemande | ''>('')
const filtreRecherche = ref('')
const successMsg = ref<string | null>(null)

const showDetailModal = ref(false)
const showRejetModal = ref(false)
const commentaireRejet = ref('')
const actionLoading = ref(false)
const cibleId = ref<string | null>(null)

function statutBadge(statut: StatutDemande) {
  const map: Record<StatutDemande, string> = {
    en_attente: 'badge-warning',
    valide: 'badge-success',
    rejete: 'badge-error',
  }
  return map[statut]
}

function statutLabel(statut: StatutDemande) {
  const map: Record<StatutDemande, string> = {
    en_attente: 'En attente',
    valide: 'Validée',
    rejete: 'Rejetée',
  }
  return map[statut]
}

function formatDate(iso: string) {
  return new Date(iso).toLocaleDateString('fr-FR', { day: '2-digit', month: '2-digit', year: 'numeric' })
}

async function charger() {
  await listerDemandes({ statut: filtreStatut.value, recherche: filtreRecherche.value })
}

async function ouvrirDetail(id: string) {
  await obtenirDemande(id)
  showDetailModal.value = true
}

function ouvrirRejet(id: string) {
  cibleId.value = id
  commentaireRejet.value = ''
  showRejetModal.value = true
}

async function confirmerApprobation(id: string) {
  actionLoading.value = true
  try {
    await approuverDemande(id)
    afficherSucces('Demande approuvée — la Bibliothèque Humaine est maintenant visible publiquement.')
    showDetailModal.value = false
    await charger()
  }
  catch { /* erreur déjà dans le store */ }
  finally {
    actionLoading.value = false
  }
}

async function confirmerRejet() {
  if (!cibleId.value) return
  actionLoading.value = true
  try {
    await rejeterDemande(cibleId.value, commentaireRejet.value.trim() || undefined)
    afficherSucces('Demande rejetée.')
    showRejetModal.value = false
    showDetailModal.value = false
    await charger()
  }
  catch { /* erreur déjà dans le store */ }
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
  rechercheTimeout = setTimeout(charger, 350)
})

watch(filtreStatut, charger)

onMounted(charger)
</script>

<template>
  <div>
    <AdminPageHeader
      titre="Bibliothèques Humaines"
      sous-titre="Valider ou rejeter les demandes d'inscription"
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
              placeholder="Nom, fonction, pays…"
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
              <option value="rejete">Rejetées</option>
            </select>
          </div>
          <button
            class="btn btn-ghost btn-sm"
            @click="() => { filtreRecherche = ''; filtreStatut = ''; charger() }"
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
                  <th>Fonction</th>
                  <th>Pays</th>
                  <th>Spécialités</th>
                  <th class="text-center">Statut</th>
                  <th>Soumis le</th>
                  <th class="text-center">Actions</th>
                </tr>
              </thead>
              <tbody>
                <tr v-if="demandes.length === 0">
                  <td colspan="7" class="text-center text-base-content/50 py-8">
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
                          />
                          <span v-else class="text-xs font-bold">{{ d.prenom[0] }}{{ d.nom[0] }}</span>
                        </div>
                      </div>
                      <span class="font-semibold text-sm">{{ d.prenom }} {{ d.nom }}</span>
                    </div>
                  </td>
                  <td class="text-sm">{{ d.fonction }}</td>
                  <td class="text-sm">{{ d.pays }}</td>
                  <td>
                    <div class="flex flex-wrap gap-1">
                      <span
                        v-for="s in d.specialites.slice(0, 2)"
                        :key="s"
                        class="badge badge-ghost badge-xs"
                      >{{ s }}</span>
                      <span v-if="d.specialites.length > 2" class="badge badge-ghost badge-xs">
                        +{{ d.specialites.length - 2 }}
                      </span>
                    </div>
                  </td>
                  <td class="text-center">
                    <span :class="['badge badge-sm', statutBadge(d.statut)]">
                      {{ statutLabel(d.statut) }}
                    </span>
                  </td>
                  <td class="text-sm text-base-content/70">{{ formatDate(d.dateSubmission) }}</td>
                  <td>
                    <div class="flex gap-1 justify-center">
                      <button class="btn btn-ghost btn-xs" @click="ouvrirDetail(d.id)">
                        <font-awesome-icon icon="eye" />
                      </button>
                      <button
                        v-if="d.statut !== 'valide'"
                        class="btn btn-success btn-xs"
                        title="Approuver"
                        @click="confirmerApprobation(d.id)"
                      >
                        <font-awesome-icon icon="check" />
                      </button>
                      <button
                        v-if="d.statut !== 'rejete'"
                        class="btn btn-error btn-xs"
                        title="Rejeter"
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
        </template>
      </div>
    </div>

    <!-- Modal détail -->
    <Teleport to="body">
      <div v-if="showDetailModal && demandeDetail" class="modal modal-open">
        <div class="modal-box max-w-2xl">
          <button
            class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
            @click="showDetailModal = false"
          >✕</button>

          <div class="flex items-center gap-4 mb-4">
            <div class="avatar">
              <div class="w-16 rounded-full bg-primary text-primary-content flex items-center justify-center">
                <img v-if="demandeDetail.photoUrl" :src="demandeDetail.photoUrl" :alt="demandeDetail.prenom" />
                <span v-else class="text-xl font-bold">{{ demandeDetail.prenom[0] }}{{ demandeDetail.nom[0] }}</span>
              </div>
            </div>
            <div>
              <h3 class="text-xl font-bold">{{ demandeDetail.prenom }} {{ demandeDetail.nom }}</h3>
              <p class="text-base-content/70 text-sm">{{ demandeDetail.fonction }} — {{ demandeDetail.pays }}</p>
              <span :class="['badge mt-1', statutBadge(demandeDetail.statut)]">
                {{ statutLabel(demandeDetail.statut) }}
              </span>
            </div>
          </div>

          <div class="divider my-2" />

          <div class="space-y-4 text-sm">
            <div>
              <p class="font-semibold text-xs text-base-content/50 uppercase tracking-wide mb-1">Biographie</p>
              <p class="text-base-content/80 leading-relaxed">{{ demandeDetail.biographie }}</p>
            </div>
            <div>
              <p class="font-semibold text-xs text-base-content/50 uppercase tracking-wide mb-1">Spécialités</p>
              <div class="flex flex-wrap gap-1">
                <span
                  v-for="s in demandeDetail.specialites"
                  :key="s"
                  class="badge badge-outline badge-sm"
                >{{ s }}</span>
              </div>
            </div>
            <div class="flex gap-8">
              <div>
                <p class="font-semibold text-xs text-base-content/50 uppercase tracking-wide mb-0.5">Soumis le</p>
                <p>{{ formatDate(demandeDetail.dateSubmission) }}</p>
              </div>
              <div v-if="demandeDetail.dateMaj">
                <p class="font-semibold text-xs text-base-content/50 uppercase tracking-wide mb-0.5">Traité le</p>
                <p>{{ formatDate(demandeDetail.dateMaj) }}</p>
              </div>
            </div>
            <div v-if="demandeDetail.commentaireAdmin" class="alert alert-error alert-sm py-2">
              <font-awesome-icon icon="comment" class="shrink-0" />
              <div>
                <p class="font-semibold text-xs mb-0.5">Commentaire admin</p>
                <p>{{ demandeDetail.commentaireAdmin }}</p>
              </div>
            </div>
          </div>

          <div class="modal-action">
            <button class="btn btn-ghost" @click="showDetailModal = false">Fermer</button>
            <button
              v-if="demandeDetail.statut !== 'rejete'"
              class="btn btn-error btn-sm"
              :disabled="actionLoading"
              @click="() => { ouvrirRejet(demandeDetail!.id); showDetailModal = false }"
            >
              <font-awesome-icon icon="xmark" class="mr-1" /> Rejeter
            </button>
            <button
              v-if="demandeDetail.statut !== 'valide'"
              class="btn btn-success"
              :disabled="actionLoading"
              @click="confirmerApprobation(demandeDetail.id)"
            >
              <span v-if="actionLoading" class="loading loading-spinner loading-sm" />
              <font-awesome-icon v-else icon="check" class="mr-1" /> Approuver
            </button>
          </div>
        </div>
        <div class="modal-backdrop" @click="showDetailModal = false" />
      </div>
    </Teleport>

    <!-- Modal rejet -->
    <Teleport to="body">
      <div v-if="showRejetModal" class="modal modal-open">
        <div class="modal-box max-w-md">
          <h3 class="font-bold text-lg mb-1">Rejeter la demande</h3>
          <p class="text-base-content/70 text-sm mb-4">
            Vous pouvez laisser un commentaire pour expliquer le motif du rejet. L'utilisateur pourra le consulter depuis son profil.
          </p>
          <textarea
            v-model="commentaireRejet"
            rows="4"
            class="textarea textarea-bordered w-full text-sm"
            placeholder="Motif du rejet (facultatif — ex : biographie trop courte…)"
          />
          <div class="modal-action">
            <button class="btn btn-ghost" @click="showRejetModal = false">Annuler</button>
            <button
              class="btn btn-error"
              :disabled="actionLoading"
              @click="confirmerRejet"
            >
              <span v-if="actionLoading" class="loading loading-spinner loading-sm" />
              Confirmer le rejet
            </button>
          </div>
        </div>
        <div class="modal-backdrop" @click="showRejetModal = false" />
      </div>
    </Teleport>
  </div>
</template>
