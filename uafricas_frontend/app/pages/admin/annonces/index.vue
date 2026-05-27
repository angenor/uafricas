<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { annonces, filtres, pagination, sort, loading, chargerListe, supprimer, changerEtat, allerPage, changerTri, reinitialiserPagination } = useAdminAnnonces()

const showDelete = ref(false)
const deleteTarget = ref<{ id: string; titre: string } | null>(null)
const deleteLoading = ref(false)
const showEtat = ref(false)
const etatTarget = ref<{ id: string; titre: string; etat: string } | null>(null)
const etatLoading = ref(false)
const nouvelEtat = ref('')

const colonnes: TableColumn[] = [
  { key: 'photo_url', label: '', width: 'w-12' },
  { key: 'titre', label: 'Titre', sortable: true },
  { key: 'type_operation', label: 'Type', width: 'w-24' },
  { key: 'etat', label: 'Etat', width: 'w-28' },
  { key: 'prix', label: 'Prix', sortable: true, width: 'w-28', align: 'right', format: (v: number, row: any) => v ? `${v.toLocaleString('fr-FR')} ${row.devise || 'XOF'}` : '—' },
  { key: 'categorie_nom', label: 'Categorie', width: 'w-28' },
  { key: 'pays_nom', label: 'Territoire', width: 'w-24' },
  { key: 'auteur_nom', label: 'Auteur', width: 'w-28', format: (_v: string, row: any) => `${row.auteur_prenom} ${row.auteur_nom}` },
  { key: 'nombre_vues', label: 'Vues', sortable: true, width: 'w-16', align: 'center' },
  { key: 'created_at', label: 'Creation', sortable: true, width: 'w-28', format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]

const filterDefs: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Titre, description, ville...' },
  { key: 'etat', label: 'Etat', type: 'select', placeholder: 'Tous', options: [
    { label: 'Brouillon', value: 'brouillon' },
    { label: 'Publiee', value: 'publiee' },
    { label: 'En attente', value: 'en_attente' },
    { label: 'Expiree', value: 'expiree' },
    { label: 'Suspendue', value: 'suspendue' },
    { label: 'Supprimee', value: 'supprimee' },
  ]},
  { key: 'type_operation', label: 'Type', type: 'select', placeholder: 'Tous', options: [
    { label: 'Vente', value: 'vente' },
    { label: 'Troc', value: 'troc' },
    { label: 'Don', value: 'don' },
    { label: 'Association', value: 'association' },
    { label: 'Opportunite', value: 'opportunite' },
  ]},
]

const etatBadge = (etat: string) => {
  const map: Record<string, string> = {
    publiee: 'badge-success',
    en_attente: 'badge-warning',
    brouillon: 'badge-info',
    expiree: 'badge-neutral',
    suspendue: 'badge-error',
    supprimee: 'badge-neutral opacity-50',
  }
  return map[etat] || 'badge-neutral'
}

const etatLabel = (etat: string) => {
  const map: Record<string, string> = {
    publiee: 'Publiee',
    en_attente: 'En attente',
    brouillon: 'Brouillon',
    expiree: 'Expiree',
    suspendue: 'Suspendue',
    supprimee: 'Supprimee',
  }
  return map[etat] || etat
}

const typeLabel = (type: string) => {
  const map: Record<string, string> = {
    vente: 'Vente',
    troc: 'Troc',
    don: 'Don',
    association: 'Association',
    opportunite: 'Opportunite',
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
  filtres.etat = ''
  filtres.type_operation = ''
  filtres.categorie_id = ''
  filtres.pays_id = ''
  filtres.cree_par = ''
  reinitialiserPagination()
  chargerListe()
}

onMounted(() => chargerListe())
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerListe())
</script>

<template>
  <div>
    <AdminPageHeader titre="Annonces" sous-titre="Gerer les annonces du Marche Africain">
      <template #actions>
        <NuxtLink to="/admin/annonces/create" class="btn btn-primary btn-sm">
          <font-awesome-icon icon="plus" class="mr-1" /> Nouvelle annonce
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
      :donnees="annonces"
      :pagination="pagination"
      :tri-colonne="sort.column"
      :tri-direction="sort.direction"
      :loading="loading"
      @trier="changerTri"
      @aller-page="allerPage"
    >
      <template #cell-photo_url="{ value }">
        <div class="avatar">
          <div class="w-10 h-10 rounded">
            <img v-if="value" :src="value" alt="" class="object-cover" />
            <div v-else class="bg-base-200 w-full h-full flex items-center justify-center">
              <font-awesome-icon icon="image" class="text-base-content/30" />
            </div>
          </div>
        </div>
      </template>

      <template #cell-type_operation="{ value }">
        <span class="badge badge-outline badge-sm">{{ typeLabel(value) }}</span>
      </template>

      <template #cell-etat="{ value }">
        <span class="badge badge-sm" :class="etatBadge(value)">{{ etatLabel(value) }}</span>
      </template>

      <template #cell-categorie_nom="{ value }">
        <span v-if="value" class="text-sm">{{ value }}</span>
        <span v-else class="text-base-content/30">—</span>
      </template>

      <template #cell-pays_nom="{ value }">
        <span v-if="value" class="text-sm">{{ value }}</span>
        <span v-else class="text-base-content/30">—</span>
      </template>

      <template #actions="{ item }">
        <div class="flex gap-1">
          <button class="btn btn-ghost btn-xs" title="Changer etat" @click="ouvrirChangerEtat(item)">
            <font-awesome-icon icon="arrows-rotate" />
          </button>
          <NuxtLink :to="`/admin/annonces/${item.id}`" class="btn btn-ghost btn-xs">
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
        <h3 class="font-bold text-lg">Changer l'etat de l'annonce</h3>
        <p class="py-2 text-sm text-base-content/70">{{ etatTarget?.titre }}</p>
        <div class="form-control mt-2">
          <label class="label"><span class="label-text">Nouvel etat</span></label>
          <select v-model="nouvelEtat" class="select select-bordered">
            <option value="brouillon">Brouillon</option>
            <option value="en_attente">En attente</option>
            <option value="publiee">Publiee</option>
            <option value="suspendue">Suspendue</option>
            <option value="expiree">Expiree</option>
            <option value="supprimee">Supprimee</option>
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
