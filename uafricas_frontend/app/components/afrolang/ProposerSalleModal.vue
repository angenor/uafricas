<script setup lang="ts">
// Modale « Proposer une salle » + suivi de mes propositions.
// Feature 001-admin-salles-publiques, US1, refactor en modale (UX choix).
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
  <AfricansModale
    :model-value="open"
    titre="Proposer une salle Afrolang"
    sous-titre="Pour une langue africaine et son groupe ethnique"
    icone="fa-solid fa-lightbulb"
    taille="large"
    @update:model-value="fermer()"
  >
    <!-- Onglets -->
    <div class="-mt-2 mb-6 flex border-b border-af-bordure">
      <button
        type="button"
        class="flex items-center gap-2 border-b-2 px-4 py-3 text-base font-bold transition"
        :class="onglet === 'proposer'
          ? 'border-af-chocolat text-af-chocolat'
          : 'border-transparent text-af-atone hover:text-af-corps'"
        @click="onglet = 'proposer'"
      >
        <font-awesome-icon icon="fa-solid fa-lightbulb" />
        Soumettre
      </button>
      <button
        type="button"
        class="flex items-center gap-2 border-b-2 px-4 py-3 text-base font-bold transition"
        :class="onglet === 'mes-propositions'
          ? 'border-af-chocolat text-af-chocolat'
          : 'border-transparent text-af-atone hover:text-af-corps'"
        @click="onglet = 'mes-propositions'; rechargerListe()"
      >
        <font-awesome-icon icon="fa-solid fa-list" />
        Mes propositions
        <span
          v-if="total > 0"
          class="grid h-[18px] min-w-[18px] place-items-center rounded-full bg-af-fond px-1 text-[10px] text-af-corps"
        >
          {{ total }}
        </span>
      </button>
    </div>

    <div class="max-h-[60vh] overflow-y-auto">
      <section v-if="onglet === 'proposer'">
        <AfrolangPropositionSalleForm
          :groupes-disponibles="groupesDisponibles"
          :territoires="territoiresDisponibles"
          @soumis="ajouterProposition"
        />
      </section>

      <section v-else>
        <header class="mb-4 flex items-center justify-between gap-4">
          <h3 class="text-base font-bold text-af-encre">Suivi de mes soumissions</h3>
          <select
            v-model="filtreStatut"
            class="h-10 rounded-md border border-af-bordure bg-white px-3 text-[14px]/[1.4] text-af-corps focus:border-af-chocolat focus:outline-none"
            @change="aller(1)"
          >
            <option value="">Tous les statuts</option>
            <option value="en_attente">En attente</option>
            <option value="validee">Validées</option>
            <option value="rejetee">Rejetées</option>
            <option value="retiree">Retirées</option>
          </select>
        </header>

        <div v-if="chargementListe" class="flex items-center gap-2 text-[14px]/[1.4] text-af-atone">
          <font-awesome-icon icon="fa-solid fa-spinner" class="animate-spin" />
          Chargement…
        </div>
        <div
          v-else-if="!propositions.length"
          class="rounded-lg border border-dashed border-af-bordure bg-af-fond px-6 py-10 text-center"
        >
          <font-awesome-icon icon="fa-solid fa-inbox" class="mb-3 text-3xl text-af-atone-2" />
          <p class="text-[14px]/[1.4] text-af-corps">Aucune proposition pour le moment.</p>
        </div>
        <div v-else class="grid gap-3">
          <AfrolangPropositionSalleCard
            v-for="p in propositions"
            :key="p.id"
            :proposition="p"
            @retiree="mettreAJourProposition"
          />
        </div>

        <div v-if="totalPages > 1" class="mt-6 flex items-center justify-center gap-3 text-[14px]/[1.4]">
          <button
            type="button"
            :disabled="page <= 1"
            class="grid size-9 place-items-center rounded-md border border-af-bordure bg-white text-af-corps transition hover:border-af-chocolat disabled:opacity-40"
            @click="aller(page - 1)"
          >
            <font-awesome-icon icon="fa-solid fa-chevron-left" />
          </button>
          <span class="text-af-corps">Page {{ page }} / {{ totalPages }}</span>
          <button
            type="button"
            :disabled="page >= totalPages"
            class="grid size-9 place-items-center rounded-md border border-af-bordure bg-white text-af-corps transition hover:border-af-chocolat disabled:opacity-40"
            @click="aller(page + 1)"
          >
            <font-awesome-icon icon="fa-solid fa-chevron-right" />
          </button>
        </div>
      </section>
    </div>
  </AfricansModale>
</template>
