<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Africantives"
        sous-titre="Valoriser une initiative mettant en avant l'Afrique, les afrodescendants ou la diaspora africaine"
        aide="C'est quoi Africantives ?"
        @aide="presentationOuverte = true"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane :segments="[{ libelle: 'Africantives' }]">
        <template #centre>
          <p class="text-base font-bold text-af-encre">
            {{ totalAfricantives }} initiative{{ totalAfricantives > 1 ? 's' : '' }}
          </p>
        </template>
        <template #action>
          <AfricansBouton icone="fa-solid fa-plus" @click="handlePublish">
            Publier une initiative
          </AfricansBouton>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="flex flex-col gap-6">
      <!-- Recherche libre : validée à l'entrée, mais aussi débattue à la frappe
           (300 ms) par le watch : le bouton reste pour qui n'attend pas. -->
      <form class="flex flex-wrap gap-3" @submit.prevent="handleSearch">
        <label class="relative min-w-0 flex-1">
          <span class="sr-only">Rechercher une initiative</span>
          <font-awesome-icon
            icon="fa-solid fa-magnifying-glass"
            class="pointer-events-none absolute top-1/2 left-4 -translate-y-1/2 text-af-atone-2"
          />
          <input
            v-model="filtres.recherche"
            type="search"
            placeholder="Titre, mot-clé…"
            class="h-11 w-full rounded-[10px] border border-af-bordure bg-white pr-4 pl-11 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
          />
        </label>
        <AfricansBouton type="submit" icone="fa-solid fa-magnifying-glass">Rechercher</AfricansBouton>
      </form>

      <div ref="zoneListe" class="scroll-mt-24">
        <div v-if="chargement" class="grid gap-5 sm:grid-cols-2">
          <div v-for="n in 4" :key="n" class="h-72 animate-pulse rounded-[10px] bg-af-bordure" />
        </div>

        <div v-else-if="erreur" class="rounded-[10px] border border-af-live/30 bg-white p-12 text-center">
          <font-awesome-icon icon="fa-solid fa-circle-exclamation" class="text-4xl text-af-live" />
          <p class="mt-4 text-[16px]/[1.4] font-bold">Erreur de chargement</p>
          <p class="mx-auto mt-2 max-w-md text-[14px]/[1.4] text-af-corps">{{ erreur }}</p>
          <AfricansBouton class="mt-6" variante="secondaire" icone="fa-solid fa-rotate-right" @click="chargerAfricantives">
            Réessayer
          </AfricansBouton>
        </div>

        <div v-else-if="africantives.length === 0" class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
          <font-awesome-icon icon="fa-solid fa-lightbulb" class="text-4xl text-af-atone-2" />
          <p class="mt-4 text-[16px]/[1.4] font-bold">Aucune initiative trouvée</p>
          <p class="mx-auto mt-2 max-w-md text-[14px]/[1.4] text-af-corps">
            Essayez de modifier vos critères de recherche, ou publiez la vôtre.
          </p>
          <AfricansBouton class="mt-6" variante="secondaire" icone="fa-solid fa-rotate-left" @click="resetFilters">
            Réinitialiser les filtres
          </AfricansBouton>
        </div>

        <div v-else class="flex flex-col gap-6">
          <div class="grid gap-5 sm:grid-cols-2">
            <AfricantivesAfricantiveCard
              v-for="initiative in africantives"
              :key="initiative.id"
              :africantive="initiative"
            />
          </div>

          <nav v-if="totalPages > 1" class="flex items-center justify-center gap-2">
            <button
              type="button"
              :disabled="currentPage === 1"
              class="grid size-10 place-items-center rounded-[10px] border border-af-bordure bg-white transition hover:border-af-chocolat disabled:opacity-40"
              aria-label="Page précédente"
              @click="goToPage(currentPage - 1)"
            >
              <font-awesome-icon icon="fa-solid fa-chevron-left" />
            </button>
            <template v-for="(p, i) in visiblePages" :key="`${p}-${i}`">
              <span v-if="p === '...'" class="px-1 text-af-atone-2">…</span>
              <button
                v-else
                type="button"
                class="size-10 rounded-[10px] text-[14px]/[1.4] font-bold transition"
                :class="currentPage === p ? 'bg-af-chocolat text-white' : 'border border-af-bordure bg-white hover:border-af-chocolat'"
                :aria-current="currentPage === p ? 'page' : undefined"
                @click="goToPage(p as number)"
              >
                {{ p }}
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
        </div>
      </div>
    </div>

    <template #rail>
      <AfricansPanneau titre="Filtres" icone="fa-solid fa-sliders" action-libelle="Réinitialiser" @action="resetFilters">
        <div class="flex flex-col gap-4">
          <!-- Le domaine tenait vingt PASTILLES au-dessus de la grille, soit
               cinq lignes qui écrasaient les résultats. Un référentiel de
               cette taille se choisit dans une liste, pas en balayant une
               barre : il rejoint les deux autres filtres, dans le seul endroit
               de la page qui en porte. -->
          <AfricansChamp v-model="filtres.domaine" libelle="Domaine" type="select">
            <option v-for="dom in domainesFiltre" :key="dom.value" :value="dom.value">
              {{ dom.label }}
            </option>
          </AfricansChamp>

          <AfricansChamp v-model="filtres.pays" libelle="Territoire" type="select">
            <option value="">Tous les territoires</option>
            <option v-for="p in paysAfricains" :key="p" :value="p">{{ p }}</option>
          </AfricansChamp>

          <AfricansChamp v-model="filtres.tri" libelle="Trier par" type="select">
            <option value="recent">Plus récent</option>
            <option value="ancien">Plus ancien</option>
            <option value="titre">Titre (A-Z)</option>
          </AfricansChamp>
        </div>
      </AfricansPanneau>

      <AfricansPanneau titre="Publier" icone="fa-solid fa-lightbulb">
        <p class="mb-4 text-[14px]/[1.4] text-af-corps">
          Vous portez une initiative qui met en avant l'Afrique, les afrodescendants ou la diaspora ? Faites-la connaître.
        </p>
        <AfricansBouton pleine-largeur icone="fa-solid fa-plus" @click="handlePublish">
          Publier une initiative
        </AfricansBouton>
      </AfricansPanneau>
    </template>

    <AfricantivesDecouverteModale v-model="presentationOuverte" />

    <AfricantivesPublierInitiativeModal
      ref="publishModalRef"
      :is-open="showPublishModal"
      @close="showPublishModal = false"
      @submit="handleSubmitInitiative"
    />
  </NuxtLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import {
  useAfricantives,
  DOMAINES_AFRICANTIVES,
  type AfricantiveAPI,
  type AfricantiveFiltres,
} from '~/composables/useAfricantives'
import { useUserStore } from '~/stores/user'
import { PAYS_AFRICAINS } from '~/composables/useAfricantives'

