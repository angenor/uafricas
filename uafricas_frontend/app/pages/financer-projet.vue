<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Financer un projet"
        sous-titre="Découvrez des projets innovants et contribuez au développement durable du continent africain"
        image="/images/finance_projet_banire.png"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane :segments="[{ libelle: 'Financer un projet' }]">
        <template #centre>
          <p class="text-base font-bold text-af-encre">
            {{ total }} projet{{ total > 1 ? 's' : '' }}
          </p>
        </template>
        <template #action>
          <AfricansBouton icone="fa-solid fa-paper-plane" vers="/soumettre-projet">
            Soumettre mon projet
          </AfricansBouton>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="flex flex-col gap-6">
      <form class="flex flex-wrap gap-3" @submit.prevent="handleSearch">
        <label class="relative min-w-0 flex-1">
          <span class="sr-only">Rechercher un projet</span>
          <font-awesome-icon
            icon="fa-solid fa-magnifying-glass"
            class="pointer-events-none absolute top-1/2 left-4 -translate-y-1/2 text-af-atone-2"
          />
          <input
            v-model="filtres.recherche"
            type="search"
            placeholder="Titre, porteur, mot-clé…"
            class="h-11 w-full rounded-[10px] border border-af-bordure bg-white pr-4 pl-11 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
          />
        </label>
        <AfricansBouton type="submit" icone="fa-solid fa-magnifying-glass">Rechercher</AfricansBouton>
      </form>

      <div v-if="loading" class="grid gap-5 sm:grid-cols-2">
        <div v-for="n in 4" :key="n" class="h-80 animate-pulse rounded-[10px] bg-af-bordure" />
      </div>

      <template v-else-if="projets.length > 0">
        <div class="grid gap-5 sm:grid-cols-2">
          <ProjetsProjetCard v-for="projet in projets" :key="projet.id" :projet="projet" />
        </div>

        <nav v-if="totalPages > 1" class="flex items-center justify-center gap-2">
          <button
            type="button"
            :disabled="currentPage === 1"
            class="grid size-10 place-items-center rounded-[10px] border border-af-bordure bg-white transition hover:border-af-chocolat disabled:opacity-40"
            aria-label="Page précédente"
            @click="currentPage--"
          >
            <font-awesome-icon icon="fa-solid fa-chevron-left" />
          </button>
          <button
            v-for="page in visiblePages"
            :key="page"
            type="button"
            class="size-10 rounded-[10px] text-[14px]/[1.4] font-bold transition"
            :class="page === currentPage ? 'bg-af-chocolat text-white' : 'border border-af-bordure bg-white hover:border-af-chocolat'"
            :aria-current="page === currentPage ? 'page' : undefined"
            @click="currentPage = page"
          >
            {{ page }}
          </button>
          <button
            type="button"
            :disabled="currentPage === totalPages"
            class="grid size-10 place-items-center rounded-[10px] border border-af-bordure bg-white transition hover:border-af-chocolat disabled:opacity-40"
            aria-label="Page suivante"
            @click="currentPage++"
          >
            <font-awesome-icon icon="fa-solid fa-chevron-right" />
          </button>
        </nav>
      </template>

      <!-- Deux vides distincts : « rien ne correspond » n'est pas « aucun
           projet n'est publié », et la sortie proposée n'est pas la même. -->
      <div v-else class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
        <font-awesome-icon icon="fa-solid fa-folder-open" class="text-4xl text-af-atone-2" />
        <p class="mt-4 text-[16px]/[1.4] font-bold">
          {{ statistics.total > 0 ? 'Aucun projet ne correspond à vos critères' : 'Aucun projet publié pour le moment' }}
        </p>
        <p class="mx-auto mt-2 max-w-md text-[14px]/[1.4] text-af-corps">
          {{ statistics.total > 0
            ? 'Essayez de modifier vos critères de recherche ou explorez d’autres filtres.'
            : 'Soyez le premier à soumettre le vôtre.' }}
        </p>
        <AfricansBouton
          v-if="statistics.total > 0"
          class="mt-6"
          variante="secondaire"
          icone="fa-solid fa-rotate-left"
          @click="resetFilters"
        >
          Réinitialiser les filtres
        </AfricansBouton>
        <AfricansBouton v-else class="mt-6" icone="fa-solid fa-paper-plane" vers="/soumettre-projet">
          Soumettre mon projet
        </AfricansBouton>
      </div>
    </div>

    <template #rail>
      <AfricansPanneau titre="Filtres" icone="fa-solid fa-sliders" action-libelle="Réinitialiser" @action="resetFilters">
        <div class="flex flex-col gap-4">
          <AfricansChamp v-model="filtres.pays" libelle="Territoire" type="select">
            <option v-for="pays in PAYS_PROJETS" :key="pays.value" :value="pays.value">{{ pays.label }}</option>
          </AfricansChamp>

          <AfricansChamp v-model="filtres.budgetMax" libelle="Budget maximum" type="select">
            <option v-for="budget in BUDGETS" :key="budget.value" :value="budget.value">{{ budget.label }}</option>
          </AfricansChamp>

          <AfricansChamp v-model="filtres.duree" libelle="Durée du projet" type="select">
            <option v-for="duree in DUREES" :key="duree.value" :value="duree.value">{{ duree.label }}</option>
          </AfricansChamp>

          <AfricansChamp v-model="filtres.sortBy" libelle="Trier par" type="select">
            <option v-for="option in OPTIONS_TRI" :key="option.value" :value="option.value">{{ option.label }}</option>
          </AfricansChamp>
        </div>
      </AfricansPanneau>

      <!-- Les quatre compteurs du bandeau d'origine. Ils viennent de l'API,
           pas d'une constante : à zéro, ils disent qu'il n'y a rien en base. -->
      <AfricansPanneau titre="Statistiques" icone="fa-solid fa-chart-line">
        <dl class="flex flex-col">
          <div
            v-for="(stat, i) in statistiquesAffichees"
            :key="stat.libelle"
            class="flex items-baseline justify-between gap-4 py-3"
            :class="i > 0 && 'border-t border-af-bordure'"
          >
            <dt class="text-[14px]/[1.4] font-bold">{{ stat.libelle }}</dt>
            <dd class="text-[14px]/[1.4] font-bold text-af-chocolat">{{ stat.valeur }}</dd>
          </div>
          <div class="flex items-baseline justify-between gap-4 border-t border-af-bordure pt-3">
            <dt class="text-[14px]/[1.4] font-bold">Résultats filtrés</dt>
            <dd class="text-[14px]/[1.4] font-bold text-af-vert">{{ total }}</dd>
          </div>
        </dl>
      </AfricansPanneau>

      <AfricansPanneau titre="Vous avez un projet ?" icone="fa-solid fa-lightbulb">
        <p class="mb-4 text-[14px]/[1.4] text-af-corps">
          Soumettez-le et bénéficiez du soutien de notre communauté d'investisseurs et de partenaires africains.
        </p>
        <AfricansBouton pleine-largeur icone="fa-solid fa-paper-plane" vers="/soumettre-projet">
          Soumettre mon projet
        </AfricansBouton>
      </AfricansPanneau>
    </template>
  </NuxtLayout>
