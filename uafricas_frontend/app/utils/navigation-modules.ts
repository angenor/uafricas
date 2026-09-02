/**
 * Arborescence des six univers de la plateforme, la SOURCE UNIQUE.
 *
 * Elle vivait dans `LayoutNavBar`, seule à la connaître ; la navigation
 * latérale de la refonte en avait besoin à son tour. Deux copies auraient
 * divergé au premier module ajouté, et rien n'aurait signalé la dérive :
 * chaque barre aurait simplement montré un menu différent.
 *
 * C'est aussi elle qui tranche les rattachements laissés « à valider » dans
 * `navigation-africans.ts` au lot 1 de la refonte : la maquette n'écrivait
 * nulle part quel univers contient quoi, la barre supérieure le sait depuis
 * toujours.
 */

/** Une application rattachée à un univers. */
export interface SousModuleAfricans {
  label: string
  to: string
  description: string
  icon: string
}

/** Un univers de la plateforme. */
export interface ModuleAfricans {
  id: string
  label: string
  /** Ligne d'accroche affichée sous le nom dans le tiroir mobile. */
  subtitle: string
  description: string
  to: string
  /** Dégradé de la vignette du méga-menu. */
  gradient: string
  image?: string
  items: SousModuleAfricans[]
}

export const MODULES_AFRICANS: ModuleAfricans[] = [
  {
    id: 'africarise',
    label: 'Africarise',
    subtitle: 'Culture & identité',
    description: 'Découvrez et célébrez la richesse culturelle et identitaire de l\'Afrique',
    to: '/africa-culture',
    gradient: 'bg-linear-to-br from-amber-700 to-orange-900',
    image: '/images/danse-afrique.jpg',
    items: [
      { label: 'Afrolang', to: '/afrolang', description: 'Sauvons nos langues', icon: 'fa-solid fa-language' },
      { label: 'Codimoi', to: '/codi-moi', description: 'Préservons nos cultures les meilleures', icon: 'fa-solid fa-book-open' },
      { label: 'Afripulse', to: '/opportunite-afrique', description: 'Promouvons notre Afrique', icon: 'fa-solid fa-briefcase' },
      { label: 'Afroculture', to: '/centres', description: 'Enrichissons-nous ici et ailleurs de notre culture diversifiée', icon: 'fa-solid fa-earth-africa' }]
  },
  {
    id: 'opafrica',
    label: 'Opafrica',
    subtitle: 'Opportunités',
    description: 'Saisir les opportunités et agir concrètement pour le développement du continent',
    to: '/actions',
    gradient: 'bg-linear-to-br from-teal-600 to-cyan-800',
    image: '/images/fiche-opportunite.jpg',
    items: [
      { label: 'Rootstree', to: '/arbre-genealogique', description: 'Tracer son arbre généalogique', icon: 'fa-solid fa-tree' },
      { label: 'Africonnect', to: '/retrouve-amis', description: 'Retrouver une personne perdue de vue', icon: 'fa-solid fa-users' },
      { label: 'Diapertise', to: '/experts', description: 'Mobiliser une expertise de pointe', icon: 'fa-solid fa-user-tie' },
      { label: 'Sabbafrica', to: '/echanges-sabbatiques', description: 'Offrir son expertise en volontariat et bénévolat', icon: 'fa-solid fa-plane' },
      { label: 'Afromarket', to: '/marche-africain', description: 'Place de marché panafricaine', icon: 'fa-solid fa-store' }]
  },
  {
    id: 'novagouv',
    label: 'Novagouv',
    subtitle: 'Gouvernance',
    description: 'Promouvoir une gouvernance transparente et responsable en Afrique',
    to: '/universite/gouvernance',
    gradient: 'bg-linear-to-br from-violet-700 to-purple-900',
    image: '/images/bonne_gouvernance.png',
    items: [
      { label: 'Factcheck', to: '/universite/gouvernance/factcheck', description: 'Vérifier des idées reçues sur l\'Afrique', icon: 'fa-solid fa-scale-balanced' },
      { label: 'Ideaforces', to: '/universite/gouvernance/ideaforces', description: 'Partager des idées et orientations sur les enjeux de développement', icon: 'fa-solid fa-lightbulb' },
      { label: 'BadGoodhabits', to: '/universite/gouvernance/bad-good-habits', description: 'Dénoncer ou féliciter des habitudes', icon: 'fa-solid fa-triangle-exclamation' }]
  },
  {
    id: 'mindshiftlab',
    label: 'Mindshiftlab',
    subtitle: 'Formation & savoir',
    description: 'Se former et développer de nouvelles compétences pour le continent',
    // Pointait sur `/universite`, c'est-à-dire sur Muniversa, l'une de ses
    // propres applications. L'univers et l'application menaient au même endroit
    // et s'allumaient tous deux dans le menu. Mindshiftlab a désormais son
    // carrefour, comme les cinq autres univers.
    to: '/mindshiftlab',
    gradient: 'bg-linear-to-br from-blue-700 to-indigo-900',
    image: '/images/education.png',
    items: [
      { label: 'Africalive', to: '/evenements/liste', description: 'Organiser un événement mettant en valeur l\'Afrique et son développement', icon: 'fa-solid fa-calendar-days' },
      { label: 'Humantech', to: '/bibliotheque/humaine', description: 'Parler à une bibliothèque humaine', icon: 'fa-solid fa-chalkboard-user' },
      { label: 'Librafrica', to: '/bibliotheque/numerique', description: 'Permettre aux Africains et aux écoles de consulter vos publications', icon: 'fa-solid fa-display' },
      { label: 'Muniversa', to: '/universite', description: 'Mindshift University of Africa, éduquer sur les enjeux prioritaires', icon: 'fa-solid fa-graduation-cap' }]
  },
  {
    id: 'africantives',
    label: 'Africantives',
    subtitle: 'Initiatives & projets',
    description: 'Valoriser les initiatives et projets porteurs du développement du continent',
    to: '/africantives',
    gradient: 'bg-linear-to-br from-rose-600 to-pink-800',
    items: []
  },
  {
    id: 'africamood',
    label: 'Africamood',
    subtitle: 'Médias',
    description: 'Suivre l\'actualité médiatique et culturelle du continent africain',
    to: '/medias',
    gradient: 'bg-linear-to-br from-emerald-600 to-green-800',
    image: '/images/tele_baniere.png',
    items: [
      { label: 'Vidafrica', to: '/vidafrica', description: 'Votre musique et des vidéos spéciales sur l\'Afrique à votre portée', icon: 'fa-solid fa-video' },
      { label: 'Télé', to: '/medias/tele', description: 'La télé au service de l\'union et du développement de l\'Afrique', icon: 'fa-solid fa-tv' },
      { label: 'Radio', to: '/medias/radios', description: 'La radio au service de l\'union et du développement de l\'Afrique', icon: 'fa-solid fa-radio' }]
  }]
