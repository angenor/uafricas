<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { ideaForces, filtres, pagination, sort, loading, chargerListe, supprimer, allerPage, changerTri, reinitialiserPagination } = useAdminIdeaForces()

const showDelete = ref(false)
const deleteTarget = ref<{ id: string; nom: string } | null>(null)
const deleteLoading = ref(false)

const categorieBadgeClass = (cat: string | null): string => {
  const classes: Record<string, string> = {
    amelioration_gouvernance: 'badge-info',
    education_formation: 'badge-primary',
    sante_publique: 'badge-success',
    emploi_jeunes: 'badge-warning',
    environnement: 'badge-accent',
    transport: 'badge-secondary',
    autre: 'badge-neutral',
  }
  return classes[cat || ''] || 'badge-neutral'
}

const categorieLabel = (cat: string | null): string => {
  const labels: Record<string, string> = {
    amelioration_gouvernance: 'Gouvernance',
    education_formation: 'Education',
    sante_publique: 'Sante',
    emploi_jeunes: 'Emploi',
    environnement: 'Environnement',
    transport: 'Transport',
    autre: 'Autre',
  }
  return labels[cat || ''] || cat || 'Non defini'
}

const urgenceBadgeClass = (urgence: string | null): string => {
  const classes: Record<string, string> = {
    faible: 'badge-success',
    elevee: 'badge-warning',
    critique: 'badge-error',
  }
  return classes[urgence || ''] || 'badge-neutral'
}

const urgenceLabel = (urgence: string | null): string => {
  const labels: Record<string, string> = {
    faible: 'Faible',
    elevee: 'Elevee',
    critique: 'Critique',
  }
  return labels[urgence || ''] || urgence || 'Non defini'
}

const tronquer = (texte: string, max: number = 60): string => {
  return texte.length > max ? texte.substring(0, max) + '...' : texte
}

const colonnes: TableColumn[] = [
  { key: 'titre', label: 'Titre' },
  { key: 'categorie_proposition', label: 'Catégorie', width: 'w-32' },
  { key: 'urgence', label: 'Urgence', width: 'w-24' },
  { key: 'etat', label: 'État', width: 'w-24' },
  { key: 'nombre_soutiens', label: 'Soutiens', width: 'w-20', align: 'center' },
  { key: 'created_at', label: 'Creation', sortable: true, width: 'w-28',
    format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]

const filterDefs: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Titre, description...' },
  { key: 'categorie_proposition', label: 'Catégorie', type: 'select', placeholder: 'Toutes', options: [
    { label: 'Gouvernance', value: 'amelioration_gouvernance' },
    { label: 'Education', value: 'education_formation' },
    { label: 'Sante', value: 'sante_publique' },
    { label: 'Emploi', value: 'emploi_jeunes' },
    { label: 'Environnement', value: 'environnement' },
    { label: 'Transport', value: 'transport' },
    { label: 'Autre', value: 'autre' },
  ]},
  { key: 'urgence', label: 'Urgence', type: 'select', placeholder: 'Toutes', options: [
    { label: 'Faible', value: 'faible' },
    { label: 'Elevee', value: 'elevee' },
    { label: 'Critique', value: 'critique' },
  ]},
  { key: 'etat', label: 'État', type: 'select', placeholder: 'Tous', options: [
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
  filtres.categorie_proposition = ''
  filtres.urgence = ''
  filtres.etat = ''
  reinitialiserPagination()
  chargerListe()
}

onMounted(() => chargerListe())
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerListe())
</script>

<template>
  <div>
    <AdminPageHeader titre="Idees forces" sous-titre="Propositions citoyennes positives">
      <template #actions>
        <NuxtLink to="/admin/idea-forces/create" class="btn btn-primary btn-sm">
          <font-awesome-icon icon="plus" class="mr-1" /> Nouvelle idee force
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
      :donnees="ideaForces"
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
      <template #cell-categorie_proposition="{ value }">
        <span class="badge badge-sm" :class="categorieBadgeClass(value)">
          {{ categorieLabel(value) }}
        </span>
      </template>
      <template #cell-urgence="{ value }">
        <span class="badge badge-sm" :class="urgenceBadgeClass(value)">
          {{ urgenceLabel(value) }}
        </span>
      </template>
      <template #cell-etat="{ value }">
        <AdminStatusBadge :statut="value" />
      </template>
      <template #cell-nombre_soutiens="{ value }">
        <span class="flex items-center justify-center gap-1">
          <font-awesome-icon icon="heart" class="text-xs text-base-content/50" />
          {{ value || 0 }}
        </span>
      </template>
      <template #actions="{ item }">
        <div class="flex gap-1">
          <NuxtLink :to="`/admin/idea-forces/${item.id}`" class="btn btn-ghost btn-xs">
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
