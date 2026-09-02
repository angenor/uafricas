<script setup lang="ts">
import type { AvisPublicResume, PaginationInfo, PaysInfo, TypeRelationRecherche } from '~/composables/useRetrouvAmis'
import { TYPES_RELATION } from '~/composables/useRetrouvAmis'
import { NOMS_PAYS_FR, PALIERS_CHALEUR, normaliserNomPays } from '~/utils/carteAfrique'

/**
 * Africonnect : avis de recherche, porté sur le gabarit de la refonte.
 *
 * Données inchangées : mêmes endpoints, mêmes filtres serveur, même carte de
 * chaleur. La grille de quatre vignettes devient le FIL de cartes pleine
 * largeur de la maquette, les filtres et le tableau de bord passent au rail.
 *
 * Le fil d'Ariane suit la NAVIGATION (Opafrica), pas le fil d'Ariane dessiné :
 * la maquette écrit « Africarise / Africonnect » alors que sa propre barre
 * latérale surligne Opafrica. Les deux ne peuvent pas être vrais.
 */
definePageMeta({ layout: false })

useHead({
  title: 'Africonnect : retrouver une personne perdue de vue | AfricanS',
  meta: [
    {
      name: 'description',
      content: 'Retrouvez vos amis, proches et connaissances perdus de vue grâce à la communauté panafricaine.',
    }],
})

const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string
const userStore = useUserStore()
const { tableauDeBord, basculerTrouvable, rechercherAvisPublics, incrementerPartage } = useRetrouvAmis()

const estConnecte = computed(() => userStore.isAuthenticated)

// Modale de présentation « C'est quoi Africonnect ? »
const presentationOuverte = ref(false)

// ── Mode d'affichage : liste ou carte ─────────────────────
const viewMode = ref<'liste' | 'carte'>('liste')

// ── Filtres ───────────────────────────────────────────────
const filtreRelation = ref<TypeRelationRecherche | ''>('')
const filtreRecherche = ref('')
const filtrePaysId = ref('')
const filtresActifs = computed(() =>
  filtreRelation.value !== '' || filtreRecherche.value.trim() !== '' || filtrePaysId.value !== '')

// ── Avis publics ──────────────────────────────────────────
const avisPublics = ref<AvisPublicResume[]>([])
const pagination = ref<PaginationInfo | null>(null)
const chargementAvis = ref(false)
const pageActuelle = ref(1)

const chargerAvisPublics = async (page: number = 1) => {
  chargementAvis.value = true
  try {
    const params: Record<string, any> = { page, par_page: 12 }
    if (filtreRelation.value) params.type_relation = filtreRelation.value
    if (filtreRecherche.value.trim()) params.recherche = filtreRecherche.value.trim()
    if (filtrePaysId.value) params.pays_id = filtrePaysId.value

    const res = await rechercherAvisPublics(params)
    if (res) {
      avisPublics.value = res.avis
      pagination.value = res.pagination
      pageActuelle.value = page
    }
  }
  finally {
    chargementAvis.value = false
  }
}

/**
 * La recherche s'applique à la frappe, amortie, l'ancienne page exigeait un
 * clic sur « Rechercher », seule de tous les modules à le faire.
 */
let rechercheTimer: ReturnType<typeof setTimeout> | null = null
watch(filtreRecherche, () => {
  if (rechercheTimer) clearTimeout(rechercheTimer)
  rechercheTimer = setTimeout(() => chargerAvisPublics(1), 300)
})

watch([filtreRelation, filtrePaysId], () => chargerAvisPublics(1))

const reinitialiserFiltres = () => {
  filtreRelation.value = ''
  filtreRecherche.value = ''
  filtrePaysId.value = ''
}

// ── Partage d'un avis ─────────────────────────────────────
const messageCopie = ref<string | null>(null)

const partagerAvis = async (avis: AvisPublicResume) => {
  if (navigator.clipboard) {
    navigator.clipboard.writeText(`${window.location.origin}/retrouve-amis/public/${avis.slug}`)
  }
  messageCopie.value = 'Lien de l\'avis copié.'
  setTimeout(() => { messageCopie.value = null }, 2500)

  // Le compteur serveur est incrémenté ET reporté localement : recharger la
  // liste entière pour un seul chiffre ferait sauter la position de lecture.
  const res = await incrementerPartage(avis.slug)
  if (!res) return
  const i = avisPublics.value.findIndex(a => a.id === avis.id)
  if (i !== -1) avisPublics.value[i] = { ...avisPublics.value[i]!, compteur_partages: res.compteur_partages }
}

