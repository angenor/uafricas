// Données mock pour les bibliothèques

export interface DocumentNumerique {
  id: string
  titre: string
  description: string
  couverture_url: string
  doc_url: string
  acces: 'Lecture' | 'Téléchargeable'
  date_heure_publication: string
  type: string
  auteur: {
    biblio: string
  }
  user: {
    nom: string
    prenom: string
  }
}

export const documentsNumeriques: DocumentNumerique[] = [
  {
    id: '1',
    titre: 'Histoire de l\'Afrique précoloniale',
    description: 'Un ouvrage complet sur l\'histoire des royaumes africains avant la colonisation.',
    couverture_url: 'https://images.unsplash.com/photo-1544716278-ca5e3f4abd8c?w=400',
    doc_url: '/documents/sample.pdf',
    acces: 'Lecture',
    date_heure_publication: '2024-01-15',
    type: 'Livre',
    auteur: { biblio: 'Historien spécialisé dans l\'Afrique ancienne' },
    user: { nom: 'Diallo', prenom: 'Amadou' },
  },
  {
    id: '2',
    titre: 'Traditions orales du Sahel',
    description: 'Recueil de contes et légendes transmis de génération en génération.',
    couverture_url: 'https://images.unsplash.com/photo-1481627834876-b7833e8f5570?w=400',
    doc_url: '/documents/sample.pdf',
    acces: 'Téléchargeable',
    date_heure_publication: '2024-03-20',
    type: 'Livre',
    auteur: { biblio: 'Ethnologue et collecteur de traditions orales' },
    user: { nom: 'Koné', prenom: 'Fatou' },
  },
  {
    id: '3',
    titre: 'L\'économie africaine moderne',
    description: 'Analyse des défis et opportunités économiques du continent africain.',
    couverture_url: 'https://images.unsplash.com/photo-1553729459-efe14ef6055d?w=400',
    doc_url: '/documents/sample.pdf',
    acces: 'Lecture',
    date_heure_publication: '2024-06-10',
    type: 'Rapport',
    auteur: { biblio: 'Économiste et chercheur à l\'université de Dakar' },
    user: { nom: 'Ndiaye', prenom: 'Moussa' },
  },
  {
    id: '4',
    titre: 'Art contemporain africain',
    description: 'Exploration des mouvements artistiques africains du 21ème siècle.',
    couverture_url: 'https://images.unsplash.com/photo-1578301978693-85fa9c0320b9?w=400',
    doc_url: '/documents/sample.pdf',
    acces: 'Téléchargeable',
    date_heure_publication: '2024-02-28',
    type: 'Article de revue',
    auteur: { biblio: 'Critique d\'art et curateur' },
    user: { nom: 'Ouattara', prenom: 'Awa' },
  },
]

export interface BiblioHumaine {
  id: string
  user_id: string
  prenom: string
  nom: string
  photo_url: string
  fonction: string
  pays: string
  specialite: string
  biographie: string
  organisation?: string
}

export const biblioHumaines: BiblioHumaine[] = [
  {
    id: '1',
    user_id: 'user-1',
    prenom: 'Mamadou',
    nom: 'Traoré',
    photo_url: 'https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=200',
    fonction: 'Griot et conteur',
    pays: 'Mali',
    specialite: 'Histoire',
    biographie: 'Héritier d\'une longue lignée de griots, je perpétue la tradition orale depuis plus de 40 ans. Mon répertoire comprend des récits de l\'empire du Mali et des épopées de Soundiata Keita.',
  },
  {
    id: '2',
    user_id: 'user-2',
    prenom: 'Aminata',
    nom: 'Diop',
    photo_url: 'https://images.unsplash.com/photo-1531746020798-e6953c6e8e04?w=200',
    fonction: 'Anthropologue',
    pays: 'Sénégal',
    specialite: 'Culture',
    biographie: 'Docteure en anthropologie culturelle, spécialisée dans les rites initiatiques et les pratiques spirituelles de l\'Afrique de l\'Ouest.',
  },
  {
    id: '3',
    user_id: 'user-3',
    prenom: 'Kwame',
    nom: 'Asante',
    photo_url: 'https://images.unsplash.com/photo-1506794778202-cad84cf45f1d?w=200',
    fonction: 'Professeur d\'histoire',
    pays: 'Ghana',
    specialite: 'Éducation',
    biographie: 'Enseignant à l\'Université du Ghana, je me consacre à l\'étude et à la transmission de l\'histoire panafricaine et des mouvements d\'indépendance.',
  },
  {
    id: '4',
    user_id: 'user-4',
    prenom: 'Fatoumata',
    nom: 'Keita',
    photo_url: 'https://images.unsplash.com/photo-1494790108377-be9c29b29330?w=200',
    fonction: 'Consultante en développement',
    pays: 'Côte d\'Ivoire',
    specialite: 'Migrations',
    biographie: 'Spécialiste des questions migratoires et de la diaspora africaine, j\'accompagne les initiatives de développement local et de retour au pays.',
  },
]

