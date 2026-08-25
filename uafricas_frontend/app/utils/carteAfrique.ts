/**
 * Données partagées pour le rendu d'une carte SVG du continent africain
 * (basée sur `@svg-maps/world`). Utilisé par les vues « carte » d'Afripulse
 * et de Retrouv'Amis.
 */

/** Noms français des pays africains, indexés par code ISO2 (minuscule). */
export const NOMS_PAYS_FR: Record<string, string> = {
  dz: 'Algerie', ao: 'Angola', bj: 'Benin', bw: 'Botswana', bf: 'Burkina Faso',
  bi: 'Burundi', cv: 'Cap-Vert', cm: 'Cameroun', cf: 'Centrafrique',
  td: 'Tchad', km: 'Comores', cg: 'Congo', cd: 'RD Congo', ci: "Cote d'Ivoire",
  dj: 'Djibouti', eg: 'Egypte', gq: 'Guinee equatoriale', er: 'Erythree',
  sz: 'Eswatini', et: 'Ethiopie', ga: 'Gabon', gm: 'Gambie', gh: 'Ghana',
  gn: 'Guinee', gw: 'Guinee-Bissau', ke: 'Kenya', ls: 'Lesotho', lr: 'Liberia',
  ly: 'Libye', mg: 'Madagascar', mw: 'Malawi', ml: 'Mali', mr: 'Mauritanie',
  mu: 'Maurice', ma: 'Maroc', mz: 'Mozambique', na: 'Namibie', ne: 'Niger',
  ng: 'Nigeria', rw: 'Rwanda', st: 'Sao Tome-et-Principe', sn: 'Senegal',
  sc: 'Seychelles', sl: 'Sierra Leone', so: 'Somalie', za: 'Afrique du Sud',
  ss: 'Soudan du Sud', sd: 'Soudan', tz: 'Tanzanie', tg: 'Togo', tn: 'Tunisie',
  ug: 'Ouganda', zm: 'Zambie', zw: 'Zimbabwe', eh: 'Sahara occidental',
}

/** ViewBox calé au plus près du continent (bbox réelle ~404,350 233×261 + marge mini). */
export const AFRICA_VIEWBOX = '401 347 239 267'

/**
 * Petites îles trop petites pour être visibles : facteur d'agrandissement
 * appliqué autour de leur centroïde, indexé par code ISO2.
 */
export const PETITES_ILES: Record<string, number> = {
  cv: 5, // Cap-Vert
  st: 6, // Sao Tome-et-Principe
  km: 5, // Comores
  mu: 6, // Maurice
  sc: 7, // Seychelles
}

/**
 * Normalise un nom de pays pour comparaison tolérante (sans accents,
 * minuscule, sans ponctuation ni espaces superflus).
 */
export const normaliserNomPays = (nom: string): string =>
  nom
    .normalize('NFD')
    .replace(/[̀-ͯ]/g, '')
    .toLowerCase()
    .replace(/[^a-z0-9]/g, '')

/**
 * Échelle de chaleur des avis de recherche (Retrouv'Amis / Africonnect).
 *
 * Partagée entre la carte et la légende du rail : deux tables séparées
 * divergent, et une légende qui ne décrit plus les couleurs affichées est
 * pire que pas de légende. Ordonnée du plus dense au plus vide : c'est ce
 * qui permet à `couleurChaleurAvis` de prendre le PREMIER palier atteint.
 */
export const PALIERS_CHALEUR = [
  { min: 10, couleur: '#b45309', libelle: '10 avis et plus' },
  { min: 6, couleur: '#d97706', libelle: '6 à 9 avis' },
  { min: 3, couleur: '#f59e0b', libelle: '3 à 5 avis' },
  { min: 1, couleur: '#fbbf24', libelle: '1 à 2 avis' },
  { min: 0, couleur: '#e5e7eb', libelle: 'Aucun avis' }]

/** Couleur d'un territoire selon son nombre d'avis. */
export const couleurChaleurAvis = (compte: number): string =>
  PALIERS_CHALEUR.find(p => compte >= p.min)?.couleur ?? '#e5e7eb'

/**
 * Boîtes englobantes des sous-tracés d'un `d` SVG.
 *
 * Calcul APPROCHÉ : les points de contrôle des courbes sont pris pour des
 * points du tracé, ce qui élargit très légèrement la boîte. Sur des côtes
 * découpées en segments courts l'écart est de l'ordre du pour cent, et le
 * cadrage l'absorbe dans sa marge. En échange, la fonction est pure, pas de
 * DOM, donc un cadrage juste dès le rendu serveur.
 */
