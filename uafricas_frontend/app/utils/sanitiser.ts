// Assainissement HTML avant tout rendu `v-html` (audit #29 — XSS stockée).
// isomorphic-dompurify fonctionne au SSR (jsdom) comme au client.
// Auto-importé par Nuxt (app/utils/).
import DOMPurify from 'isomorphic-dompurify'

export function sanitiserHtml(html: string | null | undefined): string {
  if (!html) return ''
  return DOMPurify.sanitize(html)
}
