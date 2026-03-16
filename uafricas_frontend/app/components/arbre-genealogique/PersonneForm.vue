<script setup lang="ts">
import type { CreerPersonneForm, ModifierPersonneForm, DatePartielle, Genre } from '~/mocks/arbre-genealogique'

interface Props {
  modelValue?: ModifierPersonneForm
  loading?: boolean
  modeEdition?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
  modeEdition: false,
})

const emit = defineEmits<{
  submit: [form: CreerPersonneForm | ModifierPersonneForm]
  annuler: []
}>()

// ─── État du formulaire ───────────────────────────────────────────────────

const nom = ref(props.modelValue?.nom ?? '')
const prenoms = ref(props.modelValue?.prenoms ?? '')
const genre = ref<Genre | ''>(props.modelValue?.genre ?? '')

const naissanceAnnee = ref<string>(props.modelValue?.naissance?.annee?.toString() ?? '')
const naissanceMois = ref<string>(props.modelValue?.naissance?.mois?.toString() ?? '')
const naissanceJour = ref<string>(props.modelValue?.naissance?.jour?.toString() ?? '')
const naissanceLieu = ref(props.modelValue?.naissance_lieu ?? '')

const decesAnnee = ref<string>(props.modelValue?.deces?.annee?.toString() ?? '')
const decesMois = ref<string>(props.modelValue?.deces?.mois?.toString() ?? '')
const decesJour = ref<string>(props.modelValue?.deces?.jour?.toString() ?? '')
const decesLieu = ref(props.modelValue?.deces_lieu ?? '')

// ─── Erreurs de validation ────────────────────────────────────────────────

const erreurs = ref<Record<string, string>>({})

function valider(): boolean {
  erreurs.value = {}

  if (!nom.value.trim()) {
    erreurs.value.nom = 'Le nom est obligatoire'
  }

  const na = naissanceAnnee.value ? parseInt(naissanceAnnee.value) : undefined
  const da = decesAnnee.value ? parseInt(decesAnnee.value) : undefined
  if (na && da && da < na) {
    erreurs.value.deces = 'La date de décès ne peut pas être antérieure à la date de naissance'
  }

  const nm = naissanceMois.value ? parseInt(naissanceMois.value) : undefined
  if (nm && (nm < 1 || nm > 12)) erreurs.value.naissanceMois = 'Mois invalide (1-12)'

  const nj = naissanceJour.value ? parseInt(naissanceJour.value) : undefined
  if (nj && (nj < 1 || nj > 31)) erreurs.value.naissanceJour = 'Jour invalide (1-31)'

  const dm = decesMois.value ? parseInt(decesMois.value) : undefined
  if (dm && (dm < 1 || dm > 12)) erreurs.value.decesMois = 'Mois invalide (1-12)'

  const dj = decesJour.value ? parseInt(decesJour.value) : undefined
  if (dj && (dj < 1 || dj > 31)) erreurs.value.decesJour = 'Jour invalide (1-31)'

  return Object.keys(erreurs.value).length === 0
}

// ─── Soumission ────────────────────────────────────────────────────────────

function construireDatePartielle(
  annee: string,
  mois: string,
  jour: string,
): DatePartielle | undefined {
  const a = annee ? parseInt(annee) : undefined
  const m = mois ? parseInt(mois) : undefined
  const j = jour ? parseInt(jour) : undefined
  if (!a && !m && !j) return undefined
  return { annee: a, mois: m, jour: j }
}

function soumettre() {
  if (!valider()) return

  const form: CreerPersonneForm | ModifierPersonneForm = {
    nom: nom.value.trim() || undefined,
    prenoms: prenoms.value.trim() || undefined,
    genre: genre.value || undefined,
    naissance: construireDatePartielle(naissanceAnnee.value, naissanceMois.value, naissanceJour.value),
    naissance_lieu: naissanceLieu.value.trim() || undefined,
    deces: construireDatePartielle(decesAnnee.value, decesMois.value, decesJour.value),
    deces_lieu: decesLieu.value.trim() || undefined,
  }

  emit('submit', form)
}
</script>

