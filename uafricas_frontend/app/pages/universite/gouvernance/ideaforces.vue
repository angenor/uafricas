<script setup lang="ts">
import type { ContributionCitoyenne } from '~/types/gouvernance'
import { useUserStore } from '~/stores/user'

/**
 * Ideaforces : porté sur le gabarit de la refonte.
 *
 * La logique est conservée : chargement, filtres locaux, publication et partage
 * sur le mur. La palette ambre / orange cède aux jetons communs, les filtres
 * passent dans le rail, la modale de présentation devient
 * `DecouverteIdeaforces` et la carte s'appuie sur `CarteContribution`.
 *
 * Le pied de carte perd le compteur « N vues » : `useGouvernance` écrit
 * `vues: 0` en dur : l'API ne renvoie aucune vue, si bien que chaque idée
 * affichait « 0 vues » depuis toujours. « Likes » et « soutiens », eux, sont
 * de vraies valeurs et restent.
 */
definePageMeta({ layout: false })

useHead({
  title: 'Ideaforces : Gouvernance citoyenne | AfricanS',
  meta: [
    {
      name: 'description',
      content: "Partager des idées et des orientations sur les enjeux de développement du continent.",
    }],
})

const userStore = useUserStore()
const { getContributions, partagerContribution } = useGouvernance()
const { pubCible, cibler } = usePartagePublication()
const { redirigerVersConnexion } = useAuth()

// ─── État ─────────────────────────────────────────────────────────────────

const contributions = ref<ContributionCitoyenne[]>([])
const chargement = ref(false)
const erreurChargement = ref<string | null>(null)
const recherche = ref('')
const paysSelectionne = ref('')
const modalOuvert = ref(false)
const decouverteOuverte = ref(false)

// Partage vers le mur /publications
const modalPartageOuvert = ref(false)
const contribAPartager = ref<ContributionCitoyenne | null>(null)
const modalPartageRef = ref<{ setLoading: (v: boolean) => void, setError: (m: string) => void, setSuccess: () => void } | null>(null)

// ─── Filtres ──────────────────────────────────────────────────────────────

const paysDisponibles = computed(() =>
  Array.from(new Set(contributions.value.map(c => c.localisation.pays))).sort())

const contributionsFiltrees = computed(() => contributions.value.filter((c) => {
  if (recherche.value) {
    const search = recherche.value.toLowerCase()
    if (!c.titre.toLowerCase().includes(search) && !c.description.toLowerCase().includes(search)) {
      return false
    }
  }
  if (paysSelectionne.value && c.localisation.pays !== paysSelectionne.value) return false
  return true
}))

const filtreActif = computed(() => Boolean(recherche.value || paysSelectionne.value))

const reinitialiser = () => {
  recherche.value = ''
  paysSelectionne.value = ''
}

// ─── Actions ──────────────────────────────────────────────────────────────

function ouvrirModalPublication() {
  if (!userStore.isAuthenticated) return redirigerVersConnexion()
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
    await partagerContribution('ideaforces', contribAPartager.value.id, legende || undefined)
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
    const { contributions: liste } = await getContributions({ type: 'ideaforces', parPage: 50 })
    contributions.value = liste
    cibler(liste.map(c => c.id))
  }
  catch (err) {
    erreurChargement.value = err instanceof Error ? err.message : 'Erreur lors du chargement'
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
        titre="Ideaforces"
        sous-titre="Partager des idées et des orientations sur les enjeux de développement"
        aide="C'est quoi Ideaforces ?"
        @aide="decouverteOuverte = true"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[
          { libelle: 'Novagouv', vers: '/universite/gouvernance' },
          { libelle: 'Ideaforces' }]"
      >
        <template #action>
          <AfricansBouton icone="fa-solid fa-plus" @click="ouvrirModalPublication">
            Proposer une idée
          </AfricansBouton>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="flex flex-col gap-5">
      <p class="text-[14px]/[1.4] text-af-atone">
        <span class="font-bold text-af-encre">{{ contributionsFiltrees.length }}</span>
        résultat{{ contributionsFiltrees.length > 1 ? 's' : '' }}
        <span v-if="filtreActif">(filtré{{ contributionsFiltrees.length > 1 ? 's' : '' }})</span>
      </p>

      <p v-if="erreurChargement" role="alert" class="flex items-center gap-2 rounded-[10px] bg-af-live/10 p-3 text-[14px]/[1.4] text-af-live">
        <font-awesome-icon icon="fa-solid fa-circle-exclamation" />
        {{ erreurChargement }}
      </p>

      <div v-if="chargement && !contributions.length" class="flex flex-col gap-5">
        <div v-for="n in 3" :key="n" class="h-48 animate-pulse rounded-[10px] bg-af-bordure" />
      </div>

      <div v-else-if="!contributionsFiltrees.length" class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
        <font-awesome-icon icon="fa-solid fa-lightbulb" class="text-4xl text-af-atone-2" />
        <p class="mt-4 text-[16px]/[1.4] font-bold">Aucune idée trouvée</p>
        <p class="mt-2 text-[14px]/[1.4] text-af-corps">
          {{ filtreActif ? 'Essayez de modifier vos filtres de recherche.' : 'Les propositions publiées apparaîtront ici.' }}
        </p>
      </div>

      <div v-else class="flex flex-col gap-5">
        <UniversiteGouvernanceCarteContribution
          v-for="contribution in contributionsFiltrees"
          :key="contribution.id"
          :contribution="contribution"
          icone="fa-solid fa-lightbulb"
          chemin="/universite/gouvernance/ideaforces"
          type-objet="idea_force"
          :ciblee="pubCible === contribution.id"
        >
          <div v-if="contribution.proposition" class="rounded-[10px] border border-af-bordure bg-af-fond p-4">
            <p class="flex items-center gap-2 text-[12px]/[1.4] font-bold text-af-chocolat uppercase">
              <font-awesome-icon icon="fa-solid fa-rocket" />
              Objectif
            </p>
            <p class="mt-2 line-clamp-2 text-[14px]/[1.4] text-af-encre">
              {{ contribution.proposition.objectif }}
            </p>

            <div v-if="contribution.proposition.beneficiaires?.length" class="mt-3 flex flex-wrap gap-2">
              <AfricansEtiquette v-for="b in contribution.proposition.beneficiaires" :key="b">
                {{ b }}
              </AfricansEtiquette>
            </div>
          </div>

          <template #pied>
            <span class="flex items-center gap-1.5 px-2.5 py-1">
              <font-awesome-icon icon="fa-solid fa-heart" />
              {{ contribution.stats.likes }} like{{ contribution.stats.likes > 1 ? 's' : '' }}
            </span>
            <span class="flex items-center gap-1.5 px-2.5 py-1">
              <font-awesome-icon icon="fa-solid fa-hand-fist" />
              {{ contribution.stats.soutiens || 0 }} soutien{{ (contribution.stats.soutiens || 0) > 1 ? 's' : '' }}
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
        </div>
      </AfricansPanneau>
    </template>

    <UniversiteGouvernanceIdeaForcesCreateModal
      :open="modalOuvert"
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

    <UniversiteGouvernanceDecouverteIdeaforces v-model="decouverteOuverte" />
  </NuxtLayout>
</template>
