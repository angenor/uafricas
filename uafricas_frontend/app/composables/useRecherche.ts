// Composable pour la recherche globale via les endpoints API reels

// ── Interfaces ──────────────────────────────────────────────────

export interface ResultatRecherche {
  id: string
  titre: string
  sousTexte?: string
  categorie: string
  icone: string
  lien: string
}

export interface GroupeResultats {
  categorie: string
  icone: string
  resultats: ResultatRecherche[]
  total: number
}

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

// ── Constantes ──────────────────────────────────────────────────

const CLE_RECHERCHES_RECENTES = 'uafricas_recherches_recentes'
const MAX_RECHERCHES_RECENTES = 5
const PAR_PAGE_RECHERCHE = 5

// ── Definition des sources de recherche ─────────────────────────

interface SourceRecherche {
  endpoint: string
  categorie: string
  icone: string
  extraireResultats: (data: any) => ResultatRecherche[]
}

const definirSources = (): SourceRecherche[] => [
  {
    endpoint: '/api/evenements',
    categorie: 'Événements',
    icone: 'fa-solid fa-calendar',
    extraireResultats: (data: any) =>
      (data.evenements || []).map((e: any) => ({
        id: e.id,
        titre: e.titre,
        sousTexte: `${e.type || ''} · ${e.pays || ''}${e.ville ? ', ' + e.ville : ''}`,
        categorie: 'Événements',
        icone: 'fa-solid fa-calendar',
        lien: `/evenements/${e.id}`,
      })),
  },
  {
    endpoint: '/api/annonces',
    categorie: 'Marché Africain',
    icone: 'fa-solid fa-store',
    extraireResultats: (data: any) =>
      (data.annonces || []).map((a: any) => ({
        id: a.id,
        titre: a.titre,
        sousTexte: `${a.categorie || ''} · ${a.type_echange || ''} · ${a.pays || ''}`,
        categorie: 'Marché Africain',
        icone: 'fa-solid fa-store',
        lien: `/marche-africain/${a.id}`,
      })),
  },
  {
    endpoint: '/api/livres',
    categorie: 'Bibliothèque',
    icone: 'fa-solid fa-book',
    extraireResultats: (data: any) =>
      (data.livres || []).map((l: any) => ({
        id: l.id,
        titre: l.titre,
        sousTexte: `${l.type_document || ''} · ${l.info_auteur || ''}`,
        categorie: 'Bibliothèque',
        icone: 'fa-solid fa-book',
        lien: `/bibliotheque/numerique`,
      })),
  },
  {
    endpoint: '/api/experts',
    categorie: 'Experts',
    icone: 'fa-solid fa-user-tie',
    extraireResultats: (data: any) =>
      (data.experts || []).map((exp: any) => ({
        id: exp.id,
        titre: `${exp.prenom || ''} ${exp.nom || ''}`.trim(),
        sousTexte: `${exp.expertiseInfo?.domaine || ''} · ${exp.pays || ''}`,
        categorie: 'Experts',
        icone: 'fa-solid fa-user-tie',
        lien: `/experts/${exp.id}`,
      })),
  },
  {
    endpoint: '/api/projets',
    categorie: 'Projets',
    icone: 'fa-solid fa-rocket',
    extraireResultats: (data: any) =>
      (data.projets || []).map((p: any) => ({
        id: p.id,
        titre: p.titre,
        sousTexte: `${p.organisation || ''} · ${p.pays || ''}`,
        categorie: 'Projets',
        icone: 'fa-solid fa-rocket',
        lien: `/financer-projet`,
      })),
  },
  {
    endpoint: '/api/television/chaines',
    categorie: 'Télévision',
    icone: 'fa-solid fa-tv',
    extraireResultats: (data: any) =>
      (data.chaines || []).map((c: any) => ({
        id: c.id,
        titre: c.nom,
        sousTexte: `${c.categorie || ''} · ${c.pays || ''}`,
        categorie: 'Télévision',
        icone: 'fa-solid fa-tv',
        lien: '/tele',
      })),
  },
  {
    endpoint: '/api/stations-radio',
    categorie: 'Radios',
    icone: 'fa-solid fa-radio',
    extraireResultats: (data: any) =>
      (data.stations || []).map((r: any) => ({
        id: r.id,
        titre: r.nom,
        sousTexte: `${r.genre || ''} · ${r.pays || ''}`,
        categorie: 'Radios',
        icone: 'fa-solid fa-radio',
        lien: '/radios',
      })),
  },
  {
    endpoint: '/api/centres-culturels',
    categorie: 'Centres Culturels',
    icone: 'fa-solid fa-masks-theater',
    extraireResultats: (data: any) => {
      // L'endpoint peut retourner un tableau ou un objet pagine
      const centres = Array.isArray(data) ? data : (data.centres || [])
      return centres.map((c: any) => ({
        id: c.id,
        titre: c.nom,
        sousTexte: c.adresse || c.adress || '',
        categorie: 'Centres Culturels',
        icone: 'fa-solid fa-masks-theater',
        lien: '/africa-culture',
      }))
    },
  },
  {
    endpoint: '/api/moocs',
    categorie: 'Formations',
    icone: 'fa-solid fa-graduation-cap',
    extraireResultats: (data: any) =>
      (data.moocs || []).map((m: any) => ({
        id: m.id,
        titre: m.titre,
        sousTexte: m.description ? m.description.substring(0, 60) + '...' : '',
        categorie: 'Formations',
        icone: 'fa-solid fa-graduation-cap',
        lien: `/universite/formations`,
      })),
  },
  {
    endpoint: '/api/sabbatiques',
    categorie: 'Échanges Sabbatiques',
    icone: 'fa-solid fa-plane',
    extraireResultats: (data: any) =>
      (data.programmes || []).map((s: any) => ({
        id: s.id,
        titre: s.titre,
        sousTexte: `${s.pays || ''}${s.ville ? ', ' + s.ville : ''}`,
        categorie: 'Échanges Sabbatiques',
        icone: 'fa-solid fa-plane',
        lien: '/echanges-sabbatiques',
      })),
  },
  {
    endpoint: '/api/fiches-pays',
    categorie: 'Opportunités Afrique',
    icone: 'fa-solid fa-earth-africa',
    extraireResultats: (data: any) => {
      const fiches = Array.isArray(data) ? data : (data.fiches || data.pays || [])
      return fiches.map((p: any) => ({
        id: p.id,
        titre: p.nom,
        sousTexte: `${p.region || ''} · Capitale : ${p.capitale || ''}`,
        categorie: 'Opportunités Afrique',
        icone: 'fa-solid fa-earth-africa',
        lien: `/opportunite-afrique/${p.id}`,
      }))
    },
  },
  {
    endpoint: '/api/africantives',
    categorie: 'Africantives',
    icone: 'fa-solid fa-lightbulb',
    extraireResultats: (data: any) =>
      (data.africantives || []).map((a: any) => ({
        id: a.id,
        titre: a.titre || a.nom,
        sousTexte: `${a.domaine || ''} · ${a.pays || ''}`,
        categorie: 'Africantives',
        icone: 'fa-solid fa-lightbulb',
        lien: `/africantives/${a.id}`,
      })),
  },
]

