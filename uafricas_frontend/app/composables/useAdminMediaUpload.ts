import type { ApiResponse } from '~/types/admin'

// Upload de fichiers média (vidéo/audio) pour le back-office radio & télé.
export const useAdminMediaUpload = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBaseUrl as string
  const userStore = useUserStore()
  const erreurUpload = ref<string | null>(null)

  // Téléverse un fichier média → renvoie l'URL relative stockée (/uploads/...)
  const uploaderMedia = async (fichier: File): Promise<string | null> => {
    erreurUpload.value = null
    const formData = new FormData()
    formData.append('fichier', fichier)
    try {
      const reponse = await $fetch<ApiResponse<{ url: string }>>(`${apiBase}/api/admin/medias/upload`, {
        method: 'POST',
        body: formData,
        // Pas de Content-Type manuel : le navigateur pose le boundary multipart
        headers: { Authorization: `Bearer ${userStore.accessToken}` },
      })
      return reponse.data?.url ?? null
    }
    catch (e: any) {
      erreurUpload.value = e?.data?.error || 'Échec du téléversement du média'
      return null
    }
  }

  // Résout une URL média relative (/uploads/...) en URL absolue pour l'aperçu
  const resoudreUrlMedia = (url: string): string => {
    if (!url) return ''
    if (url.startsWith('http://') || url.startsWith('https://')) return url
    return `${apiBase}${url}`
  }

  return { uploaderMedia, resoudreUrlMedia, erreurUpload }
}
