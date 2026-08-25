<script setup lang="ts">
import { useOpportuniteAfrique } from '~/composables/useOpportuniteAfrique'
import type {
  PersonnaliteConnueAPI,
  DomainePersonnalite,
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

const { listerPersonnalites, resoudreUrlImage } = useOpportuniteAfrique()

const personnalites = ref<PersonnaliteConnueAPI[]>([])
const chargement = ref(true)
const domaineFiltre = ref<DomainePersonnalite | 'tous'>('tous')

// Pagination locale (grille jusqu'à 4 colonnes → 8 par page)
const { page, totalPages, pageItems: personnalitesPage } = usePaginationLocale(personnalites, 8)

const domaines: { value: DomainePersonnalite | 'tous', label: string }[] = [
  { value: 'tous', label: 'Tous' },
  { value: 'politique', label: 'Politique' },
  { value: 'artiste_musicien', label: 'Artiste musicien' },
  { value: 'artiste_autre', label: 'Artiste (autre)' },
  { value: 'sportif', label: 'Sportif' },
  { value: 'entrepreneur', label: 'Entrepreneur' },
  { value: 'scientifique', label: 'Scientifique' },
  { value: 'militaire_historique', label: 'Militaire / Historique' },
  { value: 'autre', label: 'Autre' },
]

const labelDomaine = (d: DomainePersonnalite): string => {
  const found = domaines.find(x => x.value === d)
  return found ? found.label : d
}

const getInitiales = (nom: string): string => {
  const parts = nom.trim().split(/\s+/)
  const first = parts[0]?.charAt(0) ?? ''
  const last = parts.length > 1 ? (parts[parts.length - 1]?.charAt(0) ?? '') : ''
  return `${first}${last}`.toUpperCase()
}

const charger = async () => {
  chargement.value = true
  const d = domaineFiltre.value === 'tous' ? undefined : domaineFiltre.value
  personnalites.value = await listerPersonnalites(props.ficheId, d)
  chargement.value = false
}

onMounted(charger)

watch(domaineFiltre, charger)

const { redirigerVersConnexion } = useAuth()

const ouvrirDetail = (personnalite: PersonnaliteConnueAPI) => {
  navigateTo(`/opportunite-afrique/${props.ficheId}/personnalites/${personnalite.id}`)
}

const ouvrirContribution = (
  type_contribution: 'ajout' | 'edition' | 'suppression',
  personnalite?: PersonnaliteConnueAPI,
) => {
  if (!props.estAuthentifie) {
    redirigerVersConnexion()
    return
  }
  emit('open-contribution', {
    type_objet_contribution: 'personnalite_connue',
    section_afripulse: 'personnalites',
    type_contribution,
    target_id: personnalite?.id,
    donnees_actuelles: personnalite
      ? {
          nom_complet: personnalite.nom_complet,
          domaine: personnalite.domaine,
          biographie_courte: personnalite.biographie_courte,
          annee_naissance: personnalite.annee_naissance,
          annee_deces: personnalite.annee_deces,
          portrait_url: personnalite.portrait_url,
          lien_reference: personnalite.lien_reference,
        }
      : undefined,
    libelle: personnalite?.nom_complet,
  })
}

const proposerPersonnalite = () => {
  ouvrirContribution('ajout')
}
</script>

<template>
  <AfricansAccordeon
    titre="Personnalités connues"
    icone="fa-solid fa-user-tie"
    :model-value="!replie"
    @update:model-value="replie = !$event"
  >
      <div v-show="!replie">
      <!-- Les pastilles de domaine ET l'action tiennent sur la même ligne :
           `items-center` les aligne sur la même base, `ml-auto` pousse le
           bouton à droite. -->
      <div class="mb-8 flex flex-wrap items-center gap-2">
        <button
          v-for="d in domaines"
          :key="d.value"
          type="button"
          class="px-3 py-1.5 rounded-full text-sm font-medium transition-colors border"
          :class="domaineFiltre === d.value
            ? 'bg-af-chocolat text-white border-af-chocolat'
            : 'bg-white text-af-corps border-af-bordure hover:bg-af-fond'"
          @click="domaineFiltre = d.value"
        >
          {{ d.label }}
        </button>

        <AfricansBoutonIcone
          class="ml-auto"
          libelle="Proposer une personnalité"
          icone="fa-solid fa-plus"
          @click="proposerPersonnalite"
        />
      </div>

      <div v-if="chargement" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-5">
        <div v-for="n in 4" :key="n" class="bg-gray-100 rounded-lg h-80 animate-pulse" />
      </div>

      <div
        v-else-if="personnalites.length === 0"
        class="rounded-[10px] border border-af-bordure bg-white py-12 text-center"
      >
        <p class="text-[14px]/[1.4] text-af-corps">Aucune personnalité pour l'instant.</p>
      </div>

      <div
        v-else
        class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-5"
      >
        <article
          v-for="p in personnalitesPage"
          :key="p.id"
          class="bg-white rounded-lg overflow-hidden shadow-sm hover:shadow-md transition-shadow flex flex-col"
        >
          <button
            type="button"
            class="aspect-square bg-gray-100 relative overflow-hidden cursor-pointer group"
            :aria-label="`Voir la fiche de ${p.nom_complet}`"
            @click="ouvrirDetail(p)"
          >
            <img
              v-if="p.portrait_url"
              :src="resoudreUrlImage(p.portrait_url)"
              :alt="p.nom_complet"
              class="w-full h-full object-cover transition-transform group-hover:scale-105"
            />
            <div
              v-else
              class="w-full h-full flex items-center justify-center bg-af-chocolat/10 text-4xl font-bold text-af-chocolat"
            >
              {{ getInitiales(p.nom_complet) }}
            </div>
          </button>
          <div class="p-4 flex-1 flex flex-col">
            <h3
              class="text-[17px]/[1.4] font-bold text-af-encre cursor-pointer transition hover:text-af-chocolat"
              @click="ouvrirDetail(p)"
            >
              {{ p.nom_complet }}
            </h3>
            <span
              class="inline-block self-start rounded-full bg-af-fond px-2 py-0.5 text-[12px]/[1.4] font-bold text-af-corps mb-2"
            >
              {{ labelDomaine(p.domaine) }}
            </span>
            <p
              v-if="p.annee_naissance || p.annee_deces"
              class="text-xs text-gray-500 mb-2"
            >
              {{ p.annee_naissance ?? '?' }} - {{ p.annee_deces ?? '' }}
            </p>
            <p class="text-sm text-gray-600 line-clamp-3 flex-1">
              {{ p.biographie_courte }}
            </p>
            <a
              v-if="p.lien_reference"
              :href="p.lien_reference"
              target="_blank"
              rel="noopener noreferrer"
              class="mt-3 inline-block text-[12px]/[1.4] font-bold text-af-chocolat transition hover:opacity-70"
            >
              En savoir plus
            </a>
            <!-- Bandeau de suspension (>10 signalements) -->
            <div
              v-if="p.suspendu"
              class="mt-3 flex items-start gap-2 rounded-[10px] border border-af-live/30 bg-af-live/5 px-3 py-2 text-[12px]/[1.4] text-af-live"
            >
              <font-awesome-icon :icon="['fas', 'triangle-exclamation']" class="w-3.5 h-3.5 mt-0.5 shrink-0" />
              <span>Contribution suspendue : en cours de vérification.</span>
            </div>

            <div class="flex flex-wrap items-center gap-3 mt-3 pt-3 border-t border-af-bordure">
              <button
                type="button"
                class="inline-flex items-center gap-1 text-[12px]/[1.4] font-bold text-af-corps transition hover:text-af-chocolat"
                @click="ouvrirDetail(p)"
              >
                <font-awesome-icon :icon="['fas', 'circle-info']" class="w-3.5 h-3.5" />
                Détails
              </button>
              <template v-if="!p.suspendu">
                <button
                  type="button"
                  class="inline-flex items-center gap-1 text-[12px]/[1.4] font-bold text-af-corps transition hover:text-af-chocolat"
                  @click="ouvrirContribution('edition', p)"
                >
                  <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                  </svg>
                  Modifier
                </button>
                <button
                  type="button"
                  class="inline-flex items-center gap-1 text-[12px]/[1.4] font-bold text-af-corps transition hover:text-af-live"
                  @click="ouvrirContribution('suppression', p)"
                >
                  <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                  </svg>
                  Supprimer
                </button>
              </template>
              <OpportuniteAfriqueContributionSignalerBouton
                type-objet="personnalite_connue"
                :objet-id="p.id"
                :libelle="p.nom_complet"
                :a-signale="p.a_signale"
                :est-authentifie="estAuthentifie"
                @require-login="emit('require-login')"
                @suspendu="p.suspendu = true"
              />
            </div>
          </div>
        </article>
      </div>

      <OpportuniteAfriquePaginationLocale
        v-if="!chargement && personnalites.length"
        v-model:page="page"
        :total-pages="totalPages"
        accent-class="bg-af-chocolat border-af-chocolat text-white"
      />
      </div>
  </AfricansAccordeon>
</template>
