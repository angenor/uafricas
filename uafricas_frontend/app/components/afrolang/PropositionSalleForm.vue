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
          v-model="form.groupe_ethnique_id"
          required
          class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm focus:border-orange-500 focus:ring-2 focus:ring-orange-200 focus:outline-none"
        >
          <option value="" disabled>— Sélectionner un groupe ethnique —</option>
          <option
            v-for="g in groupesDisponibles"
            :key="g.id"
            :value="g.id"
          >
            {{ g.nom }}<span v-if="g.pays_nom"> · {{ g.pays_nom }}</span>
          </option>
        </select>
        <p class="text-xs text-gray-500 mt-1">
          Seuls les groupes ethniques sans salle active sont proposés.
        </p>
      </div>

      <div class="md:col-span-2">
        <label class="block text-sm font-medium text-gray-700 mb-1">
          Pays d'origine <span class="text-red-500">*</span>
        </label>
        <div class="rounded-lg border border-gray-300 p-3 max-h-48 overflow-y-auto bg-gray-50">
          <div v-if="!paysDisponibles.length" class="text-sm text-gray-500">
            Chargement des pays africains…
          </div>
          <div v-else class="grid grid-cols-2 sm:grid-cols-3 gap-2">
            <label
              v-for="p in paysDisponibles"
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
        </div>
        <p class="text-xs text-gray-500 mt-1">
          Sélectionnez au moins un pays où la langue cible est parlée à l'origine.
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
  PaysOrigineLight,
  PropositionSalle,
  SoumettrePropositionPayload,
} from '~/composables/useAfrolang'

interface GroupeOption {
  id: string
  nom: string
  pays_nom: string | null
}

const props = defineProps<{
  groupesDisponibles: GroupeOption[]
  paysDisponibles: PaysOrigineLight[]
}>()

const emit = defineEmits<{
  (e: 'soumis', proposition: PropositionSalle): void
}>()

const { proposerSalle } = useAfrolang()

const enCours = ref(false)
const messageErreur = ref<string | null>(null)
const messageSucces = ref<string | null>(null)

const form = reactive<SoumettrePropositionPayload>({
  titre: '',
  description: '',
  justification: '',
  langue_cible: '',
  langue_code: '',
  groupe_ethnique_id: '',
  pays_origine_ids: [],
})

const formulaireValide = computed(() =>
  form.titre.trim().length > 0
  && form.description.trim().length > 0
  && form.justification.trim().length > 0
  && form.langue_cible.trim().length > 0
  && form.groupe_ethnique_id.length > 0
  && form.pays_origine_ids.length > 0,
)

const reinitialiser = () => {
  form.titre = ''
  form.description = ''
  form.justification = ''
  form.langue_cible = ''
  form.langue_code = ''
  form.groupe_ethnique_id = ''
  form.pays_origine_ids = []
}

const soumettre = async () => {
  messageErreur.value = null
  messageSucces.value = null
  if (!formulaireValide.value) return
  enCours.value = true
  try {
    const payload: SoumettrePropositionPayload = {
      titre: form.titre.trim(),
      description: form.description.trim(),
      justification: form.justification.trim(),
      langue_cible: form.langue_cible.trim(),
      langue_code: form.langue_code?.trim() || null,
      groupe_ethnique_id: form.groupe_ethnique_id,
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
