<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { favoris, stats, filtres, pagination, sort, loading, chargerListe, chargerStats, allerPage, changerTri, reinitialiserPagination } = useAdminAnnoncesFavoris()

const colonnes: TableColumn[] = [
  { key: 'annonce_titre', label: 'Annonce' },
  { key: 'annonce_etat', label: 'Etat', width: 'w-24' },
  { key: 'utilisateur_nom', label: 'Utilisateur', format: (_v: string, row: any) => `${row.utilisateur_prenom} ${row.utilisateur_nom}` },
  { key: 'utilisateur_email', label: 'Email', width: 'w-40' },
  { key: 'created_at', label: 'Date favori', sortable: true, width: 'w-28', format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]

const filterDefs: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Annonce, utilisateur...' },
]

const etatBadge = (etat: string) => {
  const map: Record<string, string> = {
    publiee: 'badge-success', en_attente: 'badge-warning', brouillon: 'badge-info',
    expiree: 'badge-neutral', suspendue: 'badge-error', supprimee: 'badge-neutral opacity-50',
  }
  return map[etat] || 'badge-neutral'
}

const reinitialiser = () => {
  filtres.recherche = ''
  filtres.annonce_id = ''
  reinitialiserPagination()
  chargerListe()
}

onMounted(() => {
  chargerListe()
  chargerStats()
})
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerListe())
</script>

<template>
  <div>
    <AdminPageHeader titre="Favoris" sous-titre="Statistiques de popularite des annonces">
      <template #actions>
        <NuxtLink to="/admin/annonces" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Annonces
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <!-- Stats globales -->
    <div v-if="stats" class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
      <div class="stat bg-base-100 shadow-sm rounded-box">
        <div class="stat-figure text-primary">
          <font-awesome-icon icon="heart" class="text-2xl" />
        </div>
        <div class="stat-title">Total favoris</div>
        <div class="stat-value text-primary">{{ stats.total_favoris }}</div>
      </div>
      <div class="stat bg-base-100 shadow-sm rounded-box">
        <div class="stat-figure text-secondary">
          <font-awesome-icon icon="star" class="text-2xl" />
        </div>
        <div class="stat-title">Top annonce</div>
        <div class="stat-value text-secondary text-lg">
          {{ stats.top_annonces[0]?.annonce_titre || '-' }}
        </div>
        <div v-if="stats.top_annonces[0]" class="stat-desc">{{ stats.top_annonces[0].nombre_favoris }} favoris</div>
      </div>
      <div class="stat bg-base-100 shadow-sm rounded-box">
        <div class="stat-figure text-accent">
          <font-awesome-icon icon="chart-line" class="text-2xl" />
        </div>
        <div class="stat-title">Annonces avec favoris</div>
        <div class="stat-value text-accent">{{ stats.top_annonces.length }}</div>
      </div>
    </div>

    <!-- Top annonces -->
    <div v-if="stats && stats.top_annonces.length > 0" class="card bg-base-100 shadow-sm mb-6">
      <div class="card-body">
        <h3 class="card-title text-base">Top annonces les plus mises en favoris</h3>
        <div class="overflow-x-auto">
          <table class="table table-sm">
            <thead>
              <tr>
                <th class="w-12">#</th>
                <th>Annonce</th>
                <th class="w-32 text-right">Favoris</th>
                <th class="w-48">Popularite</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="(item, index) in stats.top_annonces.slice(0, 10)" :key="item.annonce_id">
                <td class="font-bold">{{ index + 1 }}</td>
                <td>{{ item.annonce_titre }}</td>
                <td class="text-right font-semibold">{{ item.nombre_favoris }}</td>
                <td>
                  <progress
                    class="progress progress-primary w-full"
                    :value="item.nombre_favoris"
                    :max="stats.top_annonces[0]?.nombre_favoris || 1"
                  />
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>

    <!-- Liste des favoris -->
    <AdminFilters
      :filtres="filterDefs"
      v-model="filtres"
      @rechercher="() => { reinitialiserPagination(); chargerListe() }"
      @reinitialiser="reinitialiser"
    />

    <AdminDataTable
      :colonnes="colonnes"
      :donnees="favoris"
      :pagination="pagination"
      :tri-colonne="sort.column"
      :tri-direction="sort.direction"
      :loading="loading"
      @trier="changerTri"
      @aller-page="allerPage"
    >
      <template #cell-annonce_etat="{ value }">
        <span class="badge badge-sm" :class="etatBadge(value)">{{ value }}</span>
      </template>
    </AdminDataTable>
  </div>
</template>
