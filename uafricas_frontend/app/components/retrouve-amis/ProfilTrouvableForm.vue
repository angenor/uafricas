<script setup lang="ts">
import type { ParcoursTrouvable, PaysInfo } from '~/composables/useRetrouvAmis'

// ── Types ────────────────────────────────────────────────────
type TypeEntree = 'ecole' | 'ville_residence'

interface DonneesParcours {
  type_entree: TypeEntree
  nom: string
  ville: string
  pays_id: string
  periode_debut: number | undefined
  periode_fin: number | undefined
}

// ── Props & Emits ────────────────────────────────────────────
defineProps<{
  estTrouvable: boolean
  parcours: ParcoursTrouvable[]
  chargement: boolean
}>()

const emit = defineEmits<{
  'basculer-trouvable': []
  'ajouter-parcours': [data: DonneesParcours]
  'modifier-parcours': [id: string, data: DonneesParcours]
  'supprimer-parcours': [id: string]
}>()

// ── Chargement des pays ──────────────────────────────────────
const listePays = ref<PaysInfo[]>([])

onMounted(async () => {
  try {
    const config = useRuntimeConfig()
    const apiBase = config.public.apiBaseUrl as string
    const reponse = await $fetch<{ success: boolean; data: PaysInfo[] }>(
      `${apiBase}/api/retrouve-amis/pays`,
    )
    if (reponse.success && reponse.data) {
      listePays.value = reponse.data
    }
  }
  catch {
    // Silencieux
  }
})

// ── Etat local ───────────────────────────────────────────────
const formulaireOuvert = ref(false)
const modeEdition = ref(false)
const idEdition = ref<string | null>(null)

const formulaire = reactive<DonneesParcours>({
  type_entree: 'ecole',
  nom: '',
  ville: '',
  pays_id: '',
  periode_debut: undefined,
  periode_fin: undefined,
})

// ── Helpers ──────────────────────────────────────────────────
const typesEntree: { valeur: TypeEntree; label: string }[] = [
  { valeur: 'ecole', label: 'École / Université' },
  { valeur: 'ville_residence', label: 'Ville de residence' },
]

const iconeTypeEntree = (type: TypeEntree): string => {
  const icones: Record<TypeEntree, string> = {
    ecole: 'graduation-cap',
    ville_residence: 'building',
  }
  return icones[type] ?? 'circle'
}

const labelTypeEntree = (type: TypeEntree): string => {
  return typesEntree.find(t => t.valeur === type)?.label ?? type
}

const formaterPeriode = (debut?: number, fin?: number): string => {
  if (debut && fin) return `${debut} - ${fin}`
  if (debut) return `Depuis ${debut}`
  if (fin) return `Jusqu'en ${fin}`
  return ''
}

// ── Actions formulaire ───────────────────────────────────────
const reinitialiserFormulaire = () => {
  formulaire.type_entree = 'ecole'
  formulaire.nom = ''
  formulaire.ville = ''
  formulaire.pays_id = ''
  formulaire.periode_debut = undefined
  formulaire.periode_fin = undefined
  modeEdition.value = false
  idEdition.value = null
}

const ouvrirAjout = () => {
  reinitialiserFormulaire()
  formulaireOuvert.value = true
}

const ouvrirEdition = (p: ParcoursTrouvable) => {
  formulaire.type_entree = p.type_entree as TypeEntree
  formulaire.nom = p.nom
  formulaire.ville = p.ville ?? ''
  formulaire.pays_id = p.pays?.id ?? ''
  formulaire.periode_debut = p.periode_debut ?? undefined
  formulaire.periode_fin = p.periode_fin ?? undefined
  modeEdition.value = true
  idEdition.value = p.id
  formulaireOuvert.value = true
}

const annuler = () => {
  formulaireOuvert.value = false
  reinitialiserFormulaire()
}

const soumettre = () => {
  if (!formulaire.nom.trim()) return

  const donnees: DonneesParcours = {
    type_entree: formulaire.type_entree,
    nom: formulaire.nom.trim(),
    ville: formulaire.ville.trim(),
    pays_id: formulaire.pays_id.trim(),
    periode_debut: formulaire.periode_debut,
    periode_fin: formulaire.periode_fin,
  }

  if (modeEdition.value && idEdition.value) {
    emit('modifier-parcours', idEdition.value, donnees)
  }
  else {
    emit('ajouter-parcours', donnees)
  }

  formulaireOuvert.value = false
  reinitialiserFormulaire()
}
</script>

