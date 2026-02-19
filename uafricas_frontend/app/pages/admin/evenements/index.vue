<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { evenements, filtres, pagination, sort, loading, chargerListe, changerEtat, supprimer, allerPage, changerTri, reinitialiserPagination } = useAdminEvenements()

const showDelete = ref(false)
const deleteTarget = ref<{ id: string; titre: string } | null>(null)
const deleteLoading = ref(false)

const showEtat = ref(false)
const etatTarget = ref<{ id: string; titre: string; etat: string } | null>(null)
const nouvelEtat = ref('')
const etatLoading = ref(false)

const colonnes: TableColumn[] = [
  { key: 'titre', label: 'Titre', sortable: true },
  { key: 'format', label: 'Format', width: 'w-28', align: 'center' },
  { key: 'etat', label: 'Etat', sortable: true, width: 'w-28', align: 'center' },
  { key: 'date_heure_debut', label: 'Debut', sortable: true, width: 'w-32',
    format: (v: string) => v ? new Date(v).toLocaleDateString('fr-FR', { day: '2-digit', month: '2-digit', year: 'numeric', hour: '2-digit', minute: '2-digit' }) : '-' },
  { key: 'nombre_places', label: 'Places', width: 'w-20', align: 'center' },
  { key: 'pays_nom', label: 'Pays', width: 'w-28' },
  { key: 'created_at', label: 'Creation', sortable: true, width: 'w-28',
    format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]

const filterDefs: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Titre, description...' },
  { key: 'format', label: 'Format', type: 'select', placeholder: 'Tous', options: [
    { label: 'Presentiel', value: 'presentiel' },
    { label: 'En ligne', value: 'en_ligne' },
    { label: 'Hybride', value: 'hybride' },
  ]},
  { key: 'etat', label: 'Etat', type: 'select', placeholder: 'Tous', options: [
    { label: 'Brouillon', value: 'brouillon' },
    { label: 'Publie', value: 'publie' },
    { label: 'Suspendu', value: 'suspendu' },
    { label: 'Annule', value: 'annule' },
  ]},
]

const etatBadge = (etat: string) => {
  const map: Record<string, string> = {
    brouillon: 'badge-warning',
    publie: 'badge-success',
    suspendu: 'badge-error',
    annule: 'badge-ghost',
  }
  return map[etat] || 'badge-neutral'
}

const etatLabel = (etat: string) => {
  const map: Record<string, string> = {
    brouillon: 'Brouillon',
    publie: 'Publie',
    suspendu: 'Suspendu',
    annule: 'Annule',
  }
  return map[etat] || etat
}

const formatLabel = (format: string) => {
  const map: Record<string, string> = {
    presentiel: 'Presentiel',
    en_ligne: 'En ligne',
    hybride: 'Hybride',
  }
  return map[format] || format
}

const formatBadge = (format: string) => {
  const map: Record<string, string> = {
    presentiel: 'badge-info',
    en_ligne: 'badge-accent',
    hybride: 'badge-secondary',
  }
  return map[format] || 'badge-neutral'
}

const confirmerSuppression = (item: any) => {
  deleteTarget.value = { id: item.id, titre: item.titre }
  showDelete.value = true
}

const executerSuppression = async () => {
  if (!deleteTarget.value) return
  deleteLoading.value = true
  try {
    await supprimer(deleteTarget.value.id)
    showDelete.value = false
    await chargerListe()
  } catch {} finally { deleteLoading.value = false }
}

const ouvrirChangerEtat = (item: any) => {
  etatTarget.value = { id: item.id, titre: item.titre, etat: item.etat }
  nouvelEtat.value = item.etat
  showEtat.value = true
}

const executerChangerEtat = async () => {
  if (!etatTarget.value || nouvelEtat.value === etatTarget.value.etat) return
  etatLoading.value = true
  try {
    await changerEtat(etatTarget.value.id, nouvelEtat.value)
    showEtat.value = false
    await chargerListe()
  } catch {} finally { etatLoading.value = false }
}

