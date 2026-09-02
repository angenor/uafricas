<script setup lang="ts">
/**
 * Vitrine d'une famille de stations radio, le corps commun des deux pages
 * `/medias/radio/africans` et `/medias/radio/nationales`.
 *
 * Les deux fichiers faisaient 217 et 219 lignes pour ne différer que par trois
 * choses : l'origine de publication, une teinte, et quatre phrases. Deux copies
 * d'une même mécanique de filtre finissent par diverger, et rien ne le
 * signalerait : chaque page se contenterait de filtrer un peu différemment.
 *
 * Les PAGES restent distinctes (FR-012) : deux adresses, deux titres, aucune
 * redirection, aucune fusion. C'est seulement leur corps qui est mis en commun,
 * comme `MediaDecorRadio` l'avait déjà été pour le décor.
 *
 * `origine` est fixée par la page appelante et n'est JAMAIS offerte au visiteur
 * comme filtre : c'est elle qui garantit qu'aucune station ne figure sur les
 * deux pages (FR-014).
 */
import type { StationSection } from '~/composables/useStationsRadio'

const props = defineProps<{
  origine: 'africans' | 'territoire'
  titre: string
  accroche: string
  image: string
  /** Fil d'Ariane : dernier segment. */
  segment: string
  /** État vide, quand aucun filtre n'est actif. */
  videTitre: string
  videTexte: string
}>()

const { listerSections, listerPays, listerGenres, chargement } = useStationsRadio()
// Référentiel de FILTRE (US3) : seulement ce qui est réellement déclaré.
const { listerThematiquesDisponibles } = useMediaSupport()

const sections = ref<StationSection[]>([])
const paysDisponibles = ref<string[]>([])
const genresDisponibles = ref<string[]>([])

const propositionOuverte = ref(false)
const reglesOuvertes = ref(false)

const page = ref(1)
const totalPages = ref(1)
const totalStations = ref(0)

const thematiquesSelectionnees = ref<string[]>([])
const thematiquesDisponibles = ref<{ id: string, nom: string, nombre_supports: number }[]>([])

const TYPES_PROGRAMME = ['Tous les types', 'Nationales', 'Local', 'International']

/** Les deux autres applications de l'univers Africamood. */
const AUTRES_MEDIAS = [
  { libelle: 'Télévision africaine', to: '/medias/tele', icone: 'fa-solid fa-tv' },
  { libelle: 'Vidafrica', to: '/vidafrica', icone: 'fa-solid fa-video' }]

const typeSelectionne = ref('Tous les types')
const paysSelectionne = ref('Tous les territoires')
const genreSelectionne = ref('Tous les genres')

const chargerSections = async (numero = 1) => {
  const resultat = await listerSections({
    origine: props.origine,
    type_station: typeSelectionne.value,
    pays: paysSelectionne.value,
    genre: genreSelectionne.value,
    thematiques: thematiquesSelectionnees.value,
    page: numero,
    par_page: 6,
  })
  if (resultat) {
    sections.value = numero === 1 ? resultat.sections : [...sections.value, ...resultat.sections]
    totalPages.value = resultat.totalPages
    totalStations.value = resultat.total
    page.value = resultat.page
  }
}

const reinitialiserFiltres = () => {
  typeSelectionne.value = 'Tous les types'
  paysSelectionne.value = 'Tous les territoires'
  genreSelectionne.value = 'Tous les genres'
  thematiquesSelectionnees.value = []
}

watch(
  [typeSelectionne, paysSelectionne, genreSelectionne, thematiquesSelectionnees],
  () => chargerSections(1))

const encoreDesStations = computed(() => page.value < totalPages.value)

const filtresActifs = computed(() =>
  typeSelectionne.value !== 'Tous les types'
  || paysSelectionne.value !== 'Tous les territoires'
  || genreSelectionne.value !== 'Tous les genres'
  || thematiquesSelectionnees.value.length > 0)

