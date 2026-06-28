<script setup lang="ts">
import { useOpportuniteAfrique } from '~/composables/useOpportuniteAfrique'
import type {
  RecetteCulinaireAPI,
  SectionAfripulse,
  TypeObjetContribution,
} from '~/composables/useOpportuniteAfrique'

interface Props {
  ficheId: string
  estAuthentifie: boolean
}

const props = defineProps<Props>()

// Section rétractable — repliée par défaut
const replie = ref(true)

type OpenContributionPayload = {
  type_objet_contribution: TypeObjetContribution
  section_afripulse: SectionAfripulse
  type_contribution: 'ajout' | 'edition' | 'suppression'
  target_id?: string
  donnees_actuelles?: Record<string, unknown>
  libelle?: string
}

const emit = defineEmits<{
  (e: 'open-contribution', payload: OpenContributionPayload): void
  (e: 'require-login'): void
}>()

const { listerRecettesCulinaires, resoudreUrlImage } = useOpportuniteAfrique()

const recettes = ref<RecetteCulinaireAPI[]>([])
const chargement = ref(true)

// Filtre par zone (ville) de consommation
const zoneSelectionnee = ref('')

// Liste des zones distinctes (les territoires peuvent être séparés par des virgules)
const zonesDisponibles = computed(() => {
  const ensemble = new Set<string>()
  for (const recette of recettes.value) {
    if (!recette.territoires_consommation) continue
    for (const zone of recette.territoires_consommation.split(',')) {
      const valeur = zone.trim()
      if (valeur) ensemble.add(valeur)
    }
  }
  return Array.from(ensemble).sort((a, b) => a.localeCompare(b, 'fr'))
})

const recettesFiltrees = computed(() => {
  if (!zoneSelectionnee.value) return recettes.value
  return recettes.value.filter((recette) =>
    (recette.territoires_consommation || '')
      .split(',')
      .map((z) => z.trim())
      .includes(zoneSelectionnee.value),
  )
})

// Pagination locale (grille 3 colonnes → 9 par page)
const { page, totalPages, pageItems: recettesPage } = usePaginationLocale(recettesFiltrees, 9)

// Modale de détail
const recetteSelectionnee = ref<RecetteCulinaireAPI | null>(null)
const showDetail = ref(false)

const ouvrirDetail = (recette: RecetteCulinaireAPI) => {
  recetteSelectionnee.value = recette
  showDetail.value = true
}
const fermerDetail = () => {
  showDetail.value = false
}

onMounted(async () => {
  chargement.value = true
  recettes.value = await listerRecettesCulinaires(props.ficheId)
  chargement.value = false
})

const ouvrirContribution = (
  type_contribution: 'ajout' | 'edition' | 'suppression',
  recette?: RecetteCulinaireAPI,
) => {
  if (!props.estAuthentifie) {
    emit('require-login')
    return
  }
  emit('open-contribution', {
    type_objet_contribution: 'recette_culinaire',
    section_afripulse: 'recettes_culinaires',
    type_contribution,
    target_id: recette?.id,
    donnees_actuelles: recette
      ? {
          titre: recette.titre,
          territoires_consommation: recette.territoires_consommation,
          histoire: recette.histoire,
          ingredients: recette.ingredients,
          etapes_preparation: recette.etapes_preparation,
          images: recette.images,
        }
      : undefined,
    libelle: recette?.titre,
  })
}

const proposerRecette = () => ouvrirContribution('ajout')
</script>