<template>
  <div class="space-y-6">
    <!-- Section bascule trouvable -->
    <div class="rounded-xl border border-gray-200 bg-white p-6 shadow-sm">
      <div class="flex items-center justify-between gap-4">
        <div class="flex-1">
          <h3 class="text-lg font-bold text-gray-900">Profil trouvable</h3>
          <p class="mt-1 text-sm text-gray-500">
            Lorsque votre profil est trouvable, d'autres utilisateurs peuvent vous identifier
            comme correspondance potentielle dans leurs recherches.
          </p>
          <span
            class="mt-3 inline-flex items-center gap-1.5 rounded-full px-3 py-1 text-xs font-semibold"
            :class="estTrouvable
              ? 'bg-custom-green/10 text-custom-green'
              : 'bg-gray-100 text-gray-500'"
          >
            <span
              class="h-2 w-2 rounded-full"
              :class="estTrouvable ? 'bg-custom-green' : 'bg-gray-400'"
            />
            {{ estTrouvable ? 'Votre profil est visible' : 'Votre profil est masque' }}
          </span>
        </div>
        <button
          type="button"
          class="relative inline-flex h-8 w-14 shrink-0 cursor-pointer items-center rounded-full transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-custom-green focus:ring-offset-2"
          :class="estTrouvable ? 'bg-custom-green' : 'bg-gray-300'"
          :disabled="chargement"
          @click="emit('basculer-trouvable')"
        >
          <span
            class="inline-block h-6 w-6 transform rounded-full bg-white shadow-md transition-transform duration-200"
            :class="estTrouvable ? 'translate-x-7' : 'translate-x-1'"
          />
        </button>
      </div>
    </div>

    <!-- Liste des parcours -->
    <div class="rounded-xl border border-gray-200 bg-white p-6 shadow-sm">
      <div class="mb-4 flex items-center justify-between">
        <h3 class="text-lg font-bold text-gray-900">Mon parcours</h3>
        <button
          v-if="!formulaireOuvert"
          type="button"
          class="inline-flex items-center gap-1.5 rounded-lg bg-custom-chocolat px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-custom-chocolat/90"
          :disabled="chargement"
          @click="ouvrirAjout"
        >
          <font-awesome-icon icon="plus" class="h-3 w-3" />
          Ajouter
        </button>
      </div>

      <!-- Liste vide -->
      <div
        v-if="parcours.length === 0 && !formulaireOuvert"
        class="rounded-lg border border-dashed border-gray-300 py-10 text-center"
      >
        <font-awesome-icon icon="route" class="mx-auto mb-3 h-8 w-8 text-gray-300" />
        <p class="text-sm text-gray-500">Aucun parcours renseigne.</p>
        <p class="mt-1 text-xs text-gray-400">
          Ajoutez vos ecoles et villes de residence pour etre retrouve plus facilement.
        </p>
      </div>

      <!-- Cartes parcours -->
      <div v-if="parcours.length > 0" class="space-y-3">
        <div
          v-for="item in parcours"
          :key="item.id"
          class="flex items-start justify-between gap-3 rounded-lg border border-gray-100 bg-gray-50 p-4 transition-colors hover:bg-gray-100"
        >
          <div class="flex items-start gap-3">
            <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-custom-chocolat/10 text-custom-chocolat">
              <font-awesome-icon :icon="iconeTypeEntree(item.type_entree as TypeEntree)" class="h-4 w-4" />
            </div>
            <div>
              <p class="text-sm font-semibold text-gray-900">{{ item.nom }}</p>
              <p class="text-xs text-gray-500">{{ labelTypeEntree(item.type_entree as TypeEntree) }}</p>
              <div class="mt-1 flex flex-wrap gap-2">
                <span v-if="item.ville" class="inline-flex items-center gap-1 text-xs text-gray-500">
                  <font-awesome-icon icon="location-dot" class="h-3 w-3" />
                  {{ item.ville }}
                </span>
                <span v-if="item.pays" class="inline-flex items-center gap-1 text-xs text-gray-500">
                  <font-awesome-icon icon="earth-africa" class="h-3 w-3" />
                  {{ item.pays.nom }}
                </span>
                <span
                  v-if="item.periode_debut || item.periode_fin"
                  class="inline-flex items-center gap-1 text-xs text-gray-500"
                >
                  <font-awesome-icon icon="calendar" class="h-3 w-3" />
                  {{ formaterPeriode(item.periode_debut, item.periode_fin) }}
                </span>
              </div>
            </div>
          </div>
          <div class="flex shrink-0 items-center gap-1">
            <button
              type="button"
              class="rounded-lg p-2 text-gray-400 transition-colors hover:bg-custom-chocolat/10 hover:text-custom-chocolat"
              title="Modifier"
              :disabled="chargement"
              @click="ouvrirEdition(item)"
            >
              <font-awesome-icon icon="pen-to-square" class="h-3.5 w-3.5" />
            </button>
            <button
              type="button"
              class="rounded-lg p-2 text-gray-400 transition-colors hover:bg-red-50 hover:text-red-500"
              title="Supprimer"
              :disabled="chargement"
              @click="emit('supprimer-parcours', item.id)"
            >
              <font-awesome-icon icon="trash" class="h-3.5 w-3.5" />
            </button>
          </div>
        </div>
      </div>

      <!-- Formulaire ajout / edition -->
      <div
        v-if="formulaireOuvert"
        class="mt-4 rounded-lg border border-gray-200 bg-white p-5"
      >
        <h4 class="mb-4 text-sm font-semibold text-gray-800">
          {{ modeEdition ? 'Modifier le parcours' : 'Ajouter un parcours' }}
        </h4>

        <div class="space-y-4">
          <div>
            <label class="mb-1 block text-sm font-medium text-gray-700">Type</label>
            <select
              v-model="formulaire.type_entree"
              class="w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm outline-none transition-shadow focus:ring-2 focus:ring-custom-chocolat"
            >
              <option v-for="type in typesEntree" :key="type.valeur" :value="type.valeur">
                {{ type.label }}
              </option>
            </select>
          </div>

          <div>
            <label class="mb-1 block text-sm font-medium text-gray-700">
              Nom <span class="text-red-500">*</span>
            </label>
            <input
              v-model="formulaire.nom"
              type="text"
              maxlength="150"
              :placeholder="formulaire.type_entree === 'ecole' ? 'Ex : Universite Cheikh Anta Diop' : 'Ex : Dakar'"
              class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm outline-none transition-shadow focus:ring-2 focus:ring-custom-chocolat"
            >
          </div>

          <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
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
              <label class="mb-1 block text-sm font-medium text-gray-700">Territoire</label>
              <select
                v-model="formulaire.pays_id"
                class="w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm outline-none transition-shadow focus:ring-2 focus:ring-custom-chocolat"
              >
                <option value="">-- Sélectionner un territoire --</option>
                <option v-for="pays in listePays" :key="pays.id" :value="pays.id">
                  {{ pays.nom }}
                </option>
              </select>
            </div>
          </div>

          <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
            <div>
              <label class="mb-1 block text-sm font-medium text-gray-700">Annee de debut</label>
              <input
                v-model.number="formulaire.periode_debut"
                type="number"
                min="1900"
                :max="new Date().getFullYear()"
                placeholder="Ex : 2005"
                class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm outline-none transition-shadow focus:ring-2 focus:ring-custom-chocolat"
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
                class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm outline-none transition-shadow focus:ring-2 focus:ring-custom-chocolat"
              >
            </div>
          </div>
        </div>

        <div class="mt-5 flex items-center justify-end gap-3">
          <button
            type="button"
            class="rounded-lg border border-gray-300 px-4 py-2 text-sm font-medium text-gray-600 transition-colors hover:bg-gray-100"
            @click="annuler"
          >
            Annuler
          </button>
          <button
            type="button"
            class="rounded-lg bg-custom-green px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-custom-green/90 disabled:opacity-50"
            :disabled="!formulaire.nom.trim() || chargement"
            @click="soumettre"
          >
            {{ modeEdition ? 'Enregistrer' : 'Ajouter' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
