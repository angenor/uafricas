<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const {
  avis, stats, filtresAvis, pagination, sort, loading,
  chargerAvis, chargerStatistiques, changerEtatAvis,
  allerPage, changerTri, reinitialiserPagination,
} = useAdminRetrouvAmis()

const showEtat = ref(false)
const etatTarget = ref<{ id: string; nom: string; etat: string } | null>(null)
const etatLoading = ref(false)

const etatBadgeClass = (etat: string): string => {
  const classes: Record<string, string> = {
    actif: 'badge-success',
    suspendu: 'badge-error',
    cloture: 'badge-ghost',
  }
  return classes[etat] || 'badge-neutral'
}

const etatLabel = (etat: string): string => {
  const labels: Record<string, string> = {
    actif: 'Actif',
    suspendu: 'Suspendu',
    cloture: 'Cloture',
  }
  return labels[etat] || etat
}

const colonnes: TableColumn[] = [
  {
    key: 'auteur',
    label: 'Auteur',
    width: 'w-36',
    format: (_v: any, row: any) => `${row.auteur?.prenom || ''} ${row.auteur?.nom || ''}`.trim(),
  },
  { key: 'nom_recherche', label: 'Personne recherchee', sortable: true },
  { key: 'ville', label: 'Ville', width: 'w-28' },
  { key: 'etat', label: 'Etat', width: 'w-24' },
  { key: 'nb_correspondances', label: 'Corresp.', width: 'w-20', align: 'center' },
  { key: 'nb_signalements', label: 'Signal.', width: 'w-20', align: 'center' },
  {
    key: 'created_at',
    label: 'Creation',
    sortable: true,
    width: 'w-28',
    format: (v: string) => new Date(v).toLocaleDateString('fr-FR'),
  },
]

const filterDefs: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Nom, prenom, ville...' },
  { key: 'etat', label: 'Etat', type: 'select', placeholder: 'Tous', options: [
    { label: 'Actif', value: 'actif' },
    { label: 'Suspendu', value: 'suspendu' },
    { label: 'Cloture', value: 'cloture' },
  ]},
  { key: 'pays_id', label: 'Territoire', type: 'text', placeholder: 'ID du territoire...' },
]

const ouvrirChangerEtat = (item: any) => {
  const nouvelEtat = item.etat === 'actif' ? 'suspendu' : 'actif'
  etatTarget.value = { id: item.id, nom: item.nom_recherche, etat: nouvelEtat }
  showEtat.value = true
}

const executerChangerEtat = async () => {
  if (!etatTarget.value) return
  etatLoading.value = true
  try {
    await changerEtatAvis(etatTarget.value.id, etatTarget.value.etat)
    showEtat.value = false
    await chargerAvis()
  } catch {} finally { etatLoading.value = false }
}

const reinitialiser = () => {
  filtresAvis.recherche = ''
  filtresAvis.etat = ''
  filtresAvis.pays_id = ''
  reinitialiserPagination()
  chargerAvis()
}

onMounted(() => {
  chargerAvis()
  chargerStatistiques()
})
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerAvis())
</script>

<template>
  <div>
    <AdminPageHeader titre="Avis de recherche — Retrouve Amis" sous-titre="Gestion et moderation des avis de recherche">
      <template #actions>
        <div v-if="stats" class="flex items-center gap-3 text-sm">
          <span class="badge badge-success badge-sm">{{ stats.avis_actifs }} actifs</span>
          <span class="badge badge-error badge-sm">{{ stats.avis_suspendus }} suspendus</span>
          <span class="badge badge-ghost badge-sm">{{ stats.avis_clotures }} clotures</span>
          <span class="badge badge-outline badge-sm">{{ stats.total_avis }} total</span>
        </div>
        <NuxtLink to="/admin/retrouve-amis/signalements" class="btn btn-outline btn-sm">
          <font-awesome-icon icon="flag" class="mr-1" /> Signalements
          <span v-if="stats && stats.signalements_en_attente > 0" class="badge badge-error badge-xs ml-1">{{ stats.signalements_en_attente }}</span>
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <AdminFilters
      :filtres="filterDefs"
      v-model="filtresAvis"
      @rechercher="() => { reinitialiserPagination(); chargerAvis() }"
      @reinitialiser="reinitialiser"
    />

    <AdminDataTable
      :colonnes="colonnes"
      :donnees="avis"
      :pagination="pagination"
      :tri-colonne="sort.column"
      :tri-direction="sort.direction"
      :loading="loading"
      @trier="changerTri"
      @aller-page="allerPage"
    >
      <template #cell-auteur="{ item }">
        <span class="text-sm font-medium">{{ item.auteur?.prenom }} {{ item.auteur?.nom }}</span>
      </template>

      <template #cell-ville="{ value }">
        <span v-if="value" class="text-sm">{{ value }}</span>
        <span v-else class="text-base-content/30">--</span>
      </template>

      <template #cell-etat="{ value }">
        <span class="badge badge-sm" :class="etatBadgeClass(value)">{{ etatLabel(value) }}</span>
      </template>

      <template #cell-nb_correspondances="{ value }">
        <span class="flex items-center justify-center gap-1">
          <font-awesome-icon icon="link" class="text-xs text-base-content/50" />
          {{ value }}
        </span>
      </template>

      <template #cell-nb_signalements="{ value }">
        <span class="flex items-center justify-center gap-1" :class="{ 'text-error': value > 0 }">
          <font-awesome-icon icon="flag" class="text-xs" />
          {{ value }}
        </span>
      </template>

      <template #actions="{ item }">
        <div class="flex gap-1">
          <NuxtLink :to="`/admin/retrouve-amis/${item.id}`" class="btn btn-ghost btn-xs" title="Voir le detail">
            <font-awesome-icon icon="eye" />
          </NuxtLink>
          <button
            v-if="item.etat !== 'cloture'"
            class="btn btn-ghost btn-xs"
            :class="item.etat === 'actif' ? 'text-error' : 'text-success'"
            :title="item.etat === 'actif' ? 'Suspendre' : 'Reactiver'"
            @click="ouvrirChangerEtat(item)"
          >
            <font-awesome-icon :icon="item.etat === 'actif' ? 'ban' : 'circle-check'" />
          </button>
        </div>
      </template>
    </AdminDataTable>

    <!-- Modal confirmation changement d'etat -->
    <div v-if="showEtat" class="modal modal-open">
      <div class="modal-box">
        <h3 class="font-bold text-lg">
          {{ etatTarget?.etat === 'suspendu' ? 'Suspendre' : 'Reactiver' }} l'avis
        </h3>
        <p class="py-2 text-sm text-base-content/70">
          Avis de recherche : <strong>{{ etatTarget?.nom }}</strong>
        </p>
        <p class="text-sm text-base-content/60">
          {{ etatTarget?.etat === 'suspendu'
            ? 'Cet avis ne sera plus visible par les autres utilisateurs.'
            : 'Cet avis sera de nouveau visible et actif.' }}
        </p>
        <div class="modal-action">
          <button class="btn btn-ghost" @click="showEtat = false">Annuler</button>
          <button
            class="btn"
            :class="etatTarget?.etat === 'suspendu' ? 'btn-error' : 'btn-success'"
            :disabled="etatLoading"
            @click="executerChangerEtat"
          >
            <span v-if="etatLoading" class="loading loading-spinner loading-xs" />
            {{ etatTarget?.etat === 'suspendu' ? 'Suspendre' : 'Reactiver' }}
          </button>
        </div>
      </div>
      <div class="modal-backdrop" @click="showEtat = false" />
    </div>
  </div>
</template>
