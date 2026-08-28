<script setup lang="ts">
import { getCategoryLabel, type CodiMoiPostAPI, type CategoriePost, type CommentaireAPI } from '~/composables/useCodiMoi'
import { formaterDuree } from '~/mocks/vidafrica'
import type { PartageFicheAPI, PartageElementAPI } from '~/composables/useOpportuniteAfrique'
import type { PartageProfilAPI } from '~/composables/useMembres'
import type { PartageContributionAPI } from '~/composables/useGouvernance'
import type { PartageVideoAPI } from '~/composables/useVidafrica'
import type { PartageMediaAPI } from '~/composables/useMediaSocial'
import type { SalleAPI } from '~/composables/useAfrolang'
import type { ContributionCitoyenne } from '~/types/gouvernance'
import type { MembreLightAPI } from '~/composables/useAmis'
import type { AvisPublicResume } from '~/composables/useRetrouvAmis'
import type { AuteurAfricanitesAPI } from '~/composables/useAfricanite'
import type { MembreAPI } from '~/composables/useMembres'
import type { BrouillonCodimoi } from '~/components/codi-moi/PublierModale.vue'

/**
 * Fil d'actualité : le mur unique de la plateforme, porté sur le gabarit de la
 * refonte. Huit sources y convergent, triées par date.
 *
 * Le chargement est INCHANGÉ : mêmes huit appels en parallèle, mêmes filtres
 * côté client, même modale de détail Codimoi. Ce qui change est la carte : les
 * six sources de PARTAGE rendaient six composants distincts, chacun avec son
 * dégradé et sa palette, alors qu'elles partagent exactement la même anatomie
 * auteur, légende, aperçu de l'objet relayé. Elles passent sur une carte
 * unique, le type étant dit par le badge.
 *
 * Aucun cadre Figma ne décrit cet écran : la maquette dessine le « Fil
 * d'actualité » de sa barre latérale, mais son cadre « Accueil » est celui du
 * SITE PUBLIC : désormais servi à `/decouvrir`. Le bandeau reprend l'image de
 * ce cadre à titre provisoire.
 *
 * C'est la RACINE du site : la plateforme démarre sur le fil. `/publications`
 * reste servie et redirige ici : l'adresse a circulé en partages et en signets,
 * la casser romprait des liens déjà publiés.
 */
definePageMeta({ layout: false })

useHead({
  title: 'Publications de la Communauté | AfricanS',
})

type FiltreValue = 'tous' | 'codimoi' | 'factcheck' | 'ideaforces' | 'badhabits' | 'territoire_partage' | 'element_partage' | 'profil_partage' | 'video_partage' | 'media_partage' | 'afrolang_direct' | 'avis_recherche'

interface BasePublication {
  key: string
  date: Date
}

interface PublicationCodimoi extends BasePublication {
  source: 'codimoi'
  data: CodiMoiPostAPI
  typeFiltre: 'codimoi'
}

interface PublicationGouvernance extends BasePublication {
  source: 'gouvernance'
  data: ContributionCitoyenne
  typeFiltre: 'factcheck' | 'ideaforces' | 'badhabits'
}

interface PublicationTerritoirePartage extends BasePublication {
  source: 'territoire_partage'
  data: PartageFicheAPI
  typeFiltre: 'territoire_partage'
}

interface PublicationElementPartage extends BasePublication {
  source: 'element_partage'
  data: PartageElementAPI
  typeFiltre: 'element_partage'
}

interface PublicationProfilPartage extends BasePublication {
  source: 'profil_partage'
  data: PartageProfilAPI
  typeFiltre: 'profil_partage'
}

interface PublicationContributionPartage extends BasePublication {
  source: 'contribution_partage'
  data: PartageContributionAPI
  typeFiltre: 'factcheck' | 'ideaforces' | 'badhabits'
}

interface PublicationVideoPartage extends BasePublication {
  source: 'video_partage'
  data: PartageVideoAPI
  typeFiltre: 'video_partage'
}

interface PublicationMediaPartage extends BasePublication {
  source: 'media_partage'
  data: PartageMediaAPI
  typeFiltre: 'media_partage'
}

/**
 * Les avis de recherche Africonnect entrent au fil DIRECTEMENT, comme les
 * posts Codimoi et les contributions citoyennes — pas par un partage.
 *
 * Les huit autres sources de partage s'appuient chacune sur une table qui
 * garde une ligne par partage, avec son auteur et sa date. `avis_recherche`
 * n'a qu'un `compteur_partages` INTEGER : il n'existe ni ligne de partage,
 * ni endpoint pour les lister. Attendre un partage aurait donc voulu dire
 * n'afficher jamais aucun avis.
 */
interface PublicationAvisRecherche extends BasePublication {
  source: 'avis_recherche'
  data: AvisPublicResume
  typeFiltre: 'avis_recherche'
}

interface PublicationAfrolangDirect extends BasePublication {
  source: 'afrolang_direct'
  data: SalleAPI
  typeFiltre: 'afrolang_direct'
}

type Publication = PublicationAfrolangDirect | PublicationCodimoi | PublicationGouvernance | PublicationTerritoirePartage
  | PublicationElementPartage | PublicationProfilPartage | PublicationContributionPartage
  | PublicationVideoPartage | PublicationMediaPartage | PublicationAvisRecherche

/**
 * Les onze filtres du rail. La couleur d'habillage propre à chaque source a
 * disparu : dans la refonte le type est dit par le badge de la carte, et dix
 * jeux de dégradés ne disaient rien de plus que dix libellés.
 */
const FILTRES: { value: FiltreValue, label: string, icone: string }[] = [
  { value: 'tous', label: 'Toutes', icone: 'fa-solid fa-layer-group' },
  { value: 'codimoi', label: 'Codimoi', icone: 'fa-solid fa-quote-left' },
  { value: 'factcheck', label: 'FactCheck', icone: 'fa-solid fa-magnifying-glass' },
  { value: 'ideaforces', label: 'IdeaForces', icone: 'fa-solid fa-lightbulb' },
  { value: 'badhabits', label: 'BadGoodhabits', icone: 'fa-solid fa-triangle-exclamation' },
  { value: 'territoire_partage', label: 'Territoires partagés', icone: 'fa-solid fa-earth-africa' },
  { value: 'element_partage', label: 'Découvertes partagées', icone: 'fa-solid fa-share-nodes' },
  { value: 'profil_partage', label: 'Profils partagés', icone: 'fa-solid fa-user' },
  { value: 'video_partage', label: 'Vidéos partagées', icone: 'fa-solid fa-video' },
  { value: 'media_partage', label: 'Radio & télé', icone: 'fa-solid fa-tv' },
  { value: 'avis_recherche', label: 'Avis de recherche', icone: 'fa-solid fa-users' },
  { value: 'afrolang_direct', label: 'En direct', icone: 'fa-solid fa-video' }]

