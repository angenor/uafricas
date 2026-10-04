<script setup lang="ts">
/**
 * Back-office des épreuves (feature 013) : modules ouverts au jeu et vivier.
 */
import type { TableColumn, FilterDefinition } from '~/types/admin'
import {
  CLASSES_ETAT_EPREUVE,
  LIBELLES_DIFFICULTE,
  LIBELLES_ETAT_EPREUVE,
  LIBELLES_SOURCE_ETAT,
  messageErreurAdminJeu,
  type EtatEpreuve,
  type ModuleAdminAPI,
  type SourceEtat,
} from '~/composables/useAdminJeu'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const {
  epreuves, filtres, pagination, sort, loading,
  chargerEpreuves, listerModules, modifierModule,
  allerPage, changerTri, reinitialiserPagination,
} = useAdminJeu()

const modules = ref<ModuleAdminAPI[]>([])
const erreur = ref('')

const chargerModules = async () => {
  try {
    modules.value = await listerModules()
  }
  catch (e) {
    erreur.value = messageErreurAdminJeu(e, 'Impossible de charger les modules.')
  }
}

const basculerModule = async (module: ModuleAdminAPI) => {
  erreur.value = ''
  try {
    await modifierModule(module.code, !module.ouvert)
    await chargerModules()
  }
  catch (e) {
    erreur.value = messageErreurAdminJeu(e)
  }
}

const tronquer = (texte: string, max = 90) =>
  texte.length > max ? `${texte.slice(0, max)}…` : texte

const pourcentage = (taux: number | null) => (taux == null ? '—' : `${Math.round(taux * 100)} %`)

const colonnes: TableColumn[] = [
  { key: 'enonce', label: 'Énoncé' },
  { key: 'module_code', label: 'Module', sortable: true, width: 'w-28' },
  { key: 'etat', label: 'État', sortable: true, width: 'w-24' },
  { key: 'difficulte', label: 'Difficulté', sortable: true, width: 'w-24' },
  { key: 'nombre_servie', label: 'Servie', sortable: true, width: 'w-20', align: 'center' },
  { key: 'taux_reussite', label: 'Réussite', width: 'w-24', align: 'center' },
  { key: 'source_etat', label: 'Source', width: 'w-36' },
]

const filterDefs = computed<FilterDefinition[]>(() => [
  { key: 'recherche', label: 'Recherche', type: 'text', placeholder: 'Dans l\'énoncé…' },
  { key: 'module', label: 'Module', type: 'select', placeholder: 'Tous',
    options: modules.value.map(m => ({ label: m.libelle, value: m.code })) },
  { key: 'etat', label: 'État', type: 'select', placeholder: 'Tous',
    options: Object.entries(LIBELLES_ETAT_EPREUVE).map(([value, label]) => ({ label, value })) },
  { key: 'origine', label: 'Origine', type: 'select', placeholder: 'Toutes', options: [
    { label: 'Saisie', value: 'saisie' },
    { label: 'Dérivée', value: 'derivee' },
  ] },
  { key: 'anomalie', label: 'Relecture', type: 'select', placeholder: 'Toutes', options: [
    { label: 'Taux de réussite anormal', value: 'true' },
  ] },
])

const rechercher = () => {
  reinitialiserPagination()
  chargerEpreuves()
}

const reinitialiser = () => {
  filtres.recherche = ''
  filtres.module = ''
  filtres.etat = ''
  filtres.origine = ''
  filtres.difficulte = ''
  filtres.anomalie = ''
  rechercher()
}

onMounted(async () => {
  await Promise.all([chargerModules(), chargerEpreuves()])
})
watch([() => pagination.page, () => sort.column, () => sort.direction], () => chargerEpreuves())
</script>

<template>
  <div>
    <AdminPageHeader titre="Épreuves" sous-titre="Le vivier de questions des activités ludiques">
      <template #actions>
        <NuxtLink to="/admin/activites/epreuves/nouvelle" class="btn btn-primary btn-sm">
          <font-awesome-icon icon="plus" class="mr-1" /> Nouvelle épreuve
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="erreur" class="alert alert-error mb-4 text-sm">{{ erreur }}</div>

    <!-- Modules : ouverture au jeu et état du vivier -->
    <div class="mb-6 grid gap-3 sm:grid-cols-2 xl:grid-cols-4">
      <div v-for="module in modules" :key="module.code" class="card bg-base-200">
        <div class="card-body gap-2 p-4">
          <div class="flex items-center justify-between gap-2">
            <h2 class="font-bold">{{ module.libelle }}</h2>
            <label class="flex cursor-pointer items-center gap-2 text-xs">
              {{ module.ouvert ? 'Ouvert' : 'Fermé' }}
              <input
                type="checkbox"
                class="toggle toggle-success toggle-sm"
                :checked="module.ouvert"
                :aria-label="`Ouvrir les activités de ${module.libelle}`"
                @change="basculerModule(module)"
              >
            </label>
          </div>
          <p class="text-sm">
            <span class="font-bold text-success">{{ module.jouables }}</span> jouable{{ module.jouables > 1 ? 's' : '' }}
          </p>
          <p class="text-xs text-base-content/60">
            {{ module.candidates }} brouillon{{ module.candidates > 1 ? 's' : '' }}
            · {{ module.a_revoir }} à revoir
            · {{ module.retirees }} retirée{{ module.retirees > 1 ? 's' : '' }}
          </p>
        </div>
      </div>
    </div>

    <AdminFilters
      v-model="filtres"
      :filtres="filterDefs"
      @rechercher="rechercher"
      @reinitialiser="reinitialiser"
    />

    <AdminDataTable
      :colonnes="colonnes"
      :donnees="epreuves"
      :pagination="pagination"
      :tri-colonne="sort.column"
      :tri-direction="sort.direction"
      :loading="loading"
      @trier="changerTri"
      @aller-page="allerPage"
    >
      <template #cell-enonce="{ value, item }">
        <NuxtLink :to="`/admin/activites/epreuves/${item.id}`" class="link-hover link text-sm">
          {{ tronquer(value) }}
        </NuxtLink>
        <font-awesome-icon
          v-if="item.media_type"
          :icon="item.media_type === 'audio' ? 'volume-high' : 'image'"
          class="ml-2 text-xs text-base-content/50"
        />
      </template>
      <template #cell-etat="{ value }">
        <span class="badge badge-sm" :class="CLASSES_ETAT_EPREUVE[value as EtatEpreuve]">
          {{ LIBELLES_ETAT_EPREUVE[value as EtatEpreuve] }}
        </span>
      </template>
      <template #cell-difficulte="{ value }">
        <span class="text-sm">{{ LIBELLES_DIFFICULTE[value] }}</span>
      </template>
      <template #cell-taux_reussite="{ value }">
        <span class="text-sm tabular-nums">{{ pourcentage(value) }}</span>
      </template>
      <template #cell-source_etat="{ value }">
        <span
          class="text-xs"
          :class="{
            'text-base-content/50': value === 'aucune',
            'text-success': value === 'conforme',
            'text-warning': value === 'modifiee',
            'text-error': value === 'indisponible',
          }"
        >{{ LIBELLES_SOURCE_ETAT[value as SourceEtat] }}</span>
      </template>
      <template #actions="{ item }">
        <NuxtLink :to="`/admin/activites/epreuves/${item.id}`" class="btn btn-ghost btn-xs" aria-label="Modifier">
          <font-awesome-icon icon="pen-to-square" />
        </NuxtLink>
      </template>
    </AdminDataTable>
  </div>
</template>
