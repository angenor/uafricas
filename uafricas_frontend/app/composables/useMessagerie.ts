// Composable du domaine messagerie : conversations, messages, compteur de
// non-lus, état global alimenté en temps réel par le flux SSE (US3).
import type { MembreLightAPI } from '~/composables/useAmis'
import { useUserStore } from '~/stores/user'

// ──────────────────────────────────────────────────────────────
// Types (alignés sur contracts/api.md)
// ──────────────────────────────────────────────────────────────

export interface ExtraitMessageAPI {
  extrait: string
  created_at: string
}

export interface AnnonceContexteAPI {
  id: string
  titre: string
  slug: string | null
}

export interface ConversationAPI {
  conversation_id: string
  ami: MembreLightAPI
  dernier_message?: ExtraitMessageAPI
  non_lus: number
  verrouillee: boolean
  annonce?: AnnonceContexteAPI
}

export interface MessageAPI {
  id: string
  expediteur_id: string
  contenu: string | null
  supprime: boolean
  created_at: string
  lu_at: string | null
}

interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

// Évènements SSE (discriminés par `type`)
export interface EvenementSse {
  type: 'message' | 'message_supprime' | 'non_lus' | 'demande_recue' | 'demande_acceptee'
  conversation_id?: string
  message?: MessageAPI
  message_id?: string
  non_lus?: number
  demande_id?: string
  demandeur?: MembreLightAPI
  utilisateur?: MembreLightAPI
}

