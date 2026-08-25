<script setup lang="ts">
import {
  useAfrolang,
  type SalleAPI,
  type SallePriveeAPI,
  type SalleFiltres,
  type AfrolangStats,
  type PaysOrigineLight,
} from '~/composables/useAfrolang'
import { useUserStore } from '~/stores/user'

/**
 * Afrolang : listing des salles publiques, porté sur le gabarit de la refonte.
 *
 * La logique de données est celle d'avant, inchangée : mêmes endpoints, mêmes
 * filtres serveur, même pagination, même parcours d'entrée en session. Seule la
 * présentation bascule, et deux choses se déplacent :
 *   - les filtres passent de la colonne GAUCHE au rail droit, là où la maquette
 *     les pose ; la colonne gauche est désormais la navigation du gabarit ;
 *   - la fiche d'une salle devient une modale (« Infos salle Afrolang »), la
 *     page `/afrolang/[id]` n'étant plus qu'une redirection vers la session.
 */
definePageMeta({ layout: false })

useHead({
  title: 'Salles Afrolang - AfricanS',
  meta: [
    {
      name: 'description',
      content: 'Découvrez les salles Afrolang : visioconférence dédiée à l\'apprentissage des langues africaines. Wolof, Swahili, Lingala et bien plus.',
    },
  ],
})

const ITEMS_PER_PAGE = 12
const userStore = useUserStore()
const { redirigerVersConnexion } = useAuth()

// Modale « Proposer une salle » (feature 001-admin-salles-publiques, US1)
const proposerOuvert = ref(false)

// Modale de présentation « C'est quoi Afrolang ? »
const presentationOuverte = ref(false)

// Modale « Infos salle » : la salle affichée est celle de la liste déjà
// chargée : la fiche n'entraîne AUCUNE requête supplémentaire.
const salleAffichee = ref<SalleAPI | null>(null)
const infosOuvertes = ref(false)

const {
  listerSalles,
  listerSallesPriveesParSallePublique,
  creerSallePrivee,
  verifierCodeAcces,
  modifierCodeAcces,
  archiverSallePriveeParAuteur,
  memoriserAccesJeton,
  obtenirStats,
  listerLangues,
  listerPaysDisponibles,
} = useAfrolang()

// State
const salles = ref<SalleAPI[]>([])
const total = ref(0)
const totalSalles = ref(0)
const totalPages = ref(1)
const currentPage = ref(1)
const languesDisponibles = ref<string[]>([])
const initialLoading = ref(true)

// Widget Canal privé : expansion des salles privées
const expandedSalleId = ref<string | null>(null)
const loadingPrivees = ref(false)
const sallesPriveesCache = ref<Record<string, SallePriveeAPI[]>>({})

// Entrer dans la salle (visioconférence publique)
const salleEnCoursEntree = ref<string | null>(null)
const erreurEntrer = ref<string | null>(null)

const stats = ref<AfrolangStats>({
  total_salles: 0,
  total_salles_privees: 0,
  sessions_en_cours: 0,
  sessions_terminees: 0,
  total_participants_uniques: 0,
})

const filtres = ref<SalleFiltres>({
  recherche: '',
  langue: '',
  pays_id: '',
  zone: 'tout',
})

// Pays d'origine disponibles (feature 001-afrolang-pays-origine).
// Source = endpoint dédié `/pays-disponibles` (tous les territoires de toutes
// les salles), indépendant de la pagination et des filtres en cours, sinon la
// liste se restreindrait aux seules salles de la page courante.
const paysDisponibles = ref<PaysOrigineLight[]>([])

let rechercheTimer: ReturnType<typeof setTimeout> | null = null

const buildApiFiltres = (): SalleFiltres => {
  const f: SalleFiltres = {
    page: currentPage.value,
    par_page: ITEMS_PER_PAGE,
  }
  if (filtres.value.recherche?.trim()) f.recherche = filtres.value.recherche.trim()
  if (filtres.value.langue) f.langue = filtres.value.langue
  if (filtres.value.pays_id) f.pays_id = filtres.value.pays_id
  if (filtres.value.zone) f.zone = filtres.value.zone
  return f
}

const chargerSalles = async () => {
  const resultat = await listerSalles(buildApiFiltres())
  if (resultat) {
    salles.value = resultat.salles
    total.value = resultat.total
    totalPages.value = resultat.total_pages
  }
}

