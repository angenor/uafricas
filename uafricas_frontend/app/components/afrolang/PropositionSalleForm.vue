<template>
  <form
    class="bg-white rounded-2xl shadow-sm border border-gray-200 p-6 space-y-5"
    @submit.prevent="soumettre"
  >
    <header class="flex items-center gap-3">
      <span class="inline-flex items-center justify-center w-10 h-10 rounded-full bg-orange-50 text-orange-600">
        <font-awesome-icon :icon="['fas', 'lightbulb']" class="w-5 h-5" />
      </span>
      <div>
        <h2 class="text-lg font-semibold text-gray-900">Proposer une salle publique</h2>
        <p class="text-sm text-gray-500">Décrivez la langue africaine et le groupe ethnique concernés.</p>
      </div>
    </header>

    <div v-if="messageSucces" class="rounded-lg border border-emerald-200 bg-emerald-50 px-4 py-3 text-sm text-emerald-800 flex items-start gap-2">
      <font-awesome-icon :icon="['fas', 'circle-check']" class="w-4 h-4 mt-0.5" />
      <span>{{ messageSucces }}</span>
    </div>

    <div v-if="messageErreur" class="rounded-lg border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-800 flex items-start gap-2">
      <font-awesome-icon :icon="['fas', 'triangle-exclamation']" class="w-4 h-4 mt-0.5" />
      <span>{{ messageErreur }}</span>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div class="md:col-span-2">
        <label for="prop-titre" class="block text-sm font-medium text-gray-700 mb-1">
          Titre <span class="text-red-500">*</span>
        </label>
        <input
          id="prop-titre"
          v-model="form.titre"
          type="text"
          maxlength="350"
          required
          class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm focus:border-orange-500 focus:ring-2 focus:ring-orange-200 focus:outline-none"
          placeholder="Ex. Apprenons le wolof ensemble"
        >
      </div>

      <div>
        <label for="prop-langue" class="block text-sm font-medium text-gray-700 mb-1">
          Langue cible <span class="text-red-500">*</span>
        </label>
        <input
          id="prop-langue"
          v-model="form.langue_cible"
          type="text"
          maxlength="100"
          required
          class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm focus:border-orange-500 focus:ring-2 focus:ring-orange-200 focus:outline-none"
          placeholder="Ex. Wolof"
        >
      </div>

      <div>
        <label for="prop-langue-code" class="block text-sm font-medium text-gray-700 mb-1">
          Code de langue (facultatif)
        </label>
        <input
          id="prop-langue-code"
          v-model="form.langue_code"
          type="text"
          maxlength="40"
          class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm focus:border-orange-500 focus:ring-2 focus:ring-orange-200 focus:outline-none"
          placeholder="Ex. wo"
        >
      </div>

      <div class="md:col-span-2">
        <label for="prop-groupe" class="block text-sm font-medium text-gray-700 mb-1">
          Groupe ethnique <span class="text-red-500">*</span>
        </label>
        <select
          id="prop-groupe"
          v-model="groupeSelection"
          required
          class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm focus:border-orange-500 focus:ring-2 focus:ring-orange-200 focus:outline-none"
        >
          <option value="" disabled>Sélectionner un groupe ethnique</option>
          <option
            v-for="g in groupesDisponibles"
            :key="g.id"
            :value="g.id"
          >
            {{ g.nom }}<span v-if="g.pays_nom"> · {{ g.pays_nom }}</span>
          </option>
          <option :value="AUTRE">Autre (préciser)…</option>
        </select>
        <p class="text-xs text-gray-500 mt-1">
          Liste non exhaustive : si votre groupe n'y figure pas, choisissez
          « Autre » et indiquez son nom.
        </p>

        <!-- Champ texte libre quand « Autre » est sélectionné -->
        <div v-if="groupeSelection === AUTRE" class="mt-3">
          <label for="prop-groupe-libre" class="block text-sm font-medium text-gray-700 mb-1">
            Nom du groupe ethnique <span class="text-red-500">*</span>
          </label>
          <input
            id="prop-groupe-libre"
            v-model="form.groupe_ethnique_libre"
            type="text"
            maxlength="250"
            class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm focus:border-orange-500 focus:ring-2 focus:ring-orange-200 focus:outline-none"
            placeholder="Ex. Bassa, Sérère, Créole haïtien…"
          >
        </div>
      </div>

      <div class="md:col-span-2">
        <label class="block text-sm font-medium text-gray-700 mb-1">
          Territoire d'origine <span class="text-red-500">*</span>
        </label>
        <input
          v-model="rechercheTerritoire"
          type="text"
          class="w-full mb-2 rounded-lg border border-gray-300 px-3 py-2 text-sm focus:border-orange-500 focus:ring-2 focus:ring-orange-200 focus:outline-none"
          placeholder="Rechercher un territoire…"
        >
        <div class="rounded-lg border border-gray-300 p-3 max-h-64 overflow-y-auto bg-gray-50 space-y-4">
          <div v-if="!territoires.length" class="text-sm text-gray-500">
            Chargement des territoires…
          </div>
          <div v-else-if="!territoiresParContinent.length" class="text-sm text-gray-500">
            Aucun territoire ne correspond à votre recherche.
          </div>
          <fieldset
            v-for="bloc in territoiresParContinent"
            v-else
            :key="bloc.continent"
          >
            <legend class="text-xs font-semibold uppercase tracking-wide text-gray-500 mb-2">
              {{ bloc.continent }}
            </legend>
            <div class="grid grid-cols-2 sm:grid-cols-3 gap-2">
              <label
                v-for="p in bloc.territoires"
                :key="p.id"
                class="flex items-center gap-2 text-sm text-gray-700 cursor-pointer hover:text-gray-900"
              >
                <input
                  v-model="form.pays_origine_ids"
                  type="checkbox"
                  :value="p.id"
                  class="w-4 h-4 rounded border-gray-300 text-orange-600 focus:ring-orange-500"
                >
                <span>{{ p.nom }}</span>
              </label>
            </div>
          </fieldset>
        </div>
        <p class="text-xs text-gray-500 mt-1">
          Sélectionnez au moins un territoire où la langue cible est parlée, y
          compris hors d'Afrique (diaspora, créoles, langues afro-descendantes).
          {{ form.pays_origine_ids.length }} sélectionné(s).
        </p>
      </div>

      <div class="md:col-span-2">
        <label for="prop-description" class="block text-sm font-medium text-gray-700 mb-1">
          Description <span class="text-red-500">*</span>
        </label>
        <textarea
          id="prop-description"
          v-model="form.description"
          rows="3"
          required
          class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm focus:border-orange-500 focus:ring-2 focus:ring-orange-200 focus:outline-none"
          placeholder="Présentez brièvement la salle envisagée…"
        />
      </div>

      <div class="md:col-span-2">
        <label for="prop-justification" class="block text-sm font-medium text-gray-700 mb-1">
          Justification <span class="text-red-500">*</span>
        </label>
        <textarea
          id="prop-justification"
          v-model="form.justification"
          rows="3"
          required
          class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm focus:border-orange-500 focus:ring-2 focus:ring-orange-200 focus:outline-none"
          placeholder="Pourquoi cette salle serait utile ? Quel public ?"
        />
      </div>
    </div>

    <div class="flex items-center justify-between gap-3 pt-2">
      <p class="text-xs text-gray-500">
        Votre proposition sera examinée par un administrateur de la plateforme.
      </p>
      <button
        type="submit"
        :disabled="enCours || !formulaireValide"
        class="inline-flex items-center gap-2 rounded-lg bg-orange-600 px-4 py-2 text-sm font-semibold text-white shadow-sm hover:bg-orange-700 disabled:bg-gray-300 disabled:cursor-not-allowed transition-colors"
      >
        <font-awesome-icon
          :icon="['fas', enCours ? 'spinner' : 'paper-plane']"
          :class="enCours ? 'animate-spin' : ''"
          class="w-4 h-4"
        />
        {{ enCours ? 'Envoi…' : 'Soumettre la proposition' }}
      </button>
    </div>
  </form>
