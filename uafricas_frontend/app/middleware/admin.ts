// Middleware de navigation admin
// Redirige vers /login si non connecte ou non admin
export default defineNuxtRouteMiddleware((to) => {
  const userStore = useUserStore()

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
