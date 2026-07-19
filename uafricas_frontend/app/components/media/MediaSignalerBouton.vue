<script setup lang="ts">
import type { TypeMedia } from '~/composables/useMediaSocial'

const props = withDefaults(
  defineProps<{
    typeMedia: TypeMedia
    mediaId: string
    /** Titre rappelé dans la modale de confirmation. */
    titre: string
    /**
     * `pilule` : bouton bordé sur fond sombre (pages de détail et sections).
     * `discret` : lien texte, pour les zones denses.
     */
    variante?: 'pilule' | 'discret'
  }>(),
  { variante: 'pilule' },
)

/**
 * `suspendu` remonte quand le signalement a fait franchir le seuil : la page
 * parente peut alors recharger et constater le retrait de l'antenne.
 */
const emit = defineEmits<{ (e: 'suspendu'): void }>()

const { signaler } = useMediaSocial()
const { redirigerVersConnexion } = useAuth()
const userStore = useUserStore()

const ouvert = ref(false)
const aSignale = ref(false)
const modalRef = ref<{
  setLoading: (v: boolean) => void
  setError: (m: string) => void
  setSuccess: (m: string) => void
} | null>(null)

// Convention du dossier `media/` : le composant lit lui-même la session plutôt
// que de recevoir `estAuthentifie` en prop (cf. SectionChaine).
const ouvrir = () => {
  if (!userStore.accessToken) {
    redirigerVersConnexion()
    return
  }
  ouvert.value = true
}

const soumettre = async (payload: { motif: string, description: string }) => {
  modalRef.value?.setLoading(true)
  try {
    const etat = await signaler(props.typeMedia, props.mediaId, payload)
    if (!etat) {
      modalRef.value?.setError('Une erreur est survenue. Veuillez réessayer.')
      return
    }
    aSignale.value = true
    if (etat.suspendu) emit('suspendu')
    // Le signaleur reste devant un contenu d'apparence intacte : sans énoncer
    // ce qui vient de se produire, il en déduit soit que son signalement n'est
    // pas passé, soit que la plateforme n'agit pas — les deux inférences allant
    // contre l'objectif de FR-049/050, qui est de rendre la modération
    // communautaire lisible par celui qui l'exerce.
    modalRef.value?.setSuccess(
      etat.suspendu
        ? 'Merci. Ce contenu a atteint le seuil de signalements : il vient d\'être retiré de l\'antenne et sera examiné par un administrateur.'
        : etat.deja_signale
          ? 'Vous aviez déjà signalé ce contenu. Votre signalement reste enregistré.'
          : 'Merci, votre signalement a été enregistré. Il sera examiné par un administrateur.',
    )
  } catch {
    modalRef.value?.setError('Une erreur est survenue. Veuillez réessayer.')
  }
}
</script>

<template>
  <button
    v-if="variante === 'pilule'"
    type="button"
    :disabled="aSignale"
    class="inline-flex items-center gap-2 rounded-full border px-4 py-2 text-sm transition-colors"
    :class="aSignale
      ? 'border-white/10 text-gray-500 cursor-default'
      : 'border-white/20 text-gray-300 hover:border-orange-400 hover:text-orange-400 cursor-pointer'"
    @click="ouvrir"
  >
    <font-awesome-icon :icon="['fas', 'flag']" />
    {{ aSignale ? 'Signalé' : 'Signaler' }}
  </button>

  <button
    v-else
    type="button"
    :disabled="aSignale"
    class="inline-flex items-center gap-1.5 text-sm transition-colors"
    :class="aSignale
      ? 'text-gray-400 cursor-default'
      : 'text-orange-500 hover:text-orange-400 hover:underline cursor-pointer'"
    @click="ouvrir"
  >
    <font-awesome-icon :icon="['fas', 'flag']" class="w-3.5 h-3.5" />
    {{ aSignale ? 'Signalé' : 'Signaler' }}
  </button>

  <MediaSignalerModal
    ref="modalRef"
    :is-open="ouvert"
    :titre="titre"
    @close="ouvert = false"
    @submit="soumettre"
  />
</template>