// ── Statut « trouvable » utilisateur ──────────────────────
const estTrouvable = ref(false)
const chargementTrouvable = ref(false)

const chargerStatutTrouvable = async () => {
  if (!estConnecte.value) return
  try {
    const res = await tableauDeBord()
    estTrouvable.value = res?.est_trouvable ?? false
  }
  catch {
    // silencieux sur page publique
  }
}

const onActiverTrouvable = async () => {
  chargementTrouvable.value = true
  try {
    const res = await basculerTrouvable(!estTrouvable.value)
    if (res) estTrouvable.value = res.est_trouvable
  }
  finally {
    chargementTrouvable.value = false
  }
}

const ETAPES = [
  {
    icone: 'fa-solid fa-pen-to-square',
    titre: 'Déposez un avis',
    description: 'Décrivez la personne que vous recherchez : nom, lieu de dernière rencontre, époque, détails physiques ou anecdotes.',
  },
  {
    icone: 'fa-solid fa-magnifying-glass',
    titre: 'Le système compare',
    description: 'Notre algorithme croise votre avis avec les profils et autres avis de recherche pour identifier des correspondances potentielles.',
  },
  {
    icone: 'fa-solid fa-handshake',
    titre: 'Acceptez le contact',
    description: 'Quand une correspondance est trouvée, les deux parties doivent accepter avant que les coordonnées ne soient partagées.',
  }]

const LIENS_ESPACE = [
  { to: '/retrouve-amis/mon-profil', libelle: 'Mon profil', icone: 'fa-solid fa-user-shield' },
  { to: '/retrouve-amis/nouveau', libelle: 'Nouvel avis', icone: 'fa-solid fa-plus' },
  { to: '/retrouve-amis/mes-recherches', libelle: 'Mes recherches', icone: 'fa-solid fa-magnifying-glass' },
  { to: '/retrouve-amis/correspondances', libelle: 'Correspondances', icone: 'fa-solid fa-handshake' }]

// ── Mode carte : répartition géographique des avis ────────
const chargementCarte = ref(false)
const carteChargee = ref(false)
const avisCarteBrut = ref<AvisPublicResume[]>([])

// Index de résolution pays (id / nom normalisé → code ISO2, et ISO2 → pays_id)
const isoParPaysId = ref<Record<string, string>>({})
const isoParNomNorm = ref<Record<string, string>>({})
const paysIdParIso = ref<Record<string, string>>({})
/** Référentiel complet, trié : alimente le filtre « Territoire » du rail. */
const paysReferentiel = ref<{ id: string, nom: string }[]>([])

/** Résout le code ISO2 (minuscule) d'un pays d'avis via son id puis son nom. */
const resoudreIso = (pays?: PaysInfo): string | null => {
  if (!pays) return null
  const parId = isoParPaysId.value[pays.id]
  if (parId) return parId
  return isoParNomNorm.value[normaliserNomPays(pays.nom)] ?? null
}

/** Nombre d'avis par code ISO2 (recalculé quand les avis/index changent). */
const comptesParIso = computed<Record<string, number>>(() => {
  const map: Record<string, number> = {}
  for (const avis of avisCarteBrut.value) {
    const iso = resoudreIso(avis.pays)
    if (iso) map[iso] = (map[iso] || 0) + 1
  }
  return map
})

/** Charge le référentiel des pays (id, nom, code ISO2) une seule fois. */
const chargerReferentielPays = async () => {
  if (paysReferentiel.value.length > 0) return
  try {
    const rep = await $fetch<{ success: boolean, data: { id: string, nom: string, code_iso2: string | null }[] | null }>(
      `${apiBase}/api/pays`)
    const parId: Record<string, string> = {}
    const parNom: Record<string, string> = {}
    const idParIso: Record<string, string> = {}
    for (const p of rep.data ?? []) {
      const iso = p.code_iso2?.toLowerCase()
      if (!iso) continue
      parId[p.id] = iso
      parNom[normaliserNomPays(p.nom)] = iso
      idParIso[iso] = p.id
    }
    isoParPaysId.value = parId
    isoParNomNorm.value = parNom
    paysIdParIso.value = idParIso
    paysReferentiel.value = (rep.data ?? [])
      .map(p => ({ id: p.id, nom: p.nom }))
      .sort((a, b) => a.nom.localeCompare(b.nom, 'fr'))
  }
  catch (e) {
    console.error('Erreur chargement référentiel pays:', e)
  }
}

