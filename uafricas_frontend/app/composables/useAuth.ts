// Composable pour l'authentification
import { mockLogin, mockLoginWithGoogle, mockLogout } from '~/mocks/auth'
import { useUserStore } from '~/stores/user'

export const useAuth = () => {
  const userStore = useUserStore()
  const loading = ref(false)
  const error = ref<string | null>(null)

  // Connexion email/password
  const login = async (email: string, password: string) => {
    loading.value = true
    error.value = null

    try {
      const user = await mockLogin(email, password)
      userStore.setUser(user)
      return user
    } catch (e) {
      const errorMessage = e instanceof Error ? e.message : 'Erreur de connexion'
      error.value = errorMessage
      throw e
    } finally {
      loading.value = false
    }
  }

  // Connexion avec Google
  const loginWithGoogle = async () => {
    loading.value = true
    error.value = null

    try {
      const user = await mockLoginWithGoogle()
      userStore.setUser(user)
      return user
    } catch (e) {
      const errorMessage = e instanceof Error ? e.message : 'Erreur de connexion Google'
      error.value = errorMessage
      throw e
    } finally {
      loading.value = false
    }
  }

  // Déconnexion
  const logout = async () => {
    loading.value = true
    error.value = null

    try {
      await mockLogout()
      userStore.clearUser()
    } catch (e) {
      const errorMessage = e instanceof Error ? e.message : 'Erreur de déconnexion'
      error.value = errorMessage
      throw e
    } finally {
      loading.value = false
    }
  }

  // Vérifier si l'utilisateur a un rôle
  const hasRole = (role: string): boolean => {
    return userStore.user?.roles?.includes(role) || false
  }

  return {
    // État
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
    login,
    loginWithGoogle,
    logout,
    hasRole,
  }
}
