<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { ecoles, filtres, pagination, sort, loading, chargerListe, supprimer, allerPage, changerTri, reinitialiserPagination } = useAdminEcolesPartenaires()

const showDelete = ref(false)
const deleteTarget = ref<{ id: string; nom: string } | null>(null)
const deleteLoading = ref(false)
const erreurSuppression = ref<string | null>(null)

const colonnes: TableColumn[] = [
  { key: 'nom', label: 'Nom', sortable: true },
  { key: 'type', label: 'Type', width: 'w-28', align: 'center' },
  { key: 'ville', label: 'Ville', sortable: true, width: 'w-36' },
  { key: 'pays_nom', label: 'Territoire', width: 'w-32' },
  { key: 'nombre_facultes', label: 'Facultés', width: 'w-24', align: 'center' },
  { key: 'actif', label: 'Actif', width: 'w-20', align: 'center' },
  { key: 'created_at', label: 'Création', sortable: true, width: 'w-28',
    format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]

const filterDefs: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Nom, ville...' },
]

const confirmerSuppression = (item: any) => {
  erreurSuppression.value = null
  deleteTarget.value = { id: item.id, nom: item.nom }
  showDelete.value = true
}

const executerSuppression = async () => {
  if (!deleteTarget.value) return
  deleteLoading.value = true
  erreurSuppression.value = null
  try {
    await supprimer(deleteTarget.value.id)
    showDelete.value = false
    await chargerListe()
  } catch (e: any) {
    erreurSuppression.value = e?.data?.error || e?.message || 'Erreur lors de la désactivation'
  } finally { deleteLoading.value = false }
}

const reinitialiser = () => {
  filtres.recherche = ''
  reinitialiserPagination()
  chargerListe()
}

onMounted(() => chargerListe())
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerListe())
</script>

<template>
  <div>
    <AdminPageHeader titre="Écoles partenaires" sous-titre="Établissements partenaires de l'INUDA">
      <template #actions>
        <NuxtLink to="/admin/ecoles-partenaires/create" class="btn btn-primary btn-sm">
          <font-awesome-icon icon="plus" class="mr-1" /> Nouvelle école
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="erreurSuppression" class="alert alert-error mb-4">
      <font-awesome-icon icon="circle-exclamation" />
      <span>{{ erreurSuppression }}</span>
    </div>

    <AdminFilters
      :filtres="filterDefs"
      v-model="filtres"
      @rechercher="() => { reinitialiserPagination(); chargerListe() }"
      @reinitialiser="reinitialiser"
    />

    <AdminDataTable
      :colonnes="colonnes"
      :donnees="ecoles"
      :pagination="pagination"
      :tri-colonne="sort.column"
      :tri-direction="sort.direction"
      :loading="loading"
      @trier="changerTri"
      @aller-page="allerPage"
    >
      <template #cell-type="{ value }">
        <span :class="value === 'publique' ? 'badge badge-info badge-sm' : 'badge badge-ghost badge-sm'">
          {{ value === 'publique' ? 'Public' : 'Privé' }}
        </span>
      </template>
      <template #cell-actif="{ value }">
        <span :class="value ? 'badge badge-success badge-sm' : 'badge badge-neutral badge-sm'">
          {{ value ? 'Oui' : 'Non' }}
        </span>
      </template>
      <template #actions="{ item }">
        <div class="flex gap-1">
          <NuxtLink :to="`/admin/ecoles-partenaires/${item.id}`" class="btn btn-ghost btn-xs">
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
