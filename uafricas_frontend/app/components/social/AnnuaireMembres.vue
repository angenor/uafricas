<script setup lang="ts">
import type { MembreAPI } from '~/composables/useMembres'
import type { AmiAPI, EtatRelation, MembreLightAPI } from '~/composables/useAmis'
import { useUserStore } from '~/stores/user'

// Annuaire compact des inscrits pour la messagerie flottante.
// Les amis sont énumérés en tête (avec appel direct + message) ; l'annuaire
// général suit, sans les amis (déjà listés au-dessus).
const { listerMembres, chargement } = useMembres()
const { obtenirEtatsRelationLot, listerAmis } = useAmis()
const { appeler, erreurAppel } = useAppels()
const { demanderOuverture } = useMessagerie()
const userStore = useUserStore()

const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string

const PAR_PAGE = 10

const membres = ref<MembreAPI[]>([])
const etats = ref<Record<string, EtatRelation>>({})
const amis = ref<AmiAPI[]>([])
const recherche = ref('')
const rechercheActive = ref('')
const page = ref(1)
const totalPages = ref(1)

// Ensemble des ids amis : sert à les exclure de l'annuaire général.
const amiIds = computed(() => new Set(amis.value.map(a => a.utilisateur.id)))

// Amis filtrés par la recherche (côté client : liste complète déjà chargée).
const amisFiltres = computed<MembreLightAPI[]>(() => {
  const q = rechercheActive.value.toLowerCase()
  const liste = amis.value.map(a => a.utilisateur)
  const filtres = q
    ? liste.filter(u => `${u.prenom} ${u.nom}`.toLowerCase().includes(q))
    : liste
  return [...filtres].sort((a, b) =>
    `${a.prenom} ${a.nom}`.localeCompare(`${b.prenom} ${b.nom}`, 'fr'))
})

// Annuaire général sans les amis (ils figurent déjà dans « Mes amis »).
const membresAffiches = computed(() => membres.value.filter(m => !amiIds.value.has(m.id)))

const chargerAmis = async (): Promise<void> => {
  amis.value = await listerAmis()
}

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

onMounted(() => {
  erreurAppel.value = null
  charger()
  chargerAmis()
})

const photoComplete = (url: string | null): string | null => {
  if (!url) return null
  return url.startsWith('http') ? url : `${apiBase}${url}`
}

const initiaux = (prenom: string, nom: string): string =>
  `${prenom?.charAt(0)?.toUpperCase() || ''}${nom?.charAt(0)?.toUpperCase() || ''}`

const localisation = (m: MembreAPI): string =>
  [m.ville, m.pays].filter(Boolean).join(', ')

const majEtat = (id: string, etat: EtatRelation): void => {
  etats.value = { ...etats.value, [id]: etat }
  // Une nouvelle amitié (ou rupture) modifie la section « Mes amis ».
  if (etat === 'amis') chargerAmis()
}

const lancerAppel = (membre: MembreLightAPI): void => {
  void appeler(membre.id)
}

const ouvrirMessage = (membre: MembreLightAPI): void => {
  demanderOuverture(membre)
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

    <!-- Erreur d'appel éventuelle -->
    <div
      v-if="erreurAppel"
      class="mx-3 mt-2 px-3 py-2 text-xs text-red-700 bg-red-50 border border-red-100 rounded-lg shrink-0"
    >
      {{ erreurAppel }}
    </div>

    <!-- Contenu défilant : amis puis annuaire -->
    <div class="flex-1 min-h-0 overflow-y-auto">
      <!-- Section « Mes amis » -->
      <div v-if="amisFiltres.length > 0">
        <p class="px-3 pt-3 pb-1 text-[11px] font-bold uppercase tracking-wide text-custom-chocolat">
          Mes amis
        </p>
        <ul class="divide-y divide-gray-100">
          <li
            v-for="u in amisFiltres"
            :key="u.id"
            class="flex items-center gap-3 px-3 py-2.5 hover:bg-gray-50 transition"
          >
            <NuxtLink :to="`/profil/${u.id}`" class="shrink-0">
              <img
                v-if="photoComplete(u.photoUrl)"
                :src="photoComplete(u.photoUrl)!"
                :alt="`${u.prenom} ${u.nom}`"
                class="w-10 h-10 rounded-full object-cover border border-gray-200"
              >
              <div
                v-else
                class="w-10 h-10 rounded-full bg-custom-green text-white flex items-center justify-center text-xs font-bold"
              >
                {{ initiaux(u.prenom, u.nom) }}
              </div>
            </NuxtLink>

            <NuxtLink :to="`/profil/${u.id}`" class="flex-1 min-w-0">
              <p class="font-semibold text-gray-800 text-sm truncate">{{ u.prenom }} {{ u.nom }}</p>
              <p v-if="u.fonction" class="text-xs text-gray-500 truncate">{{ u.fonction }}</p>
            </NuxtLink>

            <!-- Message -->
            <button
              type="button"
              class="shrink-0 w-9 h-9 rounded-full text-gray-500 hover:text-custom-chocolat hover:bg-custom-chocolat/10 flex items-center justify-center transition"
              :aria-label="`Envoyer un message à ${u.prenom}`"
              :title="`Message à ${u.prenom}`"
              @click="ouvrirMessage(u)"
            >
              <font-awesome-icon icon="fa-solid fa-comment-dots" />
            </button>
            <!-- Appel direct -->
            <button
              type="button"
              class="shrink-0 w-9 h-9 rounded-full bg-custom-green text-white hover:brightness-110 flex items-center justify-center transition shadow-sm"
              :aria-label="`Appeler ${u.prenom}`"
              :title="`Appeler ${u.prenom} en visio`"
              @click="lancerAppel(u)"
            >
              <font-awesome-icon icon="fa-solid fa-video" />
            </button>
          </li>
        </ul>
      </div>

      <!-- Section annuaire général -->
      <p
        v-if="amisFiltres.length > 0 && membresAffiches.length > 0"
        class="px-3 pt-3 pb-1 text-[11px] font-bold uppercase tracking-wide text-gray-400"
      >
        Tous les membres
      </p>

      <div v-if="chargement && membres.length === 0" class="flex items-center justify-center py-10 text-gray-400">
        <font-awesome-icon icon="fa-solid fa-spinner" class="animate-spin text-xl" />
      </div>

      <div
        v-else-if="membresAffiches.length === 0 && amisFiltres.length === 0"
        class="flex flex-col items-center justify-center text-center px-6 py-10"
      >
        <font-awesome-icon icon="fa-solid fa-user-slash" class="text-2xl text-gray-300 mb-3" />
        <p class="text-sm text-gray-500">Aucun membre trouvé.</p>
      </div>

      <ul v-else-if="membresAffiches.length > 0" class="divide-y divide-gray-100">
        <li
          v-for="m in membresAffiches"
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
              {{ initiaux(m.prenom, m.nom) }}
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
