<script setup lang="ts">
import World from '@svg-maps/world'
import { PAYS_AFRICAINS_ISO2 } from '~/constants/afripulsePaysAutorises'
import {
  useEvenements,
  ANNEES,
  TYPES_EVENEMENT,
  PAYS_AFRICAINS,
  type EvenementAPI,
  type EvenementFiltres,
} from '~/composables/useEvenements'

/**
 * Africalive : porté sur le gabarit de la refonte.
 *
 * Les deux modes (grille et carte) et tous les filtres sont conservés, ainsi
 * que la règle qui les lie : la carte d'Afrique n'a de sens qu'en zone
 * « Afrique », et quitter cette zone rebascule en grille.
 *
 * Les filtres passent dans le rail, écrits sur les jetons communs : la barre
 * `EvenementFilters` était horizontale, bordée de `custom-chocolat`, et portait
 * un second bouton « Proposer un événement » que le fil d'Ariane porte déjà.
 *
 * Trois tables recopiées disparaissent au profit de `utils/carteAfrique.ts`,
 * qui les portait déjà pour Retrouv'Amis et Afripulse : `NOMS_PAYS_FR`,
 * `PETITES_ILES` et `AFRICA_VIEWBOX`. Trois copies d'une même liste de 54
 * territoires divergent au premier ajout.
 */
definePageMeta({ layout: false })

useHead({
  title: 'Africalive : Événements & ateliers | AfricanS',
  meta: [
    {
      name: 'description',
      content: "Webinaires, conférences, forums et ateliers de l'Afrique et de ses diasporas.",
    }],
})

const { listerEvenements, creerEvenement, chargement, erreur } = useEvenements()

// ─── État ─────────────────────────────────────────────────────────────────

const showModal = ref(false)
const decouverteOuverte = ref(false)
const viewMode = ref<'grille' | 'carte'>('grille')
const anneeSelected = ref(new Date().getFullYear().toString())
const filtreType = ref('')
const filtrePays = ref('')
// 'tout' = aucun filtre de zone (valeur par défaut, non transmise à l'API).
const filtreZone = ref<'afrique' | 'hors_afrique' | 'tout'>('tout')
const evenements = ref<EvenementAPI[]>([])

/** Zones proposées. « Tout » d'abord : c'est le choix le plus large. */
const ZONES = [
  { valeur: 'tout' as const, libelle: 'Mondial' },
  { valeur: 'afrique' as const, libelle: 'Afrique' },
  { valeur: 'hors_afrique' as const, libelle: 'Hors Afrique' }]

const chargerEvenements = async () => {
  const filtres: EvenementFiltres = {
    annee: parseInt(anneeSelected.value),
    format: filtreType.value || undefined,
    pays: filtrePays.value || undefined,
    zone: filtreZone.value,
    par_page: 50,
  }
  const data = await listerEvenements(filtres)
  evenements.value = data?.evenements ?? []
}

// Changer de zone : la carte d'Afrique n'a de sens que pour la zone
// « Afrique », on force donc la grille dès qu'on la quitte (sinon la carte
// resterait affichée alors que son sélecteur a disparu). Le territoire n'est
// réinitialisé qu'en « Hors Afrique », la liste proposée étant exclusivement
// africaine ; en « Tout » le territoire choisi reste valide.
watch(filtreZone, (zone) => {
  if (zone !== 'afrique') viewMode.value = 'grille'
  if (zone === 'hors_afrique') filtrePays.value = ''
})

watch([anneeSelected, filtreType, filtrePays, filtreZone], chargerEvenements)

onMounted(async () => {
  await chargerEvenements()
  if (viewMode.value === 'carte') {
    await nextTick()
    calculerTransformsIles()
  }
})

const handleSubmit = async (data: any) => {
  const result = await creerEvenement(
    {
      titre: data.titre,
      description: data.description,
      type: data.type,
      thematique: data.thematique,
      pays: data.pays,
      ville: data.ville,
      date_heure_debut: data.date_heure_debut,
      date_heure_fin: data.date_heure_fin,
      adresse: data.adresse,
      lien_en_ligne: data.lien_en_ligne,
      nombre_places: data.nombre_places,
      type_organisateur: data.type_organisateur,
      contact_nom: data.contact_nom,
      contact_email: data.contact_email,
      contact_telephone: data.contact_telephone,
      contact_site_web: data.contact_site_web,
    },
    data.couverture_file,
  )
  if (result) {
    showModal.value = false
    await chargerEvenements()
  }
}

