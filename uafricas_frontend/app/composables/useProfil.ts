// Composable pour la gestion du profil utilisateur
import { useUserStore } from '~/stores/user'

/** Reponse API standardisee */
interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

/** Profil complet de l'utilisateur */
export interface Profil {
  id: string
  nom: string
  prenom: string
  email: string
  slug: string | null
  photo_url: string | null
  telephone: string | null
  genre: string
  date_naissance: string | null
  fonction: string | null
  localite: string | null
  ville: string | null
  biographie: string | null
  langue_preferee: string
  pays_residence_id: string | null
  etat: string
  email_verifie: boolean
  roles: string[]
  derniere_connexion: string | null
  created_at: string
}

/** Donnees pour modifier le profil */
export interface ModifierProfilForm {
  nom?: string
  prenom?: string
  telephone?: string
  genre?: string
  date_naissance?: string | null
  fonction?: string
  localite?: string
  ville?: string
  biographie?: string
  langue_preferee?: string
  pays_residence_id?: string
}

/** Donnees pour changer le mot de passe */
export interface ChangerMotDePasseForm {
  ancien_mot_de_passe: string
  nouveau_mot_de_passe: string
  confirmation_mot_de_passe: string
}

export const useProfil = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()

  const profil = ref<Profil | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)
  const success = ref<string | null>(null)

  /** Headers avec token d'authentification */
  const authHeaders = computed(() => ({
    Authorization: `Bearer ${userStore.accessToken}`,
  }))

  /** Charger le profil complet depuis le backend */
  const chargerProfil = async () => {
    loading.value = true
    error.value = null

    try {
      const reponse = await $fetch<ApiResponse<Profil>>(
        `${apiBase}/api/auth/moi`,
        { headers: authHeaders.value },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors du chargement du profil')
      }

      profil.value = reponse.data
      return reponse.data
    }
    catch (e: any) {
      error.value = e?.data?.error || e?.message || 'Erreur reseau'
      throw e
    }
    finally {
      loading.value = false
    }
  }

  /** Modifier le profil */
  const modifierProfil = async (data: ModifierProfilForm) => {
    loading.value = true
    error.value = null
    success.value = null

    try {
      const reponse = await $fetch<ApiResponse<Profil>>(
        `${apiBase}/api/auth/profil`,
        {
          method: 'PUT',
          headers: authHeaders.value,
          body: data,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de la modification du profil')
      }

      profil.value = reponse.data

      // Mettre a jour le store Pinia avec les nouvelles infos basiques
      if (userStore.user) {
        userStore.user.nom = reponse.data.nom
        userStore.user.prenom = reponse.data.prenom
        userStore.user.photo_url = reponse.data.photo_url
      }

      success.value = 'Profil modifie avec succes'
      return reponse.data
    }
    catch (e: any) {
      error.value = e?.data?.error || e?.message || 'Erreur reseau'
      throw e
    }
    finally {
      loading.value = false
    }
  }

  /** Uploader une photo de profil */
  const changerPhoto = async (fichier: File) => {
    loading.value = true
    error.value = null
    success.value = null

    try {
      const formData = new FormData()
      formData.append('photo', fichier)

      const reponse = await $fetch<ApiResponse<{ photo_url: string }>>(
        `${apiBase}/api/auth/profil/photo`,
        {
          method: 'POST',
          headers: authHeaders.value,
          body: formData,
        },
      )

      if (!reponse.success || !reponse.data) {
        throw new Error(reponse.error || 'Erreur lors de l\'upload de la photo')
      }

      // Mettre a jour la photo dans le profil et le store
      if (profil.value) {
        profil.value.photo_url = reponse.data.photo_url
      }
      if (userStore.user) {
        userStore.user.photo_url = reponse.data.photo_url
      }

      success.value = 'Photo mise a jour avec succes'
      return reponse.data.photo_url
    }
    catch (e: any) {
      error.value = e?.data?.error || e?.message || 'Erreur reseau'
      throw e
    }
    finally {
      loading.value = false
    }
  }

  /** Changer le mot de passe */
  const changerMotDePasse = async (data: ChangerMotDePasseForm) => {
    loading.value = true
    error.value = null
    success.value = null

    try {
      const reponse = await $fetch<ApiResponse<{ message: string }>>(
        `${apiBase}/api/auth/changer-mot-de-passe`,
        {
          method: 'POST',
          headers: authHeaders.value,
          body: data,
        },
      )

      if (!reponse.success) {
        throw new Error(reponse.error || 'Erreur lors du changement de mot de passe')
      }

      success.value = reponse.data?.message || 'Mot de passe modifie avec succes'
      return true
    }
    catch (e: any) {
      error.value = e?.data?.error || e?.message || 'Erreur reseau'
      throw e
    }
    finally {
      loading.value = false
    }
  }

  /** Effacer les messages */
  const effacerMessages = () => {
    error.value = null
    success.value = null
  }

  return {
    profil: readonly(profil),
    loading: readonly(loading),
    error: readonly(error),
    success: readonly(success),
    chargerProfil,
    modifierProfil,
    changerPhoto,
    changerMotDePasse,
    effacerMessages,
  }
}
