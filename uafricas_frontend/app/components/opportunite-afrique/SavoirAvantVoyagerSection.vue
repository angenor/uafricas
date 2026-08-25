<script setup lang="ts">
import { useOpportuniteAfrique } from '~/composables/useOpportuniteAfrique'
import type {
  SavoirPratiqueAPI,
  CategorieSavoir,
  SectionAfripulse,
  TypeObjetContribution,
  FichePaysDetailAPI,
} from '~/composables/useOpportuniteAfrique'

interface Props {
  ficheId: string
  estAuthentifie: boolean
  /** Fiche territoire : fournit les valeurs du bloc « infos pratiques » */
  fiche?: FichePaysDetailAPI | null
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

/** Demande de contribution sur un champ scalaire ciblé de la fiche (infos pratiques) */
type ChampVoyagePayload = {
  section: string
  label: string
  valeurActuelle?: string
}

const emit = defineEmits<{
  (e: 'open-contribution', payload: OpenContributionPayload): void
  (e: 'open-champ-voyage', payload: ChampVoyagePayload): void
  (e: 'require-login'): void
}>()

/** Champs « infos pratiques » du bloc : chaque clé est une colonne fiche_pays */
const champsVoyage = [
  { section: 'voyage_langue_internationale', label: 'Langue internationale' },
  { section: 'voyage_langue_locale', label: 'Langue locale la plus utilisée' },
  { section: 'voyage_infos_visa', label: 'Informations visa' },
  { section: 'voyage_infos_sanitaires', label: 'Informations sanitaires' },
  { section: 'voyage_meteo', label: 'Météo' },
  { section: 'voyage_prises_electriques', label: 'Prises électriques' },
  { section: 'voyage_contacts_tourisme', label: 'Contacts officiels du tourisme' },
  { section: 'voyage_recommandations_securite', label: 'Recommandations sécurité (liens officiels)' },
] as const

const infosVoyage = computed(() =>
  champsVoyage.map(c => ({
    section: c.section,
    label: c.label,
    valeur: (props.fiche?.[c.section as keyof FichePaysDetailAPI] as string | null | undefined) ?? null,
  })),
)

const ouvrirChampVoyage = (section: string, label: string, valeur: string | null) => {
  if (!props.estAuthentifie) {
    emit('require-login')
    return
  }
  emit('open-champ-voyage', { section, label, valeurActuelle: valeur ?? undefined })
}

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

const { redirigerVersConnexion } = useAuth()

const ouvrirContribution = (
  type_contribution: 'ajout' | 'edition' | 'suppression',
  savoir?: SavoirPratiqueAPI,
) => {
  if (!props.estAuthentifie) {
    redirigerVersConnexion()
    return
  }
  emit('open-contribution', {
    type_objet_contribution: 'savoir_pratique',
    section_afripulse: 'savoir_avant_voyager',
    type_contribution,
    target_id: savoir?.id,
    donnees_actuelles: savoir
      ? {
          titre: savoir.titre,
          categorie: savoir.categorie,
          explication: savoir.explication,
          exemple: savoir.exemple,
        }
      : undefined,
    libelle: savoir?.titre,
  })
}

const proposerSavoir = () => {
  ouvrirContribution('ajout')
}
</script>

<template>
  <section class="bg-gray-50 transition-all" :class="replie ? 'py-5' : 'py-12'">
    <div class="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex items-center justify-between gap-4" :class="replie ? '' : 'mb-8'">
        <button
          type="button"
          class="flex items-center gap-3 text-left min-w-0 cursor-pointer hover:opacity-80 transition-opacity"
          :aria-expanded="!replie"
          @click="replie = !replie"
        >
          <font-awesome-icon
            :icon="['fas', 'chevron-down']"
            class="w-5 h-5 shrink-0 text-custom-chocolat transition-transform duration-200"
            :class="replie ? '-rotate-90' : ''"
          />
          <h2 class="font-oswald text-3xl md:text-4xl font-bold text-gray-900">
            À savoir avant de voyager
          </h2>
        </button>
        <button
          v-show="!replie"
          type="button"
          class="px-4 py-2 bg-custom-chocolat text-white rounded-md hover:bg-custom-chocolat/90 transition-colors text-sm font-medium"
          @click="proposerSavoir"
        >
          Proposer un savoir
        </button>
      </div>

      <div v-show="!replie">

      <!-- Bloc « Infos pratiques » : champs structurés contribuables par tous -->
      <div class="bg-white rounded-lg shadow-sm p-5 mb-6">
        <h3 class="font-oswald text-xl font-semibold text-gray-900 mb-1">Infos pratiques</h3>
        <p class="text-sm text-gray-500 mb-5">
          L'essentiel à connaître avant de partir. Chaque information peut être proposée ou corrigée par la communauté (validation par un administrateur).
        </p>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-x-6 gap-y-5">
          <div
            v-for="info in infosVoyage"
            :key="info.section"
            class="border-b border-gray-100 pb-4"
          >
            <div class="flex items-start justify-between gap-3">
              <h4 class="text-sm font-medium text-gray-500">{{ info.label }}</h4>
              <button
                type="button"
                class="shrink-0 inline-flex items-center gap-1 text-xs font-medium text-custom-chocolat hover:underline"
                @click="ouvrirChampVoyage(info.section, info.label, info.valeur)"
              >
                <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                </svg>
                {{ info.valeur ? 'Modifier' : 'Contribuer' }}
              </button>
            </div>
            <p v-if="info.valeur" class="mt-1 text-sm text-gray-800 leading-relaxed whitespace-pre-line">
              {{ info.valeur }}
            </p>
            <p v-else class="mt-1 text-sm italic text-gray-400">
              Non renseigné : proposez cette information.
            </p>
          </div>
        </div>
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
              <div class="flex items-center gap-3 mt-3">
                <button
                  type="button"
                  class="inline-flex items-center gap-1 text-xs font-medium text-custom-chocolat hover:underline"
                  @click="ouvrirContribution('edition', s)"
                >
                  <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                  </svg>
                  Modifier
                </button>
                <button
                  type="button"
                  class="inline-flex items-center gap-1 text-xs font-medium text-red-600 hover:underline"
                  @click="ouvrirContribution('suppression', s)"
                >
                  <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                  </svg>
                  Supprimer
                </button>
              </div>
            </article>
          </div>
        </div>
      </div>
      </div>
    </div>
  </section>
</template>
