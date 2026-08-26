<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { sessions, filtres, pagination, sort, loading, chargerListe, allerPage, changerTri, reinitialiserPagination } = useAdminSessions()

const colonnes: TableColumn[] = [
  { key: 'titre_display', label: 'Titre' },
  { key: 'salle_titre', label: 'Salle', width: 'w-32' },
  { key: 'etat', label: 'État', width: 'w-24' },
  { key: 'moderateur_display', label: 'Moderateur', width: 'w-32' },
  { key: 'nombre_participants_pic', label: 'Pic participants', width: 'w-28', align: 'center' },
  { key: 'duree_display', label: 'Durée', width: 'w-24' },
  { key: 'created_at', label: 'Date', sortable: true, width: 'w-28',
    format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]

const filterDefs: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Titre...' },
  {
    key: 'etat', label: 'État', type: 'select',
    options: [
      { value: '', label: 'Tous' },
      { value: 'planifiee', label: 'Planifiee' },
      { value: 'en_cours', label: 'En cours' },
      { value: 'terminee', label: 'Terminee' },
      { value: 'annulee', label: 'Annulee' },
    ],
  },
]

const formatDuree = (secondes: number | null) => {
  if (!secondes) return '-'
  const h = Math.floor(secondes / 3600)
  const m = Math.floor((secondes % 3600) / 60)
  if (h > 0) return `${h}h ${m}min`
  return `${m}min`
}

const sessionsAvecDisplay = computed(() =>
  sessions.value.map(s => ({
    ...s,
    titre_display: s.titre || 'Sans titre',
    moderateur_display: s.moderateur_prenom && s.moderateur_nom
      ? `${s.moderateur_prenom} ${s.moderateur_nom}`
      : '-',
    duree_display: formatDuree(s.duree_secondes),
  }))
)

const reinitialiser = () => {
  filtres.recherche = ''
  filtres.etat = ''
  filtres.salle_id = ''
  filtres.salle_privee_id = ''
  filtres.date_debut = ''
  filtres.date_fin = ''
  reinitialiserPagination()
  chargerListe()
}

onMounted(() => chargerListe())
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerListe())
</script>

<template>
  <div>
    <AdminPageHeader titre="Sessions AfroLang" sous-titre="Historique des sessions de visioconference" />

    <AdminFilters
      :filtres="filterDefs"
      v-model="filtres"
      @rechercher="() => { reinitialiserPagination(); chargerListe() }"
      @reinitialiser="reinitialiser"
    />

    <AdminDataTable
      :colonnes="colonnes"
      :donnees="sessionsAvecDisplay"
      :pagination="pagination"
      :tri-colonne="sort.column"
      :tri-direction="sort.direction"
      :loading="loading"
      @trier="changerTri"
      @aller-page="allerPage"
    >
      <template #cell-etat="{ value }">
        <span class="badge badge-sm" :class="{
          'badge-info': value === 'planifiee',
          'badge-success': value === 'en_cours',
          'badge-neutral': value === 'terminee',
          'badge-error': value === 'annulee',
        }">
          {{ value || '-' }}
        </span>
      </template>
      <template #actions="{ item }">
        <NuxtLink :to="`/admin/sessions/${item.id}`" class="btn btn-ghost btn-xs">
          <font-awesome-icon icon="eye" />
        </NuxtLink>
      </template>
    </AdminDataTable>
  </div>
</template>
