<script setup lang="ts">
const props = defineProps<{
  modelValue: boolean
}>()

const emit = defineEmits<{
  'update:modelValue': [valeur: boolean]
  'soumis': [resultat: { id: string; slug: string; titre: string }]
}>()

const { proposerVideo } = useVidafricaContribution()

const titre = ref('')
const description = ref('')
const fichierVideo = ref<File | null>(null)
const vignette = ref<File | null>(null)
const erreur = ref('')
const chargement = ref(false)
const succes = ref(false)

const MAX_VIDEO = 500 * 1024 * 1024
const MAX_VIGNETTE = 5 * 1024 * 1024

const reinitialiser = () => {
  titre.value = ''
  description.value = ''
  fichierVideo.value = null
  vignette.value = null
  erreur.value = ''
  chargement.value = false
  succes.value = false
}

const fermer = () => {
  if (chargement.value) return
  emit('update:modelValue', false)
  // Laisser l'animation se terminer avant de réinitialiser
  setTimeout(reinitialiser, 200)
}

const onVideoChange = (e: Event) => {
  const input = e.target as HTMLInputElement
  const f = input.files?.[0] || null
  erreur.value = ''
  if (!f) {
    fichierVideo.value = null
    return
  }
  const ext = f.name.split('.').pop()?.toLowerCase() || ''
  if (!['mp4', 'webm'].includes(ext)) {
    erreur.value = 'Format vidéo invalide. Formats acceptés : MP4, WebM.'
    input.value = ''
    return
  }
  if (f.size > MAX_VIDEO) {
    erreur.value = 'Le fichier vidéo dépasse la limite de 500 Mo.'
    input.value = ''
    return
  }
  fichierVideo.value = f
}

const onVignetteChange = (e: Event) => {
  const input = e.target as HTMLInputElement
  const f = input.files?.[0] || null
  erreur.value = ''
  if (!f) {
    vignette.value = null
    return
  }
  const ext = f.name.split('.').pop()?.toLowerCase() || ''
  if (!['jpg', 'jpeg', 'png', 'webp'].includes(ext)) {
    erreur.value = 'Format vignette invalide. Formats acceptés : JPG, PNG, WebP.'
    input.value = ''
    return
  }
  if (f.size > MAX_VIGNETTE) {
    erreur.value = 'La vignette dépasse la limite de 5 Mo.'
    input.value = ''
    return
  }
  vignette.value = f
}

const soumettre = async () => {
  erreur.value = ''
  if (!titre.value.trim()) {
    erreur.value = 'Le titre est requis.'
    return
  }
  if (!fichierVideo.value) {
    erreur.value = 'Le fichier vidéo est requis.'
    return
  }

  chargement.value = true
  try {
    const formData = new FormData()
    formData.append('titre', titre.value.trim())
    if (description.value.trim()) formData.append('description', description.value.trim())
    formData.append('fichier_video', fichierVideo.value)
    if (vignette.value) formData.append('vignette', vignette.value)

    const resultat = await proposerVideo(formData)
    if (resultat) {
      succes.value = true
      emit('soumis', { id: resultat.id, slug: resultat.slug, titre: resultat.titre })
    }
  } catch (e: any) {
    erreur.value = e?.data?.error || e?.message || 'Erreur lors de l\'envoi de la vidéo.'
  } finally {
    chargement.value = false
  }
}
</script>

