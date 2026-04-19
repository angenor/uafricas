<script setup lang="ts">
import { useOpportuniteAfrique } from '~/composables/useOpportuniteAfrique'
import type {
  SavoirPratiqueAPI,
  CategorieSavoir,
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

const { listerSavoirsPratiques } = useOpportuniteAfrique()

const savoirs = ref<SavoirPratiqueAPI[]>([])
const chargement = ref(true)
const categorieOuverte = ref<CategorieSavoir | null>(null)

const categories: { value: CategorieSavoir, label: string }[] = [
  { value: 'langue_argot', label: 'Langue et argot' },
  { value: 'coutumes', label: 'Coutumes' },
  { value: 'etiquette', label: 'Étiquette' },
  { value: 'securite', label: 'Sécurité' },
  { value: 'sante', label: 'Santé' },
  { value: 'transports', label: 'Transports' },
  { value: 'autre', label: 'Autre' },
]

const savoirsParCategorie = computed(() => {
  const groupes: Record<string, SavoirPratiqueAPI[]> = {}
  for (const s of savoirs.value) {
    if (!groupes[s.categorie]) groupes[s.categorie] = []
    groupes[s.categorie]!.push(s)
  }
  return groupes
})

onMounted(async () => {
  chargement.value = true
  savoirs.value = await listerSavoirsPratiques(props.ficheId)
  chargement.value = false
})

const basculerCategorie = (cat: CategorieSavoir) => {
  categorieOuverte.value = categorieOuverte.value === cat ? null : cat
}

const router = useRouter()

const proposerSavoir = () => {
  if (!props.estAuthentifie) {
    router.push('/login')
    return
  }
  emit('open-contribution', {
    type_objet_contribution: 'savoir_pratique',
    section_afripulse: 'savoir_avant_voyager',
    type_contribution: 'ajout',
  })
}
</script>

<template>
  <section class="py-12 bg-gray-50">
    <div class="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex items-center justify-between mb-8">
        <h2 class="font-oswald text-3xl md:text-4xl font-bold text-gray-900">
          À savoir avant de voyager
        </h2>
        <button
          type="button"
          class="px-4 py-2 bg-custom-chocolat text-white rounded-md hover:bg-custom-chocolat/90 transition-colors text-sm font-medium"
          @click="proposerSavoir"
        >
          Proposer un savoir
        </button>
      </div>

      <div v-if="chargement" class="space-y-3">
        <div v-for="n in 4" :key="n" class="bg-white rounded-lg h-16 animate-pulse" />
      </div>

      <div
        v-else-if="savoirs.length === 0"
        class="text-center py-12 bg-white rounded-lg"
      >
        <p class="text-gray-600">Aucun savoir pratique pour l'instant.</p>
      </div>

      <div v-else class="space-y-3">
        <div
          v-for="cat in categories"
          :key="cat.value"
          class="bg-white rounded-lg overflow-hidden shadow-sm"
        >
          <button
            v-if="savoirsParCategorie[cat.value] && savoirsParCategorie[cat.value]!.length > 0"
            type="button"
            class="w-full flex items-center justify-between px-5 py-4 text-left hover:bg-gray-50 transition-colors"
            @click="basculerCategorie(cat.value)"
          >
            <span class="font-oswald text-lg font-semibold text-gray-900">
              {{ cat.label }}
              <span class="ml-2 text-sm font-normal text-gray-500">
                ({{ savoirsParCategorie[cat.value]!.length }})
              </span>
            </span>
            <svg
              class="w-5 h-5 text-gray-500 transition-transform"
              :class="{ 'rotate-180': categorieOuverte === cat.value }"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
            </svg>
          </button>

          <div
            v-if="categorieOuverte === cat.value && savoirsParCategorie[cat.value]"
            class="border-t border-gray-200 divide-y divide-gray-100"
          >
            <article
              v-for="s in savoirsParCategorie[cat.value]"
              :key="s.id"
              class="px-5 py-4"
            >
              <h3 class="font-semibold text-gray-900 mb-2">{{ s.titre }}</h3>
              <p class="text-sm text-gray-700 leading-relaxed mb-2">{{ s.explication }}</p>
              <p v-if="s.exemple" class="text-sm italic text-gray-500 border-l-2 border-custom-green pl-3">
                {{ s.exemple }}
              </p>
            </article>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>
