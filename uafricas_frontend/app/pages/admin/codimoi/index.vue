<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { posts, filtres, pagination, sort, loading, chargerListe, supprimer, allerPage, changerTri, reinitialiserPagination } = useAdminCodimoi()

const showDelete = ref(false)
const deleteTarget = ref<{ id: string; nom: string } | null>(null)
const deleteLoading = ref(false)

const typeBadgeClass = (type: string | null): string => {
  const classes: Record<string, string> = {
    proverbe_adage: 'badge-info',
    citation: 'badge-warning',
    ressource_historique: 'badge-success',
    bonne_pratique: 'badge-neutral',
  }
  return classes[type || ''] || 'badge-neutral'
}

const typeLabel = (type: string | null): string => {
  const labels: Record<string, string> = {
    proverbe_adage: 'Proverbe / Adage',
    citation: 'Citation',
    ressource_historique: 'Ressource historique',
    bonne_pratique: 'Bonne pratique',
  }
  return labels[type || ''] || type || 'Non defini'
}

const tronquer = (texte: string, max: number = 80): string => {
  return texte.length > max ? texte.substring(0, max) + '...' : texte
}

const colonnes: TableColumn[] = [
  { key: 'type_codimoi', label: 'Type', width: 'w-40' },
  { key: 'contenu', label: 'Contenu' },
  { key: 'etat', label: 'État', width: 'w-24' },
  { key: 'nombre_likes', label: 'Likes', width: 'w-20', align: 'center' },
  { key: 'auteur', label: 'Auteur', width: 'w-36',
    format: (_v: any, row: any) => `${row.auteur_prenom} ${row.auteur_nom}` },
  { key: 'created_at', label: 'Creation', sortable: true, width: 'w-28',
    format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]

const filterDefs: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Contenu, auteur...' },
  { key: 'type_codimoi', label: 'Type', type: 'select', placeholder: 'Tous', options: [
    { label: 'Proverbe / Adage', value: 'proverbe_adage' },
    { label: 'Citation', value: 'citation' },
    { label: 'Ressource historique', value: 'ressource_historique' },
    { label: 'Bonne pratique', value: 'bonne_pratique' },
  ]},
  { key: 'etat', label: 'État', type: 'select', placeholder: 'Tous', options: [
    { label: 'Publie', value: 'publie' },
    { label: 'Brouillon', value: 'brouillon' },
    { label: 'Suspendu', value: 'suspendu' },
    { label: 'Supprime', value: 'supprime' },
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
  filtres.type_codimoi = ''
  filtres.etat = ''
  reinitialiserPagination()
  chargerListe()
}

onMounted(() => chargerListe())
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerListe())
</script>

<template>
  <div>
    <AdminPageHeader titre="Codi-Moi" sous-titre="Gerer les proverbes, citations et ressources culturelles">
      <template #actions>
        <NuxtLink to="/admin/codimoi/create" class="btn btn-primary btn-sm">
          <font-awesome-icon icon="plus" class="mr-1" /> Nouveau codi-moi
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
      :donnees="posts"
      :pagination="pagination"
      :tri-colonne="sort.column"
      :tri-direction="sort.direction"
      :loading="loading"
      @trier="changerTri"
      @aller-page="allerPage"
    >
      <template #cell-type_codimoi="{ value }">
        <span class="badge badge-sm" :class="typeBadgeClass(value)">
          {{ typeLabel(value) }}
        </span>
      </template>
      <template #cell-contenu="{ value }">
        <span class="text-sm">{{ tronquer(value) }}</span>
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
      <template #actions="{ item }">
        <div class="flex gap-1">
          <NuxtLink :to="`/admin/codimoi/${item.id}`" class="btn btn-ghost btn-xs">
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
