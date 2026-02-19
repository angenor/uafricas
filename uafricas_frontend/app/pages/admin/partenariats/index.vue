<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { partenariats, filtres, pagination, sort, loading, chargerListe, supprimer, allerPage, changerTri, reinitialiserPagination } = useAdminPartenariats()

const showDelete = ref(false)
const deleteTarget = ref<{ id: string; nom: string } | null>(null)
const deleteLoading = ref(false)

const colonnes: TableColumn[] = [
  { key: 'organisation_denomination', label: 'Organisation' },
  { key: 'type_partenariat', label: 'Type', width: 'w-32' },
  { key: 'date_debut', label: 'Debut', width: 'w-28', format: (v: string) => v ? new Date(v).toLocaleDateString('fr-FR') : '—' },
  { key: 'date_fin', label: 'Fin', width: 'w-28', format: (v: string) => v ? new Date(v).toLocaleDateString('fr-FR') : '—' },
  { key: 'actif', label: 'Actif', width: 'w-20', align: 'center' },
  { key: 'created_at', label: 'Creation', sortable: true, width: 'w-28', format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]

const filterDefs: FilterDefinition[] = [
  { key: 'type_partenariat', label: 'Type', type: 'select', placeholder: 'Tous', options: [
    { label: 'Sponsor', value: 'Sponsor' },
    { label: 'Contributeur', value: 'Contributeur' },
    { label: 'Associe', value: 'Associe' },
  ] },
]

const confirmerSuppression = (item: any) => {
  deleteTarget.value = { id: item.id, nom: item.organisation_denomination }
  showDelete.value = true
}

const executerSuppression = async () => {
  if (!deleteTarget.value) return
  deleteLoading.value = true
  try {
    await supprimer(deleteTarget.value.id)
    showDelete.value = false
    await chargerListe()
  }
  catch {}
  finally { deleteLoading.value = false }
}

onMounted(() => chargerListe())
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerListe())
</script>

<template>
  <div>
    <AdminPageHeader titre="Partenariats" sous-titre="Gerer les partenariats avec les organisations">
      <template #actions>
        <NuxtLink to="/admin/partenariats/create" class="btn btn-primary btn-sm">
          <font-awesome-icon icon="plus" class="mr-1" /> Nouveau partenariat
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <AdminFilters :filtres="filterDefs" v-model="filtres" @rechercher="() => { reinitialiserPagination(); chargerListe() }" @reinitialiser="() => { filtres.type_partenariat = ''; filtres.organisation_id = ''; filtres.actif = ''; reinitialiserPagination(); chargerListe() }" />

    <AdminDataTable :colonnes="colonnes" :donnees="partenariats" :pagination="pagination" :tri-colonne="sort.column" :tri-direction="sort.direction" :loading="loading" @trier="changerTri" @aller-page="allerPage; chargerListe()">
      <template #cell-actif="{ value }">
        <font-awesome-icon :icon="value ? 'circle-check' : 'circle-xmark'" :class="value ? 'text-success' : 'text-error'" />
      </template>
      <template #actions="{ item }">
        <div class="flex gap-1">
          <NuxtLink :to="`/admin/partenariats/${item.id}`" class="btn btn-ghost btn-xs"><font-awesome-icon icon="pen-to-square" /></NuxtLink>
          <button class="btn btn-ghost btn-xs text-error" @click="confirmerSuppression(item)"><font-awesome-icon icon="trash" /></button>
        </div>
      </template>
    </AdminDataTable>

    <AdminDeleteConfirm v-model:visible="showDelete" :titre="deleteTarget?.nom" :loading="deleteLoading" @confirmer="executerSuppression" />
  </div>
</template>
