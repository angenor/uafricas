<script setup lang="ts">
definePageMeta({ middleware: 'auth' })

import type { PersonneListe, CreerPersonneForm } from '~/mocks/arbre-genealogique'
import { useArbreGenealogique } from '~/composables/useArbreGenealogique'
import { useDecouvertes } from '~/composables/useDecouvertes'
import PersonneForm from '~/components/arbre-genealogique/PersonneForm.vue'
import PersonneCard from '~/components/arbre-genealogique/PersonneCard.vue'
import AssistantAjoutPersonne from '~/components/arbre-genealogique/AssistantAjoutPersonne.vue'

const { listerPersonnes, creerPersonne } = useArbreGenealogique()
const { listerDecouvertes } = useDecouvertes()

// ─── Compteur découvertes ────────────────────────────────────────────────

const nbDecouvertes = ref(0)

async function chargerDecouvertes() {
  try {
    const res = await listerDecouvertes()
    if (res.success && res.data) {
      nbDecouvertes.value = res.data.en_attente.total + res.data.en_cours.total
    }
  } catch {}
}

onMounted(chargerDecouvertes)

// ─── État ─────────────────────────────────────────────────────────────────

const liste = ref<PersonneListe | null>(null)
const chargement = ref(true)
const page = ref(1)
const parPage = 12
const recherche = ref('')
const rechercheDebounce = ref('')
let timerId: ReturnType<typeof setTimeout> | null = null

const modeAjout = ref<'wizard' | 'classique' | null>(null)
const creationEnCours = ref(false)

// ─── Debounce recherche ───────────────────────────────────────────────────

watch(recherche, (val) => {
  if (timerId) clearTimeout(timerId)
  timerId = setTimeout(() => {
    rechercheDebounce.value = val
    page.value = 1
  }, 350)
})

// ─── Chargement ────────────────────────────────────────────────────────────

async function charger() {
  chargement.value = true
  try {
    const res = await listerPersonnes({
      page: page.value,
      par_page: parPage,
      recherche: rechercheDebounce.value || undefined,
    })
    if (res.success && res.data) {
      liste.value = res.data
    }
  } finally {
    chargement.value = false
  }
}

watch([page, rechercheDebounce], charger, { immediate: true })

// ─── Navigation ────────────────────────────────────────────────────────────

const router = useRouter()

function ouvrirFiche(id: string) {
  router.push(`/arbre-genealogique/${id}`)
}

// ─── Création ─────────────────────────────────────────────────────────────

async function ajouterPersonne(form: CreerPersonneForm) {
  creationEnCours.value = true
  try {
    const res = await creerPersonne(form)
    if (res.success && res.data) {
      modeAjout.value = null
      await charger()
    }
  } catch (e: any) {
    // Si 401, tenter un refresh du token et réessayer
    if (e?.statusCode === 401 || e?.status === 401) {
      try {
        const { refreshAccessToken } = useAuth()
        await refreshAccessToken()
        const res = await creerPersonne(form)
        if (res.success && res.data) {
          modeAjout.value = null
          await charger()
        }
      } catch {
        navigateTo('/login?redirect=/arbre-genealogique')
      }
    }
  } finally {
    creationEnCours.value = false
  }
}

// ─── Pagination ────────────────────────────────────────────────────────────

const totalPages = computed(() => liste.value?.total_pages ?? 1)
const infoPagination = computed(() => {
  if (!liste.value) return ''
  const debut = (page.value - 1) * parPage + 1
  const fin = Math.min(page.value * parPage, liste.value.total)
  return `${debut}–${fin} sur ${liste.value.total}`
})
</script>