/**
 * Africantives : porté sur le gabarit de la refonte.
 *
 * Aucun critère n'est ajouté ni retiré : recherche libre, domaine, territoire
 * et tri, comme avant. Ce qui change :
 *   - territoire et tri passent dans le rail, qui remplace À LA FOIS la colonne
 *     de filtres et le tiroir mobile : le gabarit empile déjà le rail sous le
 *     contenu en dessous de 64rem, un tiroir en plus n'apporterait rien ;
 *   - l'image de fond du bandeau, hébergée sur unsplash.com, est retirée ;
 *   - la modale « Connexion requise » écrite à la main laisse place à la
 *     redirection standard vers `/login`, qui, elle, ramène ici après connexion
 *     (`?redirect=`). L'ancienne modale envoyait vers `/login` sans retour.
 */
definePageMeta({ layout: false })

useHead({
  title: 'Africantives - Initiatives Africaines - AfricanS',
  meta: [
    {
      name: 'description',
      content: 'Découvrez les initiatives africaines qui transforment le continent. Agriculture, technologie, éducation, santé et plus encore.',
    },
  ],
})

const ITEMS_PER_PAGE = 12

const userStore = useUserStore()
const { chargement, erreur, listerAfricantives, creerAfricantive } = useAfricantives()

// State
const africantives = ref<AfricantiveAPI[]>([])
const totalAfricantives = ref(0)
const totalPages = ref(1)
const currentPage = ref(1)
const showPublishModal = ref(false)

/** Ancre de la grille, pour y ramener le visiteur au changement de page. */
const zoneListe = ref<HTMLElement | null>(null)

// Modale de présentation « C'est quoi Africantives ? »
const presentationOuverte = ref(false)
const publishModalRef = ref<{ setLoading: (v: boolean) => void; setError: (msg: string) => void; setSuccess: () => void } | null>(null)

const filtres = ref({
  domaine: '',
  pays: '',
  recherche: '',
  tri: 'recent',
})

// Domaines pour les pastilles de filtre
const domainesFiltre = DOMAINES_AFRICANTIVES
const paysAfricains = PAYS_AFRICAINS

// Debounce timer pour la recherche
let rechercheTimer: ReturnType<typeof setTimeout> | null = null

