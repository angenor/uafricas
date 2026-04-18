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

const emit = defineEmits<{
  (
    e: 'open-contribution',
    payload: {
      type_objet_contribution: TypeObjetContribution
      section_afripulse: SectionAfripulse
      type_contribution: 'ajout'
    }
  ): void
}>()

const { listerSitesTouristiques } = useOpportuniteAfrique()

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

const proposerSite = (section: 'sites_emblematiques' | 'sites_prives') => {
  if (!props.estAuthentifie) {
    router.push('/login')
    return
  }
  emit('open-contribution', {
    type_objet_contribution: 'site_touristique',
    section_afripulse: section,
    type_contribution: 'ajout',
  })
}
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
                  :src="site.image_url"
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
                  :src="site.image_url"
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
              </div>
            </article>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>
