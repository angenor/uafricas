<script setup lang="ts">
import type { AdminContributionDetail } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const {
  contributions, contributionDetail, filtres,
  pagination, sort, loading, error,
  chargerListe, chargerDetail, moderer,
  allerPage, changerTri, reinitialiserPagination,
} = useAdminContributions()

const showDetail = ref(false)
const detailLoading = ref(false)
const moderationLoading = ref(false)
const noteModeration = ref('')
const filtreEtat = ref('')

// ── Badges helpers ──────────────────────────────────────────

const typeBadge = (type: string) => {
  const map: Record<string, string> = {
    modification: 'badge-info',
    ajout: 'badge-success',
    suppression: 'badge-error',
  }
  return map[type] || 'badge-neutral'
}

const typeLabel = (type: string) => {
  const map: Record<string, string> = {
    modification: 'Modification',
    ajout: 'Ajout',
    suppression: 'Suppression',
  }
  return map[type] || type
}

const etatBadge = (etat: string) => {
  const map: Record<string, string> = {
    en_attente: 'badge-warning',
    approuvee: 'badge-success',
    rejetee: 'badge-error',
  }
  return map[etat] || 'badge-neutral'
}

const etatLabel = (etat: string) => {
  const map: Record<string, string> = {
    en_attente: 'En attente',
    approuvee: 'Approuvee',
    rejetee: 'Rejetee',
  }
  return map[etat] || etat
}

const formatDate = (d: string | null) => {
  if (!d) return '-'
  return new Date(d).toLocaleDateString('fr-FR', {
    day: '2-digit', month: '2-digit', year: 'numeric',
  })
}

// ── Filtre par etat ─────────────────────────────────────────

const appliquerFiltre = () => {
  filtres.etat = filtreEtat.value
  reinitialiserPagination()
  chargerListe()
}

const reinitialiserFiltres = () => {
  filtreEtat.value = ''
  filtres.etat = ''
  filtres.fiche_pays_id = ''
  filtres.cree_par = ''
  reinitialiserPagination()
  chargerListe()
}

// ── Detail modal ────────────────────────────────────────────

const ouvrirDetail = async (id: string) => {
  detailLoading.value = true
  noteModeration.value = ''
  showDetail.value = true
  try {
    await chargerDetail(id)
  } finally {
    detailLoading.value = false
  }
}

const fermerDetail = () => {
  showDetail.value = false
  contributionDetail.value = null
  noteModeration.value = ''
}

// ── Moderation ──────────────────────────────────────────────

const executerModeration = async (etat: 'approuvee' | 'rejetee') => {
  if (!contributionDetail.value) return
  moderationLoading.value = true
  try {
    await moderer(contributionDetail.value.id, etat, noteModeration.value || undefined)
    fermerDetail()
    await chargerListe()
  } catch {
    // erreur geree par le composable
  } finally {
    moderationLoading.value = false
  }
}

// ── Lifecycle ───────────────────────────────────────────────

onMounted(() => chargerListe())
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerListe())
</script>