/** Prépare la vue carte : référentiel pays + lot d'avis à géolocaliser. */
const chargerCarte = async () => {
  if (carteChargee.value) return
  chargementCarte.value = true
  try {
    await chargerReferentielPays()
    const res = await rechercherAvisPublics({ par_page: 300 })
    if (res) avisCarteBrut.value = res.avis
    carteChargee.value = true
  }
  finally {
    chargementCarte.value = false
  }
}

watch(viewMode, (mode) => {
  if (mode === 'carte') chargerCarte()
})

// ── Territoire sélectionné sur la carte ───────────────────
const paysSelectionneIso = ref<string | null>(null)
const avisPaysSelectionne = ref<AvisPublicResume[]>([])
const chargementPaysSel = ref(false)

const nomPaysSelectionne = computed(() =>
  paysSelectionneIso.value ? (NOMS_PAYS_FR[paysSelectionneIso.value] ?? paysSelectionneIso.value) : '',
)

const panneauPaysRef = ref<HTMLElement | null>(null)

const onSelectPays = async (iso: string) => {
  // Re-cliquer le territoire déjà retenu le désélectionne : sans cela, la
  // croix du panneau était le seul retour à la carte nue.
  if (paysSelectionneIso.value === iso) {
    paysSelectionneIso.value = null
    return
  }
  paysSelectionneIso.value = iso
  // Vidée AVANT le chargement : le titre change immédiatement, la liste non.
  // Sans cela, le panneau affiche un instant les avis du territoire précédent
  // sous le nom du nouveau.
  avisPaysSelectionne.value = []
  // Le panneau naît SOUS la carte : sans ce défilement, le clic n'a d'effet
  // visible que dans la couleur du territoire. Avant le chargement, pour que
  // le déplacement ne dépende pas de la latence du réseau.
  await nextTick()
  amenerSousLaBarre(panneauPaysRef.value)
  const paysId = paysIdParIso.value[iso]
  if (!paysId) {
    avisPaysSelectionne.value = []
    return
  }
  chargementPaysSel.value = true
  try {
    const res = await rechercherAvisPublics({ pays_id: paysId, par_page: 60 })
    avisPaysSelectionne.value = res?.avis ?? []
  }
  finally {
    chargementPaysSel.value = false
  }
}

const partagesCumules = computed(() =>
  avisPublics.value.reduce((n, a) => n + (Number(a.compteur_partages) || 0), 0))

const pagesVisibles = computed(() => {
  const total = pagination.value?.pages ?? 0
  return Array.from({ length: total }, (_, i) => i + 1).filter(
    p => p === 1 || p === total || (p >= pageActuelle.value - 1 && p <= pageActuelle.value + 1))
})

