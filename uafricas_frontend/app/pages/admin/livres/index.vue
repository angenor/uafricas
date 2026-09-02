<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { livres, filtres, pagination, sort, loading, chargerListe, supprimer, changerEtat, allerPage, changerTri, reinitialiserPagination } = useAdminLivres()

const showDelete = ref(false)
const deleteTarget = ref<{ id: string; titre: string } | null>(null)
const deleteLoading = ref(false)
const showEtat = ref(false)
const etatTarget = ref<{ id: string; titre: string; etat: string } | null>(null)
const etatLoading = ref(false)
const nouvelEtat = ref('')

const colonnes: TableColumn[] = [
  { key: 'titre', label: 'Titre', sortable: true },
  { key: 'info_auteur', label: 'Auteur', width: 'w-32' },
  { key: 'type_document', label: 'Type', width: 'w-24' },
  { key: 'acces', label: 'Acces', width: 'w-24' },
  { key: 'etat', label: 'État', sortable: true, width: 'w-24' },
  { key: 'nombre_vues', label: 'Vues', width: 'w-16', align: 'center' },
  { key: 'nombre_telechargements', label: 'Telech.', width: 'w-16', align: 'center' },
  { key: 'created_at', label: 'Creation', sortable: true, width: 'w-28', format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]

const filterDefs: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Titre, auteur...' },
  { key: 'type_document', label: 'Type', type: 'select', placeholder: 'Tous', options: [
    { label: 'Livre', value: 'livre' },
    { label: 'Article', value: 'article' },
    { label: 'Rapport', value: 'rapport' },
    { label: 'These', value: 'these' },
    { label: 'Memoire', value: 'memoire' },
    { label: 'Revue', value: 'revue' },
  ]},
  { key: 'acces', label: 'Acces', type: 'select', placeholder: 'Tous', options: [
    { label: 'Gratuit', value: 'gratuit' },
    { label: 'Premium', value: 'premium' },
    { label: 'Restreint', value: 'restreint' },
  ]},
  { key: 'etat', label: 'État', type: 'select', placeholder: 'Tous', options: [
    { label: 'Brouillon', value: 'brouillon' },
    { label: 'Publie', value: 'publie' },
    { label: 'Suspendu', value: 'suspendu' },
  ]},
]

const etatBadge = (etat: string) => {
  const map: Record<string, string> = {
    publie: 'badge-success',
    brouillon: 'badge-info',
    suspendu: 'badge-error',
  }
  return map[etat] || 'badge-neutral'
}

const etatLabel = (etat: string) => {
  const map: Record<string, string> = {
    publie: 'Publie',
    brouillon: 'Brouillon',
    suspendu: 'Suspendu',
  }
  return map[etat] || etat
}

const accesBadge = (acces: string) => {
  const map: Record<string, string> = {
    gratuit: 'badge-success',
    premium: 'badge-warning',
    restreint: 'badge-error',
  }
  return map[acces] || 'badge-neutral'
}

const accesLabel = (acces: string) => {
  const map: Record<string, string> = {
    gratuit: 'Gratuit',
    premium: 'Premium',
    restreint: 'Restreint',
  }
  return map[acces] || acces
}

const typeLabel = (type: string) => {
  const map: Record<string, string> = {
    livre: 'Livre',
    article: 'Article',
    rapport: 'Rapport',
    these: 'These',
    memoire: 'Memoire',
    revue: 'Revue',
  }
  return map[type] || type
}

const confirmerSuppression = (item: any) => {
  deleteTarget.value = { id: item.id, titre: item.titre }
  showDelete.value = true
}

const executerSuppression = async () => {
  if (!deleteTarget.value) return
  deleteLoading.value = true
  try {
    await supprimer(deleteTarget.value.id)
    showDelete.value = false
    await chargerListe()
  } catch {}
  finally { deleteLoading.value = false }
}

const ouvrirChangerEtat = (item: any) => {
  etatTarget.value = { id: item.id, titre: item.titre, etat: item.etat }
  nouvelEtat.value = item.etat
  showEtat.value = true
}

const executerChangerEtat = async () => {
  if (!etatTarget.value || nouvelEtat.value === etatTarget.value.etat) return
  etatLoading.value = true
  try {
    await changerEtat(etatTarget.value.id, nouvelEtat.value)
    showEtat.value = false
    await chargerListe()
  } catch {}
  finally { etatLoading.value = false }
}

const reinitialiser = () => {
  filtres.recherche = ''
  filtres.type_document = ''
  filtres.categorie_id = ''
  filtres.acces = ''
  filtres.etat = ''
  reinitialiserPagination()
  chargerListe()
}

onMounted(() => chargerListe())
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerListe())
</script>

<template>
  <div>
    <AdminPageHeader titre="Bibliotheque" sous-titre="Gerer les livres et documents">
      <template #actions>
        <NuxtLink to="/admin/livres/create" class="btn btn-primary btn-sm">
          <font-awesome-icon icon="plus" class="mr-1" /> Nouveau livre
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
      :donnees="livres"
      :pagination="pagination"
      :tri-colonne="sort.column"
      :tri-direction="sort.direction"
      :loading="loading"
      @trier="changerTri"
      @aller-page="allerPage"
    >
      <template #cell-type_document="{ value }">
        <span class="badge badge-outline badge-sm">{{ typeLabel(value) }}</span>
      </template>

      <template #cell-acces="{ value }">
        <span class="badge badge-sm" :class="accesBadge(value)">{{ accesLabel(value) }}</span>
      </template>

      <template #cell-etat="{ value }">
        <span class="badge badge-sm" :class="etatBadge(value)">{{ etatLabel(value) }}</span>
      </template>

      <template #actions="{ item }">
        <div class="flex gap-1">
          <button class="btn btn-ghost btn-xs" title="Changer état" @click="ouvrirChangerEtat(item)">
            <font-awesome-icon icon="arrows-rotate" />
          </button>
          <NuxtLink :to="`/admin/livres/${item.id}`" class="btn btn-ghost btn-xs">
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

    <!-- Modal changement d'etat -->
    <div v-if="showEtat" class="modal modal-open">
      <div class="modal-box">
        <h3 class="font-bold text-lg">Changer l'état du document</h3>
        <p class="py-2 text-sm text-base-content/70">{{ etatTarget?.titre }}</p>
        <div class="form-control mt-2">
          <label class="label"><span class="label-text">Nouvel état</span></label>
          <select v-model="nouvelEtat" class="select select-bordered">
            <option value="brouillon">Brouillon</option>
            <option value="publie">Publie</option>
            <option value="suspendu">Suspendu</option>
          </select>
        </div>
        <div class="modal-action">
          <button class="btn btn-ghost" @click="showEtat = false">Annuler</button>
          <button
            class="btn btn-primary"
            :class="{ loading: etatLoading }"
            :disabled="etatLoading || nouvelEtat === etatTarget?.etat"
            @click="executerChangerEtat"
          >
            Confirmer
          </button>
        </div>
      </div>
      <div class="modal-backdrop" @click="showEtat = false" />
    </div>
  </div>
</template>
