<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Formations disponibles"
        sous-titre="MOOC, CLOM, ateliers et concertations pour développer vos compétences"
        image="/images/education.png"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[
          { libelle: 'Mindshiftlab', vers: '/mindshiftlab' },
          { libelle: 'Muniversa', vers: '/universite' },
          { libelle: 'Formations' },
        ]"
      >
        <template #centre>
          <p class="text-base font-bold text-af-encre">
            {{ formationsTriees.length }} formation{{ formationsTriees.length > 1 ? 's' : '' }}
          </p>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="flex flex-col gap-6">
      <div class="flex flex-wrap items-center justify-between gap-3">
        <label class="relative min-w-0 flex-1">
          <span class="sr-only">Rechercher une formation</span>
          <font-awesome-icon
            icon="fa-solid fa-magnifying-glass"
            class="pointer-events-none absolute top-1/2 left-4 -translate-y-1/2 text-af-atone-2"
          />
          <input
            v-model="recherche"
            type="search"
            placeholder="Titre, intervenant, mot-clé…"
            class="h-11 w-full rounded-[10px] border border-af-bordure bg-white pr-4 pl-11 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
          />
        </label>
        <label class="flex items-center gap-2">
          <span class="text-[14px]/[1.4] text-af-corps">Trier par</span>
          <select
            v-model="triSelectionne"
            class="h-11 rounded-[10px] border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:outline-2 focus:outline-af-chocolat"
          >
            <option value="date">Date</option>
            <option value="titre">Titre</option>
          </select>
        </label>
      </div>

      <div
        v-if="erreur"
        class="flex items-center gap-2 rounded-[10px] border border-af-live/30 bg-af-live/5 px-4 py-3 text-[14px]/[1.4] text-af-live"
      >
        <font-awesome-icon icon="fa-solid fa-circle-exclamation" class="shrink-0" />
        <span class="min-w-0 flex-1">{{ erreur }}</span>
        <button type="button" class="shrink-0 font-bold underline" @click="chargerFormations">Réessayer</button>
      </div>

      <div v-if="chargement" class="grid gap-5 sm:grid-cols-2">
        <div v-for="n in 4" :key="n" class="h-72 animate-pulse rounded-[10px] bg-af-bordure" />
      </div>

      <template v-else-if="formationsTriees.length > 0">
        <div class="grid gap-5 sm:grid-cols-2">
          <UniversiteInudaFormationCard
            v-for="formation in formationsTriees"
            :key="formation.id"
            :formation="formation"
            @click="voirDetail"
            @inscrire="ouvrirInscription"
          />
        </div>

        <nav v-if="totalPages > 1" class="flex items-center justify-center gap-2">
          <button
            v-for="p in totalPages"
            :key="p"
            type="button"
            class="size-10 rounded-[10px] text-[14px]/[1.4] font-bold transition"
            :class="p === pageActuelle ? 'bg-af-chocolat text-white' : 'border border-af-bordure bg-white hover:border-af-chocolat'"
            :aria-current="p === pageActuelle ? 'page' : undefined"
            @click="allerPage(p)"
          >
            {{ p }}
          </button>
        </nav>
      </template>

      <!-- Deux vides distincts : « rien ne correspond » n'est pas « rien n'est
           programmé », et la sortie proposée n'est pas la même. -->
      <div v-else class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
        <font-awesome-icon icon="fa-solid fa-graduation-cap" class="text-4xl text-af-atone-2" />
        <p class="mt-4 text-[16px]/[1.4] font-bold">
          {{ formations.length > 0 ? 'Aucune formation ne correspond à vos critères' : 'Aucune formation programmée pour le moment' }}
        </p>
        <AfricansBouton
          v-if="formations.length > 0"
          class="mt-6"
          variante="secondaire"
          icone="fa-solid fa-rotate-left"
          @click="reinitialiserFiltres"
        >
          Réinitialiser les filtres
        </AfricansBouton>
      </div>
    </div>

    <template #rail>
      <AfricansPanneau titre="Filtres" icone="fa-solid fa-sliders" action-libelle="Réinitialiser" @action="reinitialiserFiltres">
        <UniversiteInudaFiltresFormations @filtres-changes="appliquerFiltres" />
      </AfricansPanneau>

      <AfricansPanneau titre="Aussi dans Muniversa" icone="fa-solid fa-graduation-cap">
        <ul class="flex flex-col gap-1">
          <li v-for="lien in AUTRES_INUDA" :key="lien.to">
            <NuxtLink
              :to="lien.to"
              class="flex items-center gap-3 rounded-lg px-3 py-2.5 text-[14px]/[1.4] font-bold text-af-corps transition hover:bg-af-chocolat/[0.07] hover:text-af-chocolat"
            >
              <font-awesome-icon :icon="lien.icone" class="size-5 shrink-0" />
              {{ lien.libelle }}
            </NuxtLink>
          </li>
        </ul>
      </AfricansPanneau>
    </template>

    <!-- Confirmation d'inscription -->
    <AfricansModale
      :model-value="formationSelectionnee !== null"
      titre="S'inscrire à la formation"
      icone="fa-solid fa-graduation-cap"
      @update:model-value="formationSelectionnee = null"
    >
      <p v-if="formationSelectionnee" class="text-[14px]/[1.5] text-af-corps">
        Vous souhaitez vous inscrire à
        <strong class="font-bold text-af-encre">{{ formationSelectionnee.titre }}</strong>.
      </p>
      <p v-if="messageInscription" class="mt-3 text-[14px]/[1.4]" :class="inscriptionReussie ? 'text-af-vert' : 'text-af-live'">
        {{ messageInscription }}
      </p>

      <template #actions>
        <AfricansBouton variante="secondaire" @click="formationSelectionnee = null">Annuler</AfricansBouton>
        <AfricansBouton :desactive="inscriptionEnCours" :tourne="inscriptionEnCours" @click="confirmerInscription">
          {{ inscriptionEnCours ? 'Inscription…' : 'Confirmer' }}
        </AfricansBouton>
      </template>
    </AfricansModale>
  </NuxtLayout>