onMounted(() => {
  chargerAvisPublics()
  chargerStatutTrouvable()
  // Le référentiel alimente le filtre « Territoire » du rail, disponible dès
  // le mode liste : il ne dépend donc plus du passage en vue carte.
  chargerReferentielPays()
})
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Africonnect"
        sous-titre="Retrouvez vos amis, proches et connaissances perdus de vue grâce à la communauté panafricaine."
        image="/images/africans/heros/hero-africonnect.jpg"
        aide="C'est quoi Africonnect ?"
        @aide="presentationOuverte = true"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane :segments="[{ libelle: 'Opafrica', vers: '/retrouve-amis' }, { libelle: 'Africonnect' }]">
        <template #action>
          <AfricansBouton
            icone="fa-solid fa-plus"
            :vers="estConnecte ? '/retrouve-amis/nouveau' : '/login'"
          >
            {{ estConnecte ? 'Créer un avis' : 'Se connecter pour créer un avis' }}
          </AfricansBouton>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="flex flex-col gap-6">
      <!-- Replié par défaut : c'est une explication, pas le contenu. En v-show,
           donc toujours atteignable par la recherche du navigateur. -->
      <AfricansAccordeon titre="Comment ça marche ?" icone="fa-solid fa-circle-info">
        <ol class="grid gap-5 sm:grid-cols-3">
          <li v-for="(etape, index) in ETAPES" :key="etape.titre" class="flex gap-3">
            <span class="grid size-7 shrink-0 place-items-center rounded-full bg-af-chocolat text-[12px]/[1.4] font-bold text-white">
              {{ index + 1 }}
            </span>
            <div>
              <p class="flex items-center gap-2 text-[14px]/[1.4] font-bold">
                <font-awesome-icon :icon="etape.icone" class="text-af-chocolat" />
                {{ etape.titre }}
              </p>
              <p class="mt-1 text-[12px]/[1.4] text-af-corps">{{ etape.description }}</p>
            </div>
          </li>
        </ol>
      </AfricansAccordeon>

      <div class="flex flex-wrap items-center gap-4">
        <p v-if="pagination && viewMode === 'liste'" class="text-[14px]/[1.4] text-af-corps">
          <strong class="font-bold">{{ pagination.total }}</strong> avis de recherche
          <template v-if="filtresActifs">correspondant à vos critères</template>
        </p>
        <p v-else-if="viewMode === 'carte'" class="flex items-center gap-2 text-[14px]/[1.4] text-af-corps">
          <font-awesome-icon icon="fa-solid fa-location-dot" class="text-af-chocolat" />
          Cliquez sur un territoire pour voir les avis qui y sont rattachés.
        </p>

        <AfricansBascule
          v-model="viewMode"
          class="ml-auto"
          libelle="Mode d'affichage des avis"
          :options="[
            { valeur: 'liste', libelle: 'Liste', icone: 'fa-solid fa-list-check' },
            { valeur: 'carte', libelle: 'Carte', icone: 'fa-solid fa-earth-africa' }]"
        />
      </div>

      <!-- ── Mode carte ── -->
      <template v-if="viewMode === 'carte'">
        <div v-if="chargementCarte" class="flex flex-col items-center gap-3 py-20">
          <font-awesome-icon icon="fa-solid fa-spinner" class="animate-spin text-2xl text-af-chocolat" />
          <p class="text-[14px]/[1.4] text-af-corps">Chargement de la carte…</p>
        </div>

        <template v-else>
          <div class="rounded-[10px] border border-af-bordure bg-white p-4">
            <RetrouveAmisCarteAfrique
              :comptes="comptesParIso"
              :selected-iso="paysSelectionneIso"
              @select="onSelectPays"
            />
          </div>

          <section v-if="paysSelectionneIso" ref="panneauPaysRef" class="flex flex-col gap-5">
            <h2 class="flex items-center gap-3 text-[20px]/[1.4] font-bold text-af-chocolat">
              <font-awesome-icon icon="fa-solid fa-location-dot" class="size-6" />
              {{ nomPaysSelectionne }}
              <button
                type="button"
                class="ml-auto text-af-atone transition hover:text-af-encre"
                aria-label="Fermer la sélection"
                @click="paysSelectionneIso = null"
              >
                <font-awesome-icon icon="fa-solid fa-xmark" />
              </button>
            </h2>

            <div v-if="chargementPaysSel" class="flex justify-center py-10">
              <font-awesome-icon icon="fa-solid fa-spinner" class="animate-spin text-2xl text-af-chocolat" />
            </div>

            <p v-else-if="!avisPaysSelectionne.length" class="text-[14px]/[1.4] text-af-atone">
              Aucun avis de recherche pour ce territoire.
            </p>

            <div v-else class="flex flex-col gap-6">
              <RetrouveAmisCarteAvisFil
                v-for="avis in avisPaysSelectionne"
                :key="avis.id"
                :avis="avis"
                @partager="partagerAvis(avis)"
              />
            </div>
          </section>
        </template>
      </template>

      <!-- ── Mode liste ── -->
      <template v-else>
        <div v-if="chargementAvis" class="flex flex-col gap-6">
          <div v-for="n in 2" :key="n" class="overflow-hidden rounded-[10px] border border-af-bordure bg-white">
            <div class="flex items-center gap-3 p-4">
              <div class="size-11 animate-pulse rounded-full bg-af-bordure" />
              <div class="h-3 w-1/3 animate-pulse rounded bg-af-bordure" />
            </div>
            <div class="aspect-[16/10] w-full animate-pulse bg-af-bordure" />
            <div class="h-10" />
          </div>
        </div>

        <template v-else-if="avisPublics.length">
          <div class="flex flex-col gap-6">
            <RetrouveAmisCarteAvisFil
              v-for="avis in avisPublics"
              :key="avis.id"
              :avis="avis"
              @partager="partagerAvis(avis)"
            />
          </div>

          <nav
            v-if="pagination && pagination.pages > 1"
            class="flex items-center justify-center gap-2"
            aria-label="Pagination des avis"
          >
            <button
              type="button"
              class="grid size-10 place-items-center rounded-lg border border-af-bordure bg-white text-af-corps transition hover:bg-af-chocolat/[0.07] disabled:opacity-40"
              :disabled="pageActuelle <= 1"
              aria-label="Page précédente"
              @click="chargerAvisPublics(pageActuelle - 1)"
            >
              <font-awesome-icon icon="fa-solid fa-chevron-left" />
            </button>

            <template v-for="(p, i) in pagesVisibles" :key="p">
              <span v-if="i > 0 && p - pagesVisibles[i - 1]! > 1" class="px-2 text-af-atone">…</span>
              <button
                type="button"
                class="h-10 min-w-10 rounded-lg px-3 text-[14px]/[1.4] font-bold transition"
                :class="p === pageActuelle
                  ? 'bg-af-degrade text-white'
                  : 'border border-af-bordure bg-white text-af-corps hover:bg-af-chocolat/[0.07]'"
                :aria-current="p === pageActuelle ? 'page' : undefined"
                @click="chargerAvisPublics(p)"
              >
                {{ p }}
              </button>
            </template>

            <button
              type="button"
              class="grid size-10 place-items-center rounded-lg border border-af-bordure bg-white text-af-corps transition hover:bg-af-chocolat/[0.07] disabled:opacity-40"
              :disabled="pageActuelle >= pagination.pages"
              aria-label="Page suivante"
              @click="chargerAvisPublics(pageActuelle + 1)"
            >
              <font-awesome-icon icon="fa-solid fa-chevron-right" />
            </button>
          </nav>
        </template>

        <!-- Deux vides distincts : « rien ne correspond » n'est pas « rien
             n'existe », et la sortie proposée n'est pas la même. -->
        <div v-else class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
          <font-awesome-icon
            :icon="filtresActifs ? 'fa-solid fa-filter' : 'fa-solid fa-users'"
            class="text-4xl text-af-atone-2"
          />
          <p class="mt-4 text-[16px]/[1.4] font-bold">
            {{ filtresActifs ? 'Aucun résultat pour ces critères' : 'Aucun avis de recherche pour le moment' }}
          </p>
          <p class="mt-2 text-[14px]/[1.4] text-af-corps">
            {{ filtresActifs
              ? 'Essayez d\'autres critères, ou repartez de zéro.'
              : 'Soyez le premier à publier un avis et aidez à réunir des proches séparés.' }}
          </p>
          <AfricansBouton
            v-if="filtresActifs"
            class="mt-5"
            variante="secondaire"
            icone="fa-solid fa-rotate-left"
            @click="reinitialiserFiltres"
          >
            Réinitialiser les filtres
          </AfricansBouton>
          <AfricansBouton
            v-else
            class="mt-5"
            icone="fa-solid fa-plus"
            :vers="estConnecte ? '/retrouve-amis/nouveau' : '/login'"
          >
            {{ estConnecte ? 'Créer le premier avis' : 'Se connecter pour créer un avis' }}
          </AfricansBouton>
        </div>
      </template>
    </div>

    <template #rail>
      <AfricansRecherche v-model="filtreRecherche" placeholder="Nom, lieu, école…" />

      <AfricansPanneau titre="Filtres" icone="fa-solid fa-sliders" action-libelle="Réinitialiser" @action="reinitialiserFiltres">
        <div class="flex flex-col gap-5">
          <AfricansChamp v-model="filtreRelation" libelle="Relation" type="select">
            <option value="">Toutes les relations</option>
            <option v-for="t in TYPES_RELATION" :key="t.value" :value="t.value">{{ t.label }}</option>
          </AfricansChamp>

          <!-- La maquette scinde les territoires en « Afrique / Hors Afrique ».
               `rechercherAvisPublics` n'a pas de paramètre de zone, contrairement
               à Afrolang : la scission serait une illusion, le référentiel est
               donc servi d'une seule liste. -->
          <AfricansChamp v-model="filtrePaysId" libelle="Territoire" type="select">
            <option value="">Tous les territoires</option>
            <option v-for="p in paysReferentiel" :key="p.id" :value="p.id">{{ p.nom }}</option>
          </AfricansChamp>
        </div>
      </AfricansPanneau>

      <AfricansPanneau v-if="estConnecte" titre="Votre visibilité" icone="fa-solid fa-user-shield">
        <p class="text-[14px]/[1.4] text-af-corps">
          {{ estTrouvable
            ? 'Vous êtes trouvable : les avis de recherche peuvent vous être rapprochés.'
            : 'Vous n\'êtes pas trouvable. Aucun avis ne peut vous être rapproché.' }}
        </p>
        <AfricansBouton
          class="mt-4"
          pleine-largeur
          :variante="estTrouvable ? 'secondaire' : 'primaire'"
          :desactive="chargementTrouvable"
          :tourne="chargementTrouvable"
          :icone="chargementTrouvable ? 'fa-solid fa-spinner' : (estTrouvable ? 'fa-solid fa-eye-slash' : 'fa-solid fa-eye')"
          @click="onActiverTrouvable"
        >
          {{ estTrouvable ? 'Ne plus être trouvable' : 'Devenir trouvable' }}
        </AfricansBouton>
      </AfricansPanneau>

      <AfricansPanneau v-if="estConnecte" titre="Mon espace" icone="fa-solid fa-folder-open">
        <ul class="flex flex-col">
          <li v-for="lien in LIENS_ESPACE" :key="lien.to" class="border-t border-af-bordure first:border-t-0">
            <NuxtLink
              :to="lien.to"
              class="flex items-center gap-3 py-3 text-[14px]/[1.4] font-bold transition hover:text-af-chocolat"
            >
              <font-awesome-icon :icon="lien.icone" class="text-af-chocolat" />
              {{ lien.libelle }}
            </NuxtLink>
          </li>
        </ul>
      </AfricansPanneau>

      <!-- La maquette compte aussi les avis LUS : aucune donnée serveur ne
           tient cette grandeur, elle n'est donc pas affichée. -->
      <AfricansPanneau titre="Statistiques" icone="fa-solid fa-chart-line">
        <dl class="flex flex-col">
          <div class="flex items-baseline justify-between gap-4 pb-3">
            <dt class="text-[14px]/[1.4] font-bold">Avis publiés</dt>
            <dd class="text-[14px]/[1.4] font-bold text-af-chocolat">{{ pagination?.total ?? 0 }}</dd>
          </div>
          <div class="flex items-baseline justify-between gap-4 border-t border-af-bordure py-3">
            <dt class="text-[14px]/[1.4] text-af-corps">Partages (page affichée)</dt>
            <dd class="text-[14px]/[1.4] font-bold">{{ partagesCumules }}</dd>
          </div>
        </dl>
      </AfricansPanneau>

      <AfricansPanneau v-if="viewMode === 'carte'" titre="Légende" icone="fa-solid fa-map-pin">
        <ul class="flex flex-col gap-2">
          <li v-for="palier in PALIERS_CHALEUR" :key="palier.libelle" class="flex items-center gap-2 text-[12px]/[1.4]">
            <span class="size-3 shrink-0 rounded-full" :style="{ backgroundColor: palier.couleur }" />
            {{ palier.libelle }}
          </li>
        </ul>
      </AfricansPanneau>
    </template>

    <RetrouveAmisDecouverteModale v-model="presentationOuverte" />

    <Transition name="af-surgir">
      <div
        v-if="messageCopie"
        class="fixed right-6 bottom-6 z-100 rounded-[10px] border border-af-vert bg-white px-5 py-4 shadow-xl font-af"
        role="status"
      >
        <p class="flex items-center gap-3 text-[14px]/[1.4]">
          <font-awesome-icon icon="fa-solid fa-circle-check" class="text-af-vert" />
          {{ messageCopie }}
        </p>
      </div>
    </Transition>
  </NuxtLayout>
</template>


<style scoped>
.af-surgir-enter-active,
.af-surgir-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}
.af-surgir-enter-from,
.af-surgir-leave-to {
  opacity: 0;
  transform: translateY(12px);
}
</style>
