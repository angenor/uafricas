// Plugin client-only pour restaurer la session utilisateur au chargement de la page
export default defineNuxtPlugin(async () => {
  const { initAuth } = useAuth()
  await initAuth()
})
