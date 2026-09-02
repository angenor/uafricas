<script setup lang="ts">
import type { TypeObjetElement } from '~/composables/useOpportuniteAfrique'

/** Partage d'un sous-objet d'une fiche territoire (site, recette, secteur…). */
defineProps<{
  isOpen: boolean
  /** Titre lisible du sous-objet (nom, titre, nom complet). */
  titre: string
  /** Libellé du type affiché dans l'en-tête (ex. « ce secteur »). */
  typeLabel: string
  typeObjet: TypeObjetElement
  objetId: string
  estConnecte?: boolean
}>()

defineEmits<{ (e: 'close'): void, (e: 'submit', legende: string): void }>()

const coquille = ref<{ setLoading: (v: boolean) => void, setError: (m: string) => void, setSuccess: () => void } | null>(null)
defineExpose({
  setLoading: (v: boolean) => coquille.value?.setLoading(v),
  setError: (m: string) => coquille.value?.setError(m),
  setSuccess: () => coquille.value?.setSuccess(),
})
</script>

<template>
  <AfricansModalePartage
    ref="coquille"
    :is-open="isOpen"
    :est-connecte="estConnecte !== false"
    :titre="`Partager ${typeLabel}`"
    :texte-partage="`Découvrez ${titre} sur AfricanS`"
    :type-objet="typeObjet"
    :objet-id="objetId"
    @close="$emit('close')"
    @submit="$emit('submit', $event)"
  >
    Vous partagez <strong class="font-bold text-af-encre">{{ titre }}</strong>.
  </AfricansModalePartage>
</template>
