<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { programmations, filtres, pagination, sort, loading, chargerListe, supprimer, allerPage, changerTri, reinitialiserPagination } = useAdminProgrammations()

const showDelete = ref(false)
const deleteTarget = ref<{ id: string; nom: string } | null>(null)
const deleteLoading = ref(false)

const modeBadge = (mode: string | null) => {
  switch (mode) {
    case 'presentiel': return 'badge badge-info badge-sm'
    case 'en_ligne': return 'badge badge-success badge-sm'
    case 'hybride': return 'badge badge-warning badge-sm'
    default: return 'badge badge-neutral badge-sm'
  }
}

const modeLabel = (mode: string | null) => {
  switch (mode) {
    case 'presentiel': return 'Presentiel'
    case 'en_ligne': return 'En ligne'
    case 'hybride': return 'Hybride'
    default: return mode || '-'
  }
}

const colonnes: TableColumn[] = [
  { key: 'titre', label: 'Titre', sortable: true },
  { key: 'centre_nom', label: 'Centre culturel', width: 'w-40' },
  { key: 'mode', label: 'Mode', width: 'w-28', align: 'center' },
  { key: 'date_heure_debut', label: 'Date debut', sortable: true, width: 'w-32',
    format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
  { key: 'nombre_places', label: 'Places', width: 'w-20', align: 'center',
    format: (v: number | null) => v !== null && v !== undefined ? String(v) : '-' },
  { key: 'created_at', label: 'Creation', sortable: true, width: 'w-28',
    format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]

const filterDefs: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Titre, lieu...' },
  { key: 'centre_culturel_id', label: 'Centre culturel', type: 'text', placeholder: 'ID centre culturel' },
  { key: 'mode', label: 'Mode', type: 'select', placeholder: 'Tous', options: [
    { label: 'Presentiel', value: 'presentiel' },
    { label: 'En ligne', value: 'en_ligne' },
    { label: 'Hybride', value: 'hybride' },
  ]},
]

const confirmerSuppression = (item: any) => {
  deleteTarget.value = { id: item.id, nom: item.titre }
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

const reinitialiser = () => {
  filtres.recherche = ''
  filtres.centre_culturel_id = ''
  filtres.mode = ''
  reinitialiserPagination()
  chargerListe()
}

onMounted(() => chargerListe())
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerListe())
</script>

<template>
  <div>
    <AdminPageHeader titre="Programmations" sous-titre="Gerer les programmations des centres culturels">
      <template #actions>
        <NuxtLink to="/admin/programmations/create" class="btn btn-primary btn-sm">
          <font-awesome-icon icon="plus" class="mr-1" /> Nouvelle programmation
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
      :donnees="programmations"
      :pagination="pagination"
      :tri-colonne="sort.column"
      :tri-direction="sort.direction"
      :loading="loading"
      @trier="changerTri"
      @aller-page="allerPage"
    >
      <template #cell-mode="{ value }">
        <span :class="modeBadge(value)">
          {{ modeLabel(value) }}
        </span>
      </template>
      <template #actions="{ item }">
        <div class="flex gap-1">
          <NuxtLink :to="`/admin/programmations/${item.id}`" class="btn btn-ghost btn-xs">
            <font-awesome-icon icon="pen-to-square" />
          </NuxtLink>
          <button class="btn btn-ghost btn-xs text-error" @click="confirmerSuppression(item)">
            <font-awesome-icon icon="trash" />
          </button>
        </div>
      </template>
    </AdminDataTable>

    <AdminDeleteConfirm
      v-model:visible="showDelete"
      :titre="deleteTarget?.nom"
      :loading="deleteLoading"
      @confirmer="executerSuppression"
    />
  </div>
</template>
