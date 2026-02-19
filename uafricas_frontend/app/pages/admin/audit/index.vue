<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { audits, filtres, pagination, sort, loading, chargerListe, allerPage, changerTri, reinitialiserPagination } = useAdminAudit()

const colonnes: TableColumn[] = [
  {
    key: 'created_at',
    label: 'Date',
    sortable: true,
    width: 'w-40',
    format: (v: string) => new Date(v).toLocaleString('fr-FR', {
      day: '2-digit', month: '2-digit', year: 'numeric',
      hour: '2-digit', minute: '2-digit',
    }),
  },
  { key: 'utilisateur_nom', label: 'Utilisateur', width: 'w-36' },
  { key: 'action', label: 'Action', sortable: true, width: 'w-24' },
  { key: 'schema_name', label: 'Schema', sortable: true, width: 'w-28' },
  { key: 'table_name', label: 'Table', sortable: true, width: 'w-32' },
  { key: 'ip_address', label: 'IP', width: 'w-28' },
]

const filterDefs: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Rechercher dans les donnees...' },
  { key: 'action', label: 'Action', type: 'select', placeholder: 'Toutes', options: [
    { label: 'CREATE', value: 'CREATE' },
    { label: 'UPDATE', value: 'UPDATE' },
    { label: 'DELETE', value: 'DELETE' },
    { label: 'LOGIN', value: 'LOGIN' },
  ]},
  { key: 'schema_name', label: 'Schema', type: 'select', placeholder: 'Tous', options: [
    { label: 'IAM', value: 'iam' },
    { label: 'Marketplace', value: 'marketplace' },
    { label: 'Exchange', value: 'exchange' },
    { label: 'Innovation', value: 'innovation' },
    { label: 'Culture', value: 'culture' },
    { label: 'AfroLang', value: 'afrolang' },
    { label: 'Media Content', value: 'media_content' },
    { label: 'Governance', value: 'governance' },
    { label: 'Country Profile', value: 'country_profile' },
    { label: 'Shared', value: 'shared' },
  ]},
  { key: 'date_debut', label: 'Date debut', type: 'date' },
]

const actionBadge = (action: string) => {
  const map: Record<string, string> = {
    CREATE: 'badge-success',
    UPDATE: 'badge-info',
    DELETE: 'badge-error',
    LOGIN: 'badge-neutral',
  }
  return map[action] || 'badge-neutral'
}

const reinitialiser = () => {
  filtres.recherche = ''
  filtres.action = ''
  filtres.schema_name = ''
  filtres.table_name = ''
  filtres.ip_address = ''
  filtres.date_debut = ''
  filtres.date_fin = ''
  reinitialiserPagination()
  chargerListe()
}

onMounted(() => chargerListe())
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerListe())
</script>

<template>
  <div>
    <AdminPageHeader titre="Audit & Logs" sous-titre="Journal des actions administratives">
      <template #actions>
        <span class="text-sm text-base-content/50">
          <font-awesome-icon icon="clock-rotate-left" class="mr-1" /> Lecture seule
        </span>
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
      :donnees="audits"
      :pagination="pagination"
      :tri-colonne="sort.column"
      :tri-direction="sort.direction"
      :loading="loading"
      @trier="changerTri"
      @aller-page="allerPage"
    >
      <template #cell-utilisateur_nom="{ value }">
        <span v-if="value">{{ value }}</span>
        <span v-else class="text-base-content/40 italic">Systeme</span>
      </template>

      <template #cell-action="{ value }">
        <span class="badge badge-sm" :class="actionBadge(value)">{{ value }}</span>
      </template>

      <template #cell-schema_name="{ value }">
        <span class="badge badge-outline badge-sm">{{ value }}</span>
      </template>

      <template #cell-ip_address="{ value }">
        <span v-if="value" class="font-mono text-xs">{{ value }}</span>
        <span v-else class="text-base-content/40">-</span>
      </template>

      <template #actions="{ item }">
        <NuxtLink :to="`/admin/audit/${item.id}`" class="btn btn-ghost btn-xs">
          <font-awesome-icon icon="eye" />
        </NuxtLink>
      </template>
    </AdminDataTable>
  </div>
</template>
