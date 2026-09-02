// Assainissement HTML avant tout rendu `v-html` (audit #29, XSS stockée).
// isomorphic-dompurify fonctionne au SSR (jsdom) comme au client.
// Auto-importé par Nuxt (app/utils/).
import DOMPurify from 'isomorphic-dompurify'

export function sanitiserHtml(html: string | null | undefined): string {
  if (!html) return ''
  return DOMPurify.sanitize(html)
}

/**
 * Réduit du HTML à son texte, pour les cartes du fil.
 *
 * `AfricansCartePublication` rend son `texte` en `{{ }}`, donc ÉCHAPPÉ : une
 * description produite par l'éditeur riche (Sabbafrica) s'y affichait avec ses
 * balises en toutes lettres. Le fil n'est pas l'endroit d'un rendu formaté :
 * treize sources y voisinent et doivent partager la même typographie.
 *
 * Le passage par un FRAGMENT DOM n'est pas un détour. Deux essais plus simples
 * échouent, et le test le montre :
 *  - `ALLOWED_TAGS: []` renvoie une chaîne ré-échappée : « &amp; » restait
 *    littéral à l'écran ;
 *  - retirer les balises laisse les blocs collés : « collectivités.Vous
 *    accompagnerez ». D'où le séparateur inséré après chaque bloc.
 */
export function texteBrut(html: string | null | undefined, longueurMax = 280): string {
  if (!html) return ''

  const fragment = DOMPurify.sanitize(html, { RETURN_DOM_FRAGMENT: true }) as DocumentFragment
  const boite = fragment.ownerDocument.createElement('div')
  boite.appendChild(fragment)
  for (const bloc of boite.querySelectorAll('p, div, li, br, h1, h2, h3, h4, h5, h6, tr')) {
    bloc.after(boite.ownerDocument.createTextNode(' '))
  }

  const texte = (boite.textContent ?? '').replace(/\s+/g, ' ').trim()
  if (texte.length <= longueurMax) return texte

  // Coupe au dernier mot entier : une phrase tranchée en plein milieu d'un mot
  // se lit comme une erreur d'affichage.
  const coupe = texte.slice(0, longueurMax)
  const espace = coupe.lastIndexOf(' ')
  return `${espace > 0 ? coupe.slice(0, espace) : coupe}…`
}
