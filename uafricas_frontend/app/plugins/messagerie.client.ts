// Plugin client : ouvre et maintient le flux SSE de la messagerie après
// authentification, et dispatche les évènements dans l'état global (useMessagerie).
// EventSource ne supporte pas les en-têtes → le token JWT passe en query (Décision 3).
import { useUserStore } from '~/stores/user'

export default defineNuxtPlugin(() => {
  const userStore = useUserStore()
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const { gererEvenement, listerConversations, obtenirNonLus } = useMessagerie()
  const { gererEvenement: gererEvenementRdv } = useRendezVous()
  const { compteurNonLues } = useNotifications()

  let source: EventSource | null = null
  let reconnexion: ReturnType<typeof setTimeout> | null = null
  let fermeVolontairement = false

  const fermer = () => {
    if (reconnexion) {
      clearTimeout(reconnexion)
      reconnexion = null
    }
    if (source) {
      source.close()
      source = null
    }
  }

  const connecter = () => {
    if (!userStore.accessToken) return
    fermer()
    fermeVolontairement = false

    const url = `${apiBase}/api/messagerie/flux?token=${encodeURIComponent(userStore.accessToken)}`
    source = new EventSource(url)

    // Resynchronisation à chaque (re)connexion : rattrape les messages reçus
    // pendant une éventuelle coupure (T035b).
    source.onopen = () => {
      listerConversations()
      obtenirNonLus()
    }

    source.onmessage = (e) => {
      try {
        const evt = JSON.parse(e.data)
        // Évènements rendez-vous : rafraîchir la liste RDV + le badge de la cloche.
        if (typeof evt?.type === 'string' && evt.type.startsWith('rdv_')) {
          gererEvenementRdv(evt)
          compteurNonLues()
        }
        else {
          gererEvenement(evt)
        }
      }
      catch {
        // Trames commentaires (`: keep-alive`, `: connected`) ignorées.
      }
    }

    source.onerror = () => {
      if (source) {
        source.close()
        source = null
      }
      if (fermeVolontairement || !userStore.isAuthenticated || reconnexion) return
      // L'access token a pu expirer (15 min) : on le rafraîchit puis on rouvre.
      reconnexion = setTimeout(async () => {
        reconnexion = null
        const { refreshAccessToken } = useAuth()
        await refreshAccessToken()
        if (userStore.accessToken && !fermeVolontairement) connecter()
      }, 3000)
    }
  }

  // Ouvre/ferme le flux au gré de l'authentification.
  watch(
    () => userStore.accessToken,
    (token) => {
      if (token) {
        connecter()
      }
      else {
        fermeVolontairement = true
        fermer()
      }
    },
    { immediate: true },
  )
})
