<script setup lang="ts">
import type { AvisRecherche, CreerAvisRequest, PaysInfo } from '~/composables/useRetrouvAmis'

// ── Props & Emits ──────────────────────────────────────────────
interface Props {
  mode?: 'creation' | 'modification'
  avisExistant?: AvisRecherche
}

const props = withDefaults(defineProps<Props>(), {
  mode: 'creation',
})

const emit = defineEmits<{
  submit: [data: CreerAvisRequest]
  annuler: []
}>()

// ── Etapes du formulaire ───────────────────────────────────────
const etapes = [
  { numero: 1, titre: 'Identite' },
  { numero: 2, titre: 'Education' },
  { numero: 3, titre: 'Localisation' },
  { numero: 4, titre: 'Periode' },
  { numero: 5, titre: 'Recapitulatif' },
]
const etapeCourante = ref(1)

// ── Donnees du formulaire ──────────────────────────────────────
const formulaire = reactive<CreerAvisRequest>({
  nom_recherche: props.avisExistant?.nom_recherche ?? '',
  prenom_recherche: props.avisExistant?.prenom_recherche ?? '',
  surnom: props.avisExistant?.surnom ?? '',
  ecole: props.avisExistant?.ecole ?? '',
  ville: props.avisExistant?.ville ?? '',
  pays_id: props.avisExistant?.pays?.id ?? '',
  periode_debut: props.avisExistant?.periode_debut ?? undefined,
  periode_fin: props.avisExistant?.periode_fin ?? undefined,
  description: props.avisExistant?.description ?? '',
})

// ── Chargement des pays ────────────────────────────────────────
const listePays = ref<PaysInfo[]>([])
const chargementPays = ref(false)

onMounted(async () => {
  chargementPays.value = true
  try {
    const config = useRuntimeConfig()
    const apiBase = config.public.apiBaseUrl as string
    const reponse = await $fetch<{ success: boolean; data: { pays: PaysInfo[] } }>(
      `${apiBase}/api/admin/pays`,
    )
    if (reponse.success && reponse.data?.pays) {
      listePays.value = reponse.data.pays
    }
  }
  catch {
    // Silencieux — le select restera vide
  }
  finally {
    chargementPays.value = false
  }
})

// ── Erreurs de validation ──────────────────────────────────────
const erreurs = reactive<Record<string, string>>({})

const nettoyerErreurs = () => {
  Object.keys(erreurs).forEach(k => delete erreurs[k])
}

// ── Validation par etape ───────────────────────────────────────
const validerEtape = (etape: number): boolean => {
  nettoyerErreurs()

  if (etape === 1) {
    if (!formulaire.nom_recherche.trim()) {
      erreurs.nom_recherche = 'Le nom de la personne recherchee est requis.'
    }
    if (!formulaire.prenom_recherche?.trim() && !formulaire.surnom?.trim()) {
      erreurs.critere_supplementaire = 'Veuillez renseigner au moins le prenom ou le surnom.'
    }
  }

  if (etape === 4) {
    if (formulaire.periode_debut && formulaire.periode_fin && formulaire.periode_debut > formulaire.periode_fin) {
      erreurs.periode = 'L\'annee de debut doit etre anterieure ou egale a l\'annee de fin.'
    }
  }

  return Object.keys(erreurs).length === 0
}

// ── Validation globale (au moins 1 critere au-dela du nom) ─────
const validerGlobal = (): boolean => {
  const aCritereSupplementaire =
    !!formulaire.prenom_recherche?.trim()
    || !!formulaire.surnom?.trim()
    || !!formulaire.ecole?.trim()
    || !!formulaire.ville?.trim()
    || !!formulaire.pays_id?.trim()
    || !!formulaire.periode_debut
    || !!formulaire.periode_fin

  if (!aCritereSupplementaire) {
    erreurs.global = 'Veuillez renseigner au moins un critere en plus du nom (prenom, ecole, ville, pays ou periode).'
    return false
  }
  return true
}

// ── Navigation ─────────────────────────────────────────────────
const suivant = () => {
  if (validerEtape(etapeCourante.value)) {
    etapeCourante.value++
  }
}

const precedent = () => {
  nettoyerErreurs()
  etapeCourante.value--
}

// ── Soumission ─────────────────────────────────────────────────
const soumettre = () => {
  nettoyerErreurs()
  if (!validerEtape(1)) { etapeCourante.value = 1; return }
  if (!validerEtape(4)) { etapeCourante.value = 4; return }
  if (!validerGlobal()) return

  // Construire le payload propre (sans champs vides)
  const data: CreerAvisRequest = {
    nom_recherche: formulaire.nom_recherche.trim(),
  }
  if (formulaire.prenom_recherche?.trim()) data.prenom_recherche = formulaire.prenom_recherche.trim()
  if (formulaire.surnom?.trim()) data.surnom = formulaire.surnom.trim()
  if (formulaire.ecole?.trim()) data.ecole = formulaire.ecole.trim()
  if (formulaire.ville?.trim()) data.ville = formulaire.ville.trim()
  if (formulaire.pays_id?.trim()) data.pays_id = formulaire.pays_id.trim()
  if (formulaire.periode_debut) data.periode_debut = formulaire.periode_debut
  if (formulaire.periode_fin) data.periode_fin = formulaire.periode_fin
  if (formulaire.description?.trim()) data.description = formulaire.description.trim()

  emit('submit', data)
}