<template>
  <div class="mt-28">
    <!-- Hero Section -->
    <div class="bg-gradient-to-br from-[var(--color-custom-green)]/10 via-white to-[var(--color-custom-chocolat)]/5 px-4 py-12 text-center">
      <div class="mx-auto max-w-2xl">
        <div class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-[var(--color-custom-green)]/20">
          <font-awesome-icon :icon="['fas', 'tree']" class="text-2xl text-[var(--color-custom-green)]" />
        </div>
        <h1 class="mb-2 text-3xl font-bold text-stone-800">Rootstree</h1>
        <p class="text-stone-500">
          Explorez votre arbre généalogique, découvrez vos origines et connectez-vous avec votre famille à travers les générations.
        </p>
      </div>
    </div>

    <div class="max-w-5xl mx-auto px-4 py-8">
    <!-- En-tête -->
    <div class="flex items-center justify-between mb-6">
      <div>
        <h2 class="text-xl font-bold text-stone-800">Mon arbre</h2>
        <p v-if="liste && liste.total > 0" class="text-sm text-stone-500 mt-1">
          {{ liste.total }} personne{{ liste.total > 1 ? 's' : '' }} dans votre arbre
        </p>
      </div>
      <div class="flex items-center gap-3">
        <NuxtLink
          to="/arbre-genealogique/decouvertes"
          class="relative flex items-center gap-2 px-4 py-2.5 border-2 border-amber-500 text-amber-600 font-semibold rounded-xl hover:bg-amber-50 transition-colors"
        >
          <font-awesome-icon :icon="['fas', 'users']" class="text-sm" />
          Découvertes
          <span
            v-if="nbDecouvertes > 0"
            class="absolute -right-2 -top-2 flex h-5 min-w-5 items-center justify-center rounded-full bg-red-500 px-1 text-[10px] font-bold text-white"
          >
            {{ nbDecouvertes }}
          </span>
        </NuxtLink>
        <NuxtLink
          to="/arbre-genealogique/visualisation"
          class="flex items-center gap-2 px-5 py-2.5 border-2 border-[var(--color-custom-green)] text-[var(--color-custom-green)] font-semibold rounded-xl hover:bg-[var(--color-custom-green)]/10 transition-colors"
        >
          <font-awesome-icon :icon="['fas', 'project-diagram']" class="text-sm" />
          Voir mon arbre
        </NuxtLink>
        <button
          class="px-5 py-2.5 bg-custom-chocolat text-white font-semibold rounded-xl hover:opacity-90 transition-opacity"
          @click="modeAjout = 'wizard'"
        >
          + Ajouter une personne
        </button>
      </div>
    </div>

    <!-- Wizard ajout -->
    <AssistantAjoutPersonne
      v-if="modeAjout === 'wizard'"
      :loading="creationEnCours"
      @submit="ajouterPersonne"
      @annuler="modeAjout = null"
      @formulaire-classique="modeAjout = 'classique'"
    />

    <!-- Modal classique -->
    <div
      v-if="modeAjout === 'classique'"
      class="fixed inset-0 bg-black/50 z-50 flex items-center justify-center p-4"
      @click.self="modeAjout = null"
    >
      <div class="bg-white rounded-2xl shadow-xl w-full max-w-lg max-h-[90vh] overflow-y-auto p-6">
        <div class="flex items-center justify-between mb-4">
          <h2 class="text-lg font-bold text-stone-800">Nouvelle personne</h2>
          <button
            class="text-stone-400 hover:text-stone-600 transition-colors"
            @click="modeAjout = null"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
        <PersonneForm
          :loading="creationEnCours"
          @submit="ajouterPersonne"
          @annuler="modeAjout = null"
        />
      </div>
    </div>

    <!-- Recherche -->
    <div class="relative mb-6">
      <svg
        class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-stone-400"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0" />
      </svg>
      <input
        v-model="recherche"
        type="text"
        placeholder="Rechercher par nom ou prénom…"
        class="w-full pl-10 pr-4 py-2.5 border border-stone-300 rounded-xl text-stone-800 placeholder-stone-400 focus:outline-none focus:ring-2 focus:ring-custom-chocolat/50"
      />
    </div>

    <!-- Chargement -->
    <div v-if="chargement" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
      <div
        v-for="i in 6"
        :key="i"
        class="h-20 bg-stone-100 rounded-xl animate-pulse"
      />
    </div>

    <!-- Liste -->
    <div v-else-if="liste && liste.personnes.length > 0">
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
        <PersonneCard
          v-for="personne in liste.personnes"
          :key="personne.id"
          :personne="personne"
          @click="ouvrirFiche"
        />
      </div>

      <!-- Pagination -->
      <div v-if="totalPages > 1" class="flex items-center justify-between mt-8">
        <p class="text-sm text-stone-500">{{ infoPagination }}</p>
        <div class="flex items-center gap-2">
          <button
            :disabled="page <= 1"
            class="px-4 py-2 text-sm font-semibold border border-stone-300 text-stone-700 rounded-lg hover:bg-stone-50 transition-colors disabled:opacity-40 disabled:cursor-not-allowed"
            @click="page--"
          >
            Précédent
          </button>
          <span class="text-sm text-stone-600 px-2">{{ page }} / {{ totalPages }}</span>
          <button
            :disabled="page >= totalPages"
            class="px-4 py-2 text-sm font-semibold border border-stone-300 text-stone-700 rounded-lg hover:bg-stone-50 transition-colors disabled:opacity-40 disabled:cursor-not-allowed"
            @click="page++"
          >
            Suivant
          </button>
        </div>
      </div>
    </div>

    <!-- État vide -->
    <div v-else class="text-center py-20">
      <div class="w-16 h-16 bg-stone-100 rounded-full flex items-center justify-center mx-auto mb-4">
        <svg class="w-8 h-8 text-stone-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z" />
        </svg>
      </div>
      <p class="text-xl font-bold text-stone-700 mb-2">
        {{ recherche ? 'Aucune personne trouvée' : 'Votre arbre est vide' }}
      </p>
      <p class="text-stone-500 mb-6">
        {{ recherche
          ? `Aucun résultat pour « ${recherche} »`
          : 'Ajoutez votre première personne pour commencer à construire votre arbre généalogique.'
        }}
      </p>
      <button
        v-if="!recherche"
        class="px-6 py-3 bg-custom-chocolat text-white font-semibold rounded-xl hover:opacity-90 transition-opacity"
        @click="modeAjout = 'wizard'"
      >
        Ajouter ma première personne
      </button>
    </div>
    </div>
  </div>
</template>
