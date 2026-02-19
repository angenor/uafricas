// Middleware de navigation admin
// Redirige vers /login si non connecte ou non admin
// L'auth est basee sur JWT + localStorage, donc la verification
// ne peut se faire que cote client (SSR n'a pas acces a localStorage)
export default defineNuxtRouteMiddleware(async (to) => {
  // Cote serveur : on laisse passer (le client verifiera)
  if (import.meta.server) return

  const userStore = useUserStore()

  // Cote client : si le store n'est pas encore hydrate mais qu'un refresh_token
  // existe dans localStorage, tenter de restaurer la session avant de rediriger
  if (!userStore.isAuthenticated) {
    const refreshToken = userStore.loadRefreshToken()
    if (refreshToken) {
      const { refreshAccessToken } = useAuth()
      await refreshAccessToken()
    }
  }

  // Verifier l'authentification
  if (!userStore.isAuthenticated) {
    return navigateTo('/login', {
      query: { redirect: to.fullPath },
    })
  }

  // Verifier le role admin
  if (!userStore.isAdmin) {
    return navigateTo('/')
  }
})
