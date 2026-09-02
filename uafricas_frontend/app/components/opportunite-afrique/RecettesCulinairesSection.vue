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

// Section rétractable : repliée par défaut
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

// Navigation vers la page de détail dédiée
const ouvrirDetail = (recette: RecetteCulinaireAPI) => {
  navigateTo(`/opportunite-afrique/${props.ficheId}/recettes/${recette.id}`)
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
  <AfricansAccordeon
    titre="Recettes culinaires populaires"
    icone="fa-solid fa-utensils"
    :model-value="!replie"
    @update:model-value="replie = !$event"
  >

      <div v-show="!replie">
        <div class="mb-6 flex flex-wrap items-center gap-3">
            <template v-if="zonesDisponibles.length">
              <label for="filtre-zone-recette" class="sr-only">Filtrer par zone de consommation</label>
            <font-awesome-icon icon="fa-solid fa-location-dot" class="text-af-atone" />
            <select
              id="filtre-zone-recette"
              v-model="zoneSelectionnee"
              class="h-10 rounded-[10px] border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:outline-2 focus:outline-af-chocolat"
            >
              <option value="">Toutes les zones</option>
              <option v-for="zone in zonesDisponibles" :key="zone" :value="zone">{{ zone }}</option>
            </select>
            <button
              v-if="zoneSelectionnee"
              type="button"
              class="text-[14px]/[1.4] font-bold text-af-chocolat transition hover:opacity-70 cursor-pointer"
              @click="zoneSelectionnee = ''"
            >
              Réinitialiser
            </button>
            </template>

            <AfricansBoutonIcone
              class="ml-auto"
              libelle="Proposer une recette"
              icone="fa-solid fa-plus"
              @click="proposerRecette"
            />
          </div>

        <div v-if="chargement" class="space-y-4">
          <div v-for="n in 2" :key="n" class="bg-gray-200 rounded-lg h-40 animate-pulse" />
        </div>

        <div
          v-else-if="recettes.length === 0"
          class="rounded-[10px] border border-af-bordure bg-white py-12 text-center"
        >
          <p class="text-[14px]/[1.4] text-af-corps">Aucune recette pour l'instant.</p>
        </div>

        <template v-else>
          <!-- Filtre : zone (ville) de consommation -->
          

          <div
            v-if="recettesFiltrees.length === 0"
            class="rounded-[10px] border border-af-bordure bg-white py-12 text-center"
          >
            <p class="text-[14px]/[1.4] text-af-corps">Aucune recette pour cette zone.</p>
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
                class="w-full h-full flex items-center justify-center bg-af-fond"
              >
                <font-awesome-icon :icon="['fas', 'utensils']" class="w-10 h-10 text-af-atone-2" />
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
                class="line-clamp-2 text-[17px]/[1.4] font-bold text-af-encre cursor-pointer transition hover:text-af-chocolat"
                @click="ouvrirDetail(recette)"
              >
                {{ recette.titre }}
              </h3>

              <p
                v-if="recette.territoires_consommation"
                class="inline-flex items-center gap-1.5 text-xs text-gray-500 mt-1.5"
              >
                <font-awesome-icon :icon="['fas', 'location-dot']" class="w-3 h-3 text-af-atone shrink-0" />
                <span class="truncate">{{ recette.territoires_consommation }}</span>
              </p>

              <!-- Méta compacte -->
              <div class="flex flex-wrap gap-2 mt-3">
                <span
                  v-if="recette.ingredients && recette.ingredients.length"
                  class="rounded-full bg-af-fond px-2 py-0.5 text-[12px]/[1.4] font-bold text-af-corps"
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
                class="mt-3 flex items-start gap-2 rounded-[10px] border border-af-live/30 bg-af-live/5 px-3 py-2 text-[12px]/[1.4] text-af-live"
              >
                <font-awesome-icon :icon="['fas', 'triangle-exclamation']" class="w-3.5 h-3.5 mt-0.5 shrink-0" />
                <span>Contribution suspendue : en cours de vérification par la modération.</span>
              </div>

              <!-- Actions -->
              <div class="flex flex-wrap items-center gap-x-3 gap-y-1.5 mt-auto pt-3 border-t border-af-bordure text-xs">
                <button
                  type="button"
                  class="inline-flex items-center gap-1 font-bold text-af-corps transition hover:text-af-chocolat cursor-pointer"
                  @click="ouvrirDetail(recette)"
                >
                  <font-awesome-icon :icon="['fas', 'eye']" class="w-3.5 h-3.5" />
                  Détails
                </button>
                <template v-if="!recette.suspendu">
                  <button
                    type="button"
                    class="inline-flex items-center gap-1 font-bold text-af-corps transition hover:text-af-chocolat cursor-pointer"
                    @click="ouvrirContribution('edition', recette)"
                  >
                    <font-awesome-icon :icon="['fas', 'pen']" class="w-3 h-3" />
                    Modifier
                  </button>
                  <button
                    type="button"
                    class="inline-flex items-center gap-1 font-bold text-af-corps transition hover:text-af-live cursor-pointer"
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
            accent-class="bg-af-chocolat border-af-chocolat text-white"
          />
        </template>
      </div>
  </AfricansAccordeon>
</template>
