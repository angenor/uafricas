<script setup lang="ts">
import type { ContributionCitoyenne } from '~/types/gouvernance'
import type { TypePratique } from '~/composables/useGouvernance'
import { useUserStore } from '~/stores/user'

/**
 * BadGoodhabits : porté sur le gabarit de la refonte.
 *
 * Les deux familles cohabitent toujours sur une seule page, séparées par les
 * trois onglets (Toutes / Badhabits / Goodhabits) ; la gravité et l'impact
 * restent filtrables, chacun n'apparaissant que pour la famille qu'il
 * concerne. Ce qui change : les six fonctions de classes de couleur
 * (`getBandeClass`, `getIconeWrapperClass`, `getGraviteClass`,
 * `getImpactClass`…) tombent : quatre nuances de rouge et quatre de vert pour
 * dire un niveau que le libellé disait déjà. Le niveau reste écrit, en toutes
 * lettres, et l'icône distingue les deux familles.
 *
 * Le pied de carte perd « N vues » : `useGouvernance` écrit `vues: 0` en dur,
 * chaque contribution affichait donc « 0 vues » depuis toujours.
 */
definePageMeta({ layout: false })

useHead({
  title: 'BadGoodhabits : Gouvernance citoyenne | AfricanS',
  meta: [
    {
      name: 'description',
      content: 'Dénoncer les mauvaises pratiques et féliciter les bonnes actions.',
    }],
})

const { getContributions, partagerContribution } = useGouvernance()
const { pubCible, cibler } = usePartagePublication()
const { redirigerVersConnexion } = useAuth()
const userStore = useUserStore()

type VueActive = 'toutes' | 'mauvaise' | 'bonne'

// ─── État ─────────────────────────────────────────────────────────────────

const contributions = ref<ContributionCitoyenne[]>([])
const chargement = ref(false)
const erreurChargement = ref<string | null>(null)
const vueActive = ref<VueActive>('toutes')
const recherche = ref('')
const paysSelectionne = ref('')
const graviteSelectionnee = ref('')
const impactSelectionne = ref('')
const modalOuvert = ref(false)
const typePratiqueInitial = ref<TypePratique>('mauvaise')
const decouverteOuverte = ref(false)

// Partage vers le mur /publications
const modalPartageOuvert = ref(false)
const contribAPartager = ref<ContributionCitoyenne | null>(null)
const modalPartageRef = ref<{ setLoading: (v: boolean) => void, setError: (m: string) => void, setSuccess: () => void } | null>(null)

// Visionneuse d'images (preuves photos)
const viewerOuvert = ref(false)
const viewerImages = ref<string[]>([])
const viewerIndex = ref(0)
function ouvrirViewer(images: string[], index = 0) {
  viewerImages.value = images
  viewerIndex.value = index
  viewerOuvert.value = true
}