/**
 * Onglets de la maquette. « Pour vous » garde l'ordre du fil : le plus récent
 * d'abord. « Tendances » réordonne par ENGAGEMENT : likes plus commentaires,
 * les deux seuls compteurs que les sources tiennent réellement. Les partages
 * n'en portant aucun, ils descendent en bas de cet onglet ; c'est exact, pas
 * un défaut de tri.
 */
const ongletActif = ref<'pour-vous' | 'tendances'>('pour-vous')

const activeFilter = ref<FiltreValue>('tous')
const recherche = ref('')
const paysSelectionne = ref('')
const publications = ref<Publication[]>([])
const loading = ref(false)
const erreurChargement = ref<string | null>(null)

// APIs
const { erreur: erreurCodimoi, listerPosts, creerPost, reagir, listerCommentaires, creerCommentaire } = useCodiMoi()
const { getContributions, listerPartagesContributions } = useGouvernance()
const { listerPartagesFiches, listerPartagesElements } = useOpportuniteAfrique()
const { listerPartagesProfils } = useMembres()
const { listerPartagesVideos } = useVidafrica()
const { listerPartages: listerPartagesMedias } = useMediaSocial()
const { listerSalles } = useAfrolang()
const { rechercherAvisPublics, incrementerPartage } = useRetrouvAmis()

// Africanités en tête de fil (spec 012)
const { listerAfricanites, marquerVue } = useAfricanite()
const africanites = ref<AuteurAfricanitesAPI[]>([])
const composeurAfricaniteOuvert = ref(false)
const visionneuseOuverte = ref(false)
const auteurVise = ref<string | null>(null)

const chargerAfricanites = async () => {
  africanites.value = userStore.isAuthenticated ? await listerAfricanites() : []
}

/** Publier exige un compte : `POST /africanites` répondrait 401 après la saisie. */
const ouvrirAfricanite = () => {
  if (!userStore.isAuthenticated) {
    redirigerVersConnexion()
    return
  }
  composeurAfricaniteOuvert.value = true
}

const ouvrirVisionneuse = (auteurId: string) => {
  auteurVise.value = auteurId
  visionneuseOuverte.value = true
}

/**
 * L'anneau est corrigé LOCALEMENT à la volée, sans recharger : le serveur a
 * déjà enregistré la vue, et recharger la liste au milieu d'un enchaînement
 * ferait sauter la visionneuse.
 */
const surVue = async (africaniteId: string) => {
  await marquerVue(africaniteId)
  for (const groupe of africanites.value) {
    const a = groupe.africanites.find(x => x.id === africaniteId)
    if (!a) continue
    a.vue = true
    groupe.a_du_nouveau = groupe.africanites.some(x => !x.vue)
    break
  }
}

// Suggestions du rail
const { listerAmis } = useAmis()
const { listerMembres } = useMembres()
const amis = ref<MembreLightAPI[]>([])
const membresDecouverte = ref<MembreAPI[]>([])

// Composeur : le texte saisi amorce la modale de publication Codimoi
const publierOuvert = ref(false)
const brouillon = ref('')
const publicationEnCours = ref(false)

// Toast
const showToast = ref(false)
const toastMessage = ref('')

const notifier = (message: string) => {
  toastMessage.value = message
  showToast.value = true
  setTimeout(() => { showToast.value = false }, 2500)
}

// Modale codimoi
const selectedPost = ref<CodiMoiPostAPI | null>(null)
const selectedPostCommentaires = ref<CommentaireAPI[]>([])
const chargementCommentaires = ref(false)

// Compteurs par type
const compteurs = computed<Record<FiltreValue, number>>(() => {
  const c: Record<FiltreValue, number> = {
    tous: publications.value.length,
    codimoi: 0,
    factcheck: 0,
    ideaforces: 0,
    badhabits: 0,
    territoire_partage: 0,
    element_partage: 0,
    profil_partage: 0,
    video_partage: 0,
    media_partage: 0,
    afrolang_direct: 0,
  }
  for (const p of publications.value) {
    c[p.typeFiltre]++
  }
  return c
})

const paysDisponibles = computed(() => {
  const pays = new Set<string>()
  for (const p of publications.value) {
    const pays_ = paysPub(p)
    if (pays_) pays.add(pays_)
  }
  return Array.from(pays).sort()
})

const hasFiltresActifs = computed(() =>
  activeFilter.value !== 'tous' || !!recherche.value || !!paysSelectionne.value
)

const publicationsFiltrees = computed<Publication[]>(() => {
  return publications.value.filter(p => {
    // Filtre par catégorie
    if (activeFilter.value !== 'tous' && p.typeFiltre !== activeFilter.value) return false

    // Filtre par recherche
    if (recherche.value) {
      const q = recherche.value.toLowerCase()
      let titre: string
      let desc: string
      if (p.source === 'codimoi') {
        titre = titreCodimoi(p.data).toLowerCase()
        desc = (p.data.explication || p.data.contenu).toLowerCase()
      }
      else if (p.source === 'territoire_partage') {
        titre = p.data.fiche.nom.toLowerCase()
        desc = `${p.data.legende ?? ''} ${p.data.fiche.slogan ?? ''} ${p.data.fiche.region ?? ''} ${p.data.fiche.capitale ?? ''}`.toLowerCase()
      }
      else if (p.source === 'element_partage') {
        titre = p.data.element.titre.toLowerCase()
        desc = `${p.data.legende ?? ''} ${p.data.element.territoire_nom ?? ''}`.toLowerCase()
      }
      else if (p.source === 'profil_partage') {
        titre = `${p.data.profil.prenom} ${p.data.profil.nom}`.toLowerCase()
        desc = `${p.data.legende ?? ''} ${p.data.profil.fonction ?? ''} ${p.data.profil.pays ?? ''} ${p.data.profil.ville ?? ''}`.toLowerCase()
      }
      else if (p.source === 'contribution_partage') {
        titre = p.data.contribution.titre.toLowerCase()
        desc = `${p.data.legende ?? ''} ${p.data.contribution.description ?? ''} ${p.data.contribution.categorie ?? ''}`.toLowerCase()
      }
      else if (p.source === 'video_partage') {
        titre = p.data.video.titre.toLowerCase()
        desc = `${p.data.legende ?? ''}`.toLowerCase()
      }
      else if (p.source === 'afrolang_direct') {
        titre = p.data.titre.toLowerCase()
        desc = `${p.data.description ?? ''} ${p.data.langue_cible ?? ''}`.toLowerCase()
      }
      else if (p.source === 'avis_recherche') {
        titre = `${p.data.prenom_recherche ?? ''} ${p.data.nom_recherche}`.toLowerCase()
        desc = `${p.data.ecole_rencontre ?? ''} ${p.data.ville_rencontre ?? ''} ${p.data.localite_rencontre ?? ''} ${p.data.description_physique ?? ''}`.toLowerCase()
      }
      else {
        titre = p.data.titre.toLowerCase()
        desc = p.data.description.toLowerCase()
      }
      if (!titre.includes(q) && !desc.includes(q)) return false
    }

    // Filtre par pays
    if (paysSelectionne.value && paysPub(p) !== paysSelectionne.value) return false

    return true
  })
})

