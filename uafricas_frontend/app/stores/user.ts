// Store Pinia pour la gestion de l'utilisateur et de l'authentification
import { defineStore } from 'pinia'

// Interface correspondant au DTO UtilisateurResponse du backend
export interface Utilisateur {
  id: string
  nom: string
  prenom: string
  email: string
  slug: string | null
  photo_url: string | null
  etat: string
  roles: string[]
  created_at: string
}

interface AuthState {
  user: Utilisateur | null
  isAuthenticated: boolean
  accessToken: string | null
  refreshToken: string | null
}

export const useUserStore = defineStore('user', {
  state: (): AuthState => ({
    user: null,
    isAuthenticated: false,
    accessToken: null,
    refreshToken: null,
  }),

  getters: {
    // Nom complet de l'utilisateur
    fullName: (state): string => {
      if (!state.user) return ''
      return `${state.user.prenom} ${state.user.nom}`
    },

    // Nom d'affichage (prenom ou email)
    displayName: (state): string => {
      if (!state.user) return ''
      return state.user.prenom || state.user.email
    },

    // Verifier si l'utilisateur est valide (compte actif)
    isValidated: (state): boolean => {
      return state.user?.etat === 'actif'
    },

    // Verifier si l'utilisateur est admin
    isAdmin: (state): boolean => {
      return state.user?.roles?.includes('admin')
        || state.user?.roles?.includes('super_admin')
        || false
    },
  },

  actions: {
    // Definir l'utilisateur connecte avec ses tokens
    setAuth(user: Utilisateur, accessToken: string, refreshToken: string) {
      this.user = user
      this.isAuthenticated = true
      this.accessToken = accessToken
      this.refreshToken = refreshToken
      // Persister le refresh token dans localStorage pour recuperation de session
      if (import.meta.client) {
        localStorage.setItem('refresh_token', refreshToken)
      }
    },

    // Effacer l'utilisateur (deconnexion)
    clearUser() {
      this.user = null
      this.isAuthenticated = false
      this.accessToken = null
      this.refreshToken = null
      if (import.meta.client) {
        localStorage.removeItem('refresh_token')
      }
    },

    // Charger le refresh token depuis localStorage (au reload)
    loadRefreshToken(): string | null {
      if (import.meta.client) {
        const token = localStorage.getItem('refresh_token')
        if (token) {
          this.refreshToken = token
        }
        return token
      }
      return null
    },
  },
})
