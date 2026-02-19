<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { badHabits, filtres, pagination, sort, loading, chargerListe, supprimer, allerPage, changerTri, reinitialiserPagination } = useAdminBadHabits()

const showDelete = ref(false)
const deleteTarget = ref<{ id: string; nom: string } | null>(null)
const deleteLoading = ref(false)

const categorieBadgeClass = (categorie: string | null): string => {
  const classes: Record<string, string> = {
    corruption: 'badge-error',
    service_public_defaillant: 'badge-warning',
    infrastructure_degradee: 'badge-info',
    acces_services_limite: 'badge-neutral',
    insalubrite: 'badge-warning',
    probleme_securite: 'badge-error',
    autre: 'badge-neutral',
  }
  return classes[categorie || ''] || 'badge-neutral'
}

const categorieLabel = (categorie: string | null): string => {
  const labels: Record<string, string> = {
    corruption: 'Corruption',
    service_public_defaillant: 'Service public defaillant',
    infrastructure_degradee: 'Infrastructure degradee',
    acces_services_limite: 'Acces services limite',
    insalubrite: 'Insalubrite',
    probleme_securite: 'Probleme securite',
    autre: 'Autre',
  }
  return labels[categorie || ''] || categorie || 'Non defini'
}

const graviteBadgeClass = (gravite: string | null): string => {
  const classes: Record<string, string> = {
    faible: 'badge-success',
    elevee: 'badge-warning',
    critique: 'badge-error',
  }
  return classes[gravite || ''] || 'badge-neutral'
}

const graviteLabel = (gravite: string | null): string => {
  const labels: Record<string, string> = {
    faible: 'Faible',
    elevee: 'Elevee',
    critique: 'Critique',
  }
  return labels[gravite || ''] || gravite || 'Non defini'
}

const tronquer = (texte: string, max: number = 60): string => {
  return texte.length > max ? texte.substring(0, max) + '...' : texte
}

const colonnes: TableColumn[] = [
  { key: 'titre', label: 'Titre' },
  { key: 'categorie_probleme', label: 'Categorie', width: 'w-44' },
  { key: 'gravite', label: 'Gravite', width: 'w-24' },
  { key: 'etat', label: 'Etat', width: 'w-24' },
  { key: 'nombre_soutiens', label: 'Soutiens', width: 'w-20', align: 'center' },
  { key: 'publication_anonyme', label: 'Anonyme', width: 'w-20', align: 'center' },
  { key: 'created_at', label: 'Creation', sortable: true, width: 'w-28',
    format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]

const filterDefs: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Titre, description...' },
  { key: 'categorie_probleme', label: 'Categorie', type: 'select', placeholder: 'Toutes', options: [
    { label: 'Corruption', value: 'corruption' },
    { label: 'Service public defaillant', value: 'service_public_defaillant' },
    { label: 'Infrastructure degradee', value: 'infrastructure_degradee' },
    { label: 'Acces services limite', value: 'acces_services_limite' },
    { label: 'Insalubrite', value: 'insalubrite' },
    { label: 'Probleme securite', value: 'probleme_securite' },
    { label: 'Autre', value: 'autre' },
  ]},
  { key: 'gravite', label: 'Gravite', type: 'select', placeholder: 'Toutes', options: [
    { label: 'Faible', value: 'faible' },
    { label: 'Elevee', value: 'elevee' },
    { label: 'Critique', value: 'critique' },
  ]},
  { key: 'etat', label: 'Etat', type: 'select', placeholder: 'Tous', options: [
    { label: 'En attente', value: 'en_attente' },
    { label: 'Publie', value: 'publie' },
    { label: 'Suspendu', value: 'suspendu' },
  ]},
]

const confirmerSuppression = (item: any) => {
  deleteTarget.value = { id: item.id, nom: tronquer(item.titre, 50) }
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

const reinitialiser = () => {
  filtres.recherche = ''
  filtres.categorie_probleme = ''
  filtres.gravite = ''
  filtres.etat = ''
  reinitialiserPagination()
  chargerListe()
}

onMounted(() => chargerListe())
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerListe())
</script>

<template>
  <div>
    <AdminPageHeader titre="Mauvaises pratiques" sous-titre="Signalements citoyens">
      <template #actions>
        <NuxtLink to="/admin/bad-habits/create" class="btn btn-primary btn-sm">
          <font-awesome-icon icon="plus" class="mr-1" /> Nouveau signalement
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
      :donnees="badHabits"
      :pagination="pagination"
      :tri-colonne="sort.column"
      :tri-direction="sort.direction"
      :loading="loading"
      @trier="changerTri"
      @aller-page="allerPage"
    >
      <template #cell-titre="{ value }">
        <span class="text-sm font-medium">{{ tronquer(value) }}</span>
      </template>
      <template #cell-categorie_probleme="{ value }">
        <span class="badge badge-sm" :class="categorieBadgeClass(value)">
          {{ categorieLabel(value) }}
        </span>
      </template>
      <template #cell-gravite="{ value }">
        <span class="badge badge-sm" :class="graviteBadgeClass(value)">
          {{ graviteLabel(value) }}
        </span>
      </template>
      <template #cell-etat="{ value }">
        <AdminStatusBadge :statut="value" />
      </template>
      <template #cell-nombre_soutiens="{ value }">
        <span class="flex items-center justify-center gap-1">
          <font-awesome-icon icon="thumbs-up" class="text-xs text-base-content/50" />
          {{ value }}
        </span>
      </template>
      <template #cell-publication_anonyme="{ value }">
        <font-awesome-icon
          :icon="value ? 'user-secret' : 'user'"
          class="text-sm"
          :class="value ? 'text-warning' : 'text-base-content/40'"
        />
      </template>
      <template #actions="{ item }">
        <div class="flex gap-1">
          <NuxtLink :to="`/admin/bad-habits/${item.id}`" class="btn btn-ghost btn-xs">
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