/** Engagement d'une publication : seuls Codimoi et la gouvernance en portent. */
function engagementDe(pub: Publication): number {
  // Un avis ne porte NI like NI commentaire : son seul compteur tenu par le
  // serveur est le partage. Il descend donc en bas de « Tendances », ce qui
  // est exact et non un defaut de tri.
  if (pub.source === 'avis_recherche') return pub.data.compteur_partages
  if (pub.source === 'afrolang_direct') return pub.data.sessions_en_cours
  if (pub.source === 'codimoi') return pub.data.nombre_likes + pub.data.nombre_commentaires
  if (pub.source === 'gouvernance') return pub.data.stats.likes + pub.data.stats.commentaires
  return 0
}

const publicationsAffichees = computed<Publication[]>(() => {
  if (ongletActif.value === 'pour-vous') return publicationsFiltrees.value
  return [...publicationsFiltrees.value].sort((a, b) => engagementDe(b) - engagementDe(a))
})

/**
 * Mots-dièse du rail, comptés sur le fil CHARGÉ, huit sources interrogées à
 * trente éléments chacune. Le panneau le dit : présentés comme un décompte de
 * plateforme, ces nombres seraient faux.
 */
/** La maquette montre quatre mots-dièse puis « Voir plus ». */
const TENDANCES_VISIBLES = 4
const tendancesDepliees = ref(false)

const hashtagsTendance = computed(() => {
  const comptes = new Map<string, number>()
  for (const pub of publications.value) {
    const tags = pub.source === 'codimoi'
      ? pub.data.hashtags
      : pub.source === 'gouvernance'
        ? pub.data.tags
        : null
    for (const tag of tags ?? []) {
      const clef = tag.trim()
      if (clef) comptes.set(clef, (comptes.get(clef) ?? 0) + 1)
    }
  }
  return [...comptes.entries()]
    .sort((a, b) => b[1] - a[1])
    .slice(0, 12)
    .map(([tag, total]) => ({ tag, total }))
})

const tendancesAffichees = computed(() =>
  tendancesDepliees.value ? hashtagsTendance.value : hashtagsTendance.value.slice(0, TENDANCES_VISIBLES))

// Helpers données
function titreCodimoi(post: CodiMoiPostAPI): string {
  if (isQuoteType(post.type)) {
    return getCategoryLabel(post.type as CategoriePost)
  }
  return post.contenu.length > 80 ? post.contenu.slice(0, 80) + '…' : post.contenu
}

function isQuoteType(type: string): boolean {
  return type === 'proverbe_adage' || type === 'citation'
}

function nomAuteur(pub: Publication): string {
  // L'anonymat est une GARANTIE du module, pas une donnée manquante : quand
  // l'auteur l'a choisi, le serveur ne transmet même pas de pseudonyme.
  if (pub.source === 'avis_recherche') {
    return pub.data.auteur_anonyme ? 'Anonyme' : (pub.data.auteur_pseudonyme || 'Un membre')
  }
  if (pub.source === 'afrolang_direct') {
    const admin = pub.data.administrateurs?.[0]
    return admin ? `${admin.prenom} ${admin.nom}`.trim() : 'AfricanS'
  }
  if (pub.source === 'codimoi') {
    const { prenom, nom } = pub.data.auteur
    return `${prenom ?? ''} ${nom}`.trim() || 'Anonyme'
  }
  if (pub.source === 'territoire_partage' || pub.source === 'element_partage' || pub.source === 'profil_partage' || pub.source === 'contribution_partage' || pub.source === 'video_partage' || pub.source === 'media_partage') {
    const { prenom, nom } = pub.data.auteur
    return `${prenom ?? ''} ${nom ?? ''}`.trim() || 'Anonyme'
  }
  return `${pub.data.auteur.prenom} ${pub.data.auteur.nom}`
}

function paysPub(pub: Publication): string | null {
  if (pub.source === 'afrolang_direct') return pub.data.pays_origine?.[0]?.nom || null
  if (pub.source === 'codimoi') return pub.data.pays || null
  if (pub.source === 'territoire_partage') return pub.data.fiche.nom || null
  if (pub.source === 'element_partage') return pub.data.element.territoire_nom || null
  if (pub.source === 'profil_partage') return pub.data.profil.pays || null
  if (pub.source === 'contribution_partage' || pub.source === 'video_partage' || pub.source === 'media_partage') return null
  if (pub.source === 'avis_recherche') return pub.data.pays?.nom || null
  return pub.data.localisation.pays || null
}

function formatDate(date: Date): string {
  return new Intl.DateTimeFormat('fr-FR', {
    day: 'numeric',
    month: 'short',
    year: 'numeric',
  }).format(date)
}

/**
 * Partage d'un avis depuis le fil : copie du lien public, puis incrément du
 * compteur serveur reporté LOCALEMENT. Recharger le fil entier pour un seul
 * chiffre ferait sauter la position de lecture.
 */
