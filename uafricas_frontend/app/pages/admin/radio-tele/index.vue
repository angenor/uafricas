<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { stations, chaines, programmes, filtresStations, filtresChaines, filtresProgrammes, pagination, sort, loading, chargerStations, chargerChaines, chargerProgrammes, supprimerStation, supprimerChaine, supprimerProgramme, allerPage, changerTri, reinitialiserPagination } = useAdminRadioTele()

const ongletActif = ref<'stations' | 'chaines' | 'programmes'>('stations')

const colonnesStations: TableColumn[] = [
  { key: 'nom', label: 'Nom', sortable: true },
  { key: 'type_station', label: 'Type', sortable: true },
  { key: 'genre', label: 'Genre' },
  { key: 'etat', label: 'Etat', sortable: true, width: 'w-24', align: 'center' },
  { key: 'pays_nom', label: 'Territoire' },
  { key: 'created_at', label: 'Creation', sortable: true, width: 'w-28',
    format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]

const colonnesChaines: TableColumn[] = [
  { key: 'nom', label: 'Nom', sortable: true },
  { key: 'categorie', label: 'Categorie', sortable: true },
  { key: 'etat', label: 'Etat', sortable: true, width: 'w-24', align: 'center' },
  { key: 'est_en_direct', label: 'En direct', width: 'w-24', align: 'center' },
  { key: 'pays_nom', label: 'Territoire' },
  { key: 'created_at', label: 'Creation', sortable: true, width: 'w-28',
    format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]

const colonnesProgrammes: TableColumn[] = [
  { key: 'nom_emission', label: 'Emission', sortable: true },
  { key: 'type_programme', label: 'Type', sortable: true, width: 'w-24', align: 'center' },
  { key: 'chaine_nom', label: 'Télé / À la une' },
  { key: 'etat', label: 'Etat', sortable: true, width: 'w-24', align: 'center' },
  { key: 'categorie_radio', label: 'Categorie radio' },
  { key: 'langue', label: 'Langue' },
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

const filtresDefChaines: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Nom, description...' },
  { key: 'categorie', label: 'Categorie', type: 'select', placeholder: 'Toutes', options: [
    { value: 'generaliste', label: 'Generaliste' },
    { value: 'info', label: 'Info' },
    { value: 'sport', label: 'Sport' },
    { value: 'culture', label: 'Culture' },
    { value: 'divertissement', label: 'Divertissement' },
    { value: 'education', label: 'Education' },
    { value: 'musique', label: 'Musique' },
  ]},
  { key: 'etat', label: 'Etat', type: 'select', placeholder: 'Tous les etats', options: [
    { value: 'brouillon', label: 'Brouillon' },
    { value: 'publie', label: 'Publie' },
    { value: 'suspendu', label: 'Suspendu' },
  ]},
]

const filtresDefProgrammes: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Nom emission...' },
  { key: 'type_programme', label: 'Type', type: 'select', placeholder: 'Tous', options: [
    { value: 'radio', label: 'Radio' },
    { value: 'tele', label: 'Tele' },
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
  else if (ongletActif.value === 'chaines') { await supprimerChaine(suppressionId.value); await chargerChaines() }
  else { await supprimerProgramme(suppressionId.value); await chargerProgrammes() }
  suppressionId.value = null
}

const chargerDonnees = () => {
  if (ongletActif.value === 'stations') chargerStations()
  else if (ongletActif.value === 'chaines') chargerChaines()
  else chargerProgrammes()
}

const filtresActifs = computed(() => {
  if (ongletActif.value === 'stations') return filtresDefStations
  if (ongletActif.value === 'chaines') return filtresDefChaines
  return filtresDefProgrammes
})

const donneesActives = computed(() => {
  if (ongletActif.value === 'stations') return stations.value
  if (ongletActif.value === 'chaines') return chaines.value
  return programmes.value
})

const colonnesActives = computed(() => {
  if (ongletActif.value === 'stations') return colonnesStations
  if (ongletActif.value === 'chaines') return colonnesChaines
  return colonnesProgrammes
})

const filtresValeurs = computed(() => {
  if (ongletActif.value === 'stations') return filtresStations
  if (ongletActif.value === 'chaines') return filtresChaines
  return filtresProgrammes
})

const reinitialiser = () => {
  if (ongletActif.value === 'stations') { filtresStations.recherche = ''; filtresStations.type_station = ''; filtresStations.etat = '' }
  else if (ongletActif.value === 'chaines') { filtresChaines.recherche = ''; filtresChaines.categorie = ''; filtresChaines.etat = '' }
  else { filtresProgrammes.recherche = ''; filtresProgrammes.type_programme = ''; filtresProgrammes.etat = '' }
  reinitialiserPagination()
  chargerDonnees()
}

watch(ongletActif, () => { reinitialiserPagination(); chargerDonnees() })
onMounted(() => chargerStations())
watch([() => pagination.page, () => sort.column, () => sort.direction], chargerDonnees)
</script>

<template>
  <div>
    <AdminPageHeader titre="Radio & TV" sous-titre="Gestion des stations radio, chaines TV et programmes">
      <template #actions>
        <NuxtLink :to="`/admin/radio-tele/create?type=${ongletActif}`" class="btn btn-primary btn-sm">
          <font-awesome-icon icon="plus" class="mr-1" /> Creer
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div role="tablist" class="tabs tabs-bordered mb-6">
      <a role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'stations' }" @click="ongletActif = 'stations'">
        <font-awesome-icon icon="broadcast-tower" class="mr-2" /> Stations Radio
      </a>
      <a role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'chaines' }" @click="ongletActif = 'chaines'">
        <font-awesome-icon icon="tv" class="mr-2" /> Chaines TV
      </a>
      <a role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'programmes' }" @click="ongletActif = 'programmes'">
        <font-awesome-icon icon="film" class="mr-2" /> Programmes
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

      <template #cell-est_en_direct="{ value }">
        <span :class="['badge badge-sm', value ? 'badge-success' : 'badge-ghost']">
          {{ value ? 'En direct' : 'Hors ligne' }}
        </span>
      </template>

      <template #cell-type_programme="{ value }">
        <span :class="['badge badge-sm', value === 'radio' ? 'badge-info' : 'badge-accent']">
          {{ value === 'radio' ? 'Radio' : 'Tele' }}
        </span>
      </template>

      <template #cell-chaine_nom="{ item }">
        <div class="flex items-center gap-1">
          <span v-if="item.chaine_nom" class="text-sm">{{ item.chaine_nom }}</span>
          <span v-else class="text-base-content/40 text-sm">—</span>
          <span v-if="item.a_la_une" class="badge badge-sm badge-warning gap-1">
            <font-awesome-icon icon="star" /> À la une
          </span>
        </div>
      </template>

      <template #actions="{ item }">
        <div class="flex gap-1">
          <NuxtLink :to="`/admin/radio-tele/${item.id}?type=${ongletActif}`" class="btn btn-ghost btn-xs">
            <font-awesome-icon icon="pen-to-square" />
          </NuxtLink>
          <button class="btn btn-ghost btn-xs text-error" @click="suppressionId = item.id">
            <font-awesome-icon icon="trash" />
          </button>
        </div>
      </template>
    </AdminDataTable>

    <!-- Modal suppression -->
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
