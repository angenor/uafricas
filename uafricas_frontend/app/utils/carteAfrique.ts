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
