/**
 * Amène un élément sous la barre supérieure du gabarit.
 *
 * `scrollIntoView({ block: 'start' })` ne convient pas : il colle l'élément au
 * bord haut de la fenêtre, où la barre — COLLANTE — le recouvre. On calcule
 * donc la position nous-mêmes.
 */
export function amenerSousLaBarre(el: HTMLElement | null, margeSupplementaire = 16): void {
  if (!el || import.meta.server) return

  // La hauteur vient du jeton, pas d'une constante : elle change avec la
  // maquette. Son UNITÉ est lue aussi — le jeton vaut aujourd'hui `89px`,
  // quand les autres espacements de la refonte sont en `rem` ; supposer l'un
  // des deux donnerait un décalage d'un facteur 16.
  const racine = document.documentElement
  const jeton = getComputedStyle(racine).getPropertyValue('--spacing-af-barre').trim()
  const valeur = Number.parseFloat(jeton) || 0
  const tailleRacine = Number.parseFloat(getComputedStyle(racine).fontSize) || 16
  const barre = (jeton.endsWith('rem') ? valeur * tailleRacine : valeur) + margeSupplementaire

  const doux = !window.matchMedia('(prefers-reduced-motion: reduce)').matches
  window.scrollTo({
    top: el.getBoundingClientRect().top + window.scrollY - barre,
    behavior: doux ? 'smooth' : 'auto',
  })
}