// US1 : Entrer dans le livestream public en 1 clic.
const entrerDansSalle = async (salleId: string) => {
  erreurEntrer.value = null

  if (!userStore.isAuthenticated) {
    redirigerVersConnexion()
    return
  }

  salleEnCoursEntree.value = salleId
  try {
    await navigateTo(`/afrolang/session/${salleId}`)
  }
  catch (e: unknown) {
    const message = e instanceof Error ? e.message : 'Navigation impossible'
    erreurEntrer.value = message
    console.error('Erreur entrerDansSalle:', e)
  }
  finally {
    salleEnCoursEntree.value = null
  }
}

const ouvrirInfos = (salle: SalleAPI) => {
  salleAffichee.value = salle
  infosOuvertes.value = true
}

/** Depuis la modale : on ferme avant de naviguer, sinon la modale rendrait le
 *  focus à un bouton d'une page déjà quittée. */
const entrerDepuisInfos = async (salleId: string) => {
  infosOuvertes.value = false
  await entrerDansSalle(salleId)
}

const ouvrirProposition = () => {
  if (!userStore.isAuthenticated) {
    redirigerVersConnexion()
    return
  }
  proposerOuvert.value = true
}

// ── Widget salles privées : état modales + actions ────────────────────────

const createModalRef = ref<{
  setLoading: (v: boolean) => void
  setError: (m: string) => void
  setSuccess: () => void
  setExistante: (id?: string) => void
} | null>(null)
const createModalOpen = ref(false)
const createModalSalleId = ref('')
let codeSecretEnAttente = ''

const joinModalRef = ref<{
  setLoading: (v: boolean) => void
  setError: (m: string) => void
  setSuccess: () => void
} | null>(null)
const joinModalOpen = ref(false)
const joinModalSalleTitre = ref('')
const joinModalSallePriveeId = ref('')

const modifCodeModalOpen = ref(false)
const modifCodeSallePriveeId = ref('')
const nouveauCode = ref('')
const erreurModifCode = ref<string | null>(null)
const modifCodeEnCours = ref(false)

const sallePriveeEnCours = ref<string | null>(null)
const erreurSallePrivee = ref<string | null>(null)
const toastCreation = ref<{ code: string } | null>(null)

/** ID de la salle privée dont l'utilisateur courant est l'auteur, pour la
 *  salle publique actuellement dépliée (US4). `null` si aucune. */
const maSallePriveeIciId = computed<string | null>(() => {
  if (!expandedSalleId.value) return null
  const liste = sallesPriveesCache.value[expandedSalleId.value]
  return liste?.find(sp => sp.est_auteur)?.id ?? null
})

const ouvrirCreationModal = (salleId: string) => {
  if (!userStore.isAuthenticated) {
    redirigerVersConnexion()
    return
  }
  createModalSalleId.value = salleId
  createModalOpen.value = true
}

const soumettreCreationSallePrivee = async (payload: { titre: string, description: string, code_acces: string }) => {
  createModalRef.value?.setLoading(true)
  codeSecretEnAttente = payload.code_acces

  const resultat = await creerSallePrivee(createModalSalleId.value, {
    titre: payload.titre,
    description: payload.description,
    code_acces: payload.code_acces,
  })

  if (!resultat) {
    createModalRef.value?.setError('Échec de la création, veuillez réessayer.')
    return
  }
  if ('erreur' in resultat && resultat.erreur === 'salle_privee_unicite') {
    createModalRef.value?.setExistante(resultat.salle_privee_existante_id)
    return
  }

  // Succès : rafraîchir la liste du widget + toast
  const liste = await listerSallesPriveesParSallePublique(createModalSalleId.value)
  sallesPriveesCache.value[createModalSalleId.value] = liste
  createModalRef.value?.setSuccess()
  toastCreation.value = { code: codeSecretEnAttente }
  setTimeout(() => { toastCreation.value = null }, 8000)
}

const rediriger_vers_salle_existante = (_sallePriveeId?: string) => {
  // Aucune navigation dédiée : l'utilisateur peut cliquer "Ouvrir ma salle
  // privée" depuis le widget. Le modale se ferme, il voit le bouton basculer.
  createModalOpen.value = false
  if (createModalSalleId.value) {
    listerSallesPriveesParSallePublique(createModalSalleId.value).then((liste) => {
      sallesPriveesCache.value[createModalSalleId.value] = liste
    })
  }
}

