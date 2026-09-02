<script setup lang="ts">
import type { TypeObjetElement } from '~/composables/useOpportuniteAfrique'

/** Partage d'un sous-objet d'une fiche territoire (site, recette, secteur…). */
const props = defineProps<{
  isOpen: boolean
  /** Titre lisible du sous-objet (nom, titre, nom complet). */
  titre: string
  /** Libellé du type affiché dans l'en-tête (ex. « ce secteur »). */
  typeLabel: string
  typeObjet: TypeObjetElement
  objetId: string
  estConnecte?: boolean
}>()

defineEmits<{ (e: 'close'): void }>()

const { partagerElement } = useOpportuniteAfrique()

const coquille = ref<{
  setLoading: (v: boolean) => void
  setError: (m: string) => void
  setSuccess: () => void
} | null>(null)

/**
 * L'appel est porté ICI, et non par les quatre pages de détail.
 *
 * La coquille `AfricansModalePartage` se contente de ré-émettre `submit` ; un
 * `@submit` non branché ne produit aucune erreur, ni au build ni à l'exécution
 * — le bouton « Partager » reste inerte en silence. Ce composant a `typeObjet`
 * et `objetId`, les deux seuls arguments dont `partagerElement` a besoin.
 */
const soumettre = async (legende: string) => {
  coquille.value?.setLoading(true)
  const res = await partagerElement(props.typeObjet, props.objetId, legende || undefined)
  if (res) coquille.value?.setSuccess()
  else coquille.value?.setError('Erreur lors du partage. Veuillez réessayer.')
}
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
    @submit="soumettre"
  >
    Vous partagez <strong class="font-bold text-af-encre">{{ titre }}</strong>.
  </AfricansModalePartage>
</template>
