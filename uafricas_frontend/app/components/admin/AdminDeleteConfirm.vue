<script setup lang="ts">
const props = defineProps<{
  /** Modal visible */
  visible: boolean
  /** Titre de l'element a supprimer */
  titre?: string
  /** Message de confirmation personnalise */
  message?: string
  /** Chargement en cours */
  loading?: boolean
}>()

const emit = defineEmits<{
  'update:visible': [value: boolean]
  confirmer: []
}>()

const fermer = () => {
  if (!props.loading) {
    emit('update:visible', false)
  }
}
</script>

<template>
  <dialog class="modal" :class="{ 'modal-open': visible }">
    <div class="modal-box max-w-md">
      <h3 class="font-bold text-lg text-error">
        <font-awesome-icon icon="triangle-exclamation" class="mr-2" />
        Confirmer la suppression
      </h3>
      <p class="py-4 text-base-content/80">
        {{ message || `Etes-vous sur de vouloir supprimer "${titre}" ? Cette action est irreversible.` }}
      </p>
      <div class="modal-action">
        <button
          class="btn btn-ghost"
          :disabled="loading"
          @click="fermer"
        >
          Annuler
        </button>
        <button
          class="btn btn-error"
          :class="{ loading }"
          :disabled="loading"
          @click="emit('confirmer')"
        >
          <font-awesome-icon v-if="!loading" icon="trash" class="mr-1" />
          Supprimer
        </button>
      </div>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button @click="fermer">
        fermer
      </button>
    </form>
  </dialog>
</template>