</template>

<script setup lang="ts">
import {
  useProjets,
  convertirFiltresPageVersAPI,
  PAYS_PROJETS,
  BUDGETS,
  DUREES,
  OPTIONS_TRI,
  type ProjetAPI,
  type ProjetStatistiquesAPI,
  type FiltresProjetPage,
} from '~/composables/useProjets'

/**
 * Financer un projet, porté sur le gabarit de la refonte.
 *
 * Les données et les filtres ne bougent pas : mêmes endpoints, mêmes
 * critères (territoire, budget, durée, tri), même pagination serveur. Trois
 * déplacements :
 *   - filtres et statistiques passent dans le rail, qui remplace À LA FOIS
 *     la colonne de gauche et son tiroir mobile. Le gabarit empile déjà le
 *     rail sous le contenu en dessous de 64rem ;
 *   - l'appel à soumettre un projet quitte le pied de page pour la barre de
 *     contexte, où il est visible sans défiler ;
 *   - AOS disparaît : il n'animait que l'apparition des cartes.
 *
 * Les quatre compteurs du bandeau (total, validés, en cours, terminés)
 * viennent de `GET /api/projets/statistiques` : ils affichent 0 parce qu'il
 * n'y a aucun projet en base, pas parce qu'ils sont décoratifs.
 */
