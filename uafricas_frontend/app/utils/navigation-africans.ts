import { MODULES_AFRICANS, type SousModuleAfricans } from './navigation-modules'

/**
 * Navigation latérale de la refonte (maquette « Africans, Design »).
 *
 * La maquette dessine onze entrées plates. Six d'entre elles sont en réalité
 * des UNIVERS contenant des applications, Africarise contient Afrolang,
 * Codimoi, Afripulse, Afroculture, etc. La maquette ne pouvait pas le montrer :
 * un cadre statique ne rend pas un déroulé. C'est la barre supérieure du site
 * qui porte cette arborescence depuis toujours, et c'est d'elle que les
 * sous-entrées sont tirées, jamais recopiées.
 *
 * Les rattachements marqués « à valider » au lot 1 sont donc tranchés : la
 * cible d'un univers est celle que la barre supérieure lui donne.
 *
 * Deux entrées de la maquette ont été retirées. « Explorez » ne menait nulle
 * part et rien ne la définissait. « Messages » faisait doublon : la messagerie
 * est un dock ancré, présent sur toutes les pages et à toute heure, une entrée
 * de menu vers ce qui est déjà à l'écran n'ajoute qu'un clic.
 *
 * « Application African » mène à la page de découverte : c'est elle qui présente
 * la plateforme depuis que le fil d'actualité occupe la racine. Reste incertaine
 * la seule « Communauté ».
 */

export interface EntreeNav {
  /** Libellé exact de la maquette. */
  libelle: string
  /** Icône FontAwesome (déjà enregistrée dans le plugin). */
  icone: string
  /** Route cible. `null` = aucune route existante identifiée. */
  vers: string | null
  /** true = rattachement déduit, à faire confirmer. */
  aValider?: boolean
  /** Applications de l'univers, déroulées en accordéon. Vide = simple lien. */
  sousEntrees?: SousModuleAfricans[]
}

/** Retrouve un univers par son identifiant dans la source partagée. */
const module = (id: string) => MODULES_AFRICANS.find(m => m.id === id)

/**
 * Construit l'entrée d'un univers depuis la barre supérieure : même cible,
 * mêmes applications. `libelle` permet de forcer le libellé de la maquette
 * lorsqu'il diffère de celui de la source partagée.
 */
function universe(id: string, icone: string, libelle?: string): EntreeNav {
  const m = module(id)
  return {
    libelle: libelle ?? m?.label ?? id,
    icone,
    vers: m?.to ?? null,
    sousEntrees: m?.items?.length ? m.items : undefined,
  }
}

export const NAV_AFRICANS: EntreeNav[] = [
  { libelle: "Fil d'actualité", icone: 'fa-solid fa-home', vers: '/' },
  universe('africarise', 'fa-solid fa-book-open'),
  universe('opafrica', 'fa-solid fa-id-card'),
  universe('novagouv', 'fa-solid fa-circle'),
  universe('mindshiftlab', 'fa-solid fa-shield-halved'),
  universe('africantives', 'fa-solid fa-arrows-rotate'),
  universe('africamood', 'fa-solid fa-photo-film'),
  { libelle: 'Communauté', icone: 'fa-solid fa-user', vers: '/profil', aValider: true },
  { libelle: 'Application African', icone: 'fa-solid fa-star', vers: '/decouvrir' }]
