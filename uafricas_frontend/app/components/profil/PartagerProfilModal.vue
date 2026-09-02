<script setup lang="ts">
/**
 * Partage d'un profil sur le mur communautaire.
 *
 * Cette modale et `UniversiteGouvernancePartagerContributionModal` étaient
 * IDENTIQUES à quatre phrases près, 145 lignes chacune. Elles se réduisent à
 * ce qui les distingue vraiment ; la coquille est commune.
 */
defineProps<{ isOpen: boolean, profilNom: string, profilPrenom: string }>()

defineEmits<{ (e: 'close'): void, (e: 'submit', legende: string): void }>()

// Les pages pilotent l'envoi par `ref` : la référence est relayée telle quelle.
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
    titre="Partager ce profil"
    succes-texte="Ce profil apparaît désormais sur la page Publications."
    @close="$emit('close')"
    @submit="$emit('submit', $event)"
  >
    Vous partagez le profil de
    <strong class="font-bold text-af-encre">{{ profilPrenom }} {{ profilNom }}</strong>
    sur le mur communautaire.
  </AfricansModalePartage>
</template>
