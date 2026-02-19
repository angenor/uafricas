<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'
definePageMeta({ layout: 'admin', middleware: ['admin'] })
const { tags, filtres, pagination, sort, loading, chargerListe, supprimer, allerPage, changerTri, reinitialiserPagination } = useAdminTags()
const showDelete = ref(false)
const deleteTarget = ref<{ id: string; nom: string } | null>(null)
const deleteLoading = ref(false)
const colonnes: TableColumn[] = [
  { key: 'nom', label: 'Nom', sortable: true },
  { key: 'slug', label: 'Slug', sortable: true, width: 'w-48' },
  { key: 'created_at', label: 'Creation', sortable: true, width: 'w-28', format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]
const filterDefs: FilterDefinition[] = [{ key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Nom ou slug...' }]
const confirmerSuppression = (item: any) => { deleteTarget.value = { id: item.id, nom: item.nom }; showDelete.value = true }
const executerSuppression = async () => { if (!deleteTarget.value) return; deleteLoading.value = true; try { await supprimer(deleteTarget.value.id); showDelete.value = false; await chargerListe() } catch {} finally { deleteLoading.value = false } }
const reinitialiser = () => { filtres.recherche = ''; reinitialiserPagination(); chargerListe() }
onMounted(() => chargerListe())
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerListe())
</script>
<template>
  <div>
    <AdminPageHeader titre="Tags" sous-titre="Gerer les tags de reference">
      <template #actions><NuxtLink to="/admin/tags/create" class="btn btn-primary btn-sm"><font-awesome-icon icon="plus" class="mr-1" /> Nouveau tag</NuxtLink></template>
    </AdminPageHeader>
    <AdminFilters :filtres="filterDefs" v-model="filtres" @rechercher="() => { reinitialiserPagination(); chargerListe() }" @reinitialiser="reinitialiser" />
    <AdminDataTable :colonnes="colonnes" :donnees="tags" :pagination="pagination" :tri-colonne="sort.column" :tri-direction="sort.direction" :loading="loading" @trier="changerTri" @aller-page="allerPage">
      <template #cell-nom="{ item }"><span class="badge badge-outline">{{ item.nom }}</span></template>
      <template #actions="{ item }">
        <div class="flex gap-1">
          <NuxtLink :to="`/admin/tags/${item.id}`" class="btn btn-ghost btn-xs"><font-awesome-icon icon="pen-to-square" /></NuxtLink>
          <button class="btn btn-ghost btn-xs text-error" @click="confirmerSuppression(item)"><font-awesome-icon icon="trash" /></button>
        </div>
      </template>
    </AdminDataTable>
    <AdminDeleteConfirm v-model:visible="showDelete" :titre="deleteTarget?.nom" :loading="deleteLoading" @confirmer="executerSuppression" />
  </div>
</template>
