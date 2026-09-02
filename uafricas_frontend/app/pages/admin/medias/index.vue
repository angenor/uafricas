<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'
definePageMeta({ layout: 'admin', middleware: ['admin'] })
const { medias, filtres, pagination, sort, loading, chargerListe, supprimer, allerPage, changerTri, reinitialiserPagination } = useAdminMedias()
const showDelete = ref(false)
const deleteTarget = ref<{ id: string; nom: string } | null>(null)
const deleteLoading = ref(false)
const colonnes: TableColumn[] = [
  { key: 'nom_original', label: 'Nom du fichier', sortable: true },
  { key: 'type_mime', label: 'Type', sortable: true, width: 'w-32' },
  { key: 'taille_octets', label: 'Taille', sortable: true, width: 'w-28', align: 'right',
    format: (v: number | null) => { if (!v) return '-'; if (v < 1024) return `${v} o`; if (v < 1024 * 1024) return `${(v / 1024).toFixed(1)} Ko`; return `${(v / 1024 / 1024).toFixed(1)} Mo` } },
  { key: 'created_at', label: 'Upload', sortable: true, width: 'w-28', format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]
const filterDefs: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Nom du fichier...' },
  { key: 'type_mime', label: 'Type', type: 'select', placeholder: 'Tous', options: [
    { label: 'Images', value: 'image/' }, { label: 'Videos', value: 'video/' }, { label: 'Documents', value: 'application/' }, { label: 'Audio', value: 'audio/' },
  ]},
]
const confirmerSuppression = (item: any) => { deleteTarget.value = { id: item.id, nom: item.nom_original }; showDelete.value = true }
const executerSuppression = async () => { if (!deleteTarget.value) return; deleteLoading.value = true; try { await supprimer(deleteTarget.value.id); showDelete.value = false; await chargerListe() } catch {} finally { deleteLoading.value = false } }
const reinitialiser = () => { filtres.recherche = ''; filtres.type_mime = ''; reinitialiserPagination(); chargerListe() }
onMounted(() => chargerListe())
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerListe())
</script>
<template>
  <div>
    <AdminPageHeader titre="Mediatheque" sous-titre="Gerer les fichiers media uploades">
      <template #actions><span class="text-sm text-base-content/50">Upload a venir</span></template>
    </AdminPageHeader>
    <AdminFilters :filtres="filterDefs" v-model="filtres" @rechercher="() => { reinitialiserPagination(); chargerListe() }" @reinitialiser="reinitialiser" />
    <AdminDataTable :colonnes="colonnes" :donnees="medias" :pagination="pagination" :tri-colonne="sort.column" :tri-direction="sort.direction" :loading="loading" @trier="changerTri" @aller-page="allerPage">
      <template #cell-nom_original="{ item }">
        <div class="flex items-center gap-2">
          <div v-if="item.url_publique && item.type_mime?.startsWith('image/')" class="avatar"><div class="w-8 h-8 rounded"><img :src="item.url_publique" :alt="item.nom_original" class="object-cover" /></div></div>
          <span class="font-medium truncate max-w-xs">{{ item.nom_original }}</span>
        </div>
      </template>
      <template #cell-type_mime="{ value }">
        <span v-if="value" class="badge badge-sm" :class="{ 'badge-info': value.startsWith('image/'), 'badge-accent': value.startsWith('video/'), 'badge-warning': value.startsWith('audio/'), 'badge-neutral': value.startsWith('application/') }">{{ value.split('/')[1] || value }}</span>
        <span v-else class="text-base-content/30">-</span>
      </template>
      <template #actions="{ item }">
        <div class="flex gap-1">
          <a v-if="item.url_publique" :href="item.url_publique" target="_blank" class="btn btn-ghost btn-xs"><font-awesome-icon icon="eye" /></a>
          <button class="btn btn-ghost btn-xs text-error" @click="confirmerSuppression(item)"><font-awesome-icon icon="trash" /></button>
        </div>
      </template>
    </AdminDataTable>
    <AdminDeleteConfirm v-model:visible="showDelete" :titre="deleteTarget?.nom" :loading="deleteLoading" @confirmer="executerSuppression" />
  </div>
</template>
