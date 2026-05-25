<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { pays, filtres, pagination, sort, loading, chargerListe, supprimer, allerPage, changerTri, reinitialiserPagination } = useAdminPays()

const showDelete = ref(false)
const deleteTarget = ref<{ id: string; nom: string } | null>(null)
const deleteLoading = ref(false)

const colonnes: TableColumn[] = [
  { key: 'nom', label: 'Nom', sortable: true },
  { key: 'code_iso2', label: 'ISO2', width: 'w-20', align: 'center' },
  { key: 'capitale', label: 'Capitale', width: 'w-36' },
  { key: 'continent', label: 'Continent', sortable: true, width: 'w-32' },
  { key: 'actif', label: 'Actif', width: 'w-20', align: 'center' },
  { key: 'created_at', label: 'Creation', sortable: true, width: 'w-28',
    format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]

const filterDefs: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Nom, code ISO, capitale...' },
  { key: 'continent', label: 'Continent', type: 'select', placeholder: 'Tous', options: [
    { label: 'Afrique', value: 'Afrique' },
    { label: 'Europe', value: 'Europe' },
    { label: 'Amerique', value: 'Amerique' },
    { label: 'Asie', value: 'Asie' },
    { label: 'Oceanie', value: 'Oceanie' },
  ]},
]

const confirmerSuppression = (item: any) => {
  deleteTarget.value = { id: item.id, nom: item.nom }
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
  filtres.continent = ''
  reinitialiserPagination()
  chargerListe()
}

onMounted(() => chargerListe())
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerListe())
</script>

<template>
  <div>
    <AdminPageHeader titre="Territoires" sous-titre="Gerer les territoires de reference">
      <template #actions>
        <NuxtLink to="/admin/pays/create" class="btn btn-primary btn-sm">
          <font-awesome-icon icon="plus" class="mr-1" /> Nouveau territoire
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
      :donnees="pays"
      :pagination="pagination"
      :tri-colonne="sort.column"
      :tri-direction="sort.direction"
      :loading="loading"
      @trier="changerTri"
      @aller-page="allerPage"
    >
      <template #cell-actif="{ value }">
        <span :class="value ? 'badge badge-success badge-sm' : 'badge badge-neutral badge-sm'">
          {{ value ? 'Oui' : 'Non' }}
        </span>
      </template>
      <template #actions="{ item }">
        <div class="flex gap-1">
          <NuxtLink :to="`/admin/pays/${item.id}`" class="btn btn-ghost btn-xs">
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
