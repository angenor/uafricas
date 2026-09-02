<script setup lang="ts">
/** Partage d'une contribution de gouvernance sur le mur communautaire. */
defineProps<{ isOpen: boolean, titre: string }>()

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
    titre="Partager cette publication"
    @close="$emit('close')"
    @submit="$emit('submit', $event)"
  >
    Vous partagez <strong class="font-bold text-af-encre">« {{ titre }} »</strong>
    sur le mur communautaire.
  </AfricansModalePartage>
</template>