const ouvrirJoinModal = (sallePriveeId: string) => {
  if (!userStore.isAuthenticated) {
    redirigerVersConnexion()
    return
  }
  const liste = expandedSalleId.value ? sallesPriveesCache.value[expandedSalleId.value] : null
  joinModalSalleTitre.value = liste?.find(sp => sp.id === sallePriveeId)?.titre ?? ''
  joinModalSallePriveeId.value = sallePriveeId
  joinModalOpen.value = true
}

const soumettreCodeAcces = async (code: string) => {
  joinModalRef.value?.setLoading(true)
  const resultat = await verifierCodeAcces(joinModalSallePriveeId.value, code)

  if (!resultat) {
    joinModalRef.value?.setError('Erreur réseau, veuillez réessayer.')
    return
  }
  if ('erreur' in resultat && resultat.erreur === 'code_incorrect') {
    joinModalRef.value?.setError('Code incorrect.')
    return
  }
  if ('erreur' in resultat && resultat.erreur === 'rate_limit') {
    joinModalRef.value?.setError('Trop de tentatives, réessayez dans quelques minutes.')
    return
  }

  memoriserAccesJeton(joinModalSallePriveeId.value, resultat.acces_jeton, resultat.expires_at)
  joinModalRef.value?.setSuccess()
  sallePriveeEnCours.value = joinModalSallePriveeId.value
  await navigateTo(`/afrolang/session/privee/${joinModalSallePriveeId.value}`)
}

/** Court-circuit auteur : pas de saisie, appel direct `verifier-code` avec
 *  le code actuel inconnu côté client → le backend retourne le jeton parce
 *  que l'utilisateur est `cree_par`. On envoie une chaîne quelconque. */
const ouvrirMaSallePrivee = async (sallePriveeId: string) => {
  if (!userStore.isAuthenticated) {
    redirigerVersConnexion()
    return
  }
  sallePriveeEnCours.value = sallePriveeId
  erreurSallePrivee.value = null

  const resultat = await verifierCodeAcces(sallePriveeId, '')
  if (!resultat || 'erreur' in resultat) {
    sallePriveeEnCours.value = null
    erreurSallePrivee.value = 'Impossible d\'ouvrir votre salle privée.'
    return
  }
  memoriserAccesJeton(sallePriveeId, resultat.acces_jeton, resultat.expires_at)
  await navigateTo(`/afrolang/session/privee/${sallePriveeId}`)
}

const ouvrirModifCodeModal = (sallePriveeId: string) => {
  modifCodeSallePriveeId.value = sallePriveeId
  nouveauCode.value = ''
  erreurModifCode.value = null
  modifCodeModalOpen.value = true
}

