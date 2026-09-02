<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const {
  signalements, filtresSignalements, pagination, sort, loading,
  chargerSignalements, modererSignalement,
  allerPage, changerTri, reinitialiserPagination,
} = useAdminRetrouvAmis()

const showModeration = ref(false)
const moderationTarget = ref<{ id: string; nom: string; decision: string } | null>(null)
const moderationLoading = ref(false)
const successMsg = ref<string | null>(null)

const etatBadgeClass = (etat: string): string => {
  const classes: Record<string, string> = {
    en_attente: 'badge-warning',
    approuve: 'badge-success',
    rejete: 'badge-error',
  }
  return classes[etat] || 'badge-neutral'
}

const etatLabel = (etat: string): string => {
  const labels: Record<string, string> = {
    en_attente: 'En attente',
    approuve: 'Approuve',
    rejete: 'Rejete',
  }
  return labels[etat] || etat
}

const motifBadgeClass = (motif: string): string => {
  const classes: Record<string, string> = {
    faux_avis: 'badge-error',
    contenu_inapproprie: 'badge-warning',
    usurpation_identite: 'badge-error',
    spam: 'badge-neutral',
    autre: 'badge-ghost',
  }
  return classes[motif] || 'badge-neutral'
}

const motifLabel = (motif: string): string => {
  const labels: Record<string, string> = {
    faux_avis: 'Faux avis',
    contenu_inapproprie: 'Contenu inapproprie',
    usurpation_identite: 'Usurpation d\'identite',
    spam: 'Spam',
    autre: 'Autre',
  }
  return labels[motif] || motif
}

const colonnes: TableColumn[] = [
  {
    key: 'avis',
    label: 'Avis de recherche',
    format: (_v: any, row: any) => row.avis?.nom_recherche || '--',
  },
  {
    key: 'signale_par',
    label: 'Signale par',
    width: 'w-36',
    format: (_v: any, row: any) => `${row.signale_par?.prenom || ''} ${row.signale_par?.nom || ''}`.trim(),
  },
  { key: 'motif', label: 'Motif', width: 'w-36' },
  { key: 'etat', label: 'État', width: 'w-28' },
  {
    key: 'created_at',
    label: 'Date',
    sortable: true,
    width: 'w-28',
    format: (v: string) => new Date(v).toLocaleDateString('fr-FR'),
  },
]

const filterDefs: FilterDefinition[] = [
  { key: 'etat', label: 'État', type: 'select', placeholder: 'Tous', options: [
    { label: 'En attente', value: 'en_attente' },
    { label: 'Approuve', value: 'approuve' },
    { label: 'Rejete', value: 'rejete' },
  ]},
  { key: 'motif', label: 'Motif', type: 'select', placeholder: 'Tous', options: [
    { label: 'Faux avis', value: 'faux_avis' },
    { label: 'Contenu inapproprie', value: 'contenu_inapproprie' },
    { label: 'Usurpation d\'identite', value: 'usurpation_identite' },
    { label: 'Spam', value: 'spam' },
    { label: 'Autre', value: 'autre' },
  ]},
]

const ouvrirModeration = (item: any, decision: string) => {
  moderationTarget.value = {
    id: item.id,
    nom: item.avis?.nom_recherche || 'Avis inconnu',
    decision,
  }
  showModeration.value = true
}

const executerModeration = async () => {
  if (!moderationTarget.value) return
  moderationLoading.value = true
  try {
    await modererSignalement(moderationTarget.value.id, moderationTarget.value.decision)
    showModeration.value = false
    successMsg.value = `Signalement ${moderationTarget.value.decision === 'approuve' ? 'approuve' : 'rejete'} avec succes`
    setTimeout(() => { successMsg.value = null }, 3000)
    await chargerSignalements()
  } catch {} finally { moderationLoading.value = false }
}

const reinitialiser = () => {
  filtresSignalements.etat = ''
  filtresSignalements.motif = ''
  reinitialiserPagination()
  chargerSignalements()
}

onMounted(() => chargerSignalements())
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerSignalements())
</script>