<template>
  <section class="bg-amber-50 transition-all" :class="replie ? 'py-5' : 'py-12'">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex items-center justify-between gap-4" :class="replie ? '' : 'mb-8'">
        <button
          type="button"
          class="flex items-center gap-3 text-left min-w-0 cursor-pointer hover:opacity-80 transition-opacity"
          :aria-expanded="!replie"
          @click="replie = !replie"
        >
          <font-awesome-icon
            :icon="['fas', 'chevron-down']"
            class="w-5 h-5 shrink-0 text-amber-700 transition-transform duration-200"
            :class="replie ? '-rotate-90' : ''"
          />
          <h2 class="font-oswald text-3xl md:text-4xl font-bold text-gray-900">
            Recettes culinaires populaires
          </h2>
        </button>
        <button
          v-show="!replie"
          type="button"
          class="px-4 py-2 bg-amber-700 text-white rounded-md hover:bg-amber-800 transition-colors text-sm font-medium shrink-0"
          @click="proposerRecette"
        >
          Proposer une recette
        </button>
      </div>

      <div v-show="!replie">
        <div v-if="chargement" class="space-y-4">
          <div v-for="n in 2" :key="n" class="bg-gray-200 rounded-lg h-40 animate-pulse" />
        </div>

        <div
          v-else-if="recettes.length === 0"
          class="text-center py-12 bg-white rounded-lg"
        >
          <p class="text-gray-600 mb-4">Aucune recette pour l'instant.</p>
          <button
            type="button"
            class="px-4 py-2 bg-amber-700 text-white rounded-md hover:bg-amber-800 transition-colors text-sm font-medium"
            @click="proposerRecette"
          >
            Proposer une recette
          </button>
        </div>

        <template v-else>
          <!-- Filtre : zone (ville) de consommation -->
          <div v-if="zonesDisponibles.length" class="mb-6 flex flex-wrap items-center gap-3">
            <label for="filtre-zone-recette" class="inline-flex items-center gap-2 text-sm font-medium text-gray-700">
              <font-awesome-icon :icon="['fas', 'location-dot']" class="w-4 h-4 text-amber-700" />
              Zone (ville) de consommation
            </label>
            <select
              id="filtre-zone-recette"
              v-model="zoneSelectionnee"
              class="rounded-md border border-gray-300 bg-white px-3 py-2 text-sm text-gray-900 focus:border-amber-600 focus:outline-none focus:ring-1 focus:ring-amber-600"
            >
              <option value="">Toutes les zones</option>
              <option v-for="zone in zonesDisponibles" :key="zone" :value="zone">{{ zone }}</option>
            </select>
            <button
              v-if="zoneSelectionnee"
              type="button"
              class="text-sm font-medium text-amber-700 hover:underline cursor-pointer"
              @click="zoneSelectionnee = ''"
            >
              Réinitialiser
            </button>
          </div>

          <div
            v-if="recettesFiltrees.length === 0"
            class="text-center py-12 bg-white rounded-lg"
          >
            <p class="text-gray-600">Aucune recette pour cette zone.</p>
          </div>

          <div v-else class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
          <article
            v-for="recette in recettesPage"
            :key="recette.id"
            class="group bg-white rounded-lg overflow-hidden shadow-sm hover:shadow-md transition-shadow flex flex-col"
          >
            <!-- Couverture cliquable -->
            <button
              type="button"
              class="relative block w-full h-40 overflow-hidden cursor-pointer"
              @click="ouvrirDetail(recette)"
            >
              <img
                v-if="recette.images && recette.images.length"
                :src="resoudreUrlImage(recette.images[0])"
                :alt="recette.titre"
                class="w-full h-full object-cover transition-transform duration-300 group-hover:scale-105"
              />
              <span
                v-else
                class="w-full h-full flex items-center justify-center bg-gradient-to-br from-amber-100 to-orange-200"
              >
                <font-awesome-icon :icon="['fas', 'utensils']" class="w-10 h-10 text-amber-600" />
              </span>
              <span
                v-if="recette.images && recette.images.length > 1"
                class="absolute bottom-2 right-2 px-1.5 py-0.5 rounded bg-black/55 text-white text-[11px] font-medium inline-flex items-center gap-1"
              >
                <font-awesome-icon :icon="['fas', 'images']" class="w-3 h-3" />
                {{ recette.images.length }}
              </span>
            </button>

            <!-- Infos minimales -->
            <div class="p-4 flex-1 flex flex-col">
              <h3
                class="font-oswald text-lg font-semibold text-gray-900 leading-snug cursor-pointer hover:text-amber-700 transition-colors line-clamp-2"
                @click="ouvrirDetail(recette)"
              >
                {{ recette.titre }}
              </h3>

              <p
                v-if="recette.territoires_consommation"
                class="inline-flex items-center gap-1.5 text-xs text-gray-500 mt-1.5"
              >
                <font-awesome-icon :icon="['fas', 'location-dot']" class="w-3 h-3 text-amber-700 shrink-0" />
                <span class="truncate">{{ recette.territoires_consommation }}</span>
              </p>

              <!-- Méta compacte -->
              <div class="flex flex-wrap gap-2 mt-3">
                <span
                  v-if="recette.ingredients && recette.ingredients.length"
                  class="px-2 py-0.5 text-[11px] font-medium bg-amber-100 text-amber-800 rounded-full"
                >
                  {{ recette.ingredients.length }} ingrédient{{ recette.ingredients.length > 1 ? 's' : '' }}
                </span>
                <span
                  v-if="recette.etapes_preparation && recette.etapes_preparation.length"
                  class="px-2 py-0.5 text-[11px] font-medium bg-orange-100 text-orange-800 rounded-full"
                >
                  {{ recette.etapes_preparation.length }} étape{{ recette.etapes_preparation.length > 1 ? 's' : '' }}
                </span>
              </div>

              <!-- Bandeau de suspension (>10 signalements) -->
              <div
                v-if="recette.suspendu"
                class="mt-3 flex items-start gap-2 rounded-md bg-orange-50 border border-orange-200 px-3 py-2 text-xs text-orange-800"
              >
                <font-awesome-icon :icon="['fas', 'triangle-exclamation']" class="w-3.5 h-3.5 mt-0.5 shrink-0" />
                <span>Contribution suspendue — en cours de vérification par la modération.</span>
              </div>

              <!-- Actions -->
              <div class="flex flex-wrap items-center gap-x-3 gap-y-1.5 mt-auto pt-3 border-t border-gray-100 text-xs">
                <button
                  type="button"
                  class="inline-flex items-center gap-1 font-medium text-amber-700 hover:underline cursor-pointer"
                  @click="ouvrirDetail(recette)"
                >
                  <font-awesome-icon :icon="['fas', 'eye']" class="w-3.5 h-3.5" />
                  Détails
                </button>
                <template v-if="!recette.suspendu">
                  <button
                    type="button"
                    class="inline-flex items-center gap-1 font-medium text-custom-chocolat hover:underline cursor-pointer"
                    @click="ouvrirContribution('edition', recette)"
                  >
                    <font-awesome-icon :icon="['fas', 'pen']" class="w-3 h-3" />
                    Modifier
                  </button>
                  <button
                    type="button"
                    class="inline-flex items-center gap-1 font-medium text-red-600 hover:underline cursor-pointer"
                    @click="ouvrirContribution('suppression', recette)"
                  >
                    <font-awesome-icon :icon="['fas', 'trash']" class="w-3 h-3" />
                    Supprimer
                  </button>
                </template>
                <OpportuniteAfriqueContributionSignalerBouton
                  type-objet="recette_culinaire"
                  :objet-id="recette.id"
                  :libelle="recette.titre"
                  :a-signale="recette.a_signale"
                  :est-authentifie="estAuthentifie"
                  @require-login="emit('require-login')"
                  @suspendu="recette.suspendu = true"
                />
              </div>
            </div>
          </article>
          </div>

          <OpportuniteAfriquePaginationLocale
            v-model:page="page"
            :total-pages="totalPages"
            accent-class="bg-amber-700 border-amber-700 text-white"
          />
        </template>
      </div>
    </div>

    <!-- Modale de détail -->
    <OpportuniteAfriqueRecetteCulinaireDetailModal
      :is-open="showDetail"
      :recette="recetteSelectionnee"
      @close="fermerDetail"
      @edit="(r) => { fermerDetail(); ouvrirContribution('edition', r) }"
      @delete="(r) => { fermerDetail(); ouvrirContribution('suppression', r) }"
    />
  </section>
</template>
