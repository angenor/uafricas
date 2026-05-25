<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'
import { STATUTS } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { projets, filtres, pagination, sort, loading, chargerListe, supprimer, allerPage, changerTri, reinitialiserPagination } = useAdminProjets()

const showDelete = ref(false)
const deleteTarget = ref<{ id: string; nom: string } | null>(null)
const deleteLoading = ref(false)

const etatProjetConfig: Record<string, { label: string; class: string }> = {
  soumis: { label: 'Soumis', class: 'badge-info' },
  en_revue: { label: 'En revue', class: 'badge-warning' },
  approuve: { label: 'Approuve', class: 'badge-success' },
  en_cours: { label: 'En cours', class: 'badge-primary' },
  termine: { label: 'Termine', class: 'badge-neutral' },
  suspendu: { label: 'Suspendu', class: 'badge-warning' },
  rejete: { label: 'Rejete', class: 'badge-error' },
}

const colonnes: TableColumn[] = [
  { key: 'titre', label: 'Titre', sortable: true },
  { key: 'etat', label: 'Etat', sortable: true, width: 'w-28', align: 'center' },
  { key: 'nom_organisation', label: 'Organisation', width: 'w-36' },
  { key: 'pays_nom', label: 'Territoire', width: 'w-32' },
  { key: 'cout_total', label: 'Budget', sortable: true, width: 'w-28', align: 'right',
    format: (v: number | null) => v ? `${v.toLocaleString('fr-FR')} XOF` : '-' },
  { key: 'duree_mois', label: 'Duree', width: 'w-24', align: 'center',
    format: (v: number | null) => v ? `${v} mois` : '-' },
  { key: 'created_at', label: 'Soumis le', sortable: true, width: 'w-28',
    format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]

const filterDefs: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Titre, description, organisation...' },
  { key: 'etat', label: 'Etat', type: 'select', placeholder: 'Tous', options: [
    { label: 'Soumis', value: 'soumis' },
    { label: 'En revue', value: 'en_revue' },
    { label: 'Approuve', value: 'approuve' },
    { label: 'En cours', value: 'en_cours' },
    { label: 'Termine', value: 'termine' },
    { label: 'Suspendu', value: 'suspendu' },
    { label: 'Rejete', value: 'rejete' },
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
  filtres.pays_id = ''
  filtres.organisation = ''
  reinitialiserPagination()
  chargerListe()
}

onMounted(() => chargerListe())
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerListe())
</script>

<template>
  <div>
    <AdminPageHeader titre="Projets" sous-titre="Gerer les projets a financer">
      <template #actions>
        <NuxtLink to="/admin/projets/create" class="btn btn-primary btn-sm">
          <font-awesome-icon icon="plus" class="mr-1" /> Nouveau projet
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
      :donnees="projets"
      :pagination="pagination"
      :tri-colonne="sort.column"
      :tri-direction="sort.direction"
      :loading="loading"
      @trier="changerTri"
      @aller-page="allerPage"
    >
      <template #cell-etat="{ value }">
        <span class="badge badge-sm" :class="etatProjetConfig[value]?.class || 'badge-neutral'">
          {{ etatProjetConfig[value]?.label || value }}
        </span>
      </template>
      <template #actions="{ item }">
        <div class="flex gap-1">
          <NuxtLink :to="`/admin/projets/${item.id}`" class="btn btn-ghost btn-xs">
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
