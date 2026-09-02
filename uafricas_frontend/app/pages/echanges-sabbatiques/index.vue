<script setup lang="ts">
import World from '@svg-maps/world'
import { PAYS_AFRICAINS_ISO2 } from '~/constants/afripulsePaysAutorises'
import {
  useSabbatiques,
  TYPES_PROGRAMME,
  PAYS_AFRICAINS,
  PAYS_HORS_AFRIQUE,
  DOMAINES,
  type SabbatiqueAPI,
  type SabbatiqueFiltres,
} from '~/composables/useSabbatiques'

/**
 * Sabbafrica : porté sur le gabarit de la refonte.
 *
 * Les deux modes (carte d'Afrique par défaut, grille) et tous les filtres
 * serveur sont conservés, y compris la règle qui vide le territoire choisi à
 * chaque changement de zone (listes disjointes).
 *
 * Comme pour Africalive, les tables de la carte, `NOMS_PAYS_FR`,
 * `PETITES_ILES`, `AFRICA_VIEWBOX`, viennent désormais de
 * `utils/carteAfrique.ts` au lieu d'être recopiées ici.
 */
definePageMeta({ layout: false })

useHead({
  title: 'Sabbafrica : Échanges sabbatiques | AfricanS',
  meta: [
    {
      name: 'description',
      content: "Partagez votre expertise auprès d'organisations africaines, le temps d'une mission.",
    }],
})

const { listerProgrammes, chargement } = useSabbatiques()

// ─── État ─────────────────────────────────────────────────────────────────

const decouverteOuverte = ref(false)
const viewMode = ref<'grille' | 'carte'>('carte')
const programmes = ref<SabbatiqueAPI[]>([])
const total = ref(0)

const filtres = ref<SabbatiqueFiltres>({
  type: 'tous',
  pays: '',
  domaine: '',
  recherche: '',
})

const ZONES = [
  { valeur: 'tout' as const, libelle: 'Mondial' },
  { valeur: 'afrique' as const, libelle: 'Afrique' },
  { valeur: 'hors_afrique' as const, libelle: 'Hors Afrique' }]
type ZoneTerritoire = (typeof ZONES)[number]['valeur']
const zoneTerritoire = ref<ZoneTerritoire>('tout')

/** En zone « Tout », les deux listes fusionnent sous une seule entrée vide. */
const territoiresDisponibles = computed(() => {
  if (zoneTerritoire.value === 'afrique') return PAYS_AFRICAINS
  if (zoneTerritoire.value === 'hors_afrique') return PAYS_HORS_AFRIQUE
  const tous = [...PAYS_AFRICAINS.slice(1), ...PAYS_HORS_AFRIQUE.slice(1)]
    .sort((a, b) => a.label.localeCompare(b.label, 'fr'))
  return [{ value: '', label: 'Tous les territoires' }, ...tous]
})

const filtresActifs = computed(() =>
  zoneTerritoire.value !== 'tout'
  || filtres.value.type !== 'tous'
  || Boolean(filtres.value.pays)
  || Boolean(filtres.value.domaine)
  || Boolean(filtres.value.recherche))

// ─── Chargement ───────────────────────────────────────────────────────────

const chargerProgrammes = async () => {
  const result = await listerProgrammes({
    ...filtres.value,
    zone: zoneTerritoire.value,
    par_page: 60,
  })
  if (result) {
    programmes.value = result.programmes
    total.value = result.total
  }
}

// Changer de zone vide le territoire choisi (listes disjointes) et recharge.
watch(zoneTerritoire, () => {
  filtres.value.pays = ''
  chargerProgrammes()
})

watch(() => [filtres.value.type, filtres.value.pays, filtres.value.domaine], chargerProgrammes)

let searchTimeout: ReturnType<typeof setTimeout> | null = null
const onSearchInput = () => {
  if (searchTimeout) clearTimeout(searchTimeout)
  searchTimeout = setTimeout(chargerProgrammes, 500)
}

