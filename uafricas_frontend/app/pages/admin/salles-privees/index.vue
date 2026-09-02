<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { sallesPrivees, filtres, pagination, sort, loading, chargerListe, allerPage, changerTri, reinitialiserPagination } = useAdminSallesPrivees()

const colonnes: TableColumn[] = [
  { key: 'titre', label: 'Titre', sortable: true },
  { key: 'salle_titre', label: 'Salle parente', width: 'w-36' },
  { key: 'salle_langue', label: 'Langue', width: 'w-24' },
  { key: 'createur_display', label: 'Createur', width: 'w-32' },
  { key: 'max_participants', label: 'Max participants', width: 'w-28', align: 'center' },
  { key: 'nombre_sessions', label: 'Sessions', width: 'w-24', align: 'center' },
  { key: 'actif', label: 'Actif', width: 'w-20', align: 'center' },
  { key: 'created_at', label: 'Creation', sortable: true, width: 'w-28',
    format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]

const filterDefs: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Titre...' },
]

const sallesAvecDisplay = computed(() =>
  sallesPrivees.value.map(sp => ({
    ...sp,
    createur_display: sp.createur_prenom && sp.createur_nom
      ? `${sp.createur_prenom} ${sp.createur_nom}`
      : '-',
  }))
)

const reinitialiser = () => {
  filtres.recherche = ''
  filtres.salle_id = ''
  reinitialiserPagination()
  chargerListe()
}

onMounted(() => chargerListe())
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerListe())
</script>

<template>
  <div>
    <AdminPageHeader titre="Salles privees" sous-titre="Supervision des salles privees AfroLang (lecture seule)" />

    <AdminFilters
      :filtres="filterDefs"
      v-model="filtres"
      @rechercher="() => { reinitialiserPagination(); chargerListe() }"
      @reinitialiser="reinitialiser"
    />

    <AdminDataTable
      :colonnes="colonnes"
      :donnees="sallesAvecDisplay"
      :pagination="pagination"
      :tri-colonne="sort.column"
      :tri-direction="sort.direction"
      :loading="loading"
      @trier="changerTri"
      @aller-page="allerPage"
    >
      <template #cell-actif="{ value }">
        <span :class="value ? 'badge badge-success badge-sm' : 'badge badge-neutral badge-sm'">
          {{ value ? 'Oui' : 'Non' }}
        </span>
      </template>
      <template #actions="{ item }">
        <NuxtLink :to="`/admin/salles-privees/${item.id}`" class="btn btn-ghost btn-xs">
          <font-awesome-icon icon="eye" />
        </NuxtLink>
      </template>
    </AdminDataTable>
  </div>
</template>
