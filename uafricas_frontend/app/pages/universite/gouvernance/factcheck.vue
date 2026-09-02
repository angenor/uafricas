<script setup lang="ts">
import type { ContributionCitoyenne, TypePublicationFactcheck, TypeReactionGlobale } from '~/types/gouvernance'
import { useUserStore } from '~/stores/user'

/**
 * Factcheck : porté sur le gabarit de la refonte.
 *
 * Toute la logique est conservée : garde de séquence au chargement, réactions
 * globales et par volet appliquées de façon immuable, signalement avec retrait
 * de la liste au-delà du seuil, partage sur le mur, visionneuse d'images.
 *
 * Changent : la palette bleu / indigo cède aux jetons communs, les filtres
 * passent dans le rail, la modale « C'est quoi Factcheck ? » : 110 lignes
 * recopiées dans le template : devient `DecouverteFactcheck`, et la carte
 * s'appuie sur `CarteContribution`, partagée avec les deux autres espaces.
 */
definePageMeta({ layout: false })

useHead({
  title: 'Factcheck : Gouvernance citoyenne | AfricanS',
  meta: [
    {
      name: 'description',
      content: "Vérifier ensemble les idées reçues et les fausses informations qui circulent sur l'Afrique.",
    }],
})

const { getContributions, reagir, signaler, partagerContribution } = useGouvernance()
const { pubCible, cibler } = usePartagePublication()
const { redirigerVersConnexion } = useAuth()
const userStore = useUserStore()

// ─── État ─────────────────────────────────────────────────────────────────

const contributions = ref<ContributionCitoyenne[]>([])
const recherche = ref('')
const paysSelectionne = ref('')
const seulementVerifies = ref(false)
const modalOuvert = ref(false)
const chargement = ref(false)
const erreurReaction = ref<string | null>(null)
const messageInfo = ref<string | null>(null)
const decouverteOuverte = ref(false)

// Partage vers le mur /publications
const modalPartageOuvert = ref(false)
const contribAPartager = ref<ContributionCitoyenne | null>(null)
const modalPartageRef = ref<{ setLoading: (v: boolean) => void, setError: (m: string) => void, setSuccess: () => void } | null>(null)

// Visionneuse d'images
const viewerOuvert = ref(false)
const viewerImages = ref<string[]>([])
function ouvrirViewer(images: string[]) {
  viewerImages.value = images
  viewerOuvert.value = true
}

// ─── Type de publication ──────────────────────────────────────────────────

/** Libellé et icône du type de publication. */
function typeBadge(type: TypePublicationFactcheck | undefined): { label: string, icone: string } {
  switch (type) {
    case 'adage_legende': return { label: 'Adage / Légende', icone: 'fa-solid fa-book-open' }
    case 'fait_vecu': return { label: 'Fait vécu ou observé', icone: 'fa-solid fa-location-dot' }
    default: return { label: 'On dit / entendu', icone: 'fa-solid fa-comments' }
  }
}

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
  if (seulementVerifies.value && !c.verified) return false
  return true
}))

const filtreActif = computed(() => Boolean(recherche.value || paysSelectionne.value || seulementVerifies.value))

const reinitialiser = () => {
  recherche.value = ''
  paysSelectionne.value = ''
  seulementVerifies.value = false
}

// ─── Réactions ────────────────────────────────────────────────────────────

const REACTIONS: { type: TypeReactionGlobale, icone: string, label: string }[] = [
  { type: 'coeur', icone: 'fa-solid fa-heart', label: 'Cœur' },
  { type: 'pouce', icone: 'fa-solid fa-thumbs-up', label: 'J\'aime' },
  { type: 'rire', icone: 'fa-solid fa-face-laugh-squint', label: 'Ça me fait rire' },
  { type: 'jaime_pas', icone: 'fa-solid fa-thumbs-down', label: 'Je n\'aime pas' }]

function nombreReaction(c: ContributionCitoyenne, type: TypeReactionGlobale): number {
  const r = c.reactions
  if (!r) return 0
  switch (type) {
    case 'coeur': return r.coeur
    case 'pouce': return r.pouce
    case 'rire': return r.rire
    case 'jaime_pas': return r.jaimePas
  }
}

/** Remplace immuablement la contribution avec le nouvel état de réaction. */
function appliquerEtat(id: string, etat: Awaited<ReturnType<typeof reagir>>) {
  const index = contributions.value.findIndex(c => c.id === id)
  if (index === -1) return
  const actuelle = contributions.value[index]!
  const maj: ContributionCitoyenne = {
    ...actuelle,
    reactions: etat.reactions,
    aLikePrejuge: etat.aLikePrejuge,
    aLikeRealite: etat.aLikeRealite,
    factcheck: actuelle.factcheck
      ? {
          prejuge: { ...actuelle.factcheck.prejuge, likes: etat.prejugeLikes },
          contrePrejuge: { ...actuelle.factcheck.contrePrejuge, likes: etat.realiteLikes },
        }
      : undefined,
  }
  const copie = [...contributions.value]
  copie[index] = maj
  contributions.value = copie
}

