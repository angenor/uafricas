<script setup lang="ts">
// Liste paginée et filtrable des propositions de salle (admin)
// Feature 001-admin-salles-publiques, US2
import type {
  PropositionListeAPI,
  StatutProposition,
} from '~/composables/useAfrolang'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { listerPropositions, loading, error } = useAdminPropositionsSalle()

const liste = ref<PropositionListeAPI | null>(null)

const filtres = reactive<{
  statut: StatutProposition | ''
  langue_code: string
  date_debut: string
  date_fin: string
  page: number
  taille: number
}>({
  statut: 'en_attente',
  langue_code: '',
  date_debut: '',
  date_fin: '',
  page: 1,
  taille: 20,
})

const charger = async () => {
  const payload: Record<string, unknown> = {
    page: filtres.page,
    taille: filtres.taille,
  }
  if (filtres.statut) payload.statut = filtres.statut
  if (filtres.langue_code.trim()) payload.langue_code = filtres.langue_code.trim()
  if (filtres.date_debut) payload.date_debut = new Date(filtres.date_debut).toISOString()
  if (filtres.date_fin) payload.date_fin = new Date(filtres.date_fin).toISOString()
  liste.value = await listerPropositions(payload)
}

const totalPages = computed(() => {
  if (!liste.value || liste.value.taille === 0) return 1
  return Math.max(1, Math.ceil(liste.value.total / liste.value.taille))
})

const allerPage = (n: number) => {
  filtres.page = Math.max(1, Math.min(n, totalPages.value))
  charger()
}

const reinitialiser = () => {
  filtres.statut = 'en_attente'
  filtres.langue_code = ''
  filtres.date_debut = ''
  filtres.date_fin = ''
  filtres.page = 1
  charger()
}

watch([
  () => filtres.statut,
  () => filtres.langue_code,
  () => filtres.date_debut,
  () => filtres.date_fin,
], () => {
  filtres.page = 1
  charger()
})

onMounted(charger)
</script>

<template>
  <div>
    <AdminPageHeader
      titre="Propositions de salles publiques"
      sous-titre="File de modération communautaire (Afrolang)"
    />

    <!-- Filtres -->
    <div class="card bg-base-100 shadow-sm mb-4">
      <div class="card-body grid grid-cols-1 md:grid-cols-4 gap-3">
        <div class="form-control">
          <label class="label"><span class="label-text">Statut</span></label>
          <select v-model="filtres.statut" class="select select-bordered">
            <option value="">Tous</option>
            <option value="en_attente">En attente</option>
            <option value="validee">Validée</option>
            <option value="rejetee">Rejetée</option>
            <option value="retiree">Retirée</option>
          </select>
        </div>
        <div class="form-control">
          <label class="label"><span class="label-text">Code langue</span></label>
          <input
            v-model="filtres.langue_code"
            type="text"
            class="input input-bordered"
            placeholder="ex : sw"
          />
        </div>
        <div class="form-control">
          <label class="label"><span class="label-text">Du</span></label>
          <input v-model="filtres.date_debut" type="date" class="input input-bordered" />
        </div>
        <div class="form-control">
          <label class="label"><span class="label-text">Au</span></label>
          <input v-model="filtres.date_fin" type="date" class="input input-bordered" />
        </div>
        <div class="md:col-span-4 flex justify-end">
          <button class="btn btn-ghost btn-sm" @click="reinitialiser">
            <font-awesome-icon icon="rotate-left" class="mr-1" /> Réinitialiser
          </button>
        </div>
      </div>
    </div>

    <div v-if="error" class="alert alert-error mb-4">
      <font-awesome-icon icon="circle-exclamation" />
      <span>{{ error }}</span>
    </div>

    <div class="card bg-base-100 shadow-sm">
      <div class="card-body">
        <div v-if="loading && !liste" class="flex justify-center py-8">
          <span class="loading loading-spinner loading-lg" />
        </div>
        <table v-else class="table table-sm">
          <thead>
            <tr>
              <th>Titre</th>
              <th>Auteur</th>
              <th>Groupe ethnique</th>
              <th>Langue</th>
              <th>Statut</th>
              <th>Soumise le</th>
              <th />
            </tr>
          </thead>
          <tbody>
            <AdminAfrolangPropositionRow
              v-for="p in liste?.items ?? []"
              :key="p.id"
              :proposition="p"
            />
            <tr v-if="!loading && (liste?.items.length ?? 0) === 0">
              <td colspan="7" class="text-center text-base-content/50 py-6">
                Aucune proposition
              </td>
            </tr>
          </tbody>
        </table>

        <div v-if="totalPages > 1" class="flex justify-center mt-4 join">
          <button
            class="btn btn-sm join-item"
            :disabled="filtres.page <= 1"
            @click="allerPage(filtres.page - 1)"
          >
            «
          </button>
          <button class="btn btn-sm join-item">
            Page {{ filtres.page }} / {{ totalPages }}
          </button>
          <button
            class="btn btn-sm join-item"
            :disabled="filtres.page >= totalPages"
            @click="allerPage(filtres.page + 1)"
          >
            »
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
