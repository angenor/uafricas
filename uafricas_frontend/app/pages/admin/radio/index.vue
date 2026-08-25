<script setup lang="ts">
/**
 * Stations de radio : back-office.
 *
 * L'onglet « Émissions » a disparu d'ici : depuis 09q une émission est une
 * `emission_*` commune aux deux familles, gérée sur `/admin/medias/emissions`.
 */
import type { TableColumn, FilterDefinition } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const {
  stations, filtresStations,
  pagination, sort, loading,
  chargerStations, supprimerStation,
  allerPage, changerTri, reinitialiserPagination,
} = useAdminRadio()

const colonnesStations: TableColumn[] = [
  { key: 'nom', label: 'Nom', sortable: true },
  { key: 'type_station', label: 'Type', sortable: true },
  { key: 'genre', label: 'Genre' },
  { key: 'etat', label: 'Etat', sortable: true, width: 'w-24', align: 'center' },
  { key: 'pays_nom', label: 'Territoire' },
  { key: 'ville', label: 'Ville' },
  { key: 'created_at', label: 'Creation', sortable: true, width: 'w-28',
    format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]

const filtresDefStations: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Nom, description...' },
  { key: 'type_station', label: 'Type', type: 'select', placeholder: 'Tous les types', options: [
    { value: 'nationale', label: 'Nationale' },
    { value: 'locale', label: 'Locale' },
    { value: 'internationale', label: 'Internationale' },
  ]},
  { key: 'etat', label: 'Etat', type: 'select', placeholder: 'Tous les etats', options: [
    { value: 'brouillon', label: 'Brouillon' },
    { value: 'publie', label: 'Publie' },
    { value: 'suspendu', label: 'Suspendu' },
  ]},
]

const suppressionId = ref<string | null>(null)

const badgeEtat = (etat: string) => {
  const map: Record<string, string> = { brouillon: 'badge-warning', publie: 'badge-success', suspendu: 'badge-error', supprime: 'badge-ghost' }
  return map[etat] || 'badge-info'
}
const etatLabel = (etat: string) => {
  const map: Record<string, string> = { brouillon: 'Brouillon', publie: 'Publie', suspendu: 'Suspendu' }
  return map[etat] || etat
}
const labelType = (type: string) => {
  const map: Record<string, string> = { nationale: 'Nationale', locale: 'Locale', internationale: 'Internationale' }
  return map[type] || type
}

const supprimerElement = async () => {
  if (!suppressionId.value) return
  await supprimerStation(suppressionId.value)
  await chargerStations()
  suppressionId.value = null
}

const reinitialiser = () => {
  filtresStations.recherche = ''
  filtresStations.type_station = ''
  filtresStations.etat = ''
  reinitialiserPagination()
  chargerStations()
}

onMounted(() => chargerStations())
watch([() => pagination.page, () => sort.column, () => sort.direction], chargerStations)
</script>

<template>
  <div>
    <AdminPageHeader titre="Radio" sous-titre="Gestion des stations de radio">
      <template #actions>
        <NuxtLink to="/admin/medias/emissions?type=radio" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="film" class="mr-1" /> Émissions
        </NuxtLink>
        <NuxtLink to="/admin/radio/create?type=stations" class="btn btn-primary btn-sm">
          <font-awesome-icon icon="plus" class="mr-1" /> Creer
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div class="alert alert-info mb-6">
      <font-awesome-icon icon="circle-info" />
      <span>
        Les émissions et leurs épisodes se gèrent désormais sur
        <NuxtLink to="/admin/medias/emissions?type=radio" class="link font-semibold">Médias &rsaquo; Programmes</NuxtLink>,
        commun à la radio et à la télévision.
      </span>
    </div>

    <AdminFilters
      v-model="filtresStations"
      :filtres="filtresDefStations"
      @rechercher="() => { reinitialiserPagination(); chargerStations() }"
      @reinitialiser="reinitialiser"
    />

    <AdminDataTable
      :colonnes="colonnesStations"
      :donnees="stations"
      :pagination="pagination"
      :tri-colonne="sort.column"
      :tri-direction="sort.direction"
      :loading="loading"
      @trier="changerTri"
      @aller-page="allerPage"
    >
      <template #cell-etat="{ value }">
        <span :class="['badge badge-sm', badgeEtat(value)]">{{ etatLabel(value) }}</span>
      </template>

      <template #cell-type_station="{ value }">
        {{ labelType(value) }}
      </template>

      <template #actions="{ item }">
        <div class="flex gap-1">
          <NuxtLink :to="`/admin/medias/emissions?type=radio&support_id=${item.id}`" class="btn btn-ghost btn-xs" title="Émissions de cette station">
            <font-awesome-icon icon="film" />
          </NuxtLink>
          <NuxtLink :to="`/admin/radio/${item.id}?type=stations`" class="btn btn-ghost btn-xs">
            <font-awesome-icon icon="pen-to-square" />
          </NuxtLink>
          <button class="btn btn-ghost btn-xs text-error" @click="suppressionId = item.id">
            <font-awesome-icon icon="trash" />
          </button>
        </div>
      </template>
    </AdminDataTable>

    <div v-if="suppressionId" class="modal modal-open">
      <div class="modal-box">
        <h3 class="font-bold text-lg">Confirmer la suppression</h3>
        <p class="py-4">Voulez-vous vraiment supprimer cette station ?</p>
        <div class="modal-action">
          <button class="btn btn-ghost" @click="suppressionId = null">Annuler</button>
          <button class="btn btn-error" @click="supprimerElement">Supprimer</button>
        </div>
      </div>
      <div class="modal-backdrop" @click="suppressionId = null" />
    </div>
  </div>
</template>