definePageMeta({ layout: false })

useHead({
  title: 'Financer un projet | AfricanS',
  meta: [
    {
      name: 'description',
      content: 'Découvrez des projets de développement innovants en Afrique et contribuez à leur financement.',
    },
  ],
})

const { listerProjets, obtenirStatistiques, chargement } = useProjets()

// State
const projets = ref<ProjetAPI[]>([])
const loading = computed(() => chargement.value)
const total = ref(0)
const totalPages = ref(1)
const currentPage = ref(1)
const itemsPerPage = 12

const filtres = ref<FiltresProjetPage>({
  pays: '',
  budgetMax: '',
  duree: '',
  recherche: '',
  sortBy: 'dateCreation',
})

// Statistiques
const statistics = ref<ProjetStatistiquesAPI>({
  total: 0,
  valides: 0,
  en_cours: 0,
  termines: 0,
})

/** Les quatre compteurs du bandeau d'origine, tels que l'API les renvoie. */
const statistiquesAffichees = computed(() => [
  { libelle: 'Projets', valeur: statistics.value.total },
  { libelle: 'Validés', valeur: statistics.value.valides },
  { libelle: 'En cours', valeur: statistics.value.en_cours },
  { libelle: 'Terminés', valeur: statistics.value.termines },
])

// Visible pages pour la pagination
const visiblePages = computed(() => {
  const pages: number[] = []
  const tp = totalPages.value
  const current = currentPage.value

  if (tp <= 5) {
    for (let i = 1; i <= tp; i++) {
      pages.push(i)
    }
  } else {
    if (current <= 3) {
      pages.push(1, 2, 3, 4, 5)
    } else if (current >= tp - 2) {
      pages.push(tp - 4, tp - 3, tp - 2, tp - 1, tp)
    } else {
      pages.push(current - 2, current - 1, current, current + 1, current + 2)
    }
  }

  return pages.filter((p) => p >= 1 && p <= tp)
})

// Charger les projets depuis l'API
const chargerProjets = async () => {
  const apiParams = convertirFiltresPageVersAPI(filtres.value)
  apiParams.page = currentPage.value
  apiParams.par_page = itemsPerPage

  const result = await listerProjets(apiParams)
  if (result) {
    projets.value = result.projets
    total.value = result.total
    totalPages.value = result.total_pages
  }
}

// Methods
const resetFilters = () => {
  filtres.value = {
    pays: '',
    budgetMax: '',
    duree: '',
    recherche: '',
    sortBy: 'dateCreation',
  }
  currentPage.value = 1
}

const handleSearch = () => {
  currentPage.value = 1
  chargerProjets()
}

// Watch filters to reload
watch(filtres, () => {
  currentPage.value = 1
  chargerProjets()
}, { deep: true })

// Watch page changes
watch(currentPage, () => {
  chargerProjets()
})

// Lifecycle
onMounted(async () => {
  // Load data in parallel
  const [, stats] = await Promise.all([
    chargerProjets(),
    obtenirStatistiques(),
  ])
  if (stats) {
    statistics.value = stats
  }
})
</script>
