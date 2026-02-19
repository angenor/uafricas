<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'
definePageMeta({ layout: 'admin', middleware: ['admin'] })
const { specialites, filtres, pagination, sort, loading, chargerListe, supprimer, allerPage, changerTri, reinitialiserPagination } = useAdminSpecialites()
const showDelete = ref(false)
const deleteTarget = ref<{ id: string; nom: string } | null>(null)
const deleteLoading = ref(false)
const colonnes: TableColumn[] = [
  { key: 'nom', label: 'Nom', sortable: true },
  { key: 'slug', label: 'Slug', sortable: true, width: 'w-48' },
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
    <AdminPageHeader titre="Specialites Biblio" sous-titre="Gerer les specialites de la bibliotheque humaine">
      <template #actions><NuxtLink to="/admin/specialites/create" class="btn btn-primary btn-sm"><font-awesome-icon icon="plus" class="mr-1" /> Nouvelle specialite</NuxtLink></template>
    </AdminPageHeader>
    <AdminFilters :filtres="filterDefs" v-model="filtres" @rechercher="() => { reinitialiserPagination(); chargerListe() }" @reinitialiser="reinitialiser" />
    <AdminDataTable :colonnes="colonnes" :donnees="specialites" :pagination="pagination" :tri-colonne="sort.column" :tri-direction="sort.direction" :loading="loading" @trier="changerTri" @aller-page="allerPage">
      <template #actions="{ item }">
        <div class="flex gap-1">
          <NuxtLink :to="`/admin/specialites/${item.id}`" class="btn btn-ghost btn-xs"><font-awesome-icon icon="pen-to-square" /></NuxtLink>
          <button class="btn btn-ghost btn-xs text-error" @click="confirmerSuppression(item)"><font-awesome-icon icon="trash" /></button>
        </div>
      </template>
    </AdminDataTable>
    <AdminDeleteConfirm v-model:visible="showDelete" :titre="deleteTarget?.nom" :loading="deleteLoading" @confirmer="executerSuppression" />
  </div>
</template>
