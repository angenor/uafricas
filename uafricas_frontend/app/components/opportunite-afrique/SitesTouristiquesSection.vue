<script setup lang="ts">
import { useOpportuniteAfrique } from '~/composables/useOpportuniteAfrique'
import type {
  SiteTouristiqueAPI,
  SectionAfripulse,
  TypeObjetContribution,
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

const { listerSitesTouristiques, resoudreUrlImage } = useOpportuniteAfrique()

const sitesEmblematiques = ref<SiteTouristiqueAPI[]>([])
const sitesPrives = ref<SiteTouristiqueAPI[]>([])
const chargementEmblematiques = ref(true)
const chargementPrives = ref(true)

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
    donnees_actuelles: site
      ? { nom: site.nom, description: site.description, image_url: site.image_url }
      : undefined,
    libelle: site?.nom,
  })
}

const proposerSite = (section: 'sites_emblematiques' | 'sites_prives') => {
  ouvrirContribution('ajout', section)
}

const sectionDe = (site: SiteTouristiqueAPI): 'sites_emblematiques' | 'sites_prives' =>
  site.categorie === 'prive' ? 'sites_prives' : 'sites_emblematiques'
</script>

<template>
  <section class="py-12">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <h2 class="font-oswald text-3xl md:text-4xl font-bold text-gray-900 mb-8">
        Sites touristiques
      </h2>

      <div class="space-y-12">
        <div>
          <div class="flex items-center justify-between mb-6">
            <h3 class="font-oswald text-2xl font-semibold text-custom-chocolat">
              Sites emblématiques
            </h3>
            <button
              type="button"
              class="px-4 py-2 bg-custom-chocolat text-white rounded-md hover:bg-custom-chocolat/90 transition-colors text-sm font-medium"
              @click="proposerSite('sites_emblematiques')"
            >
              Proposer un site
            </button>
          </div>

          <div v-if="chargementEmblematiques" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
            <div v-for="n in 3" :key="n" class="bg-gray-100 rounded-lg h-64 animate-pulse" />
          </div>

          <div
            v-else-if="sitesEmblematiques.length === 0"
            class="text-center py-10 bg-gray-50 rounded-lg"
          >
            <p class="text-gray-600 mb-4">Aucun site pour l'instant.</p>
            <button
              type="button"
              class="px-4 py-2 bg-custom-chocolat text-white rounded-md hover:bg-custom-chocolat/90 transition-colors text-sm font-medium"
              @click="proposerSite('sites_emblematiques')"
            >
              Proposer un site
            </button>
          </div>

          <div v-else class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
            <article
              v-for="site in sitesEmblematiques"
              :key="site.id"
              class="bg-white rounded-lg overflow-hidden shadow-sm hover:shadow-md transition-shadow"
            >
              <div class="aspect-video relative overflow-hidden">
                <img
                  v-if="site.image_url"
                  :src="resoudreUrlImage(site.image_url)"
                  :alt="site.nom"
                  class="w-full h-full object-cover"
                />
                <div
                  v-else
                  class="w-full h-full bg-gradient-to-br from-custom-chocolat to-custom-chocolat/60"
                />
              </div>
              <div class="p-4">
                <h4 class="font-oswald text-lg font-semibold text-gray-900 mb-2">{{ site.nom }}</h4>
                <p v-if="site.description" class="text-sm text-gray-600 line-clamp-3">
                  {{ site.description }}
                </p>
                <div class="flex items-center gap-3 mt-3 pt-3 border-t border-gray-100">
                  <button
                    type="button"
                    class="inline-flex items-center gap-1 text-xs font-medium text-custom-chocolat hover:underline"
                    @click="ouvrirContribution('edition', sectionDe(site), site)"
                  >
                    <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                    </svg>
                    Modifier
                  </button>
                  <button
                    type="button"
                    class="inline-flex items-center gap-1 text-xs font-medium text-red-600 hover:underline"
                    @click="ouvrirContribution('suppression', sectionDe(site), site)"
                  >
                    <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                    </svg>
                    Supprimer
                  </button>
                </div>
              </div>
            </article>
          </div>
        </div>

        <div>
          <div class="flex items-center justify-between mb-6">
            <h3 class="font-oswald text-2xl font-semibold text-custom-green">
              Sites privés
            </h3>
            <button
              type="button"
              class="px-4 py-2 bg-custom-green text-white rounded-md hover:bg-custom-green/90 transition-colors text-sm font-medium"
              @click="proposerSite('sites_prives')"
            >
              Proposer un site
            </button>
          </div>

          <div v-if="chargementPrives" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
            <div v-for="n in 3" :key="n" class="bg-gray-100 rounded-lg h-64 animate-pulse" />
          </div>

          <div
            v-else-if="sitesPrives.length === 0"
            class="text-center py-10 bg-gray-50 rounded-lg"
          >
            <p class="text-gray-600 mb-4">Aucun site pour l'instant.</p>
            <button
              type="button"
              class="px-4 py-2 bg-custom-green text-white rounded-md hover:bg-custom-green/90 transition-colors text-sm font-medium"
              @click="proposerSite('sites_prives')"
            >
              Proposer un site
            </button>
          </div>

          <div v-else class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
            <article
              v-for="site in sitesPrives"
              :key="site.id"
              class="bg-white rounded-lg overflow-hidden shadow-sm hover:shadow-md transition-shadow"
            >
              <div class="aspect-video relative overflow-hidden">
                <img
                  v-if="site.image_url"
                  :src="resoudreUrlImage(site.image_url)"
                  :alt="site.nom"
                  class="w-full h-full object-cover"
                />
                <div
                  v-else
                  class="w-full h-full bg-gradient-to-br from-custom-chocolat to-custom-green"
                />
              </div>
              <div class="p-4">
                <h4 class="font-oswald text-lg font-semibold text-gray-900 mb-2">{{ site.nom }}</h4>
                <p v-if="site.description" class="text-sm text-gray-600 line-clamp-3">
                  {{ site.description }}
                </p>
                <div class="flex items-center gap-3 mt-3 pt-3 border-t border-gray-100">
                  <button
                    type="button"
                    class="inline-flex items-center gap-1 text-xs font-medium text-custom-chocolat hover:underline"
                    @click="ouvrirContribution('edition', sectionDe(site), site)"
                  >
                    <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                    </svg>
                    Modifier
                  </button>
                  <button
                    type="button"
                    class="inline-flex items-center gap-1 text-xs font-medium text-red-600 hover:underline"
                    @click="ouvrirContribution('suppression', sectionDe(site), site)"
                  >
                    <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                    </svg>
                    Supprimer
                  </button>
                </div>
              </div>
            </article>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>
