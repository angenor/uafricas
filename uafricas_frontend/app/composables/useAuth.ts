// Composable pour l'authentification - connecte au backend API
import { useUserStore } from '~/stores/user'
import type { Utilisateur } from '~/stores/user'

/** Reponse API standardisee */
interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

/** Reponse d'authentification du backend */
interface AuthApiResponse {
  utilisateur: Utilisateur
  access_token: string
  refresh_token: string
}

/** Donnees du formulaire d'inscription */
export interface InscriptionForm {
  nom: string
  prenom: string
  email: string
  mot_de_passe: string
  confirmation_mot_de_passe: string
}

export const useAuth = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()
  const loading = ref(false)
  const error = ref<string | null>(null)

  // ── Inscription ────────────────────────────────────
  const register = async (form: InscriptionForm) => {
    loading.value = true
    error.value = null

    try {
      const reponse = await $fetch<ApiResponse<AuthApiResponse>>(
        `${apiBase}/api/auth/inscription`,
        {
          method: 'POST',
          body: form,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de l\'inscription')
      }

      const { utilisateur, access_token, refresh_token } = reponse.data
      userStore.setAuth(utilisateur, access_token, refresh_token)
      return utilisateur
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur reseau'
      error.value = message
      throw e
    }
    finally {
      loading.value = false
    }
  }

  // ── Connexion ──────────────────────────────────────
  const login = async (email: string, password: string) => {
    loading.value = true
    error.value = null

    try {
      const reponse = await $fetch<ApiResponse<AuthApiResponse>>(
        `${apiBase}/api/auth/connexion`,
        {
          method: 'POST',
          body: { email, mot_de_passe: password },
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur de connexion')
      }

      const { utilisateur, access_token, refresh_token } = reponse.data
      userStore.setAuth(utilisateur, access_token, refresh_token)
      return utilisateur
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Erreur de connexion'
      error.value = message
      throw e
    }
    finally {
      loading.value = false
    }
  }

  // ── Deconnexion ────────────────────────────────────
  const logout = async () => {
    loading.value = true
    error.value = null

    try {
      if (userStore.refreshToken) {
        await $fetch<ApiResponse<null>>(
          `${apiBase}/api/auth/deconnexion`,
          {
            method: 'POST',
            body: { refresh_token: userStore.refreshToken },
          },
        ).catch(() => {}) // Silencieux - l'utilisateur se deconnecte quoi qu'il arrive
      }
      userStore.clearUser()
    }
    finally {
      loading.value = false
    }
  }

  // ── Rafraichir le token ────────────────────────────
  const refreshAccessToken = async (): Promise<boolean> => {
    const refreshToken = userStore.refreshToken || userStore.loadRefreshToken()
    if (!refreshToken) return false

    try {
      const reponse = await $fetch<ApiResponse<AuthApiResponse>>(
        `${apiBase}/api/auth/rafraichir`,
        {
          method: 'POST',
          body: { refresh_token: refreshToken },
        },
      )

      if (!reponse.success || !reponse.data) return false

      const { utilisateur, access_token, refresh_token: newRefreshToken } = reponse.data
      userStore.setAuth(utilisateur, access_token, newRefreshToken)
      return true
    }
    catch {
      userStore.clearUser()
      return false
    }
  }

  // ── Initialiser la session au reload ───────────────
  const initAuth = async () => {
    const refreshToken = userStore.loadRefreshToken()
    if (refreshToken && !userStore.isAuthenticated) {
      await refreshAccessToken()
    }
  }

  // ── Google login (placeholder) ─────────────────────
  const loginWithGoogle = async () => {
    error.value = 'La connexion Google n\'est pas encore disponible'
    throw new Error(error.value)
  }

  // ── Verifier un role ───────────────────────────────
  const hasRole = (role: string): boolean => {
    return userStore.user?.roles?.includes(role) || false
  }

  return {
    // Etat
    loading: readonly(loading),
    error: readonly(error),

    // Getters du store
    isAuthenticated: computed(() => userStore.isAuthenticated),
    user: computed(() => userStore.user),
    fullName: computed(() => userStore.fullName),
    displayName: computed(() => userStore.displayName),
    isValidated: computed(() => userStore.isValidated),
    isAdmin: computed(() => userStore.isAdmin),

    // Actions
    register,
    login,
    loginWithGoogle,
    logout,
    refreshAccessToken,
    initAuth,
    hasRole,
  }
}
