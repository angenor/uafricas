// Store Pinia pour la gestion de l'utilisateur
import { defineStore } from 'pinia'
import type { MockUser } from '~/mocks/auth'

interface UserState {
  user: MockUser | null
  isAuthenticated: boolean
}

export const useUserStore = defineStore('user', {
  state: (): UserState => ({
    user: null,
    isAuthenticated: false,
  }),

  getters: {
    // Nom complet de l'utilisateur
    fullName: (state): string => {
      if (!state.user) return ''
      return `${state.user.prenom} ${state.user.nom}`
    },

    // Nom d'affichage (prénom ou email)
    displayName: (state): string => {
      if (!state.user) return ''
      return state.user.prenom || state.user.email
    },

    // Vérifier si l'utilisateur est validé
    isValidated: (state): boolean => {
      return state.user?.statut === 'validé'
    },

    // Vérifier si l'utilisateur est admin
    isAdmin: (state): boolean => {
      return state.user?.roles?.includes('admin') || state.user?.roles?.includes('super_admin') || false
    },

    // Statistiques utilisateur
    userStats: (state) => {
      return state.user?.stats || { annoncesPubliees: 0, contributionsValidees: 0 }
    },
  },

  actions: {
    // Définir l'utilisateur connecté
    setUser(user: MockUser) {
      this.user = user
      this.isAuthenticated = true
    },

    // Effacer l'utilisateur (déconnexion)
    clearUser() {
      this.user = null
      this.isAuthenticated = false
    },

    // Mettre à jour les données utilisateur
    updateUserData(updates: Partial<MockUser>) {
      if (this.user) {
        this.user = { ...this.user, ...updates }
      }
    },
  },
})
