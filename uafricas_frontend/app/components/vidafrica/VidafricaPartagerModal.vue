<script setup lang="ts">
import type { VideoAfrica, PartageVideoAPI } from '~/composables/useVidafrica'

/**
 * Partage d'une vidéo sur le mur communautaire.
 *
 * Contrairement aux cinq autres modales de partage, celle-ci APPELLE l'API
 * elle-même au lieu de déléguer à la page. Son contrat est conservé
 * (`v-model` + événement `partage`), seule la coquille est partagée.
 */
const props = defineProps<{ modelValue: boolean, video: VideoAfrica }>()

const emit = defineEmits<{
  'update:modelValue': [valeur: boolean]
  'partage': [resultat: PartageVideoAPI]
}>()

const { partagerVideo } = useVidafrica()

const coquille = ref<{ setLoading: (v: boolean) => void, setError: (m: string) => void, setSuccess: () => void } | null>(null)

const soumettre = async (legende: string) => {
  coquille.value?.setLoading(true)
  try {
    const resultat = await partagerVideo(props.video.id, legende || undefined)
    if (resultat) {
      coquille.value?.setSuccess()
      emit('partage', resultat)
    }
    else {
      coquille.value?.setError('Le partage a échoué.')
    }
  }
  catch (e: any) {
    coquille.value?.setError(e?.data?.error || e?.message || 'Erreur lors du partage.')
  }
}
</script>

<template>
  <AfricansModalePartage
    ref="coquille"
    :is-open="modelValue"
    titre="Partager cette vidéo"
    :texte-partage="`Découvrez ${video.titre} sur AfricanS`"
    type-objet="video"
    :objet-id="video.id"
    @close="emit('update:modelValue', false)"
    @submit="soumettre"
  >
    Vous partagez <strong class="font-bold text-af-encre">{{ video.titre }}</strong>.
  </AfricansModalePartage>
</template>
