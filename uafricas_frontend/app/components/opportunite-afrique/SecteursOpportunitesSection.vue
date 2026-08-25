<script setup lang="ts">
import { useOpportuniteAfrique } from '~/composables/useOpportuniteAfrique'
import type {
  SecteurOpportuniteAPI,
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
}>()

const { listerSecteursOpportunites, resoudreUrlImage } = useOpportuniteAfrique()

const secteurs = ref<SecteurOpportuniteAPI[]>([])
const chargement = ref(true)

// Filtre par localité (ville, région, zone concernée)
const localiteSelectionnee = ref('')

// Liste des localités distinctes présentes dans les secteurs
const localitesDisponibles = computed(() => {
  const ensemble = new Set<string>()
  for (const secteur of secteurs.value) {
    const valeur = (secteur.localite || '').trim()
    if (valeur) ensemble.add(valeur)
  }
  return Array.from(ensemble).sort((a, b) => a.localeCompare(b, 'fr'))
})

const secteursFiltres = computed(() =>
  localiteSelectionnee.value
    ? secteurs.value.filter((s) => (s.localite || '').trim() === localiteSelectionnee.value)
    : secteurs.value,
)

// Pagination locale (liste pleine largeur → 6 par page)
const { page, totalPages, pageItems: secteursPage } = usePaginationLocale(secteursFiltres, 6)

onMounted(async () => {
  chargement.value = true
  secteurs.value = await listerSecteursOpportunites(props.ficheId)
  chargement.value = false
})

const { redirigerVersConnexion } = useAuth()

const ouvrirDetail = (secteur: SecteurOpportuniteAPI) => {
  navigateTo(`/opportunite-afrique/${props.ficheId}/secteurs/${secteur.id}`)
}

const ouvrirContribution = (
  type_contribution: 'ajout' | 'edition' | 'suppression',
  secteur?: SecteurOpportuniteAPI,
) => {
  if (!props.estAuthentifie) {
    redirigerVersConnexion()
    return
  }
  emit('open-contribution', {
    type_objet_contribution: 'secteur_developpement',
    section_afripulse: 'secteurs_opportunites',
    type_contribution,
    target_id: secteur?.id,
    donnees_actuelles: secteur
      ? {
          nom: secteur.nom,
          description: secteur.description,
          localite: secteur.localite,
          contact_telephone: secteur.contact_telephone,
          contact_courriel: secteur.contact_courriel,
          contact_adresse: secteur.contact_adresse,
          references_utiles: secteur.references_utiles,
          site_web_url: secteur.site_web_url,
          image_url: secteur.image_url,
        }
      : undefined,
    libelle: secteur?.nom,
  })
}

const proposerSecteur = () => {
  ouvrirContribution('ajout')
}
</script>