const partagerAvisFil = async (avis: AvisPublicResume) => {
  if (import.meta.client && navigator.clipboard) {
    navigator.clipboard.writeText(`${window.location.origin}/retrouve-amis/public/${avis.slug}`)
  }
  notifier("Lien de l'avis copié.")

  const res = await incrementerPartage(avis.slug)
  if (!res) return
  const pub = publications.value.find(p => p.source === 'avis_recherche' && p.data.id === avis.id)
  if (pub && pub.source === 'avis_recherche') {
    pub.data = { ...pub.data, compteur_partages: res.compteur_partages }
  }
}

const chargerTout = async () => {
  loading.value = true
  erreurChargement.value = null

  const [resCodimoi, resGouv, resPartages, resPartagesElements, resPartagesProfils, resPartagesContrib, resPartagesVideos, resPartagesMedias, resSalles, resAvis] = await Promise.allSettled([
    listerPosts({ page: 1, par_page: 30 }),
    getContributions({ page: 1, parPage: 30 }),
    listerPartagesFiches(1, 30),
    listerPartagesElements(1, 30),
    listerPartagesProfils(1, 30),
    listerPartagesContributions(1, 30),
    listerPartagesVideos(1, 30),
    listerPartagesMedias(1, 30),
    // Salles Afrolang EN COURS. Aucun endpoint public ne
    // liste les sessions actives toutes salles confondues ; la liste des salles
    // porte `sessions_en_cours`, un seul appel suffit donc à les trouver.
    listerSalles({ page: 1, par_page: 30 }),
    // Dixième source : les avis de recherche publics, servis tels quels.
    rechercherAvisPublics({ page: 1, par_page: 30 })])

  const items: Publication[] = []

  if (resCodimoi.status === 'fulfilled' && resCodimoi.value?.posts) {
    for (const p of resCodimoi.value.posts) {
      items.push({
        key: `codimoi-${p.id}`,
        source: 'codimoi',
        data: p,
        date: new Date(p.created_at),
        typeFiltre: 'codimoi',
      })
    }
  }
  else if (resCodimoi.status === 'rejected') {
    console.error('Erreur chargement Codimoi:', resCodimoi.reason)
  }

  if (resGouv.status === 'fulfilled' && resGouv.value?.contributions) {
    for (const c of resGouv.value.contributions) {
      items.push({
        key: `gouv-${c.id}`,
        source: 'gouvernance',
        data: c,
        date: c.dateCreation instanceof Date ? c.dateCreation : new Date(c.dateCreation),
        typeFiltre: c.type,
      })
    }
  }
  else if (resGouv.status === 'rejected') {
    console.error('Erreur chargement Gouvernance:', resGouv.reason)
  }

  if (resPartages.status === 'fulfilled' && resPartages.value?.partages) {
    for (const partage of resPartages.value.partages) {
      items.push({
        key: `partage-${partage.id}`,
        source: 'territoire_partage',
        data: partage,
        date: new Date(partage.created_at),
        typeFiltre: 'territoire_partage',
      })
    }
  }
  else if (resPartages.status === 'rejected') {
    console.error('Erreur chargement Partages:', resPartages.reason)
  }

  if (resPartagesElements.status === 'fulfilled' && resPartagesElements.value?.partages) {
    for (const partage of resPartagesElements.value.partages) {
      items.push({
        key: `partage-element-${partage.id}`,
        source: 'element_partage',
        data: partage,
        date: new Date(partage.created_at),
        typeFiltre: 'element_partage',
      })
    }
  }
  else if (resPartagesElements.status === 'rejected') {
    console.error('Erreur chargement Partages éléments:', resPartagesElements.reason)
  }

  if (resPartagesProfils.status === 'fulfilled' && resPartagesProfils.value?.partages) {
    for (const partage of resPartagesProfils.value.partages) {
      items.push({
        key: `partage-profil-${partage.id}`,
        source: 'profil_partage',
        data: partage,
        date: new Date(partage.created_at),
        typeFiltre: 'profil_partage',
      })
    }
  }
  else if (resPartagesProfils.status === 'rejected') {
    console.error('Erreur chargement Partages profils:', resPartagesProfils.reason)
  }

  if (resPartagesContrib.status === 'fulfilled' && resPartagesContrib.value?.partages) {
    for (const partage of resPartagesContrib.value.partages) {
      items.push({
        key: `partage-contrib-${partage.id}`,
        source: 'contribution_partage',
        data: partage,
        date: new Date(partage.created_at),
        typeFiltre: partage.contribution.type_contribution,
      })
    }
  }
  else if (resPartagesContrib.status === 'rejected') {
    console.error('Erreur chargement Partages contributions:', resPartagesContrib.reason)
  }

  if (resPartagesVideos.status === 'fulfilled' && resPartagesVideos.value?.partages) {
    for (const partage of resPartagesVideos.value.partages) {
      items.push({
        key: `partage-video-${partage.id}`,
        source: 'video_partage',
        data: partage,
        date: new Date(partage.created_at),
        typeFiltre: 'video_partage',
      })
    }
  }
  else if (resPartagesVideos.status === 'rejected') {
    console.error('Erreur chargement Partages vidéos:', resPartagesVideos.reason)
  }

  if (resPartagesMedias.status === 'fulfilled' && resPartagesMedias.value?.partages) {
    for (const partage of resPartagesMedias.value.partages) {
      items.push({
        key: `partage-media-${partage.id}`,
        source: 'media_partage',
        data: partage,
        date: new Date(partage.created_at),
        typeFiltre: 'media_partage',
      })
    }
  }
  else if (resPartagesMedias.status === 'rejected') {
    console.error('Erreur chargement Partages médias:', resPartagesMedias.reason)
  }

  if (resSalles.status === 'fulfilled' && resSalles.value?.salles) {
    for (const salle of resSalles.value.salles) {
      if (salle.sessions_en_cours <= 0) continue
      items.push({
        key: `direct-${salle.id}`,
        source: 'afrolang_direct',
        data: salle,
        // `updated_at` est le seul horodatage que porte la liste des salles :
        // la date d'ouverture de la session n'y figure pas. Une salle allumée
        // depuis longtemps peut donc passer sous une publication plus récente.
        date: new Date(salle.updated_at),
        typeFiltre: 'afrolang_direct',
      })
    }
  }
  if (resAvis.status === 'fulfilled' && resAvis.value?.avis) {
    for (const avis of resAvis.value.avis) {
      items.push({
        key: `avis-${avis.id}`,
        source: 'avis_recherche',
        data: avis,
        date: new Date(avis.created_at),
        typeFiltre: 'avis_recherche',
      })
    }
  }
  else if (resAvis.status === 'rejected') {
    console.error('Erreur chargement Avis de recherche:', resAvis.reason)
  }

  if (resSalles.status === 'rejected') {
    console.error('Erreur chargement salles Afrolang:', resSalles.reason)
  }

  items.sort((a, b) => b.date.getTime() - a.date.getTime())
  publications.value = items

  if (items.length === 0 && resCodimoi.status === 'rejected' && resGouv.status === 'rejected') {
    erreurChargement.value = 'Impossible de charger les publications'
  }

  loading.value = false
}

