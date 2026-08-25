<script setup lang="ts">
import {
  useOpportuniteAfrique,
  type FichePaysAPI,
} from '~/composables/useOpportuniteAfrique'

/**
 * Afripulse : territoires africains, porté sur le gabarit de la refonte.
 *
 * Données inchangées : même endpoint, mêmes filtres serveur, même carte SVG.
 * Trois déplacements de présentation :
 *   - la bascule Grille / Carte monte dans la barre de contexte, où la
 *     maquette la pose (le composant `Bascule` avait été relevé sur cet écran) ;
 *   - les filtres et la légende passent dans le rail droit ;
 *   - la carte SVG et ses agrandissements d'îles sortent en composant, la page
 *     faisait 657 lignes dont la moitié de géométrie.
 */
definePageMeta({ layout: false })

useHead({
  title: 'Afripulse - Opportunités en Afrique | AfricanS',
  meta: [
    { name: 'description', content: 'Decouvrez les fiches territoires africains et leurs opportunites economiques, culturelles et sociales.' }],
})

const { chargement, listerFiches, listerRegions } = useOpportuniteAfrique()

// Modale de présentation « C'est quoi Afripulse ? »
const presentationOuverte = ref(false)

const paysList = ref<FichePaysAPI[]>([])
// `?recherche=` : la fiche d'un territoire porte le même champ de recherche
// dans son rail (comme la maquette), et il renvoie ici. Sans cette lecture,
// arriver avec la requête dans l'URL afficherait la liste entière.
const route = useRoute()
const searchTerm = ref((route.query.recherche as string) || '')
const selectedRegion = ref('')
const regions = ref<string[]>([])
const totalPays = ref(0)
const viewMode = ref<'grille' | 'carte'>('carte')

// Territoire sélectionné sur la carte
const selectedMapPays = ref<FichePaysAPI | null>(null)
const ficheCarteRef = ref<HTMLElement | null>(null)

/**
 * `block: 'nearest'` ne fait défiler que le strict nécessaire : au-delà de
 * 1280 px la fiche est déjà dans le champ, l'appel ne bouge donc rien. En
 * dessous, où le rail s'empile sous la carte, il l'amène à l'écran.
 */
watch(selectedMapPays, async (pays) => {
  if (!pays) return
  await nextTick()
  ficheCarteRef.value?.scrollIntoView({ block: 'nearest', behavior: 'smooth' })
})

const chargerFiches = async () => {
  const result = await listerFiches({
    recherche: searchTerm.value || undefined,
    region: selectedRegion.value || undefined,
    par_page: 60,
  })

  if (result) {
    paysList.value = result.fiches
    totalPays.value = result.total
  }
}

let searchTimeout: ReturnType<typeof setTimeout> | null = null

watch(searchTerm, () => {
  if (searchTimeout) clearTimeout(searchTimeout)
  searchTimeout = setTimeout(chargerFiches, 500)
})

watch(selectedRegion, () => {
  chargerFiches()
})

const resetFilters = () => {
  searchTerm.value = ''
  selectedRegion.value = ''
}

const aucunFiltreActif = computed(() => !searchTerm.value && !selectedRegion.value)

const navigateToDetail = (id: string) => navigateTo(`/opportunite-afrique/${id}`)

/** Les régions réellement présentes dans la sélection courante : la légende ne
 *  doit décrire que des couleurs visibles sur la carte affichée. */
const regionsAffichees = computed(() => {
  const presentes = new Set(paysList.value.map(f => f.region))
  return Object.keys(COULEURS_REGION).filter(r => presentes.has(r))
})