<template>
  <form class="space-y-6" @submit.prevent="soumettre">
    <!-- Nom -->
    <div>
      <label class="block text-sm font-semibold text-stone-700 mb-1">
        Nom <span class="text-red-500">*</span>
      </label>
      <input
        v-model="nom"
        type="text"
        placeholder="Nom de famille"
        class="w-full px-3 py-2 border rounded-lg text-stone-800 placeholder-stone-400 focus:outline-none focus:ring-2 focus:ring-custom-chocolat/50"
        :class="erreurs.nom ? 'border-red-400' : 'border-stone-300'"
      />
      <p v-if="erreurs.nom" class="mt-1 text-xs text-red-500">{{ erreurs.nom }}</p>
    </div>

    <!-- Prénoms -->
    <div>
      <label class="block text-sm font-semibold text-stone-700 mb-1">Prénoms</label>
      <input
        v-model="prenoms"
        type="text"
        placeholder="Prénoms (optionnel)"
        class="w-full px-3 py-2 border border-stone-300 rounded-lg text-stone-800 placeholder-stone-400 focus:outline-none focus:ring-2 focus:ring-custom-chocolat/50"
      />
    </div>

    <!-- Genre -->
    <div>
      <label class="block text-sm font-semibold text-stone-700 mb-1">Genre</label>
      <select
        v-model="genre"
        class="w-full px-3 py-2 border border-stone-300 rounded-lg text-stone-800 bg-white focus:outline-none focus:ring-2 focus:ring-custom-chocolat/50"
      >
        <option value="">Non précisé</option>
        <option value="masculin">Masculin</option>
        <option value="feminin">Féminin</option>
        <option value="autre">Autre</option>
        <option value="non_precise">Ne souhaite pas préciser</option>
      </select>
    </div>

    <!-- Date de naissance -->
    <fieldset class="border border-stone-200 rounded-lg p-4">
      <legend class="text-sm font-semibold text-stone-700 px-2">Date de naissance</legend>
      <div class="grid grid-cols-3 gap-3 mt-2">
        <div>
          <label class="block text-xs text-stone-500 mb-1">Année</label>
          <input
            v-model="naissanceAnnee"
            type="number"
            placeholder="ex : 1920"
            min="1"
            max="9999"
            class="w-full px-2 py-1.5 border rounded text-stone-800 placeholder-stone-400 text-sm focus:outline-none focus:ring-2 focus:ring-custom-chocolat/50"
            :class="erreurs.naissanceAnnee ? 'border-red-400' : 'border-stone-300'"
          />
        </div>
        <div>
          <label class="block text-xs text-stone-500 mb-1">Mois</label>
          <input
            v-model="naissanceMois"
            type="number"
            placeholder="1–12"
            min="1"
            max="12"
            class="w-full px-2 py-1.5 border rounded text-stone-800 placeholder-stone-400 text-sm focus:outline-none focus:ring-2 focus:ring-custom-chocolat/50"
            :class="erreurs.naissanceMois ? 'border-red-400' : 'border-stone-300'"
          />
          <p v-if="erreurs.naissanceMois" class="mt-0.5 text-xs text-red-500">{{ erreurs.naissanceMois }}</p>
        </div>
        <div>
          <label class="block text-xs text-stone-500 mb-1">Jour</label>
          <input
            v-model="naissanceJour"
            type="number"
            placeholder="1–31"
            min="1"
            max="31"
            class="w-full px-2 py-1.5 border rounded text-stone-800 placeholder-stone-400 text-sm focus:outline-none focus:ring-2 focus:ring-custom-chocolat/50"
            :class="erreurs.naissanceJour ? 'border-red-400' : 'border-stone-300'"
          />
          <p v-if="erreurs.naissanceJour" class="mt-0.5 text-xs text-red-500">{{ erreurs.naissanceJour }}</p>
        </div>
      </div>
      <p class="text-xs text-stone-400 mt-2">Vous pouvez indiquer uniquement l'année si la date complète est inconnue.</p>
    </fieldset>

    <!-- Lieu de naissance -->
    <div>
      <label class="block text-sm font-semibold text-stone-700 mb-1">Lieu de naissance</label>
      <input
        v-model="naissanceLieu"
        type="text"
        placeholder="ex : Dakar, Sénégal"
        class="w-full px-3 py-2 border border-stone-300 rounded-lg text-stone-800 placeholder-stone-400 focus:outline-none focus:ring-2 focus:ring-custom-chocolat/50"
      />
    </div>

    <!-- Date de décès -->
    <fieldset class="border border-stone-200 rounded-lg p-4">
      <legend class="text-sm font-semibold text-stone-700 px-2">Date de décès</legend>
      <p v-if="erreurs.deces" class="text-xs text-red-500 mb-2">{{ erreurs.deces }}</p>
      <div class="grid grid-cols-3 gap-3 mt-2">
        <div>
          <label class="block text-xs text-stone-500 mb-1">Année</label>
          <input
            v-model="decesAnnee"
            type="number"
            placeholder="ex : 1985"
            min="1"
            max="9999"
            class="w-full px-2 py-1.5 border rounded text-stone-800 placeholder-stone-400 text-sm focus:outline-none focus:ring-2 focus:ring-custom-chocolat/50"
            :class="erreurs.deces ? 'border-red-400' : 'border-stone-300'"
          />
        </div>
        <div>
          <label class="block text-xs text-stone-500 mb-1">Mois</label>
          <input
            v-model="decesMois"
            type="number"
            placeholder="1–12"
            min="1"
            max="12"
            class="w-full px-2 py-1.5 border rounded text-stone-800 placeholder-stone-400 text-sm focus:outline-none focus:ring-2 focus:ring-custom-chocolat/50"
            :class="erreurs.decesMois ? 'border-red-400' : 'border-stone-300'"
          />
          <p v-if="erreurs.decesMois" class="mt-0.5 text-xs text-red-500">{{ erreurs.decesMois }}</p>
        </div>
        <div>
          <label class="block text-xs text-stone-500 mb-1">Jour</label>
          <input
            v-model="decesJour"
            type="number"
            placeholder="1–31"
            min="1"
            max="31"
            class="w-full px-2 py-1.5 border rounded text-stone-800 placeholder-stone-400 text-sm focus:outline-none focus:ring-2 focus:ring-custom-chocolat/50"
            :class="erreurs.decesJour ? 'border-red-400' : 'border-stone-300'"
          />
          <p v-if="erreurs.decesJour" class="mt-0.5 text-xs text-red-500">{{ erreurs.decesJour }}</p>
        </div>
      </div>
    </fieldset>

    <!-- Lieu de décès -->
    <div>
      <label class="block text-sm font-semibold text-stone-700 mb-1">Lieu de décès</label>
      <input
        v-model="decesLieu"
        type="text"
        placeholder="ex : Bamako, Mali"
        class="w-full px-3 py-2 border border-stone-300 rounded-lg text-stone-800 placeholder-stone-400 focus:outline-none focus:ring-2 focus:ring-custom-chocolat/50"
      />
    </div>

    <!-- Actions -->
    <div class="flex gap-3 pt-2">
      <button
        type="submit"
        :disabled="loading"
        class="flex-1 py-2.5 bg-custom-chocolat text-white font-semibold rounded-lg hover:opacity-90 transition-opacity disabled:opacity-50 disabled:cursor-not-allowed"
      >
        <span v-if="loading">Enregistrement…</span>
        <span v-else>{{ modeEdition ? 'Enregistrer les modifications' : 'Ajouter la personne' }}</span>
      </button>
      <button
        type="button"
        class="px-4 py-2.5 border border-stone-300 text-stone-700 font-semibold rounded-lg hover:bg-stone-50 transition-colors"
        @click="emit('annuler')"
      >
        Annuler
      </button>
    </div>
  </form>
</template>
