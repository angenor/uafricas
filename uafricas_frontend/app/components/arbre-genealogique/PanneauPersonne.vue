<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { formaterDate, formeVide, getLibelleLien } from '~/mocks/arbre-genealogique'
import type { ModePanneau, ContexteAjout, CreerPersonneForm, TypeLien } from '~/mocks/arbre-genealogique'
import type { NoeudArbre } from '~/composables/useLayoutArbre'
import PersonneForm from '~/components/arbre-genealogique/PersonneForm.vue'

const props = defineProps<{
  personne: NoeudArbre
  mode: ModePanneau
  contexteAjout?: ContexteAjout | null
  mutationEnCours?: boolean
  erreurMutation?: string | null
}>()

const emit = defineEmits<{
  fermer: []
  'action-ajouter': [typeAction: 'parent' | 'enfant' | 'conjoint']
  'action-modifier': []
  'action-supprimer': []
  'personne-ajoutee': [form: CreerPersonneForm, typeLien: TypeLien]
  'personne-modifiee': [form: CreerPersonneForm]
  'retour-fiche': []
}>()

// ─── Computed ────────────────────────────────────────────────────────

const nomComplet = computed(() => {
  return props.personne.prenoms
    ? `${props.personne.prenoms} ${props.personne.nom}`
    : props.personne.nom
})

const initiales = computed(() => {
  const premierPrenom = props.personne.prenoms?.split(' ')[0] ?? ''
  return `${premierPrenom.charAt(0)}${props.personne.nom.charAt(0)}`.toUpperCase()
})

const genreLabel = computed(() => {
  const labels: Record<string, string> = {
    masculin: 'Homme',
    feminin: 'Femme',
    autre: 'Autre',
    non_precise: 'Non précisé',
  }
  return labels[props.personne.genre] || props.personne.genre
})

const fondInitiales = computed(() => {
  if (props.personne.genre === 'masculin') return 'bg-sky-100 text-sky-700'
  if (props.personne.genre === 'feminin') return 'bg-pink-100 text-pink-700'
  return 'bg-stone-100 text-stone-600'
})

const nbLiensTotal = computed(() => {
  return props.personne.parents.length + props.personne.enfants.length + props.personne.conjoints.length
})

const titreMode = computed(() => {
  if (props.mode === 'ajout' && props.contexteAjout) {
    const labels: Record<string, string> = {
      parent: 'Ajouter un parent de',
      enfant: 'Ajouter un enfant de',
      conjoint: 'Ajouter un(e) conjoint(e) de',
    }
    return `${labels[props.contexteAjout.typeAction]} ${props.contexteAjout.personneSourceNom}`
  }
  if (props.mode === 'modifier') return `Modifier ${nomComplet.value}`
  return 'Fiche personne'
})

// ─── Formulaire ──────────────────────────────────────────────────────

const typeLienChoisi = ref<TypeLien>(props.contexteAjout?.typeLienSuggere ?? 'parent')

// Synchroniser quand le contexte change
watch(() => props.contexteAjout?.typeLienSuggere, (val) => {
  if (val) typeLienChoisi.value = val
})

// Données pré-remplies pour modification
const formModification = computed<CreerPersonneForm>(() => ({
  nom: props.personne.nom,
  prenoms: props.personne.prenoms,
  genre: props.personne.genre as any,
  naissance: props.personne.naissance,
  naissance_lieu: props.personne.naissance_lieu,
  deces: props.personne.deces,
  deces_lieu: props.personne.deces_lieu,
}))

// ─── Confirmation suppression ────────────────────────────────────────

const confirmationSuppression = ref(false)

const surSoumettreAjout = (form: CreerPersonneForm) => {
  emit('personne-ajoutee', form, typeLienChoisi.value)
}

const surSoumettreModification = (form: CreerPersonneForm) => {
  emit('personne-modifiee', form)
}

const surConfirmerSuppression = () => {
  confirmationSuppression.value = false
  emit('action-supprimer')
}
</script>