function boitesSousTraces(d: string): Boite[] {
  const jetons = d.match(/[a-zA-Z]|-?[\d.]+(?:e-?\d+)?/g) ?? []
  const boites: Boite[] = []
  let x = 0, y = 0, departX = 0, departY = 0, commande = '', i = 0
  let courante: Boite | null = null

  const nombre = () => Number.parseFloat(jetons[i++] as string)
  const marquer = () => {
    if (!courante) return
    courante.minX = Math.min(courante.minX, x)
    courante.maxX = Math.max(courante.maxX, x)
    courante.minY = Math.min(courante.minY, y)
    courante.maxY = Math.max(courante.maxY, y)
  }

  while (i < jetons.length) {
    // `^…$` : le motif est ANCRÉ, et ce n'est pas cosmétique. `test` sans
    // ancre répond vrai dès qu'une lettre apparaît QUELQUE PART : or un
    // coordonnée en notation scientifique en contient une : `-10e-4`, présent
    // dans le tracé du Sénégal, était donc pris pour une commande. Le parseur
    // basculait sur une commande inconnue, tombait dans `else i++` et cessait
    // d'accumuler : le Sénégal mesurait 1,1 × 3,9 au lieu de 17,3 × 12,5, et
    // sa carte débordait de dix fois son cadre.
    if (/^[a-zA-Z]$/.test(jetons[i] as string)) commande = jetons[i++] as string
    const relatif = commande === commande.toLowerCase()
    const c = commande.toLowerCase()

    if (c === 'm') {
      const a = nombre(); const b = nombre()
      x = relatif ? x + a : a
      y = relatif ? y + b : b
      departX = x; departY = y
      courante = { minX: x, maxX: x, minY: y, maxY: y }
      boites.push(courante)
      // Après un `moveto`, les paires suivantes sont des `lineto` implicites.
      commande = relatif ? 'l' : 'L'
    }
    else if (c === 'l' || c === 't') {
      const a = nombre(); const b = nombre()
      x = relatif ? x + a : a
      y = relatif ? y + b : b
      marquer()
    }
    else if (c === 'h') { const a = nombre(); x = relatif ? x + a : a; marquer() }
    else if (c === 'v') { const a = nombre(); y = relatif ? y + a : a; marquer() }
    else if (c === 'c' || c === 's' || c === 'q' || c === 'a') {
      // On saute les points de contrôle (4 pour C, 2 pour S/Q, 5 pour A) et on
      // ne retient que le point d'arrivée.
      const aSauter = c === 'c' ? 4 : c === 'a' ? 5 : 2
      for (let k = 0; k < aSauter; k++) nombre()
      const a = nombre(); const b = nombre()
      x = relatif ? x + a : a
      y = relatif ? y + b : b
      marquer()
    }
    else if (c === 'z') { x = departX; y = departY }
    else i++
  }

  return boites
}

/** Boîte englobante d'un ensemble de boîtes. */
const reunir = (boites: Boite[]): Boite => boites.reduce((a, b) => ({
  minX: Math.min(a.minX, b.minX),
  maxX: Math.max(a.maxX, b.maxX),
  minY: Math.min(a.minY, b.minY),
  maxY: Math.max(a.maxY, b.maxY),
}))

const aire = (b: Boite) => (b.maxX - b.minX) * (b.maxY - b.minY)

export interface Boite { minX: number, maxX: number, minY: number, maxY: number }

/**
 * `viewBox` cadré sur un territoire, îlots lointains exclus.
 *
 * Trois pays traînent dans leur tracé une possession minuscule à des
 * centaines de kilomètres des côtes : les îles du Prince-Édouard pour
 * l'Afrique du Sud, et des cas analogues au Nigeria et en Tanzanie. Cadrer sur
 * la boîte TOTALE écrase alors le pays dans un coin : l'Afrique du Sud
 * n'occupait que 36 % du cadre, le Nigeria 23 %.
 *
 * Un sous-tracé n'est écarté que s'il réunit DEUX conditions : peser moins de
 * 1 % du sous-tracé principal, ET tomber hors de sa boîte élargie de 20 %.
 * Les deux ensemble, sinon les archipels disparaîtraient, au Cap-Vert aucune
 * des huit îles ne domine, elles se gardent toutes.
 *
 * @param d      chemin SVG du territoire
 * @param marge  marge autour du cadre, en proportion du plus grand côté
 */
export function cadrageTerritoire(d: string, marge = 0.06): string | null {
  const boites = boitesSousTraces(d)
  if (!boites.length) return null

  let retenues = boites
  if (boites.length > 1) {
    const principal = boites.reduce((a, b) => (aire(b) > aire(a) ? b : a))
    const dx = (principal.maxX - principal.minX) * 0.2
    const dy = (principal.maxY - principal.minY) * 0.2
    retenues = boites.filter(b => b === principal
      || aire(b) >= aire(principal) * 0.01
      || (b.maxX >= principal.minX - dx && b.minX <= principal.maxX + dx
        && b.maxY >= principal.minY - dy && b.minY <= principal.maxY + dy))
  }

  const boite = reunir(retenues)
  const largeur = boite.maxX - boite.minX
  const hauteur = boite.maxY - boite.minY
  if (!largeur || !hauteur) return null

  const m = Math.max(largeur, hauteur) * marge
  return [boite.minX - m, boite.minY - m, largeur + m * 2, hauteur + m * 2].join(' ')
}
