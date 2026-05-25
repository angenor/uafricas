<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { fichesPays, filtres, pagination, sort, loading, chargerListe, supprimer, allerPage, changerTri, reinitialiserPagination } = useAdminProfilsPays()

const showDelete = ref(false)
const deleteTarget = ref<{ id: string; titre: string } | null>(null)
const deleteLoading = ref(false)

const colonnes: TableColumn[] = [
  { key: 'pays_nom', label: 'Territoire', sortable: false },
  { key: 'pays_code', label: 'Code', width: 'w-20', align: 'center' },
  { key: 'slogan', label: 'Slogan' },
  { key: 'population', label: 'Population', width: 'w-28' },
  { key: 'superficie_km2', label: 'Superficie', width: 'w-28' },
  { key: 'created_at', label: 'Creation', sortable: true, width: 'w-28',
    format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]

const filterDefs: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Nom du territoire, code...' },
  { key: 'continent', label: 'Continent', type: 'select', placeholder: 'Tous', options: [
    { label: 'Afrique', value: 'Afrique' },
    { label: 'Europe', value: 'Europe' },
    { label: 'Asie', value: 'Asie' },
    { label: 'Amerique', value: 'Amerique' },
    { label: 'Oceanie', value: 'Oceanie' },
  ]},
]

const confirmerSuppression = (item: any) => {
  deleteTarget.value = { id: item.id, titre: item.pays_nom }
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
  filtres.continent = ''
  reinitialiserPagination()
  chargerListe()
}

const formaterPopulation = (val: number | null) => {
  if (val == null) return '-'
  return val.toLocaleString('fr-FR')
}

const formaterSuperficie = (val: number | null) => {
  if (val == null) return '-'
  return `${val.toLocaleString('fr-FR')} km\u00B2`
}

onMounted(() => chargerListe())
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerListe())
</script>

<template>
  <div>
    <AdminPageHeader titre="Profils territoires" sous-titre="Gerer les fiches territoires de la plateforme">
      <template #actions>
        <NuxtLink to="/admin/profils-pays/contributions" class="btn btn-outline btn-sm">
          <font-awesome-icon icon="clipboard-list" class="mr-1" /> Contributions
        </NuxtLink>
        <NuxtLink to="/admin/profils-pays/create" class="btn btn-primary btn-sm">
          <font-awesome-icon icon="plus" class="mr-1" /> Nouvelle fiche
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
      :donnees="fichesPays"
      :pagination="pagination"
      :tri-colonne="sort.column"
      :tri-direction="sort.direction"
      :loading="loading"
      @trier="changerTri"
      @aller-page="allerPage"
    >
      <template #cell-pays_nom="{ item }">
        <div class="flex items-center gap-2">
          <img
            v-if="item.image_drapeau_url"
            :src="item.image_drapeau_url"
            :alt="`Drapeau ${item.pays_nom}`"
            class="w-6 h-4 object-cover rounded-sm border border-base-300"
          />
          <span class="font-medium">{{ item.pays_nom }}</span>
        </div>
      </template>

      <template #cell-pays_code="{ value }">
        <span v-if="value" class="badge badge-sm badge-ghost">{{ value }}</span>
        <span v-else class="text-base-content/30">-</span>
      </template>

      <template #cell-slogan="{ value }">
        <span v-if="value" class="text-sm line-clamp-1">{{ value }}</span>
        <span v-else class="text-base-content/30">-</span>
      </template>

      <template #cell-population="{ item }">
        {{ formaterPopulation(item.population) }}
      </template>

      <template #cell-superficie_km2="{ item }">
        {{ formaterSuperficie(item.superficie_km2) }}
      </template>

      <template #actions="{ item }">
        <div class="flex gap-1">
          <NuxtLink :to="`/admin/profils-pays/${item.id}`" class="btn btn-ghost btn-xs">
            <font-awesome-icon icon="pen-to-square" />
          </NuxtLink>
          <button class="btn btn-ghost btn-xs text-error" @click="confirmerSuppression(item)">
            <font-awesome-icon icon="trash" />
          </button>
        </div>
      </template>
    </AdminDataTable>

    <!-- Modal suppression -->
    <AdminDeleteConfirm
      v-model:visible="showDelete"
      :titre="deleteTarget?.titre"
      :loading="deleteLoading"
      @confirmer="executerSuppression"
    />
  </div>
</template>