onMounted(async () => {
  const regionsResult = await listerRegions()
  if (regionsResult) regions.value = regionsResult
  await chargerFiches()
})
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Afripulse"
        image="/images/africans/heros/hero-afripulse.jpg"
        aide="C'est quoi Afripulse ?"
        @aide="presentationOuverte = true"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane :segments="[{ libelle: 'Africarise', vers: '/codi-moi' }, { libelle: 'Afripulse' }]">
        <template #centre>
          <p class="text-base font-bold text-af-encre">
            Explorez les territoires africains et leurs richesses
          </p>
        </template>
        <template #action>
          <AfricansBascule
            v-model="viewMode"
            libelle="Mode d'affichage des territoires"
            :options="[
              { valeur: 'grille', libelle: 'Grille', icone: 'fa-solid fa-table-cells-large' },
              { valeur: 'carte', libelle: 'Carte', icone: 'fa-solid fa-earth-africa' }]"
          />
        </template>
      </AfricansFilAriane>
    </template>

    <div class="flex flex-col gap-6">
      <!-- Chargement : squelettes aux dimensions réelles des cartes. -->
      <div v-if="chargement" class="grid gap-5 sm:grid-cols-2">
        <div v-for="i in 4" :key="i" class="overflow-hidden rounded-[10px] border border-af-bordure bg-white">
          <div class="aspect-[16/10] w-full animate-pulse bg-af-bordure" />
          <div class="flex flex-col gap-3 p-4">
            <div class="h-4 w-2/3 animate-pulse rounded bg-af-bordure" />
            <div class="h-3 w-1/2 animate-pulse rounded bg-af-bordure" />
          </div>
        </div>
      </div>

      <template v-else-if="viewMode === 'grille'">
        <div v-if="paysList.length" class="grid gap-5 sm:grid-cols-2">
          <AfricansCarteTerritoire
            v-for="pays in paysList"
            :key="pays.id"
            :nom="pays.nom"
            :region="pays.region"
            :devise="pays.slogan || undefined"
            :capitale="pays.capitale || undefined"
            :population="pays.population || undefined"
            :drapeau-url="pays.drapeau_url"
            :image="pays.image_couverture"
            :contributions="pays.nombre_contributions"
            :vers="`/opportunite-afrique/${pays.id}`"
          />
        </div>

        <!-- Deux vides distincts : « rien ne correspond » n'est pas « rien
             n'existe », et la sortie proposée n'est pas la même. -->
        <div v-else class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
          <font-awesome-icon icon="fa-solid fa-earth-africa" class="text-4xl text-af-atone-2" />
          <p class="mt-4 text-[16px]/[1.4] font-bold">
            {{ aucunFiltreActif ? 'Aucun territoire publié pour le moment' : 'Aucun territoire ne correspond à vos critères' }}
          </p>
          <AfricansBouton
            v-if="!aucunFiltreActif"
            class="mt-5"
            variante="secondaire"
            icone="fa-solid fa-rotate-right"
            @click="resetFilters"
          >
            Réinitialiser les filtres
          </AfricansBouton>
        </div>
      </template>

      <OpportuniteAfriqueCarteAfrique
        v-else
        :fiches="paysList"
        :selection="selectedMapPays"
        @selectionner="selectedMapPays = $event"
      />
    </div>

    <template #rail>
      <!-- Territoire cliqué sur la carte. Il vivait SOUS la carte, laquelle
           occupe 70 % de la hauteur d'écran : la fiche naissait hors champ, et
           rien ne signalait que le clic avait produit quelque chose. Dans le
           rail elle est à hauteur de regard, et la carte reste visible.
           Le rail ne devient une vraie colonne latérale qu'à partir de 1280 px ;
           en dessous il s'empile, d'où le `scrollIntoView` ci-dessous. -->
      <div v-if="viewMode === 'carte'" ref="ficheCarteRef" class="scroll-mt-24">
        <AfricansPanneau titre="Territoire sélectionné" icone="fa-solid fa-map-pin">
          <Transition name="af-glisser" mode="out-in">
            <div v-if="selectedMapPays" :key="selectedMapPays.id" class="flex flex-col gap-3">
              <AfricansCarteTerritoire
                :nom="selectedMapPays.nom"
                :region="selectedMapPays.region"
                :devise="selectedMapPays.slogan || undefined"
                :capitale="selectedMapPays.capitale || undefined"
                :population="selectedMapPays.population || undefined"
                :drapeau-url="selectedMapPays.drapeau_url"
                :image="selectedMapPays.image_couverture"
                :contributions="selectedMapPays.nombre_contributions"
              />
              <AfricansBouton
                pleine-largeur
                icone="fa-solid fa-arrow-right"
                @click="navigateToDetail(selectedMapPays.id)"
              >
                Voir la fiche territoire
              </AfricansBouton>
            </div>

            <p v-else class="text-[14px]/[1.4] text-af-atone">
              Cliquez sur un territoire coloré pour en afficher la fiche. Les territoires en gris
              n'ont pas encore de fiche publiée.
            </p>
          </Transition>
        </AfricansPanneau>
      </div>

      <AfricansRecherche v-model="searchTerm" placeholder="Territoire, région, pays…" />

      <!-- La maquette montre aussi un filtre « Langues » : `listerFiches` n'a
           pas de paramètre correspondant, et le poser ici ne filtrerait rien.
           Il est omis plutôt que rendu inerte. -->
      <AfricansPanneau titre="Filtres" icone="fa-solid fa-sliders" action-libelle="Réinitialiser" @action="resetFilters">
        <AfricansChamp v-model="selectedRegion" libelle="Régions" type="select">
          <option value="">Toutes les régions</option>
          <option v-for="region in regions" :key="region" :value="region">{{ region }}</option>
        </AfricansChamp>
      </AfricansPanneau>

      <AfricansPanneau v-if="viewMode === 'carte'" titre="Légende" icone="fa-solid fa-map-pin">
        <ul class="flex flex-col gap-2">
          <li v-for="region in regionsAffichees" :key="region" class="flex items-center gap-2 text-[12px]/[1.4]">
            <span class="size-3 shrink-0 rounded-full" :style="{ backgroundColor: COULEURS_REGION[region] }" />
            {{ region }}
          </li>
          <li class="flex items-center gap-2 text-[12px]/[1.4]">
            <span class="size-3 shrink-0 rounded-full" :style="{ backgroundColor: COULEUR_SELECTION }" />
            Sélectionné
          </li>
          <li class="flex items-center gap-2 text-[12px]/[1.4]">
            <span class="size-3 shrink-0 rounded-full" :style="{ backgroundColor: COULEUR_SANS_FICHE }" />
            Fiche non disponible
          </li>
        </ul>
      </AfricansPanneau>

      <AfricansPanneau titre="Statistiques" icone="fa-solid fa-chart-line">
        <dl class="flex flex-col">
          <div class="flex items-baseline justify-between gap-4 pb-3">
            <dt class="text-[14px]/[1.4] font-bold">Territoires</dt>
            <dd class="text-[14px]/[1.4] font-bold text-af-chocolat">{{ totalPays }}</dd>
          </div>
          <div class="flex items-baseline justify-between gap-4 border-t border-af-bordure py-3">
            <dt class="text-[14px]/[1.4] font-bold">Contributions</dt>
            <dd class="text-[14px]/[1.4] font-bold text-af-chocolat">
              {{ paysList.reduce((n, p) => n + (Number(p.nombre_contributions) || 0), 0) }}
            </dd>
          </div>
        </dl>
      </AfricansPanneau>
    </template>

    <OpportuniteAfriqueDecouverteModale v-model="presentationOuverte" />
  </NuxtLayout>
</template>

<style scoped>
.af-glisser-enter-active {
  transition: all 0.3s ease-out;
}
.af-glisser-leave-active {
  transition: all 0.2s ease-in;
}
.af-glisser-enter-from,
.af-glisser-leave-to {
  opacity: 0;
  transform: translateX(24px);
}
</style>
