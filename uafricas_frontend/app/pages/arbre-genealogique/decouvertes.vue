<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useDecouvertes } from '~/composables/useDecouvertes'
import type { SuggestionCorrespondance, DecouvertesResponse } from '~/mocks/matching'
import CarteSuggestion from '~/components/arbre-genealogique/CarteSuggestion.vue'

definePageMeta({
  middleware: 'auth',
  layout: 'default',
})

const { listerDecouvertes, confirmerSuggestion, rejeterSuggestion } = useDecouvertes()

// ─── État ─────────────────────────────────────────────────────────────

const chargement = ref(true)
const erreur = ref<string | null>(null)
const donnees = ref<DecouvertesResponse | null>(null)
const actionEnCours = ref<string | null>(null)

const estVide = computed(() => {
  if (!donnees.value) return true
  return donnees.value.en_attente.total === 0
    && donnees.value.en_cours.total === 0
    && donnees.value.confirmees.total === 0
})

// ─── Chargement ──────────────────────────────────────────────────────

const charger = async () => {
  chargement.value = true
  erreur.value = null
  try {
    const reponse = await listerDecouvertes()
    if (reponse.success && reponse.data) {
      donnees.value = reponse.data
    }
  } catch (e: any) {
    erreur.value = e?.data?.error || 'Erreur lors du chargement des découvertes'
  } finally {
    chargement.value = false
  }
}

// ─── Actions ─────────────────────────────────────────────────────────

const surConfirmer = async (id: string) => {
  actionEnCours.value = id
  try {
    const reponse = await confirmerSuggestion(id)
    if (reponse.success) {
      await charger()
    }
  } catch (e: any) {
    erreur.value = e?.data?.error || 'Erreur lors de la confirmation'
  } finally {
    actionEnCours.value = null
  }
}

const surRejeter = async (id: string) => {
  actionEnCours.value = id
  try {
    const reponse = await rejeterSuggestion(id)
    if (reponse.success) {
      await charger()
    }
  } catch (e: any) {
    erreur.value = e?.data?.error || 'Erreur lors du rejet'
  } finally {
    actionEnCours.value = null
  }
}

onMounted(charger)
</script>

<template>
  <div class="mt-28 min-h-screen">
    <!-- Hero Section -->
    <div class="bg-gradient-to-br from-[var(--color-custom-green)]/10 to-[var(--color-custom-chocolat)]/5 px-4 py-12 text-center">
      <div class="mx-auto max-w-2xl">
        <div class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-[var(--color-custom-green)]/20">
          <font-awesome-icon :icon="['fas', 'users']" class="text-2xl text-[var(--color-custom-green)]" />
        </div>
        <h1 class="mb-2 text-3xl font-bold text-stone-800">Mes Découvertes</h1>
        <p class="text-stone-500">
          Explorez les correspondances détectées entre votre arbre et ceux d'autres membres de la communauté.
        </p>
      </div>
    </div>

    <div class="mx-auto max-w-4xl px-4 py-8">
      <!-- Chargement -->
      <div v-if="chargement" class="space-y-4">
        <div v-for="i in 3" :key="i" class="h-32 animate-pulse rounded-xl bg-stone-100" />
      </div>

      <!-- Erreur -->
      <div v-else-if="erreur" class="text-center py-12">
        <p class="mb-4 text-lg text-red-600">{{ erreur }}</p>
        <button
          class="rounded-lg bg-[var(--color-custom-chocolat)] px-6 py-2 text-white transition-colors hover:bg-[var(--color-custom-chocolat)]/90"
          @click="charger"
        >
          Réessayer
        </button>
      </div>

      <!-- État vide -->
      <div v-else-if="estVide" class="text-center py-16">
        <div class="mx-auto mb-4 flex h-20 w-20 items-center justify-center rounded-full bg-stone-100">
          <font-awesome-icon :icon="['fas', 'search']" class="text-3xl text-stone-400" />
        </div>
        <h2 class="mb-2 text-xl font-semibold text-stone-700">Aucune correspondance pour l'instant</h2>
        <p class="mb-6 max-w-md mx-auto text-stone-500">
          Enrichissez votre arbre généalogique avec plus de personnes, noms et lieux pour augmenter vos chances de découvrir des liens familiaux.
        </p>
        <NuxtLink
          to="/arbre-genealogique"
          class="rounded-lg bg-[var(--color-custom-chocolat)] px-6 py-2 text-white transition-colors hover:bg-[var(--color-custom-chocolat)]/90"
        >
          Enrichir mon arbre
        </NuxtLink>
      </div>

      <!-- Sections -->
      <template v-else-if="donnees">
        <!-- Suggestions en attente -->
        <section v-if="donnees.en_attente.total > 0" class="mb-10">
          <div class="mb-4 flex items-center gap-2">
            <h2 class="text-lg font-bold text-stone-800">Nouvelles suggestions</h2>
            <span class="rounded-full bg-amber-100 px-2.5 py-0.5 text-xs font-semibold text-amber-700">
              {{ donnees.en_attente.total }}
            </span>
          </div>
          <div class="space-y-4">
            <CarteSuggestion
              v-for="s in donnees.en_attente.suggestions"
              :key="s.id"
              :suggestion="s"
              @confirmer="surConfirmer"
              @rejeter="surRejeter"
            />
          </div>
        </section>

        <!-- En cours de confirmation -->
        <section v-if="donnees.en_cours.total > 0" class="mb-10">
          <div class="mb-4 flex items-center gap-2">
            <h2 class="text-lg font-bold text-stone-800">En attente de l'autre membre</h2>
            <span class="rounded-full bg-stone-100 px-2.5 py-0.5 text-xs font-semibold text-stone-600">
              {{ donnees.en_cours.total }}
            </span>
          </div>
          <div class="space-y-4">
            <CarteSuggestion
              v-for="s in donnees.en_cours.suggestions"
              :key="s.id"
              :suggestion="s"
              @confirmer="surConfirmer"
              @rejeter="surRejeter"
            />
          </div>
        </section>

        <!-- Correspondances confirmées -->
        <section v-if="donnees.confirmees.total > 0" class="mb-10">
          <div class="mb-4 flex items-center gap-2">
            <h2 class="text-lg font-bold text-stone-800">Correspondances confirmées</h2>
            <span class="rounded-full bg-green-100 px-2.5 py-0.5 text-xs font-semibold text-green-700">
              {{ donnees.confirmees.total }}
            </span>
          </div>
          <div class="space-y-4">
            <div
              v-for="s in donnees.confirmees.suggestions"
              :key="s.id"
              class="rounded-xl border border-green-200 bg-green-50/50 p-4"
            >
              <div class="mb-2 flex items-center justify-between">
                <p class="font-semibold text-stone-800">
                  {{ s.ma_personne.prenoms }} {{ s.ma_personne.nom }}
                  ↔
                  {{ s.personne_matchee.prenoms }} {{ s.personne_matchee.nom }}
                </p>
                <span class="text-xs text-green-600">Confirmée</span>
              </div>
              <div class="flex items-center gap-3">
                <NuxtLink
                  :to="`/arbre-genealogique/visualisation?centre=${s.ma_personne.rattachement_id}`"
                  class="rounded-lg bg-[var(--color-custom-green)] px-4 py-1.5 text-sm font-semibold text-white transition-colors hover:bg-[var(--color-custom-green)]/90"
                >
                  Voir dans l'arbre
                </NuxtLink>
                <span class="text-xs text-stone-500">{{ s.membre_id_anonymise }}</span>
              </div>
            </div>
          </div>
        </section>
      </template>
    </div>
  </div>
</template>
