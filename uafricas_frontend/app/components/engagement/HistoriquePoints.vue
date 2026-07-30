<script setup lang="ts">
/**
 * Historique des points — liste paginée, filtres catégorie + période.
 * Tailwind v4 pur (Principe VI).
 *
 * Aucun libellé du barème n'est écrit ici : `libelle` vient de la règle, la
 * catégorie du mouvement lui-même. Le composant charge son propre journal parce
 * que ses filtres et sa pagination lui sont propres.
 */
import { computed, ref, onMounted } from 'vue'
import {
  useEngagement,
  type MouvementPoints,
  type CategorieVentilation,
} from '~/composables/useEngagement'

const props = defineProps<{
  /** Catégories réellement présentes dans le journal (alimente le filtre). */
  categories: CategorieVentilation[]
}>()

const { listerMonJournal } = useEngagement()

const mouvements = ref<MouvementPoints[]>([])
const total = ref(0)
const page = ref(1)
const taille = 20
const chargement = ref(true)

const filtreCategorie = ref('')
const filtreDepuis = ref('')
const filtreJusquA = ref('')

const totalPages = computed(() => Math.max(1, Math.ceil(total.value / taille)))

const charger = async (p = 1) => {
  chargement.value = true
  try {
    const res = await listerMonJournal(p, taille, {
      categorie: filtreCategorie.value || undefined,
      depuis: filtreDepuis.value || undefined,
      jusquA: filtreJusquA.value || undefined,
    })
    if (res) {
      mouvements.value = res.elements
      total.value = res.total
      page.value = res.page
    }
  } catch {
    mouvements.value = []
    total.value = 0
  } finally {
    chargement.value = false
  }
}

/** Tout changement de filtre ramène à la première page. */
const appliquerFiltres = () => charger(1)

const reinitialiser = () => {
  filtreCategorie.value = ''
  filtreDepuis.value = ''
  filtreJusquA.value = ''
  charger(1)
}

const changerPage = (p: number) => {
  if (p < 1 || p > totalPages.value) return
  charger(p)
}

onMounted(() => charger(1))

const auMoinsUnFiltre = computed(
  () => !!(filtreCategorie.value || filtreDepuis.value || filtreJusquA.value),
)

const formaterDate = (iso: string) =>
  new Date(iso).toLocaleDateString('fr-FR', { day: '2-digit', month: 'short', year: 'numeric' })

const signe = (n: number) => (n > 0 ? `+${n}` : `${n}`)

/**
 * Mention d'écrêtage (R14). `plafond_atteint` couvre deux situations très
 * différentes pour le membre — les confondre ferait croire à une perte de points
 * là où il n'y en a pas, ou l'inverse.
 */
const mentionPlafond = (m: MouvementPoints): string | null => {
  if (!m.plafond_atteint) return null
  return m.points > 0
    ? `écrêté à ${m.points} point${m.points > 1 ? 's' : ''} (plafond du jour)`
    : 'plafond atteint, aucun point crédité'
}
</script>

