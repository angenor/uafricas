<script setup lang="ts">
import { useOpportuniteAfrique, type TypeObjetContribution } from '~/composables/useOpportuniteAfrique'

const props = defineProps<{
  typeObjet: TypeObjetContribution
  objetId: string
  libelle: string
  aSignale: boolean
  estAuthentifie: boolean
}>()

const emit = defineEmits<{
  (e: 'require-login'): void
  /** Émis quand la contribution bascule en suspendu (>10 signalements). */
  (e: 'suspendu'): void
}>()

const { signalerContribution } = useOpportuniteAfrique()

const ouvert = ref(false)
const aSignaleLocal = ref(props.aSignale)
const modalRef = ref<{
  setLoading: (v: boolean) => void
  setError: (m: string) => void
  setSuccess: (m: string) => void
} | null>(null)

// Resynchronise si la donnée parent change (re-fetch).
watch(() => props.aSignale, (v) => { aSignaleLocal.value = v })

const ouvrir = () => {
  if (!props.estAuthentifie) {
    emit('require-login')
    return
  }
  ouvert.value = true
}

const soumettre = async (payload: { motif: string, description: string }) => {
  modalRef.value?.setLoading(true)
  const etat = await signalerContribution(props.typeObjet, props.objetId, payload)
  if (!etat) {
    modalRef.value?.setError('Une erreur est survenue. Veuillez réessayer.')
    return
  }
  aSignaleLocal.value = true
  if (etat.suspendu) emit('suspendu')
  modalRef.value?.setSuccess(
    etat.deja_signale
      ? 'Vous aviez déjà signalé cette contribution.'
      : 'Merci, votre signalement a été pris en compte.',
  )
}
</script>

<template>
  <span>
    <button
      type="button"
      :disabled="aSignaleLocal"
      class="inline-flex items-center gap-1 font-medium transition-colors"
      :class="aSignaleLocal
        ? 'text-gray-400 cursor-default'
        : 'text-af-corps transition hover:text-af-live cursor-pointer'"
      :title="aSignaleLocal ? 'Vous avez déjà signalé cette contribution' : 'Signaler cette contribution'"
      @click="ouvrir"
    >
      <font-awesome-icon :icon="['fas', 'flag']" class="w-3 h-3" />
      {{ aSignaleLocal ? 'Signalé' : 'Signaler' }}
    </button>

    <OpportuniteAfriqueSignalerContributionModal
      ref="modalRef"
      :is-open="ouvert"
      :libelle="libelle"
      @close="ouvert = false"
      @submit="soumettre"
    />
  </span>
</template>