onMounted(async () => {
  const [pays, genres, thematiques] = await Promise.all([
    listerPays(),
    listerGenres(),
    listerThematiquesDisponibles('station_radio')])
  if (pays) paysDisponibles.value = pays
  if (genres) genresDisponibles.value = genres
  thematiquesDisponibles.value = thematiques
  await chargerSections(1)
})
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule :titre="titre" :sous-titre="accroche" :image="image" />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[
          { libelle: 'Africamood', vers: '/medias' },
          { libelle: 'Radios', vers: '/medias/radios' },
          { libelle: segment }]"
      >
        <template #centre>
          <p v-if="totalStations > 0" class="text-base font-bold text-af-encre">
            {{ totalStations }} station{{ totalStations > 1 ? 's' : '' }}
          </p>
        </template>
        <template #action>
          <AfricansBouton icone="fa-solid fa-plus" @click="propositionOuverte = true">
            Proposer un contenu
          </AfricansBouton>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="flex flex-col gap-6">
      <!-- Thématique déclarée : d'une autre nature que le genre, qui décrit la
           couleur d'antenne (US3). Elle reste au-dessus de la liste : c'est un
           filtre à sélection multiple, il lui faut de la largeur. -->
      <MediaBarreFiltresSupport
        v-model:thematiques="thematiquesSelectionnees"
        :thematiques-disponibles="thematiquesDisponibles"
      />

      <div v-if="chargement && !sections.length" class="flex flex-col gap-6">
        <div v-for="n in 2" :key="n" class="h-64 animate-pulse rounded-[10px] bg-af-bordure" />
      </div>

      <template v-else-if="sections.length">
        <MediaSectionStation
          v-for="section in sections"
          :key="section.station.id"
          :section="section"
        />

        <div v-if="encoreDesStations" class="flex justify-center">
          <AfricansBouton variante="secondaire" icone="fa-solid fa-arrow-down" @click="chargerSections(page + 1)">
            Voir plus de stations
          </AfricansBouton>
        </div>
      </template>

      <!-- État vide explicite (FR-019) : dire ce qui manque, et pourquoi. Les
           deux cas restent distincts : « rien ne correspond » n'est pas « rien
           n'est publié », et la sortie proposée n'est pas la même. -->
      <div v-else class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
        <font-awesome-icon icon="fa-solid fa-radio" class="text-4xl text-af-atone-2" />
        <p class="mt-4 text-[16px]/[1.4] font-bold">
          {{ filtresActifs ? 'Aucune station ne correspond à ces filtres' : videTitre }}
        </p>
        <p class="mx-auto mt-2 max-w-md text-[14px]/[1.4] text-af-corps">
          {{ filtresActifs ? 'Essayez d’élargir votre recherche.' : videTexte }}
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
    </div>

    <template #rail>
      <AfricansPanneau titre="Filtres" icone="fa-solid fa-sliders" action-libelle="Réinitialiser" @action="reinitialiserFiltres">
        <div class="flex flex-col gap-4">
          <AfricansChamp v-model="typeSelectionne" libelle="Type de programme" type="select">
            <option v-for="t in TYPES_PROGRAMME" :key="t" :value="t">{{ t }}</option>
          </AfricansChamp>

          <AfricansChamp v-model="paysSelectionne" libelle="Territoire" type="select">
            <option value="Tous les territoires">Tous les territoires</option>
            <option v-for="p in paysDisponibles" :key="p" :value="p">{{ p }}</option>
          </AfricansChamp>

          <AfricansChamp v-model="genreSelectionne" libelle="Genre" type="select">
            <option value="Tous les genres">Tous les genres</option>
            <option v-for="g in genresDisponibles" :key="g" :value="g">{{ g }}</option>
          </AfricansChamp>
        </div>
      </AfricansPanneau>

      <AfricansPanneau titre="Contribuer" icone="fa-solid fa-plus">
        <div class="flex flex-col gap-3">
          <p class="text-[14px]/[1.4] text-af-corps">
            Toute proposition part en attente de validation.
          </p>
          <AfricansBouton pleine-largeur icone="fa-solid fa-plus" @click="propositionOuverte = true">
            Proposer un contenu
          </AfricansBouton>
          <AfricansBouton pleine-largeur variante="secondaire" icone="fa-solid fa-shield-halved" @click="reglesOuvertes = true">
            Règles de contenu
          </AfricansBouton>
        </div>
      </AfricansPanneau>

      <AfricansPanneau titre="Aussi dans Africamood" icone="fa-solid fa-photo-film">
        <ul class="flex flex-col gap-1">
          <li v-for="lien in AUTRES_MEDIAS" :key="lien.to">
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

    <!-- Toute proposition part en attente de validation (FR-031). -->
    <MediaProposerMediaModal
      :is-open="propositionOuverte"
      :types-offerts="['station_radio', 'emission_radio']"
      @close="propositionOuverte = false"
    />

    <!-- Contenus interdits et conséquences d'un signalement (FR-048). -->
    <MediaReglesContenuModal :open="reglesOuvertes" @close="reglesOuvertes = false" />
  </NuxtLayout>
</template>