// ── Helpers pour le recapitulatif ──────────────────────────────
const nomPaysSelectionne = computed(() => {
  if (!formulaire.pays_id) return ''
  return listePays.value.find(p => p.id === formulaire.pays_id)?.nom ?? formulaire.pays_id
})

const lignesRecap = computed(() => {
  const lignes: { label: string; valeur: string }[] = []
  lignes.push({ label: 'Nom recherche', valeur: formulaire.nom_recherche })
  if (formulaire.prenom_recherche?.trim()) lignes.push({ label: 'Prenom', valeur: formulaire.prenom_recherche })
  if (formulaire.surnom?.trim()) lignes.push({ label: 'Surnom', valeur: formulaire.surnom })
  if (formulaire.ecole?.trim()) lignes.push({ label: 'Ecole / Universite', valeur: formulaire.ecole })
  if (formulaire.ville?.trim()) lignes.push({ label: 'Ville', valeur: formulaire.ville })
  if (nomPaysSelectionne.value) lignes.push({ label: 'Pays', valeur: nomPaysSelectionne.value })
  if (formulaire.periode_debut) lignes.push({ label: 'Periode debut', valeur: String(formulaire.periode_debut) })
  if (formulaire.periode_fin) lignes.push({ label: 'Periode fin', valeur: String(formulaire.periode_fin) })
  if (formulaire.description?.trim()) lignes.push({ label: 'Description', valeur: formulaire.description })
  return lignes
})
</script>

