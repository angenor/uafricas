<script setup lang="ts">
import { useOpportuniteAfrique } from '~/composables/useOpportuniteAfrique'
import {
  SOUS_TYPES_PAR_CATEGORIE,
  LIBELLES_SOUS_TYPE,
  type SiteTouristiqueAPI,
  type SectionAfripulse,
  type TypeObjetContribution,
  type SousTypeSite,
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

const { listerSitesTouristiques } = useOpportuniteAfrique()

const sitesEmblematiques = ref<SiteTouristiqueAPI[]>([])
const sitesPrives = ref<SiteTouristiqueAPI[]>([])
const chargementEmblematiques = ref(true)
const chargementPrives = ref(true)

// Filtres par sous-type / ville / village (client-side) par famille
const filtreEmblematique = ref<SousTypeSite | ''>('')
const filtreVilleEmblematique = ref('')
const filtreVillageEmblematique = ref('')
const filtrePrive = ref<SousTypeSite | ''>('')
const filtreVillePrive = ref('')
const filtreVillagePrive = ref('')

const optionsEmblematiques = SOUS_TYPES_PAR_CATEGORIE.emblematique.map(v => ({ value: v, label: LIBELLES_SOUS_TYPE[v] }))
const optionsPrives = SOUS_TYPES_PAR_CATEGORIE.prive.map(v => ({ value: v, label: LIBELLES_SOUS_TYPE[v] }))

// Listes distinctes (triées) de villes / villages présentes dans chaque famille
const valeursDistinctes = (sites: SiteTouristiqueAPI[], champ: 'ville' | 'village') => {
  const ensemble = new Set<string>()
  for (const site of sites) {
    const valeur = (site[champ] || '').trim()
    if (valeur) ensemble.add(valeur)
  }
  return Array.from(ensemble).sort((a, b) => a.localeCompare(b, 'fr'))
}

const villesEmblematiques = computed(() => valeursDistinctes(sitesEmblematiques.value, 'ville'))
const villagesEmblematiques = computed(() => valeursDistinctes(sitesEmblematiques.value, 'village'))
const villesPrives = computed(() => valeursDistinctes(sitesPrives.value, 'ville'))
const villagesPrives = computed(() => valeursDistinctes(sitesPrives.value, 'village'))

const emblematiquesFiltres = computed(() =>
  sitesEmblematiques.value.filter(s =>
    (!filtreEmblematique.value || s.sous_type === filtreEmblematique.value) &&
    (!filtreVilleEmblematique.value || (s.ville || '').trim() === filtreVilleEmblematique.value) &&
    (!filtreVillageEmblematique.value || (s.village || '').trim() === filtreVillageEmblematique.value),
  ),
)
const privesFiltres = computed(() =>
  sitesPrives.value.filter(s =>
    (!filtrePrive.value || s.sous_type === filtrePrive.value) &&
    (!filtreVillePrive.value || (s.ville || '').trim() === filtreVillePrive.value) &&
    (!filtreVillagePrive.value || (s.village || '').trim() === filtreVillagePrive.value),
  ),
)

// Pagination locale par famille (grille 3 colonnes → 9 par page)
const {
  page: pageEmblematique,
  totalPages: totalPagesEmblematique,
  pageItems: emblematiquesPage,
} = usePaginationLocale(emblematiquesFiltres, 9)
const {
  page: pagePrive,
  totalPages: totalPagesPrive,
  pageItems: privesPage,
} = usePaginationLocale(privesFiltres, 9)

const chargerEmblematiques = async () => {
  chargementEmblematiques.value = true
  sitesEmblematiques.value = await listerSitesTouristiques(props.ficheId, 'emblematique')
  chargementEmblematiques.value = false
}

const chargerPrives = async () => {
  chargementPrives.value = true
  sitesPrives.value = await listerSitesTouristiques(props.ficheId, 'prive')
  chargementPrives.value = false
}

onMounted(async () => {
  await Promise.all([chargerEmblematiques(), chargerPrives()])
})

const { redirigerVersConnexion } = useAuth()

/** Construit le snapshot complet des champs d'un site (pré-remplissage édition). */
const snapshotSite = (site: SiteTouristiqueAPI): Record<string, unknown> => ({
  nom: site.nom,
  sous_type: site.sous_type,
  description: site.description,
  info_pertinente: site.info_pertinente,
  image_url: site.image_url,
  images: site.images,
  gestionnaire: site.gestionnaire,
  ville: site.ville,
  village: site.village,
  latitude: site.latitude,
  longitude: site.longitude,
  contact_telephone: site.contact_telephone,
  contact_courriel: site.contact_courriel,
  contact_adresse: site.contact_adresse,
  constitution_statut_juridique: site.constitution_statut_juridique,
  constitution_numero: site.constitution_numero,
  constitution_document_url: site.constitution_document_url,
  site_web_url: site.site_web_url,
})

const ouvrirContribution = (
  type_contribution: 'ajout' | 'edition' | 'suppression',
  section: 'sites_emblematiques' | 'sites_prives',
  site?: SiteTouristiqueAPI,
) => {
  if (!props.estAuthentifie) {
    redirigerVersConnexion()
    return
  }
  emit('open-contribution', {
    type_objet_contribution: 'site_touristique',
    section_afripulse: section,
    type_contribution,
    target_id: site?.id,
    donnees_actuelles: site ? snapshotSite(site) : undefined,
    libelle: site?.nom,
  })
}

const proposerSite = (section: 'sites_emblematiques' | 'sites_prives') => {
  ouvrirContribution('ajout', section)
}

const sectionDe = (site: SiteTouristiqueAPI): 'sites_emblematiques' | 'sites_prives' =>
  site.categorie === 'prive' ? 'sites_prives' : 'sites_emblematiques'

const onEdit = (site: SiteTouristiqueAPI) => ouvrirContribution('edition', sectionDe(site), site)
const onDelete = (site: SiteTouristiqueAPI) => ouvrirContribution('suppression', sectionDe(site), site)
</script>

<template>
  <AfricansAccordeon
    titre="Sites touristiques"
    icone="fa-solid fa-map-pin"
    :model-value="!replie"
    @update:model-value="replie = !$event"
  >
      <div v-show="!replie" class="space-y-12">
        <!-- Sites emblématiques -->
        <div>
          <div class="mb-6 flex flex-col gap-4">
            <div class="flex flex-wrap items-center justify-between gap-3">
              <h3 class="text-[17px]/[1.4] font-bold text-af-encre">
              Sites emblématiques
            </h3>
            </div>
            <div class="flex flex-wrap items-center gap-3">
              <select
                v-model="filtreEmblematique"
                class="h-10 rounded-[10px] border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:outline-2 focus:outline-af-chocolat"
              >
                <option value="">Tous les types</option>
                <option v-for="o in optionsEmblematiques" :key="o.value" :value="o.value">{{ o.label }}</option>
              </select>
              <select
                v-if="villesEmblematiques.length"
                v-model="filtreVilleEmblematique"
                class="h-10 rounded-[10px] border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:outline-2 focus:outline-af-chocolat"
              >
                <option value="">Toutes les villes</option>
                <option v-for="v in villesEmblematiques" :key="v" :value="v">{{ v }}</option>
              </select>
              <select
                v-if="villagesEmblematiques.length"
                v-model="filtreVillageEmblematique"
                class="h-10 rounded-[10px] border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:outline-2 focus:outline-af-chocolat"
              >
                <option value="">Tous les villages</option>
                <option v-for="v in villagesEmblematiques" :key="v" :value="v">{{ v }}</option>
              </select>

              <AfricansBoutonIcone
                class="ml-auto"
                libelle="Proposer un site emblématique"
                icone="fa-solid fa-plus"
                @click="proposerSite('sites_emblematiques')"
              />
            </div>
          </div>

          <div v-if="chargementEmblematiques" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
            <div v-for="n in 3" :key="n" class="h-64 animate-pulse rounded-[10px] bg-af-bordure" />
          </div>

          <div
            v-else-if="emblematiquesFiltres.length === 0"
            class="rounded-[10px] border border-af-bordure bg-white py-10 text-center"
          >
            <p class="text-[14px]/[1.4] text-af-corps">
              {{ filtreEmblematique ? 'Aucun site pour ce type.' : 'Aucun site pour l\'instant.' }}
            </p>
          </div>

          <div v-else class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
            <OpportuniteAfriqueSiteTouristiqueCarte
              v-for="site in emblematiquesPage"
              :key="site.id"
              :site="site"
              :est-authentifie="estAuthentifie"
              @edit="onEdit"
              @delete="onDelete"
              @require-login="emit('require-login')"
              @suspendu="(s) => (s.suspendu = true)"
            />
          </div>

          <OpportuniteAfriquePaginationLocale
            v-model:page="pageEmblematique"
            :total-pages="totalPagesEmblematique"
            accent-class="bg-af-chocolat border-af-chocolat text-white"
          />
        </div>

        <!-- Sites privés -->
        <div>
          <div class="mb-6 flex flex-col gap-4">
            <div class="flex flex-wrap items-center justify-between gap-3">
              <h3 class="text-[17px]/[1.4] font-bold text-af-encre">
              Sites privés
            </h3>
            </div>
            <div class="flex flex-wrap items-center gap-3">
              <select
                v-model="filtrePrive"
                class="h-10 rounded-[10px] border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:outline-2 focus:outline-af-chocolat"
              >
                <option value="">Tous les types</option>
                <option v-for="o in optionsPrives" :key="o.value" :value="o.value">{{ o.label }}</option>
              </select>
              <select
                v-if="villesPrives.length"
                v-model="filtreVillePrive"
                class="h-10 rounded-[10px] border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:outline-2 focus:outline-af-chocolat"
              >
                <option value="">Toutes les villes</option>
                <option v-for="v in villesPrives" :key="v" :value="v">{{ v }}</option>
              </select>
              <select
                v-if="villagesPrives.length"
                v-model="filtreVillagePrive"
                class="h-10 rounded-[10px] border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:outline-2 focus:outline-af-chocolat"
              >
                <option value="">Tous les villages</option>
                <option v-for="v in villagesPrives" :key="v" :value="v">{{ v }}</option>
              </select>

              <AfricansBoutonIcone
                class="ml-auto"
                libelle="Proposer un site privé"
                icone="fa-solid fa-plus"
                @click="proposerSite('sites_prives')"
              />
            </div>
          </div>

          <div v-if="chargementPrives" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
            <div v-for="n in 3" :key="n" class="h-64 animate-pulse rounded-[10px] bg-af-bordure" />
          </div>

          <div
            v-else-if="privesFiltres.length === 0"
            class="rounded-[10px] border border-af-bordure bg-white py-10 text-center"
          >
            <p class="text-[14px]/[1.4] text-af-corps">
              {{ filtrePrive ? 'Aucun site pour ce type.' : 'Aucun site pour l\'instant.' }}
            </p>
          </div>

          <div v-else class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
            <OpportuniteAfriqueSiteTouristiqueCarte
              v-for="site in privesPage"
              :key="site.id"
              :site="site"
              :est-authentifie="estAuthentifie"
              @edit="onEdit"
              @delete="onDelete"
              @require-login="emit('require-login')"
              @suspendu="(s) => (s.suspendu = true)"
            />
          </div>

          <OpportuniteAfriquePaginationLocale
            v-model:page="pagePrive"
            :total-pages="totalPagesPrive"
            accent-class="bg-af-chocolat border-af-chocolat text-white"
          />
        </div>
      </div>
  </AfricansAccordeon>
</template>
