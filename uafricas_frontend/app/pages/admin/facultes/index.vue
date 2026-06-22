<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { facultes, filtres, pagination, sort, loading, chargerListe, supprimer, allerPage, changerTri, reinitialiserPagination } = useAdminFacultes()

const showDelete = ref(false)
const deleteTarget = ref<{ id: string; titre: string } | null>(null)
const deleteLoading = ref(false)

const colonnes: TableColumn[] = [
  { key: 'titre', label: 'Faculté', sortable: true },
  { key: 'acronyme', label: 'Acronyme', sortable: true, width: 'w-28' },
  { key: 'ecole_nom', label: 'École partenaire', width: 'w-48' },
  { key: 'pays_nom', label: 'Territoire', width: 'w-32' },
  { key: 'statut', label: 'Statut', width: 'w-24', align: 'center' },
  { key: 'accepte_nouveaux_inscrits', label: 'Inscriptions', width: 'w-28', align: 'center' },
  { key: 'nombre_inscrits_total', label: 'Inscrits', width: 'w-24', align: 'center' },
  { key: 'created_at', label: 'Création', sortable: true, width: 'w-28',
    format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]

const filterDefs: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Titre, acronyme...' },
  { key: 'statut', label: 'Statut', type: 'select', options: [
    { value: '', label: 'Tous' },
    { value: 'active', label: 'Active' },
    { value: 'inactive', label: 'Inactive' },
  ] },
]

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

const reinitialiser = () => {
  filtres.recherche = ''
  filtres.statut = ''
  reinitialiserPagination()
  chargerListe()
}

onMounted(() => chargerListe())
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerListe())
</script>

<template>
  <div>
    <AdminPageHeader titre="Facultés" sous-titre="Facultés partenaires de l'INUDA">
      <template #actions>
        <NuxtLink to="/admin/facultes/create" class="btn btn-primary btn-sm">
          <font-awesome-icon icon="plus" class="mr-1" /> Nouvelle faculté
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
      :donnees="facultes"
      :pagination="pagination"
      :tri-colonne="sort.column"
      :tri-direction="sort.direction"
      :loading="loading"
      @trier="changerTri"
      @aller-page="allerPage"
    >
      <template #cell-statut="{ value }">
        <span :class="value === 'active' ? 'badge badge-success badge-sm' : 'badge badge-neutral badge-sm'">
          {{ value === 'active' ? 'Active' : 'Inactive' }}
        </span>
      </template>
      <template #cell-accepte_nouveaux_inscrits="{ value }">
        <span :class="value ? 'badge badge-info badge-sm' : 'badge badge-ghost badge-sm'">
          {{ value ? 'Ouvertes' : 'Fermées' }}
        </span>
      </template>
      <template #actions="{ item }">
        <div class="flex gap-1">
          <NuxtLink :to="`/admin/facultes/${item.id}`" class="btn btn-ghost btn-xs">
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
      :titre="deleteTarget?.titre"
      :loading="deleteLoading"
      @confirmer="executerSuppression"
    />
  </div>
</template>