export interface Country {
  value: string
  label: string
}

export const countries: Country[] = [
  { value: 'Algerie', label: 'Algérie' },
  { value: 'Angola', label: 'Angola' },
  { value: 'Benin', label: 'Bénin' },
  { value: 'Botswana', label: 'Botswana' },
  { value: 'Burkina_Faso', label: 'Burkina Faso' },
  { value: 'Burundi', label: 'Burundi' },
  { value: 'Cameroun', label: 'Cameroun' },
  { value: 'Cap_Vert', label: 'Cap-Vert' },
  { value: 'Centrafrique', label: 'Centrafrique' },
  { value: 'Comores', label: 'Comores' },
  { value: 'Congo', label: 'Congo' },
  { value: 'Cote_dIvoire', label: 'Côte d\'Ivoire' },
  { value: 'Djibouti', label: 'Djibouti' },
  { value: 'Egypte', label: 'Égypte' },
  { value: 'Erythree', label: 'Érythrée' },
  { value: 'Ethiopie', label: 'Éthiopie' },
  { value: 'Gabon', label: 'Gabon' },
  { value: 'Gambie', label: 'Gambie' },
  { value: 'Ghana', label: 'Ghana' },
  { value: 'Guinee', label: 'Guinée' },
  { value: 'Guinee_Bissau', label: 'Guinée-Bissau' },
  { value: 'Guinee_Equatoriale', label: 'Guinée Équatoriale' },
  { value: 'Kenya', label: 'Kenya' },
  { value: 'Lesotho', label: 'Lesotho' },
  { value: 'Liberia', label: 'Liberia' },
  { value: 'Libye', label: 'Libye' },
  { value: 'Madagascar', label: 'Madagascar' },
  { value: 'Malawi', label: 'Malawi' },
  { value: 'Mali', label: 'Mali' },
  { value: 'Maroc', label: 'Maroc' },
  { value: 'Maurice', label: 'Maurice' },
  { value: 'Mauritanie', label: 'Mauritanie' },
  { value: 'Mozambique', label: 'Mozambique' },
  { value: 'Namibie', label: 'Namibie' },
  { value: 'Niger', label: 'Niger' },
  { value: 'Nigeria', label: 'Nigeria' },
  { value: 'Ouganda', label: 'Ouganda' },
  { value: 'RDC', label: 'RD Congo' },
  { value: 'Rwanda', label: 'Rwanda' },
  { value: 'Sao_Tome', label: 'São Tomé-et-Príncipe' },
  { value: 'Senegal', label: 'Sénégal' },
  { value: 'Seychelles', label: 'Seychelles' },
  { value: 'Sierra_Leone', label: 'Sierra Leone' },
  { value: 'Somalie', label: 'Somalie' },
  { value: 'Soudan', label: 'Soudan' },
  { value: 'Soudan_du_Sud', label: 'Soudan du Sud' },
  { value: 'Eswatini', label: 'Eswatini' },
  { value: 'Tanzanie', label: 'Tanzanie' },
  { value: 'Tchad', label: 'Tchad' },
  { value: 'Togo', label: 'Togo' },
  { value: 'Tunisie', label: 'Tunisie' },
  { value: 'Zambie', label: 'Zambie' },
  { value: 'Zimbabwe', label: 'Zimbabwe' },
  { value: 'Afrique_du_Sud', label: 'Afrique du Sud' },
]

export const specialites = [
  'Immigration africain',
  'Colonisation',
  'Société africaine',
  'Histoire',
  'Éducation et conseils à l\'africaine',
  'Mariage en Afrique',
  'Contes et proverbes',
  'Migrations',
  'Alliances entre peuples',
  'Totems et interdits',
  'Rites et initiations de peuples',
  'Bonnes pratiques',
  'Hommes historiques populaires',
  'Culture',
  'Générale',
  'Savoirs et innovations',
  'Spirituel et religions',
]

export const documentTypes = ['Livre', 'Article de revue', 'Rapport', 'Roman', 'Thèse', 'Mémoire', 'Autre']
export const filterTypes = ['Tous', 'Histoire', 'Culture', 'Éducation', 'Migrations']
export const rapportOptions = ['Auteur', 'Co-auteur', 'Aucun']

// Mock user pour l'affichage dans le formulaire d'ajout de document
export const mockCurrentUser = {
  uid: 'mock-user-1',
  nom: 'Diallo',
  prenom: 'Amadou',
  photo_url: null,
  email: 'amadou.diallo@example.com'
}
