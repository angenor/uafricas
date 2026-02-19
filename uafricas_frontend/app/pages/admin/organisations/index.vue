<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { organisations, filtres, pagination, sort, loading, chargerListe, supprimer, allerPage, changerTri, reinitialiserPagination } = useAdminOrganisations()

const showDelete = ref(false)
const deleteTarget = ref<{ id: string; nom: string } | null>(null)
const deleteLoading = ref(false)

const colonnes: TableColumn[] = [
  { key: 'denomination', label: 'Denomination', sortable: true },
  { key: 'type_organisation', label: 'Type', width: 'w-32' },
  { key: 'pays_nom', label: 'Pays', width: 'w-28' },
  { key: 'etat', label: 'Etat', sortable: true, width: 'w-28' },
  { key: 'nombre_membres', label: 'Membres', width: 'w-24', align: 'center' },
  { key: 'created_at', label: 'Creation', sortable: true, width: 'w-28', format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]

const filterDefs: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Denomination...' },
  { key: 'etat', label: 'Etat', type: 'select', placeholder: 'Tous', options: [
    { label: 'Actif', value: 'actif' },
    { label: 'En attente', value: 'en_attente' },
    { label: 'Suspendu', value: 'suspendu' },
  ] },
]

const confirmerSuppression = (item: any) => {
  deleteTarget.value = { id: item.id, nom: item.denomination }
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

const reinitialiser = () => {
  filtres.recherche = ''
  filtres.type_organisation = ''
  filtres.pays = ''
  filtres.etat = ''
  reinitialiserPagination()
  chargerListe()
}

onMounted(() => chargerListe())
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerListe())
</script>

<template>
  <div>
    <AdminPageHeader titre="Organisations" sous-titre="Gerer les organisations partenaires">
      <template #actions>
        <NuxtLink to="/admin/organisations/create" class="btn btn-primary btn-sm">
          <font-awesome-icon icon="plus" class="mr-1" /> Nouvelle organisation
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <AdminFilters :filtres="filterDefs" v-model="filtres" @rechercher="() => { reinitialiserPagination(); chargerListe() }" @reinitialiser="reinitialiser" />

    <AdminDataTable :colonnes="colonnes" :donnees="organisations" :pagination="pagination" :tri-colonne="sort.column" :tri-direction="sort.direction" :loading="loading" @trier="changerTri" @aller-page="allerPage; chargerListe()">
      <template #cell-etat="{ value }"><AdminStatusBadge :statut="value" /></template>
      <template #cell-denomination="{ item }">
        <span class="font-semibold">{{ item.denomination }}</span>
      </template>
      <template #actions="{ item }">
        <div class="flex gap-1">
          <NuxtLink :to="`/admin/organisations/${item.id}`" class="btn btn-ghost btn-xs"><font-awesome-icon icon="pen-to-square" /></NuxtLink>
          <button class="btn btn-ghost btn-xs text-error" @click="confirmerSuppression(item)"><font-awesome-icon icon="trash" /></button>
        </div>
      </template>
    </AdminDataTable>

    <AdminDeleteConfirm v-model:visible="showDelete" :titre="deleteTarget?.nom" :loading="deleteLoading" @confirmer="executerSuppression" />
  </div>
</template>
