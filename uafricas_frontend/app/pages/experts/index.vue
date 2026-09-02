<script setup lang="ts">
import {
  useExperts,
  CATEGORIES_EXPERTISE as categories,
  type ExpertAPI,
} from '~/composables/useExperts'

/**
 * Diapertise : porté sur le gabarit de la refonte.
 *
 * Aucun critère de recherche n'est ajouté ni retiré : recherche libre, domaine,
 * zone, territoire, spécialité, situation et tri, comme avant. Ce qui change :
 *   - les filtres passent dans le rail (`ExpertsPanneauFiltres`), qui remplace
 *     à la fois la colonne de gauche et le tiroir mobile, le gabarit empile
 *     déjà le rail sous le contenu en dessous de 64rem ;
 *   - les domaines d'expertise ne sont plus tronqués à cinq avec un menu
 *     « Plus » : ils sont neuf en tout, ils tiennent en pastilles ;
 *   - le tri à trois icônes muettes devient un menu déroulant nommé. Une icône
 *     d'étoile pour « Note » se devine ; une horloge pour « Récent », non.
 */
definePageMeta({ layout: false })

useHead({
  title: 'Diapertise : Le répertoire des expertises africaines | AfricanS',
  meta: [
    {
      name: 'description',
      content: 'Trouvez les meilleurs experts africains et afro-descendants dans tous les domaines',
    },
  ],
})

const { listerExperts, listerSpecialites, chargement } = useExperts()

// ─── État ─────────────────────────────────────────────────────────────────

const experts = ref<ExpertAPI[]>([])
const totalExperts = ref(0)
const totalPages = ref(1)
const searchTerm = ref('')
const categorySelected = ref('Tout')
const selectedCountry = ref('')
/** Zone géographique du territoire : filtre aussi la liste des experts.
 *  Défaut « tout » : non restrictif. */
const selectedZone = ref<'afrique' | 'hors_afrique' | 'tout'>('tout')
const selectedProfile = ref('')
/** Spécialité choisie dans les filtres ('' = toutes). */
const selectedSpecialty = ref('')
/** Spécialités réellement déclarées par les experts (chargées au montage). */
const specialites = ref<string[]>([])
const sortOrder = ref<'recent' | 'experience' | 'rating'>('recent')
const currentPage = ref(1)
const parPage = 12
const filtreSurMesureOuvert = ref(false)
const decouverteOuverte = ref(false)

const TRIS = [
  { id: 'recent' as const, libelle: 'Les plus récents' },
  { id: 'experience' as const, libelle: 'Les plus expérimentés' },
  { id: 'rating' as const, libelle: 'Les mieux notés' }]

// ─── Chargement (pagination côté serveur) ─────────────────────────────────

const chargerExperts = async () => {
  const result = await listerExperts({
    recherche: searchTerm.value || undefined,
    domaine: categorySelected.value !== 'Tout' ? categorySelected.value : undefined,
    pays: selectedCountry.value || undefined,
    zone: selectedZone.value,
    situation: selectedProfile.value && selectedProfile.value !== 'tous'
      ? selectedProfile.value
      : undefined,
    specialite: selectedSpecialty.value || undefined,
    tri: sortOrder.value,
    page: currentPage.value,
    par_page: parPage,
  })

  if (result) {
    experts.value = result.experts
    totalExperts.value = result.total
    totalPages.value = result.total_pages
  }
}

// ─── Pagination ───────────────────────────────────────────────────────────

const visiblePages = computed(() => {
  const pages: number[] = []
  const total = totalPages.value
  const current = currentPage.value

  if (total <= 5) {
    for (let i = 1; i <= total; i++) pages.push(i)
  }
  else if (current <= 3) pages.push(1, 2, 3, 4, 5)
  else if (current >= total - 2) pages.push(total - 4, total - 3, total - 2, total - 1, total)
  else pages.push(current - 2, current - 1, current, current + 1, current + 2)

  return pages.filter(p => p >= 1 && p <= total)
})

// ─── Actions ──────────────────────────────────────────────────────────────

const filterByProfile = (profileId: string) => {
  selectedProfile.value = profileId
}

const resetFilters = () => {
  categorySelected.value = 'Tout'
  selectedCountry.value = ''
  selectedZone.value = 'tout'
  selectedProfile.value = ''
  selectedSpecialty.value = ''
  searchTerm.value = ''
  currentPage.value = 1
}

const handleSearch = () => {
  currentPage.value = 1
  chargerExperts()
}

/** Critères choisis dans la modale « sur mesure ». */
const appliquerFiltreSurMesure = (filtres: {
  domaine: string
  pays: string
  situation: string
  recherche: string
}) => {
  categorySelected.value = filtres.domaine || 'Tout'
  selectedCountry.value = filtres.pays
  selectedProfile.value = filtres.situation
  searchTerm.value = filtres.recherche
  currentPage.value = 1
  filtreSurMesureOuvert.value = false
  chargerExperts()
}

const contactExpert = (expert: ExpertAPI) => {
  if (expert.email) {
    window.location.href = `mailto:${expert.email}`
  }
}

// Recharger quand les filtres changent (retour page 1 + appel API)
watch([categorySelected, selectedCountry, selectedZone, selectedProfile, selectedSpecialty, sortOrder], () => {
  currentPage.value = 1
  chargerExperts()
})