</template>

<script setup lang="ts">
import { computed, reactive, ref } from 'vue'
import type {
  GroupeEthniqueResume,
  PropositionSalle,
  SoumettrePropositionPayload,
  TerritoireAPI,
} from '~/composables/useAfrolang'

interface GroupeOption {
  id: string
  nom: string
  pays_nom: string | null
}

const props = defineProps<{
  groupesDisponibles: GroupeOption[]
  territoires: TerritoireAPI[]
}>()

const emit = defineEmits<{
  (e: 'soumis', proposition: PropositionSalle): void
}>()

const { proposerSalle } = useAfrolang()

/** Valeur sentinelle du sélecteur de groupe pour l'option « Autre ». */
const AUTRE = '__autre__'

const enCours = ref(false)
const messageErreur = ref<string | null>(null)
const messageSucces = ref<string | null>(null)

/** Sélection du groupe ethnique : '' (aucun), un UUID de groupe, ou AUTRE. */
const groupeSelection = ref<string>('')

const form = reactive({
  titre: '',
  description: '',
  justification: '',
  langue_cible: '',
  langue_code: '',
  groupe_ethnique_libre: '',
  pays_origine_ids: [] as string[],
})

// ── Territoires groupés par continent (Afrique en tête, ordre fourni par l'API) ──
const rechercheTerritoire = ref('')

