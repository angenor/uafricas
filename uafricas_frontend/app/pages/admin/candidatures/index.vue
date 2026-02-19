<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { candidatures, filtres, pagination, sort, loading, chargerListe, allerPage, changerTri, reinitialiserPagination } = useAdminCandidatures()

const colonnes: TableColumn[] = [
  { key: 'candidat_nom', label: 'Candidat', sortable: false },
  { key: 'candidat_email', label: 'Email' },
  { key: 'programme_titre', label: 'Programme' },
  { key: 'statut', label: 'Statut', sortable: true, width: 'w-24', align: 'center' },
  { key: 'created_at', label: 'Date', sortable: true, width: 'w-28',
    format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]

const filterDefs: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Nom, email, programme...' },
  { key: 'statut', label: 'Statut', type: 'select', placeholder: 'Tous', options: [
    { label: 'Soumise', value: 'soumise' },
    { label: 'En revue', value: 'en_revue' },
    { label: 'Acceptee', value: 'acceptee' },
    { label: 'Refusee', value: 'refusee' },
    { label: 'Retiree', value: 'retiree' },
  ]},
]

const statutBadge = (statut: string) => {
  const map: Record<string, string> = {
    soumise: 'badge-info',
    en_revue: 'badge-warning',
    acceptee: 'badge-success',
    refusee: 'badge-error',
    retiree: 'badge-neutral',
  }
  return map[statut] || 'badge-neutral'
}

const statutLabel = (statut: string) => {
  const map: Record<string, string> = {
    soumise: 'Soumise',
    en_revue: 'En revue',
    acceptee: 'Acceptee',
    refusee: 'Refusee',
    retiree: 'Retiree',
  }
  return map[statut] || statut
}

const reinitialiser = () => {
  filtres.recherche = ''
  filtres.statut = ''
  filtres.programme_id = ''
  reinitialiserPagination()
  chargerListe()
}

onMounted(() => chargerListe())
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerListe())
</script>

<template>
  <div>
    <AdminPageHeader titre="Candidatures" sous-titre="Gerer les candidatures aux programmes d'echange" />

    <AdminFilters
      :filtres="filterDefs"
      v-model="filtres"
      @rechercher="() => { reinitialiserPagination(); chargerListe() }"
      @reinitialiser="reinitialiser"
    />

    <AdminDataTable
      :colonnes="colonnes"
      :donnees="candidatures"
      :pagination="pagination"
      :tri-colonne="sort.column"
      :tri-direction="sort.direction"
      :loading="loading"
      @trier="changerTri"
      @aller-page="allerPage"
    >
      <template #cell-statut="{ value }">
        <span :class="['badge badge-sm', statutBadge(value)]">
          {{ statutLabel(value) }}
        </span>
      </template>
      <template #actions="{ item }">
        <NuxtLink :to="`/admin/candidatures/${item.id}`" class="btn btn-ghost btn-xs">
          <font-awesome-icon icon="eye" /> Revue
        </NuxtLink>
      </template>
    </AdminDataTable>
  </div>
</template>