</template>

<script setup lang="ts">
import {
  useFormations,
  type FormationAPI,
  type FormationFiltres,
  getTypeLabel,
  getTypeClasses,
  getStatutLabel,
  getStatutColor,
  getActionLabel,
  peutSInscrire,
  formatDateCourt,
} from '~/composables/useFormations'

/**
 * Formations de Muniversa, portées sur le gabarit de la refonte.
 *
 * La bascule grille / liste disparaît : les deux vues montraient exactement
 * les mêmes informations, et la seconde réimplémentait à la main, en cinquante
 * lignes, ce que `FormationCard` fait déjà. Deux rendus d'une même carte
 * finissent par diverger.
 *
 * Le tri et les filtres sont inchangés, et restent appliqués côté client comme
 * avant : l'endpoint ne les porte pas.
 */
definePageMeta({ layout: false })

/** Les autres portes de l'univers, reprises de `MODULES_AFRICANS`. */
const AUTRES_INUDA = [
  { libelle: 'Facultés partenaires', to: '/universite/facultes', icone: 'fa-solid fa-building-columns' },
  { libelle: 'Mon espace', to: '/universite/mon-espace', icone: 'fa-solid fa-user-graduate' },
  { libelle: 'Africalive', to: '/evenements/liste', icone: 'fa-solid fa-calendar-days' },
]

useHead({ title: 'Formations | Muniversa' })

const { chargement, erreur, listerFormations, inscrireFormation } = useFormations()

const formations = ref<FormationAPI[]>([])
const recherche = ref('')
const filtresActifs = ref<{ types?: string[]; statuts?: string[] }>({})
const triSelectionne = ref('date')
const formationSelectionnee = ref<FormationAPI | null>(null)
const pageActuelle = ref(1)
const totalPages = ref(1)
const inscriptionEnCours = ref(false)
const inscriptionReussie = ref(false)
const messageInscription = ref('')

// Formations triees cote client
const formationsTriees = computed(() => {
  let resultats = [...formations.value]

  // Filtrer par type (cote client si plusieurs types selectionnes)
  if (filtresActifs.value.types?.length) {
    resultats = resultats.filter(f => f.type && filtresActifs.value.types!.includes(f.type))
  }

  // Filtrer par statut
  if (filtresActifs.value.statuts?.length) {
    resultats = resultats.filter(f => filtresActifs.value.statuts!.includes(f.statut))
  }

  // Appliquer le tri
  switch (triSelectionne.value) {
    case 'titre':
      resultats.sort((a, b) => a.titre.localeCompare(b.titre))
      break
    case 'date':
    default:
      resultats.sort((a, b) => new Date(a.date_heure_debut).getTime() - new Date(b.date_heure_debut).getTime())
  }

  return resultats
})

const voirDetail = (formation: FormationAPI) => {
  navigateTo(`/universite/formations/${formation.id}`)
}

const ouvrirInscription = (formation: FormationAPI) => {
  formationSelectionnee.value = formation
  messageInscription.value = ''
  inscriptionReussie.value = false
}

const confirmerInscription = async () => {
  if (!formationSelectionnee.value) return
  inscriptionEnCours.value = true
  messageInscription.value = ''

  const succes = await inscrireFormation(formationSelectionnee.value.id)

  if (succes) {
    inscriptionReussie.value = true
    messageInscription.value = 'Inscription réussie !'
    // Recharger pour mettre a jour le compteur
    await chargerFormations()
  } else {
    messageInscription.value = 'Erreur lors de l\'inscription. Vérifiez que vous êtes connecté.'
  }

  inscriptionEnCours.value = false
}

const appliquerFiltres = (nouveauxFiltres: { types: string[]; statuts: string[]; gratuit: boolean | null }) => {
  filtresActifs.value = {
    types: nouveauxFiltres.types,
    statuts: nouveauxFiltres.statuts,
  }
}

const reinitialiserFiltres = () => {
  recherche.value = ''
  filtresActifs.value = {}
  chargerFormations()
}

const allerPage = (page: number) => {
  pageActuelle.value = page
  chargerFormations()
}

const chargerFormations = async () => {
  const filtres: FormationFiltres = {
    page: pageActuelle.value,
    par_page: 50,
  }
  if (recherche.value) filtres.recherche = recherche.value

  const data = await listerFormations(filtres)
  if (data) {
    formations.value = data.formations
    totalPages.value = data.total_pages
  }
}

// Recherche reactive avec debounce
let debounceTimer: ReturnType<typeof setTimeout>
watch(recherche, () => {
  clearTimeout(debounceTimer)
  debounceTimer = setTimeout(() => {
    pageActuelle.value = 1
    chargerFormations()
  }, 300)
})

onMounted(() => {
  chargerFormations()
})
</script>
