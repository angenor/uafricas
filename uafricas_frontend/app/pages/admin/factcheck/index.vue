<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { factchecks, filtres, pagination, sort, loading, chargerListe, supprimer, allerPage, changerTri, reinitialiserPagination } = useAdminFactcheck()

const showDelete = ref(false)
const deleteTarget = ref<{ id: string; nom: string } | null>(null)
const deleteLoading = ref(false)

const verdictBadgeClass = (verdict: string | null): string => {
  const classes: Record<string, string> = {
    vrai: 'badge-success',
    faux: 'badge-error',
    partiellement_vrai: 'badge-warning',
    trompeur: 'badge-warning',
    non_verifie: 'badge-ghost',
  }
  return classes[verdict || ''] || 'badge-ghost'
}

const verdictLabel = (verdict: string | null): string => {
  const labels: Record<string, string> = {
    vrai: 'Vrai',
    faux: 'Faux',
    partiellement_vrai: 'Partiellement vrai',
    trompeur: 'Trompeur',
    non_verifie: 'Non verifie',
  }
  return labels[verdict || ''] || verdict || 'Non defini'
}

const tronquer = (texte: string, max: number = 80): string => {
  return texte.length > max ? texte.substring(0, max) + '...' : texte
}

const colonnes: TableColumn[] = [
  { key: 'contenu', label: 'Contenu' },
  { key: 'verdict', label: 'Verdict', width: 'w-40' },
  { key: 'etat', label: 'État', width: 'w-24' },
  { key: 'nombre_likes', label: 'Likes', width: 'w-20', align: 'center' },
  { key: 'nombre_dislikes', label: 'Dislikes', width: 'w-20', align: 'center' },
  { key: 'created_at', label: 'Creation', sortable: true, width: 'w-28',
    format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]

const filterDefs: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Contenu...' },
  { key: 'verdict', label: 'Verdict', type: 'select', placeholder: 'Tous', options: [
    { label: 'Vrai', value: 'vrai' },
    { label: 'Faux', value: 'faux' },
    { label: 'Partiellement vrai', value: 'partiellement_vrai' },
    { label: 'Trompeur', value: 'trompeur' },
    { label: 'Non vérifié', value: 'non_verifie' },
  ]},
  { key: 'etat', label: 'État', type: 'select', placeholder: 'Tous', options: [
    { label: 'Publie', value: 'publie' },
    { label: 'Brouillon', value: 'brouillon' },
    { label: 'Suspendu', value: 'suspendu' },
  ]},
]

const confirmerSuppression = (item: any) => {
  deleteTarget.value = { id: item.id, nom: tronquer(item.contenu, 50) }
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
  filtres.verdict = ''
  filtres.etat = ''
  reinitialiserPagination()
  chargerListe()
}

onMounted(() => chargerListe())
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerListe())
</script>

<template>
  <div>
    <AdminPageHeader titre="FactCheck" sous-titre="Verifier les affirmations">
      <template #actions>
        <NuxtLink to="/admin/factcheck/create" class="btn btn-primary btn-sm">
          <font-awesome-icon icon="plus" class="mr-1" /> Nouveau factcheck
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
      :donnees="factchecks"
      :pagination="pagination"
      :tri-colonne="sort.column"
      :tri-direction="sort.direction"
      :loading="loading"
      @trier="changerTri"
      @aller-page="allerPage"
    >
      <template #cell-contenu="{ value, item }">
        <span class="text-sm">{{ tronquer(value) }}</span>
      </template>
      <template #cell-verdict="{ value }">
        <span class="badge badge-sm" :class="verdictBadgeClass(value)">
          {{ verdictLabel(value) }}
        </span>
      </template>
      <template #cell-etat="{ value }">
        <AdminStatusBadge :statut="value" />
      </template>
      <template #cell-nombre_likes="{ value }">
        <span class="flex items-center justify-center gap-1">
          <font-awesome-icon icon="thumbs-up" class="text-xs text-base-content/50" />
          {{ value }}
        </span>
      </template>
      <template #cell-nombre_dislikes="{ value }">
        <span class="flex items-center justify-center gap-1">
          <font-awesome-icon icon="thumbs-down" class="text-xs text-base-content/50" />
          {{ value }}
        </span>
      </template>
      <template #actions="{ item }">
        <div class="flex gap-1">
          <NuxtLink :to="`/admin/factcheck/${item.id}`" class="btn btn-ghost btn-xs">
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