const reinitialiserFiltres = () => {
  selectedPays.value = null
  zoneTerritoire.value = 'tout'
  filtres.value = { type: 'tous', pays: '', domaine: '', recherche: '' }
  chargerProgrammes()
}

onMounted(async () => {
  await chargerProgrammes()
  if (viewMode.value === 'carte') {
    await nextTick()
    calculerTransformsIles()
  }
})

// ─── Mode carte ───────────────────────────────────────────────────────────

const PAYS_AFRICAINS_SET = new Set<string>(PAYS_AFRICAINS_ISO2)

/** Index nom-français-normalisé → code ISO2. */
const isoParNom: Record<string, string> = Object.fromEntries(
  Object.entries(NOMS_PAYS_FR).map(([iso, nom]) => [normaliserNomPays(nom), iso]))

const africaLocations = computed(() =>
  World.locations.filter(loc => PAYS_AFRICAINS_SET.has(loc.id.toLowerCase())))

const programmesParPays = computed<Record<string, SabbatiqueAPI[]>>(() => {
  const groupes: Record<string, SabbatiqueAPI[]> = {}
  for (const p of programmes.value) {
    if (!p.pays) continue
    const iso = isoParNom[normaliserNomPays(p.pays)]
    if (!iso) continue
    ;(groupes[iso] ||= []).push(p)
  }
  return groupes
})

const strokeWidth = (id: string): number => {
  const facteur = PETITES_ILES[id]
  return facteur ? 0.5 / facteur : 0.5
}

const svgRef = ref<SVGSVGElement | null>(null)
const mapTransforms = ref<Record<string, string>>({})

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

const programmesPaysSelectionne = computed(() =>
  selectedPays.value ? (programmesParPays.value[selectedPays.value] || []) : [])

/**
 * Un filtre peut vider le territoire retenu de tous ses programmes. Le
 * panneau restait alors ouvert sur « 0 programme » et une grille vide, alors
 * que la carte, elle, avait déjà repeint le pays en gris.
 */
watch(programmesParPays, (groupes) => {
  if (selectedPays.value && !groupes[selectedPays.value]?.length) selectedPays.value = null
})

const getMapColor = (id: string): string => {
  const survole = hoveredCountry.value?.id === id
  const aProgrammes = Boolean(programmesParPays.value[id]?.length)

  if (aProgrammes) {
    if (selectedPays.value === id) return '#FFD700'
    return survole ? '#1d761d' : '#228B22'
  }
  return survole ? '#bdbdbd' : '#e5e7eb'
}

const handleMapMouseMove = (event: MouseEvent) => {
  const rect = (event.currentTarget as HTMLElement).getBoundingClientRect()
  mousePos.value = { x: event.clientX - rect.left, y: event.clientY - rect.top }
}

const panneauPaysRef = ref<HTMLElement | null>(null)

/**
 * Le panneau du territoire naît SOUS la carte, qui occupe jusqu'à 70svh : il
 * apparaissait donc hors de l'écran, et le clic restait sans effet visible
 * ailleurs que dans la couleur du pays.
 */
const amenerPanneauALEcran = async () => {
  await nextTick()
  amenerSousLaBarre(panneauPaysRef.value)
}

