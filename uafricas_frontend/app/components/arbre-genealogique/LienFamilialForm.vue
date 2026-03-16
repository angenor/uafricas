<script setup lang="ts">
import type { PersonneListe, CreerLienForm, TypeLien } from '~/mocks/arbre-genealogique'
import { useArbreGenealogique } from '~/composables/useArbreGenealogique'

interface Props {
  rattachementSourceId: string
  loading?: boolean
}

const props = withDefaults(defineProps<Props>(), { loading: false })
const emit = defineEmits<{
  submit: [form: CreerLienForm]
  annuler: []
}>()

// ─── État ─────────────────────────────────────────────────────────────────

const { listerPersonnes } = useArbreGenealogique()

const personnes = ref<PersonneListe['personnes']>([])
const rattachementCibleId = ref('')
const typeLien = ref<TypeLien | ''>('')
const erreurApi = ref('')
const recherchePersonne = ref('')
const chargementPersonnes = ref(false)

// ─── Charger la liste des personnes au montage ────────────────────────────

onMounted(async () => {
  chargementPersonnes.value = true
  try {
    const res = await listerPersonnes({ par_page: 50 })
    if (res.success && res.data) {
      personnes.value = res.data.personnes
    }
  } catch {
    // silencieux
  } finally {
    chargementPersonnes.value = false
  }
})

// ─── Personnes filtrées (exclure la source) ───────────────────────────────

const personnesFiltrees = computed(() => {
  const terme = recherchePersonne.value.toLowerCase()
  return personnes.value.filter((p) => {
    if (p.id === props.rattachementSourceId) return false
    if (!terme) return true
    return (
      p.nom.toLowerCase().includes(terme) ||
      p.prenoms?.toLowerCase().includes(terme)
    )
  })
})

const typesLien: { valeur: TypeLien; libelle: string }[] = [
  { valeur: 'pere', libelle: 'Père' },
  { valeur: 'mere', libelle: 'Mère' },
  { valeur: 'parent', libelle: 'Parent (non précisé)' },
  { valeur: 'conjoint', libelle: 'Conjoint(e)' },
]

// ─── Soumission ────────────────────────────────────────────────────────────

function soumettre() {
  erreurApi.value = ''
  if (!rattachementCibleId.value) {
    erreurApi.value = 'Veuillez sélectionner une personne'
    return
  }
  if (!typeLien.value) {
    erreurApi.value = 'Veuillez choisir un type de lien'
    return
  }

  emit('submit', {
    rattachement_source_id: props.rattachementSourceId,
    rattachement_cible_id: rattachementCibleId.value,
    type_lien: typeLien.value,
  })
}

// Permet au parent d'afficher une erreur API (409 doublon, 422 cycle)
defineExpose({ afficherErreur: (msg: string) => { erreurApi.value = msg } })
</script>

<template>
  <form class="space-y-4" @submit.prevent="soumettre">
    <p v-if="erreurApi" class="text-sm text-red-500 bg-red-50 border border-red-200 rounded-lg px-3 py-2">
      {{ erreurApi }}
    </p>

    <!-- Recherche + sélection de la personne cible -->
    <div>
      <label class="block text-sm font-semibold text-stone-700 mb-1">
        Relier avec <span class="text-red-500">*</span>
      </label>
      <input
        v-model="recherchePersonne"
        type="text"
        placeholder="Rechercher une personne…"
        class="w-full px-3 py-2 border border-stone-300 rounded-lg text-sm text-stone-800 placeholder-stone-400 focus:outline-none focus:ring-2 focus:ring-custom-chocolat/50 mb-2"
      />
      <div v-if="chargementPersonnes" class="text-sm text-stone-400 text-center py-4">
        Chargement des personnes…
      </div>
      <div
        v-else
        class="border border-stone-200 rounded-lg max-h-48 overflow-y-auto divide-y divide-stone-100"
      >
        <label
          v-for="p in personnesFiltrees"
          :key="p.id"
          class="flex items-center gap-3 px-3 py-2.5 cursor-pointer hover:bg-stone-50 transition-colors"
          :class="rattachementCibleId === p.id ? 'bg-custom-chocolat/5' : ''"
        >
          <input
            v-model="rattachementCibleId"
            type="radio"
            :value="p.id"
            class="accent-custom-chocolat"
          />
          <span class="text-sm text-stone-800">
            {{ p.prenoms ? `${p.prenoms} ${p.nom}` : p.nom }}
            <span v-if="p.naissance?.annee" class="text-stone-400 text-xs ml-1">({{ p.naissance.annee }})</span>
          </span>
        </label>
        <div v-if="personnesFiltrees.length === 0" class="text-sm text-stone-400 text-center py-4">
          Aucune personne trouvée
        </div>
      </div>
    </div>

    <!-- Type de lien -->
    <div>
      <label class="block text-sm font-semibold text-stone-700 mb-1">
        Type de lien <span class="text-red-500">*</span>
      </label>
      <div class="grid grid-cols-2 gap-2">
        <label
          v-for="t in typesLien"
          :key="t.valeur"
          class="flex items-center gap-2 border rounded-lg px-3 py-2 cursor-pointer transition-colors"
          :class="typeLien === t.valeur
            ? 'border-custom-chocolat bg-custom-chocolat/5 text-custom-chocolat'
            : 'border-stone-200 hover:border-stone-300 text-stone-700'"
        >
          <input v-model="typeLien" type="radio" :value="t.valeur" class="sr-only" />
          <span class="text-sm font-medium">{{ t.libelle }}</span>
        </label>
      </div>
    </div>

    <!-- Actions -->
    <div class="flex gap-3 pt-2">
      <button
        type="submit"
        :disabled="loading"
        class="flex-1 py-2.5 bg-custom-chocolat text-white font-semibold rounded-lg hover:opacity-90 transition-opacity disabled:opacity-50"
      >
        <span v-if="loading">Création…</span>
        <span v-else>Créer le lien</span>
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
