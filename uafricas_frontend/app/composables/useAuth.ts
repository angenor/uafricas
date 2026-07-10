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

/** Reponse d'inscription (sans tokens) */
interface InscriptionApiResponse {
  message: string
  email: string
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
  const register = async (form: InscriptionForm): Promise<string> => {
    loading.value = true
    error.value = null

    try {
      const reponse = await $fetch<ApiResponse<InscriptionApiResponse>>(
        `${apiBase}/api/auth/inscription`,
        {
          method: 'POST',
          body: form,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de l\'inscription')
      }

      // Retourner l'email pour la page de confirmation
      return reponse.data.email
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

  // ── Verification email ───────────────────────────────
  const verifierEmail = async (token: string) => {
    loading.value = true
    error.value = null

    try {
      const reponse = await $fetch<ApiResponse<AuthApiResponse>>(
        `${apiBase}/api/auth/verifier-email`,
        {
          method: 'POST',
          body: { token },
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la verification')
      }

      const { utilisateur, access_token, refresh_token } = reponse.data
      userStore.setAuth(utilisateur, access_token, refresh_token)
      return utilisateur
    }
    catch (e: any) {
      const message = e?.data?.error || e?.message || 'Token invalide ou expire'
      error.value = message
      throw e
    }
    finally {
      loading.value = false
    }
  }

  // ── Renvoyer email de verification ───────────────────
  const renvoyerVerification = async (email: string): Promise<string> => {
    loading.value = true
    error.value = null

    try {
      const reponse = await $fetch<ApiResponse<{ message: string }>>(
        `${apiBase}/api/auth/renvoyer-verification`,
        {
          method: 'POST',
          body: { email },
        },
      )

      if (!reponse.success) {
        throw new Error(reponse.error || 'Erreur lors du renvoi')
      }

      return reponse.data?.message || 'Email envoye'
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

  // ── Rediriger vers la connexion en conservant la page cible ──
  // Ajoute automatiquement ?redirect=<page courante> pour revenir sur la
  // page d'origine apres authentification. Passer `cible` pour forcer une
  // destination differente de la page courante.
  const redirigerVersConnexion = (cible?: string) => {
    const router = useRouter()
    const chemin = cible || router.currentRoute.value.fullPath
    const query
      = typeof chemin === 'string' && chemin.startsWith('/') && !chemin.startsWith('//')
        ? { redirect: chemin }
        : undefined
    return navigateTo({ path: '/login', query })
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
    verifierEmail,
    renvoyerVerification,
    login,
    loginWithGoogle,
    logout,
    refreshAccessToken,
    initAuth,
    hasRole,
    redirigerVersConnexion,
  }
}