async function reagirGlobal(c: ContributionCitoyenne, type: TypeReactionGlobale) {
  if (!userStore.isAuthenticated) return redirigerVersConnexion()
  erreurReaction.value = null
  try {
    appliquerEtat(c.id, await reagir(c.id, 'general', type))
  }
  catch (err) {
    erreurReaction.value = err instanceof Error ? err.message : 'Erreur lors de la réaction'
  }
}

async function reagirVolet(c: ContributionCitoyenne, cible: 'prejuge' | 'realite') {
  if (!userStore.isAuthenticated) return redirigerVersConnexion()
  erreurReaction.value = null
  try {
    appliquerEtat(c.id, await reagir(c.id, cible))
  }
  catch (err) {
    erreurReaction.value = err instanceof Error ? err.message : 'Erreur lors de la réaction'
  }
}

// ─── Signalement ──────────────────────────────────────────────────────────

async function signalerContribution(c: ContributionCitoyenne) {
  if (!userStore.isAuthenticated) return redirigerVersConnexion()
  if (c.aSignale) return
  if (!confirm('Signaler cette publication comme inappropriée ou trompeuse ?')) return
  erreurReaction.value = null
  try {
    const etat = await signaler(c.id)
    if (etat.suspendu) {
      // Suspendue : elle quitte la liste publique.
      contributions.value = contributions.value.filter(x => x.id !== c.id)
      messageInfo.value = 'Publication suspendue : elle a dépassé le seuil de signalements.'
    }
    else {
      const index = contributions.value.findIndex(x => x.id === c.id)
      if (index !== -1) {
        const copie = [...contributions.value]
        copie[index] = { ...contributions.value[index]!, aSignale: true }
        contributions.value = copie
      }
      messageInfo.value = etat.dejaSignale
        ? 'Vous aviez déjà signalé cette publication.'
        : 'Merci, votre signalement a été pris en compte.'
    }
    setTimeout(() => { messageInfo.value = null }, 4000)
  }
  catch (err) {
    erreurReaction.value = err instanceof Error ? err.message : 'Erreur lors du signalement'
  }
}

// ─── Partage sur le mur ───────────────────────────────────────────────────

function ouvrirPartage(c: ContributionCitoyenne) {
  if (!userStore.isAuthenticated) return redirigerVersConnexion()
  contribAPartager.value = c
  modalPartageOuvert.value = true
}

async function soumettrePartage(legende: string) {
  if (!contribAPartager.value) return
  modalPartageRef.value?.setLoading(true)
  try {
    await partagerContribution('factcheck', contribAPartager.value.id, legende || undefined)
    modalPartageRef.value?.setSuccess()
  }
  catch (e) {
    modalPartageRef.value?.setError(e instanceof Error ? e.message : 'Erreur lors du partage.')
  }
}

// ─── Publication ──────────────────────────────────────────────────────────

function ouvrirModalPublication() {
  if (!userStore.isAuthenticated) return redirigerVersConnexion()
  modalOuvert.value = true
}

// ─── Chargement ───────────────────────────────────────────────────────────

// Garde de séquence : le chargement anonyme (onMounted) et le rechargement
// authentifié (après restauration du token) peuvent se courser. Seule la
// requête la plus récente applique son résultat, pour ne pas écraser l'état
// personnalisé (ma_reaction / a_like_*) par une réponse anonyme tardive.
let chargementSeq = 0
async function chargerContributions() {
  const seq = ++chargementSeq
  chargement.value = true
  try {
    const { contributions: liste } = await getContributions({ type: 'factcheck', parPage: 50 })
    if (seq !== chargementSeq) return
    contributions.value = liste
    cibler(liste.map(c => c.id))
  }
  catch (err) {
    if (seq === chargementSeq) {
      erreurReaction.value = err instanceof Error ? err.message : 'Erreur lors du chargement'
    }
  }
  finally {
    if (seq === chargementSeq) chargement.value = false
  }
}

function apresPublicationRecharger() {
  modalOuvert.value = false
  chargerContributions()
}

onMounted(chargerContributions)

