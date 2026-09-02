// Données de la page Promotion des Valeurs Africaines

export interface PromotionCard {
  id: number
  title: string
  description: string
  image: string
  link: string
  buttonText: string
}

export const promotionValeurCards: PromotionCard[] = [
  {
    id: 1,
    title: 'ForAfrica',
    description: 'Forum des valeurs africaines et afro-descendantes pour promouvoir notre héritage culturel.',
    // Était hébergée sur upload.wikimedia.org, et ne chargeait pas : la carte
    // ne montrait que son texte de remplacement, « Continent africain ».
    image: '/images/carte-afrique.jpg',
    link: '/evenements',
    buttonText: 'Découvrir',
  },
  {
    id: 2,
    title: 'Afrocult',
    description: 'Explorez la richesse de la culture africaine et afro-descendante à travers nos programmes culturels.',
    // Était une VIGNETTE D'APERÇU istockphoto (`612x612` avec sa signature
    // dans l'URL). Au-delà de la fragilité du lien, ces aperçus ne sont pas
    // libres d'emploi : les servir sur un site public expose à une réclamation.
    image: '/images/culture_africaine.png',
    link: '/centres',
    buttonText: 'Découvrir',
  },
  {
    id: 3,
    title: 'Afromarket',
    description: "Découvrez l'artisanat, la gastronomie et les produits authentiques du continent africain.",
    // Était hébergée sur img.static-rmg.be, même fragilité.
    image: '/images/marche-afrique.png',
    link: '/marche-africain',
    buttonText: 'Découvrir',
  }]
