// Données mock pour la page Promotion des Valeurs Africaines

export interface PromotionCard {
  id: number
  title: string
  description: string
  image: string
  altText: string
  link: string
  gradient: string
  buttonText: string
  imageStyle: 'icon' | 'cover'
}

export interface PromotionValeurHero {
  title: string
  backgroundClass: string
}

export interface PromotionValeurPageData {
  hero: PromotionValeurHero
  cards: PromotionCard[]
}

export const promotionValeurPageData: PromotionValeurPageData = {
  hero: {
    title: 'Valeurs africaines et afro-descendantes',
    backgroundClass: 'bg-font-centre-culturel',
  },
  cards: [
    {
      id: 1,
      title: 'ForAfrica',
      description: 'Forum des valeurs africaines et afro-descendantes pour promouvoir notre héritage culturel.',
      image: 'https://upload.wikimedia.org/wikipedia/commons/thumb/6/60/Africa-outline.png/2048px-Africa-outline.png',
      altText: 'Continent africain',
      link: '/evenements',
      gradient: 'from-green-500 to-blue-500',
      buttonText: 'Découvrir',
      imageStyle: 'icon',
    },
    {
      id: 2,
      title: 'Afrocult',
      description: 'Explorez la richesse de la culture africaine et afro-descendante à travers nos programmes culturels.',
      image: 'https://media.istockphoto.com/id/1294238081/fr/vectoriel/verticale-des-jeunes-coiffures-afro-am%C3%A9ricaines-vecteur.jpg?s=612x612&w=0&k=20&c=3xMoHrtWkbpBLYZVDavlj0fZS6vVLMIqWdkurO__q4A=',
      altText: 'Culture Afro-descendante',
      link: '/africain-afro-americain',
      gradient: 'from-purple-500 to-pink-500',
      buttonText: 'Découvrir',
      imageStyle: 'cover',
    },
    {
      id: 3,
      title: 'Afromarket',
      description: "Découvrez l'artisanat, la gastronomie et les produits authentiques du continent africain.",
      image: 'https://img.static-rmg.be/a/view/q75/w962/h503/4255325/1863099-jpg.jpg',
      altText: 'Marché africain',
      link: '/marche-africain',
      gradient: 'from-orange-500 to-red-500',
      buttonText: 'Découvrir',
      imageStyle: 'cover',
    },
  ],
}
