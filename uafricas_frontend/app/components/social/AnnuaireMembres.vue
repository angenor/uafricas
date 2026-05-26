<script setup lang="ts">
import type { MembreAPI } from '~/composables/useMembres'
import type { EtatRelation } from '~/composables/useAmis'
import { useUserStore } from '~/stores/user'

// Annuaire compact des inscrits pour la messagerie flottante.
// Réutilise la logique de la page /profil : listerMembres + états de relation en lot.
const { listerMembres, chargement } = useMembres()
const { obtenirEtatsRelationLot } = useAmis()
const userStore = useUserStore()

const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string

const PAR_PAGE = 10

const membres = ref<MembreAPI[]>([])
const etats = ref<Record<string, EtatRelation>>({})
const recherche = ref('')
const rechercheActive = ref('')
const page = ref(1)
const totalPages = ref(1)

const charger = async (): Promise<void> => {
  const res = await listerMembres({
    recherche: rechercheActive.value || undefined,
    page: page.value,
    par_page: PAR_PAGE,
  })
  if (!res) return

  // Exclure l'utilisateur courant de la liste : il ne se contacte pas lui-même.
  membres.value = res.membres.filter(m => m.id !== userStore.user?.id)
  totalPages.value = res.total_pages

  // Charger les états de relation en lot (anti N+1), comme dans /profil.
  // L'id courant est déjà exclu ci-dessus : sinon le backend rejette le lot entier.
  const ids = membres.value.map(m => m.id)
  etats.value = ids.length > 0 ? await obtenirEtatsRelationLot(ids) : {}
}

// Debounce de la recherche.
let minuteur: ReturnType<typeof setTimeout> | null = null
watch(recherche, (v) => {
  if (minuteur) clearTimeout(minuteur)
  minuteur = setTimeout(() => {
    rechercheActive.value = v.trim()
  }, 350)
})

watch([rechercheActive], () => {
  page.value = 1
  charger()
})

watch(page, charger)

onMounted(charger)

const photoComplete = (url: string | null): string | null => {
  if (!url) return null
  return url.startsWith('http') ? url : `${apiBase}${url}`
}

const initiaux = (m: MembreAPI): string =>
  `${m.prenom?.charAt(0)?.toUpperCase() || ''}${m.nom?.charAt(0)?.toUpperCase() || ''}`

const localisation = (m: MembreAPI): string =>
  [m.ville, m.pays].filter(Boolean).join(', ')

const majEtat = (id: string, etat: EtatRelation): void => {
  etats.value = { ...etats.value, [id]: etat }
}

const pagePrecedente = (): void => {
  if (page.value > 1) page.value -= 1
}

const pageSuivante = (): void => {
  if (page.value < totalPages.value) page.value += 1
}
</script>

<template>
  <div class="flex flex-col min-h-0">
    <!-- Barre de recherche -->
    <div class="px-3 py-2 border-b border-gray-100 shrink-0">
      <div class="relative">
        <font-awesome-icon
          icon="fa-solid fa-magnifying-glass"
          class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 text-xs"
        />
        <input
          v-model="recherche"
          type="search"
          placeholder="Rechercher un membre…"
          class="w-full pl-8 pr-3 py-2 text-sm rounded-xl bg-gray-50 border border-gray-200 focus:border-custom-chocolat focus:outline-none transition"
        >
      </div>
    </div>

    <!-- Liste des membres -->
    <div class="flex-1 min-h-0 overflow-y-auto">
      <div v-if="chargement && membres.length === 0" class="flex items-center justify-center py-10 text-gray-400">
        <font-awesome-icon icon="fa-solid fa-spinner" class="animate-spin text-xl" />
      </div>

      <div
        v-else-if="membres.length === 0"
        class="flex flex-col items-center justify-center text-center px-6 py-10"
      >
        <font-awesome-icon icon="fa-solid fa-user-slash" class="text-2xl text-gray-300 mb-3" />
        <p class="text-sm text-gray-500">Aucun membre trouvé.</p>
      </div>

      <ul v-else class="divide-y divide-gray-100">
        <li
          v-for="m in membres"
          :key="m.id"
          class="flex items-center gap-3 px-3 py-2.5 hover:bg-gray-50 transition"
        >
          <NuxtLink :to="`/profil/${m.id}`" class="shrink-0">
            <img
              v-if="photoComplete(m.photoUrl)"
              :src="photoComplete(m.photoUrl)!"
              :alt="`${m.prenom} ${m.nom}`"
              class="w-10 h-10 rounded-full object-cover border border-gray-200"
            >
            <div
              v-else
              class="w-10 h-10 rounded-full bg-custom-chocolat text-white flex items-center justify-center text-xs font-bold"
            >
              {{ initiaux(m) }}
            </div>
          </NuxtLink>

          <NuxtLink :to="`/profil/${m.id}`" class="flex-1 min-w-0">
            <p class="font-semibold text-gray-800 text-sm truncate">{{ m.prenom }} {{ m.nom }}</p>
            <p v-if="m.fonction" class="text-xs text-gray-500 truncate">{{ m.fonction }}</p>
            <p v-else-if="localisation(m)" class="text-xs text-gray-400 truncate">
              <font-awesome-icon icon="fa-solid fa-location-dot" class="mr-1" />{{ localisation(m) }}
            </p>
          </NuxtLink>

          <SocialBoutonAmitie
            :utilisateur-id="m.id"
            :etat="etats[m.id] || 'aucune'"
            taille="sm"
            class="shrink-0"
            @update="(e) => majEtat(m.id, e)"
          />
        </li>
      </ul>
    </div>

    <!-- Pagination -->
    <div
      v-if="totalPages > 1"
      class="flex items-center justify-center gap-3 px-3 py-2 border-t border-gray-100 shrink-0"
    >
      <button
        type="button"
        :disabled="page <= 1"
        class="p-1.5 rounded-lg text-gray-500 hover:bg-gray-100 disabled:opacity-40 disabled:hover:bg-transparent transition"
        aria-label="Page précédente"
        @click="pagePrecedente"
      >
        <font-awesome-icon icon="fa-solid fa-chevron-left" class="text-xs" />
      </button>
      <span class="text-xs text-gray-500">{{ page }} / {{ totalPages }}</span>
      <button
        type="button"
        :disabled="page >= totalPages"
        class="p-1.5 rounded-lg text-gray-500 hover:bg-gray-100 disabled:opacity-40 disabled:hover:bg-transparent transition"
        aria-label="Page suivante"
        @click="pageSuivante"
      >
        <font-awesome-icon icon="fa-solid fa-chevron-right" class="text-xs" />
      </button>
    </div>
  </div>
</template>