<template>
  <div>
    <AdminPageHeader titre="Signalements, Retrouve Amis" sous-titre="Moderation des signalements d'avis de recherche">
      <template #actions>
        <NuxtLink to="/admin/retrouve-amis" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour aux avis
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <!-- Message de succes -->
    <div v-if="successMsg" class="alert alert-success mb-4">
      <font-awesome-icon icon="circle-check" />
      <span>{{ successMsg }}</span>
    </div>

    <AdminFilters
      :filtres="filterDefs"
      v-model="filtresSignalements"
      @rechercher="() => { reinitialiserPagination(); chargerSignalements() }"
      @reinitialiser="reinitialiser"
    />

    <AdminDataTable
      :colonnes="colonnes"
      :donnees="signalements"
      :pagination="pagination"
      :tri-colonne="sort.column"
      :tri-direction="sort.direction"
      :loading="loading"
      @trier="changerTri"
      @aller-page="allerPage"
    >
      <template #cell-avis="{ item }">
        <NuxtLink
          v-if="item.avis?.id"
          :to="`/admin/retrouve-amis/${item.avis.id}`"
          class="link link-primary text-sm font-medium"
        >
          {{ item.avis.nom_recherche }}
        </NuxtLink>
        <span v-else class="text-base-content/30">--</span>
      </template>

      <template #cell-signale_par="{ item }">
        <span class="text-sm">{{ item.signale_par?.prenom }} {{ item.signale_par?.nom }}</span>
      </template>

      <template #cell-motif="{ value }">
        <span class="badge badge-sm" :class="motifBadgeClass(value)">{{ motifLabel(value) }}</span>
      </template>

      <template #cell-etat="{ value }">
        <span class="badge badge-sm" :class="etatBadgeClass(value)">{{ etatLabel(value) }}</span>
      </template>

      <template #actions="{ item }">
        <div class="flex gap-1">
          <NuxtLink
            v-if="item.avis?.id"
            :to="`/admin/retrouve-amis/${item.avis.id}`"
            class="btn btn-ghost btn-xs"
            title="Voir l'avis"
          >
            <font-awesome-icon icon="eye" />
          </NuxtLink>
          <template v-if="item.etat === 'en_attente'">
            <button
              class="btn btn-ghost btn-xs text-success"
              title="Approuver"
              @click="ouvrirModeration(item, 'approuve')"
            >
              <font-awesome-icon icon="circle-check" />
            </button>
            <button
              class="btn btn-ghost btn-xs text-error"
              title="Rejeter"
              @click="ouvrirModeration(item, 'rejete')"
            >
              <font-awesome-icon icon="circle-xmark" />
            </button>
          </template>
        </div>
      </template>
    </AdminDataTable>

    <!-- Modal confirmation moderation -->
    <div v-if="showModeration" class="modal modal-open">
      <div class="modal-box">
        <h3 class="font-bold text-lg">
          {{ moderationTarget?.decision === 'approuve' ? 'Approuver' : 'Rejeter' }} le signalement
        </h3>
        <p class="py-2 text-sm text-base-content/70">
          Signalement concernant l'avis : <strong>{{ moderationTarget?.nom }}</strong>
        </p>
        <p class="text-sm text-base-content/60">
          <template v-if="moderationTarget?.decision === 'approuve'">
            En approuvant ce signalement, l'avis de recherche pourra etre suspendu si le nombre de signalements approuves le justifie.
          </template>
          <template v-else>
            En rejetant ce signalement, aucune action ne sera prise sur l'avis de recherche.
          </template>
        </p>
        <div class="modal-action">
          <button class="btn btn-ghost" @click="showModeration = false">Annuler</button>
          <button
            class="btn"
            :class="moderationTarget?.decision === 'approuve' ? 'btn-success' : 'btn-error'"
            :disabled="moderationLoading"
            @click="executerModeration"
          >
            <span v-if="moderationLoading" class="loading loading-spinner loading-xs" />
            {{ moderationTarget?.decision === 'approuve' ? 'Approuver' : 'Rejeter' }}
          </button>
        </div>
      </div>
      <div class="modal-backdrop" @click="showModeration = false" />
    </div>
  </div>
</template>
