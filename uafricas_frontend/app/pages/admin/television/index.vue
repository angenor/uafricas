<script setup lang="ts">
/**
 * Chaînes de télévision : back-office.
 *
 * L'onglet « Programmes » a disparu d'ici : depuis 09q un programme est une
 * `emission_*` commune aux deux familles, gérée sur `/admin/medias/emissions`.
 * Le renvoi explicite en tête de page évite qu'un administrateur cherche
 * l'onglet disparu.
 */
import type { TableColumn, FilterDefinition } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const {
  chaines, filtresChaines,
  pagination, sort, loading,
  chargerChaines, supprimerChaine,
  allerPage, changerTri, reinitialiserPagination,
} = useAdminTelevision()

const colonnesChaines: TableColumn[] = [
  { key: 'nom', label: 'Nom', sortable: true },
  { key: 'categorie', label: 'Catégorie', sortable: true },
  { key: 'etat', label: 'État', sortable: true, width: 'w-24', align: 'center' },
  { key: 'est_en_direct', label: 'En direct', width: 'w-24', align: 'center' },
  { key: 'est_thematique', label: 'Thématique', width: 'w-28', align: 'center',
    format: (v: boolean) => (v ? 'Oui' : '—') },
  { key: 'created_at', label: 'Creation', sortable: true, width: 'w-28',
    format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]

const filtresDefChaines: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Nom, description...' },
  { key: 'categorie', label: 'Catégorie', type: 'select', placeholder: 'Toutes', options: [
    { value: 'generaliste', label: 'Generaliste' },
    { value: 'info', label: 'Info' },
    { value: 'sport', label: 'Sport' },
    { value: 'culture', label: 'Culture' },
    { value: 'divertissement', label: 'Divertissement' },
    { value: 'education', label: 'Education' },
    { value: 'musique', label: 'Musique' },
  ]},
  { key: 'etat', label: 'État', type: 'select', placeholder: 'Tous les états', options: [
    { value: 'brouillon', label: 'Brouillon' },
    { value: 'publie', label: 'Publie' },
    { value: 'suspendu', label: 'Suspendu' },
  ]},
]

const suppressionId = ref<string | null>(null)

const badgeEtat = (etat: string) => {
  const map: Record<string, string> = { brouillon: 'badge-warning', publie: 'badge-success', suspendu: 'badge-error', supprime: 'badge-ghost' }
  return map[etat] || 'badge-info'
}
const etatLabel = (etat: string) => {
  const map: Record<string, string> = { brouillon: 'Brouillon', publie: 'Publie', suspendu: 'Suspendu' }
  return map[etat] || etat
}

const supprimerElement = async () => {
  if (!suppressionId.value) return
  await supprimerChaine(suppressionId.value)
  await chargerChaines()
  suppressionId.value = null
}

const reinitialiser = () => {
  filtresChaines.recherche = ''
  filtresChaines.categorie = ''
  filtresChaines.etat = ''
  reinitialiserPagination()
  chargerChaines()
}

onMounted(() => chargerChaines())
watch([() => pagination.page, () => sort.column, () => sort.direction], chargerChaines)
</script>

<template>
  <div>
    <AdminPageHeader titre="Télévision" sous-titre="Gestion des chaînes de télévision">
      <template #actions>
        <NuxtLink to="/admin/medias/emissions?type=tele" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="film" class="mr-1" /> Programmes
        </NuxtLink>
        <NuxtLink to="/admin/television/create?type=chaines" class="btn btn-primary btn-sm">
          <font-awesome-icon icon="plus" class="mr-1" /> Creer
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div class="alert alert-info mb-6">
      <font-awesome-icon icon="circle-info" />
      <span>
        Les programmes et leurs épisodes se gèrent désormais sur
        <NuxtLink to="/admin/medias/emissions?type=tele" class="link font-semibold">Médias &rsaquo; Programmes</NuxtLink>,
        commun à la télévision et à la radio.
      </span>
    </div>

    <AdminFilters
      v-model="filtresChaines"
      :filtres="filtresDefChaines"
      @rechercher="() => { reinitialiserPagination(); chargerChaines() }"
      @reinitialiser="reinitialiser"
    />

    <AdminDataTable
      :colonnes="colonnesChaines"
      :donnees="chaines"
      :pagination="pagination"
      :tri-colonne="sort.column"
      :tri-direction="sort.direction"
      :loading="loading"
      @trier="changerTri"
      @aller-page="allerPage"
    >
      <template #cell-etat="{ value }">
        <span :class="['badge badge-sm', badgeEtat(value)]">{{ etatLabel(value) }}</span>
      </template>

      <template #cell-est_en_direct="{ value }">
        <span :class="['badge badge-sm', value ? 'badge-success' : 'badge-ghost']">
          {{ value ? 'En direct' : 'Hors ligne' }}
        </span>
      </template>

      <template #actions="{ item }">
        <div class="flex gap-1">
          <NuxtLink :to="`/admin/medias/emissions?type=tele&support_id=${item.id}`" class="btn btn-ghost btn-xs" title="Programmes de cette chaîne">
            <font-awesome-icon icon="film" />
          </NuxtLink>
          <NuxtLink :to="`/admin/television/${item.id}?type=chaines`" class="btn btn-ghost btn-xs">
            <font-awesome-icon icon="pen-to-square" />
          </NuxtLink>
          <button class="btn btn-ghost btn-xs text-error" @click="suppressionId = item.id">
            <font-awesome-icon icon="trash" />
          </button>
        </div>
      </template>
    </AdminDataTable>

    <div v-if="suppressionId" class="modal modal-open">
      <div class="modal-box">
        <h3 class="font-bold text-lg">Confirmer la suppression</h3>
        <p class="py-4">Voulez-vous vraiment supprimer cette chaîne ?</p>
        <div class="modal-action">
          <button class="btn btn-ghost" @click="suppressionId = null">Annuler</button>
          <button class="btn btn-error" @click="supprimerElement">Supprimer</button>
        </div>
      </div>
      <div class="modal-backdrop" @click="suppressionId = null" />
    </div>
  </div>
</template>