// Construire les filtres API
const buildApiFiltres = (): AfricantiveFiltres => {
  const f: AfricantiveFiltres = {
    page: currentPage.value,
    par_page: ITEMS_PER_PAGE,
  }
  if (filtres.value.recherche.trim()) f.recherche = filtres.value.recherche.trim()
  if (filtres.value.domaine) f.domaine = filtres.value.domaine
  if (filtres.value.pays) f.pays = filtres.value.pays
  if (filtres.value.tri !== 'recent') f.tri = filtres.value.tri
  return f
}

// Charger les initiatives
const chargerAfricantives = async () => {
  const resultat = await listerAfricantives(buildApiFiltres())
  if (resultat) {
    africantives.value = resultat.africantives
    totalAfricantives.value = resultat.total
    totalPages.value = resultat.total_pages
  }
}

// Computed
const visiblePages = computed(() => {
  const pages: (number | string)[] = []
  const total = totalPages.value
  const current = currentPage.value

  if (total <= 7) {
    for (let i = 1; i <= total; i++) pages.push(i)
  } else {
    pages.push(1)
    if (current > 3) pages.push('...')
    const start = Math.max(2, current - 1)
    const end = Math.min(total - 1, current + 1)
    for (let i = start; i <= end; i++) pages.push(i)
    if (current < total - 2) pages.push('...')
    pages.push(total)
  }

  return pages
})

// Watchers
watch(
  () => ({
    domaine: filtres.value.domaine,
    pays: filtres.value.pays,
    tri: filtres.value.tri,
  }),
  () => {
    currentPage.value = 1
    chargerAfricantives()
  },
  { deep: true },
)

// Debounce la recherche textuelle (300ms)
watch(
  () => filtres.value.recherche,
  () => {
    if (rechercheTimer) clearTimeout(rechercheTimer)
    rechercheTimer = setTimeout(() => {
      currentPage.value = 1
      chargerAfricantives()
    }, 300)
  },
)

// Methods
const handleSearch = () => {
  if (rechercheTimer) clearTimeout(rechercheTimer)
  currentPage.value = 1
  chargerAfricantives()
}

const { redirigerVersConnexion } = useAuth()

const handlePublish = () => {
  if (!userStore.isAuthenticated) {
    redirigerVersConnexion()
    return
  }
  showPublishModal.value = true
}

const handleSubmitInitiative = async (data: {
  titre: string
  description: string
  domaine: string
  domaine_autre: string
  pays: string
  ville: string
  site_web_url: string
  lien_reseau_social: string
  contact1_courriel: string
  contact1_telephone: string
  contact1_adresse: string
  contact2_courriel: string
  contact2_telephone: string
  contact2_adresse: string
  couvertureFile: File | null
}) => {
  publishModalRef.value?.setLoading(true)

  const resultat = await creerAfricantive(
    {
      titre: data.titre,
      description: data.description,
      domaine: data.domaine || undefined,
      domaine_autre: data.domaine_autre || undefined,
      pays: data.pays || undefined,
      ville: data.ville || undefined,
      site_web_url: data.site_web_url || undefined,
      lien_reseau_social: data.lien_reseau_social || undefined,
      contact1_courriel: data.contact1_courriel || undefined,
      contact1_telephone: data.contact1_telephone || undefined,
      contact1_adresse: data.contact1_adresse || undefined,
      contact2_courriel: data.contact2_courriel || undefined,
      contact2_telephone: data.contact2_telephone || undefined,
      contact2_adresse: data.contact2_adresse || undefined,
    },
    data.couvertureFile,
  )

  if (resultat) {
    publishModalRef.value?.setSuccess()
    // Recharger la liste pour afficher la nouvelle initiative
    await chargerAfricantives()
  } else {
    publishModalRef.value?.setError(erreur.value || 'Une erreur est survenue lors de la publication.')
  }
}

const resetFilters = () => {
  filtres.value = {
    domaine: '',
    pays: '',
    recherche: '',
    tri: 'recent',
  }
  currentPage.value = 1
}

const goToPage = (page: number) => {
  if (page >= 1 && page <= totalPages.value) {
    currentPage.value = page
    chargerAfricantives()
    // Remonter en tête de liste : sans cela, changer de page laisse le
    // visiteur au bas d'une grille qu'il n'a pas encore vue. La valeur en dur
    // de 400 px était calée sur la hauteur de l'ancien bandeau.
    zoneListe.value?.scrollIntoView({ behavior: 'smooth', block: 'start' })
  }
}

// Lifecycle
onMounted(async () => {
  await chargerAfricantives()
})
</script>