<template>
  <section class="space-y-4">
    <h2 class="font-oswald text-xl font-bold text-gray-900">Historique de mes points</h2>

    <!-- Filtres -->
    <div class="flex flex-wrap items-end gap-3 rounded-2xl border border-gray-100 bg-white p-4">
      <label class="flex min-w-40 flex-1 flex-col gap-1">
        <span class="text-xs font-medium text-gray-500">Catégorie</span>
        <select
          v-model="filtreCategorie"
          class="rounded-lg border border-gray-200 px-3 py-2 text-sm focus:border-custom-green focus:outline-none"
          @change="appliquerFiltres"
        >
          <option value="">Toutes les catégories</option>
          <option v-for="c in props.categories" :key="c.code || 'autres'" :value="c.code || ''">
            {{ c.libelle }}
          </option>
        </select>
      </label>

      <label class="flex flex-col gap-1">
        <span class="text-xs font-medium text-gray-500">Du</span>
        <input
          v-model="filtreDepuis"
          type="date"
          class="rounded-lg border border-gray-200 px-3 py-2 text-sm focus:border-custom-green focus:outline-none"
          @change="appliquerFiltres"
        >
      </label>

      <label class="flex flex-col gap-1">
        <span class="text-xs font-medium text-gray-500">Au</span>
        <input
          v-model="filtreJusquA"
          type="date"
          class="rounded-lg border border-gray-200 px-3 py-2 text-sm focus:border-custom-green focus:outline-none"
          @change="appliquerFiltres"
        >
      </label>

      <button
        v-if="auMoinsUnFiltre"
        type="button"
        class="rounded-lg px-3 py-2 text-sm text-gray-500 transition hover:bg-gray-50 hover:text-gray-800"
        @click="reinitialiser"
      >
        <font-awesome-icon icon="fa-solid fa-xmark" /> Réinitialiser
      </button>
    </div>

    <!-- Liste -->
    <div class="overflow-hidden rounded-2xl border border-gray-100 bg-white">
      <p v-if="chargement" class="px-5 py-10 text-center text-sm text-gray-400">
        <font-awesome-icon icon="fa-solid fa-spinner" class="animate-spin" /> Chargement…
      </p>

      <p v-else-if="mouvements.length === 0" class="px-5 py-10 text-center text-sm text-gray-400">
        {{ auMoinsUnFiltre
          ? 'Aucun mouvement ne correspond à ces filtres.'
          : 'Aucun mouvement pour l\'instant.' }}
      </p>

      <ul v-else class="divide-y divide-gray-50">
        <li
          v-for="m in mouvements"
          :key="m.id"
          class="flex items-start justify-between gap-3 px-5 py-3.5"
        >
          <div class="min-w-0">
            <p class="truncate text-sm font-medium text-gray-800">
              {{ m.libelle || m.type_action }}
            </p>
            <div class="mt-0.5 flex flex-wrap items-center gap-x-2 gap-y-1 text-xs">
              <span class="text-gray-400">{{ formaterDate(m.created_at) }}</span>
              <span
                v-if="m.categorie_libelle"
                class="rounded-full bg-gray-100 px-2 py-0.5 text-gray-600"
              >{{ m.categorie_libelle }}</span>
              <span v-if="mentionPlafond(m)" class="font-medium text-amber-600">
                <font-awesome-icon icon="fa-solid fa-circle-info" />
                {{ mentionPlafond(m) }}
              </span>
            </div>
          </div>

          <div class="shrink-0 pl-2 text-right">
            <span
              class="font-oswald text-base font-bold"
              :class="m.points > 0 ? 'text-custom-green' : m.points < 0 ? 'text-red-600' : 'text-gray-400'"
            >{{ signe(m.points) }}</span>
            <span v-if="m.reputation_delta !== 0" class="block text-[11px] text-custom-chocolat">
              réputation {{ signe(m.reputation_delta) }}
            </span>
            <span class="block text-[11px] text-gray-400">solde {{ m.solde_apres }}</span>
          </div>
        </li>
      </ul>

      <div
        v-if="totalPages > 1"
        class="flex items-center justify-between border-t border-gray-100 px-5 py-3"
      >
        <button
          type="button"
          class="text-sm text-gray-500 transition hover:text-gray-900 disabled:opacity-40"
          :disabled="page <= 1"
          @click="changerPage(page - 1)"
        >
          <font-awesome-icon icon="fa-solid fa-chevron-left" /> Précédent
        </button>
        <span class="text-xs text-gray-400">Page {{ page }} / {{ totalPages }} — {{ total }} mouvement{{ total > 1 ? 's' : '' }}</span>
        <button
          type="button"
          class="text-sm text-gray-500 transition hover:text-gray-900 disabled:opacity-40"
          :disabled="page >= totalPages"
          @click="changerPage(page + 1)"
        >
          Suivant <font-awesome-icon icon="fa-solid fa-chevron-right" />
        </button>
      </div>
    </div>
  </section>
</template>