const reinitialiser = () => {
  activeFilter.value = 'tous'
  recherche.value = ''
  paysSelectionne.value = ''
}

/**
 * Réaction depuis le FIL (la modale a la sienne). Le post rendu est celui du
 * tableau `publications` : `reagir` renvoie déjà l'objet à jour, remplacer la
 * ligne suffit : recharger les huit sources pour un pouce ferait tout sauter.
 */
const reagirFil = async (post: CodiMoiPostAPI, type: 'like' | 'dislike') => {
  const updated = await reagir(post.id, type)
  if (updated) mettreAJourPostCodimoi(updated)
}

/** Le composeur ne publie pas seul : il passe la main à la modale Codimoi,
 *  qui exige en plus une catégorie et un territoire. */
const ouvrirComposeur = (texte = '') => {
  // Un visiteur peut lire le fil : c'est la racine, mais pas y publier :
  // `creerPost` répondrait 401 après la saisie complète du formulaire.
  if (!userStore.isAuthenticated) {
    redirigerVersConnexion()
    return
  }
  brouillon.value = texte
  publierOuvert.value = true
}

const publierDepuisLeFil = async (donnees: BrouillonCodimoi & { hashtags: string[] }) => {
  publicationEnCours.value = true
  const nouveau = await creerPost({
    type: donnees.categorie,
    contenu: donnees.contenu,
    explication: donnees.explication || undefined,
    nom_auteur_originel: donnees.nomAuteur || undefined,
    pays: donnees.pays || undefined,
    groupe_ethnique: donnees.groupeEthnique || undefined,
    couleur_fond: donnees.couleurFond || undefined,
    hashtags: donnees.hashtags.length ? donnees.hashtags : undefined,
  })
  publicationEnCours.value = false

  // La modale ne se referme QUE si la publication a abouti : la refermer sur
  // un échec jetterait la saisie avec elle.
  if (!nouveau) {
    notifier(erreurCodimoi.value || 'Erreur lors de la création')
    return
  }
  publierOuvert.value = false
  brouillon.value = ''
  notifier('Publication créée.')
  await chargerTout()
}

/** Codimoi n'enregistre pas les partages : l'action copie un lien. */
const partagerCodimoi = (post: CodiMoiPostAPI) => {
  if (import.meta.client && navigator.clipboard) {
    navigator.clipboard.writeText(`${window.location.origin}/codi-moi/${post.id}`)
  }
  notifier('Lien copié dans le presse-papiers.')
}

// Actions Codimoi (modale)
const ouvrirPostCodimoi = async (post: CodiMoiPostAPI) => {
  selectedPost.value = post
  chargementCommentaires.value = true
  selectedPostCommentaires.value = []

  const resultat = await listerCommentaires(post.id)
  if (resultat) {
    selectedPostCommentaires.value = resultat.commentaires
  }
  chargementCommentaires.value = false
}

const fermerPostCodimoi = () => {
  selectedPost.value = null
  selectedPostCommentaires.value = []
}

const mettreAJourPostCodimoi = (updated: CodiMoiPostAPI) => {
  const index = publications.value.findIndex(
    p => p.source === 'codimoi' && p.data.id === updated.id
  )
  if (index !== -1) {
    const current = publications.value[index] as PublicationCodimoi
    publications.value[index] = { ...current, data: updated }
  }
  if (selectedPost.value?.id === updated.id) {
    selectedPost.value = updated
  }
}

const reagirModalCodimoi = async (type: 'like' | 'dislike') => {
  if (!selectedPost.value) return
  const updated = await reagir(selectedPost.value.id, type)
  if (updated) mettreAJourPostCodimoi(updated)
}

const commenterModalCodimoi = async (contenu: string) => {
  if (!selectedPost.value) return
  const commentaire = await creerCommentaire(selectedPost.value.id, contenu)
  if (commentaire) {
    selectedPostCommentaires.value.unshift(commentaire)
    if (selectedPost.value) {
      selectedPost.value.nombre_commentaires++
    }
    const index = publications.value.findIndex(
      p => p.source === 'codimoi' && p.data.id === selectedPost.value?.id
    )
    if (index !== -1) {
      const current = publications.value[index] as PublicationCodimoi
      current.data.nombre_commentaires++
    }
  }
  else {
    notifier(erreurCodimoi.value || 'Erreur lors de la publication du commentaire')
  }
}

const partagerModalCodimoi = () => {
  if (!selectedPost.value) return
  if (import.meta.client && navigator.clipboard) {
    navigator.clipboard.writeText(`${window.location.origin}/codi-moi/${selectedPost.value.id}`)
    notifier('Lien copié dans le presse-papiers !')
  }
}

// ── Mise en forme des partages ────────────────────────────────────────────
// Les six sources de partage rendent la MÊME carte : seuls le verbe, le badge
// et l'aperçu changent. Ces fonctions produisent ces trois-là, rien d'autre.

const userStore = useUserStore()
// Composables appelés au SETUP : hors de la portée du composant, Nuxt ne sait
// plus à quelle instance les rattacher.
const { initAuth, redirigerVersConnexion } = useAuth()
const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string

const resoudreUrl = (url: string | null | undefined): string | null => {
  if (!url) return null
  return url.startsWith('http') ? url : `${apiBase}${url}`
}

/** Vocabulaire des découvertes Afripulse : « a partagé une recette » ne se dit
 *  pas comme « a partagé un secteur d'opportunité ». */
const METAS_ELEMENT: Record<string, { article: string, badge: string, segment: string, icone: string }> = {
  secteur_developpement: { article: "un secteur d'opportunité", badge: 'Secteur', segment: 'secteurs', icone: 'fa-solid fa-briefcase' },
  recette_culinaire: { article: 'une recette', badge: 'Recette', segment: 'recettes', icone: 'fa-solid fa-utensils' },
  site_touristique: { article: 'un site touristique', badge: 'Site', segment: 'sites', icone: 'fa-solid fa-location-dot' },
  personnalite_connue: { article: 'une personnalité', badge: 'Personnalité', segment: 'personnalites', icone: 'fa-solid fa-user' },
}

