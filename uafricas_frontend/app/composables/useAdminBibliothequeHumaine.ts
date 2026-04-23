import { demandesBiblioHumaineReactif, type DemandeBiblioHumaine, type StatutDemande } from '~/mocks/bibliotheques-humaines'

const simulerDelai = () => new Promise<void>(resolve => setTimeout(resolve, 300))

export interface FiltresDemandes {
  statut?: StatutDemande | ''
  recherche?: string
}

export function useAdminBibliothequeHumaine() {
  const chargement = ref(false)
  const erreur = ref<string | null>(null)
  const demandes = ref<DemandeBiblioHumaine[]>([])
  const demandeDetail = ref<DemandeBiblioHumaine | null>(null)

  const nbEnAttente = computed(
    () => demandesBiblioHumaineReactif.value.filter(d => d.statut === 'en_attente').length,
  )

  async function listerDemandes(filtres: FiltresDemandes = {}): Promise<void> {
    chargement.value = true
    erreur.value = null
    try {
      await simulerDelai()
      let liste = [...demandesBiblioHumaineReactif.value]

      if (filtres.statut) {
        liste = liste.filter(d => d.statut === filtres.statut)
      }
      if (filtres.recherche) {
        const q = filtres.recherche.toLowerCase()
        liste = liste.filter(
          d =>
            d.nom.toLowerCase().includes(q)
            || d.prenom.toLowerCase().includes(q)
            || d.fonction.toLowerCase().includes(q)
            || d.pays.toLowerCase().includes(q),
        )
      }

      liste.sort((a, b) => new Date(b.dateSubmission).getTime() - new Date(a.dateSubmission).getTime())
      demandes.value = liste
    }
    catch (e: unknown) {
      erreur.value = e instanceof Error ? e.message : 'Erreur inconnue'
    }
    finally {
      chargement.value = false
    }
  }

  async function obtenirDemande(id: string): Promise<void> {
    chargement.value = true
    erreur.value = null
    try {
      await simulerDelai()
      const trouve = demandesBiblioHumaineReactif.value.find(d => d.id === id)
      if (!trouve) throw new Error('Demande introuvable')
      demandeDetail.value = { ...trouve }
    }
    catch (e: unknown) {
      erreur.value = e instanceof Error ? e.message : 'Erreur inconnue'
    }
    finally {
      chargement.value = false
    }
  }

  async function approuverDemande(id: string): Promise<void> {
    chargement.value = true
    erreur.value = null
    try {
      await simulerDelai()
      const idx = demandesBiblioHumaineReactif.value.findIndex(d => d.id === id)
      if (idx === -1) throw new Error('Demande introuvable')
      demandesBiblioHumaineReactif.value = demandesBiblioHumaineReactif.value.map(d =>
        d.id === id
          ? { ...d, statut: 'valide' as StatutDemande, dateMaj: new Date().toISOString(), commentaireAdmin: null }
          : d,
      )
      if (demandeDetail.value?.id === id) {
        demandeDetail.value = {
          ...demandeDetail.value,
          statut: 'valide',
          dateMaj: new Date().toISOString(),
          commentaireAdmin: null,
        }
      }
    }
    catch (e: unknown) {
      erreur.value = e instanceof Error ? e.message : 'Erreur inconnue'
      throw e
    }
    finally {
      chargement.value = false
    }
  }

  async function rejeterDemande(id: string, commentaire?: string): Promise<void> {
    chargement.value = true
    erreur.value = null
    try {
      await simulerDelai()
      const idx = demandesBiblioHumaineReactif.value.findIndex(d => d.id === id)
      if (idx === -1) throw new Error('Demande introuvable')
      demandesBiblioHumaineReactif.value = demandesBiblioHumaineReactif.value.map(d =>
        d.id === id
          ? {
              ...d,
              statut: 'rejete' as StatutDemande,
              dateMaj: new Date().toISOString(),
              commentaireAdmin: commentaire || null,
            }
          : d,
      )
      if (demandeDetail.value?.id === id) {
        demandeDetail.value = {
          ...demandeDetail.value,
          statut: 'rejete',
          dateMaj: new Date().toISOString(),
          commentaireAdmin: commentaire || null,
        }
      }
    }
    catch (e: unknown) {
      erreur.value = e instanceof Error ? e.message : 'Erreur inconnue'
      throw e
    }
    finally {
      chargement.value = false
    }
  }

  return {
    chargement: readonly(chargement),
    erreur: readonly(erreur),
    demandes: readonly(demandes),
    demandeDetail: readonly(demandeDetail),
    nbEnAttente,
    listerDemandes,
    obtenirDemande,
    approuverDemande,
    rejeterDemande,
  }
}