export const useMessagerie = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()

  // État global partagé (SSR-safe)
  const conversations = useState<ConversationAPI[]>('messagerie:conversations', () => [])
  const nonLusTotal = useState<number>('messagerie:nonlus', () => 0)
  const conversationOuverte = useState<string | null>('messagerie:ouverte', () => null)
  const messages = useState<MessageAPI[]>('messagerie:messages', () => [])
  const chargementMessages = useState<boolean>('messagerie:chargement', () => false)
  // Signal d'ouverture programmatique de la fenêtre flottante sur un ami précis
  // (ex. bouton « Envoyer un message » depuis /codi-moi). La fenêtre l'observe.
  const demandeOuverture = useState<MembreLightAPI | null>('messagerie:demande-ouverture', () => null)

  const authHeaders = (): Record<string, string> => {
    if (userStore.accessToken) return { Authorization: `Bearer ${userStore.accessToken}` }
    return {}
  }

  const recalculerNonLus = () => {
    nonLusTotal.value = conversations.value.reduce((acc, c) => acc + c.non_lus, 0)
  }

  // ── Appels API ──────────────────────────────────────────────

  const listerConversations = async (): Promise<void> => {
    try {
      const r = await $fetch<ApiResponse<ConversationAPI[]>>(
        `${apiBase}/api/messagerie/conversations`,
        { headers: authHeaders() },
      )
      if (r.success && r.data) {
        conversations.value = r.data
        recalculerNonLus()
      }
    }
    catch (e) {
      console.error('Erreur listerConversations:', e)
    }
  }

  const obtenirNonLus = async (): Promise<void> => {
    try {
      const r = await $fetch<ApiResponse<{ total: number }>>(
        `${apiBase}/api/messagerie/non-lus`,
        { headers: authHeaders() },
      )
      if (r.success && r.data) nonLusTotal.value = r.data.total
    }
    catch (e) {
      console.error('Erreur obtenirNonLus:', e)
    }
  }

  const listerMessages = async (amiId: string, avant?: string): Promise<MessageAPI[]> => {
    try {
      const params = new URLSearchParams()
      if (avant) params.set('avant', avant)
      const qs = params.toString()
      const r = await $fetch<ApiResponse<MessageAPI[]>>(
        `${apiBase}/api/messagerie/conversations/${amiId}/messages${qs ? `?${qs}` : ''}`,
        { headers: authHeaders() },
      )
      return r.success && r.data ? r.data : []
    }
    catch (e) {
      console.error('Erreur listerMessages:', e)
      return []
    }
  }

  const envoyerMessage = async (amiId: string, contenu: string): Promise<MessageAPI | null> => {
    try {
      const r = await $fetch<ApiResponse<{ message: MessageAPI }>>(
        `${apiBase}/api/messagerie/conversations/${amiId}/messages`,
        {
          method: 'POST',
          headers: { 'Content-Type': 'application/json', ...authHeaders() },
          body: { contenu },
        },
      )
      if (r.success && r.data) {
        // Ajout optimiste (le SSE renverra le même message → dédup par id)
        ajouterMessageSiAbsent(r.data.message)
        return r.data.message
      }
      return null
    }
    catch (e) {
      console.error('Erreur envoyerMessage:', e)
      return null
    }
  }

  const marquerLue = async (amiId: string): Promise<void> => {
    try {
      await $fetch<ApiResponse<unknown>>(
        `${apiBase}/api/messagerie/conversations/${amiId}/lu`,
        { method: 'POST', headers: authHeaders() },
      )
      const conv = conversations.value.find(c => c.ami.id === amiId)
      if (conv) {
        conv.non_lus = 0
        recalculerNonLus()
      }
    }
    catch (e) {
      console.error('Erreur marquerLue:', e)
    }
  }

  const supprimerMessage = async (messageId: string): Promise<void> => {
    try {
      await $fetch<ApiResponse<unknown>>(
        `${apiBase}/api/messagerie/messages/${messageId}`,
        { method: 'DELETE', headers: authHeaders() },
      )
      // Le SSE marquera le message supprimé ; mise à jour optimiste locale
      marquerMessageSupprime(messageId)
    }
    catch (e) {
      console.error('Erreur supprimerMessage:', e)
    }
  }

  // ── Gestion de la conversation ouverte ──────────────────────

  const ouvrirConversation = async (amiId: string): Promise<void> => {
    conversationOuverte.value = amiId
    chargementMessages.value = true
    messages.value = await listerMessages(amiId)
    chargementMessages.value = false
    await marquerLue(amiId)
  }

  const fermerConversation = (): void => {
    conversationOuverte.value = null
    messages.value = []
  }

  /** Demande l'ouverture de la fenêtre flottante sur la conversation d'un ami. */
  const demanderOuverture = (ami: MembreLightAPI): void => {
    demandeOuverture.value = ami
  }

  // ── Helpers internes ────────────────────────────────────────

  const amiDeConversation = (conversationId: string): string | undefined =>
    conversations.value.find(c => c.conversation_id === conversationId)?.ami.id

  const ajouterMessageSiAbsent = (msg: MessageAPI) => {
    if (!messages.value.some(m => m.id === msg.id)) {
      messages.value = [...messages.value, msg]
    }
  }

  const marquerMessageSupprime = (messageId: string) => {
    messages.value = messages.value.map(m =>
      m.id === messageId ? { ...m, contenu: null, supprime: true } : m,
    )
  }

  // ── Dispatch des évènements SSE ──────────────────────────────

  const gererEvenement = async (evt: EvenementSse): Promise<void> => {
    const moi = userStore.user?.id
    switch (evt.type) {
      case 'message': {
        if (!evt.conversation_id || !evt.message) break
        const amiId = amiDeConversation(evt.conversation_id)
        const estOuverte = amiId && conversationOuverte.value === amiId
        const conv = conversations.value.find(c => c.conversation_id === evt.conversation_id)

        if (!conv) {
          // Conversation inconnue (nouvel ami) → resynchroniser la liste
          await listerConversations()
        }
        else {
          conv.dernier_message = {
            extrait: evt.message.contenu ?? '',
            created_at: evt.message.created_at,
          }
          // Remonter la conversation en tête
          conversations.value = [
            conv,
            ...conversations.value.filter(c => c.conversation_id !== evt.conversation_id),
          ]
        }

        if (estOuverte) {
          ajouterMessageSiAbsent(evt.message)
          if (amiId) await marquerLue(amiId)
        }
        else if (conv && evt.message.expediteur_id !== moi) {
          conv.non_lus += 1
        }
        recalculerNonLus()
        break
      }
      case 'message_supprime': {
        if (evt.message_id) marquerMessageSupprime(evt.message_id)
        break
      }
      case 'non_lus': {
        const conv = conversations.value.find(c => c.conversation_id === evt.conversation_id)
        if (conv && typeof evt.non_lus === 'number') {
          conv.non_lus = evt.non_lus
          recalculerNonLus()
        }
        break
      }
      case 'demande_acceptee': {
        // Un nouvel ami → recharger la liste pour faire apparaître sa conversation
        await listerConversations()
        break
      }
      case 'demande_recue':
        // Pas d'impact messagerie (géré par les notifications relationnelles)
        break
    }
  }

  return {
    conversations,
    nonLusTotal,
    conversationOuverte,
    messages,
    chargementMessages,
    demandeOuverture,
    listerConversations,
    obtenirNonLus,
    listerMessages,
    envoyerMessage,
    marquerLue,
    supprimerMessage,
    ouvrirConversation,
    fermerConversation,
    demanderOuverture,
    gererEvenement,
  }
}
