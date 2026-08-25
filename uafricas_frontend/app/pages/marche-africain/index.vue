<script setup lang="ts">
import {
  useMarcheAfricain,
  CATEGORIES,
  TYPES_ECHANGE,
  mapperTypesVersDb,
  type AnnonceAPI,
  type AnnonceDetailAPI,
  type AnnonceFiltres,
  type FiltresAnnonce,
  type Categorie,
  type TypeEchange,
} from '~/composables/useMarcheAfricain'
import { useUserStore } from '~/stores/user'

/**
 * Afromarket : porté sur le gabarit de la refonte.
 *
 * Filtres, tri, recherche débattue et pagination serveur sont inchangés. Les
 * critères (type d'échange, fourchette de prix) passent dans le rail : ils
 * étaient dupliqués entre une colonne desktop et un tiroir mobile avec voile,
 * que le gabarit rend inutile en empilant le rail sous 64rem.
 *
 * Les catégories étaient offertes DEUX FOIS sur la même page : en pastilles
 * sous la recherche, et par `MarcheCategoryButtons` plus bas. Une seule série
 * subsiste.
 */
definePageMeta({ layout: false })

useHead({
  title: 'Afromarket : Place de marché panafricaine | AfricanS',
  meta: [
    {
      name: 'description',
      content: "Annonces de vente, troc et dons à travers toute l'Afrique : agriculture, informatique, immobilier et plus.",
    }],
})

const ITEMS_PER_PAGE = 12

const { chargement, erreur, listerAnnonces } = useMarcheAfricain()
const userStore = useUserStore()
const { redirigerVersConnexion } = useAuth()

// ─── État ─────────────────────────────────────────────────────────────────

const annonces = ref<AnnonceAPI[]>([])
const totalAnnonces = ref(0)
const totalPages = ref(1)
const currentPage = ref(1)
const showPublishModal = ref(false)
const decouverteOuverte = ref(false)

const filtres = ref<FiltresAnnonce>({
  categorie: 'Tout',
  typesEchange: [],
  prixMin: null,
  prixMax: null,
  recherche: '',
  tri: 'recent',
})

const TRIS = [
  { valeur: 'recent' as const, libelle: 'Les plus récentes' },
  { valeur: 'price-asc' as const, libelle: 'Prix croissant' },
  { valeur: 'price-desc' as const, libelle: 'Prix décroissant' }]

let rechercheTimer: ReturnType<typeof setTimeout> | null = null

// ─── Chargement ───────────────────────────────────────────────────────────

const buildApiFiltres = (): AnnonceFiltres => {
  const f: AnnonceFiltres = {
    page: currentPage.value,
    par_page: ITEMS_PER_PAGE,
    tri: filtres.value.tri,
  }
  if (filtres.value.recherche.trim()) f.recherche = filtres.value.recherche.trim()
  if (filtres.value.categorie !== 'Tout') f.categorie = filtres.value.categorie
  if (filtres.value.typesEchange.length > 0) {
    f.type_operation = mapperTypesVersDb(filtres.value.typesEchange)
  }
  if (filtres.value.prixMin != null) f.prix_min = filtres.value.prixMin
  if (filtres.value.prixMax != null) f.prix_max = filtres.value.prixMax
  return f
}

const chargerAnnonces = async () => {
  const resultat = await listerAnnonces(buildApiFiltres())
  if (resultat) {
    annonces.value = resultat.annonces
    totalAnnonces.value = resultat.total
    totalPages.value = resultat.total_pages
  }
}

// Tous les filtres sauf la recherche, qui a son propre délai.
watch(
  () => ({
    categorie: filtres.value.categorie,
    typesEchange: [...filtres.value.typesEchange],
    prixMin: filtres.value.prixMin,
    prixMax: filtres.value.prixMax,
    tri: filtres.value.tri,
  }),
  () => {
    currentPage.value = 1
    chargerAnnonces()
  },
  { deep: true },
)

watch(() => filtres.value.recherche, () => {
  if (rechercheTimer) clearTimeout(rechercheTimer)
  rechercheTimer = setTimeout(() => {
    currentPage.value = 1
    chargerAnnonces()
  }, 300)
})

onMounted(chargerAnnonces)

// ─── Pagination ───────────────────────────────────────────────────────────

const visiblePages = computed(() => {
  const pages: (number | string)[] = []
  const total = totalPages.value
  const current = currentPage.value

  if (total <= 7) {
    for (let i = 1; i <= total; i++) pages.push(i)
  }
  else {
    pages.push(1)
    if (current > 3) pages.push('…')
    const debut = Math.max(2, current - 1)
    const fin = Math.min(total - 1, current + 1)
    for (let i = debut; i <= fin; i++) pages.push(i)
    if (current < total - 2) pages.push('…')
    pages.push(total)
  }

  return pages
})