/** Même principe pour les six types de médias radio et télé. */
const METAS_MEDIA: Record<string, { article: string, badge: string, icone: string }> = {
  chaine_tv: { article: 'une chaîne de télévision', badge: 'Chaîne', icone: 'fa-solid fa-tv' },
  station_radio: { article: 'une station de radio', badge: 'Station', icone: 'fa-solid fa-radio' },
  emission_tele: { article: 'un programme de télévision', badge: 'Programme TV', icone: 'fa-solid fa-layer-group' },
  emission_radio: { article: 'un programme de radio', badge: 'Programme radio', icone: 'fa-solid fa-layer-group' },
  episode_tele: { article: 'un épisode de télévision', badge: 'Épisode TV', icone: 'fa-solid fa-tv' },
  episode_radio: { article: 'un épisode de radio', badge: 'Épisode radio', icone: 'fa-solid fa-microphone' },
}

const LABELS_CONTRIBUTION: Record<string, { label: string, path: string }> = {
  factcheck: { label: 'FactCheck', path: '/universite/gouvernance/factcheck' },
  ideaforces: { label: 'IdeaForces', path: '/universite/gouvernance/ideaforces' },
  badhabits: { label: 'BadGoodhabits', path: '/universite/gouvernance/bad-good-habits' },
}

/**
 * Traduit une publication de partage en propriétés de `PublicationsCartePartage`.
 * Renvoie `null` pour les deux sources qui ne sont PAS des partages (Codimoi et
 * gouvernance) : elles ont leur propre carte.
 */
function partageEn(pub: Publication) {
  const auteurBase = { nom: nomAuteur(pub), photo: resoudreUrl((pub.data as any).auteur?.photo_url) }
  const quand = formatDate(pub.date)

  if (pub.source === 'territoire_partage') {
    const f = pub.data.fiche
    return {
      auteur: { ...auteurBase, action: 'a partagé un territoire' },
      categorie: 'Territoire',
      legende: pub.data.legende,
      quand,
      apercu: {
        titre: f.nom,
        sousTitre: f.slogan ?? undefined,
        meta: [f.region, f.capitale ? `Capitale : ${f.capitale}` : null].filter(Boolean) as string[],
        image: resoudreUrl(f.image_couverture),
        icone: 'fa-solid fa-earth-africa',
        vers: `/opportunite-afrique/${f.id}`,
      },
    }
  }

  if (pub.source === 'element_partage') {
    const e = pub.data.element
    const meta = METAS_ELEMENT[e.type_objet] ?? { article: 'une découverte', badge: 'Découverte', segment: 'secteurs', icone: 'fa-solid fa-share-nodes' }
    return {
      auteur: { ...auteurBase, action: `a partagé ${meta.article}` },
      categorie: meta.badge,
      legende: pub.data.legende,
      quand,
      apercu: {
        titre: e.titre,
        meta: [e.territoire_nom].filter(Boolean) as string[],
        image: resoudreUrl(e.image_url),
        icone: meta.icone,
        vers: `/opportunite-afrique/${e.fiche_pays_id}/${meta.segment}/${e.objet_id}`,
      },
    }
  }

  if (pub.source === 'profil_partage') {
    const p = pub.data.profil
    return {
      auteur: { ...auteurBase, action: 'a partagé un profil' },
      categorie: 'Profil',
      legende: pub.data.legende,
      quand,
      apercu: {
        titre: `${p.prenom} ${p.nom}`.trim(),
        sousTitre: p.fonction ?? undefined,
        meta: [p.ville, p.pays].filter(Boolean) as string[],
        image: resoudreUrl(p.photo_url),
        icone: 'fa-solid fa-user',
        vers: `/profil/${p.id}`,
      },
    }
  }

  if (pub.source === 'contribution_partage') {
    const c = pub.data.contribution
    const meta = LABELS_CONTRIBUTION[c.type_contribution] ?? { label: 'Publication', path: '/universite/gouvernance' }
    return {
      auteur: { ...auteurBase, action: 'a partagé une publication' },
      categorie: meta.label,
      legende: pub.data.legende,
      quand,
      apercu: {
        titre: c.titre,
        sousTitre: c.description ?? undefined,
        meta: [c.categorie].filter(Boolean) as string[],
        image: resoudreUrl(c.image_couverture_url),
        icone: 'fa-solid fa-scale-balanced',
        vers: `${meta.path}?pub=${c.id}`,
      },
    }
  }

  if (pub.source === 'video_partage') {
    const v = pub.data.video
    return {
      auteur: { ...auteurBase, action: 'a partagé une vidéo' },
      categorie: 'Vidéo',
      legende: pub.data.legende,
      quand,
      apercu: {
        titre: v.titre,
        image: resoudreUrl(v.vignette_url),
        surImage: v.duree_secondes ? formaterDuree(v.duree_secondes) : undefined,
        icone: 'fa-solid fa-video',
        vers: `/vidafrica/${v.slug}`,
      },
    }
  }

  if (pub.source === 'media_partage') {
    const m = pub.data.media
    const meta = METAS_MEDIA[m.type_media] ?? { article: 'un média', badge: 'Média', icone: 'fa-solid fa-tv' }
    return {
      auteur: { ...auteurBase, action: `a partagé ${meta.article}` },
      categorie: meta.badge,
      legende: pub.data.legende,
      quand,
      // `url` est calculée côté serveur et peut manquer : on retombe alors sur
      // la vitrine plutôt que sur un lien mort.
      apercu: {
        titre: m.titre,
        image: resoudreUrl(m.image_url),
        icone: meta.icone,
        vers: m.url ?? '/medias',
      },
    }
  }

  return null
}

onMounted(async () => {
  await initAuth()
  await chargerTout()

  // Suggestions du rail : publiques, elles ne dépendent pas de la session.
  const liste = await listerMembres({ par_page: 5 })
  if (liste) membresDecouverte.value = liste.membres

  // Africanités et ami(e)s n'existent que pour un membre connecté.
  if (userStore.isAuthenticated) {
    const mesAmis = await listerAmis()
    amis.value = mesAmis.map(a => a.utilisateur)
    await chargerAfricanites()
  }
})
</script>