const handleMapClick = (location: { id: string }) => {
  if (!programmesParPays.value[location.id]?.length) return
  // Re-cliquer le territoire déjà retenu le désélectionne : sans cela, le
  // seul moyen de revenir à la carte nue était la croix du panneau.
  if (selectedPays.value === location.id) {
    selectedPays.value = null
    return
  }
  selectedPays.value = location.id
  amenerPanneauALEcran()
}
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Sabbafrica"
        sous-titre="Offrir un peu de son temps au développement de l'Afrique"
        image="/images/alliance-afrique.jpg"
        aide="C'est quoi Sabbafrica ?"
        @aide="decouverteOuverte = true"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane :segments="[{ libelle: 'Sabbafrica' }]">
        <template #action>
          <AfricansBouton icone="fa-solid fa-plus" vers="/echanges-sabbatiques/proposer?type=interafricain">
            Proposer un échange
          </AfricansBouton>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="flex flex-col gap-6">
      <div class="flex gap-2">
        <button
          v-for="mode in [
            { valeur: 'carte' as const, libelle: 'Carte', icone: 'fa-solid fa-earth-africa' },
            { valeur: 'grille' as const, libelle: 'Grille', icone: 'fa-solid fa-table-cells-large' }]"
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
        <div v-for="i in 4" :key="i" class="h-64 animate-pulse rounded-[10px] bg-af-bordure" />
      </div>

      <!-- Grille -->
      <template v-else-if="viewMode === 'grille'">
        <div v-if="programmes.length" class="grid gap-5 sm:grid-cols-2">
          <SabbatiqueCard
            v-for="programme in programmes"
            :key="programme.id"
            :programme="programme"
          />
        </div>

        <div v-else class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
          <font-awesome-icon icon="fa-solid fa-plane" class="text-4xl text-af-atone-2" />
          <p class="mt-4 text-[16px]/[1.4] font-bold">Aucun programme trouvé</p>
          <p class="mt-2 text-[14px]/[1.4] text-af-corps">
            {{ filtresActifs ? 'Essayez de modifier vos filtres.' : 'Les programmes proposés apparaîtront ici.' }}
          </p>
          <AfricansBouton
            v-if="filtresActifs"
            class="mt-6"
            variante="secondaire"
            icone="fa-solid fa-rotate-left"
            @click="reinitialiserFiltres"
          >
            Réinitialiser les filtres
          </AfricansBouton>
        </div>
      </template>

      <!-- Carte -->
      <template v-else>
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
                :class="programmesParPays[location.id]?.length && 'cursor-pointer'"
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
              <span v-if="programmesParPays[hoveredCountry.id]?.length" class="block opacity-70">
                {{ programmesParPays[hoveredCountry.id]!.length }}
                programme{{ programmesParPays[hoveredCountry.id]!.length > 1 ? 's' : '' }}
              </span>
            </div>
          </div>

          <ul class="flex flex-wrap gap-4 px-3 pt-2 pb-3">
            <li
              v-for="etat in [
                { couleur: '#228B22', libelle: 'Programmes disponibles' },
                { couleur: '#FFD700', libelle: 'Sélectionné' },
                { couleur: '#e5e7eb', libelle: 'Aucun programme' }]"
              :key="etat.libelle"
              class="flex items-center gap-2 text-[12px]/[1.4] text-af-corps"
            >
              <span class="size-3 shrink-0 rounded-full" :style="{ backgroundColor: etat.couleur }" />
              {{ etat.libelle }}
            </li>
          </ul>
        </div>

        <div v-if="selectedPays" ref="panneauPaysRef" class="flex flex-col gap-4 scroll-mt-af-barre">
          <div class="flex items-center justify-between gap-4">
            <h2 class="text-[20px]/[1.4] font-bold text-af-chocolat">
              {{ NOMS_PAYS_FR[selectedPays] || selectedPays }}
              <span class="text-[14px]/[1.4] font-normal text-af-atone">
                {{ programmesPaysSelectionne.length }}
                programme{{ programmesPaysSelectionne.length > 1 ? 's' : '' }}
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
            <SabbatiqueCard
              v-for="programme in programmesPaysSelectionne"
              :key="programme.id"
              :programme="programme"
            />
          </div>
        </div>

        <p v-else class="rounded-[10px] border border-af-bordure bg-white p-6 text-center text-[14px]/[1.4] text-af-corps">
          <font-awesome-icon icon="fa-solid fa-hand-pointer" class="mr-2 text-af-atone-2" />
          Cliquez sur un territoire mis en évidence pour voir ses programmes.
        </p>
      </template>
    </div>

    <template #rail>
      <AfricansPanneau titre="Filtres" icone="fa-solid fa-sliders" action-libelle="Réinitialiser" @action="reinitialiserFiltres">
        <div class="flex flex-col gap-5">
          <label class="relative block">
            <span class="sr-only">Rechercher un programme</span>
            <font-awesome-icon
              icon="fa-solid fa-magnifying-glass"
              class="pointer-events-none absolute top-1/2 left-3 -translate-y-1/2 text-af-atone-2"
            />
            <input
              v-model="filtres.recherche"
              type="search"
              placeholder="Rechercher…"
              class="h-10 w-full rounded-[10px] border border-af-bordure bg-white pr-3 pl-9 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
              @input="onSearchInput"
            />
          </label>

          <div class="flex flex-col gap-2">
            <p class="text-[12px]/[1.4] font-bold text-af-atone uppercase">Type de programme</p>
            <label
              v-for="type in TYPES_PROGRAMME"
              :key="type.value"
              class="flex cursor-pointer items-center gap-3 text-[14px]/[1.4] text-af-corps"
            >
              <input
                v-model="filtres.type"
                type="radio"
                :value="type.value"
                class="size-4 accent-af-chocolat"
              />
              {{ type.label }}
            </label>
          </div>

          <div class="flex flex-col gap-2">
            <p class="text-[12px]/[1.4] font-bold text-af-atone uppercase">Zone</p>
            <div class="flex flex-wrap gap-2">
              <button
                v-for="zone in ZONES"
                :key="zone.valeur"
                type="button"
                class="rounded-full px-3 py-1.5 text-[12px]/[1.4] font-bold transition"
                :class="zoneTerritoire === zone.valeur ? 'bg-af-chocolat text-white' : 'bg-af-fond text-af-corps hover:bg-af-bordure'"
                :aria-pressed="zoneTerritoire === zone.valeur"
                @click="zoneTerritoire = zone.valeur"
              >
                {{ zone.libelle }}
              </button>
            </div>
          </div>

          <label class="flex flex-col gap-2">
            <span class="text-[12px]/[1.4] font-bold text-af-atone uppercase">Territoire</span>
            <select
              v-model="filtres.pays"
              class="h-10 w-full rounded-[10px] border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:outline-2 focus:outline-af-chocolat"
            >
              <option v-for="pays in territoiresDisponibles" :key="pays.value" :value="pays.value">
                {{ pays.label }}
              </option>
            </select>
          </label>

          <label class="flex flex-col gap-2">
            <span class="text-[12px]/[1.4] font-bold text-af-atone uppercase">Domaine</span>
            <select
              v-model="filtres.domaine"
              class="h-10 w-full rounded-[10px] border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:outline-2 focus:outline-af-chocolat"
            >
              <option v-for="domaine in DOMAINES" :key="domaine.value" :value="domaine.value">
                {{ domaine.label }}
              </option>
            </select>
          </label>
        </div>
      </AfricansPanneau>

      <AfricansPanneau titre="Proposer" icone="fa-solid fa-plane">
        <div class="flex flex-col gap-3">
          <p class="text-[14px]/[1.4] text-af-atone">
            <span class="text-[20px]/[1.4] font-bold text-af-chocolat">{{ total }}</span>
            programme{{ total > 1 ? 's' : '' }} publié{{ total > 1 ? 's' : '' }}
          </p>
          <AfricansBouton icone="fa-solid fa-plus" vers="/echanges-sabbatiques/proposer?type=interafricain">
            Échange interafricain
          </AfricansBouton>
          <AfricansBouton variante="secondaire" icone="fa-solid fa-plus" vers="/echanges-sabbatiques/proposer?type=hors_afrique">
            Échange hors Afrique
          </AfricansBouton>
        </div>
      </AfricansPanneau>
    </template>

    <SabbatiqueDecouverteModale v-model="decouverteOuverte" />
  </NuxtLayout>
</template>
