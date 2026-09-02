<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'
import { STATUTS } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { africantives, filtres, pagination, sort, loading, chargerListe, supprimer, allerPage, changerTri, reinitialiserPagination } = useAdminAfricantives()

const showDelete = ref(false)
const deleteTarget = ref<{ id: string; nom: string } | null>(null)
const deleteLoading = ref(false)

const colonnes: TableColumn[] = [
  { key: 'titre', label: 'Titre', sortable: true },
  { key: 'etat', label: 'État', sortable: true, width: 'w-28', align: 'center' },
  { key: 'domaine_nom', label: 'Domaine', width: 'w-36' },
  { key: 'pays_nom', label: 'Territoire', width: 'w-32' },
  { key: 'auteur_nom', label: 'Auteur', width: 'w-36',
    format: (_v: string, row: any) => `${row.auteur_prenom} ${row.auteur_nom}` },
  { key: 'created_at', label: 'Creation', sortable: true, width: 'w-28',
    format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]

const filterDefs: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Titre, description, ville...' },
  { key: 'etat', label: 'État', type: 'select', placeholder: 'Tous', options: [
    { label: 'Brouillon', value: 'brouillon' },
    { label: 'Publie', value: 'publie' },
    { label: 'Suspendu', value: 'suspendu' },
  ] },
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
  }
  catch {}
  finally { deleteLoading.value = false }
}

const reinitialiser = () => {
  filtres.recherche = ''
  filtres.etat = ''
  filtres.domaine_id = ''
  filtres.pays_id = ''
  reinitialiserPagination()
  chargerListe()
}

onMounted(() => chargerListe())
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerListe())
</script>

<template>
  <div>
    <AdminPageHeader titre="Africantives" sous-titre="Gerer les initiatives africaines">
      <template #actions>
        <NuxtLink to="/admin/africantives/create" class="btn btn-primary btn-sm">
          <font-awesome-icon icon="plus" class="mr-1" /> Nouvelle africantive
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
      :donnees="africantives"
      :pagination="pagination"
      :tri-colonne="sort.column"
      :tri-direction="sort.direction"
      :loading="loading"
      @trier="changerTri"
      @aller-page="allerPage"
    >
      <template #cell-etat="{ value }">
        <span
          class="badge badge-sm"
          :class="{
            'badge-success': value === 'publie',
            'badge-info': value === 'brouillon',
            'badge-warning': value === 'suspendu',
            'badge-neutral': value === 'supprime',
          }"
        >
          {{ STATUTS[value]?.label || value }}
        </span>
      </template>
      <template #actions="{ item }">
        <div class="flex gap-1">
          <NuxtLink :to="`/admin/africantives/${item.id}`" class="btn btn-ghost btn-xs">
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
