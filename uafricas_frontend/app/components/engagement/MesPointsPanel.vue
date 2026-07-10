<script setup lang="ts">
// Panneau « Mes points / mon statut / mes badges » — Tailwind pur
import { ref, computed, onMounted } from 'vue'
import {
  useEngagement,
  type CompteEngagement,
  type MouvementPoints,
} from '~/composables/useEngagement'

const { obtenirMonCompte, listerMonJournal } = useEngagement()

const compte = ref<CompteEngagement | null>(null)
const mouvements = ref<MouvementPoints[]>([])
const total = ref(0)
const page = ref(1)
const taille = 20
const chargement = ref(true)
const erreur = ref('')

const chargerCompte = async () => {
  try {
    compte.value = await obtenirMonCompte()
  } catch {
    erreur.value = 'Impossible de charger votre compte d\'engagement.'
  }
}

const chargerJournal = async (p = 1) => {
  try {
    const res = await listerMonJournal(p, taille)
    if (res) {
      mouvements.value = res.elements
      total.value = res.total
      page.value = res.page
    }
  } catch {
    // journal facultatif — silencieux
  }
}

onMounted(async () => {
  chargement.value = true
  await Promise.all([chargerCompte(), chargerJournal(1)])
  chargement.value = false
})

const totalPages = computed(() => Math.max(1, Math.ceil(total.value / taille)))

const changerPage = async (p: number) => {
  if (p < 1 || p > totalPages.value) return
  await chargerJournal(p)
}

// Progression vers le prochain niveau (barre)
const progression = computed(() => {
  if (!compte.value) return 0
  const prochain = compte.value.prochain_niveau
  if (!prochain) return 100
  const base = compte.value.niveau.seuil_min
  const cible = prochain.seuil_min
  const solde = compte.value.solde_points
  if (cible <= base) return 100
  return Math.min(100, Math.round(((solde - base) / (cible - base)) * 100))
})

const formaterDate = (iso: string) =>
  new Date(iso).toLocaleDateString('fr-FR', { day: '2-digit', month: 'short', year: 'numeric' })

const signe = (n: number) => (n > 0 ? `+${n}` : `${n}`)
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <h2 class="text-lg font-semibold text-gray-800">Mes points</h2>
    </div>

    <div v-if="chargement" class="py-10 text-center text-gray-400">
      <font-awesome-icon icon="fa-solid fa-spinner" class="animate-spin" /> Chargement…
    </div>

    <p v-else-if="erreur" class="text-sm text-red-600">{{ erreur }}</p>

    <template v-else-if="compte">
      <!-- Cartes solde / statut / réputation -->
      <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
        <div class="rounded-2xl border border-gray-100 bg-gradient-to-br from-custom-green/5 to-custom-green/10 p-5">
          <p class="text-xs uppercase tracking-wide text-gray-500">Solde de points</p>
          <p class="mt-1 text-3xl font-bold text-custom-green">{{ compte.solde_points }}</p>
          <p class="text-xs text-gray-400 mt-1">{{ compte.solde_points_mensuel }} ce mois-ci</p>
        </div>

        <div class="rounded-2xl border border-gray-100 bg-white p-5 flex flex-col justify-between">
          <p class="text-xs uppercase tracking-wide text-gray-500">Mon statut</p>
          <div class="mt-2">
            <EngagementBadgeStatut :niveau="compte.niveau" />
          </div>
        </div>

        <div class="rounded-2xl border border-gray-100 bg-white p-5">
          <p class="text-xs uppercase tracking-wide text-gray-500">Réputation</p>
          <p class="mt-1 text-3xl font-bold text-custom-chocolat">{{ compte.reputation }}</p>
          <p class="text-xs text-gray-400 mt-1">Score de confiance</p>
        </div>
      </div>

      <!-- Progression vers le prochain niveau -->
      <div v-if="compte.prochain_niveau" class="rounded-2xl border border-gray-100 bg-white p-5">
        <div class="flex items-center justify-between text-sm">
          <span class="text-gray-600">
            Prochain palier : <span class="font-semibold">{{ compte.prochain_niveau.libelle }}</span>
          </span>
          <span class="text-gray-400">
            encore {{ compte.prochain_niveau.points_restants }} pts
          </span>
        </div>
        <div class="mt-3 h-2.5 w-full rounded-full bg-gray-100 overflow-hidden">
          <div class="h-full rounded-full bg-custom-green transition-all" :style="{ width: progression + '%' }"></div>
        </div>
      </div>

      <!-- Historique -->
      <div class="rounded-2xl border border-gray-100 bg-white overflow-hidden">
        <div class="px-5 py-3 border-b border-gray-100">
          <h3 class="text-sm font-semibold text-gray-700">Historique des points</h3>
        </div>

        <p v-if="mouvements.length === 0" class="px-5 py-8 text-center text-sm text-gray-400">
          Aucun mouvement pour l'instant. Contribuez pour gagner vos premiers points !
        </p>

        <ul v-else class="divide-y divide-gray-50">
          <li v-for="m in mouvements" :key="m.id" class="flex items-center justify-between px-5 py-3">
            <div class="min-w-0">
              <p class="text-sm font-medium text-gray-800 truncate">
                {{ m.libelle || m.type_action }}
                <span v-if="m.plafond_atteint" class="ml-1 text-[10px] text-amber-600">(plafond atteint)</span>
              </p>
              <p class="text-xs text-gray-400">{{ formaterDate(m.created_at) }}</p>
            </div>
            <div class="text-right shrink-0 pl-3">
              <span
                class="text-sm font-bold"
                :class="m.points >= 0 ? 'text-custom-green' : 'text-red-600'"
              >{{ signe(m.points) }}</span>
              <span v-if="m.reputation_delta !== 0" class="block text-[11px] text-custom-chocolat">
                réputation {{ signe(m.reputation_delta) }}
              </span>
            </div>
          </li>
        </ul>

        <div v-if="totalPages > 1" class="flex items-center justify-between px-5 py-3 border-t border-gray-100">
          <button
            class="text-sm text-gray-500 disabled:opacity-40 hover:text-gray-800"
            :disabled="page <= 1"
            @click="changerPage(page - 1)"
          >
            <font-awesome-icon icon="fa-solid fa-chevron-left" /> Précédent
          </button>
          <span class="text-xs text-gray-400">Page {{ page }} / {{ totalPages }}</span>
          <button
            class="text-sm text-gray-500 disabled:opacity-40 hover:text-gray-800"
            :disabled="page >= totalPages"
            @click="changerPage(page + 1)"
          >
            Suivant <font-awesome-icon icon="fa-solid fa-chevron-right" />
          </button>
        </div>
      </div>
    </template>
  </div>
</template>
