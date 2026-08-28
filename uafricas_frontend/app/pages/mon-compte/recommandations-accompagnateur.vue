<template>
  <NuxtLayout name="africans">
    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[{ libelle: 'Mon compte', vers: '/mon-compte/profil' }, { libelle: 'Recommandations' }]"
      />
    </template>

    <div class="flex flex-col gap-6">
      <header>
        <h1 class="text-[24px]/[1.3] font-bold text-af-encre">Mes recommandations d'accompagnateur</h1>
        <p class="mt-1 text-[14px]/[1.5] text-af-corps">Vous avez été recommandé(e) pour accompagner certaines salles Afrolang. Acceptez ou refusez chaque proposition.</p>
      </header>
      <!-- Filtres -->
      <div class="bg-white rounded-2xl shadow-sm border border-af-bordure p-4 mb-6 flex flex-wrap gap-2">
        <button v-for="f in filtres"
                :key="f.code ?? 'tous'"
                type="button"
                class="px-3 py-1.5 text-sm rounded-full border transition-colors"
                :class="statutFiltre === f.code
                  ? 'bg-af-chocolat text-white border-af-chocolat'
                  : 'bg-white text-af-corps border-af-bordure hover:border-af-chocolat hover:text-af-chocolat'"
                @click="statutFiltre = f.code; page = 1; recharger()">
          {{ f.libelle }}
        </button>
      </div>

      <!-- Liste -->
      <div v-if="chargement" class="text-center py-12 text-sm text-af-atone">
        <font-awesome-icon icon="fa-solid fa-spinner" class="animate-spin text-2xl text-af-chocolat mb-3" />
        <p>Chargement…</p>
      </div>
      <div v-else-if="recommandations.length === 0" class="text-center py-12 text-sm text-af-atone bg-white rounded-2xl border border-af-bordure">
        <font-awesome-icon icon="fa-regular fa-envelope-open" class="text-4xl text-af-atone-2 mb-3" />
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
                class="px-3 py-1.5 text-xs rounded-md border border-af-bordure disabled:opacity-40 hover:bg-af-fond"
                @click="page = Math.max(1, page - 1); recharger()">
          Précédent
        </button>
        <span class="text-xs text-af-corps">Page {{ page }} / {{ totalPages }}</span>
        <button type="button"
                :disabled="page >= totalPages"
                class="px-3 py-1.5 text-xs rounded-md border border-af-bordure disabled:opacity-40 hover:bg-af-fond"
                @click="page = Math.min(totalPages, page + 1); recharger()">
          Suivant
        </button>
      </div>

      <div v-if="erreur" class="mt-4 text-sm text-af-live bg-af-live/5 border border-af-live/30 rounded-md px-3 py-2">
        {{ erreur }}
      </div>
    </div>

    <template #rail>
      <ComptePanneauNavigation />
    </template>
  </NuxtLayout>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useUserStore } from '~/stores/user'
import {
  useAfrolangAccompagnateur,
  type RecommandationRecueAPI,
} from '~/composables/useAfrolangAccompagnateur'
import type { StatutAccompagnateur } from '~/composables/useAfrolangRessources'

definePageMeta({ layout: false, middleware: 'auth' })
useHead({ title: 'Recommandations accompagnateur | AfricanS' })

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
