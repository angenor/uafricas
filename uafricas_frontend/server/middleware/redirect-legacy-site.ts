/**
 * Middleware Nitro — redirections permanentes 301 depuis les anciennes URLs
 * `/site/:id` et `/site/:siteId/programmation/:programmationId`.
 *
 * Les routeRules natives de Nuxt ne substituent pas les paramètres `:param`
 * dans le chemin de destination. On utilise donc un middleware serveur pour
 * faire la correspondance et la réécriture avec conservation des segments
 * dynamiques (et la renommage `programmation` → `programmations`).
 *
 * Feature : 001-centres-reorganisation (Décision 1 research.md).
 */
export default defineEventHandler((event) => {
  const rawPath = event.path || ''
  const [pathname, query = ''] = rawPath.split('?')
  const suffix = query ? `?${query}` : ''

  // /site/:siteId/programmation/:programmationId → /centres/:siteId/programmations/:programmationId
  const progMatch = pathname.match(/^\/site\/([^/]+)\/programmation\/([^/]+)\/?$/)
  if (progMatch) {
    return sendRedirect(
      event,
      `/centres/${progMatch[1]}/programmations/${progMatch[2]}${suffix}`,
      301,
    )
  }

  // /site/:id → /centres/:id
  const centreMatch = pathname.match(/^\/site\/([^/]+)\/?$/)
  if (centreMatch) {
    return sendRedirect(event, `/centres/${centreMatch[1]}${suffix}`, 301)
  }
})