// ─── Mode carte ───────────────────────────────────────────────────────────

const PAYS_AFRICAINS_SET = new Set<string>(PAYS_AFRICAINS_ISO2)

/** Index nom-français-normalisé → code ISO2. */
const isoParNom: Record<string, string> = Object.fromEntries(
  Object.entries(NOMS_PAYS_FR).map(([iso, nom]) => [normaliserNomPays(nom), iso]))

const africaLocations = computed(() =>
  World.locations.filter(loc => PAYS_AFRICAINS_SET.has(loc.id.toLowerCase())))

/** Événements regroupés par code ISO2. */
const evenementsParPays = computed<Record<string, EvenementAPI[]>>(() => {
  const groupes: Record<string, EvenementAPI[]> = {}
  for (const e of evenements.value) {
    if (!e.pays) continue
    const iso = isoParNom[normaliserNomPays(e.pays)]
    if (!iso) continue
    ;(groupes[iso] ||= []).push(e)
  }
  return groupes
})

/** Épaisseur de trait : réduite pour les petites îles agrandies. */
const strokeWidth = (id: string): number => {
  const facteur = PETITES_ILES[id]
  return facteur ? 0.5 / facteur : 0.5
}

const svgRef = ref<SVGSVGElement | null>(null)
const mapTransforms = ref<Record<string, string>>({})

/** Scale centré sur le centroïde de chaque petite île. */
const calculerTransformsIles = () => {
  const svg = svgRef.value
  if (!svg) return
  const transforms: Record<string, string> = {}
  for (const [id, facteur] of Object.entries(PETITES_ILES)) {
    const path = svg.querySelector<SVGPathElement>(`path[data-id="${id}"]`)
    if (!path) continue
    const bbox = path.getBBox()
    const cx = bbox.x + bbox.width / 2
    const cy = bbox.y + bbox.height / 2
    transforms[id] = `translate(${cx} ${cy}) scale(${facteur}) translate(${-cx} ${-cy})`
  }
  mapTransforms.value = transforms
}

watch([viewMode, africaLocations], async () => {
  if (viewMode.value === 'carte') {
    await nextTick()
    calculerTransformsIles()
  }
})

const hoveredCountry = ref<{ id: string, name: string } | null>(null)
const mousePos = ref({ x: 0, y: 0 })
const selectedPays = ref<string | null>(null)

const evenementsPaysSelectionne = computed(() =>
  selectedPays.value ? (evenementsParPays.value[selectedPays.value] || []) : [])

/** Trois états seulement : sélectionné, porteur d'événements, vide. */
const getMapColor = (id: string): string => {
  const survole = hoveredCountry.value?.id === id
  const aEvenements = Boolean(evenementsParPays.value[id]?.length)

  if (aEvenements) {
    if (selectedPays.value === id) return '#FFD700'
    return survole ? '#1d761d' : '#228B22'
  }
  return survole ? '#bdbdbd' : '#e5e7eb'
}

const handleMapMouseMove = (event: MouseEvent) => {
  const rect = (event.currentTarget as HTMLElement).getBoundingClientRect()
  mousePos.value = { x: event.clientX - rect.left, y: event.clientY - rect.top }
}

