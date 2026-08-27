<script setup lang="ts">
/**
 * Partage d'une fiche territoire, sur le mur et vers les réseaux.
 *
 * Les six URL de réseaux étaient reconstruites ici à la main, comme dans cinq
 * autres composants. Elles viennent désormais de `construireReseaux`.
 */
defineProps<{
  isOpen: boolean
  paysNom: string
  estConnecte?: boolean
  /**
   * Identifiant de la fiche, nécessaire au traçage des partages externes.
   * Optionnel : sans lui le partage fonctionne, il n'est simplement pas compté.
   */
  ficheId?: string
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
    titre="Partager ce territoire"
    :texte-partage="`Découvrez ${paysNom} sur AfricanS`"
    type-objet="fiche_pays"
    :objet-id="ficheId"
    @close="$emit('close')"
    @submit="$emit('submit', $event)"
  >
    Vous partagez la fiche de <strong class="font-bold text-af-encre">{{ paysNom }}</strong>.
  </AfricansModalePartage>
</template>
