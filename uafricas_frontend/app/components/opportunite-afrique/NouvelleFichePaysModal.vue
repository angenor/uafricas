<script setup lang="ts">
/**
 * NouvelleFichePaysModal (T051-T053) : Tailwind v4 pur, aucune classe daisyUI.
 *
 * US3 : proposer la création d'une fiche pays africain non encore référencée.
 * Gère les erreurs typées 401 / 409 (fiche existante) / 422 (hors périmètre)
 * / 429 (rate-limit). Affiche un message UX explicite sur conflit avec lien
 * vers la fiche existante.
 */
import { PAYS_AFRICAINS_ISO2 } from '~/constants/afripulsePaysAutorises'
import { NOMS_PAYS_FR } from '~/constants/nomsPaysAfrique'

const props = defineProps<{ visible: boolean }>()
const emit = defineEmits<{
  (e: 'fermer'): void
  (e: 'succes', payload: { id: string, code_iso2: string }): void
}>()

const { creerFichePays } = useOpportuniteAfrique()

const form = reactive({
  code_iso2: '',
  slogan: '',
  population: '' as string,
  superficie_km2: '' as string,
  biographie: '',
  contexte: '',
  monnaie: '',
  langue_officielle: '',
  langues_populaires: '',
  hymne_national: '',
  fuseau_horaire: '',
  image_couverture_url: '',
  image_drapeau_url: '',
  image_embleme_url: '',
  justification: '',
})

const chargement = ref(false)
const erreurGlobale = ref<string | null>(null)
const conflit = ref<{ fiche_pays_id?: string, message: string } | null>(null)
const horsPerimetre = ref(false)

// Trier alphabétiquement par nom français
const paysOrdonnes = computed(() => {
  return PAYS_AFRICAINS_ISO2
    .map(code => ({ code, nom: NOMS_PAYS_FR[code] ?? code.toUpperCase() }))
    .sort((a, b) => a.nom.localeCompare(b.nom, 'fr'))
})

const reinitialiser = () => {
  Object.assign(form, {
    code_iso2: '', slogan: '', population: '', superficie_km2: '',
    biographie: '', contexte: '', monnaie: '', langue_officielle: '',
    langues_populaires: '', hymne_national: '', fuseau_horaire: '',
    image_couverture_url: '', image_drapeau_url: '', image_embleme_url: '',
    justification: '',
  })
  erreurGlobale.value = null
  conflit.value = null
  horsPerimetre.value = false
}

const fermer = () => {
  reinitialiser()
  emit('fermer')
}

const soumettre = async () => {
  erreurGlobale.value = null
  conflit.value = null
  horsPerimetre.value = false

  if (!form.code_iso2) {
    erreurGlobale.value = 'Veuillez sélectionner un territoire.'
    return
  }
  if (form.biographie && form.biographie.length > 5000) {
    erreurGlobale.value = 'La biographie ne doit pas dépasser 5000 caractères.'
    return
  }

  chargement.value = true
  try {
    const payload = {
      code_iso2: form.code_iso2.toLowerCase(),
      slogan: form.slogan.trim() || undefined,
      population: form.population ? Number(form.population) : undefined,
      superficie_km2: form.superficie_km2 ? Number(form.superficie_km2) : undefined,
      biographie: form.biographie.trim() || undefined,
      contexte: form.contexte.trim() || undefined,
      monnaie: form.monnaie.trim() || undefined,
      langue_officielle: form.langue_officielle.trim() || undefined,
      langues_populaires: form.langues_populaires.trim() || undefined,
      hymne_national: form.hymne_national.trim() || undefined,
      fuseau_horaire: form.fuseau_horaire.trim() || undefined,
      image_couverture_url: form.image_couverture_url.trim() || undefined,
      image_drapeau_url: form.image_drapeau_url.trim() || undefined,
      image_embleme_url: form.image_embleme_url.trim() || undefined,
      justification: form.justification.trim() || undefined,
    }
    const result = await creerFichePays(payload)
    if (result) {
      emit('succes', { id: result.id, code_iso2: payload.code_iso2 })
      fermer()
    }
  }
  catch (e: any) {
    const status = e?.response?.status ?? e?.statusCode
    const message = e?.data?.error || e?.message || 'Erreur inconnue'
    if (status === 409) {
      conflit.value = { fiche_pays_id: e?.data?.fiche_pays_id, message }
    }
    else if (status === 422) {
      horsPerimetre.value = true
    }
    else {
      erreurGlobale.value = message
    }
  }
  finally {
    chargement.value = false
  }
}
</script>

