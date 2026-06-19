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

onMounted(async () => {
  chargement.value = true
  secteurs.value = await listerSecteursOpportunites(props.ficheId)
  chargement.value = false
})

const router = useRouter()

const ouvrirContribution = (
  type_contribution: 'ajout' | 'edition' | 'suppression',
  secteur?: SecteurOpportuniteAPI,
) => {
  if (!props.estAuthentifie) {
    router.push('/login')
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
          class="flex flex-col sm:flex-row gap-5 bg-white rounded-lg p-6 shadow-sm hover:shadow-md transition-shadow border-l-4 border-custom-green"
        >
          <!-- Image illustrative (optionnelle) -->
          <img
            v-if="secteur.image_url"
            :src="resoudreUrlImage(secteur.image_url)"
            :alt="secteur.nom"
            class="w-full sm:w-44 h-40 sm:h-auto sm:max-h-44 shrink-0 rounded-lg object-cover"
          />

          <div class="min-w-0 flex-1">
            <h3 class="font-oswald text-xl font-semibold text-gray-900 mb-1">
              {{ secteur.nom }}
            </h3>
            <p v-if="secteur.localite" class="inline-flex items-center gap-1.5 text-sm text-gray-500 mb-2">
              <font-awesome-icon :icon="['fas', 'location-dot']" class="w-3.5 h-3.5 text-custom-chocolat" />
              {{ secteur.localite }}
            </p>
            <p v-if="secteur.description" class="text-gray-600 leading-relaxed">
              {{ secteur.description }}
            </p>

            <!-- Références -->
            <p v-if="secteur.references_utiles" class="mt-3 text-sm text-gray-500">
              <span class="font-medium text-gray-600">Références : </span>{{ secteur.references_utiles }}
            </p>

            <!-- Contacts + site web -->
            <div
              v-if="secteur.contact_telephone || secteur.contact_courriel || secteur.contact_adresse || secteur.site_web_url"
              class="mt-3 flex flex-col gap-1.5 text-sm rounded-md bg-gray-50 px-4 py-3"
            >
              <a
                v-if="secteur.contact_telephone"
                :href="`tel:${secteur.contact_telephone}`"
                class="inline-flex items-center gap-2 text-gray-700 hover:text-custom-green"
              >
                <font-awesome-icon :icon="['fas', 'phone']" class="w-3.5 h-3.5 text-custom-green" />
                {{ secteur.contact_telephone }}
              </a>
              <a
                v-if="secteur.contact_courriel"
                :href="`mailto:${secteur.contact_courriel}`"
                class="inline-flex items-center gap-2 text-gray-700 hover:text-custom-green break-all"
              >
                <font-awesome-icon :icon="['fas', 'envelope']" class="w-3.5 h-3.5 text-custom-green" />
                {{ secteur.contact_courriel }}
              </a>
              <span v-if="secteur.contact_adresse" class="inline-flex items-center gap-2 text-gray-700">
                <font-awesome-icon :icon="['fas', 'location-dot']" class="w-3.5 h-3.5 text-custom-green" />
                {{ secteur.contact_adresse }}
              </span>
              <a
                v-if="secteur.site_web_url"
                :href="secteur.site_web_url"
                target="_blank"
                rel="noopener noreferrer"
                class="inline-flex items-center gap-2 font-medium text-custom-chocolat hover:underline"
              >
                <font-awesome-icon :icon="['fas', 'arrow-up-right-from-square']" class="w-3.5 h-3.5" />
                Visiter le site web
              </a>
            </div>

          <div class="flex items-center gap-3 mt-3 pt-3 border-t border-gray-100">
            <button
              type="button"
              class="inline-flex items-center gap-1 text-xs font-medium text-custom-chocolat hover:underline"
              @click="ouvrirContribution('edition', secteur)"
            >
              <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
              </svg>
              Modifier
            </button>
            <button
              type="button"
              class="inline-flex items-center gap-1 text-xs font-medium text-red-600 hover:underline"
              @click="ouvrirContribution('suppression', secteur)"
            >
              <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
              </svg>
              Supprimer
            </button>
          </div>
          </div>
        </li>
      </ul>
    </div>
  </section>
</template>
