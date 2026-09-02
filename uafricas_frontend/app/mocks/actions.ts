// Données de la page Actions

export interface ActionCard {
  id: number
  title: string
  description: string
  image: string
  icon: string
  /** `null` = pas encore de destination : la carte reste lisible sans être un lien. */
  link: string | null
}

export const actionCards: ActionCard[] = [
  {
    id: 1,
    title: 'Culture et développement',
    description:
      'Valoriser et promouvoir la richesse culturelle africaine comme moteur de développement.',
    image: '/images/culturel_danse.jpg',
    icon: 'fa-solid fa-masks-theater',
    link: '/africa-culture',
  },
  {
    id: 2,
    title: 'Environnement et climat',
    description:
      "Mettre en œuvre des solutions durables pour la préservation de l'environnement africain.",
    // Portait `/images/diaspora1.jpg` : la photo de la conférence de la
    // diaspora, sans rapport avec le climat.
    image: '/images/dev_durable.jpg',
    icon: 'fa-solid fa-leaf',
    // Aucun module ne traite encore ce thème. `'#'` renvoyait en haut de la
    // page : un lien qui ne mène nulle part vaut moins que pas de lien.
    link: null,
  },
  {
    id: 3,
    title: 'Éducation et formation',
    description:
      "Renforcer les capacités par l'accès à une éducation de qualité pour tous.",
    // Était hébergée sur citinewsroom.com. Une image chez un tiers dépend de
    // son hébergeur ET du navigateur du visiteur : bloquée, elle ne laisse que
    // son texte de remplacement en travers de la carte.
    image: '/images/education.png',
    icon: 'fa-solid fa-graduation-cap',
    link: '/universite',
  },
  {
    id: 4,
    title: 'Intégration et marché africain',
    description:
      "Favoriser la coopération économique et l'unité des marchés à travers le continent.",
    // Était hébergée sur barlamane.com : même raison.
    image: '/images/alliance-afrique.jpg',
    icon: 'fa-solid fa-handshake',
    // Destination d'origine conservée, mais elle détonne : « Promotion des
    // valeurs africaines » est une page culturelle, pas un espace de marché.
    link: '/promotion-valeur',
  },
]
