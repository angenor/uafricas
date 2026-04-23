import { ref } from 'vue'

export type StatutDemande = 'en_attente' | 'valide' | 'rejete'

export interface DemandeBiblioHumaine {
  id: string
  userId: string
  nom: string
  prenom: string
  photoUrl: string | null
  fonction: string
  pays: string
  biographie: string
  specialites: string[]
  statut: StatutDemande
  dateSubmission: string
  dateMaj: string | null
  commentaireAdmin: string | null
}

const demandesMock: DemandeBiblioHumaine[] = [
  {
    id: 'bh-1',
    userId: 'user-bh-1',
    nom: 'Ndiaye',
    prenom: 'Aissatou',
    photoUrl: 'https://images.unsplash.com/photo-1531123897727-8f129e1688ce?w=200',
    fonction: 'Griotte et Conteuse',
    pays: 'Sénégal',
    biographie: "Gardienne des traditions orales wolof depuis plus de 20 ans, je transmets les légendes et l'histoire de mon peuple à travers le chant et le conte. J'ai formé des centaines de jeunes à l'art de la parole et de la mémoire collective.",
    specialites: ['Traditions orales', 'Culture wolof', 'Musique africaine'],
    statut: 'valide',
    dateSubmission: '2025-11-10T09:00:00Z',
    dateMaj: '2025-11-12T14:30:00Z',
    commentaireAdmin: null,
  },
  {
    id: 'bh-2',
    userId: 'user-bh-2',
    nom: 'Traoré',
    prenom: 'Moussa',
    photoUrl: 'https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=200',
    fonction: 'Historien et Chercheur',
    pays: 'Mali',
    biographie: "Spécialiste de l'histoire précoloniale des empires soudanais, je partage mes recherches sur les royaumes du Mali, du Songhaï et du Ghana. Auteur de 3 ouvrages sur la mémoire africaine.",
    specialites: ['Histoire africaine', 'Archéologie', 'Manuscrits anciens'],
    statut: 'valide',
    dateSubmission: '2025-11-05T11:20:00Z',
    dateMaj: '2025-11-07T09:45:00Z',
    commentaireAdmin: null,
  },
  {
    id: 'bh-3',
    userId: 'user-bh-3',
    nom: 'Mensah',
    prenom: 'Abena',
    photoUrl: 'https://images.unsplash.com/photo-1544005313-94ddf0286df2?w=200',
    fonction: 'Médecin traditionnelle',
    pays: 'Ghana',
    biographie: "Je pratique la médecine traditionnelle Akan et documente les plantes médicinales du Ghana. Mon objectif est de préserver ce savoir ancestral et de le transmettre aux générations futures de façon responsable.",
    specialites: ['Médecine traditionnelle', 'Ethnobotanique', 'Santé communautaire'],
    statut: 'valide',
    dateSubmission: '2025-10-28T15:00:00Z',
    dateMaj: '2025-10-30T10:00:00Z',
    commentaireAdmin: null,
  },
  {
    id: 'bh-4',
    userId: 'user-bh-4',
    nom: 'Diop',
    prenom: 'Fatou',
    photoUrl: 'https://images.unsplash.com/photo-1580489944761-15a19d654956?w=200',
    fonction: 'Économiste et Entrepreneur',
    pays: 'Sénégal',
    biographie: "Après 15 ans dans la finance internationale, je suis revenue au Sénégal pour accompagner les jeunes entrepreneurs africains. J'ai cofondé 3 startups et je partage mes expériences d'échec et de succès.",
    specialites: ['Entrepreneuriat', 'Finance', 'Développement économique'],
    statut: 'valide',
    dateSubmission: '2025-10-20T08:30:00Z',
    dateMaj: '2025-10-22T13:00:00Z',
    commentaireAdmin: null,
  },
  {
    id: 'bh-5',
    userId: 'user-bh-5',
    nom: 'Ouédraogo',
    prenom: 'Karim',
    photoUrl: null,
    fonction: 'Architecte et Urbaniste',
    pays: 'Burkina Faso',
    biographie: "Je conçois des bâtiments inspirés de l'architecture traditionnelle du Sahel. Mon approche mêle le banco, les techniques de ventilation naturelle et les savoirs constructifs ancestraux aux exigences contemporaines.",
    specialites: ['Architecture africaine', 'Urbanisme', 'Patrimoine'],
    statut: 'valide',
    dateSubmission: '2025-10-15T10:00:00Z',
    dateMaj: '2025-10-17T16:20:00Z',
    commentaireAdmin: null,
  },
  {
    id: 'bh-6',
    userId: 'user-bh-6',
    nom: 'Kamara',
    prenom: 'Hawa',
    photoUrl: 'https://images.unsplash.com/photo-1494790108377-be9c29b29330?w=200',
    fonction: 'Tisserande et Artisane',
    pays: 'Guinée',
    biographie: "Héritière d'une lignée de tisserandes, je perpétue les techniques de tissage kente et bogolan. Je forme des femmes de ma communauté à cet art pour leur garantir une indépendance économique.",
    specialites: ['Artisanat africain', 'Tissage', 'Autonomisation des femmes'],
    statut: 'en_attente',
    dateSubmission: '2026-04-18T14:00:00Z',
    dateMaj: null,
    commentaireAdmin: null,
  },
  {
    id: 'bh-7',
    userId: 'user-bh-7',
    nom: 'Soglo',
    prenom: 'Théodore',
    photoUrl: null,
    fonction: 'Ingénieur Agronome',
    pays: 'Bénin',
    biographie: "Spécialisé dans l'agroforesterie tropicale, je travaille depuis 12 ans à restaurer les terres dégradées du Bénin en combinant savoirs paysans et techniques modernes. J'ai contribué à la reforestation de 5 000 hectares.",
    specialites: ['Agriculture durable', 'Agroforesterie', 'Environnement'],
    statut: 'en_attente',
    dateSubmission: '2026-04-19T09:30:00Z',
    dateMaj: null,
    commentaireAdmin: null,
  },
  {
    id: 'bh-8',
    userId: 'user-bh-8',
    nom: 'Amara',
    prenom: 'Binta',
    photoUrl: 'https://images.unsplash.com/photo-1487412720507-e7ab37603c6f?w=200',
    fonction: 'Juriste et Militante',
    pays: 'Sierra Leone',
    biographie: "Avocate spécialisée en droits humains, j'ai défendu des victimes de violences basées sur le genre pendant 10 ans. Je souhaite partager mon expérience judiciaire et les mécanismes de protection juridique en Afrique de l'Ouest.",
    specialites: ['Droits humains', 'Justice de genre', 'Droit international'],
    statut: 'en_attente',
    dateSubmission: '2026-04-20T16:45:00Z',
    dateMaj: null,
    commentaireAdmin: null,
  },
  {
    id: 'bh-9',
    userId: 'user-bh-9',
    nom: 'Konan',
    prenom: 'Éric',
    photoUrl: null,
    fonction: 'Journaliste',
    pays: "Côte d'Ivoire",
    biographie: "Journaliste depuis 5 ans. Je couvre l'actualité politique.",
    specialites: ['Journalisme'],
    statut: 'rejete',
    dateSubmission: '2026-04-10T11:00:00Z',
    dateMaj: '2026-04-12T08:30:00Z',
    commentaireAdmin: "La biographie est trop courte et ne répond pas aux critères minimaux de la plateforme. Veuillez développer votre parcours, vos spécialités et ce que vous souhaitez apporter à la communauté.",
  },
  {
    id: 'bh-10',
    userId: 'user-bh-10',
    nom: 'Fall',
    prenom: 'Mariama',
    photoUrl: 'https://images.unsplash.com/photo-1438761681033-6461ffad8d80?w=200',
    fonction: 'Coach de vie',
    pays: 'Sénégal',
    biographie: "Coach certifiée, j'accompagne des entrepreneurs africains dans leur développement personnel et professionnel. Mon approche intègre les valeurs africaines de communauté et de solidarité.",
    specialites: ['Développement personnel', 'Coaching', 'Leadership'],
    statut: 'rejete',
    dateSubmission: '2026-04-08T14:20:00Z',
    dateMaj: '2026-04-11T10:00:00Z',
    commentaireAdmin: "Le profil ne correspond pas encore aux critères de la Bibliothèque Humaine. La candidature manque d'ancrage dans des savoirs ou expériences uniques. Nous vous encourageons à resoumettre en mettant en avant une spécialité distinctive.",
  },
]

export const demandesBiblioHumaineReactif = ref<DemandeBiblioHumaine[]>([...demandesMock])
