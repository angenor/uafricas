<script setup lang="ts">
// Modale « Proposer une salle » + suivi de mes propositions.
// Feature 001-admin-salles-publiques, US1 — refactor en modale (UX choix).
import type {
  PropositionSalle,
  StatutProposition,
  TerritoireAPI,
} from '~/composables/useAfrolang'

const props = defineProps<{
  open: boolean
}>()

const emit = defineEmits<{
  close: []
}>()

const { listerGroupesEthniques, listerMesPropositions, listerTerritoires } = useAfrolang()

interface GroupeOption {
  id: string
  nom: string
  pays_nom: string | null
}

const onglet = ref<'proposer' | 'mes-propositions'>('proposer')
const groupesDisponibles = ref<GroupeOption[]>([])
const territoiresDisponibles = ref<TerritoireAPI[]>([])
const propositions = ref<PropositionSalle[]>([])
const chargementListe = ref(false)
const filtreStatut = ref<StatutProposition | ''>('')
const page = ref(1)
const taille = 12
const total = ref(0)
const totalPages = computed(() => Math.max(1, Math.ceil(total.value / taille)))

const chargerGroupes = async () => {
  const liste = await listerGroupesEthniques({ par_page: 100 })
  if (!liste) return
  groupesDisponibles.value = liste.groupes
    .filter(g => !g.salle_active)
    .map((g): GroupeOption => ({
      id: g.id,
      nom: g.nom,
      pays_nom: g.pays_nom ?? null,
    }))
}

const chargerTerritoires = async () => {
  territoiresDisponibles.value = await listerTerritoires()
}

const rechargerListe = async () => {
  chargementListe.value = true
  try {
    const reponse = await listerMesPropositions({
      statut: filtreStatut.value || undefined,
      page: page.value,
      taille,
    })
    if (reponse) {
      propositions.value = reponse.items
      total.value = reponse.total
    }
  }
  finally {
    chargementListe.value = false
  }
}

const aller = (n: number) => {
  if (n < 1 || n > totalPages.value) return
  page.value = n
  rechargerListe()
}

const ajouterProposition = (_p: PropositionSalle) => {
  page.value = 1
  filtreStatut.value = ''
  onglet.value = 'mes-propositions'
  rechargerListe()
}

const mettreAJourProposition = (p: PropositionSalle) => {
  const idx = propositions.value.findIndex(x => x.id === p.id)
  if (idx >= 0) propositions.value[idx] = p
}

let dejaCharge = false
const chargerSiBesoin = async () => {
  if (dejaCharge) return
  dejaCharge = true
  await Promise.all([chargerGroupes(), chargerTerritoires(), rechargerListe()])
}

watch(() => props.open, (val) => {
  if (val) chargerSiBesoin()
})

const fermer = () => emit('close')

onMounted(() => {
  if (props.open) chargerSiBesoin()
})
</script>

