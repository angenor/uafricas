// Mock data pour l'authentification

export interface MockUser {
  uid: string
  email: string
  prenom: string
  nom: string
  photoURL: string | null
  roles: string[]
  statut: 'validé' | 'en_attente' | 'bloqué'
  dateCreation: Date
  stats: {
    annoncesPubliees: number
    contributionsValidees: number
  }
}

// Utilisateurs de test
export const mockUsers: MockUser[] = [
  {
    uid: 'user-001',
    email: 'test@uafricas.com',
    prenom: 'Jean',
    nom: 'Dupont',
    photoURL: '/images/profil1.jpg',
    roles: ['user'],
    statut: 'validé',
    dateCreation: new Date('2024-01-15'),
    stats: {
      annoncesPubliees: 5,
      contributionsValidees: 12,
    },
  },
  {
    uid: 'user-002',
    email: 'admin@uafricas.com',
    prenom: 'Marie',
    nom: 'Koné',
    photoURL: null,
    roles: ['user', 'admin'],
    statut: 'validé',
    dateCreation: new Date('2023-06-20'),
    stats: {
      annoncesPubliees: 25,
      contributionsValidees: 48,
    },
  },
  {
    uid: 'user-003',
    email: 'demo@uafricas.com',
    prenom: 'Kofi',
    nom: 'Mensah',
    photoURL: '/images/profil2.jpg',
    roles: ['user'],
    statut: 'validé',
    dateCreation: new Date('2024-03-10'),
    stats: {
      annoncesPubliees: 2,
      contributionsValidees: 7,
    },
  },
]

// Simuler un délai réseau
const delay = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms))

// Fonction de connexion mock
export const mockLogin = async (email: string, password: string): Promise<MockUser> => {
  await delay(800) // Simuler latence réseau

  const user = mockUsers.find((u) => u.email.toLowerCase() === email.toLowerCase())

  if (!user) {
    throw new Error('Aucun compte trouvé avec cet email')
  }

  // Pour le mock, on accepte n'importe quel mot de passe non vide
  if (!password || password.length < 4) {
    throw new Error('Mot de passe incorrect')
  }

  if (user.statut === 'bloqué') {
    throw new Error('Ce compte a été bloqué. Contactez le support.')
  }

  return user
}

// Fonction de connexion Google mock
export const mockLoginWithGoogle = async (): Promise<MockUser> => {
  await delay(1000) // Simuler popup Google

  // Retourner un utilisateur par défaut pour Google
  return {
    uid: 'google-' + Date.now(),
    email: 'google.user@gmail.com',
    prenom: 'Utilisateur',
    nom: 'Google',
    photoURL: 'https://lh3.googleusercontent.com/a/default-user',
    roles: ['user'],
    statut: 'validé',
    dateCreation: new Date(),
    stats: {
      annoncesPubliees: 0,
      contributionsValidees: 0,
    },
  }
}

// Fonction de déconnexion mock
export const mockLogout = async (): Promise<void> => {
  await delay(300)
}
