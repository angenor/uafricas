/**
 * Ancienneté en toutes lettres, pour l'horodatage des cartes du fil.
 *
 * Extraite de `CarteAvisFil`, qui la portait en interne : quatre modules
 * l'affichent désormais côte à côte dans le même fil, et quatre copies
 * auraient divergé au premier ajustement de seuil.
 */
export function dateRelativeDepuis(iso: string): string {
  const ms = Date.now() - new Date(iso).getTime()
  const heures = Math.floor(ms / 3_600_000)
  if (heures < 1) return "à l'instant"
  if (heures < 24) return `il y a ${heures} h`
  const jours = Math.floor(heures / 24)
  if (jours < 31) return `il y a ${jours} j`
  return new Date(iso).toLocaleDateString('fr-FR')
}