<template>
  <Teleport to="body">
    <div
      v-if="open"
      class="fixed inset-0 z-[100] flex items-start justify-center bg-black/50 backdrop-blur-sm overflow-y-auto py-8 px-4"
      @click.self="fermer"
    >
      <div class="bg-white rounded-2xl shadow-2xl w-full max-w-3xl my-auto">
        <!-- En-tête -->
        <header class="flex items-center justify-between px-6 py-4 border-b border-gray-200">
          <div>
            <h2 class="text-lg font-bold text-gray-900">Proposer une salle Afrolang</h2>
            <p class="text-xs text-gray-500 mt-0.5">
              Pour une langue africaine et son groupe ethnique
            </p>
          </div>
          <button
            type="button"
            class="text-gray-400 hover:text-gray-600 transition-colors"
            aria-label="Fermer"
            @click="fermer"
          >
            <font-awesome-icon :icon="['fas', 'xmark']" class="w-5 h-5" />
          </button>
        </header>

        <!-- Onglets -->
        <div class="flex border-b border-gray-200 px-6">
          <button
            type="button"
            class="px-4 py-3 text-sm font-medium border-b-2 transition-colors"
            :class="onglet === 'proposer'
              ? 'border-custom-chocolat text-custom-chocolat'
              : 'border-transparent text-gray-500 hover:text-gray-700'"
            @click="onglet = 'proposer'"
          >
            <font-awesome-icon :icon="['fas', 'lightbulb']" class="w-3.5 h-3.5 mr-1.5" />
            Soumettre
          </button>
          <button
            type="button"
            class="px-4 py-3 text-sm font-medium border-b-2 transition-colors"
            :class="onglet === 'mes-propositions'
              ? 'border-custom-chocolat text-custom-chocolat'
              : 'border-transparent text-gray-500 hover:text-gray-700'"
            @click="onglet = 'mes-propositions'; rechargerListe()"
          >
            <font-awesome-icon :icon="['fas', 'list']" class="w-3.5 h-3.5 mr-1.5" />
            Mes propositions
            <span v-if="total > 0" class="ml-1.5 inline-flex items-center justify-center min-w-[18px] h-[18px] px-1 rounded-full text-[10px] bg-gray-100 text-gray-700">
              {{ total }}
            </span>
          </button>
        </div>

        <!-- Contenu -->
        <div class="p-6 max-h-[70vh] overflow-y-auto">
          <section v-if="onglet === 'proposer'">
            <AfrolangPropositionSalleForm
              :groupes-disponibles="groupesDisponibles"
              :territoires="territoiresDisponibles"
              @soumis="ajouterProposition"
            />
          </section>

          <section v-else>
            <header class="flex items-center justify-between mb-4">
              <h3 class="text-sm font-semibold text-gray-900">Suivi de mes soumissions</h3>
              <select
                v-model="filtreStatut"
                class="rounded-lg border border-gray-300 bg-white px-3 py-1.5 text-xs text-gray-700 focus:border-orange-500 focus:ring-2 focus:ring-orange-200 focus:outline-none"
                @change="aller(1)"
              >
                <option value="">Tous les statuts</option>
                <option value="en_attente">En attente</option>
                <option value="validee">Validées</option>
                <option value="rejetee">Rejetées</option>
                <option value="retiree">Retirées</option>
              </select>
            </header>

            <div v-if="chargementListe" class="text-sm text-gray-500 flex items-center gap-2">
              <font-awesome-icon :icon="['fas', 'spinner']" class="w-4 h-4 animate-spin" />
              Chargement…
            </div>
            <div v-else-if="!propositions.length" class="rounded-xl border border-dashed border-gray-300 bg-gray-50 px-6 py-10 text-center">
              <font-awesome-icon :icon="['fas', 'inbox']" class="w-8 h-8 text-gray-400 mb-3" />
              <p class="text-sm text-gray-600">Aucune proposition pour le moment.</p>
            </div>
            <div v-else class="grid grid-cols-1 gap-3">
              <AfrolangPropositionSalleCard
                v-for="p in propositions"
                :key="p.id"
                :proposition="p"
                @retiree="mettreAJourProposition"
              />
            </div>

            <div v-if="totalPages > 1" class="mt-6 flex items-center justify-center gap-2 text-sm">
              <button
                type="button"
                :disabled="page <= 1"
                class="rounded-lg border border-gray-300 bg-white px-3 py-1.5 text-gray-700 hover:bg-gray-50 disabled:opacity-50"
                @click="aller(page - 1)"
              >
                <font-awesome-icon :icon="['fas', 'chevron-left']" class="w-3 h-3" />
              </button>
              <span class="text-gray-600">Page {{ page }} / {{ totalPages }}</span>
              <button
                type="button"
                :disabled="page >= totalPages"
                class="rounded-lg border border-gray-300 bg-white px-3 py-1.5 text-gray-700 hover:bg-gray-50 disabled:opacity-50"
                @click="aller(page + 1)"
              >
                <font-awesome-icon :icon="['fas', 'chevron-right']" class="w-3 h-3" />
              </button>
            </div>
          </section>
        </div>
      </div>
    </div>
  </Teleport>
</template>