const goToPage = (page: number) => {
  if (page < 1 || page > totalPages.value) return
  currentPage.value = page
  chargerAnnonces()
  window.scrollTo({ top: 0, behavior: 'smooth' })
}

// ─── Actions ──────────────────────────────────────────────────────────────

const nombreFiltresActifs = computed(() => {
  let n = 0
  if (filtres.value.categorie !== 'Tout') n++
  if (filtres.value.typesEchange.length > 0) n++
  if (filtres.value.prixMin !== null) n++
  if (filtres.value.prixMax !== null) n++
  if (filtres.value.recherche.trim() !== '') n++
  return n
})

const basculerType = (type: TypeEchange) => {
  const liste = filtres.value.typesEchange
  filtres.value.typesEchange = liste.includes(type)
    ? liste.filter(t => t !== type)
    : [...liste, type]
}

const resetFilters = () => {
  filtres.value = {
    categorie: 'Tout',
    typesEchange: [],
    prixMin: null,
    prixMax: null,
    recherche: '',
    tri: 'recent',
  }
  currentPage.value = 1
}

const handlePublish = () => {
  if (!userStore.isAuthenticated) return redirigerVersConnexion()
  showPublishModal.value = true
}

const onPublicationReussie = async (detail: AnnonceDetailAPI) => {
  showPublishModal.value = false
  await chargerAnnonces()
  navigateTo(`/marche-africain/${detail.id}`)
}
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Afromarket"
        sous-titre="Acheter, vendre, échanger et s'entraider à travers le continent"
        image="/images/marche-afrique.png"
        aide="C'est quoi Afromarket ?"
        @aide="decouverteOuverte = true"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane :segments="[{ libelle: 'Afromarket' }]">
        <template #action>
          <AfricansBouton icone="fa-solid fa-plus" @click="handlePublish">
            Publier une annonce
          </AfricansBouton>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="flex flex-col gap-6">
      <label class="relative block">
        <span class="sr-only">Rechercher une annonce</span>
        <font-awesome-icon
          icon="fa-solid fa-magnifying-glass"
          class="pointer-events-none absolute top-1/2 left-4 -translate-y-1/2 text-af-atone-2"
        />
        <input
          v-model="filtres.recherche"
          type="search"
          placeholder="Produit, service, mot-clé…"
          class="h-11 w-full rounded-[10px] border border-af-bordure bg-white pr-4 pl-11 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
        />
      </label>

      <!-- Catégories : une seule série, ici. -->
      <div class="flex flex-wrap gap-2">
        <button
          v-for="cat in CATEGORIES"
          :key="cat.key"
          type="button"
          class="rounded-full px-3 py-1.5 text-[12px]/[1.4] font-bold transition"
          :class="filtres.categorie === cat.key ? 'bg-af-chocolat text-white' : 'bg-af-fond text-af-corps hover:bg-af-bordure'"
          :aria-pressed="filtres.categorie === cat.key"
          @click="filtres.categorie = cat.key as Categorie | 'Tout'"
        >
          {{ cat.label }}
        </button>
      </div>

      <div class="flex flex-wrap items-center justify-between gap-3">
        <p class="text-[14px]/[1.4] text-af-atone">
          <span class="font-bold text-af-encre">{{ totalAnnonces }}</span>
          annonce{{ totalAnnonces > 1 ? 's' : '' }}
          <span v-if="nombreFiltresActifs > 0">
            ({{ nombreFiltresActifs }} filtre{{ nombreFiltresActifs > 1 ? 's' : '' }})
          </span>
        </p>
        <label class="flex items-center gap-2">
          <span class="text-[14px]/[1.4] text-af-corps">Trier par</span>
          <select
            v-model="filtres.tri"
            class="h-10 rounded-[10px] border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:outline-2 focus:outline-af-chocolat"
          >
            <option v-for="tri in TRIS" :key="tri.valeur" :value="tri.valeur">{{ tri.libelle }}</option>
          </select>
        </label>
      </div>

      <div v-if="chargement" class="grid gap-5 sm:grid-cols-2">
        <div v-for="n in 4" :key="n" class="h-72 animate-pulse rounded-[10px] bg-af-bordure" />
      </div>

      <div v-else-if="erreur" class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
        <font-awesome-icon icon="fa-solid fa-triangle-exclamation" class="text-4xl text-af-live" />
        <p class="mt-4 text-[16px]/[1.4] font-bold">Impossible de charger les annonces</p>
        <p class="mt-2 text-[14px]/[1.4] text-af-corps">{{ erreur }}</p>
        <AfricansBouton class="mt-6" variante="secondaire" icone="fa-solid fa-rotate-left" @click="chargerAnnonces">
          Réessayer
        </AfricansBouton>
      </div>

      <template v-else-if="annonces.length">
        <div class="grid gap-5 sm:grid-cols-2">
          <MarcheAnnonceCard v-for="annonce in annonces" :key="annonce.id" :annonce="annonce" />
        </div>

        <nav v-if="totalPages > 1" class="flex flex-wrap items-center justify-center gap-2">
          <button
            type="button"
            :disabled="currentPage === 1"
            class="grid size-10 place-items-center rounded-[10px] border border-af-bordure bg-white transition hover:border-af-chocolat disabled:opacity-40"
            aria-label="Page précédente"
            @click="goToPage(currentPage - 1)"
          >
            <font-awesome-icon icon="fa-solid fa-chevron-left" />
          </button>

          <template v-for="(page, i) in visiblePages" :key="`${page}-${i}`">
            <span v-if="typeof page === 'string'" class="px-1 text-af-atone">{{ page }}</span>
            <button
              v-else
              type="button"
              class="size-10 rounded-[10px] text-[14px]/[1.4] font-bold transition"
              :class="page === currentPage ? 'bg-af-chocolat text-white' : 'border border-af-bordure bg-white hover:border-af-chocolat'"
              :aria-current="page === currentPage ? 'page' : undefined"
              @click="goToPage(page)"
            >
              {{ page }}
            </button>
          </template>

          <button
            type="button"
            :disabled="currentPage === totalPages"
            class="grid size-10 place-items-center rounded-[10px] border border-af-bordure bg-white transition hover:border-af-chocolat disabled:opacity-40"
            aria-label="Page suivante"
            @click="goToPage(currentPage + 1)"
          >
            <font-awesome-icon icon="fa-solid fa-chevron-right" />
          </button>
        </nav>
      </template>

      <div v-else class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
        <font-awesome-icon icon="fa-solid fa-store" class="text-4xl text-af-atone-2" />
        <p class="mt-4 text-[16px]/[1.4] font-bold">Aucune annonce trouvée</p>
        <p class="mt-2 text-[14px]/[1.4] text-af-corps">
          Essayez de modifier vos filtres, ou publiez la première annonce.
        </p>
        <AfricansBouton class="mt-6" variante="secondaire" icone="fa-solid fa-rotate-left" @click="resetFilters">
          Réinitialiser les filtres
        </AfricansBouton>
      </div>
    </div>

    <template #rail>
      <AfricansPanneau titre="Filtres" icone="fa-solid fa-sliders" action-libelle="Réinitialiser" @action="resetFilters">
        <div class="flex flex-col gap-5">
          <div class="flex flex-col gap-2">
            <p class="text-[12px]/[1.4] font-bold text-af-atone uppercase">Type d'échange</p>
            <label
              v-for="type in TYPES_ECHANGE"
              :key="type.value"
              class="flex cursor-pointer items-center gap-3 text-[14px]/[1.4] text-af-corps"
            >
              <input
                type="checkbox"
                class="size-4 accent-af-chocolat"
                :checked="filtres.typesEchange.includes(type.value)"
                @change="basculerType(type.value)"
              />
              {{ type.label }}
            </label>
          </div>

          <div class="flex flex-col gap-2">
            <p class="text-[12px]/[1.4] font-bold text-af-atone uppercase">Fourchette de prix</p>
            <div class="flex items-center gap-2">
              <input
                v-model.number="filtres.prixMin"
                type="number"
                min="0"
                placeholder="Min"
                class="h-10 w-full rounded-[10px] border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:outline-2 focus:outline-af-chocolat"
              />
              <span class="text-af-atone">–</span>
              <input
                v-model.number="filtres.prixMax"
                type="number"
                min="0"
                placeholder="Max"
                class="h-10 w-full rounded-[10px] border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:outline-2 focus:outline-af-chocolat"
              />
            </div>
          </div>
        </div>
      </AfricansPanneau>

      <AfricansPanneau v-if="userStore.isAuthenticated" titre="Mon espace" icone="fa-solid fa-store">
        <div class="flex flex-col gap-3">
          <AfricansBouton variante="secondaire" icone="fa-solid fa-list" vers="/marche-africain/mes-annonces">
            Mes annonces
          </AfricansBouton>
          <AfricansBouton variante="secondaire" icone="fa-solid fa-heart" vers="/marche-africain/favoris">
            Mes favoris
          </AfricansBouton>
        </div>
      </AfricansPanneau>
    </template>

    <!-- Publication d'une annonce -->
    <AfricansModale
      v-model="showPublishModal"
      titre="Publier une annonce"
      sous-titre="Vente, troc, don ou opportunité d'investissement"
    >
      <MarcheAnnonceForm @success="onPublicationReussie" @cancel="showPublishModal = false" />
    </AfricansModale>

    <MarcheDecouverteModale v-model="decouverteOuverte" />
  </NuxtLayout>
</template>
