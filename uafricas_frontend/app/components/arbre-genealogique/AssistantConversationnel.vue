<script setup lang="ts">
import { ref, reactive, watch, computed, nextTick, onMounted } from 'vue'
import { useAssistantArbre } from '~/composables/useAssistantArbre'
import type { CreerPersonneForm, Genre } from '~/mocks/arbre-genealogique'

const emit = defineEmits<{
  fermer: []
  'arbre-modifie': []
}>()

const {
  messages,
  promptActif,
  promptSeq,
  enCours,
  erreur,
  termine,
  phaseLabel,
  demarrer,
  reinitialiser,
  soumettrePersonne,
  passer,
  repondre,
} = useAssistantArbre({ onArbreModifie: () => emit('arbre-modifie') })

// ─── Formulaire de saisie (réinitialisé à chaque nouvelle question) ─────────

const formulaire = reactive<{
  nom: string
  prenoms: string
  genre: Genre | ''
  annee: string
  lieu: string
  est_decede: boolean
}>({
  nom: '',
  prenoms: '',
  genre: '',
  annee: '',
  lieu: '',
  est_decede: false,
})

const erreurChamp = ref('')

const reinitialiserFormulaire = () => {
  formulaire.nom = ''
  formulaire.prenoms = ''
  formulaire.genre = promptActif.value?.type === 'saisie' ? (promptActif.value.genrePreset ?? '') : ''
  formulaire.annee = ''
  formulaire.lieu = ''
  formulaire.est_decede = false
  erreurChamp.value = ''
}

// Chaque nouveau prompt (promptSeq) ⇒ on remet le formulaire à zéro.
watch(promptSeq, () => {
  reinitialiserFormulaire()
  defilerEnBas()
})

const options_genre: { valeur: Genre; label: string }[] = [
  { valeur: 'masculin', label: 'Homme' },
  { valeur: 'feminin', label: 'Femme' },
  { valeur: 'autre', label: 'Autre' },
]

const anneeMax = new Date().getFullYear()

const valider = () => {
  if (!promptActif.value || promptActif.value.type !== 'saisie') return
  erreurChamp.value = ''

  if (!formulaire.nom.trim()) {
    erreurChamp.value = 'Le nom est obligatoire.'
    return
  }
  if (formulaire.annee) {
    const a = parseInt(formulaire.annee)
    if (isNaN(a) || a < 1 || a > anneeMax) {
      erreurChamp.value = 'Année de naissance invalide.'
      return
    }
  }

  const annee = formulaire.annee ? parseInt(formulaire.annee) : undefined
  const form: CreerPersonneForm = {
    nom: formulaire.nom.trim(),
    prenoms: formulaire.prenoms.trim() || undefined,
    genre: (formulaire.genre as Genre) || undefined,
    naissance: annee ? { annee } : undefined,
    naissance_lieu: formulaire.lieu.trim() || undefined,
    est_decede: formulaire.est_decede,
  }
  soumettrePersonne(form)
}

// ─── Défilement automatique vers le bas ────────────────────────────────────

const zoneMessages = ref<HTMLElement | null>(null)
const defilerEnBas = () => {
  nextTick(() => {
    if (zoneMessages.value) {
      zoneMessages.value.scrollTop = zoneMessages.value.scrollHeight
    }
  })
}
watch(() => messages.value.length, defilerEnBas)

// ─── Confirmation de réinitialisation ───────────────────────────────────────

const confirmerReset = ref(false)
const relancer = () => {
  confirmerReset.value = false
  reinitialiser()
  demarrer()
}

const estSaisie = computed(() => promptActif.value?.type === 'saisie')
const estQuestion = computed(() => promptActif.value?.type === 'question')

onMounted(demarrer)
</script>

