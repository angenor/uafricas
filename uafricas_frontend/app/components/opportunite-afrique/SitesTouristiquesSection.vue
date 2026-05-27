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

// Filtres par sous-type (client-side) par famille
const filtreEmblematique = ref<SousTypeSite | ''>('')
const filtrePrive = ref<SousTypeSite | ''>('')

const optionsEmblematiques = SOUS_TYPES_PAR_CATEGORIE.emblematique.map(v => ({ value: v, label: LIBELLES_SOUS_TYPE[v] }))
const optionsPrives = SOUS_TYPES_PAR_CATEGORIE.prive.map(v => ({ value: v, label: LIBELLES_SOUS_TYPE[v] }))

const emblematiquesFiltres = computed(() =>
  filtreEmblematique.value
    ? sitesEmblematiques.value.filter(s => s.sous_type === filtreEmblematique.value)
    : sitesEmblematiques.value,
)
const privesFiltres = computed(() =>
  filtrePrive.value
    ? sitesPrives.value.filter(s => s.sous_type === filtrePrive.value)
    : sitesPrives.value,
)

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

const router = useRouter()

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
})

const ouvrirContribution = (
  type_contribution: 'ajout' | 'edition' | 'suppression',
  section: 'sites_emblematiques' | 'sites_prives',
  site?: SiteTouristiqueAPI,
) => {
  if (!props.estAuthentifie) {
    router.push('/login')
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
  <section class="py-12">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <h2 class="font-oswald text-3xl md:text-4xl font-bold text-gray-900 mb-8">
        Sites touristiques
      </h2>

      <div class="space-y-12">
        <!-- Sites emblématiques -->
        <div>
          <div class="flex flex-wrap items-center justify-between gap-3 mb-6">
            <h3 class="font-oswald text-2xl font-semibold text-custom-chocolat">
              Sites emblématiques
            </h3>
            <div class="flex items-center gap-3">
              <select
                v-model="filtreEmblematique"
                class="px-3 py-2 text-sm border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent"
              >
                <option value="">Tous les types</option>
                <option v-for="o in optionsEmblematiques" :key="o.value" :value="o.value">{{ o.label }}</option>
              </select>
              <button
                type="button"
                class="px-4 py-2 bg-custom-chocolat text-white rounded-md hover:bg-custom-chocolat/90 transition-colors text-sm font-medium"
                @click="proposerSite('sites_emblematiques')"
              >
                Proposer un site
              </button>
            </div>
          </div>

          <div v-if="chargementEmblematiques" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
            <div v-for="n in 3" :key="n" class="bg-gray-100 rounded-lg h-64 animate-pulse" />
          </div>

          <div
            v-else-if="emblematiquesFiltres.length === 0"
            class="text-center py-10 bg-gray-50 rounded-lg"
          >
            <p class="text-gray-600 mb-4">
              {{ filtreEmblematique ? 'Aucun site pour ce type.' : 'Aucun site pour l\'instant.' }}
            </p>
            <button
              type="button"
              class="px-4 py-2 bg-custom-chocolat text-white rounded-md hover:bg-custom-chocolat/90 transition-colors text-sm font-medium"
              @click="proposerSite('sites_emblematiques')"
            >
              Proposer un site
            </button>
          </div>

          <div v-else class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
            <OpportuniteAfriqueSiteTouristiqueCarte
              v-for="site in emblematiquesFiltres"
              :key="site.id"
              :site="site"
              :est-authentifie="estAuthentifie"
              @edit="onEdit"
              @delete="onDelete"
            />
          </div>
        </div>

        <!-- Sites privés -->
        <div>
          <div class="flex flex-wrap items-center justify-between gap-3 mb-6">
            <h3 class="font-oswald text-2xl font-semibold text-custom-green">
              Sites privés
            </h3>
            <div class="flex items-center gap-3">
              <select
                v-model="filtrePrive"
                class="px-3 py-2 text-sm border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-green focus:border-transparent"
              >
                <option value="">Tous les types</option>
                <option v-for="o in optionsPrives" :key="o.value" :value="o.value">{{ o.label }}</option>
              </select>
              <button
                type="button"
                class="px-4 py-2 bg-custom-green text-white rounded-md hover:bg-custom-green/90 transition-colors text-sm font-medium"
                @click="proposerSite('sites_prives')"
              >
                Proposer un site
              </button>
            </div>
          </div>

          <div v-if="chargementPrives" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
            <div v-for="n in 3" :key="n" class="bg-gray-100 rounded-lg h-64 animate-pulse" />
          </div>

          <div
            v-else-if="privesFiltres.length === 0"
            class="text-center py-10 bg-gray-50 rounded-lg"
          >
            <p class="text-gray-600 mb-4">
              {{ filtrePrive ? 'Aucun site pour ce type.' : 'Aucun site pour l\'instant.' }}
            </p>
            <button
              type="button"
              class="px-4 py-2 bg-custom-green text-white rounded-md hover:bg-custom-green/90 transition-colors text-sm font-medium"
              @click="proposerSite('sites_prives')"
            >
              Proposer un site
            </button>
          </div>

          <div v-else class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
            <OpportuniteAfriqueSiteTouristiqueCarte
              v-for="site in privesFiltres"
              :key="site.id"
              :site="site"
              :est-authentifie="estAuthentifie"
              @edit="onEdit"
              @delete="onDelete"
            />
          </div>
        </div>
      </div>
    </div>
  </section>
</template>
