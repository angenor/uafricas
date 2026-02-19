<script setup lang="ts">
import type { TableColumn, FilterDefinition } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { utilisateurs, filtres, pagination, sort, loading, chargerListe, supprimer, allerPage, changerTri, reinitialiserPagination } = useAdminUtilisateurs()

const showDelete = ref(false)
const deleteTarget = ref<{ id: string; nom: string } | null>(null)
const deleteLoading = ref(false)

const colonnes: TableColumn[] = [
  { key: 'nom_complet', label: 'Utilisateur', sortable: true, width: 'w-60' },
  { key: 'email', label: 'Email', sortable: true },
  { key: 'etat', label: 'Etat', sortable: true, width: 'w-28' },
  { key: 'roles', label: 'Roles', width: 'w-40' },
  { key: 'email_verifie', label: 'Verifie', width: 'w-20', align: 'center' },
  { key: 'created_at', label: 'Inscription', sortable: true, width: 'w-32', format: (v: string) => new Date(v).toLocaleDateString('fr-FR') },
]

const filterDefs: FilterDefinition[] = [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Nom, prenom, email...' },
  { key: 'etat', label: 'Etat', type: 'select', placeholder: 'Tous les etats', options: [
    { label: 'Actif', value: 'actif' },
    { label: 'En attente', value: 'en_attente' },
    { label: 'Suspendu', value: 'suspendu' },
    { label: 'Bloque', value: 'bloque' },
  ] },
  { key: 'role', label: 'Role', type: 'select', placeholder: 'Tous les roles', options: [
    { label: 'Admin', value: 'admin' },
    { label: 'Super Admin', value: 'super_admin' },
    { label: 'Moderateur', value: 'moderateur' },
    { label: 'Utilisateur', value: 'utilisateur' },
  ] },
]

const confirmerSuppression = (item: any) => {
  deleteTarget.value = { id: item.id, nom: `${item.prenom} ${item.nom}` }
  showDelete.value = true
}

const executerSuppression = async () => {
  if (!deleteTarget.value) return
  deleteLoading.value = true
  try {
    await supprimer(deleteTarget.value.id)
    showDelete.value = false
    await chargerListe()
  }
  catch {}
  finally {
    deleteLoading.value = false
  }
}

const reinitialiser = () => {
  filtres.recherche = ''
  filtres.etat = ''
  filtres.role = ''
  filtres.pays = ''
  reinitialiserPagination()
  chargerListe()
}

// Charger les donnees au montage et lors des changements de pagination/tri
onMounted(() => chargerListe())

watch([() => pagination.page, () => sort.column, () => sort.direction], () => {
  chargerListe()
})
</script>

<template>
  <div>
    <AdminPageHeader titre="Utilisateurs" sous-titre="Gerer les comptes utilisateurs de la plateforme">
      <template #actions>
        <NuxtLink to="/admin/utilisateurs/create" class="btn btn-primary btn-sm">
          <font-awesome-icon icon="plus" class="mr-1" />
          Nouvel utilisateur
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
      :donnees="utilisateurs"
      :pagination="pagination"
      :tri-colonne="sort.column"
      :tri-direction="sort.direction"
      :loading="loading"
      @trier="changerTri"
      @aller-page="allerPage; chargerListe()"
    >
      <template #cell-nom_complet="{ item }">
        <div class="flex items-center gap-3">
          <div class="avatar placeholder">
            <div class="bg-neutral text-neutral-content rounded-full w-8 h-8">
              <span class="text-xs">{{ (item.prenom?.[0] || '') + (item.nom?.[0] || '') }}</span>
            </div>
          </div>
          <div>
            <div class="font-semibold text-sm">{{ item.prenom }} {{ item.nom }}</div>
          </div>
        </div>
      </template>

      <template #cell-etat="{ value }">
        <AdminStatusBadge :statut="value" />
      </template>

      <template #cell-roles="{ item }">
        <div class="flex flex-wrap gap-1">
          <span
            v-for="role in item.roles"
            :key="role"
            class="badge badge-xs badge-outline"
          >
            {{ role }}
          </span>
        </div>
      </template>

      <template #cell-email_verifie="{ value }">
        <font-awesome-icon
          :icon="value ? 'circle-check' : 'circle-xmark'"
          :class="value ? 'text-success' : 'text-error'"
        />
      </template>

      <template #actions="{ item }">
        <div class="flex gap-1">
          <NuxtLink :to="`/admin/utilisateurs/${item.id}`" class="btn btn-ghost btn-xs">
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