// ── Composable principal ────────────────────────────────────────

export const useRecherche = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string

  const termeRecherche = ref('')
  const resultatsGroupes = ref<GroupeResultats[]>([])
  const enChargement = ref(false)
  const nombreTotalResultats = ref(0)

  const sources = definirSources()

  // Recherches recentes (localStorage)
  const recherchesRecentes = ref<string[]>([])

  if (import.meta.client) {
    try {
      const stockees = localStorage.getItem(CLE_RECHERCHES_RECENTES)
      if (stockees) {
        recherchesRecentes.value = JSON.parse(stockees)
      }
    }
    catch {
      // Ignorer erreur localStorage
    }
  }

  const sauvegarderRecherche = (terme: string) => {
    if (!terme.trim()) return
    const recentes = recherchesRecentes.value.filter(r => r !== terme)
    recentes.unshift(terme)
    recherchesRecentes.value = recentes.slice(0, MAX_RECHERCHES_RECENTES)
    if (import.meta.client) {
      try {
        localStorage.setItem(CLE_RECHERCHES_RECENTES, JSON.stringify(recherchesRecentes.value))
      }
      catch {
        // Ignorer
      }
    }
  }

  const viderRecherchesRecentes = () => {
    recherchesRecentes.value = []
    if (import.meta.client) {
      localStorage.removeItem(CLE_RECHERCHES_RECENTES)
    }
  }

  const viderRecherche = () => {
    termeRecherche.value = ''
    resultatsGroupes.value = []
    nombreTotalResultats.value = 0
  }

  // Recherche parallele sur tous les endpoints
  const effectuerRecherche = async (terme: string) => {
    if (!terme.trim() || terme.trim().length < 2) {
      resultatsGroupes.value = []
      nombreTotalResultats.value = 0
      enChargement.value = false
      return
    }

    const params = new URLSearchParams({
      recherche: terme.trim(),
      par_page: String(PAR_PAGE_RECHERCHE),
      page: '1',
    })

    // Appeler tous les endpoints en parallele
    const promesses = sources.map(async (source): Promise<GroupeResultats | null> => {
      try {
        const url = `${apiBase}${source.endpoint}?${params.toString()}`
        const reponse = await $fetch<ApiResponse<any>>(url, { timeout: 5000 })

        if (!reponse.success || !reponse.data) return null

        const resultats = source.extraireResultats(reponse.data)
        if (resultats.length === 0) return null

        const total = reponse.data.total || resultats.length

        return {
          categorie: source.categorie,
          icone: source.icone,
          resultats,
          total,
        }
      }
      catch {
        // Endpoint indisponible ou erreur, on ignore silencieusement
        return null
      }
    })

    const resultats = await Promise.allSettled(promesses)

    const groupes: GroupeResultats[] = []
    let totalResultats = 0

    for (const resultat of resultats) {
      if (resultat.status === 'fulfilled' && resultat.value) {
        groupes.push(resultat.value)
        totalResultats += resultat.value.total
      }
    }

    // Trier les groupes par nombre de resultats
    groupes.sort((a, b) => b.total - a.total)

    resultatsGroupes.value = groupes
    nombreTotalResultats.value = totalResultats
    enChargement.value = false
  }

  // Debounce 350ms (un peu plus que 250ms pour les appels reseau)
  let timeoutId: ReturnType<typeof setTimeout> | null = null

  watch(termeRecherche, (nouveau) => {
    if (timeoutId) clearTimeout(timeoutId)

    if (!nouveau.trim() || nouveau.trim().length < 2) {
      resultatsGroupes.value = []
      nombreTotalResultats.value = 0
      enChargement.value = false
      return
    }

    enChargement.value = true
    timeoutId = setTimeout(() => {
      effectuerRecherche(nouveau)
    }, 350)
  })

  return {
    termeRecherche,
    resultatsGroupes,
    enChargement,
    nombreTotalResultats,
    recherchesRecentes,
    viderRecherche,
    viderRecherchesRecentes,
    sauvegarderRecherche,
  }
}
