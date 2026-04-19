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

const { listerSecteursOpportunites } = useOpportuniteAfrique()

const secteurs = ref<SecteurOpportuniteAPI[]>([])
const chargement = ref(true)

onMounted(async () => {
  chargement.value = true
  secteurs.value = await listerSecteursOpportunites(props.ficheId)
  chargement.value = false
})

const router = useRouter()

const proposerSecteur = () => {
  if (!props.estAuthentifie) {
    router.push('/login')
    return
  }
  emit('open-contribution', {
    type_objet_contribution: 'secteur_developpement',
    section_afripulse: 'secteurs_opportunites',
    type_contribution: 'ajout',
  })
}
</script>

<template>
  <section class="py-12 bg-gray-50">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex items-center justify-between mb-8">
        <h2 class="font-oswald text-3xl md:text-4xl font-bold text-gray-900">
          Secteurs d'opportunités
        </h2>
        <button
          type="button"
          class="px-4 py-2 bg-custom-green text-white rounded-md hover:bg-custom-green/90 transition-colors text-sm font-medium"
          @click="proposerSecteur"
        >
          Proposer un secteur
        </button>
      </div>

      <div v-if="chargement" class="space-y-4">
        <div v-for="n in 3" :key="n" class="bg-gray-200 rounded-lg h-24 animate-pulse" />
      </div>

      <div
        v-else-if="secteurs.length === 0"
        class="text-center py-12 bg-white rounded-lg"
      >
        <p class="text-gray-600 mb-4">Aucun secteur pour l'instant.</p>
        <button
          type="button"
          class="px-4 py-2 bg-custom-green text-white rounded-md hover:bg-custom-green/90 transition-colors text-sm font-medium"
          @click="proposerSecteur"
        >
          Proposer un secteur
        </button>
      </div>

      <ul v-else class="space-y-4">
        <li
          v-for="secteur in secteurs"
          :key="secteur.id"
          class="bg-white rounded-lg p-6 shadow-sm hover:shadow-md transition-shadow border-l-4 border-custom-green"
        >
          <h3 class="font-oswald text-xl font-semibold text-gray-900 mb-2">
            {{ secteur.nom }}
          </h3>
          <p v-if="secteur.description" class="text-gray-600 leading-relaxed">
            {{ secteur.description }}
          </p>
        </li>
      </ul>
    </div>
  </section>
</template>
