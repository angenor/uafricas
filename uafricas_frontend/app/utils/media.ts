/**
 * Utilitaires de routage des médias.
 *
 * Un contenu radio ou télé porte soit un fichier servi par la plateforme
 * (`/uploads/…`), lisible par une balise `<video>` / `<audio>` native, soit un
 * lien vers un service tiers, qui exige son propre lecteur. Confondre les deux
 * ne produit aucune erreur visible : la balise reste simplement noire et muette.
 */

/**
 * Convertit une URL YouTube (watch, youtu.be, shorts, live, embed) en URL d'iframe embed.
 * Renvoie null si l'URL n'est pas reconnue comme une vidéo YouTube.
 */
export const youtubeEmbedUrl = (url: string | null | undefined): string | null => {
  if (!url) return null
  const u = url.trim()
  const patterns = [
    /(?:youtube\.com\/watch\?(?:.*&)?v=)([\w-]{11})/,
    /(?:youtu\.be\/)([\w-]{11})/,
    /(?:youtube\.com\/embed\/)([\w-]{11})/,
    /(?:youtube\.com\/live\/)([\w-]{11})/,
    /(?:youtube\.com\/shorts\/)([\w-]{11})/,
  ]
  for (const p of patterns) {
    const m = u.match(p)
    if (m) return `https://www.youtube.com/embed/${m[1]}`
  }
  return null
}

/**
 * L'URL désigne-t-elle un média hébergé ailleurs que sur la plateforme ?
 * Les fichiers téléversés sont servis sous `/uploads/` ; tout le reste est tiers.
 */
export const estMediaExterne = (url: string | null | undefined): boolean => {
  if (!url) return false
  const u = url.trim()
  return !u.startsWith('/uploads/')
}

/** Miroir côté client de `source_media` calculé par le backend. */
export type SourceMedia = 'hebergee' | 'externe' | 'aucune'

export const sourceMedia = (url: string | null | undefined): SourceMedia => {
  if (!url) return 'aucune'
  return estMediaExterne(url) ? 'externe' : 'hebergee'
}

/**
 * Le média est-il jouable par l'un des lecteurs dont nous disposons ?
 * Un lien tiers non reconnu (ni fichier, ni YouTube) ne l'est pas : la page doit
 * alors proposer d'ouvrir la source plutôt qu'un lecteur qui resterait vide.
 */
export const estMediaJouable = (url: string | null | undefined): boolean => {
  if (!url) return false
  return !estMediaExterne(url) || youtubeEmbedUrl(url) !== null
}
