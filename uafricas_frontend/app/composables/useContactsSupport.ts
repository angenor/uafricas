/**
 * Coordonnées publiques d'un support média (chaîne TV, station radio), 09p.
 *
 * Le bloc est servi tel quel par l'API (`contacts`), absent du JSON quand le
 * support n'en publie aucune : côté front, `contacts == null` suffit à décider
 * de ne rien afficher, sans inspecter cinq champs.
 */
export interface ContactsSupport {
  email?: string | null
  telephone?: string | null
  whatsapp?: string | null
  site_web?: string | null
  adresse?: string | null
}

/**
 * `wa.me` n'accepte qu'une suite de chiffres commençant par l'indicatif pays :
 * espaces, tirets, parenthèses et le « + » font échouer le lien silencieusement
 * la page WhatsApp s'ouvre alors sur « numéro invalide ».
 *
 * Le « 00 » initial est retiré au même titre que le « + » : c'est l'autre
 * écriture de l'indicatif international, et la laisser produirait un numéro
 * inexistant (`0022507…` au lieu de `22507…`).
 */
export const lienWhatsapp = (numero?: string | null): string | null => {
  const chiffres = (numero ?? '').replace(/\D/g, '').replace(/^00/, '')
  return chiffres ? `https://wa.me/${chiffres}` : null
}

/** `tel:` tolère le « + », mais pas les espaces. */
export const lienTelephone = (numero?: string | null): string | null => {
  const nettoye = (numero ?? '').replace(/[^\d+]/g, '')
  return nettoye ? `tel:${nettoye}` : null
}

export const lienEmail = (email?: string | null): string | null => {
  const valeur = (email ?? '').trim()
  return valeur ? `mailto:${valeur}` : null
}

/**
 * Le serveur préfixe déjà `https://` à l'écriture ; ce repli couvre les lignes
 * saisies avant cette normalisation, qu'un `href` relatif enverrait sur une
 * page introuvable du site.
 */
export const lienSiteWeb = (url?: string | null): string | null => {
  const valeur = (url ?? '').trim()
  if (!valeur) return null
  return /^https?:\/\//i.test(valeur) ? valeur : `https://${valeur}`
}

/** Étiquette lisible d'un site web : sans schéma ni barre oblique finale. */
export const libelleSiteWeb = (url?: string | null): string =>
  (url ?? '').trim().replace(/^https?:\/\//i, '').replace(/\/$/, '')

/** `true` dès qu'une seule coordonnée est renseignée. */
export const aDesContacts = (contacts?: ContactsSupport | null): boolean =>
  !!contacts
  && Object.values(contacts).some(v => typeof v === 'string' && v.trim() !== '')
