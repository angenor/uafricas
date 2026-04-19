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

const { listerPersonnalites } = useOpportuniteAfrique()

const personnalites = ref<PersonnaliteConnueAPI[]>([])
const chargement = ref(true)
const domaineFiltre = ref<DomainePersonnalite | 'tous'>('tous')

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

const router = useRouter()

const proposerPersonnalite = () => {
  if (!props.estAuthentifie) {
    router.push('/login')
    return
  }
  emit('open-contribution', {
    type_objet_contribution: 'personnalite_connue',
    section_afripulse: 'personnalites',
    type_contribution: 'ajout',
  })
}
</script>

<template>
  <section class="py-12">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex items-center justify-between mb-8">
        <h2 class="font-oswald text-3xl md:text-4xl font-bold text-gray-900">
          Personnalités connues
        </h2>
        <button
          type="button"
          class="px-4 py-2 bg-custom-chocolat text-white rounded-md hover:bg-custom-chocolat/90 transition-colors text-sm font-medium"
          @click="proposerPersonnalite"
        >
          Proposer une personnalité
        </button>
      </div>

      <div class="flex flex-wrap gap-2 mb-8">
        <button
          v-for="d in domaines"
          :key="d.value"
          type="button"
          class="px-3 py-1.5 rounded-full text-sm font-medium transition-colors border"
          :class="domaineFiltre === d.value
            ? 'bg-custom-chocolat text-white border-custom-chocolat'
            : 'bg-white text-gray-700 border-gray-300 hover:bg-gray-50'"
          @click="domaineFiltre = d.value"
        >
          {{ d.label }}
        </button>
      </div>

      <div v-if="chargement" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
        <div v-for="n in 4" :key="n" class="bg-gray-100 rounded-lg h-80 animate-pulse" />
      </div>

      <div
        v-else-if="personnalites.length === 0"
        class="text-center py-12 bg-gray-50 rounded-lg"
      >
        <p class="text-gray-600">Aucune personnalité pour l'instant.</p>
      </div>

      <div
        v-else
        class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6"
      >
        <article
          v-for="p in personnalites"
          :key="p.id"
          class="bg-white rounded-lg overflow-hidden shadow-sm hover:shadow-md transition-shadow flex flex-col"
        >
          <div class="aspect-square bg-gray-100 relative overflow-hidden">
            <img
              v-if="p.portrait_url"
              :src="p.portrait_url"
              :alt="p.nom_complet"
              class="w-full h-full object-cover"
            />
            <div
              v-else
              class="w-full h-full flex items-center justify-center bg-gradient-to-br from-custom-chocolat/20 to-custom-green/20 font-oswald text-4xl font-bold text-custom-chocolat"
            >
              {{ getInitiales(p.nom_complet) }}
            </div>
          </div>
          <div class="p-4 flex-1 flex flex-col">
            <h3 class="font-oswald text-lg font-semibold text-gray-900 mb-1">
              {{ p.nom_complet }}
            </h3>
            <span
              class="inline-block self-start px-2 py-0.5 text-xs font-medium bg-custom-green/10 text-custom-green rounded-full mb-2"
            >
              {{ labelDomaine(p.domaine) }}
            </span>
            <p
              v-if="p.annee_naissance || p.annee_deces"
              class="text-xs text-gray-500 mb-2"
            >
              {{ p.annee_naissance ?? '?' }} — {{ p.annee_deces ?? '' }}
            </p>
            <p class="text-sm text-gray-600 line-clamp-3 flex-1">
              {{ p.biographie_courte }}
            </p>
            <a
              v-if="p.lien_reference"
              :href="p.lien_reference"
              target="_blank"
              rel="noopener noreferrer"
              class="text-xs text-custom-chocolat hover:underline mt-3 inline-block"
            >
              En savoir plus
            </a>
          </div>
        </article>
      </div>
    </div>
  </section>
</template>
