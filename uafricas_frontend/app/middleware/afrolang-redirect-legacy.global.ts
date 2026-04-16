// Middleware global : redirige les anciennes routes Afrolang legacy
// (feature 005) vers la nouvelle page d'accueil Afrolang refondue.
// Couvre FR-006 et SC-007 : l'utilisateur qui tape une ancienne URL
// ne tombe jamais sur une 404.

export default defineNuxtRouteMiddleware((to) => {
  if (/^\/afrolang\/salle-privee\//.test(to.path)) {
    return navigateTo('/afrolang', { replace: true })
  }
  if (to.path === '/afrolang/proposer') {
    return navigateTo('/afrolang', { replace: true })
  }
})