<template>
  <aside
    class="
      flex w-full shrink-0 flex-col border-l border-stone-200 bg-stone-50
      md:w-[24rem]
      max-md:fixed max-md:inset-0 max-md:z-40 max-md:w-full max-md:border-l-0
    "
  >
    <!-- En-tête -->
    <header class="flex shrink-0 items-center justify-between border-b border-stone-200 bg-white px-4 py-3">
      <div class="flex items-center gap-3">
        <div class="flex h-10 w-10 items-center justify-center rounded-full bg-[var(--color-custom-green)]/15">
          <font-awesome-icon :icon="['fas', 'seedling']" class="text-[var(--color-custom-green)]" />
        </div>
        <div>
          <p class="text-sm font-bold text-stone-800">Assistant arbre</p>
          <p class="text-xs text-stone-500">{{ phaseLabel }}</p>
        </div>
      </div>
      <div class="flex items-center gap-1">
        <button
          class="flex h-8 w-8 items-center justify-center rounded-full text-stone-400 transition-colors hover:bg-stone-100 hover:text-stone-600"
          title="Recommencer"
          @click="confirmerReset = true"
        >
          <font-awesome-icon :icon="['fas', 'rotate-left']" class="text-sm" />
        </button>
        <button
          class="flex h-8 w-8 items-center justify-center rounded-full text-stone-400 transition-colors hover:bg-stone-100 hover:text-stone-600"
          title="Fermer"
          @click="emit('fermer')"
        >
          <font-awesome-icon :icon="['fas', 'xmark']" />
        </button>
      </div>
    </header>

    <!-- Confirmation reset -->
    <div v-if="confirmerReset" class="border-b border-amber-200 bg-amber-50 px-4 py-3">
      <p class="mb-2 text-sm text-amber-800">Recommencer la conversation ? Les personnes déjà ajoutées resteront dans l’arbre.</p>
      <div class="flex gap-2">
        <button
          class="rounded-lg bg-amber-600 px-3 py-1.5 text-xs font-semibold text-white transition-colors hover:bg-amber-700"
          @click="relancer"
        >
          Recommencer
        </button>
        <button
          class="rounded-lg border border-stone-300 px-3 py-1.5 text-xs text-stone-600 transition-colors hover:bg-white"
          @click="confirmerReset = false"
        >
          Annuler
        </button>
      </div>
    </div>

    <!-- Fil de discussion -->
    <div ref="zoneMessages" class="flex-1 space-y-3 overflow-y-auto px-4 py-4">
      <div
        v-for="msg in messages"
        :key="msg.id"
        class="flex"
        :class="msg.role === 'user' ? 'justify-end' : 'justify-start'"
      >
        <!-- Bot -->
        <div v-if="msg.role === 'bot'" class="flex max-w-[85%] items-start gap-2">
          <div class="mt-0.5 flex h-7 w-7 shrink-0 items-center justify-center rounded-full bg-[var(--color-custom-green)]/15">
            <font-awesome-icon :icon="['fas', msg.icone || 'comment-dots']" class="text-xs text-[var(--color-custom-green)]" />
          </div>
          <div class="rounded-2xl rounded-tl-sm bg-white px-3 py-2 text-sm leading-relaxed text-stone-700 shadow-sm">
            {{ msg.texte }}
          </div>
        </div>
        <!-- Utilisateur -->
        <div
          v-else
          class="max-w-[85%] rounded-2xl rounded-tr-sm bg-[var(--color-custom-chocolat)] px-3 py-2 text-sm text-white shadow-sm"
        >
          {{ msg.texte }}
        </div>
      </div>

      <!-- Indicateur de traitement -->
      <div v-if="enCours" class="flex items-center gap-2 pl-9 text-xs text-stone-400">
        <font-awesome-icon :icon="['fas', 'spinner']" class="animate-spin" />
        Enregistrement…
      </div>
    </div>

    <!-- Zone d'interaction -->
    <footer class="shrink-0 border-t border-stone-200 bg-white px-4 py-3">
      <!-- Erreur -->
      <div v-if="erreur" class="mb-2 rounded-lg bg-red-50 px-3 py-2 text-xs text-red-700">
        {{ erreur }}
      </div>

      <!-- SAISIE : mini-formulaire personne -->
      <form v-if="estSaisie" class="space-y-2.5" @submit.prevent="valider">
        <div class="grid grid-cols-2 gap-2">
          <input
            v-model="formulaire.prenoms"
            type="text"
            placeholder="Prénom(s)"
            class="rounded-lg border border-stone-300 px-3 py-2 text-sm text-stone-800 placeholder-stone-400 outline-none focus:ring-2 focus:ring-[var(--color-custom-chocolat)]/40"
          />
          <input
            v-model="formulaire.nom"
            type="text"
            placeholder="Nom *"
            class="rounded-lg border px-3 py-2 text-sm text-stone-800 placeholder-stone-400 outline-none focus:ring-2 focus:ring-[var(--color-custom-chocolat)]/40"
            :class="erreurChamp && !formulaire.nom.trim() ? 'border-red-400' : 'border-stone-300'"
          />
        </div>

        <div class="flex gap-1.5">
          <button
            v-for="opt in options_genre"
            :key="opt.valeur"
            type="button"
            class="flex-1 rounded-lg border px-2 py-1.5 text-xs transition-all"
            :class="formulaire.genre === opt.valeur
              ? 'border-[var(--color-custom-chocolat)] bg-[var(--color-custom-chocolat)]/10 font-semibold text-[var(--color-custom-chocolat)]'
              : 'border-stone-300 text-stone-600 hover:border-[var(--color-custom-chocolat)]/60'"
            @click="formulaire.genre = opt.valeur"
          >
            {{ opt.label }}
          </button>
        </div>

        <div class="grid grid-cols-2 gap-2">
          <input
            v-model="formulaire.annee"
            type="number"
            placeholder="Année naiss."
            min="1"
            :max="anneeMax"
            class="rounded-lg border border-stone-300 px-3 py-2 text-sm text-stone-800 placeholder-stone-400 outline-none focus:ring-2 focus:ring-[var(--color-custom-chocolat)]/40"
          />
          <input
            v-model="formulaire.lieu"
            type="text"
            placeholder="Lieu naiss."
            class="rounded-lg border border-stone-300 px-3 py-2 text-sm text-stone-800 placeholder-stone-400 outline-none focus:ring-2 focus:ring-[var(--color-custom-chocolat)]/40"
          />
        </div>

        <label class="flex items-center gap-2 text-xs text-stone-500">
          <input v-model="formulaire.est_decede" type="checkbox" class="rounded border-stone-300 text-[var(--color-custom-chocolat)] focus:ring-[var(--color-custom-chocolat)]/40" />
          Personne décédée
        </label>

        <p v-if="erreurChamp" class="text-xs text-red-500">{{ erreurChamp }}</p>

        <div class="flex items-center gap-2 pt-1">
          <button
            type="submit"
            :disabled="enCours"
            class="flex flex-1 items-center justify-center gap-2 rounded-lg bg-[var(--color-custom-green)] px-4 py-2.5 text-sm font-semibold text-white transition-opacity hover:opacity-90 disabled:cursor-not-allowed disabled:opacity-50"
          >
            <font-awesome-icon v-if="enCours" :icon="['fas', 'spinner']" class="animate-spin" />
            <font-awesome-icon v-else :icon="['fas', 'paper-plane']" class="text-xs" />
            Ajouter
          </button>
          <button
            v-if="promptActif && promptActif.type === 'saisie' && promptActif.permetPasser"
            type="button"
            :disabled="enCours"
            class="rounded-lg border border-stone-300 px-3 py-2.5 text-xs text-stone-500 transition-colors hover:bg-stone-50 disabled:opacity-50"
            @click="passer"
          >
            Je ne sais pas
          </button>
        </div>
      </form>

      <!-- QUESTION : Oui / Non -->
      <div v-else-if="estQuestion && promptActif && promptActif.type === 'question'" class="flex gap-2">
        <button
          class="flex flex-1 items-center justify-center gap-2 rounded-lg bg-[var(--color-custom-green)] px-4 py-2.5 text-sm font-semibold text-white transition-opacity hover:opacity-90"
          @click="repondre(true)"
        >
          <font-awesome-icon :icon="['fas', 'check']" class="text-xs" />
          {{ promptActif.ouiLabel }}
        </button>
        <button
          class="flex flex-1 items-center justify-center gap-2 rounded-lg border border-stone-300 px-4 py-2.5 text-sm font-semibold text-stone-600 transition-colors hover:bg-stone-50"
          @click="repondre(false)"
        >
          <font-awesome-icon :icon="['fas', 'xmark']" class="text-xs" />
          {{ promptActif.nonLabel }}
        </button>
      </div>

      <!-- FIN -->
      <div v-else-if="termine" class="flex gap-2">
        <button
          class="flex flex-1 items-center justify-center gap-2 rounded-lg bg-[var(--color-custom-chocolat)] px-4 py-2.5 text-sm font-semibold text-white transition-opacity hover:opacity-90"
          @click="emit('fermer')"
        >
          <font-awesome-icon :icon="['fas', 'tree']" class="text-xs" />
          Voir mon arbre
        </button>
        <button
          class="rounded-lg border border-stone-300 px-3 py-2.5 text-xs text-stone-500 transition-colors hover:bg-stone-50"
          @click="confirmerReset = true"
        >
          Recommencer
        </button>
      </div>
    </footer>
  </aside>
</template>
