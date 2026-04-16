<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'
import { formaterDuree } from '~/mocks/vidafrica'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { videos, filtres, pagination, sort, loading, chargerListe, supprimer, changerEtat, allerPage, changerTri, reinitialiserPagination } = useAdminVidafrica()

const showDelete = ref(false)
const deleteTarget = ref<{ id: string; titre: string } | null>(null)
const deleteLoading = ref(false)
const showEtat = ref(false)
const etatTarget = ref<{ id: string; titre: string; etat: string } | null>(null)
const etatLoading = ref(false)
const nouvelEtat = ref('')

const colonnes: TableColumn[] = [
  { key: 'vignette_url', label: '', width: 'w-16' },
  { key: 'titre', label: 'Titre', sortable: true },
  { key: 'duree_secondes', label: 'Durée', sortable: true, width: 'w-20', format: (v: number) => formaterDuree(v) },
  { key: 'nombre_pistes', label: 'Pistes', width: 'w-16', align: 'center' },
  { key: 'etat', label: 'État', sortable: true, width: 'w-24' },
  { key: 'created_at', label: 'Création', sortable: true, width: 'w-28', format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]

const filterDefs: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Titre, description...' },
  { key: 'etat', label: 'État', type: 'select', placeholder: 'Tous', options: [
    { label: 'Brouillon', value: 'brouillon' },
    { label: 'Publié', value: 'publie' },
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
    publie: 'Publié',
    brouillon: 'Brouillon',
    suspendu: 'Suspendu',
  }
  return map[etat] || etat
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
  reinitialiserPagination()
  chargerListe()
}

onMounted(() => chargerListe())
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerListe())
</script>

<template>
  <div>
    <AdminPageHeader titre="Vidafrica" sous-titre="Gérer les vidéos sous-titrées">
      <template #actions>
        <NuxtLink to="/admin/vidafrica/create" class="btn btn-primary btn-sm">
          <font-awesome-icon icon="plus" class="mr-1" /> Nouvelle vidéo
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
      :donnees="videos"
      :pagination="pagination"
      :tri-colonne="sort.column"
      :tri-direction="sort.direction"
      :loading="loading"
      @trier="changerTri"
      @aller-page="allerPage"
    >
      <template #cell-vignette_url="{ value }">
        <div class="w-12 h-8 rounded overflow-hidden bg-base-200 flex items-center justify-center">
          <img v-if="value" :src="value" class="w-full h-full object-cover" alt="" />
          <font-awesome-icon v-else icon="video" class="text-base-content/30" />
        </div>
      </template>

      <template #cell-etat="{ value }">
        <span class="badge badge-sm" :class="etatBadge(value)">{{ etatLabel(value) }}</span>
      </template>

      <template #actions="{ item }">
        <div class="flex gap-1">
          <button class="btn btn-ghost btn-xs" title="Changer état" @click="ouvrirChangerEtat(item)">
            <font-awesome-icon icon="arrows-rotate" />
          </button>
          <NuxtLink :to="`/admin/vidafrica/${item.id}`" class="btn btn-ghost btn-xs">
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

    <!-- Modal changement d'état -->
    <div v-if="showEtat" class="modal modal-open">
      <div class="modal-box">
        <h3 class="font-bold text-lg">Changer l'état de la vidéo</h3>
        <p class="py-2 text-sm text-base-content/70">{{ etatTarget?.titre }}</p>
        <div class="form-control mt-2">
          <label class="label"><span class="label-text">Nouvel état</span></label>
          <select v-model="nouvelEtat" class="select select-bordered">
            <option value="brouillon">Brouillon</option>
            <option value="publie">Publié</option>
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