watch(currentPage, chargerExperts)

onMounted(async () => {
  chargerExperts()
  specialites.value = await listerSpecialites()
})
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Diapertise"
        sous-titre="Le répertoire des expertises africaines et afrodescendantes"
        image="/images/apporter-expertise.png"
        aide="C'est quoi Diapertise ?"
        @aide="decouverteOuverte = true"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane :segments="[{ libelle: 'Diapertise' }]">
        <template #action>
          <AfricansBouton icone="fa-solid fa-user-plus" vers="/devenir-expert">
            Faire connaître mon expertise
          </AfricansBouton>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="flex flex-col gap-6">
      <!-- Recherche libre : validée à l'entrée, comme avant (elle n'est pas dans
           le watch des filtres : une frappe par requête serait de trop). -->
      <form class="flex flex-wrap gap-3" @submit.prevent="handleSearch">
        <label class="relative min-w-0 flex-1">
          <span class="sr-only">Rechercher un(e) expert(e)</span>
          <font-awesome-icon
            icon="fa-solid fa-magnifying-glass"
            class="pointer-events-none absolute top-1/2 left-4 -translate-y-1/2 text-af-atone-2"
          />
          <input
            v-model="searchTerm"
            type="search"
            placeholder="Nom, domaine, mot-clé…"
            class="h-11 w-full rounded-[10px] border border-af-bordure bg-white pr-4 pl-11 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
          />
        </label>
        <AfricansBouton type="submit" icone="fa-solid fa-magnifying-glass">Rechercher</AfricansBouton>
        <AfricansBouton variante="secondaire" icone="fa-solid fa-wand-magic-sparkles" @click="filtreSurMesureOuvert = true">
          Sur mesure
        </AfricansBouton>
      </form>

      <!-- Domaines d'expertise -->
      <div class="flex flex-wrap items-center gap-2">
        <button
          v-for="category in categories"
          :key="category"
          type="button"
          class="rounded-full px-3 py-1.5 text-[12px]/[1.4] font-bold transition"
          :class="categorySelected === category ? 'bg-af-chocolat text-white' : 'bg-af-fond text-af-corps hover:bg-af-bordure'"
          :aria-pressed="categorySelected === category"
          @click="categorySelected = category"
        >
          {{ category }}
        </button>
      </div>

      <div class="flex flex-wrap items-center justify-between gap-3">
        <p class="text-[14px]/[1.4] text-af-atone">
          {{ totalExperts }} expert{{ totalExperts > 1 ? 's' : '' }}
        </p>
        <label class="flex items-center gap-2">
          <span class="text-[14px]/[1.4] text-af-corps">Trier par</span>
          <select
            v-model="sortOrder"
            class="h-10 rounded-[10px] border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:outline-2 focus:outline-af-chocolat"
          >
            <option v-for="tri in TRIS" :key="tri.id" :value="tri.id">{{ tri.libelle }}</option>
          </select>
        </label>
      </div>

      <div v-if="chargement" class="grid gap-5 sm:grid-cols-2">
        <div v-for="n in 4" :key="n" class="h-64 animate-pulse rounded-[10px] bg-af-bordure" />
      </div>

      <template v-else-if="experts.length > 0">
        <div class="grid gap-5 sm:grid-cols-2">
          <ExpertsExpertCard
            v-for="expert in experts"
            :key="expert.id"
            :expert="expert"
            @contact="contactExpert"
          />
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
            v-for="p in visiblePages"
            :key="p"
            type="button"
            class="size-10 rounded-[10px] text-[14px]/[1.4] font-bold transition"
            :class="p === currentPage ? 'bg-af-chocolat text-white' : 'border border-af-bordure bg-white hover:border-af-chocolat'"
            :aria-current="p === currentPage ? 'page' : undefined"
            @click="currentPage = p"
          >
            {{ p }}
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

      <div v-else class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
        <font-awesome-icon icon="fa-solid fa-user-slash" class="text-4xl text-af-atone-2" />
        <p class="mt-4 text-[16px]/[1.4] font-bold">Aucun(e) expert(e) trouvé(e)</p>
        <p class="mx-auto mt-2 max-w-md text-[14px]/[1.4] text-af-corps">
          Essayez de modifier vos critères de recherche ou explorez d'autres domaines d'expertise.
        </p>
        <AfricansBouton class="mt-6" variante="secondaire" icone="fa-solid fa-rotate-left" @click="resetFilters">
          Réinitialiser les filtres
        </AfricansBouton>
      </div>
    </div>

    <template #rail>
      <ExpertsPanneauFiltres
        v-model:selected-country="selectedCountry"
        v-model:selected-specialty="selectedSpecialty"
        v-model:zone="selectedZone"
        :selected-profile="selectedProfile"
        :specialites="specialites"
        @filtrer-profil="filterByProfile"
        @reset="resetFilters"
      />
    </template>

    <ExpertsExpertFiltreSurMesureModal
      :is-open="filtreSurMesureOuvert"
      @close="filtreSurMesureOuvert = false"
      @apply="appliquerFiltreSurMesure"
    />

    <ExpertsDecouverteModale v-model="decouverteOuverte" />
  </NuxtLayout>
</template>
