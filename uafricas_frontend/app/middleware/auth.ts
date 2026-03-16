// Middleware d'authentification
// Redirige vers /login si non connecté
// L'auth est basée sur JWT + localStorage, donc la vérification
// ne peut se faire que côté client (SSR n'a pas accès à localStorage)
export default defineNuxtRouteMiddleware(async (to) => {
  // Côté serveur : on laisse passer (le client vérifiera)
  if (import.meta.server) return

  const userStore = useUserStore()

  // Côté client : si le store n'est pas encore hydraté mais qu'un refresh_token
  // existe dans localStorage, tenter de restaurer la session avant de rediriger
  if (!userStore.isAuthenticated) {
    const refreshToken = userStore.loadRefreshToken()
    if (refreshToken) {
      const { refreshAccessToken } = useAuth()
      await refreshAccessToken()
    }
  }

  // Vérifier l'authentification
  if (!userStore.isAuthenticated) {
    return navigateTo('/login', {
      query: { redirect: to.fullPath },
    })
  }
})
