<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'
definePageMeta({ layout: 'admin', middleware: ['admin'] })
const { categories, filtres, pagination, sort, loading, chargerListe, supprimer, allerPage, changerTri, reinitialiserPagination } = useAdminCategories()
const showDelete = ref(false)
const deleteTarget = ref<{ id: string; nom: string } | null>(null)
const deleteLoading = ref(false)
const colonnes: TableColumn[] = [
  { key: 'nom', label: 'Nom', sortable: true },
  { key: 'contexte', label: 'Contexte', sortable: true, width: 'w-28' },
  { key: 'parent_id', label: 'Parent', width: 'w-20', align: 'center' },
  { key: 'ordre', label: 'Ordre', sortable: true, width: 'w-20', align: 'center' },
  { key: 'actif', label: 'Actif', width: 'w-20', align: 'center' },
  { key: 'created_at', label: 'Creation', sortable: true, width: 'w-28', format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]
const filterDefs: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Nom...' },
  { key: 'contexte', label: 'Contexte', type: 'select', placeholder: 'Tous', options: [
    { label: 'Annonce', value: 'annonce' }, { label: 'Livre', value: 'livre' }, { label: 'Radio', value: 'radio' },
    { label: 'Television', value: 'television' }, { label: 'Evenement', value: 'evenement' }, { label: 'Formation', value: 'formation' }, { label: 'Projet', value: 'projet' },
  ]},
]
const confirmerSuppression = (item: any) => { deleteTarget.value = { id: item.id, nom: item.nom }; showDelete.value = true }
const executerSuppression = async () => {
  if (!deleteTarget.value) return
  deleteLoading.value = true
  try { await supprimer(deleteTarget.value.id); showDelete.value = false; await chargerListe() } catch {} finally { deleteLoading.value = false }
}
const reinitialiser = () => { filtres.recherche = ''; filtres.contexte = ''; filtres.parent_id = ''; reinitialiserPagination(); chargerListe() }
onMounted(() => chargerListe())
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerListe())
</script>
<template>
  <div>
    <AdminPageHeader titre="Categories" sous-titre="Gerer les categories hierarchiques">
      <template #actions><NuxtLink to="/admin/categories/create" class="btn btn-primary btn-sm"><font-awesome-icon icon="plus" class="mr-1" /> Nouvelle categorie</NuxtLink></template>
    </AdminPageHeader>
    <AdminFilters :filtres="filterDefs" v-model="filtres" @rechercher="() => { reinitialiserPagination(); chargerListe() }" @reinitialiser="reinitialiser" />
    <AdminDataTable :colonnes="colonnes" :donnees="categories" :pagination="pagination" :tri-colonne="sort.column" :tri-direction="sort.direction" :loading="loading" @trier="changerTri" @aller-page="allerPage">
      <template #cell-contexte="{ value }"><span v-if="value" class="badge badge-info badge-sm">{{ value }}</span><span v-else class="text-base-content/30">-</span></template>
      <template #cell-parent_id="{ value }"><span v-if="value" class="badge badge-outline badge-sm">Enfant</span><span v-else class="badge badge-primary badge-sm">Racine</span></template>
      <template #cell-actif="{ value }"><span :class="value ? 'badge badge-success badge-sm' : 'badge badge-neutral badge-sm'">{{ value ? 'Oui' : 'Non' }}</span></template>
      <template #actions="{ item }">
        <div class="flex gap-1">
          <NuxtLink :to="`/admin/categories/${item.id}`" class="btn btn-ghost btn-xs"><font-awesome-icon icon="pen-to-square" /></NuxtLink>
          <button class="btn btn-ghost btn-xs text-error" @click="confirmerSuppression(item)"><font-awesome-icon icon="trash" /></button>
        </div>
      </template>
    </AdminDataTable>
    <AdminDeleteConfirm v-model:visible="showDelete" :titre="deleteTarget?.nom" :loading="deleteLoading" @confirmer="executerSuppression" />
  </div>
</template>
