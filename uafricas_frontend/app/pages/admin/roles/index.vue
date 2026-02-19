<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { roles, filtres, pagination, sort, loading, chargerListe, supprimer, allerPage, changerTri, reinitialiserPagination } = useAdminRoles()

const showDelete = ref(false)
const deleteTarget = ref<{ id: string; nom: string } | null>(null)
const deleteLoading = ref(false)

const colonnes: TableColumn[] = [
  { key: 'nom', label: 'Nom', sortable: true },
  { key: 'slug', label: 'Slug', width: 'w-32' },
  { key: 'description', label: 'Description' },
  { key: 'est_systeme', label: 'Systeme', width: 'w-20', align: 'center' },
  { key: 'nombre_utilisateurs', label: 'Utilisateurs', width: 'w-28', align: 'center' },
  { key: 'created_at', label: 'Creation', sortable: true, width: 'w-28', format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]

const filterDefs: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Nom, slug...' },
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
  }
  catch {}
  finally { deleteLoading.value = false }
}

onMounted(() => chargerListe())
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerListe())
</script>

<template>
  <div>
    <AdminPageHeader titre="Roles & Permissions" sous-titre="Gerer les roles et les permissions de la plateforme">
      <template #actions>
        <NuxtLink to="/admin/roles/create" class="btn btn-primary btn-sm">
          <font-awesome-icon icon="plus" class="mr-1" /> Nouveau role
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <AdminFilters :filtres="filterDefs" v-model="filtres" @rechercher="() => { reinitialiserPagination(); chargerListe() }" @reinitialiser="() => { filtres.recherche = ''; reinitialiserPagination(); chargerListe() }" />

    <AdminDataTable :colonnes="colonnes" :donnees="roles" :pagination="pagination" :tri-colonne="sort.column" :tri-direction="sort.direction" :loading="loading" @trier="changerTri" @aller-page="allerPage; chargerListe()">
      <template #cell-nom="{ item }">
        <span class="font-semibold">{{ item.nom }}</span>
      </template>
      <template #cell-slug="{ value }">
        <code class="text-xs">{{ value }}</code>
      </template>
      <template #cell-est_systeme="{ value }">
        <font-awesome-icon v-if="value" icon="lock" class="text-warning" />
      </template>
      <template #actions="{ item }">
        <div class="flex gap-1">
          <NuxtLink :to="`/admin/roles/${item.id}`" class="btn btn-ghost btn-xs"><font-awesome-icon icon="pen-to-square" /></NuxtLink>
          <button
            v-if="!item.est_systeme"
            class="btn btn-ghost btn-xs text-error"
            @click="confirmerSuppression(item)"
          >
            <font-awesome-icon icon="trash" />
          </button>
        </div>
      </template>
    </AdminDataTable>

    <AdminDeleteConfirm v-model:visible="showDelete" :titre="deleteTarget?.nom" :loading="deleteLoading" @confirmer="executerSuppression" />
  </div>
</template>