<template>
  <div class="mx-auto w-full max-w-2xl rounded-xl border border-gray-200 bg-white p-6 shadow-sm md:p-8">
    <!-- Indicateur d'etapes -->
    <div class="mb-8 flex items-center justify-center gap-2">
      <template v-for="etape in etapes" :key="etape.numero">
        <div
          class="flex h-9 w-9 shrink-0 items-center justify-center rounded-full text-sm font-semibold transition-colors"
          :class="{
            'bg-custom-chocolat text-white': etape.numero === etapeCourante,
            'bg-custom-green text-white': etape.numero < etapeCourante,
            'bg-gray-200 text-gray-500': etape.numero > etapeCourante,
          }"
        >
          {{ etape.numero }}
        </div>
        <div
          v-if="etape.numero < etapes.length"
          class="h-0.5 w-6 md:w-10"
          :class="etape.numero < etapeCourante ? 'bg-custom-green' : 'bg-gray-200'"
        />
      </template>
    </div>

    <h2 class="mb-1 text-center font-display text-xl font-bold text-gray-800">
      {{ etapes[etapeCourante - 1].titre }}
    </h2>
    <p class="mb-6 text-center text-sm text-gray-500">
      Etape {{ etapeCourante }} sur {{ etapes.length }}
    </p>

    <!-- Erreur globale -->
    <p v-if="erreurs.global" class="mb-4 rounded-lg bg-red-50 px-4 py-2 text-sm text-red-600">
      {{ erreurs.global }}
    </p>

    <!-- ── Etape 1 : Identite ─────────────────────────── -->
    <div v-if="etapeCourante === 1" class="space-y-4">
      <div>
        <label class="mb-1 block text-sm font-medium text-gray-700">
          Nom de la personne recherchee <span class="text-red-500">*</span>
        </label>
        <input
          v-model="formulaire.nom_recherche"
          type="text"
          maxlength="100"
          placeholder="Ex : Diallo"
          class="w-full rounded-lg border px-3 py-2 text-sm outline-none transition-shadow focus:ring-2 focus:ring-custom-chocolat"
          :class="erreurs.nom_recherche ? 'border-red-400' : 'border-gray-300'"
        >
        <p v-if="erreurs.nom_recherche" class="mt-1 text-xs text-red-500">{{ erreurs.nom_recherche }}</p>
      </div>

      <div>
        <label class="mb-1 block text-sm font-medium text-gray-700">Prenom</label>
        <input
          v-model="formulaire.prenom_recherche"
          type="text"
          placeholder="Ex : Amadou"
          class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm outline-none transition-shadow focus:ring-2 focus:ring-custom-chocolat"
        >
      </div>

      <div>
        <label class="mb-1 block text-sm font-medium text-gray-700">Surnom</label>
        <input
          v-model="formulaire.surnom"
          type="text"
          placeholder="Ex : Le Grand"
          class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm outline-none transition-shadow focus:ring-2 focus:ring-custom-chocolat"
        >
      </div>

      <p v-if="erreurs.critere_supplementaire" class="rounded-lg bg-red-50 px-4 py-2 text-xs text-red-500">
        {{ erreurs.critere_supplementaire }}
      </p>
    </div>

    <!-- ── Etape 2 : Education ────────────────────────── -->
    <div v-if="etapeCourante === 2" class="space-y-4">
      <div>
        <label class="mb-1 block text-sm font-medium text-gray-700">Ecole / Universite</label>
        <input
          v-model="formulaire.ecole"
          type="text"
          placeholder="Ex : Universite Cheikh Anta Diop"
          class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm outline-none transition-shadow focus:ring-2 focus:ring-custom-chocolat"
        >
      </div>
    </div>

    <!-- ── Etape 3 : Localisation ─────────────────────── -->
    <div v-if="etapeCourante === 3" class="space-y-4">
      <div>
        <label class="mb-1 block text-sm font-medium text-gray-700">Ville</label>
        <input
          v-model="formulaire.ville"
          type="text"
          placeholder="Ex : Dakar"
          class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm outline-none transition-shadow focus:ring-2 focus:ring-custom-chocolat"
        >
      </div>

      <div>
        <label class="mb-1 block text-sm font-medium text-gray-700">Pays</label>
        <select
          v-model="formulaire.pays_id"
          class="w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm outline-none transition-shadow focus:ring-2 focus:ring-custom-chocolat"
        >
          <option value="">
            {{ chargementPays ? 'Chargement...' : '-- Selectionner un pays --' }}
          </option>
          <option v-for="pays in listePays" :key="pays.id" :value="pays.id">
            {{ pays.nom }}
          </option>
        </select>
      </div>
    </div>

    <!-- ── Etape 4 : Periode ──────────────────────────── -->
    <div v-if="etapeCourante === 4" class="space-y-4">
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
        <div>
          <label class="mb-1 block text-sm font-medium text-gray-700">Annee de debut</label>
          <input
            v-model.number="formulaire.periode_debut"
            type="number"
            min="1900"
            :max="new Date().getFullYear()"
            placeholder="Ex : 2005"
            class="w-full rounded-lg border px-3 py-2 text-sm outline-none transition-shadow focus:ring-2 focus:ring-custom-chocolat"
            :class="erreurs.periode ? 'border-red-400' : 'border-gray-300'"
          >
        </div>
        <div>
          <label class="mb-1 block text-sm font-medium text-gray-700">Annee de fin</label>
          <input
            v-model.number="formulaire.periode_fin"
            type="number"
            min="1900"
            :max="new Date().getFullYear()"
            placeholder="Ex : 2010"
            class="w-full rounded-lg border px-3 py-2 text-sm outline-none transition-shadow focus:ring-2 focus:ring-custom-chocolat"
            :class="erreurs.periode ? 'border-red-400' : 'border-gray-300'"
          >
        </div>
      </div>
      <p v-if="erreurs.periode" class="rounded-lg bg-red-50 px-4 py-2 text-xs text-red-500">
        {{ erreurs.periode }}
      </p>
    </div>

    <!-- ── Etape 5 : Description & Recapitulatif ──────── -->
    <div v-if="etapeCourante === 5" class="space-y-5">
      <div>
        <label class="mb-1 block text-sm font-medium text-gray-700">Description complementaire</label>
        <textarea
          v-model="formulaire.description"
          rows="3"
          placeholder="Tout detail supplementaire qui pourrait aider a retrouver cette personne..."
          class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm outline-none transition-shadow focus:ring-2 focus:ring-custom-chocolat"
        />
      </div>

      <div class="rounded-lg border border-gray-200 bg-gray-50 p-4">
        <h3 class="mb-3 text-sm font-semibold text-gray-700">Recapitulatif</h3>
        <dl class="space-y-2">
          <div v-for="ligne in lignesRecap" :key="ligne.label" class="flex gap-2 text-sm">
            <dt class="shrink-0 font-medium text-gray-500">{{ ligne.label }} :</dt>
            <dd class="text-gray-800">{{ ligne.valeur }}</dd>
          </div>
        </dl>
        <p v-if="lignesRecap.length <= 1" class="mt-2 text-xs text-amber-600">
          Attention : aucun critere supplementaire renseigne. La recherche sera peu precise.
        </p>
      </div>
    </div>

    <!-- ── Boutons de navigation ──────────────────────── -->
    <div class="mt-8 flex items-center justify-between">
      <button
        v-if="etapeCourante > 1"
        type="button"
        class="rounded-lg border border-gray-300 px-5 py-2 text-sm font-medium text-gray-600 transition-colors hover:bg-gray-100"
        @click="precedent"
      >
        Precedent
      </button>
      <button
        v-else
        type="button"
        class="rounded-lg border border-gray-300 px-5 py-2 text-sm font-medium text-gray-600 transition-colors hover:bg-gray-100"
        @click="emit('annuler')"
      >
        Annuler
      </button>

      <button
        v-if="etapeCourante < etapes.length"
        type="button"
        class="rounded-lg bg-custom-chocolat px-5 py-2 text-sm font-medium text-white transition-colors hover:bg-custom-chocolat/90"
        @click="suivant"
      >
        Suivant
      </button>
      <button
        v-else
        type="button"
        class="rounded-lg bg-custom-green px-5 py-2 text-sm font-medium text-white transition-colors hover:bg-custom-green/90"
        @click="soumettre"
      >
        {{ mode === 'modification' ? 'Enregistrer les modifications' : 'Creer l\'avis' }}
      </button>
    </div>
  </div>
</template>
