<script setup lang="ts">
// Panneau de la communauté : affiche les ressources contribuées d'une salle
// (feature 001-ressources-fermeture-session, US1). Tailwind v4 pur.
import { computed, onMounted, ref, watch } from 'vue'
import { useUserStore } from '~/stores/user'
import type { SalleAPI } from '~/composables/useAfrolang'
import { useAfrolang } from '~/composables/useAfrolang'
import { useAfrolangRessources } from '~/composables/useAfrolangRessources'
import type {
  RessourceContribueeAPI,
  TypeRessourceContribuee,
} from '~/composables/useAfrolangRessources'
import RessourceContribueeCard from './RessourceContribueeCard.vue'
import RessourceContribueeForm from './RessourceContribueeForm.vue'

const props = withDefaults(defineProps<{
  salleId: string
  sessionId?: string | null
  /** Salle (incl. desactivee_admin) pour décider l'autorisation d'écriture. */
  salle?: SalleAPI | null
  /** Si la salle est privée, indique que l'utilisateur en a déjà l'accès.
   *  Par défaut `true` : le backend re-valide les invariants côté serveur. */
  aAccesSalleSiPrivee?: boolean
}>(), {
  sessionId: null,
  salle: null,
  aAccesSalleSiPrivee: true,
})

const userStore = useUserStore()
const { peutContribuerRessource } = useAfrolang()
const { listerRessourcesContribuees, supprimerRessource, chargement, erreur } = useAfrolangRessources()

const ressources = ref<RessourceContribueeAPI[]>([])
const totalPages = ref(1)
const page = ref(1)
const typeFiltre = ref<TypeRessourceContribuee | null>(null)
const formOuvert = ref(false)

/**
 * Affichage du bouton « Contribuer » :
 *   - utilisateur authentifié (token présent)
 *   - salle (si chargée) non désactivée par l'administration
 *   - accès à la salle privée explicite (FALSE), sinon on autorise par défaut
 * Le backend re-valide tous les invariants (rate-limit, état utilisateur,
 * salle existante, désactivation). La logique FE n'est qu'un hint UX.
 */
const peutContribuer = computed(() => {
  if (!userStore.accessToken) return false
  if (props.salle?.desactivee_admin) return false
  if (props.aAccesSalleSiPrivee === false) return false
  return true
})

const motifBlocage = computed(() => {
  if (!userStore.accessToken) return 'Connectez-vous pour contribuer.'
  if (props.salle?.desactivee_admin) return 'Salle gelée par administration, contribution suspendue.'
  if (props.aAccesSalleSiPrivee === false) return 'Accès à la salle privée requis.'
  return null
})

const recharger = async () => {
  const data = await listerRessourcesContribuees(props.salleId, {
    page: page.value,
    par_page: 20,
    type: typeFiltre.value ?? undefined,
  })
  if (data) {
    ressources.value = data.data
    totalPages.value = data.total_pages
  }
}

watch([page, typeFiltre], () => { recharger() })
watch(() => props.salleId, () => { page.value = 1; recharger() })

onMounted(() => { recharger() })

const onSupprime = async (id: string) => {
  const ok = await supprimerRessource(id)
  if (ok) recharger()
}

const choixTypes: { code: TypeRessourceContribuee | null; libelle: string }[] = [
  { code: null, libelle: 'Tous' },
  { code: 'document', libelle: 'Documents' },
  { code: 'video_youtube', libelle: 'Vidéos' },
  { code: 'lien_web', libelle: 'Liens' },
  { code: 'accompagnateur', libelle: 'Accompagnateurs' },
]
</script>

<template>
  <section class="bg-gradient-to-br from-amber-50/30 to-white border border-amber-100 rounded-2xl p-5 my-6">
    <header class="flex items-start justify-between gap-3 mb-4">
      <div>
        <h3 class="text-lg font-semibold text-gray-900 flex items-center gap-2">
          <font-awesome-icon icon="fa-solid fa-users" class="text-custom-chocolat" />
          Ressources pertinentes pour apprendre la langue ajoutées par la communauté.
        </h3>
        <p class="text-xs text-gray-600 mt-1">Dictionnaire, alphabet, vocabulaire, phonétique, cours, vidéos, etc.</p>
      </div>
      <button v-if="peutContribuer"
              type="button"
              class="inline-flex items-center gap-2 px-3 py-2 bg-custom-chocolat text-white text-sm rounded-md hover:bg-custom-chocolat/90 transition-colors shrink-0"
              @click="formOuvert = true">
        <font-awesome-icon icon="fa-solid fa-plus" class="text-xs" />
        <span class="hidden sm:inline">Ajouter une ressource</span>
      </button>
      <div v-else-if="motifBlocage"
           class="text-xs text-amber-700 bg-amber-50 border border-amber-200 rounded-md px-3 py-2 max-w-xs">
        {{ motifBlocage }}
      </div>
    </header>

    <!-- Filtres par type -->
    <div class="flex flex-wrap gap-1.5 mb-4">
      <button v-for="t in choixTypes"
              :key="t.code ?? 'tous'"
              type="button"
              class="px-3 py-1 text-xs rounded-full border transition-colors"
              :class="typeFiltre === t.code
                ? 'bg-custom-chocolat text-white border-custom-chocolat'
                : 'bg-white text-gray-700 border-gray-200 hover:border-custom-chocolat hover:text-custom-chocolat'"
              @click="typeFiltre = t.code; page = 1">
        {{ t.libelle }}
      </button>
    </div>

    <!-- Liste -->
    <div v-if="chargement && ressources.length === 0" class="text-center py-8 text-sm text-gray-500">
      <font-awesome-icon icon="fa-solid fa-spinner" class="animate-spin text-2xl text-custom-chocolat mb-2" />
      <p>Chargement…</p>
    </div>
    <div v-else-if="ressources.length === 0" class="text-center py-8 text-sm text-gray-500">
      <font-awesome-icon icon="fa-regular fa-folder-open" class="text-3xl text-gray-300 mb-2" />
      <p>Aucune ressource contribuée pour le moment.</p>
    </div>
    <ul v-else class="flex flex-col gap-3">
      <li v-for="r in ressources" :key="r.id">
        <RessourceContribueeCard
          :ressource="r"
          :utilisateur-id="userStore.user?.id ?? null"
          @supprime="onSupprime" />
      </li>
    </ul>

    <!-- Pagination -->
    <div v-if="totalPages > 1" class="flex items-center justify-center gap-2 mt-4">
      <button type="button"
              :disabled="page <= 1"
              class="px-3 py-1 text-xs rounded-md border border-gray-200 disabled:opacity-40 hover:bg-gray-50"
              @click="page = Math.max(1, page - 1)">
        Précédent
      </button>
      <span class="text-xs text-gray-600">Page {{ page }} / {{ totalPages }}</span>
      <button type="button"
              :disabled="page >= totalPages"
              class="px-3 py-1 text-xs rounded-md border border-gray-200 disabled:opacity-40 hover:bg-gray-50"
              @click="page = Math.min(totalPages, page + 1)">
        Suivant
      </button>
    </div>

    <!-- Erreur globale -->
    <div v-if="erreur" class="mt-3 text-sm text-red-700 bg-red-50 border border-red-200 rounded-md px-3 py-2">
      {{ erreur }}
    </div>

    <!-- Modal -->
    <RessourceContribueeForm
      :ouvert="formOuvert"
      :salle-id="salleId"
      :session-id="sessionId ?? null"
      @fermer="formOuvert = false"
      @ajoutee="recharger" />
  </section>
</template>
