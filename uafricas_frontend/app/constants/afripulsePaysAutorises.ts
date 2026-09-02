/**
 * Périmètre Afripulse : liste figée des codes ISO2 des pays africains autorisés
 * à posséder une fiche pays sur `/opportunite-afrique`.
 *
 * Source unique partagée avec le backend
 * (`uafricas_backend/src/constants/afripulse_pays_autorises.rs`).
 * Toute modification doit être propagée dans les deux fichiers (cf. T077).
 *
 * Triés alphabétiquement, minuscules. Inclut `eh` (Sahara occidental).
 */
export const PAYS_AFRICAINS_ISO2: readonly string[] = [
  'ao', 'bf', 'bi', 'bj', 'bw', 'cd', 'cf', 'cg', 'ci', 'cm',
  'cv', 'dj', 'dz', 'eg', 'eh', 'er', 'et', 'ga', 'gh', 'gm',
  'gn', 'gq', 'gw', 'ke', 'km', 'lr', 'ls', 'ly', 'ma', 'mg',
  'ml', 'mr', 'mu', 'mw', 'mz', 'na', 'ne', 'ng', 'rw', 'sc',
  'sd', 'sl', 'sn', 'so', 'ss', 'st', 'sz', 'td', 'tg', 'tn',
  'tz', 'ug', 'za', 'zm', 'zw',
] as const

const ENSEMBLE_PAYS_AFRICAINS = new Set<string>(PAYS_AFRICAINS_ISO2)

/**
 * Indique si un code ISO2 (insensible à la casse) appartient au périmètre
 * Afripulse. Retourne `false` pour toute chaîne vide, malformée ou hors périmètre.
 */
export function estPaysAfricain(code: string): boolean {
  if (!code) return false
  return ENSEMBLE_PAYS_AFRICAINS.has(code.trim().toLowerCase())
}
