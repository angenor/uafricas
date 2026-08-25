<script setup lang="ts">
import type { MotifSignalement } from '~/composables/useRetrouvAmis'

const props = defineProps<{
  slug: string
}>()

const userStore = useUserStore()
const router = useRouter()
const { signalerAvisPublic, demanderRetrait, chargement, erreur } = useRetrouvAmis()

// ── Etat local ────────────────────────────────────────────
const formulaireRetraitOuvert = ref(false)
const formulaireSignalementOuvert = ref(false)
const motifRetrait = ref('')
const motifSignalement = ref<MotifSignalement>('contenu_abusif')
const descriptionSignalement = ref('')
const messageSucces = ref('')
const retraitEnvoye = ref(false)
const signalementEnvoye = ref(false)

const emit = defineEmits<{
  (e: 'suspendu'): void
}>()

const motifsSignalement: { value: MotifSignalement; label: string }[] = [
  { value: 'contenu_abusif', label: 'Contenu abusif ou inapproprie' },
  { value: 'usurpation_identite', label: 'Usurpation d\'identite' },
  { value: 'harcelement', label: 'Harcelement' },
  { value: 'autre', label: 'Autre motif' },
]

// ── Verifier connexion avant action ───────────────────────
const verifierConnexion = (): boolean => {
  if (!userStore.isAuthenticated) {
    router.push(`/login?redirect=${encodeURIComponent(`/retrouve-amis/public/${props.slug}`)}`)
    return false
  }
  return true
}

// ── Ouvrir formulaire de retrait ──────────────────────────
const ouvrirRetrait = () => {
  if (!verifierConnexion()) return
  formulaireRetraitOuvert.value = true
  formulaireSignalementOuvert.value = false
}

// ── Ouvrir formulaire de signalement ──────────────────────
const ouvrirSignalement = () => {
  if (!verifierConnexion()) return
  formulaireSignalementOuvert.value = true
  formulaireRetraitOuvert.value = false
}

// ── Soumettre demande de retrait ──────────────────────────
const soumettreRetrait = async () => {
  if (!motifRetrait.value.trim()) return

  const resultat = await demanderRetrait(props.slug, motifRetrait.value.trim())
  if (resultat) {
    messageSucces.value = resultat.message
    retraitEnvoye.value = true
    formulaireRetraitOuvert.value = false
    emit('suspendu')
  }
}

// ── Soumettre signalement ─────────────────────────────────
const soumettreSignalement = async () => {
  const resultat = await signalerAvisPublic(props.slug, {
    motif: motifSignalement.value,
    description: descriptionSignalement.value.trim() || undefined,
  })
  if (resultat) {
    messageSucces.value = 'Votre signalement a bien ete enregistre. Merci.'
    signalementEnvoye.value = true
    formulaireSignalementOuvert.value = false
  }
}

const fermerTout = () => {
  formulaireRetraitOuvert.value = false
  formulaireSignalementOuvert.value = false
}
</script>