// Le jeton d'accès est restauré en mémoire de façon asynchrone après le
// montage. Dès qu'il devient disponible, on recharge pour récupérer l'état
// personnalisé (ma_reaction / a_like_*) sans lequel les surbrillances manquent.
watch(() => userStore.accessToken, (token, ancien) => {
  if (token && !ancien) chargerContributions()
})
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Factcheck"
        sous-titre="Vérifier des idées reçues sur l'Afrique"
        aide="C'est quoi Factcheck ?"
        @aide="decouverteOuverte = true"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[
          { libelle: 'Novagouv', vers: '/universite/gouvernance' },
          { libelle: 'Factcheck' }]"
      >
        <template #action>
          <AfricansBouton icone="fa-solid fa-plus" @click="ouvrirModalPublication">
            Publier un factcheck
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

      <p v-if="erreurReaction" role="alert" class="flex items-center gap-2 rounded-[10px] bg-af-live/10 p-3 text-[14px]/[1.4] text-af-live">
        <font-awesome-icon icon="fa-solid fa-circle-exclamation" />
        {{ erreurReaction }}
      </p>

      <p v-if="messageInfo" class="flex items-center gap-2 rounded-[10px] bg-af-chocolat/10 p-3 text-[14px]/[1.4] text-af-chocolat">
        <font-awesome-icon icon="fa-solid fa-flag" />
        {{ messageInfo }}
      </p>

      <div v-if="chargement && !contributions.length" class="flex flex-col gap-5">
        <div v-for="n in 3" :key="n" class="h-56 animate-pulse rounded-[10px] bg-af-bordure" />
      </div>

      <div v-else-if="!contributionsFiltrees.length" class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
        <font-awesome-icon icon="fa-solid fa-magnifying-glass" class="text-4xl text-af-atone-2" />
        <p class="mt-4 text-[16px]/[1.4] font-bold">Aucun résultat trouvé</p>
        <p class="mt-2 text-[14px]/[1.4] text-af-corps">
          {{ filtreActif ? 'Essayez de modifier vos filtres de recherche.' : 'Les vérifications publiées apparaîtront ici.' }}
        </p>
      </div>

      <div v-else class="flex flex-col gap-5">
        <UniversiteGouvernanceCarteContribution
          v-for="contribution in contributionsFiltrees"
          :key="contribution.id"
          :contribution="contribution"
          :icone="contribution.verified ? 'fa-solid fa-circle-check' : 'fa-solid fa-scale-balanced'"
          chemin="/universite/gouvernance/factcheck"
          type-objet="factcheck"
          :ciblee="pubCible === contribution.id"
          @agrandir-image="ouvrirViewer"
        >
          <!-- Type de publication + preuve (fait vécu) -->
          <div v-if="contribution.typePublication" class="flex flex-wrap items-center gap-2">
            <AfricansEtiquette>
              <font-awesome-icon :icon="typeBadge(contribution.typePublication).icone" class="mr-1.5" />
              {{ typeBadge(contribution.typePublication).label }}
            </AfricansEtiquette>

            <template v-if="contribution.typePublication === 'fait_vecu'">
              <button
                v-if="contribution.preuveUrl && contribution.preuveType === 'image'"
                type="button"
                class="rounded-full bg-af-vert/10 px-2.5 py-1 text-[12px]/[1.4] font-bold text-af-vert transition hover:bg-af-vert/20"
                @click.stop="ouvrirViewer([contribution.preuveUrl!])"
              >
                <font-awesome-icon icon="fa-solid fa-image" class="mr-1.5" />
                Voir la preuve (photo)
              </button>
              <a
                v-else-if="contribution.preuveUrl"
                :href="contribution.preuveUrl"
                target="_blank"
                rel="noopener noreferrer"
                class="rounded-full bg-af-vert/10 px-2.5 py-1 text-[12px]/[1.4] font-bold text-af-vert transition hover:bg-af-vert/20"
                @click.stop
              >
                <font-awesome-icon icon="fa-solid fa-file-pdf" class="mr-1.5" />
                Preuve (PDF)
              </a>
              <AfricansEtiquette v-else>
                <font-awesome-icon icon="fa-solid fa-ban" class="mr-1.5" />
                Pas de preuve
              </AfricansEtiquette>
            </template>
          </div>

          <!-- Les deux volets : préjugé et réalité -->
          <div v-if="contribution.factcheck" class="grid gap-3 md:grid-cols-2">
            <div class="flex flex-col rounded-[10px] border-l-4 border-af-live bg-af-live/5 p-3">
              <p class="text-[12px]/[1.4] font-bold text-af-live uppercase">
                <font-awesome-icon icon="fa-solid fa-xmark" class="mr-1" />Préjugé
              </p>
              <p class="mt-1 line-clamp-2 flex-1 text-[14px]/[1.4] text-af-encre">
                {{ contribution.factcheck.prejuge.titre }}
              </p>
              <button
                type="button"
                class="mt-2 self-start rounded-full px-2.5 py-1 text-[12px]/[1.4] font-bold transition"
                :class="contribution.aLikePrejuge ? 'bg-af-live text-white' : 'bg-white text-af-live hover:bg-af-live/10'"
                @click.stop="reagirVolet(contribution, 'prejuge')"
              >
                <font-awesome-icon icon="fa-solid fa-heart" class="mr-1.5" />
                {{ contribution.factcheck.prejuge.likes || 0 }}
              </button>
            </div>

            <div class="flex flex-col rounded-[10px] border-l-4 border-af-vert bg-af-vert/5 p-3">
              <p class="text-[12px]/[1.4] font-bold text-af-vert uppercase">
                <font-awesome-icon icon="fa-solid fa-check" class="mr-1" />Réalité
              </p>
              <p class="mt-1 line-clamp-2 flex-1 text-[14px]/[1.4] text-af-encre">
                {{ contribution.factcheck.contrePrejuge.titre }}
              </p>
              <button
                type="button"
                class="mt-2 self-start rounded-full px-2.5 py-1 text-[12px]/[1.4] font-bold transition"
                :class="contribution.aLikeRealite ? 'bg-af-vert text-white' : 'bg-white text-af-vert hover:bg-af-vert/10'"
                @click.stop="reagirVolet(contribution, 'realite')"
              >
                <font-awesome-icon icon="fa-solid fa-heart" class="mr-1.5" />
                {{ contribution.factcheck.contrePrejuge.likes || 0 }}
              </button>
            </div>
          </div>

          <!-- Sources : elles appuient la réalité -->
          <div v-if="contribution.sources?.length" class="flex flex-wrap items-center gap-2">
            <span class="flex items-center gap-1.5 text-[12px]/[1.4] font-bold text-af-vert uppercase">
              <font-awesome-icon icon="fa-solid fa-link" />
              Sources
            </span>
            <a
              v-for="(source, index) in contribution.sources"
              :key="index"
              :href="source.url"
              target="_blank"
              rel="noopener noreferrer"
              class="rounded-full bg-af-vert/10 px-2.5 py-1 text-[12px]/[1.4] font-bold text-af-vert transition hover:bg-af-vert/20"
              @click.stop
            >
              <font-awesome-icon icon="fa-solid fa-circle-check" class="mr-1.5" />
              {{ source.titre }}
            </a>
          </div>

          <template #pied>
            <button
              v-for="r in REACTIONS"
              :key="r.type"
              type="button"
              :title="r.label"
              class="flex items-center gap-1.5 rounded-full px-2.5 py-1 transition hover:bg-af-fond"
              :class="contribution.reactions?.maReaction === r.type && 'bg-af-fond font-bold text-af-chocolat'"
              @click.stop="reagirGlobal(contribution, r.type)"
            >
              <font-awesome-icon :icon="r.icone" />
              {{ nombreReaction(contribution, r.type) }}
            </button>

            <button
              type="button"
              :title="contribution.aSignale ? 'Vous avez signalé cette publication' : 'Signaler cette publication'"
              :disabled="contribution.aSignale"
              class="flex items-center gap-1.5 rounded-full px-2.5 py-1 transition"
              :class="contribution.aSignale ? 'font-bold text-af-live' : 'hover:bg-af-fond hover:text-af-live'"
              @click.stop="signalerContribution(contribution)"
            >
              <font-awesome-icon icon="fa-solid fa-flag" />
              {{ contribution.aSignale ? 'Signalé' : 'Signaler' }}
            </button>

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

          <label class="flex cursor-pointer items-center gap-3 text-[14px]/[1.4]">
            <input v-model="seulementVerifies" type="checkbox" class="size-4 accent-af-chocolat" />
            Vérifiés uniquement
          </label>
        </div>
      </AfricansPanneau>
    </template>

    <UniversiteGouvernanceFactCheckCreateModal
      :open="modalOuvert"
      @close="modalOuvert = false"
      @created="apresPublicationRecharger"
    />

    <UniversiteGouvernancePartagerContributionModal
      ref="modalPartageRef"
      :is-open="modalPartageOuvert"
      :titre="contribAPartager?.titre ?? ''"
      @close="modalPartageOuvert = false"
      @submit="soumettrePartage"
    />

    <CommonImageViewer :images="viewerImages" :open="viewerOuvert" @close="viewerOuvert = false" />

    <UniversiteGouvernanceDecouverteFactcheck v-model="decouverteOuverte" />
  </NuxtLayout>
</template>
