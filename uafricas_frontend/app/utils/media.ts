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
 *
 * Les fichiers téléversés sont servis sous `/uploads/`, mais les composables
 * les absolutisent en `https://api…/uploads/…` pour qu'ils soient atteignables
 * depuis le navigateur. Tester `startsWith('/uploads/')` déclarait donc tiers
 * tout fichier réellement hébergé, et le lecteur refusait de le jouer : le
 * test porte sur le CHEMIN, quelle que soit l'origine.
 */
export const estMediaExterne = (url: string | null | undefined): boolean => {
  if (!url) return false
  const u = url.trim()
  return !cheminEstUpload(u)
}

/** `/uploads/x`, `http://host/uploads/x` et `//host/uploads/x` sont hébergés. */
const cheminEstUpload = (url: string): boolean => {
  if (url.startsWith('/uploads/')) return true
  // `URL` échoue sur les chemins relatifs : la base ne sert qu'à les parser,
  // elle n'apparaît jamais dans le résultat.
  try {
    return new URL(url, 'http://local.invalid').pathname.startsWith('/uploads/')
  }
  catch {
    return true // URL illisible : ne pas la déclarer tierce à tort.
  }
}

/** Miroir côté client de `source_media` calculé par le backend. */
export type SourceMedia = 'hebergee' | 'externe' | 'aucune'

/**
 * `sourceMediaDeclaree` est la valeur renvoyée par le backend, qui raisonne sur
 * la donnée brute et fait donc autorité. On ne retombe sur l'inspection de
 * l'URL que lorsqu'elle est absente.
 */
export const sourceMedia = (
  url: string | null | undefined,
  sourceMediaDeclaree?: SourceMedia | string | null,
): SourceMedia => {
  if (!url) return 'aucune'
  if (sourceMediaDeclaree === 'hebergee' || sourceMediaDeclaree === 'externe') {
    return sourceMediaDeclaree
  }
  return estMediaExterne(url) ? 'externe' : 'hebergee'
}

/**
 * Le média est-il jouable par l'un des lecteurs dont nous disposons ?
 * Un lien tiers non reconnu (ni fichier, ni YouTube) ne l'est pas : la page doit
 * alors proposer d'ouvrir la source plutôt qu'un lecteur qui resterait vide.
 */
export const estMediaJouable = (
  url: string | null | undefined,
  sourceMediaDeclaree?: SourceMedia | string | null,
): boolean => {
  if (!url) return false
  if (sourceMedia(url, sourceMediaDeclaree) === 'hebergee') return true
  return youtubeEmbedUrl(url) !== null
}

/**
 * Résout un chemin d'upload en URL absolue.
 *
 * Le serveur renvoie des chemins RELATIFS (`/uploads/photos-profil/…`), servis
 * par actix-files sur le port du backend. En production, nginx place les deux
 * derrière la même origine et le chemin relatif suffit : c'est pourquoi
 * l'oubli du préfixe ne s'y voit pas. En développement, le front est sur 3000
 * et le backend sur 8080 : le navigateur demande alors une image qui n'existe
 * pas de son côté, et l'affiche cassée.
 *
 * Renvoie `null` pour une entrée vide, et laisse passer les URL déjà absolues.
 */
export const urlMedia = (chemin: string | null | undefined): string | null => {
  if (!chemin) return null
  if (chemin.startsWith('http://') || chemin.startsWith('https://')) return chemin
  const base = useRuntimeConfig().public.apiBaseUrl as string
  return `${base}${chemin}`
}