const soumettreNouveauCode = async () => {
  erreurModifCode.value = null
  const code = nouveauCode.value.trim()
  if (!/^[A-Za-z0-9!@#$%&*?-]{4,16}$/.test(code)) {
    erreurModifCode.value = 'Format invalide (4 à 16 caractères, lettres/chiffres/!@#$%&*?-).'
    return
  }
  modifCodeEnCours.value = true
  const ok = await modifierCodeAcces(modifCodeSallePriveeId.value, code)
  modifCodeEnCours.value = false
  if (!ok) {
    erreurModifCode.value = 'Échec de la modification.'
    return
  }
  modifCodeModalOpen.value = false
}

const confirmerArchivage = async (sallePriveeId: string) => {
  if (!confirm('Supprimer définitivement cette salle privée ? La session en cours sera terminée.')) {
    return
  }
  sallePriveeEnCours.value = sallePriveeId
  const ok = await archiverSallePriveeParAuteur(sallePriveeId)
  sallePriveeEnCours.value = null
  if (!ok) {
    erreurSallePrivee.value = 'Échec de la suppression.'
    return
  }
  if (expandedSalleId.value) {
    const liste = await listerSallesPriveesParSallePublique(expandedSalleId.value)
    sallesPriveesCache.value[expandedSalleId.value] = liste
  }
}

// Widget Canal privé : toggle dropdown
const togglePrivees = async (salleId: string) => {
  if (expandedSalleId.value === salleId) {
    expandedSalleId.value = null
    return
  }

  expandedSalleId.value = salleId

  if (!sallesPriveesCache.value[salleId]) {
    loadingPrivees.value = true
    try {
      const liste = await listerSallesPriveesParSallePublique(salleId)
      sallesPriveesCache.value[salleId] = liste
    }
    catch (e: unknown) {
      console.error('Erreur listerSallesPriveesParSallePublique:', e)
      sallesPriveesCache.value[salleId] = []
    }
    finally {
      loadingPrivees.value = false
    }
  }
}

const visiblePages = computed(() => {
  const pages: (number | string)[] = []
  const tp = totalPages.value
  const current = currentPage.value

  if (tp <= 7) {
    for (let i = 1; i <= tp; i++) pages.push(i)
  }
  else {
    pages.push(1)
    if (current > 3) pages.push('...')
    const start = Math.max(2, current - 1)
    const end = Math.min(tp - 1, current + 1)
    for (let i = start; i <= end; i++) pages.push(i)
    if (current < tp - 2) pages.push('...')
    pages.push(tp)
  }

  return pages
})

/**
 * UN watcher pour les trois filtres serveur, et non un par champ : changer de
 * zone vide aussi le territoire, et deux watchers séparés déclencheraient deux
 * requêtes pour un seul geste. La recherche garde le sien, elle est amortie.
 */
watch(
  () => [filtres.value.langue, filtres.value.pays_id, filtres.value.zone],
  () => {
    currentPage.value = 1
    chargerSalles()
  })

watch(
  () => filtres.value.recherche,
  () => {
    if (rechercheTimer) clearTimeout(rechercheTimer)
    rechercheTimer = setTimeout(() => {
      currentPage.value = 1
      chargerSalles()
    }, 300)
  })

const resetFilters = () => {
  filtres.value = { recherche: '', langue: '', pays_id: '', zone: 'tout' }
  currentPage.value = 1
}

const goToPage = (page: number) => {
  if (page >= 1 && page <= totalPages.value) {
    currentPage.value = page
    chargerSalles()
    window.scrollTo({ top: 400, behavior: 'smooth' })
  }
}

/** Compteurs de la maquette, à deux chiffres : ils s'alignent d'une carte à
 *  l'autre dans la grille, ce qu'un « 2 » nu ne fait pas. */
const surDeux = (n: number) => String(n).padStart(2, '0')

/**
 * Métrique de pied de vignette. La maquette y met un nombre de participants
 * qu'aucune donnée serveur ne porte par salle ; le compteur réellement servi
 * est celui des modérateurs attitrés. À zéro, il n'est PAS affiché : une
 * colonne de « 00 » sur toute la grille n'apprend rien et fait du bruit.
 */
const metriqueDe = (salle: SalleAPI) => {
  const n = salle.nombre_moderateurs_attitres
  if (!n) return undefined
  return {
    icone: 'fa-solid fa-user-shield',
    texte: `${surDeux(n)} modérateur${n > 1 ? 's' : ''}`,
  }
}

/** Territoires d'origine en une ligne : au-delà de deux, la carte déborderait. */
const lieuDe = (salle: SalleAPI): string | undefined => {
  const noms = salle.pays_origine?.map(p => p.nom) ?? []
  if (!noms.length) return undefined
  return noms.length <= 2 ? noms.join(', ') : `${noms.slice(0, 2).join(', ')} +${noms.length - 2}`
}

const aucunFiltreActif = computed(() =>
  !filtres.value.recherche && !filtres.value.langue && !filtres.value.pays_id
  && (!filtres.value.zone || filtres.value.zone === 'tout'))

onMounted(async () => {
  const [statsResult, languesResult, paysResult] = await Promise.all([
    obtenirStats(),
    listerLangues(),
    listerPaysDisponibles(),
  ])

  if (statsResult) {
    stats.value = statsResult
    totalSalles.value = statsResult.total_salles
  }
  languesDisponibles.value = languesResult
  paysDisponibles.value = paysResult

  await chargerSalles()
  initialLoading.value = false
})
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Afrolang"
        image="/images/africans/heros/hero-afrolang.jpg"
        aide="C'est quoi Afrolang ?"
        @aide="presentationOuverte = true"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane :segments="[{ libelle: 'Africarise', vers: '/codi-moi' }, { libelle: 'Afrolang' }]">
        <template #action>
          <!-- La maquette écrit « Nouvelle Salle ». Le bouton n'en crée aucune :
               une salle publique naît d'une proposition soumise à validation.
               Le libellé dit ce qui va réellement se passer. -->
          <AfricansBouton icone="fa-solid fa-lightbulb" @click="ouvrirProposition">
            Proposer une salle
          </AfricansBouton>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="flex flex-col gap-8">
      <p class="max-w-3xl text-[14px]/[1.4] text-af-corps">
        Apprenez une langue africaine ou afro-descendante à distance, et rencontrez celles et ceux
        qui la pratiquent. Chaque salle est un espace de visioconférence ouvert&nbsp;; les cours
        privés y sont protégés par un code remis par leur auteur.
      </p>

      <!-- Erreurs d'action : au-dessus de la liste, parce qu'elles répondent à
           un geste que l'utilisateur vient de faire dans la grille. -->
      <div
        v-if="erreurEntrer || erreurSallePrivee"
        class="flex items-start gap-3 rounded-[10px] border border-af-live/30 bg-af-live/[0.05] px-5 py-4"
      >
        <font-awesome-icon icon="fa-solid fa-circle-exclamation" class="mt-1 text-af-live" />
        <p class="flex-1 text-[14px]/[1.4] text-af-corps">{{ erreurEntrer || erreurSallePrivee }}</p>
        <button
          type="button"
          class="text-af-atone transition hover:text-af-encre"
          aria-label="Masquer le message"
          @click="erreurEntrer = null; erreurSallePrivee = null"
        >
          <font-awesome-icon icon="fa-solid fa-xmark" />
        </button>
      </div>

      <!-- Chargement : squelettes aux dimensions réelles des cartes, pour que
           la mise en page ne saute pas à l'arrivée des données. -->
      <div v-if="initialLoading" class="grid gap-5 sm:grid-cols-2">
        <div v-for="n in 4" :key="n" class="overflow-hidden rounded-[10px] border border-af-bordure bg-white">
          <div class="aspect-video w-full animate-pulse bg-af-bordure" />
          <div class="flex flex-col gap-3 p-4">
            <div class="h-4 w-2/3 animate-pulse rounded bg-af-bordure" />
            <div class="h-3 w-full animate-pulse rounded bg-af-bordure" />
            <div class="h-10 w-full animate-pulse rounded-lg bg-af-bordure" />
          </div>
        </div>
      </div>

      <template v-else-if="salles.length">
        <div class="grid gap-5 sm:grid-cols-2">
          <template v-for="salle in salles" :key="salle.id">
            <AfricansCarteSalle
              :titre="salle.titre"
              :description="salle.description || undefined"
              :lieu="lieuDe(salle)"
              :langue="salle.langue_cible || undefined"
              :image="salle.image_couverture_url"
              :en-direct="salle.sessions_en_cours > 0"
              :metrique="metriqueDe(salle)"
              :chargement="salleEnCoursEntree === salle.id"
              :desactivee="!!salle.desactivee_admin"
              @agir="entrerDansSalle(salle.id)"
            >
              <!-- Fermeture administrative : elle explique pourquoi le bouton
                   d'entrée est inerte, elle doit donc être sur la carte. -->
              <p
                v-if="salle.desactivee_admin"
                class="flex items-center gap-2 rounded border border-af-live/30 bg-af-live/[0.05] px-2 py-1.5 text-[12px]/[1.4] text-af-corps"
              >
                <font-awesome-icon icon="fa-solid fa-ban" class="text-af-live" />
                Salle fermée par l'administration
              </p>

              <p
                v-if="salle.administrateurs?.length"
                class="flex items-center gap-2 text-[12px]/[1.4] text-af-atone"
              >
                <AfricansAvatar
                  v-for="admin in salle.administrateurs.slice(0, 3)"
                  :key="admin.utilisateur_id"
                  :nom="`${admin.prenom} ${admin.nom}`"
                  :src="admin.photo_url"
                  :taille="24"
                />
                <span>
                  {{ salle.administrateurs.length }} administrateur{{ salle.administrateurs.length > 1 ? 's' : '' }}
                </span>
              </p>

              <template #actions>
                <div class="flex flex-wrap items-center gap-x-4 gap-y-2 text-[12px]/[1.4]">
                  <button
                    type="button"
                    class="flex items-center gap-1.5 text-af-chocolat transition hover:opacity-70"
                    @click="ouvrirInfos(salle)"
                  >
                    <font-awesome-icon icon="fa-solid fa-circle-info" />
                    Détails de la salle
                  </button>

                  <button
                    type="button"
                    class="ml-auto flex items-center gap-1.5 transition hover:opacity-70"
                    :class="expandedSalleId === salle.id ? 'font-bold text-af-chocolat' : 'text-af-corps'"
                    :aria-expanded="expandedSalleId === salle.id"
                    @click="togglePrivees(salle.id)"
                  >
                    <font-awesome-icon icon="fa-solid fa-door-open" />
                    Canal privé ({{ salle.nombre_salles_privees }})
                    <font-awesome-icon :icon="expandedSalleId === salle.id ? 'fa-solid fa-chevron-up' : 'fa-solid fa-chevron-down'" />
                  </button>
                </div>
              </template>
            </AfricansCarteSalle>

            <!-- Canal privé : déplié SOUS la carte cliquée, sur toute la
                 largeur de la grille. Absent de la maquette : la fonctionnalité
                 est postérieure, il reprend son vocabulaire visuel. -->
            <Transition name="af-deplier">
              <section
                v-if="expandedSalleId === salle.id"
                class="col-span-full rounded-[10px] border border-af-chocolat/30 bg-af-chocolat/[0.07] p-5"
              >
                <div v-if="loadingPrivees" class="flex items-center justify-center gap-3 py-8 text-[14px]/[1.4] text-af-corps">
                  <font-awesome-icon icon="fa-solid fa-spinner" class="animate-spin text-af-chocolat" />
                  Chargement des cours privés…
                </div>

                <template v-else>
                  <header class="mb-4 flex flex-wrap items-center gap-4">
                    <h3 class="flex items-center gap-3 text-[17px]/[1.4] font-bold">
                      <font-awesome-icon icon="fa-solid fa-door-open" class="size-6 text-af-chocolat" />
                      {{ (sallesPriveesCache[salle.id]?.length ?? 0) }}
                      cours privé{{ (sallesPriveesCache[salle.id]?.length ?? 0) > 1 ? 's' : '' }}
                      {{ salle.titre }}
                    </h3>

                    <!-- US4 : un membre n'a droit qu'à UNE salle privée par
                         salle publique ; le bouton bascule selon qu'elle existe. -->
                    <AfricansBouton
                      v-if="userStore.isAuthenticated"
                      class="ml-auto"
                      :variante="maSallePriveeIciId ? 'primaire' : 'secondaire'"
                      :icone="maSallePriveeIciId ? 'fa-solid fa-door-open' : 'fa-solid fa-plus'"
                      :desactive="sallePriveeEnCours === (maSallePriveeIciId || 'creation')"
                      @click="maSallePriveeIciId ? ouvrirMaSallePrivee(maSallePriveeIciId) : ouvrirCreationModal(salle.id)"
                    >
                      {{ maSallePriveeIciId ? 'Ouvrir ma salle privée' : 'Créer ma salle privée' }}
                    </AfricansBouton>
                  </header>

                  <div
                    v-if="(sallesPriveesCache[salle.id]?.length ?? 0) > 0"
                    class="grid gap-5 sm:grid-cols-2 xl:grid-cols-3"
                  >
                    <AfricansCarteCoursPrive
                      v-for="sp in (sallesPriveesCache[salle.id] ?? [])"
                      :key="sp.id"
                      :titre="sp.titre"
                      :description="sp.description"
                      :auteur-nom="sp.auteur_nom"
                      :est-auteur="sp.est_auteur"
                      :en-direct="sp.session_en_cours"
                      :chargement="sallePriveeEnCours === sp.id"
                      @rejoindre="ouvrirJoinModal(sp.id)"
                      @ouvrir="ouvrirMaSallePrivee(sp.id)"
                      @modifier-code="ouvrirModifCodeModal(sp.id)"
                      @archiver="confirmerArchivage(sp.id)"
                    />
                  </div>

                  <p v-else class="py-6 text-center text-[14px]/[1.4] text-af-atone">
                    Aucun cours privé dans cette salle.
                  </p>
                </template>
              </section>
            </Transition>
          </template>
        </div>

        <!-- Pagination : la grille est paginée CÔTÉ SERVEUR, le compteur du
             rail porte donc sur l'ensemble, pas sur la page affichée. -->
        <nav v-if="totalPages > 1" class="flex items-center justify-center gap-2" aria-label="Pagination des salles">
          <button
            type="button"
            class="grid size-10 place-items-center rounded-lg border border-af-bordure bg-white text-af-corps transition hover:bg-af-chocolat/[0.07] disabled:opacity-40"
            :disabled="currentPage === 1"
            aria-label="Page précédente"
            @click="goToPage(currentPage - 1)"
          >
            <font-awesome-icon icon="fa-solid fa-chevron-left" />
          </button>

          <template v-for="(page, i) in visiblePages" :key="`${page}-${i}`">
            <span v-if="page === '...'" class="px-2 text-af-atone">…</span>
            <button
              v-else
              type="button"
              class="h-10 min-w-10 rounded-lg px-3 text-[14px]/[1.4] font-bold transition"
              :class="currentPage === page
                ? 'bg-af-degrade text-white'
                : 'border border-af-bordure bg-white text-af-corps hover:bg-af-chocolat/[0.07]'"
              :aria-current="currentPage === page ? 'page' : undefined"
              @click="goToPage(page as number)"
            >
              {{ page }}
            </button>
          </template>

          <button
            type="button"
            class="grid size-10 place-items-center rounded-lg border border-af-bordure bg-white text-af-corps transition hover:bg-af-chocolat/[0.07] disabled:opacity-40"
            :disabled="currentPage === totalPages"
            aria-label="Page suivante"
            @click="goToPage(currentPage + 1)"
          >
            <font-awesome-icon icon="fa-solid fa-chevron-right" />
          </button>
        </nav>
      </template>

      <!-- Deux vides distincts : « rien ne correspond » n'est pas « rien
           n'existe », et la sortie proposée n'est pas la même. -->
      <div v-else class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
        <font-awesome-icon icon="fa-solid fa-door-open" class="text-4xl text-af-atone-2" />
        <p class="mt-4 text-[16px]/[1.4] font-bold">
          {{ aucunFiltreActif ? 'Aucune salle pour le moment' : 'Aucune salle ne correspond à vos critères' }}
        </p>
        <p class="mt-2 text-[14px]/[1.4] text-af-corps">
          {{ aucunFiltreActif
            ? 'Proposez la première salle pour la langue que vous souhaitez faire vivre.'
            : 'Essayez une autre langue, un autre territoire, ou repartez de zéro.' }}
        </p>
        <AfricansBouton
          class="mt-5"
          :variante="aucunFiltreActif ? 'primaire' : 'secondaire'"
          :icone="aucunFiltreActif ? 'fa-solid fa-lightbulb' : 'fa-solid fa-rotate-right'"
          @click="aucunFiltreActif ? ouvrirProposition() : resetFilters()"
        >
          {{ aucunFiltreActif ? 'Proposer une salle' : 'Réinitialiser les filtres' }}
        </AfricansBouton>
      </div>
    </div>

    <template #rail>
      <AfricansRecherche v-model="filtres.recherche" placeholder="Salle, langue…" />

      <AfrolangSalleFiltresPanneau
        v-model="filtres"
        :langues="languesDisponibles"
        :pays="paysDisponibles"
        :total="totalSalles"
        :resultats="total"
        @reset="resetFilters"
      />

      <!-- Les trois compteurs de la maquette (salles créées, participations,
           modérateurs suivis) n'ont pas d'équivalent servi par l'API : ceux-ci
           sont ceux que `/afrolang/stats` renvoie réellement. -->
      <AfricansPanneau titre="Statistiques" icone="fa-solid fa-chart-line">
        <dl class="flex flex-col">
          <div class="flex items-baseline justify-between gap-4 py-3">
            <dt class="text-[14px]/[1.4] font-bold">Salles</dt>
            <dd class="text-[14px]/[1.4] font-bold text-af-chocolat">{{ surDeux(stats.total_salles) }}</dd>
          </div>
          <div class="flex items-baseline justify-between gap-4 border-t border-af-bordure py-3">
            <dt class="text-[14px]/[1.4] font-bold">Cours privés</dt>
            <dd class="text-[14px]/[1.4] font-bold text-af-chocolat">{{ surDeux(stats.total_salles_privees) }}</dd>
          </div>
          <div class="flex items-baseline justify-between gap-4 border-t border-af-bordure py-3">
            <dt class="text-[14px]/[1.4] font-bold">Sessions en cours</dt>
            <dd class="text-[14px]/[1.4] font-bold text-af-chocolat">{{ surDeux(stats.sessions_en_cours) }}</dd>
          </div>
          <div class="flex items-baseline justify-between gap-4 border-t border-af-bordure py-3">
            <dt class="text-[14px]/[1.4] font-bold">Participants</dt>
            <dd class="text-[14px]/[1.4] font-bold text-af-chocolat">{{ surDeux(stats.total_participants_uniques) }}</dd>
          </div>
        </dl>
      </AfricansPanneau>
    </template>

    <!-- ══════════════ Surcouches ══════════════ -->

    <AfrolangSalleInfosModale
      v-model="infosOuvertes"
      :salle="salleAffichee"
      @agir="entrerDepuisInfos"
    />

    <AfrolangDecouverteModale v-model="presentationOuverte" />

    <AfrolangProposerSalleModal :open="proposerOuvert" @close="proposerOuvert = false" />

    <AfrolangSallePriveeCreateModal
      ref="createModalRef"
      :is-open="createModalOpen"
      :salle-id="createModalSalleId"
      @close="createModalOpen = false"
      @submit="soumettreCreationSallePrivee"
      @existante="rediriger_vers_salle_existante"
    />

    <AfrolangSallePriveeJoinModal
      ref="joinModalRef"
      :is-open="joinModalOpen"
      :salle-privee-titre="joinModalSalleTitre"
      @close="joinModalOpen = false"
      @submit="soumettreCodeAcces"
    />

    <AfricansModale v-model="modifCodeModalOpen" titre="Modifier le code secret" icone="fa-solid fa-key">
      <form id="af-form-code" class="flex flex-col gap-2" @submit.prevent="soumettreNouveauCode">
        <AfricansChamp
          v-model="nouveauCode"
          libelle="Nouveau code secret"
          placeholder="nouveauCode!"
          aide="4 à 16 caractères (lettres, chiffres ou !@#$%&*?-)."
        />
        <p v-if="erreurModifCode" class="text-[12px]/[1.4] text-af-live">{{ erreurModifCode }}</p>
      </form>
      <template #actions>
        <button
          type="button"
          class="text-base font-bold text-af-corps transition hover:opacity-70"
          @click="modifCodeModalOpen = false"
        >
          Annuler
        </button>
        <AfricansBouton
          type="submit"
          form="af-form-code"
          :desactive="modifCodeEnCours"
          :tourne="modifCodeEnCours"
          :icone="modifCodeEnCours ? 'fa-solid fa-spinner' : undefined"
        >
          {{ modifCodeEnCours ? 'Enregistrement…' : 'Enregistrer' }}
        </AfricansBouton>
      </template>
    </AfricansModale>

    <!-- Le code n'est montré qu'ICI et qu'une fois : le serveur ne le renvoie
         jamais en clair ensuite. Le message le dit explicitement. -->
    <Transition name="af-surgir">
      <div
        v-if="toastCreation"
        class="fixed right-6 bottom-6 z-100 max-w-sm rounded-[10px] border border-af-vert bg-white p-5 shadow-xl font-af"
        role="status"
      >
        <div class="flex items-start gap-3">
          <font-awesome-icon icon="fa-solid fa-circle-check" class="mt-1 text-af-vert" />
          <div class="flex-1">
            <p class="text-[14px]/[1.4] font-bold">Salle privée créée</p>
            <p class="mt-1 text-[14px]/[1.4] text-af-corps">
              Code secret :
              <code class="rounded bg-af-bordure px-1.5 py-0.5 font-mono">{{ toastCreation.code }}</code>
            </p>
            <p class="mt-1 text-[12px]/[1.4] text-af-atone">Notez-le, il ne sera plus jamais affiché.</p>
          </div>
          <button
            type="button"
            class="text-af-atone transition hover:text-af-encre"
            aria-label="Fermer"
            @click="toastCreation = null"
          >
            <font-awesome-icon icon="fa-solid fa-xmark" />
          </button>
        </div>
      </div>
    </Transition>
  </NuxtLayout>
</template>

<style scoped>
.af-deplier-enter-active,
.af-deplier-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}
.af-deplier-enter-from,
.af-deplier-leave-to {
  opacity: 0;
  transform: translateY(-8px);
}

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
