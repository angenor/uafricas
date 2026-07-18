<template>
  <div class="min-h-screen bg-gradient-to-br from-gray-50 via-white to-gray-50 pt-28 pb-16">
    <div class="max-w-3xl mx-auto px-4">
      <!-- En-tête -->
      <header class="mb-8">
        <h1 class="text-3xl font-bold text-gray-800 font-display">Mes recommandations d'accompagnateur</h1>
        <p class="mt-2 text-gray-600 text-sm">
          Vous avez été recommandé(e) pour accompagner certaines salles Afrolang. Acceptez ou refusez chaque proposition.
        </p>
      </header>

      <!-- Filtres -->
      <div class="bg-white rounded-2xl shadow-sm border border-gray-100 p-4 mb-6 flex flex-wrap gap-2">
        <button v-for="f in filtres"
                :key="f.code ?? 'tous'"
                type="button"
                class="px-3 py-1.5 text-sm rounded-full border transition-colors"
                :class="statutFiltre === f.code
                  ? 'bg-custom-chocolat text-white border-custom-chocolat'
                  : 'bg-white text-gray-700 border-gray-200 hover:border-custom-chocolat hover:text-custom-chocolat'"
                @click="statutFiltre = f.code; page = 1; recharger()">
          {{ f.libelle }}
        </button>
      </div>

      <!-- Liste -->
      <div v-if="chargement" class="text-center py-12 text-sm text-gray-500">
        <font-awesome-icon icon="fa-solid fa-spinner" class="animate-spin text-2xl text-custom-chocolat mb-3" />
        <p>Chargement…</p>
      </div>
      <div v-else-if="recommandations.length === 0" class="text-center py-12 text-sm text-gray-500 bg-white rounded-2xl border border-gray-100">
        <font-awesome-icon icon="fa-regular fa-envelope-open" class="text-4xl text-gray-300 mb-3" />
        <p>Aucune recommandation à afficher.</p>
      </div>
      <div v-else>
        <AfrolangAccompagnateurRecommandationBanner
          v-for="r in recommandations"
          :key="r.id"
          :recommandation="r"
          @actualisee="recharger" />
      </div>

      <!-- Pagination -->
      <div v-if="totalPages > 1" class="flex items-center justify-center gap-2 mt-6">
        <button type="button"
                :disabled="page <= 1"
                class="px-3 py-1.5 text-xs rounded-md border border-gray-200 disabled:opacity-40 hover:bg-gray-50"
                @click="page = Math.max(1, page - 1); recharger()">
          Précédent
        </button>
        <span class="text-xs text-gray-600">Page {{ page }} / {{ totalPages }}</span>
        <button type="button"
                :disabled="page >= totalPages"
                class="px-3 py-1.5 text-xs rounded-md border border-gray-200 disabled:opacity-40 hover:bg-gray-50"
                @click="page = Math.min(totalPages, page + 1); recharger()">
          Suivant
        </button>
      </div>

      <div v-if="erreur" class="mt-4 text-sm text-red-700 bg-red-50 border border-red-200 rounded-md px-3 py-2">
        {{ erreur }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useUserStore } from '~/stores/user'
import {
  useAfrolangAccompagnateur,
  type RecommandationRecueAPI,
} from '~/composables/useAfrolangAccompagnateur'
import type { StatutAccompagnateur } from '~/composables/useAfrolangRessources'

definePageMeta({ middleware: 'auth' })
useHead({ title: 'Recommandations accompagnateur — AfricanS' })

const userStore = useUserStore()
const { redirigerVersConnexion } = useAuth()
const { listerRecommandationsRecues, rafraichirCompteur, chargement, erreur } = useAfrolangAccompagnateur()

const recommandations = ref<RecommandationRecueAPI[]>([])
const page = ref(1)
const totalPages = ref(1)
const statutFiltre = ref<StatutAccompagnateur | null>(null)

const filtres: { code: StatutAccompagnateur | null; libelle: string }[] = [
  { code: null, libelle: 'Toutes' },
  { code: 'en_attente', libelle: 'En attente' },
  { code: 'acceptee', libelle: 'Acceptées' },
  { code: 'refusee', libelle: 'Refusées' },
  { code: 'retiree', libelle: 'Retirées' },
]

const recharger = async () => {
  const data = await listerRecommandationsRecues({
    page: page.value,
    par_page: 10,
    statut: statutFiltre.value ?? undefined,
  })
  if (data) {
    recommandations.value = data.data
    totalPages.value = data.total_pages
  }
  await rafraichirCompteur()
}

onMounted(() => {
  if (!userStore.accessToken) {
    redirigerVersConnexion()
    return
  }
  recharger()
})
</script>
