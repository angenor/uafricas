<script setup lang="ts">
import type { CreerPersonneForm, ModifierPersonneForm, Genre } from '~/mocks/arbre-genealogique'

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
const naissanceLieu = ref(props.modelValue?.naissance_lieu ?? '')

const estDecede = ref(props.modelValue?.est_decede ?? false)

// ─── Erreurs de validation ────────────────────────────────────────────────

const erreurs = ref<Record<string, string>>({})

function valider(): boolean {
  erreurs.value = {}

  if (!nom.value.trim()) {
    erreurs.value.nom = 'Le nom est obligatoire'
  }

  const na = naissanceAnnee.value ? parseInt(naissanceAnnee.value) : undefined
  if (na && (na < 1 || na > new Date().getFullYear())) {
    erreurs.value.naissanceAnnee = 'Année invalide'
  }

  return Object.keys(erreurs.value).length === 0
}

// ─── Soumission ────────────────────────────────────────────────────────────

function soumettre() {
  if (!valider()) return

  const annee = naissanceAnnee.value ? parseInt(naissanceAnnee.value) : undefined

  const form: CreerPersonneForm | ModifierPersonneForm = {
    nom: nom.value.trim() || undefined,
    prenoms: prenoms.value.trim() || undefined,
    genre: genre.value || undefined,
    naissance: annee ? { annee } : undefined,
    naissance_lieu: naissanceLieu.value.trim() || undefined,
    est_decede: estDecede.value,
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

    <!-- Année de naissance -->
    <div>
      <label class="block text-sm font-semibold text-stone-700 mb-1">Année de naissance</label>
      <input
        v-model="naissanceAnnee"
        type="number"
        placeholder="ex : 1920"
        min="1"
        max="9999"
        class="w-full px-3 py-2 border rounded-lg text-stone-800 placeholder-stone-400 focus:outline-none focus:ring-2 focus:ring-custom-chocolat/50"
        :class="erreurs.naissanceAnnee ? 'border-red-400' : 'border-stone-300'"
      />
      <p v-if="erreurs.naissanceAnnee" class="mt-1 text-xs text-red-500">{{ erreurs.naissanceAnnee }}</p>
    </div>

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

    <!-- Statut vital -->
    <div>
      <label class="block text-sm font-semibold text-stone-700 mb-2">Statut</label>
      <div class="flex gap-4">
        <label class="flex items-center gap-2 cursor-pointer">
          <input
            v-model="estDecede"
            type="radio"
            :value="false"
            class="h-4 w-4 appearance-none rounded-full border-2 border-stone-400 checked:border-custom-chocolat checked:bg-custom-chocolat checked:shadow-[inset_0_0_0_2px_white] transition-colors"
          />
          <span class="text-sm text-stone-700">En vie</span>
        </label>
        <label class="flex items-center gap-2 cursor-pointer">
          <input
            v-model="estDecede"
            type="radio"
            :value="true"
            class="h-4 w-4 appearance-none rounded-full border-2 border-stone-400 checked:border-custom-chocolat checked:bg-custom-chocolat checked:shadow-[inset_0_0_0_2px_white] transition-colors"
          />
          <span class="text-sm text-stone-700">Décédé(e)</span>
        </label>
      </div>
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
