<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import {
  useAfrolang,
  type GroupeEthniqueResume,
} from '~/composables/useAfrolang'

const props = defineProps<{
  limit?: number
}>()

const emit = defineEmits<{
  (e: 'rejoindre-salle', payload: { salleId: string }): void
  (e: 'proposer-salle', payload: { nomGroupeEthnique: string }): void
}>()

const { listerGroupesEthniques } = useAfrolang()

const recherche = ref('')
const page = ref(1)
const parPage = ref(props.limit ?? 24)
const total = ref(0)
const totalPages = ref(1)
const groupes = ref<GroupeEthniqueResume[]>([])
const chargement = ref(false)
const erreur = ref<string | null>(null)

const rechercheDebounced = ref('')
let timer: ReturnType<typeof setTimeout> | null = null
watch(recherche, (val) => {
  if (timer) clearTimeout(timer)
  timer = setTimeout(() => {
    rechercheDebounced.value = val.trim()
    page.value = 1
  }, 250)
})

const charger = async () => {
  chargement.value = true
  erreur.value = null
  try {
    const resp = await listerGroupesEthniques({
      q: rechercheDebounced.value || undefined,
      page: page.value,
      par_page: parPage.value,
    })
    if (resp) {
      groupes.value = resp.groupes
      total.value = resp.total
      totalPages.value = resp.total_pages
    } else {
      groupes.value = []
      total.value = 0
      totalPages.value = 1
    }
  } catch (e: unknown) {
    erreur.value = e instanceof Error ? e.message : 'Erreur de chargement'
  } finally {
    chargement.value = false
  }
}

watch([page, rechercheDebounced], () => {
  charger()
})

onMounted(() => {
  charger()
})

const aucunResultat = computed(
  () => !chargement.value && groupes.value.length === 0,
)

const onClickGroupe = (g: GroupeEthniqueResume) => {
  if (g.salle_id && g.salle_active) {
    emit('rejoindre-salle', { salleId: g.salle_id })
  }
}

const onClickProposer = (nom: string) => {
  emit('proposer-salle', { nomGroupeEthnique: nom })
}

const badgeEtat = (g: GroupeEthniqueResume): {
  label: string
  classe: string
} => {
  if (g.salle_active) {
    return {
      label: 'Salle active',
      classe: 'bg-green-100 text-green-700 border-green-200',
    }
  }
  if (g.proposition_en_attente) {
    return {
      label: 'Proposition en attente',
      classe: 'bg-amber-100 text-amber-700 border-amber-200',
    }
  }
  return {
    label: 'Pas encore de salle',
    classe: 'bg-gray-100 text-gray-600 border-gray-200',
  }
}

const pagePrecedente = () => {
  if (page.value > 1) page.value -= 1
}
const pageSuivante = () => {
  if (page.value < totalPages.value) page.value += 1
}
</script>

<template>
  <div class="w-full">
    <div class="flex flex-col md:flex-row md:items-center md:justify-between gap-4 mb-6">
      <div>
        <h2 class="text-2xl font-bold text-gray-900">Annuaire des groupes ethniques</h2>
        <p class="text-sm text-gray-600 mt-1">
          Rejoignez la salle publique correspondant à votre groupe ou proposez-en une nouvelle.
        </p>
      </div>
      <div class="relative w-full md:w-80">
        <font-awesome-icon
          :icon="['fas', 'magnifying-glass']"
          class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 w-4 h-4"
        />
        <input
          v-model="recherche"
          type="text"
          placeholder="Rechercher un groupe..."
          class="w-full pl-10 pr-3 py-2 border border-gray-200 rounded-lg focus:outline-hidden focus:ring-2 focus:ring-blue-500 focus:border-transparent"
        />
      </div>
    </div>

    <div
      v-if="erreur"
      class="mb-4 px-4 py-3 bg-red-50 border border-red-200 rounded-lg text-red-700 text-sm"
    >
      {{ erreur }}
    </div>

    <div v-if="chargement" class="text-gray-500 py-12 text-center">
      <font-awesome-icon :icon="['fas', 'circle-notch']" class="animate-spin w-5 h-5 mr-2" />
      Chargement…
    </div>

    <div
      v-else-if="aucunResultat"
      class="py-12 text-center bg-white rounded-xl border border-gray-200"
    >
      <font-awesome-icon :icon="['fas', 'users-viewfinder']" class="w-10 h-10 text-gray-300 mb-3" />
      <p class="text-gray-700 font-medium mb-1">Aucun groupe ethnique trouvé</p>
      <p class="text-sm text-gray-500 mb-4">
        Essayez un autre terme ou proposez une nouvelle salle.
      </p>
      <button
        v-if="rechercheDebounced"
        class="inline-flex items-center gap-2 px-4 py-2 bg-custom-chocolat text-white rounded-lg hover:opacity-90 transition"
        @click="onClickProposer(rechercheDebounced)"
      >
        <font-awesome-icon :icon="['fas', 'plus']" class="w-4 h-4" />
        Proposer « {{ rechercheDebounced }} »
      </button>
    </div>

    <div v-else class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
      <article
        v-for="g in groupes"
        :key="g.id"
        class="bg-white border border-gray-200 rounded-xl p-5 flex flex-col gap-3 hover:shadow-md transition"
      >
        <div class="flex items-start justify-between gap-2">
          <h3 class="font-semibold text-gray-900 leading-tight">{{ g.nom }}</h3>
          <span
            class="text-xs px-2 py-1 rounded-full border"
            :class="badgeEtat(g).classe"
          >
            {{ badgeEtat(g).label }}
          </span>
        </div>

        <div class="text-xs text-gray-500">
          <font-awesome-icon :icon="['fas', 'location-dot']" class="w-3 h-3 mr-1" />
          {{ g.pays_nom || 'Pays inconnu' }}
        </div>

        <div class="mt-auto pt-2 flex gap-2">
          <button
            v-if="g.salle_active && g.salle_id"
            class="flex-1 px-3 py-2 text-sm bg-custom-green text-white rounded-lg hover:opacity-90 transition"
            @click="onClickGroupe(g)"
          >
            Rejoindre la salle
          </button>
          <button
            v-else
            class="flex-1 px-3 py-2 text-sm border border-custom-chocolat text-custom-chocolat rounded-lg hover:bg-custom-chocolat hover:text-white transition"
            :disabled="g.proposition_en_attente"
            @click="onClickProposer(g.nom)"
          >
            {{ g.proposition_en_attente ? 'En attente' : 'Proposer une salle' }}
          </button>
        </div>
      </article>
    </div>

    <!-- Pagination -->
    <div
      v-if="totalPages > 1"
      class="mt-6 flex items-center justify-between text-sm text-gray-600"
    >
      <p>
        Page {{ page }} / {{ totalPages }}
        <span class="text-gray-400">({{ total }} groupe(s))</span>
      </p>
      <div class="flex gap-2">
        <button
          class="px-3 py-2 border border-gray-200 rounded-lg disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-50"
          :disabled="page <= 1"
          @click="pagePrecedente"
        >
          Précédent
        </button>
        <button
          class="px-3 py-2 border border-gray-200 rounded-lg disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-50"
          :disabled="page >= totalPages"
          @click="pageSuivante"
        >
          Suivant
        </button>
      </div>
    </div>
  </div>
</template>