<template>
  <Teleport to="body">
    <div
      v-if="props.modelValue"
      class="fixed inset-0 z-50 flex items-center justify-center p-4"
      role="dialog"
      aria-modal="true"
    >
      <!-- Fond -->
      <div class="absolute inset-0 bg-black/50" @click="fermer" />

      <!-- Boîte -->
      <div class="relative w-full max-w-lg bg-white rounded-2xl shadow-xl max-h-[90vh] overflow-y-auto">
        <!-- En-tête -->
        <div class="flex items-center justify-between px-6 py-4 border-b border-gray-100">
          <h2 class="text-lg font-bold text-gray-900 font-['Oswald']">Proposer une vidéo</h2>
          <button
            class="text-gray-400 hover:text-gray-600 transition-colors"
            :disabled="chargement"
            @click="fermer"
          >
            <font-awesome-icon icon="xmark" class="text-xl" />
          </button>
        </div>

        <!-- Corps -->
        <div class="px-6 py-5">
          <!-- Succès -->
          <div v-if="succes" class="text-center py-6">
            <div class="mx-auto w-14 h-14 rounded-full bg-custom-green/10 flex items-center justify-center mb-4">
              <font-awesome-icon icon="check" class="text-2xl text-custom-green" />
            </div>
            <p class="text-lg font-semibold text-gray-900">Vidéo envoyée !</p>
            <p class="text-sm text-gray-500 mt-2">
              Votre vidéo a été soumise et sera publiée après validation par un administrateur.
            </p>
            <button
              class="mt-5 px-5 py-2 rounded-lg bg-custom-chocolat text-white text-sm font-medium hover:bg-custom-chocolat/90 transition-colors"
              @click="fermer"
            >
              Fermer
            </button>
          </div>

          <!-- Formulaire -->
          <form v-else class="space-y-4" @submit.prevent="soumettre">
            <div v-if="erreur" class="rounded-lg bg-red-50 border border-red-200 px-3 py-2 text-sm text-red-700">
              {{ erreur }}
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Titre <span class="text-red-500">*</span></label>
              <input
                v-model="titre"
                type="text"
                maxlength="300"
                placeholder="Titre de la vidéo"
                class="w-full px-3 py-2 rounded-lg border border-gray-300 text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-custom-chocolat/40 focus:border-custom-chocolat"
              >
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Description</label>
              <textarea
                v-model="description"
                rows="3"
                placeholder="Décrivez brièvement la vidéo (optionnel)"
                class="w-full px-3 py-2 rounded-lg border border-gray-300 text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-custom-chocolat/40 focus:border-custom-chocolat"
              />
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Fichier vidéo <span class="text-red-500">*</span></label>
              <input
                type="file"
                accept=".mp4,.webm"
                class="block w-full text-sm text-gray-600 file:mr-3 file:py-2 file:px-4 file:rounded-lg file:border-0 file:text-sm file:font-medium file:bg-custom-chocolat/10 file:text-custom-chocolat hover:file:bg-custom-chocolat/20 cursor-pointer"
                @change="onVideoChange"
              >
              <p class="text-xs text-gray-400 mt-1">MP4 ou WebM, 500 Mo maximum.</p>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Vignette</label>
              <input
                type="file"
                accept=".jpg,.jpeg,.png,.webp"
                class="block w-full text-sm text-gray-600 file:mr-3 file:py-2 file:px-4 file:rounded-lg file:border-0 file:text-sm file:font-medium file:bg-gray-100 file:text-gray-700 hover:file:bg-gray-200 cursor-pointer"
                @change="onVignetteChange"
              >
              <p class="text-xs text-gray-400 mt-1">JPG, PNG ou WebP, 5 Mo maximum (optionnel).</p>
            </div>

            <div class="flex justify-end gap-2 pt-2">
              <button
                type="button"
                class="px-4 py-2 rounded-lg text-sm font-medium text-gray-600 hover:bg-gray-100 transition-colors"
                :disabled="chargement"
                @click="fermer"
              >
                Annuler
              </button>
              <button
                type="submit"
                class="px-5 py-2 rounded-lg bg-custom-chocolat text-white text-sm font-medium hover:bg-custom-chocolat/90 transition-colors disabled:opacity-60 inline-flex items-center gap-2"
                :disabled="chargement"
              >
                <span v-if="chargement" class="animate-spin rounded-full h-4 w-4 border-b-2 border-white" />
                {{ chargement ? 'Envoi…' : 'Envoyer' }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </Teleport>
</template>
