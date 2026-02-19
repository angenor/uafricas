<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { programmes, filtres, pagination, sort, loading, chargerListe, changerEtat, supprimer, allerPage, changerTri, reinitialiserPagination } = useAdminProgrammes()

const showDelete = ref(false)
const deleteTarget = ref<{ id: string; nom: string } | null>(null)
const deleteLoading = ref(false)

const showEtat = ref(false)
const etatTarget = ref<{ id: string; titre: string; etat: string } | null>(null)
const nouvelEtat = ref('')
const etatLoading = ref(false)

const colonnes: TableColumn[] = [
  { key: 'titre', label: 'Titre', sortable: true },
  { key: 'etat', label: 'Etat', sortable: true, width: 'w-28', align: 'center' },
  { key: 'duree', label: 'Duree', width: 'w-28' },
  { key: 'pays_nom', label: 'Pays', width: 'w-32' },
  { key: 'domaine_nom', label: 'Domaine', width: 'w-32' },
  { key: 'nombre_places', label: 'Places', width: 'w-20', align: 'center' },
  { key: 'date_debut', label: 'Debut', sortable: true, width: 'w-28',
    format: (v: string) => v ? new Date(v).toLocaleDateString('fr-FR') : '-' },
  { key: 'created_at', label: 'Creation', sortable: true, width: 'w-28',
    format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]

const filterDefs: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Titre, description...' },
  { key: 'etat', label: 'Etat', type: 'select', placeholder: 'Tous', options: [
    { label: 'Brouillon', value: 'brouillon' },
    { label: 'En attente', value: 'en_attente_validation' },
    { label: 'Publie', value: 'publie' },
    { label: 'En cours', value: 'en_cours' },
    { label: 'Termine', value: 'termine' },
    { label: 'Suspendu', value: 'suspendu' },
    { label: 'Annule', value: 'annule' },
  ]},
  { key: 'duree', label: 'Duree', type: 'select', placeholder: 'Toutes', options: [
    { label: '1 semaine', value: '1_semaine' },
    { label: '2 semaines', value: '2_semaines' },
    { label: '1 mois', value: '1_mois' },
    { label: '3 mois', value: '3_mois' },
    { label: '6 mois', value: '6_mois' },
    { label: '1 an', value: '1_an' },
  ]},
]

const etatBadge = (etat: string) => {
  const map: Record<string, string> = {
    brouillon: 'badge-info',
    en_attente_validation: 'badge-warning',
    publie: 'badge-success',
    en_cours: 'badge-success',
    termine: 'badge-neutral',
    suspendu: 'badge-warning',
    annule: 'badge-error',
  }
  return map[etat] || 'badge-neutral'
}

const etatLabel = (etat: string) => {
  const map: Record<string, string> = {
    brouillon: 'Brouillon',
    en_attente_validation: 'En attente',
    publie: 'Publie',
    en_cours: 'En cours',
    termine: 'Termine',
    suspendu: 'Suspendu',
    annule: 'Annule',
  }
  return map[etat] || etat
}

const dureeLabel = (duree: string | null) => {
  if (!duree) return '-'
  const map: Record<string, string> = {
    '1_semaine': '1 sem.',
    '2_semaines': '2 sem.',
    '3_semaines': '3 sem.',
    '6_semaines': '6 sem.',
    '1_mois': '1 mois',
    '2_mois': '2 mois',
    '3_mois': '3 mois',
    '6_mois': '6 mois',
    '1_an': '1 an',
  }
  return map[duree] || duree
}

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
  } catch {} finally { deleteLoading.value = false }
}

const ouvrirChangerEtat = (item: any) => {
  etatTarget.value = { id: item.id, titre: item.titre, etat: item.etat }
  nouvelEtat.value = item.etat
  showEtat.value = true
}

const executerChangerEtat = async () => {
  if (!etatTarget.value || nouvelEtat.value === etatTarget.value.etat) return
  etatLoading.value = true
  try {
    await changerEtat(etatTarget.value.id, nouvelEtat.value)
    showEtat.value = false
    await chargerListe()
  } catch {} finally { etatLoading.value = false }
}

const reinitialiser = () => {
  filtres.recherche = ''
  filtres.etat = ''
  filtres.domaine_id = ''
  filtres.duree = ''
  filtres.pays_id = ''
  reinitialiserPagination()
  chargerListe()
}

onMounted(() => chargerListe())
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerListe())
</script>

<template>
  <div>
    <AdminPageHeader titre="Programmes d'echange" sous-titre="Gerer les programmes d'echange et sabbatiques">
      <template #actions>
        <NuxtLink to="/admin/programmes/create" class="btn btn-primary btn-sm">
          <font-awesome-icon icon="plus" class="mr-1" /> Nouveau programme
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
      :donnees="programmes"
      :pagination="pagination"
      :tri-colonne="sort.column"
      :tri-direction="sort.direction"
      :loading="loading"
      @trier="changerTri"
      @aller-page="allerPage"
    >
      <template #cell-etat="{ value }">
        <span :class="['badge badge-sm', etatBadge(value)]">
          {{ etatLabel(value) }}
        </span>
      </template>
      <template #cell-duree="{ value }">
        {{ dureeLabel(value) }}
      </template>
      <template #cell-nombre_places="{ value }">
        {{ value ?? '-' }}
      </template>
      <template #actions="{ item }">
        <div class="flex gap-1">
          <button class="btn btn-ghost btn-xs" title="Changer etat" @click="ouvrirChangerEtat(item)">
            <font-awesome-icon icon="arrows-rotate" />
          </button>
          <NuxtLink :to="`/admin/programmes/${item.id}`" class="btn btn-ghost btn-xs">
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

    <!-- Modal changement etat -->
    <div v-if="showEtat" class="modal modal-open">
      <div class="modal-box">
        <h3 class="font-bold text-lg mb-4">Changer l'etat</h3>
        <p class="mb-2 text-sm text-base-content/70">Programme : {{ etatTarget?.titre }}</p>
        <div class="form-control">
          <label class="label"><span class="label-text">Nouvel etat</span></label>
          <select v-model="nouvelEtat" class="select select-bordered">
            <option value="brouillon">Brouillon</option>
            <option value="en_attente_validation">En attente de validation</option>
            <option value="publie">Publie</option>
            <option value="en_cours">En cours</option>
            <option value="termine">Termine</option>
            <option value="suspendu">Suspendu</option>
            <option value="annule">Annule</option>
          </select>
        </div>
        <div class="modal-action">
          <button class="btn btn-ghost" @click="showEtat = false">Annuler</button>
          <button class="btn btn-primary" :class="{ loading: etatLoading }" :disabled="etatLoading || nouvelEtat === etatTarget?.etat" @click="executerChangerEtat">
            Confirmer
          </button>
        </div>
      </div>
      <div class="modal-backdrop" @click="showEtat = false" />
    </div>
  </div>
</template>