<template>
  <AfricansAccordeon
    titre="Secteurs d'opportunités"
    icone="fa-solid fa-briefcase"
    :model-value="!replie"
    @update:model-value="replie = !$event"
  >

      <div v-show="!replie">
        <div class="mb-6 flex flex-wrap items-center gap-3">
          <template v-if="localitesDisponibles.length">
            <label for="filtre-localite-secteur" class="sr-only">Filtrer par localité</label>
            <font-awesome-icon icon="fa-solid fa-location-dot" class="text-af-atone" />
            <select
              id="filtre-localite-secteur"
              v-model="localiteSelectionnee"
              class="h-10 rounded-[10px] border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:outline-2 focus:outline-af-chocolat"
            >
              <option value="">Toutes les localités</option>
              <option v-for="localite in localitesDisponibles" :key="localite" :value="localite">{{ localite }}</option>
            </select>
            <button
              v-if="localiteSelectionnee"
              type="button"
              class="text-[14px]/[1.4] font-bold text-af-chocolat transition hover:opacity-70 cursor-pointer"
              @click="localiteSelectionnee = ''"
            >
              Réinitialiser
            </button>
          </template>

          <AfricansBoutonIcone
            class="ml-auto"
            libelle="Proposer un secteur"
            icone="fa-solid fa-plus"
            @click="proposerSecteur"
          />
        </div>


      <div v-if="chargement" class="space-y-4">
        <div v-for="n in 3" :key="n" class="h-24 animate-pulse rounded-[10px] bg-af-bordure" />
      </div>

      <div
        v-else-if="secteurs.length === 0"
        class="rounded-[10px] border border-af-bordure bg-white py-12 text-center"
      >
        <p class="text-[14px]/[1.4] text-af-corps">Aucun secteur pour l'instant.</p>
      </div>

      <template v-else>
        <!-- Barre d'outils : filtre à gauche, action à droite, une seule
             ligne de base. Le libellé du menu reste `sr-only` : il disait en
             toutes lettres ce que le menu montre déjà. -->
        

        <div
          v-if="secteursFiltres.length === 0"
          class="rounded-[10px] border border-af-bordure bg-white py-12 text-center"
        >
          <p class="text-[14px]/[1.4] text-af-corps">Aucun secteur pour cette localité.</p>
        </div>

        <ul v-else class="space-y-4">
        <li
          v-for="secteur in secteursPage"
          :key="secteur.id"
          class="flex flex-col sm:flex-row gap-5 rounded-[10px] border border-af-bordure bg-white p-6 transition hover:border-af-chocolat"
        >
          <!-- Image illustrative (optionnelle) : cliquable -->
          <button
            v-if="secteur.image_url"
            type="button"
            class="w-full sm:w-44 h-40 sm:h-auto sm:max-h-44 shrink-0 overflow-hidden rounded-lg cursor-pointer"
            @click="ouvrirDetail(secteur)"
          >
            <img
              :src="resoudreUrlImage(secteur.image_url)"
              :alt="secteur.nom"
              class="w-full h-full object-cover transition-transform hover:scale-105"
            />
          </button>

          <div class="min-w-0 flex-1">
            <h3
              class="text-[17px]/[1.4] font-bold text-af-encre cursor-pointer transition hover:text-af-chocolat"
              @click="ouvrirDetail(secteur)"
            >
              {{ secteur.nom }}
            </h3>
            <p v-if="secteur.localite" class="inline-flex items-center gap-1.5 text-sm text-gray-500 mb-2">
              <font-awesome-icon :icon="['fas', 'location-dot']" class="w-3.5 h-3.5 text-af-atone" />
              {{ secteur.localite }}
            </p>
            <p v-if="secteur.description" class="text-gray-600 leading-relaxed line-clamp-3">
              {{ secteur.description }}
            </p>

          <!-- Bandeau de suspension (>10 signalements) -->
          <div
            v-if="secteur.suspendu"
            class="mt-3 flex items-start gap-2 rounded-[10px] border border-af-live/30 bg-af-live/5 px-3 py-2 text-[12px]/[1.4] text-af-live"
          >
            <font-awesome-icon :icon="['fas', 'triangle-exclamation']" class="w-3.5 h-3.5 mt-0.5 shrink-0" />
            <span>Contribution suspendue : en cours de vérification par la modération.</span>
          </div>

          <div class="flex flex-wrap items-center gap-3 mt-3 pt-3 border-t border-af-bordure">
            <button
              type="button"
              class="inline-flex items-center gap-1 text-[12px]/[1.4] font-bold text-af-corps transition hover:text-af-chocolat"
              @click="ouvrirDetail(secteur)"
            >
              <font-awesome-icon :icon="['fas', 'circle-info']" class="w-3.5 h-3.5" />
              Détails
            </button>
            <template v-if="!secteur.suspendu">
              <button
                type="button"
                class="inline-flex items-center gap-1 text-[12px]/[1.4] font-bold text-af-corps transition hover:text-af-chocolat"
                @click="ouvrirContribution('edition', secteur)"
              >
                <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                </svg>
                Modifier
              </button>
              <button
                type="button"
                class="inline-flex items-center gap-1 text-[12px]/[1.4] font-bold text-af-corps transition hover:text-af-live"
                @click="ouvrirContribution('suppression', secteur)"
              >
                <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                </svg>
                Supprimer
              </button>
            </template>
            <OpportuniteAfriqueContributionSignalerBouton
              class="text-xs"
              type-objet="secteur_developpement"
              :objet-id="secteur.id"
              :libelle="secteur.nom"
              :a-signale="secteur.a_signale"
              :est-authentifie="estAuthentifie"
              @require-login="emit('require-login')"
              @suspendu="secteur.suspendu = true"
            />
          </div>
          </div>
        </li>
        </ul>

        <OpportuniteAfriquePaginationLocale v-model:page="page" :total-pages="totalPages" />
      </template>
      </div>
  </AfricansAccordeon>
</template>
