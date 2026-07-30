<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import {
  useAdminEngagement,
  type AdminJournalRow,
  type AdminCategorie,
} from '~/composables/useAdminEngagement'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { listerJournal, ajuster, listerCategories } = useAdminEngagement()

const lignes = ref<AdminJournalRow[]>([])
const total = ref(0)
const page = ref(1)
const taille = 30
const chargement = ref(false)

const categories = ref<AdminCategorie[]>([])
const filtres = ref<{ utilisateur_id: string, type_action: string, categorie: string }>({
  utilisateur_id: '', type_action: '', categorie: '',
})

// Ajustement manuel
const ajust = ref({ utilisateur_id: '', points: 0, reputation_delta: 0, motif: '' })
const message = ref('')

const totalPages = computed(() => Math.max(1, Math.ceil(total.value / taille)))

const charger = async (p = 1) => {
  chargement.value = true
  const res = await listerJournal({
    page: p,
    taille,
    utilisateur_id: filtres.value.utilisateur_id || undefined,
    type_action: filtres.value.type_action || undefined,
    categorie: filtres.value.categorie || undefined,
  })
  lignes.value = res.elements
  total.value = res.total
  page.value = res.page
  chargement.value = false
}

onMounted(async () => {
  // Le filtre par catégorie n'a de sens que si la liste vient du serveur :
  // les codes de catégorie sont paramétrables et ne se codent pas en dur.
  categories.value = await listerCategories().catch(() => [])
  await charger(1)
})

const soumettreAjustement = async () => {
  if (!ajust.value.utilisateur_id) return
  await ajuster(ajust.value.utilisateur_id, ajust.value.points, ajust.value.reputation_delta, ajust.value.motif)
  message.value = 'Ajustement appliqué'
  setTimeout(() => { message.value = '' }, 2500)
  ajust.value = { utilisateur_id: '', points: 0, reputation_delta: 0, motif: '' }
  await charger(1)
}

const formaterDate = (iso: string) =>
  new Date(iso).toLocaleString('fr-FR', { day: '2-digit', month: '2-digit', year: '2-digit', hour: '2-digit', minute: '2-digit' })
</script>

<template>
  <div class="p-6 space-y-6">
    <div class="flex items-center justify-between">
      <h1 class="text-2xl font-bold">Engagement — Journal des points</h1>
      <NuxtLink to="/admin/engagement/regles" class="btn btn-sm btn-outline">
        <font-awesome-icon icon="fa-solid fa-sliders" /> Barème
      </NuxtLink>
    </div>

    <!-- Ajustement manuel -->
    <div class="card bg-base-200 p-4">
      <h2 class="font-semibold mb-2">Ajustement manuel</h2>
      <div v-if="message" class="alert alert-success py-2 mb-2">{{ message }}</div>
      <div class="flex flex-wrap gap-2 items-end">
        <label class="form-control">
          <span class="label-text text-xs">ID utilisateur</span>
          <input v-model="ajust.utilisateur_id" class="input input-sm input-bordered w-72" placeholder="UUID" />
        </label>
        <label class="form-control">
          <span class="label-text text-xs">Points</span>
          <input v-model.number="ajust.points" type="number" class="input input-sm input-bordered w-24" />
        </label>
        <label class="form-control">
          <span class="label-text text-xs">Réputation</span>
          <input v-model.number="ajust.reputation_delta" type="number" class="input input-sm input-bordered w-24" />
        </label>
        <label class="form-control flex-1 min-w-48">
          <span class="label-text text-xs">Motif</span>
          <input v-model="ajust.motif" class="input input-sm input-bordered w-full" placeholder="Raison de l'ajustement" />
        </label>
        <button class="btn btn-sm btn-primary" @click="soumettreAjustement">Appliquer</button>
      </div>
    </div>

    <!-- Filtres -->
    <div class="flex flex-wrap gap-2 items-end">
      <label class="form-control">
        <span class="label-text text-xs">Filtrer par ID utilisateur</span>
        <input v-model="filtres.utilisateur_id" class="input input-sm input-bordered w-72" placeholder="UUID (optionnel)" />
      </label>
      <label class="form-control">
        <span class="label-text text-xs">Type d'action</span>
        <input v-model="filtres.type_action" class="input input-sm input-bordered w-56" placeholder="ex. contribution_validee" />
      </label>
      <label class="form-control">
        <span class="label-text text-xs">Catégorie</span>
        <select v-model="filtres.categorie" class="select select-sm select-bordered w-48">
          <option value="">Toutes</option>
          <option v-for="c in categories" :key="c.id" :value="c.code">{{ c.libelle }}</option>
        </select>
      </label>
      <button class="btn btn-sm" @click="charger(1)">Filtrer</button>
    </div>

    <!-- Table -->
    <div class="overflow-x-auto">
      <table class="table table-zebra table-sm">
        <thead>
          <tr><th>Date</th><th>Membre</th><th>Action</th><th>Catégorie</th><th>Objet</th><th>Points</th><th>Réput.</th><th>Solde après</th></tr>
        </thead>
        <tbody>
          <tr v-if="chargement"><td colspan="8" class="text-center py-6"><span class="loading loading-spinner"></span></td></tr>
          <tr v-else-if="lignes.length === 0"><td colspan="8" class="text-center py-6 text-gray-400">Aucun mouvement.</td></tr>
          <tr v-for="l in lignes" v-else :key="l.id">
            <td class="whitespace-nowrap text-xs">{{ formaterDate(l.created_at) }}</td>
            <td>{{ l.utilisateur_nom || l.utilisateur_id }}</td>
            <td class="font-mono text-xs">{{ l.type_action }}<span v-if="l.plafond_atteint" class="badge badge-warning badge-xs ml-1">plafond</span></td>
            <td class="text-xs">{{ l.categorie_libelle || '—' }}</td>
            <td class="text-xs">{{ l.type_objet || '—' }}</td>
            <td :class="l.points >= 0 ? 'text-success font-semibold' : 'text-error font-semibold'">{{ l.points > 0 ? '+' : '' }}{{ l.points }}</td>
            <td>{{ l.reputation_delta !== 0 ? (l.reputation_delta > 0 ? '+' : '') + l.reputation_delta : '' }}</td>
            <td>{{ l.solde_apres }}</td>
          </tr>
        </tbody>
      </table>
    </div>

    <div v-if="totalPages > 1" class="flex items-center justify-center gap-3">
      <button class="btn btn-sm" :disabled="page <= 1" @click="charger(page - 1)">Précédent</button>
      <span class="text-sm">Page {{ page }} / {{ totalPages }}</span>
      <button class="btn btn-sm" :disabled="page >= totalPages" @click="charger(page + 1)">Suivant</button>
    </div>
  </div>
</template>