const handleMapClick = (location: { id: string }) => {
  if (evenementsParPays.value[location.id]?.length) selectedPays.value = location.id
}
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Africalive"
        sous-titre="Webinaires, conférences, forums et ateliers"
        image="/images/even1.png"
        aide="C'est quoi Africalive ?"
        @aide="decouverteOuverte = true"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[{ libelle: 'Africalive', vers: '/evenements' }, { libelle: 'Événements' }]"
      >
        <template #action>
          <AfricansBouton icone="fa-solid fa-calendar-plus" @click="showModal = true">
            Proposer un événement
          </AfricansBouton>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="flex flex-col gap-6">
      <!-- Bascule grille / carte : la carte ne concerne que la zone Afrique. -->
      <div v-if="filtreZone === 'afrique'" class="flex gap-2">
        <button
          v-for="mode in [
            { valeur: 'grille' as const, libelle: 'Grille', icone: 'fa-solid fa-table-cells-large' },
            { valeur: 'carte' as const, libelle: 'Carte', icone: 'fa-solid fa-earth-africa' }]"
          :key="mode.valeur"
          type="button"
          class="flex items-center gap-2 rounded-full px-4 py-2 text-[14px]/[1.4] font-bold transition"
          :class="viewMode === mode.valeur ? 'bg-af-chocolat text-white' : 'bg-af-fond text-af-corps hover:bg-af-bordure'"
          :aria-pressed="viewMode === mode.valeur"
          @click="viewMode = mode.valeur"
        >
          <font-awesome-icon :icon="mode.icone" />
          {{ mode.libelle }}
        </button>
      </div>

      <div v-if="chargement" class="grid gap-5 sm:grid-cols-2">
        <div v-for="n in 4" :key="n" class="h-64 animate-pulse rounded-[10px] bg-af-bordure" />
      </div>

      <div v-else-if="erreur" class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
        <font-awesome-icon icon="fa-solid fa-triangle-exclamation" class="text-4xl text-af-live" />
        <p class="mt-4 text-[16px]/[1.4] font-bold">Erreur de chargement</p>
        <p class="mt-2 text-[14px]/[1.4] text-af-corps">{{ erreur }}</p>
        <AfricansBouton class="mt-6" variante="secondaire" icone="fa-solid fa-rotate-left" @click="chargerEvenements">
          Réessayer
        </AfricansBouton>
      </div>

      <!-- Grille -->
      <div v-else-if="viewMode === 'grille' && evenements.length" class="grid gap-5 sm:grid-cols-2">
        <EvenementsEvenementCard
          v-for="evenement in evenements"
          :key="evenement.id"
          :evenement="evenement"
        />
      </div>

      <div v-else-if="viewMode === 'grille'" class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
        <font-awesome-icon icon="fa-solid fa-calendar-xmark" class="text-4xl text-af-atone-2" />
        <p class="mt-4 text-[16px]/[1.4] font-bold">Aucun événement trouvé</p>
        <p class="mt-2 text-[14px]/[1.4] text-af-corps">
          Essayez de modifier vos filtres, ou proposez un nouvel événement.
        </p>
        <AfricansBouton class="mt-6" icone="fa-solid fa-calendar-plus" @click="showModal = true">
          Proposer un événement
        </AfricansBouton>
      </div>

      <!-- Carte -->
      <div v-else class="flex flex-col gap-5">
        <div class="rounded-[10px] border border-af-bordure bg-white p-2">
          <div class="relative" @mousemove="handleMapMouseMove">
            <svg
              ref="svgRef"
              :viewBox="AFRICA_VIEWBOX"
              class="mx-auto block h-auto max-h-[70svh] w-full"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path
                v-for="location in africaLocations"
                :key="location.id"
                :data-id="location.id"
                :d="location.path"
                :fill="getMapColor(location.id)"
                stroke="#fff"
                :stroke-width="strokeWidth(location.id)"
                class="transition-[fill,opacity] duration-200 hover:opacity-85"
                :class="evenementsParPays[location.id]?.length && 'cursor-pointer'"
                :transform="mapTransforms[location.id]"
                @mouseenter="hoveredCountry = location"
                @mouseleave="hoveredCountry = null"
                @click="handleMapClick(location)"
              />
            </svg>

            <div
              v-if="hoveredCountry"
              class="pointer-events-none absolute z-50 -translate-y-1/2 rounded-lg bg-black/85 px-3 py-2 text-[12px]/[1.4] whitespace-nowrap text-white"
              :style="{ left: `${mousePos.x + 15}px`, top: `${mousePos.y - 10}px` }"
            >
              <span class="font-bold">{{ NOMS_PAYS_FR[hoveredCountry.id] || hoveredCountry.name }}</span>
              <span v-if="evenementsParPays[hoveredCountry.id]?.length" class="block opacity-70">
                {{ evenementsParPays[hoveredCountry.id]!.length }}
                événement{{ evenementsParPays[hoveredCountry.id]!.length > 1 ? 's' : '' }}
              </span>
            </div>
          </div>

          <!-- Légende : les trois états que la carte peut réellement peindre. -->
          <ul class="flex flex-wrap gap-4 px-3 pt-2 pb-3">
            <li
              v-for="etat in [
                { couleur: '#228B22', libelle: 'Événements disponibles' },
                { couleur: '#FFD700', libelle: 'Sélectionné' },
                { couleur: '#e5e7eb', libelle: 'Aucun événement' }]"
              :key="etat.libelle"
              class="flex items-center gap-2 text-[12px]/[1.4] text-af-corps"
            >
              <span class="size-3 shrink-0 rounded-full" :style="{ backgroundColor: etat.couleur }" />
              {{ etat.libelle }}
            </li>
          </ul>
        </div>

        <!-- Événements du territoire sélectionné -->
        <div v-if="selectedPays" class="flex flex-col gap-4">
          <div class="flex items-center justify-between gap-4">
            <h2 class="text-[20px]/[1.4] font-bold text-af-chocolat">
              {{ NOMS_PAYS_FR[selectedPays] || selectedPays }}
              <span class="text-[14px]/[1.4] font-normal text-af-atone">
                {{ evenementsPaysSelectionne.length }}
                événement{{ evenementsPaysSelectionne.length > 1 ? 's' : '' }}
              </span>
            </h2>
            <button
              type="button"
              class="grid size-8 place-items-center rounded-full text-af-corps transition hover:bg-af-fond"
              aria-label="Fermer"
              @click="selectedPays = null"
            >
              <font-awesome-icon icon="fa-solid fa-xmark" />
            </button>
          </div>

          <div class="grid gap-5 sm:grid-cols-2">
            <EvenementsEvenementCard
              v-for="evenement in evenementsPaysSelectionne"
              :key="evenement.id"
              :evenement="evenement"
            />
          </div>
        </div>

        <p v-else class="rounded-[10px] border border-af-bordure bg-white p-6 text-center text-[14px]/[1.4] text-af-corps">
          <font-awesome-icon icon="fa-solid fa-hand-pointer" class="mr-2 text-af-atone-2" />
          Cliquez sur un territoire mis en évidence pour voir ses événements.
        </p>
      </div>
    </div>

    <template #rail>
      <AfricansPanneau titre="Filtres" icone="fa-solid fa-sliders">
        <div class="flex flex-col gap-5">
          <div class="flex flex-col gap-2">
            <p class="text-[12px]/[1.4] font-bold text-af-atone uppercase">Année</p>
            <div class="flex flex-wrap gap-2">
              <button
                v-for="annee in ANNEES"
                :key="annee"
                type="button"
                class="rounded-full px-3 py-1.5 text-[12px]/[1.4] font-bold transition"
                :class="anneeSelected === annee ? 'bg-af-chocolat text-white' : 'bg-af-fond text-af-corps hover:bg-af-bordure'"
                :aria-pressed="anneeSelected === annee"
                @click="anneeSelected = annee"
              >
                {{ annee }}
              </button>
            </div>
          </div>

          <label class="flex flex-col gap-2">
            <span class="text-[12px]/[1.4] font-bold text-af-atone uppercase">Format</span>
            <select
              v-model="filtreType"
              class="h-10 w-full rounded-[10px] border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:outline-2 focus:outline-af-chocolat"
            >
              <option v-for="type in TYPES_EVENEMENT" :key="type.value" :value="type.value">
                {{ type.label }}
              </option>
            </select>
          </label>

          <div class="flex flex-col gap-2">
            <p class="text-[12px]/[1.4] font-bold text-af-atone uppercase">Zone</p>
            <div class="flex flex-wrap gap-2">
              <button
                v-for="zone in ZONES"
                :key="zone.valeur"
                type="button"
                class="rounded-full px-3 py-1.5 text-[12px]/[1.4] font-bold transition"
                :class="filtreZone === zone.valeur ? 'bg-af-chocolat text-white' : 'bg-af-fond text-af-corps hover:bg-af-bordure'"
                :aria-pressed="filtreZone === zone.valeur"
                @click="filtreZone = zone.valeur"
              >
                {{ zone.libelle }}
              </button>
            </div>
          </div>

          <!-- La liste des territoires est exclusivement africaine : elle n'a
               rien à proposer en « Hors Afrique ». En « Tout » elle reste
               offerte, non filtrante par elle-même. -->
          <label v-if="filtreZone !== 'hors_afrique'" class="flex flex-col gap-2">
            <span class="text-[12px]/[1.4] font-bold text-af-atone uppercase">Territoire</span>
            <select
              v-model="filtrePays"
              class="h-10 w-full rounded-[10px] border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:outline-2 focus:outline-af-chocolat"
            >
              <option value="">Tous les territoires</option>
              <option v-for="pays in PAYS_AFRICAINS" :key="pays" :value="pays">{{ pays }}</option>
            </select>
          </label>
        </div>
      </AfricansPanneau>
    </template>

    <EvenementsEvenementModal :show="showModal" @close="showModal = false" @submit="handleSubmit" />

    <EvenementsDecouverteModale v-model="decouverteOuverte" />
  </NuxtLayout>
</template>