const reinitialiser = () => {
  filtres.recherche = ''
  filtres.format = ''
  filtres.etat = ''
  filtres.pays_id = ''
  filtres.date_debut = ''
  filtres.date_fin = ''
  reinitialiserPagination()
  chargerListe()
}

onMounted(() => chargerListe())
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerListe())
</script>

<template>
  <div>
    <AdminPageHeader titre="Evenements" sous-titre="Gerer les evenements de la plateforme">
      <template #actions>
        <NuxtLink to="/admin/evenements/create" class="btn btn-primary btn-sm">
          <font-awesome-icon icon="plus" class="mr-1" /> Nouvel evenement
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <AdminFilters
      :filtres="filterDefs"
      v-model="filtres"
      @rechercher="() => { reinitialiserPagination(); chargerListe() }"
      @reinitialiser="reinitialiser"
    />

    <AdminDataTable
      :colonnes="colonnes"
      :donnees="evenements"
      :pagination="pagination"
      :tri-colonne="sort.column"
      :tri-direction="sort.direction"
      :loading="loading"
      @trier="changerTri"
      @aller-page="allerPage"
    >
      <template #cell-format="{ value }">
        <span :class="['badge badge-sm badge-outline', formatBadge(value)]">
          {{ formatLabel(value) }}
        </span>
      </template>

      <template #cell-etat="{ value }">
        <span :class="['badge badge-sm', etatBadge(value)]">
          {{ etatLabel(value) }}
        </span>
      </template>

      <template #cell-nombre_places="{ value }">
        {{ value ?? '-' }}
      </template>

      <template #cell-pays_nom="{ value }">
        <span v-if="value" class="text-sm">{{ value }}</span>
        <span v-else class="text-base-content/30">-</span>
      </template>

      <template #actions="{ item }">
        <div class="flex gap-1">
          <button class="btn btn-ghost btn-xs" title="Changer etat" @click="ouvrirChangerEtat(item)">
            <font-awesome-icon icon="arrows-rotate" />
          </button>
          <NuxtLink :to="`/admin/evenements/${item.id}`" class="btn btn-ghost btn-xs">
            <font-awesome-icon icon="pen-to-square" />
          </NuxtLink>
          <button class="btn btn-ghost btn-xs text-error" @click="confirmerSuppression(item)">
            <font-awesome-icon icon="trash" />
          </button>
        </div>
      </template>
    </AdminDataTable>

    <!-- Modal suppression -->
    <AdminDeleteConfirm
      v-model:visible="showDelete"
      :titre="deleteTarget?.titre"
      :loading="deleteLoading"
      @confirmer="executerSuppression"
    />

    <!-- Modal changement etat -->
    <div v-if="showEtat" class="modal modal-open">
      <div class="modal-box">
        <h3 class="font-bold text-lg mb-4">Changer l'etat de l'evenement</h3>
        <p class="mb-2 text-sm text-base-content/70">Evenement : {{ etatTarget?.titre }}</p>
        <div class="form-control">
          <label class="label"><span class="label-text">Nouvel etat</span></label>
          <select v-model="nouvelEtat" class="select select-bordered">
            <option value="brouillon">Brouillon</option>
            <option value="publie">Publie</option>
            <option value="suspendu">Suspendu</option>
            <option value="annule">Annule</option>
          </select>
        </div>
        <div class="modal-action">
          <button class="btn btn-ghost" @click="showEtat = false">Annuler</button>
          <button class="btn btn-primary" :class="{ loading: etatLoading }" :disabled="etatLoading || nouvelEtat === etatTarget?.etat" @click="executerChangerEtat">
            Confirmer
          </button>
        </div>
      </div>
      <div class="modal-backdrop" @click="showEtat = false" />
    </div>
  </div>
</template>
