<script setup lang="ts">
import type { VideoAfrica, PartageVideoAPI } from '~/composables/useVidafrica'

const props = defineProps<{
  modelValue: boolean
  video: VideoAfrica
}>()

const emit = defineEmits<{
  'update:modelValue': [valeur: boolean]
  'partage': [resultat: PartageVideoAPI]
}>()

const { partagerVideo } = useVidafrica()

const legende = ref('')
const erreur = ref('')
const chargement = ref(false)
const succes = ref(false)

const reinitialiser = () => {
  legende.value = ''
  erreur.value = ''
  chargement.value = false
  succes.value = false
}

const fermer = () => {
  if (chargement.value) return
  emit('update:modelValue', false)
  setTimeout(reinitialiser, 200)
}

const soumettre = async () => {
  erreur.value = ''
  if (legende.value.length > 500) {
    erreur.value = 'La légende ne doit pas dépasser 500 caractères.'
    return
  }
  chargement.value = true
  try {
    const resultat = await partagerVideo(props.video.id, legende.value.trim() || undefined)
    if (resultat) {
      succes.value = true
      emit('partage', resultat)
    } else {
      erreur.value = 'Le partage a échoué.'
    }
  } catch (e: any) {
    erreur.value = e?.data?.error || e?.message || 'Erreur lors du partage.'
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
      <div class="absolute inset-0 bg-black/50" @click="fermer" />

      <div class="relative w-full max-w-lg bg-white rounded-2xl shadow-xl max-h-[90vh] overflow-y-auto">
        <!-- En-tête -->
        <div class="flex items-center justify-between px-6 py-4 border-b border-gray-100">
          <h2 class="text-lg font-bold text-gray-900 font-['Oswald']">Partager la vidéo</h2>
          <button class="text-gray-400 hover:text-gray-600 transition-colors" :disabled="chargement" @click="fermer">
            <font-awesome-icon icon="xmark" class="text-xl" />
          </button>
        </div>

        <div class="px-6 py-5">
          <!-- Succès -->
          <div v-if="succes" class="text-center py-6">
            <div class="mx-auto w-14 h-14 rounded-full bg-custom-green/10 flex items-center justify-center mb-4">
              <font-awesome-icon icon="check" class="text-2xl text-custom-green" />
            </div>
            <p class="text-lg font-semibold text-gray-900">Vidéo partagée !</p>
            <p class="text-sm text-gray-500 mt-2">
              Votre partage apparaît désormais sur le mur des publications.
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

            <!-- Aperçu vidéo -->
            <div class="flex items-center gap-3 p-3 rounded-lg bg-gray-50 border border-gray-100">
              <div class="w-20 h-12 rounded-md bg-gray-200 overflow-hidden shrink-0 flex items-center justify-center">
                <img v-if="video.vignetteUrl" :src="video.vignetteUrl" :alt="video.titre" class="w-full h-full object-cover">
                <font-awesome-icon v-else icon="video" class="text-gray-400" />
              </div>
              <p class="text-sm font-medium text-gray-800 line-clamp-2">{{ video.titre }}</p>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Légende (optionnel)</label>
              <textarea
                v-model="legende"
                rows="3"
                maxlength="500"
                placeholder="Ajoutez un mot sur ce que vous partagez…"
                class="w-full px-3 py-2 rounded-lg border border-gray-300 text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-custom-chocolat/40 focus:border-custom-chocolat"
              />
              <p class="text-xs text-gray-400 mt-1 text-right">{{ legende.length }}/500</p>
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
                {{ chargement ? 'Partage…' : 'Partager' }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </Teleport>
</template>
