<script setup lang="ts">
/**
 * Dépôt du média d'une épreuve (feature 014, research D4), sur la route du jeu :
 * la photo y est normalisée (métadonnées EXIF retirées), le son reconnu par sa
 * signature binaire et plafonné à 1 Mo.
 *
 * La DURÉE d'un extrait se mesure ici, avant l'envoi : le serveur n'a pas de
 * bibliothèque audio, il ne borne que le poids.
 */
import { messageErreurAdminJeu } from '~/composables/useAdminJeu'

const DUREE_MAX_S = 30

const props = defineProps<{ type: 'image' | 'audio' }>()
const url = defineModel<string | null>({ required: true })

const { deposerMedia } = useAdminJeu()

const envoi = ref(false)
const erreur = ref('')
const source = computed(() => urlMedia(url.value))

/** Durée d'un fichier sonore, lue par le navigateur. */
const mesurerDuree = (fichier: File) => new Promise<number>((resoudre, rejeter) => {
  const lien = URL.createObjectURL(fichier)
  const audio = new Audio()
  audio.preload = 'metadata'
  audio.onloadedmetadata = () => { URL.revokeObjectURL(lien); resoudre(audio.duration) }
  audio.onerror = () => { URL.revokeObjectURL(lien); rejeter(new Error('illisible')) }
  audio.src = lien
})

const choisir = async (evenement: Event) => {
  const champ = evenement.target as HTMLInputElement
  const fichier = champ.files?.[0]
  champ.value = ''
  if (!fichier) return
  erreur.value = ''

  if (props.type === 'audio') {
    try {
      const duree = await mesurerDuree(fichier)
      if (!Number.isFinite(duree) || duree > DUREE_MAX_S) {
        erreur.value = `Extrait trop long (${Math.round(duree)} s) : ${DUREE_MAX_S} secondes au plus.`
        return
      }
    }
    catch {
      erreur.value = 'Ce fichier sonore est illisible par le navigateur.'
      return
    }
  }

  envoi.value = true
  try {
    const depose = await deposerMedia(fichier, props.type)
    if (depose) url.value = depose.media_url
  }
  catch (e) {
    erreur.value = messageErreurAdminJeu(e, 'Le fichier n\'a pas pu être déposé.')
  }
  finally {
    envoi.value = false
  }
}
</script>

<template>
  <div class="flex flex-col gap-2">
    <div v-if="source" class="flex flex-wrap items-center gap-3">
      <img v-if="type === 'image'" :src="source" alt="" class="max-h-40 rounded-lg border border-base-300 object-contain">
      <audio v-else :src="source" controls class="w-full max-w-md" />
      <button type="button" class="btn btn-ghost btn-xs" @click="url = null">Retirer</button>
    </div>

    <label class="flex flex-col">
      <span class="label-text mb-1 font-medium">
        {{ type === 'image' ? 'Image de l\'énoncé (JPEG, PNG ou WebP)' : `Extrait sonore (MP3, OGG, M4A ou WAV, ${DUREE_MAX_S} s et 1 Mo au plus)` }}
      </span>
      <input
        type="file"
        class="file-input file-input-bordered file-input-sm w-full max-w-md"
        :accept="type === 'image' ? 'image/jpeg,image/png,image/webp' : 'audio/mpeg,audio/ogg,audio/mp4,audio/x-m4a,audio/wav'"
        :disabled="envoi"
        @change="choisir"
      >
    </label>
    <span v-if="envoi" class="loading loading-spinner loading-sm" />
    <p v-if="erreur" class="text-sm text-error" role="alert">{{ erreur }}</p>
  </div>
</template>
