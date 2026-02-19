<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'
definePageMeta({ layout: 'admin', middleware: ['admin'] })
const { domaines, filtres, pagination, sort, loading, chargerListe, supprimer, allerPage, changerTri, reinitialiserPagination } = useAdminDomaines()
const showDelete = ref(false)
const deleteTarget = ref<{ id: string; nom: string } | null>(null)
const deleteLoading = ref(false)
const colonnes: TableColumn[] = [
  { key: 'nom', label: 'Nom', sortable: true },
  { key: 'slug', label: 'Slug', sortable: true, width: 'w-40' },
  { key: 'icone', label: 'Icone', width: 'w-24', align: 'center' },
  { key: 'actif', label: 'Actif', width: 'w-20', align: 'center' },
  { key: 'created_at', label: 'Creation', sortable: true, width: 'w-28', format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]
const filterDefs: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Nom ou slug...' },
]
const confirmerSuppression = (item: any) => { deleteTarget.value = { id: item.id, nom: item.nom }; showDelete.value = true }
const executerSuppression = async () => {
  if (!deleteTarget.value) return
  deleteLoading.value = true
  try { await supprimer(deleteTarget.value.id); showDelete.value = false; await chargerListe() } catch {} finally { deleteLoading.value = false }
}
const reinitialiser = () => { filtres.recherche = ''; reinitialiserPagination(); chargerListe() }
onMounted(() => chargerListe())
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerListe())
</script>
<template>
  <div>
    <AdminPageHeader titre="Domaines & Secteurs" sous-titre="Gerer les domaines et secteurs d'activite">
      <template #actions>
        <NuxtLink to="/admin/domaines/create" class="btn btn-primary btn-sm"><font-awesome-icon icon="plus" class="mr-1" /> Nouveau domaine</NuxtLink>
      </template>
    </AdminPageHeader>
    <AdminFilters :filtres="filterDefs" v-model="filtres" @rechercher="() => { reinitialiserPagination(); chargerListe() }" @reinitialiser="reinitialiser" />
    <AdminDataTable :colonnes="colonnes" :donnees="domaines" :pagination="pagination" :tri-colonne="sort.column" :tri-direction="sort.direction" :loading="loading" @trier="changerTri" @aller-page="allerPage">
      <template #cell-actif="{ value }"><span :class="value ? 'badge badge-success badge-sm' : 'badge badge-neutral badge-sm'">{{ value ? 'Oui' : 'Non' }}</span></template>
      <template #cell-icone="{ value }"><span v-if="value" class="text-lg">{{ value }}</span><span v-else class="text-base-content/30">—</span></template>
      <template #actions="{ item }">
        <div class="flex gap-1">
          <NuxtLink :to="`/admin/domaines/${item.id}`" class="btn btn-ghost btn-xs"><font-awesome-icon icon="pen-to-square" /></NuxtLink>
          <button class="btn btn-ghost btn-xs text-error" @click="confirmerSuppression(item)"><font-awesome-icon icon="trash" /></button>
        </div>
      </template>
    </AdminDataTable>
    <AdminDeleteConfirm v-model:visible="showDelete" :titre="deleteTarget?.nom" :loading="deleteLoading" @confirmer="executerSuppression" />
  </div>
</template>