<template>
  <Teleport to="body">
    <div
      v-if="props.visible"
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4"
      @click.self="fermer"
    >
      <div class="bg-white rounded-lg shadow-xl w-full max-w-2xl max-h-[90vh] overflow-y-auto">
        <!-- En-tête -->
        <div class="flex items-center justify-between p-5 border-b border-gray-200">
          <h2 class="text-xl font-bold text-gray-900">
            Proposer une nouvelle fiche territoire
          </h2>
          <button
            type="button"
            class="text-gray-400 hover:text-gray-600 transition-colors"
            @click="fermer"
          >
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        <!-- Messages d'erreur UX -->
        <div v-if="conflit" class="m-5 p-4 bg-amber-50 border border-amber-200 rounded-lg text-sm text-amber-800">
          <p class="font-semibold mb-1">Cette fiche territoire existe déjà</p>
          <p class="mb-2">{{ conflit.message }}</p>
          <NuxtLink
            v-if="conflit.fiche_pays_id"
            :to="`/opportunite-afrique/${conflit.fiche_pays_id}`"
            class="inline-block mt-1 text-custom-chocolat font-medium underline"
          >
            Voir la fiche existante &rarr;
          </NuxtLink>
          <p class="mt-2 text-xs text-amber-700">
            Proposez plutôt une modification depuis la page du territoire.
          </p>
        </div>

        <div v-if="horsPerimetre" class="m-5 p-4 bg-red-50 border border-red-200 rounded-lg text-sm text-red-800">
          <p class="font-semibold mb-1">Territoire hors périmètre Afripulse</p>
          <p>Afripulse couvre uniquement les 54 territoires africains (codes ISO 2).</p>
        </div>

        <div v-if="erreurGlobale" class="m-5 p-3 bg-red-50 border border-red-200 rounded text-sm text-red-800">
          {{ erreurGlobale }}
        </div>

        <!-- Formulaire -->
        <form @submit.prevent="soumettre">
          <div class="p-5 space-y-4">
            <!-- Pays -->
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">
                Territoire <span class="text-red-500">*</span>
              </label>
              <select
                v-model="form.code_iso2"
                required
                class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-custom-green"
              >
                <option value="">Sélectionnez un territoire africain</option>
                <option
                  v-for="p in paysOrdonnes"
                  :key="p.code"
                  :value="p.code"
                >
                  {{ p.nom }}
                </option>
              </select>
            </div>

            <!-- Slogan -->
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Slogan</label>
              <input
                v-model="form.slogan"
                type="text"
                maxlength="300"
                class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-custom-green"
              />
            </div>

            <!-- Population / Superficie -->
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Population</label>
                <input
                  v-model="form.population"
                  type="number"
                  min="0"
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-custom-green"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Superficie (km²)</label>
                <input
                  v-model="form.superficie_km2"
                  type="number"
                  min="0"
                  step="0.01"
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-custom-green"
                />
              </div>
            </div>

            <!-- Monnaie / Fuseau -->
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Monnaie</label>
                <input
                  v-model="form.monnaie"
                  type="text"
                  maxlength="50"
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-custom-green"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Fuseau horaire</label>
                <input
                  v-model="form.fuseau_horaire"
                  type="text"
                  maxlength="50"
                  placeholder="UTC+1"
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-custom-green"
                />
              </div>
            </div>

            <!-- Langues -->
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Langue officielle</label>
                <input
                  v-model="form.langue_officielle"
                  type="text"
                  maxlength="100"
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-custom-green"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Langues populaires</label>
                <input
                  v-model="form.langues_populaires"
                  type="text"
                  maxlength="200"
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-custom-green"
                />
              </div>
            </div>

            <!-- Biographie -->
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Biographie</label>
              <textarea
                v-model="form.biographie"
                rows="4"
                maxlength="5000"
                class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-custom-green"
              />
              <p class="mt-1 text-xs text-gray-500">{{ form.biographie.length }} / 5000</p>
            </div>

            <!-- Contexte -->
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Contexte général</label>
              <textarea
                v-model="form.contexte"
                rows="3"
                maxlength="3000"
                class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-custom-green"
              />
            </div>

            <!-- Images -->
            <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">URL drapeau</label>
                <input
                  v-model="form.image_drapeau_url"
                  type="url"
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-custom-green"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">URL couverture</label>
                <input
                  v-model="form.image_couverture_url"
                  type="url"
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-custom-green"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">URL emblème</label>
                <input
                  v-model="form.image_embleme_url"
                  type="url"
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-custom-green"
                />
              </div>
            </div>

            <!-- Justification -->
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">
                Justification (optionnelle)
              </label>
              <textarea
                v-model="form.justification"
                rows="2"
                maxlength="1000"
                placeholder="Expliquez brièvement vos sources…"
                class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-custom-green"
              />
            </div>
          </div>

          <!-- Pied de modal -->
          <div class="flex items-center justify-end gap-2 p-5 border-t border-gray-200 bg-gray-50">
            <button
              type="button"
              class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
              :disabled="chargement"
              @click="fermer"
            >
              Annuler
            </button>
            <button
              type="submit"
              class="px-4 py-2 text-sm font-medium text-white bg-custom-green rounded-md hover:bg-custom-green/90 disabled:opacity-50 disabled:cursor-not-allowed"
              :disabled="chargement || !form.code_iso2"
            >
              {{ chargement ? 'Envoi…' : 'Soumettre pour modération' }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </Teleport>
</template>
