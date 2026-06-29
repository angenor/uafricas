<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const {
  stations, programmes,
  filtresStations, filtresProgrammes,
  pagination, sort, loading,
  chargerStations, chargerProgrammes,
  supprimerStation, supprimerProgramme,
  allerPage, changerTri, reinitialiserPagination,
} = useAdminRadio()

const ongletActif = ref<'stations' | 'programmes'>('stations')

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

const colonnesProgrammes: TableColumn[] = [
  { key: 'nom_emission', label: 'Emission', sortable: true },
  { key: 'station_nom', label: 'Station / À la une' },
  { key: 'categorie_radio', label: 'Categorie' },
  { key: 'langue', label: 'Langue' },
  { key: 'etat', label: 'Etat', sortable: true, width: 'w-24', align: 'center' },
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

const filtresDefProgrammes: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Nom emission...' },
  { key: 'categorie_radio', label: 'Categorie', type: 'select', placeholder: 'Toutes', options: [
    { value: 'information', label: 'Information' },
    { value: 'divertissement', label: 'Divertissement' },
    { value: 'musique', label: 'Musique' },
    { value: 'culture', label: 'Culture' },
    { value: 'sport', label: 'Sport' },
    { value: 'education', label: 'Education' },
    { value: 'debat', label: 'Debat' },
    { value: 'religieux', label: 'Religieux' },
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
  if (ongletActif.value === 'stations') { await supprimerStation(suppressionId.value); await chargerStations() }
  else { await supprimerProgramme(suppressionId.value); await chargerProgrammes() }
  suppressionId.value = null
}

const chargerDonnees = () => {
  if (ongletActif.value === 'stations') chargerStations()
  else chargerProgrammes()
}

const filtresActifs = computed(() => ongletActif.value === 'stations' ? filtresDefStations : filtresDefProgrammes)
const donneesActives = computed(() => ongletActif.value === 'stations' ? stations.value : programmes.value)
const colonnesActives = computed(() => ongletActif.value === 'stations' ? colonnesStations : colonnesProgrammes)
const filtresValeurs = computed(() => ongletActif.value === 'stations' ? filtresStations : filtresProgrammes)

const reinitialiser = () => {
  if (ongletActif.value === 'stations') { filtresStations.recherche = ''; filtresStations.type_station = ''; filtresStations.etat = '' }
  else { filtresProgrammes.recherche = ''; filtresProgrammes.categorie_radio = ''; filtresProgrammes.etat = '' }
  reinitialiserPagination()
  chargerDonnees()
}

watch(ongletActif, () => { reinitialiserPagination(); chargerDonnees() })
onMounted(() => chargerStations())
watch([() => pagination.page, () => sort.column, () => sort.direction], chargerDonnees)
</script>

<template>
  <div>
    <AdminPageHeader titre="Radio" sous-titre="Gestion des stations et des émissions radio">
      <template #actions>
        <NuxtLink :to="`/admin/radio/create?type=${ongletActif}`" class="btn btn-primary btn-sm">
          <font-awesome-icon icon="plus" class="mr-1" /> Creer
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div role="tablist" class="tabs tabs-bordered mb-6">
      <a role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'stations' }" @click="ongletActif = 'stations'">
        <font-awesome-icon icon="broadcast-tower" class="mr-2" /> Stations
      </a>
      <a role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'programmes' }" @click="ongletActif = 'programmes'">
        <font-awesome-icon icon="film" class="mr-2" /> Émissions
      </a>
    </div>

    <AdminFilters
      :filtres="filtresActifs"
      v-model="filtresValeurs"
      @rechercher="() => { reinitialiserPagination(); chargerDonnees() }"
      @reinitialiser="reinitialiser"
    />

    <AdminDataTable
      :colonnes="colonnesActives"
      :donnees="donneesActives"
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

      <template #cell-station_nom="{ item }">
        <div class="flex items-center gap-1">
          <span v-if="item.station_nom" class="text-sm">{{ item.station_nom }}</span>
          <span v-else class="text-base-content/40 text-sm">—</span>
          <span v-if="item.a_la_une" class="badge badge-sm badge-warning gap-1">
            <font-awesome-icon icon="star" /> À la une
          </span>
        </div>
      </template>

      <template #actions="{ item }">
        <div class="flex gap-1">
          <NuxtLink :to="`/admin/radio/${item.id}?type=${ongletActif}`" class="btn btn-ghost btn-xs">
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
        <p class="py-4">Voulez-vous vraiment supprimer cet element ?</p>
        <div class="modal-action">
          <button class="btn btn-ghost" @click="suppressionId = null">Annuler</button>
          <button class="btn btn-error" @click="supprimerElement">Supprimer</button>
        </div>
      </div>
      <div class="modal-backdrop" @click="suppressionId = null" />
    </div>
  </div>
</template>
