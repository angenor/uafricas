<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const {
  chaines, programmes,
  filtresChaines, filtresProgrammes,
  pagination, sort, loading,
  chargerChaines, chargerProgrammes,
  supprimerChaine, supprimerProgramme,
  allerPage, changerTri, reinitialiserPagination,
} = useAdminTelevision()

const ongletActif = ref<'chaines' | 'programmes'>('chaines')

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
  { key: 'nom_emission', label: 'Programme', sortable: true },
  { key: 'chaine_nom', label: 'Télé / À la une' },
  { key: 'langue', label: 'Langue' },
  { key: 'etat', label: 'Etat', sortable: true, width: 'w-24', align: 'center' },
  { key: 'created_at', label: 'Creation', sortable: true, width: 'w-28',
    format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
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
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Nom programme...' },
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

const supprimerElement = async () => {
  if (!suppressionId.value) return
  if (ongletActif.value === 'chaines') { await supprimerChaine(suppressionId.value); await chargerChaines() }
  else { await supprimerProgramme(suppressionId.value); await chargerProgrammes() }
  suppressionId.value = null
}

const chargerDonnees = () => {
  if (ongletActif.value === 'chaines') chargerChaines()
  else chargerProgrammes()
}

const filtresActifs = computed(() => ongletActif.value === 'chaines' ? filtresDefChaines : filtresDefProgrammes)
const donneesActives = computed(() => ongletActif.value === 'chaines' ? chaines.value : programmes.value)
const colonnesActives = computed(() => ongletActif.value === 'chaines' ? colonnesChaines : colonnesProgrammes)
const filtresValeurs = computed(() => ongletActif.value === 'chaines' ? filtresChaines : filtresProgrammes)

const reinitialiser = () => {
  if (ongletActif.value === 'chaines') { filtresChaines.recherche = ''; filtresChaines.categorie = ''; filtresChaines.etat = '' }
  else { filtresProgrammes.recherche = ''; filtresProgrammes.etat = '' }
  reinitialiserPagination()
  chargerDonnees()
}

watch(ongletActif, () => { reinitialiserPagination(); chargerDonnees() })
onMounted(() => chargerChaines())
watch([() => pagination.page, () => sort.column, () => sort.direction], chargerDonnees)
</script>

<template>
  <div>
    <AdminPageHeader titre="Télévision" sous-titre="Gestion des chaînes et des programmes télé">
      <template #actions>
        <NuxtLink :to="`/admin/television/create?type=${ongletActif}`" class="btn btn-primary btn-sm">
          <font-awesome-icon icon="plus" class="mr-1" /> Creer
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div role="tablist" class="tabs tabs-bordered mb-6">
      <a role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'chaines' }" @click="ongletActif = 'chaines'">
        <font-awesome-icon icon="tv" class="mr-2" /> Chaînes
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

      <template #cell-est_en_direct="{ value }">
        <span :class="['badge badge-sm', value ? 'badge-success' : 'badge-ghost']">
          {{ value ? 'En direct' : 'Hors ligne' }}
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
          <NuxtLink :to="`/admin/television/${item.id}?type=${ongletActif}`" class="btn btn-ghost btn-xs">
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