<template>
  <div
    class="
      absolute right-0 top-0 z-10 h-full w-80 border-l border-stone-200 bg-white shadow-lg
      max-md:bottom-0 max-md:left-0 max-md:right-0 max-md:top-auto max-md:h-auto max-md:max-h-[55vh]
      max-md:w-full max-md:rounded-t-2xl max-md:border-l-0 max-md:border-t
      animate-slide-in overflow-hidden flex flex-col
    "
  >
    <!-- En-tête -->
    <div class="flex shrink-0 items-center justify-between border-b border-stone-100 p-4">
      <div class="flex items-center gap-2">
        <button
          v-if="mode !== 'fiche'"
          class="rounded-lg p-1.5 text-stone-400 transition-colors hover:bg-stone-100 hover:text-stone-600"
          @click="emit('retour-fiche')"
        >
          <font-awesome-icon :icon="['fas', 'arrow-left']" class="text-sm" />
        </button>
        <h3 class="text-sm font-semibold text-stone-500">{{ titreMode }}</h3>
      </div>
      <button
        class="rounded-lg p-1.5 text-stone-400 transition-colors hover:bg-stone-100 hover:text-stone-600"
        @click="emit('fermer')"
      >
        <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>

    <!-- Contenu scrollable -->
    <div class="flex-1 overflow-y-auto p-4">

      <!-- ═══ MODE FICHE ═══ -->
      <template v-if="mode === 'fiche'">
        <!-- Photo / initiales -->
        <div class="mb-4 flex items-center gap-3">
          <img
            v-if="personne.photo_url"
            :src="personne.photo_url"
            :alt="nomComplet"
            class="h-14 w-14 rounded-full object-cover"
          />
          <div
            v-else
            :class="['flex h-14 w-14 items-center justify-center rounded-full text-lg font-bold', fondInitiales]"
          >
            {{ initiales }}
          </div>
          <div>
            <p class="text-lg font-bold text-stone-800">{{ nomComplet }}</p>
            <p class="text-sm text-stone-500">{{ genreLabel }}</p>
          </div>
        </div>

        <!-- Dates -->
        <div class="mb-4 space-y-2">
          <div v-if="personne.naissance" class="flex items-start gap-2 text-sm">
            <span class="mt-0.5 text-stone-400">★</span>
            <div>
              <span class="text-stone-700">{{ formaterDate(personne.naissance) }}</span>
              <span v-if="personne.naissance_lieu" class="text-stone-500"> — {{ personne.naissance_lieu }}</span>
            </div>
          </div>
          <div v-if="personne.deces" class="flex items-start gap-2 text-sm">
            <span class="mt-0.5 text-stone-400">✝</span>
            <div>
              <span class="text-stone-700">{{ formaterDate(personne.deces) }}</span>
              <span v-if="personne.deces_lieu" class="text-stone-500"> — {{ personne.deces_lieu }}</span>
            </div>
          </div>
        </div>

        <!-- Stats liens -->
        <div class="mb-4 flex flex-wrap gap-2">
          <div v-if="personne.parents.length > 0" class="rounded-lg bg-stone-50 px-3 py-1.5 text-xs text-stone-600">
            {{ personne.parents.length }} parent{{ personne.parents.length > 1 ? 's' : '' }}
          </div>
          <div v-if="personne.enfants.length > 0" class="rounded-lg bg-stone-50 px-3 py-1.5 text-xs text-stone-600">
            {{ personne.enfants.length }} enfant{{ personne.enfants.length > 1 ? 's' : '' }}
          </div>
          <div v-if="personne.conjoints.length > 0" class="rounded-lg bg-stone-50 px-3 py-1.5 text-xs text-stone-600">
            {{ personne.conjoints.length }} conjoint{{ personne.conjoints.length > 1 ? 's' : '' }}
          </div>
        </div>

        <!-- Bouton détail -->
        <NuxtLink
          :to="`/arbre-genealogique/${personne.personne_id}`"
          class="mb-4 flex w-full items-center justify-center gap-2 rounded-xl bg-[var(--color-custom-chocolat)] px-4 py-2.5 text-sm font-semibold text-white transition-colors hover:bg-[var(--color-custom-chocolat)]/90"
        >
          Voir le détail complet
          <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
          </svg>
        </NuxtLink>

        <!-- Actions -->
        <div class="space-y-2 border-t border-stone-100 pt-4">
          <p class="mb-2 text-xs font-semibold uppercase tracking-wider text-stone-400">Actions</p>
          <button
            class="flex w-full items-center gap-2 rounded-lg px-3 py-2 text-sm text-stone-700 transition-colors hover:bg-stone-50"
            @click="emit('action-ajouter', 'parent')"
          >
            <font-awesome-icon :icon="['fas', 'arrow-up']" class="w-4 text-stone-400" />
            Ajouter un parent
          </button>
          <button
            class="flex w-full items-center gap-2 rounded-lg px-3 py-2 text-sm text-stone-700 transition-colors hover:bg-stone-50"
            @click="emit('action-ajouter', 'enfant')"
          >
            <font-awesome-icon :icon="['fas', 'arrow-down']" class="w-4 text-stone-400" />
            Ajouter un enfant
          </button>
          <button
            class="flex w-full items-center gap-2 rounded-lg px-3 py-2 text-sm text-stone-700 transition-colors hover:bg-stone-50"
            @click="emit('action-ajouter', 'conjoint')"
          >
            <font-awesome-icon :icon="['fas', 'heart']" class="w-4 text-[var(--color-custom-chocolat)]" />
            Ajouter un(e) conjoint(e)
          </button>
          <div class="my-2 border-t border-stone-100" />
          <button
            class="flex w-full items-center gap-2 rounded-lg px-3 py-2 text-sm text-stone-700 transition-colors hover:bg-stone-50"
            @click="emit('action-modifier')"
          >
            <font-awesome-icon :icon="['fas', 'pen-to-square']" class="w-4 text-stone-400" />
            Modifier
          </button>
          <button
            class="flex w-full items-center gap-2 rounded-lg px-3 py-2 text-sm text-red-600 transition-colors hover:bg-red-50"
            @click="confirmationSuppression = true"
          >
            <font-awesome-icon :icon="['fas', 'trash']" class="w-4" />
            Supprimer
          </button>
        </div>

        <!-- Confirmation suppression -->
        <div
          v-if="confirmationSuppression"
          class="mt-4 rounded-xl border border-red-200 bg-red-50 p-4"
        >
          <p class="mb-2 text-sm font-semibold text-red-800">Supprimer {{ nomComplet }} ?</p>
          <p class="mb-3 text-xs text-red-600">
            Cette personne sera retirée de l'arbre.
            <span v-if="nbLiensTotal > 0">{{ nbLiensTotal }} lien{{ nbLiensTotal > 1 ? 's' : '' }} familial{{ nbLiensTotal > 1 ? 'aux' : '' }} {{ nbLiensTotal > 1 ? 'seront supprimés' : 'sera supprimé' }}.</span>
          </p>
          <div class="flex gap-2">
            <button
              class="flex-1 rounded-lg bg-red-600 px-3 py-2 text-sm font-semibold text-white transition-colors hover:bg-red-700"
              :disabled="mutationEnCours"
              @click="surConfirmerSuppression"
            >
              {{ mutationEnCours ? 'Suppression...' : 'Confirmer' }}
            </button>
            <button
              class="flex-1 rounded-lg border border-stone-300 px-3 py-2 text-sm text-stone-700 transition-colors hover:bg-stone-50"
              @click="confirmationSuppression = false"
            >
              Annuler
            </button>
          </div>
        </div>
      </template>

      <!-- ═══ MODE AJOUT ═══ -->
      <template v-else-if="mode === 'ajout' && contexteAjout">
        <!-- Sélecteur type de lien -->
        <div class="mb-4">
          <label class="mb-1 block text-xs font-semibold text-stone-500">Type de lien</label>
          <select
            v-model="typeLienChoisi"
            class="w-full rounded-lg border border-stone-300 px-3 py-2 text-sm text-stone-800 focus:outline-none focus:ring-2 focus:ring-[var(--color-custom-chocolat)]/50"
          >
            <option value="pere">Père</option>
            <option value="mere">Mère</option>
            <option value="parent">Parent (générique)</option>
            <option v-if="contexteAjout.typeAction === 'conjoint'" value="conjoint">Conjoint(e)</option>
          </select>
        </div>

        <!-- Erreur mutation -->
        <div v-if="erreurMutation" class="mb-4 rounded-lg bg-red-50 p-3 text-sm text-red-700">
          {{ erreurMutation }}
        </div>

        <!-- Formulaire -->
        <PersonneForm
          :loading="mutationEnCours"
          @submit="surSoumettreAjout"
          @annuler="emit('retour-fiche')"
        />
      </template>

      <!-- ═══ MODE MODIFIER ═══ -->
      <template v-else-if="mode === 'modifier'">
        <!-- Erreur mutation -->
        <div v-if="erreurMutation" class="mb-4 rounded-lg bg-red-50 p-3 text-sm text-red-700">
          {{ erreurMutation }}
        </div>

        <!-- Formulaire pré-rempli -->
        <PersonneForm
          :loading="mutationEnCours"
          :model-value="formModification"
          :mode-edition="true"
          @submit="surSoumettreModification"
          @annuler="emit('retour-fiche')"
        />
      </template>
    </div>
  </div>
</template>

<style scoped>
.animate-slide-in {
  animation: slideIn 0.25s ease-out;
}

@keyframes slideIn {
  from {
    transform: translateX(100%);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}

@media (max-width: 767px) {
  .animate-slide-in {
    animation: slideUp 0.25s ease-out;
  }

  @keyframes slideUp {
    from {
      transform: translateY(100%);
      opacity: 0;
    }
    to {
      transform: translateY(0);
      opacity: 1;
    }
  }
}
</style>