<template>
  <div>
    <!-- Header -->
    <AdminPageHeader titre="Contributions" sous-titre="Moderer les contributions aux fiches pays">
      <template #actions>
        <NuxtLink to="/admin/profils-pays" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <!-- Filtre simple -->
    <div class="card bg-base-100 shadow-sm mb-4">
      <div class="card-body py-3">
        <div class="flex flex-wrap items-end gap-3">
          <div class="form-control w-48">
            <label class="label pb-1">
              <span class="label-text text-xs">Etat</span>
            </label>
            <select v-model="filtreEtat" class="select select-bordered select-sm">
              <option value="">Tous</option>
              <option value="en_attente">En attente</option>
              <option value="approuvee">Approuvee</option>
              <option value="rejetee">Rejetee</option>
            </select>
          </div>
          <button class="btn btn-primary btn-sm" @click="appliquerFiltre">
            Filtrer
          </button>
          <button class="btn btn-ghost btn-sm" @click="reinitialiserFiltres">
            Reinitialiser
          </button>
        </div>
      </div>
    </div>

    <!-- Erreur -->
    <div v-if="error" class="alert alert-error mb-4">
      <font-awesome-icon icon="circle-exclamation" />
      <span>{{ error }}</span>
    </div>

    <!-- Loading -->
    <div v-if="loading && contributions.length === 0" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <!-- Table -->
    <div v-else class="card bg-base-100 shadow-sm">
      <div class="card-body p-0">
        <div class="overflow-x-auto">
          <table class="table table-sm">
            <thead>
              <tr>
                <th>Pays</th>
                <th>Section</th>
                <th>Type</th>
                <th>Contributeur</th>
                <th class="text-center">Etat</th>
                <th>Date</th>
                <th class="text-center">Actions</th>
              </tr>
            </thead>
            <tbody>
              <tr v-if="contributions.length === 0">
                <td colspan="7" class="text-center py-8 text-base-content/50">
                  <font-awesome-icon icon="inbox" class="text-2xl mb-2 block" />
                  Aucune contribution trouvee
                </td>
              </tr>
              <tr v-for="c in contributions" :key="c.id" class="hover">
                <td class="font-medium">{{ c.pays_nom }}</td>
                <td class="text-sm">{{ c.section }}</td>
                <td>
                  <span :class="['badge badge-sm badge-outline', typeBadge(c.type_contribution)]">
                    {{ typeLabel(c.type_contribution) }}
                  </span>
                </td>
                <td class="text-sm">{{ c.contributeur_nom || '-' }}</td>
                <td class="text-center">
                  <span :class="['badge badge-sm', etatBadge(c.etat)]">
                    {{ etatLabel(c.etat) }}
                  </span>
                </td>
                <td class="text-sm">{{ formatDate(c.created_at) }}</td>
                <td class="text-center">
                  <button class="btn btn-ghost btn-xs" title="Voir le detail" @click="ouvrirDetail(c.id)">
                    <font-awesome-icon icon="eye" />
                  </button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        <!-- Pagination -->
        <div v-if="pagination.total > pagination.parPage" class="flex justify-center py-3 border-t border-base-200">
          <div class="join">
            <button
              class="join-item btn btn-sm"
              :disabled="pagination.page <= 1"
              @click="allerPage(pagination.page - 1)"
            >
              &laquo;
            </button>
            <button
              v-for="p in Math.ceil(pagination.total / pagination.parPage)"
              :key="p"
              :class="['join-item btn btn-sm', { 'btn-active': p === pagination.page }]"
              @click="allerPage(p)"
            >
              {{ p }}
            </button>
            <button
              class="join-item btn btn-sm"
              :disabled="pagination.page >= Math.ceil(pagination.total / pagination.parPage)"
              @click="allerPage(pagination.page + 1)"
            >
              &raquo;
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Modal detail -->
    <div v-if="showDetail" class="modal modal-open">
      <div class="modal-box max-w-4xl">
        <!-- Chargement -->
        <div v-if="detailLoading" class="flex justify-center py-8">
          <span class="loading loading-spinner loading-lg" />
        </div>

        <template v-else-if="contributionDetail">
          <!-- En-tete modal -->
          <h3 class="font-bold text-lg mb-1">Detail de la contribution</h3>
          <p class="text-sm text-base-content/70 mb-4">
            <font-awesome-icon icon="clipboard-list" class="mr-1" />
            {{ contributionDetail.pays_nom }} — {{ contributionDetail.section }}
          </p>

          <!-- Infos principales -->
          <div class="grid grid-cols-2 sm:grid-cols-4 gap-3 mb-4">
            <div class="bg-base-200 rounded-lg p-3">
              <div class="text-xs text-base-content/60">Type</div>
              <span :class="['badge badge-sm badge-outline mt-1', typeBadge(contributionDetail.type_contribution)]">
                {{ typeLabel(contributionDetail.type_contribution) }}
              </span>
            </div>
            <div class="bg-base-200 rounded-lg p-3">
              <div class="text-xs text-base-content/60">Etat</div>
              <span :class="['badge badge-sm mt-1', etatBadge(contributionDetail.etat)]">
                {{ etatLabel(contributionDetail.etat) }}
              </span>
            </div>
            <div class="bg-base-200 rounded-lg p-3">
              <div class="text-xs text-base-content/60">Contributeur</div>
              <div class="text-sm font-medium mt-1">{{ contributionDetail.contributeur_nom || '-' }}</div>
            </div>
            <div class="bg-base-200 rounded-lg p-3">
              <div class="text-xs text-base-content/60">Date</div>
              <div class="text-sm mt-1">{{ formatDate(contributionDetail.created_at) }}</div>
            </div>
          </div>

          <!-- Diff view -->
          <div class="grid grid-cols-1 md:grid-cols-2 gap-3 mb-4">
            <div class="rounded-lg border border-error/20 bg-error/5 p-3">
              <div class="text-xs font-bold text-error mb-2">Ancienne valeur</div>
              <pre
                v-if="contributionDetail.ancienne_valeur"
                class="text-sm whitespace-pre-wrap break-words bg-error/10 rounded p-3 max-h-64 overflow-y-auto"
              >{{ contributionDetail.ancienne_valeur }}</pre>
              <p v-else class="text-sm text-base-content/40 italic">Aucune valeur</p>
            </div>
            <div class="rounded-lg border border-success/20 bg-success/5 p-3">
              <div class="text-xs font-bold text-success mb-2">Nouvelle valeur</div>
              <pre
                v-if="contributionDetail.nouvelle_valeur"
                class="text-sm whitespace-pre-wrap break-words bg-success/10 rounded p-3 max-h-64 overflow-y-auto"
              >{{ contributionDetail.nouvelle_valeur }}</pre>
              <p v-else class="text-sm text-base-content/40 italic">Aucune valeur</p>
            </div>
          </div>

          <!-- Justification -->
          <div v-if="contributionDetail.justification" class="mb-4">
            <div class="text-xs font-bold text-base-content/70 mb-1">Justification</div>
            <div class="bg-base-200 rounded-lg p-3 text-sm whitespace-pre-wrap">
              {{ contributionDetail.justification }}
            </div>
          </div>

          <!-- Si deja traitee -->
          <div v-if="contributionDetail.etat !== 'en_attente'" class="mb-4 border-t border-base-200 pt-4">
            <div class="text-xs font-bold text-base-content/70 mb-2">Moderation</div>
            <div class="grid grid-cols-1 sm:grid-cols-3 gap-3 text-sm">
              <div>
                <span class="text-base-content/60">Traitee par : </span>
                <span class="font-medium">{{ contributionDetail.traite_par_nom || '-' }}</span>
              </div>
              <div>
                <span class="text-base-content/60">Date : </span>
                <span>{{ formatDate(contributionDetail.traite_at) }}</span>
              </div>
              <div>
                <span class="text-base-content/60">Decision : </span>
                <span :class="['badge badge-sm', etatBadge(contributionDetail.etat)]">
                  {{ etatLabel(contributionDetail.etat) }}
                </span>
              </div>
            </div>
            <div v-if="contributionDetail.note_moderation" class="mt-2">
              <span class="text-xs text-base-content/60">Note de moderation :</span>
              <div class="bg-base-200 rounded-lg p-3 text-sm mt-1 whitespace-pre-wrap">
                {{ contributionDetail.note_moderation }}
              </div>
            </div>
          </div>

          <!-- Actions de moderation (si en attente) -->
          <div v-if="contributionDetail.etat === 'en_attente'" class="border-t border-base-200 pt-4">
            <div class="form-control mb-3">
              <label class="label pb-1">
                <span class="label-text text-sm">Note de moderation (optionnelle)</span>
              </label>
              <textarea
                v-model="noteModeration"
                class="textarea textarea-bordered h-20"
                placeholder="Raison de l'approbation ou du rejet..."
              />
            </div>
            <div class="flex justify-end gap-2">
              <button
                class="btn btn-error btn-sm"
                :class="{ loading: moderationLoading }"
                :disabled="moderationLoading"
                @click="executerModeration('rejetee')"
              >
                <font-awesome-icon icon="xmark" class="mr-1" /> Rejeter
              </button>
              <button
                class="btn btn-success btn-sm"
                :class="{ loading: moderationLoading }"
                :disabled="moderationLoading"
                @click="executerModeration('approuvee')"
              >
                <font-awesome-icon icon="check" class="mr-1" /> Approuver
              </button>
            </div>
          </div>
        </template>

        <!-- Footer modal -->
        <div class="modal-action">
          <button class="btn btn-ghost btn-sm" @click="fermerDetail">Fermer</button>
        </div>
      </div>
      <div class="modal-backdrop" @click="fermerDetail" />
    </div>
  </div>
</template>