const territoiresParContinent = computed(() => {
  const filtre = rechercheTerritoire.value.trim().toLowerCase()
  const groupes = new Map<string, TerritoireAPI[]>()
  for (const t of props.territoires) {
    if (filtre && !t.nom.toLowerCase().includes(filtre)) continue
    const cle = t.continent || 'Autres'
    if (!groupes.has(cle)) groupes.set(cle, [])
    groupes.get(cle)!.push(t)
  }
  return Array.from(groupes, ([continent, territoires]) => ({ continent, territoires }))
})

const groupeValide = computed(() =>
  groupeSelection.value === AUTRE
    ? form.groupe_ethnique_libre.trim().length > 0
    : groupeSelection.value.length > 0,
)

const formulaireValide = computed(() =>
  form.titre.trim().length > 0
  && form.description.trim().length > 0
  && form.justification.trim().length > 0
  && form.langue_cible.trim().length > 0
  && groupeValide.value
  && form.pays_origine_ids.length > 0,
)

const reinitialiser = () => {
  form.titre = ''
  form.description = ''
  form.justification = ''
  form.langue_cible = ''
  form.langue_code = ''
  form.groupe_ethnique_libre = ''
  form.pays_origine_ids = []
  groupeSelection.value = ''
  rechercheTerritoire.value = ''
}

const soumettre = async () => {
  messageErreur.value = null
  messageSucces.value = null
  if (!formulaireValide.value) return
  enCours.value = true
  try {
    const estAutre = groupeSelection.value === AUTRE
    const payload: SoumettrePropositionPayload = {
      titre: form.titre.trim(),
      description: form.description.trim(),
      justification: form.justification.trim(),
      langue_cible: form.langue_cible.trim(),
      langue_code: form.langue_code?.trim() || null,
      groupe_ethnique_id: estAutre ? null : groupeSelection.value,
      groupe_ethnique_libre: estAutre ? form.groupe_ethnique_libre.trim() : null,
      pays_origine_ids: [...form.pays_origine_ids],
    }
    const proposition = await proposerSalle(payload)
    if (proposition) {
      messageSucces.value = 'Proposition soumise avec succès. Un administrateur vous répondra prochainement.'
      emit('soumis', proposition)
      reinitialiser()
    }
    else {
      messageErreur.value = 'La soumission a échoué. Vérifiez les champs et réessayez.'
    }
  }
  finally {
    enCours.value = false
  }
}

// Évite "unused" lint sur GroupeEthniqueResume si Vue compile en strict
void ({} as GroupeEthniqueResume)
</script>