<template>
  <NuxtLayout name="africans">
    <!-- Pas de bandeau de module ni de fil d'Ariane : le fil d'actualité est la
         RACINE, il n'est le sous-écran de rien. La maquette le montre démarrant
         directement sous la barre supérieure. -->

    <div class="flex flex-col gap-6">
      <!-- Toujours rendue, même sans africanité et même pour un visiteur : la
           maquette la montre en tête de fil, et le cercle « + » qui l'ouvre est
           une entrée en soi. La masquer la ferait disparaître pour les deux
           publics qui en ont le plus besoin. -->
      <AfricansRangeeAfricanites
        :groupes="africanites"
        @publier="ouvrirAfricanite"
        @ouvrir="ouvrirVisionneuse"
      />

      <PublicationsComposeur @publier="ouvrirComposeur" />

      <AfricansOnglets
        v-model="ongletActif"
        :onglets="[
          { valeur: 'pour-vous', libelle: 'Pour vous' },
          { valeur: 'tendances', libelle: 'Tendances' }]"
      />

      <p class="text-[14px]/[1.4] text-af-corps">
        <strong class="font-bold">{{ publicationsFiltrees.length }}</strong>
        résultat{{ publicationsFiltrees.length > 1 ? 's' : '' }}
        <template v-if="hasFiltresActifs">sur {{ publications.length }} publications</template>
      </p>

      <!-- Chargement : squelettes à l'anatomie d'une carte du fil. -->
      <div v-if="loading" class="flex flex-col gap-6">
        <div v-for="n in 3" :key="n" class="overflow-hidden rounded-[10px] border border-af-bordure bg-white">
          <div class="flex items-center gap-3 p-4">
            <div class="size-11 animate-pulse rounded-full bg-af-bordure" />
            <div class="flex-1 space-y-2">
              <div class="h-3 w-1/3 animate-pulse rounded bg-af-bordure" />
              <div class="h-3 w-1/4 animate-pulse rounded bg-af-bordure" />
            </div>
          </div>
          <div class="aspect-[16/10] w-full animate-pulse bg-af-bordure" />
          <div class="h-10" />
        </div>
      </div>

      <!-- Erreur : le message technique est montré, pas masqué derrière un
           « une erreur est survenue » qui n'aide personne à diagnostiquer. -->
      <div v-else-if="erreurChargement" class="rounded-[10px] border border-af-live/30 bg-af-live/[0.05] p-6">
        <p class="flex items-center gap-3 text-[16px]/[1.4] font-bold">
          <font-awesome-icon icon="fa-solid fa-circle-exclamation" class="text-af-live" />
          Le fil n'a pas pu être chargé
        </p>
        <p class="mt-2 text-[14px]/[1.4] text-af-corps">{{ erreurChargement }}</p>
        <AfricansBouton class="mt-5" icone="fa-solid fa-rotate-right" @click="chargerTout">
          Réessayer
        </AfricansBouton>
      </div>

      <div v-else-if="publicationsAffichees.length" class="flex flex-col gap-6">
        <template v-for="pub in publicationsAffichees" :key="pub.key">
          <!-- Publication Codimoi : la carte du module, telle quelle. -->
          <CodiMoiCartePost
            v-if="pub.source === 'codimoi'"
            :post="pub.data"
            @jaime="reagirFil(pub.data, 'like')"
            @jaime-pas="reagirFil(pub.data, 'dislike')"
            @commenter="ouvrirPostCodimoi(pub.data)"
            @partager="partagerCodimoi(pub.data)"
          />

          <!-- Salle Afrolang en direct. -->
          <PublicationsCarteDirect
            v-else-if="pub.source === 'afrolang_direct'"
            :salle="pub.data"
          />

          <!-- Contribution citoyenne : FactCheck, IdeaForces, BadGoodhabits. -->
          <PublicationsCarteContribution
            v-else-if="pub.source === 'gouvernance'"
            :contribution="pub.data"
          />

          <!-- Avis de recherche Africonnect. `CarteAvisFil` existait déjà,
               écrite pour le fil, mais n'était montée que sur /retrouve-amis. -->
          <RetrouveAmisCarteAvisFil
            v-else-if="pub.source === 'avis_recherche'"
            :avis="pub.data"
            dans-le-fil
            @partager="partagerAvisFil(pub.data)"
          />

          <!-- Les six sources de partage : une seule et même carte. -->
          <PublicationsCartePartage v-else v-bind="partageEn(pub)!" />
        </template>
      </div>

      <div v-else class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
        <font-awesome-icon icon="fa-solid fa-layer-group" class="text-4xl text-af-atone-2" />
        <p class="mt-4 text-[16px]/[1.4] font-bold">
          {{ hasFiltresActifs ? 'Aucun résultat pour ces critères' : 'Le fil est encore vide' }}
        </p>
        <p class="mt-2 text-[14px]/[1.4] text-af-corps">
          {{ hasFiltresActifs
            ? 'Essayez une autre catégorie, un autre territoire, ou repartez de zéro.'
            : 'Les publications des modules de la plateforme apparaîtront ici.' }}
        </p>
        <AfricansBouton
          v-if="hasFiltresActifs"
          class="mt-5"
          variante="secondaire"
          icone="fa-solid fa-rotate-left"
          @click="reinitialiser"
        >
          Réinitialiser les filtres
        </AfricansBouton>
      </div>
    </div>

    <template #rail>
      <AfricansRecherche v-model="recherche" placeholder="Mot-clé…" />

      <!-- Mots-dièse du fil chargé. La maquette affiche « 48,57k Posts » ;
           aucun endpoint ne compte les tags sur tout le fonds, ceux-ci sont
           donc comptés sur ce qui est à l'écran, et le panneau le dit. -->
      <!-- Mots-dièse du fil chargé. La maquette affiche « 48,57k Posts » ;
           aucun endpoint ne compte les tags sur tout le fonds, ceux-ci sont
           donc comptés sur ce qui est à l'écran, et le panneau le dit. -->
      <AfricansPanneau v-if="hashtagsTendance.length" titre="Tendances" icone="fa-solid fa-arrow-trend-up">
        <ul class="flex flex-col">
          <li
            v-for="(entree, i) in tendancesAffichees"
            :key="entree.tag"
            class="border-t border-af-bordure first:border-t-0"
          >
            <button
              type="button"
              class="flex w-full items-center gap-3 py-3 text-left transition hover:text-af-chocolat"
              @click="recherche = entree.tag"
            >
              <span class="min-w-0 flex-1">
                <span class="block truncate text-[17px]/[1.4] font-bold">#{{ entree.tag }}</span>
                <span class="text-[14px]/[1.4] text-af-atone">
                  {{ entree.total }} publication{{ entree.total > 1 ? 's' : '' }}
                </span>
              </span>
              <!-- La flamme marque le mot-dièse le plus repris du fil, et lui
                   seul : la poser sur plusieurs lignes n'en distinguerait
                   aucune. -->
              <font-awesome-icon v-if="i === 0" icon="fa-solid fa-fire" class="shrink-0 text-af-chocolat" />
            </button>
          </li>
        </ul>

        <!-- « Voir plus » DÉPLIE la liste, il ne mène nulle part : aucune page
             ne recense les mots-dièse. Il n'apparaît que s'il reste à voir. -->
        <button
          v-if="hashtagsTendance.length > TENDANCES_VISIBLES"
          type="button"
          class="mt-3 flex items-center gap-2 text-[16px]/[1.4] font-bold text-af-chocolat transition hover:opacity-70"
          @click="tendancesDepliees = !tendancesDepliees"
        >
          {{ tendancesDepliees ? 'Voir moins' : 'Voir plus' }}
          <font-awesome-icon
            icon="fa-solid fa-chevron-right"
            class="transition-transform"
            :class="tendancesDepliees && 'rotate-90'"
          />
        </button>

        <p class="mt-3 text-[12px]/[1.4] text-af-atone">
          Comptés sur les {{ publications.length }} publications chargées.
        </p>
      </AfricansPanneau>

      <AfricansPanneau
        titre="Filtres"
        icone="fa-solid fa-sliders"
        :action-libelle="hasFiltresActifs ? 'Réinitialiser' : undefined"
        @action="reinitialiser"
      >
        <ul class="flex flex-col gap-1">
          <li v-for="filtre in FILTRES" :key="filtre.value">
            <button
              type="button"
              class="flex w-full items-center gap-3 rounded-lg px-3 py-2 text-left text-[14px]/[1.4] transition"
              :class="activeFilter === filtre.value
                ? 'bg-af-chocolat/15 font-bold text-af-chocolat'
                : 'text-af-corps hover:bg-af-chocolat/[0.07]'"
              :aria-pressed="activeFilter === filtre.value"
              @click="activeFilter = filtre.value"
            >
              <font-awesome-icon :icon="filtre.icone" class="shrink-0" />
              <span class="min-w-0 flex-1 truncate">{{ filtre.label }}</span>
              <!-- Le compteur porte sur le fil CHARGÉ, pas sur le fonds : les
                   huit sources sont interrogées à 30 éléments chacune. -->
              <span class="shrink-0 text-[12px]/[1.4] text-af-atone">{{ compteurs[filtre.value] }}</span>
            </button>
          </li>
        </ul>

        <AfricansChamp v-model="paysSelectionne" libelle="Territoire" type="select" class="mt-5">
          <option value="">Tous les territoires</option>
          <option v-for="pays in paysDisponibles" :key="pays" :value="pays">{{ pays }}</option>
        </AfricansChamp>
      </AfricansPanneau>

      <!-- La maquette titre « Créateur à suivre ». Aucun abonnement n'existe :
           promettre « suivre » afficherait un bouton qui ne mène nulle part.
           Les membres, eux, sont réels, et la pastille verte dit une
           expertise VALIDÉE, pas une popularité. -->
      <AfricansPanneau v-if="membresDecouverte.length" titre="Membres à découvrir" icone="fa-solid fa-users">
        <ul class="flex flex-col">
          <li
            v-for="membre in membresDecouverte"
            :key="membre.id"
            class="border-t border-af-bordure first:border-t-0"
          >
            <NuxtLink
              :to="`/profil/${membre.id}`"
              class="flex items-center gap-3 py-3 transition hover:text-af-chocolat"
            >
              <AfricansAvatar
                :nom="`${membre.prenom} ${membre.nom}`"
                :src="resoudreUrl(membre.photoUrl)"
                :taille="40"
              />
              <span class="min-w-0 flex-1">
                <span class="flex items-center gap-2 text-[14px]/[1.4] font-bold">
                  <span class="truncate">{{ membre.prenom }} {{ membre.nom }}</span>
                  <!-- Rosette de la maquette, en chocolat. Elle dit une
                       expertise VALIDÉE par la plateforme, pas une popularité. -->
                  <font-awesome-icon
                    v-if="membre.estExpert"
                    icon="fa-solid fa-certificate"
                    class="shrink-0 text-af-chocolat"
                    title="Expertise validée"
                  />
                </span>
                <span v-if="membre.fonction" class="block truncate text-[14px]/[1.4] text-af-atone">
                  {{ membre.fonction }}
                </span>
              </span>
            </NuxtLink>
          </li>
        </ul>
      </AfricansPanneau>
    </template>

    <AfricansComposeurAfricanite
      v-model="composeurAfricaniteOuvert"
      @publiee="chargerAfricanites"
    />

    <AfricansVisionneuseAfricanite
      v-model="visionneuseOuverte"
      :groupes="africanites"
      :auteur-initial="auteurVise"
      @vue="surVue"
    />

    <CodiMoiPublierModale
      v-model="publierOuvert"
      :en-cours="publicationEnCours"
      :contenu-initial="brouillon"
      @publier="publierDepuisLeFil"
    />

    <CodiMoiPostModal
      :post="selectedPost"
      :commentaires="selectedPostCommentaires"
      :chargement-commentaires="chargementCommentaires"
      @close="fermerPostCodimoi"
      @like="reagirModalCodimoi('like')"
      @dislike="reagirModalCodimoi('dislike')"
      @share="partagerModalCodimoi"
      @commenter="commenterModalCodimoi"
    />

    <Transition name="af-surgir">
      <div
        v-if="showToast"
        class="fixed right-6 bottom-6 z-100 rounded-[10px] border border-af-vert bg-white px-5 py-4 shadow-xl font-af"
        role="status"
      >
        <p class="flex items-center gap-3 text-[14px]/[1.4]">
          <font-awesome-icon icon="fa-solid fa-circle-check" class="text-af-vert" />
          {{ toastMessage }}
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