<template>
  <div class="mt-8 space-y-4">
    <!-- Message de succes -->
    <div
      v-if="messageSucces"
      class="bg-green-50 border border-green-200 rounded-lg p-4 flex items-start gap-3"
    >
      <font-awesome-icon :icon="['fas', 'circle-check']" class="text-custom-green text-lg mt-0.5 shrink-0" />
      <p class="text-green-800 text-sm">{{ messageSucces }}</p>
    </div>

    <!-- Erreur -->
    <div
      v-if="erreur"
      class="bg-red-50 border border-red-200 rounded-lg p-4 flex items-start gap-3"
    >
      <font-awesome-icon :icon="['fas', 'circle-exclamation']" class="text-red-500 text-lg mt-0.5 shrink-0" />
      <p class="text-red-700 text-sm">{{ erreur }}</p>
    </div>

    <!-- Boutons d'action (masques apres envoi) -->
    <div v-if="!retraitEnvoye && !signalementEnvoye" class="flex flex-col sm:flex-row gap-3">
      <!-- Bouton demande de retrait -->
      <button
        type="button"
        class="flex items-center justify-center gap-2 px-5 py-3 bg-amber-700 text-white font-medium rounded-lg hover:bg-amber-800 transition-colors text-sm"
        @click="ouvrirRetrait"
      >
        <font-awesome-icon :icon="['fas', 'user-shield']" />
        Cet avis me concerne : demander le retrait
      </button>

      <!-- Bouton signalement -->
      <button
        type="button"
        class="flex items-center justify-center gap-2 px-5 py-3 border border-gray-300 text-gray-700 font-medium rounded-lg hover:bg-gray-50 transition-colors text-sm"
        @click="ouvrirSignalement"
      >
        <font-awesome-icon :icon="['fas', 'flag']" />
        Signaler cet avis
      </button>
    </div>

    <!-- Formulaire demande de retrait -->
    <div
      v-if="formulaireRetraitOuvert"
      class="bg-white border border-gray-200 rounded-xl shadow-sm p-6 space-y-4"
    >
      <div class="flex items-center justify-between">
        <h3 class="text-lg font-semibold text-gray-800">
          Demander le retrait de cet avis
        </h3>
        <button type="button" class="text-gray-400 hover:text-gray-600" @click="fermerTout">
          <font-awesome-icon :icon="['fas', 'xmark']" class="text-xl" />
        </button>
      </div>

      <div class="bg-amber-50 border border-amber-200 rounded-lg p-3">
        <p class="text-amber-800 text-sm">
          <font-awesome-icon :icon="['fas', 'circle-info']" class="mr-1" />
          Si vous etes la personne recherchee dans cet avis et ne souhaitez pas etre retrouvee, vous pouvez demander son retrait. L'avis sera immediatement suspendu le temps qu'un administrateur examine votre demande.
        </p>
      </div>

      <div>
        <label for="motif-retrait" class="block text-sm font-medium text-gray-700 mb-1">
          Expliquez votre demande <span class="text-red-500">*</span>
        </label>
        <textarea
          id="motif-retrait"
          v-model="motifRetrait"
          rows="4"
          class="w-full rounded-lg border border-gray-300 px-4 py-3 text-sm text-gray-800 placeholder:text-gray-400 focus:border-amber-500 focus:ring-2 focus:ring-amber-500/20 focus:outline-none transition-colors resize-none"
          placeholder="Je suis la personne recherchee dans cet avis et je ne souhaite pas etre retrouvee..."
        />
      </div>

      <div class="flex justify-end gap-3">
        <button
          type="button"
          class="px-4 py-2 text-sm text-gray-600 hover:text-gray-800 transition-colors"
          @click="fermerTout"
        >
          Annuler
        </button>
        <button
          type="button"
          class="px-5 py-2 bg-amber-700 text-white text-sm font-medium rounded-lg hover:bg-amber-800 transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
          :disabled="!motifRetrait.trim() || chargement"
          @click="soumettreRetrait"
        >
          <font-awesome-icon v-if="chargement" :icon="['fas', 'spinner']" class="animate-spin" />
          Envoyer la demande
        </button>
      </div>
    </div>

    <!-- Formulaire signalement -->
    <div
      v-if="formulaireSignalementOuvert"
      class="bg-white border border-gray-200 rounded-xl shadow-sm p-6 space-y-4"
    >
      <div class="flex items-center justify-between">
        <h3 class="text-lg font-semibold text-gray-800">
          Signaler cet avis
        </h3>
        <button type="button" class="text-gray-400 hover:text-gray-600" @click="fermerTout">
          <font-awesome-icon :icon="['fas', 'xmark']" class="text-xl" />
        </button>
      </div>

      <div>
        <label for="motif-signalement" class="block text-sm font-medium text-gray-700 mb-1">
          Motif du signalement <span class="text-red-500">*</span>
        </label>
        <select
          id="motif-signalement"
          v-model="motifSignalement"
          class="w-full rounded-lg border border-gray-300 px-4 py-3 text-sm text-gray-800 focus:border-amber-500 focus:ring-2 focus:ring-amber-500/20 focus:outline-none transition-colors bg-white"
        >
          <option
            v-for="m in motifsSignalement"
            :key="m.value"
            :value="m.value"
          >
            {{ m.label }}
          </option>
        </select>
      </div>

      <div>
        <label for="description-signalement" class="block text-sm font-medium text-gray-700 mb-1">
          Description (facultatif)
        </label>
        <textarea
          id="description-signalement"
          v-model="descriptionSignalement"
          rows="3"
          class="w-full rounded-lg border border-gray-300 px-4 py-3 text-sm text-gray-800 placeholder:text-gray-400 focus:border-amber-500 focus:ring-2 focus:ring-amber-500/20 focus:outline-none transition-colors resize-none"
          placeholder="Decrivez plus en detail le probleme..."
        />
      </div>

      <div class="flex justify-end gap-3">
        <button
          type="button"
          class="px-4 py-2 text-sm text-gray-600 hover:text-gray-800 transition-colors"
          @click="fermerTout"
        >
          Annuler
        </button>
        <button
          type="button"
          class="px-5 py-2 bg-red-600 text-white text-sm font-medium rounded-lg hover:bg-red-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
          :disabled="chargement"
          @click="soumettreSignalement"
        >
          <font-awesome-icon v-if="chargement" :icon="['fas', 'spinner']" class="animate-spin" />
          Envoyer le signalement
        </button>
      </div>
    </div>
  </div>
</template>