// Séparation des preuves : images (visionneuse) vs documents PDF (lien).
const estPdf = (url: string) => /\.pdf(\?|#|$)/i.test(url)
const imagesPreuves = (c: ContributionCitoyenne) => (c.images ?? []).filter(u => !estPdf(u))
const pdfsPreuves = (c: ContributionCitoyenne) => (c.images ?? []).filter(estPdf)

// ─── Familles et onglets ──────────────────────────────────────────────────

const contributionsMauvaises = computed(() => contributions.value.filter(c => c.typePratique !== 'bonne'))
const contributionsBonnes = computed(() => contributions.value.filter(c => c.typePratique === 'bonne'))

const onglets = computed(() => [
  { valeur: 'toutes' as VueActive, libelle: 'Toutes', icone: 'fa-solid fa-layer-group', compte: contributions.value.length },
  { valeur: 'mauvaise' as VueActive, libelle: 'Badhabits', icone: 'fa-solid fa-triangle-exclamation', compte: contributionsMauvaises.value.length },
  { valeur: 'bonne' as VueActive, libelle: 'Goodhabits', icone: 'fa-solid fa-thumbs-up', compte: contributionsBonnes.value.length }])

const NIVEAUX_GRAVITE = [
  { valeur: 'critique', libelle: 'Critique' },
  { valeur: 'grave', libelle: 'Grave' },
  { valeur: 'moyenne', libelle: 'Moyenne' },
  { valeur: 'faible', libelle: 'Faible' }]

const NIVEAUX_IMPACT = [
  { valeur: 'exemplaire', libelle: 'Exemplaire' },
  { valeur: 'fort', libelle: 'Fort' },
  { valeur: 'moyen', libelle: 'Moyen' },
  { valeur: 'faible', libelle: 'Modeste' }]

const compterParGravite = (gravite: string) =>
  contributionsMauvaises.value.filter(c => c.problematique?.gravite === gravite).length

const compterParImpact = (impact: string) =>
  contributionsBonnes.value.filter(c => c.bonnePratique?.impact === impact).length

// ─── Filtres ──────────────────────────────────────────────────────────────

const paysDisponibles = computed(() =>
  Array.from(new Set(contributions.value.map(c => c.localisation.pays))).sort())

const contributionsFiltrees = computed(() => contributions.value.filter((c) => {
  if (vueActive.value === 'mauvaise' && c.typePratique === 'bonne') return false
  if (vueActive.value === 'bonne' && c.typePratique !== 'bonne') return false

  if (recherche.value) {
    const search = recherche.value.toLowerCase()
    if (!c.titre.toLowerCase().includes(search) && !c.description.toLowerCase().includes(search)) {
      return false
    }
  }
  if (paysSelectionne.value && c.localisation.pays !== paysSelectionne.value) return false
  if (vueActive.value !== 'bonne' && graviteSelectionnee.value && c.problematique?.gravite !== graviteSelectionnee.value) return false
  if (vueActive.value !== 'mauvaise' && impactSelectionne.value && c.bonnePratique?.impact !== impactSelectionne.value) return false
  return true
}))

const filtreActif = computed(() =>
  Boolean(recherche.value || paysSelectionne.value || graviteSelectionnee.value || impactSelectionne.value))

const reinitialiser = () => {
  recherche.value = ''
  paysSelectionne.value = ''
  graviteSelectionnee.value = ''
  impactSelectionne.value = ''
}

// ─── Affichage d'une contribution ─────────────────────────────────────────

const estBonne = (c: ContributionCitoyenne) => c.typePratique === 'bonne'

/** Icône de tête : la famille, pas le niveau. */
const iconeContribution = (c: ContributionCitoyenne) =>
  estBonne(c) ? 'fa-solid fa-thumbs-up' : 'fa-solid fa-triangle-exclamation'

const niveauAffiche = (c: ContributionCitoyenne) => {
  if (estBonne(c)) {
    const impact = c.bonnePratique?.impact
    return impact ? `Impact ${impact}` : null
  }
  return c.problematique?.gravite ?? null
}

const categorieAffichee = (c: ContributionCitoyenne) =>
  estBonne(c) ? c.bonnePratique?.categorie : c.problematique?.categorie

// ─── Actions ──────────────────────────────────────────────────────────────

function ouvrirModalPublication(type: TypePratique = 'mauvaise') {
  if (!userStore.isAuthenticated) return redirigerVersConnexion()
  typePratiqueInitial.value = type
  modalOuvert.value = true
}

function ouvrirPartage(c: ContributionCitoyenne) {
  if (!userStore.isAuthenticated) return redirigerVersConnexion()
  contribAPartager.value = c
  modalPartageOuvert.value = true
}

async function soumettrePartage(legende: string) {
  if (!contribAPartager.value) return
  modalPartageRef.value?.setLoading(true)
  try {
    await partagerContribution('badhabits', contribAPartager.value.id, legende || undefined)
    modalPartageRef.value?.setSuccess()
  }
  catch (e) {
    modalPartageRef.value?.setError(e instanceof Error ? e.message : 'Erreur lors du partage.')
  }
}

// ─── Chargement ───────────────────────────────────────────────────────────

async function chargerContributions() {
  chargement.value = true
  erreurChargement.value = null
  try {
    const resultat = await getContributions({ type: 'badhabits', parPage: 50 })
    contributions.value = resultat.contributions
    cibler(resultat.contributions.map(c => c.id))
  }
  catch (e: unknown) {
    erreurChargement.value = e instanceof Error ? e.message : 'Erreur inconnue'
    contributions.value = []
  }
  finally {
    chargement.value = false
  }
}

function apresPublication() {
  modalOuvert.value = false
  chargerContributions()
}

onMounted(chargerContributions)
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="BadGoodhabits"
        sous-titre="Dénoncer les mauvaises pratiques et féliciter les bonnes actions"
        aide="C'est quoi BadGoodhabits ?"
        @aide="decouverteOuverte = true"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[
          { libelle: 'Novagouv', vers: '/universite/gouvernance' },
          { libelle: 'BadGoodhabits' }]"
      >
        <template #action>
          <div class="flex flex-wrap gap-3">
            <AfricansBouton variante="secondaire" icone="fa-solid fa-thumbs-up" @click="ouvrirModalPublication('bonne')">
              Féliciter
            </AfricansBouton>
            <AfricansBouton icone="fa-solid fa-flag" @click="ouvrirModalPublication('mauvaise')">
              Signaler
            </AfricansBouton>
          </div>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="flex flex-col gap-5">
      <!-- Les trois familles -->
      <div class="flex flex-wrap gap-2">
        <button
          v-for="onglet in onglets"
          :key="onglet.valeur"
          type="button"
          class="flex items-center gap-2 rounded-full px-4 py-2 text-[14px]/[1.4] font-bold transition"
          :class="vueActive === onglet.valeur ? 'bg-af-chocolat text-white' : 'bg-af-fond text-af-corps hover:bg-af-bordure'"
          :aria-pressed="vueActive === onglet.valeur"
          @click="vueActive = onglet.valeur"
        >
          <font-awesome-icon :icon="onglet.icone" />
          {{ onglet.libelle }}
          <span class="text-[12px]/[1.4] opacity-70">{{ onglet.compte }}</span>
        </button>
      </div>

      <p class="text-[14px]/[1.4] text-af-atone">
        <span class="font-bold text-af-encre">{{ contributionsFiltrees.length }}</span>
        résultat{{ contributionsFiltrees.length > 1 ? 's' : '' }}
        <span v-if="filtreActif">(filtré{{ contributionsFiltrees.length > 1 ? 's' : '' }})</span>
      </p>

      <div v-if="chargement" class="flex flex-col gap-5">
        <div v-for="n in 3" :key="n" class="h-52 animate-pulse rounded-[10px] bg-af-bordure" />
      </div>

      <div v-else-if="erreurChargement" class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
        <font-awesome-icon icon="fa-solid fa-triangle-exclamation" class="text-4xl text-af-live" />
        <p class="mt-4 text-[16px]/[1.4] font-bold">Impossible de charger les contributions</p>
        <p class="mt-2 text-[14px]/[1.4] text-af-corps">{{ erreurChargement }}</p>
        <AfricansBouton class="mt-6" variante="secondaire" icone="fa-solid fa-rotate-left" @click="chargerContributions">
          Réessayer
        </AfricansBouton>
      </div>

      <div v-else-if="!contributionsFiltrees.length" class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
        <font-awesome-icon icon="fa-solid fa-magnifying-glass" class="text-4xl text-af-atone-2" />
        <p class="mt-4 text-[16px]/[1.4] font-bold">Aucun résultat trouvé</p>
        <p class="mt-2 text-[14px]/[1.4] text-af-corps">
          {{ filtreActif ? 'Essayez de modifier vos filtres de recherche.' : 'Les contributions publiées apparaîtront ici.' }}
        </p>
      </div>

      <div v-else class="flex flex-col gap-5">
        <UniversiteGouvernanceCarteContribution
          v-for="contribution in contributionsFiltrees"
          :key="contribution.id"
          :contribution="contribution"
          :icone="iconeContribution(contribution)"
          chemin="/universite/gouvernance/bad-good-habits"
          type-objet="bad_habit"
          :ciblee="pubCible === contribution.id"
        >
          <div class="flex flex-wrap items-center gap-2">
            <AfricansEtiquette :ton="estBonne(contribution) ? 'vert' : 'gris'">
              {{ estBonne(contribution) ? 'Goodhabits' : 'Badhabits' }}
            </AfricansEtiquette>
            <AfricansEtiquette v-if="niveauAffiche(contribution)" class="capitalize">
              {{ niveauAffiche(contribution) }}
            </AfricansEtiquette>
            <AfricansEtiquette v-if="categorieAffichee(contribution)">
              {{ categorieAffichee(contribution) }}
            </AfricansEtiquette>
          </div>

          <!-- Preuves : photos dans la visionneuse, PDF en lien -->
          <div v-if="contribution.images?.length" class="flex flex-wrap gap-2">
            <button
              v-for="(img, idx) in imagesPreuves(contribution)"
              :key="img"
              type="button"
              class="size-20 cursor-zoom-in overflow-hidden rounded-[10px] border border-af-bordure"
              @click.stop="ouvrirViewer(imagesPreuves(contribution), idx)"
            >
              <img :src="img" alt="Preuve" class="size-full object-cover transition hover:scale-105" />
            </button>
            <a
              v-for="pdf in pdfsPreuves(contribution)"
              :key="pdf"
              :href="pdf"
              target="_blank"
              rel="noopener noreferrer"
              class="flex size-20 flex-col items-center justify-center gap-1 rounded-[10px] border border-af-bordure bg-af-fond text-af-live transition hover:border-af-live"
              @click.stop
            >
              <font-awesome-icon icon="fa-solid fa-file-pdf" class="text-2xl" />
              <span class="text-[10px] font-bold">PDF</span>
            </a>
          </div>

          <template #pied>
            <span class="flex items-center gap-1.5 px-2.5 py-1">
              <font-awesome-icon :icon="estBonne(contribution) ? 'fa-solid fa-hands-clapping' : 'fa-solid fa-hand-fist'" />
              {{ contribution.stats.soutiens || 0 }}
              {{ estBonne(contribution) ? 'félicitation' : 'soutien' }}{{ (contribution.stats.soutiens || 0) > 1 ? 's' : '' }}
            </span>
            <button
              type="button"
              title="Partager sur le mur /publications"
              class="flex items-center gap-1.5 rounded-full px-2.5 py-1 transition hover:bg-af-fond hover:text-af-vert"
              @click.stop="ouvrirPartage(contribution)"
            >
              <font-awesome-icon icon="fa-solid fa-share-nodes" />
              <span class="hidden sm:inline">Partager sur le mur</span>
            </button>
          </template>
        </UniversiteGouvernanceCarteContribution>
      </div>
    </div>

    <template #rail>
      <AfricansPanneau titre="Filtres" icone="fa-solid fa-sliders" action-libelle="Réinitialiser" @action="reinitialiser">
        <div class="flex flex-col gap-5">
          <label class="relative block">
            <span class="sr-only">Rechercher</span>
            <font-awesome-icon
              icon="fa-solid fa-magnifying-glass"
              class="pointer-events-none absolute top-1/2 left-3 -translate-y-1/2 text-af-atone-2"
            />
            <input
              v-model="recherche"
              type="search"
              placeholder="Rechercher…"
              class="h-10 w-full rounded-[10px] border border-af-bordure bg-white pr-3 pl-9 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
            />
          </label>

          <label class="flex flex-col gap-2">
            <span class="text-[12px]/[1.4] font-bold text-af-atone uppercase">Territoire</span>
            <select
              v-model="paysSelectionne"
              class="h-10 w-full rounded-[10px] border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:outline-2 focus:outline-af-chocolat"
            >
              <option value="">Tous les territoires</option>
              <option v-for="pays in paysDisponibles" :key="pays" :value="pays">{{ pays }}</option>
            </select>
          </label>

          <!-- La gravité ne concerne que les Badhabits : elle disparaît dans la
               vue Goodhabits, où elle ne filtrerait rien. -->
          <div v-if="vueActive !== 'bonne'" class="flex flex-col gap-2">
            <p class="text-[12px]/[1.4] font-bold text-af-atone uppercase">Gravité</p>
            <div class="flex flex-wrap gap-2">
              <button
                v-for="n in NIVEAUX_GRAVITE"
                :key="n.valeur"
                type="button"
                class="rounded-full px-3 py-1.5 text-[12px]/[1.4] font-bold transition"
                :class="graviteSelectionnee === n.valeur ? 'bg-af-chocolat text-white' : 'bg-af-fond text-af-corps hover:bg-af-bordure'"
                @click="graviteSelectionnee = graviteSelectionnee === n.valeur ? '' : n.valeur"
              >
                {{ n.libelle }} ({{ compterParGravite(n.valeur) }})
              </button>
            </div>
          </div>

          <div v-if="vueActive !== 'mauvaise'" class="flex flex-col gap-2">
            <p class="text-[12px]/[1.4] font-bold text-af-atone uppercase">Impact</p>
            <div class="flex flex-wrap gap-2">
              <button
                v-for="n in NIVEAUX_IMPACT"
                :key="n.valeur"
                type="button"
                class="rounded-full px-3 py-1.5 text-[12px]/[1.4] font-bold transition"
                :class="impactSelectionne === n.valeur ? 'bg-af-vert text-white' : 'bg-af-fond text-af-corps hover:bg-af-bordure'"
                @click="impactSelectionne = impactSelectionne === n.valeur ? '' : n.valeur"
              >
                {{ n.libelle }} ({{ compterParImpact(n.valeur) }})
              </button>
            </div>
          </div>
        </div>
      </AfricansPanneau>
    </template>

    <UniversiteGouvernanceBadHabitsCreateModal
      :open="modalOuvert"
      :type-pratique-initial="typePratiqueInitial"
      @close="modalOuvert = false"
      @created="apresPublication"
    />

    <UniversiteGouvernancePartagerContributionModal
      ref="modalPartageRef"
      :is-open="modalPartageOuvert"
      :titre="contribAPartager?.titre ?? ''"
      @close="modalPartageOuvert = false"
      @submit="soumettrePartage"
    />

    <CommonImageViewer
      :images="viewerImages"
      :open="viewerOuvert"
      :index="viewerIndex"
      @close="viewerOuvert = false"
    />

    <UniversiteGouvernanceDecouverteBadGoodhabits v-model="decouverteOuverte" />
  </NuxtLayout>
</template>
