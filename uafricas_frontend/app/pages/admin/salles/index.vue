<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { salles, filtres, pagination, sort, loading, chargerListe, supprimer, allerPage, changerTri, reinitialiserPagination } = useAdminSalles()

const showDelete = ref(false)
const deleteTarget = ref<{ id: string; nom: string } | null>(null)
const deleteLoading = ref(false)

const colonnes: TableColumn[] = [
  { key: 'titre', label: 'Titre', sortable: true },
  { key: 'langue_cible', label: 'Langue', width: 'w-28' },
  { key: 'langue_code', label: 'Code', width: 'w-20' },
  { key: 'groupe_ethnique_nom', label: 'Groupe ethnique', width: 'w-40' },
  { key: 'nombre_salles_privees', label: 'Salles privées', width: 'w-28', align: 'center' },
  { key: 'nombre_sessions', label: 'Sessions', width: 'w-24', align: 'center' },
  { key: 'nombre_moderateurs_attitres', label: 'Mod. attitrés', width: 'w-24', align: 'center' },
  { key: 'statut', label: 'Statut', width: 'w-32', align: 'center' },
  { key: 'created_at', label: 'Création', sortable: true, width: 'w-28',
    format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]

const filterDefs: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Titre, langue...' },
  { key: 'langue_cible', label: 'Langue', type: 'text', placeholder: 'Langue cible...' },
  { key: 'langue_code', label: 'Code langue', type: 'text', placeholder: 'Ex: sw, wo...' },
]

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

const reinitialiser = () => {
  filtres.recherche = ''
  filtres.langue_cible = ''
  filtres.langue_code = ''
  filtres.groupe_ethnique_id = ''
  filtres.actif = ''
  reinitialiserPagination()
  chargerListe()
}

onMounted(() => chargerListe())
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerListe())
</script>

<template>
  <div>
    <AdminPageHeader titre="Salles AfroLang" sous-titre="Gerer les salles de visioconference linguistique">
      <template #actions>
        <NuxtLink to="/admin/salles/create" class="btn btn-primary btn-sm">
          <font-awesome-icon icon="plus" class="mr-1" /> Nouvelle salle
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
      :donnees="salles"
      :pagination="pagination"
      :tri-colonne="sort.column"
      :tri-direction="sort.direction"
      :loading="loading"
      @trier="changerTri"
      @aller-page="allerPage"
    >
      <template #cell-statut="{ item }">
        <span
          v-if="item.desactivee_admin_at"
          class="badge badge-error badge-sm gap-1"
          title="Désactivée par administration"
        >
          <font-awesome-icon icon="ban" class="text-[10px]" />
          Désactivée
        </span>
        <span
          v-else-if="item.actif"
          class="badge badge-success badge-sm"
        >Actif</span>
        <span v-else class="badge badge-neutral badge-sm">Inactif</span>
      </template>
      <template #actions="{ item }">
        <div class="flex gap-1">
          <NuxtLink :to="`/admin/salles/${item.id}`" class="btn btn-ghost btn-xs">
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
