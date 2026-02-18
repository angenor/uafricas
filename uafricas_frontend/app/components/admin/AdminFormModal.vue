<script setup lang="ts">
const props = defineProps<{
  /** Modal visible */
  visible: boolean
  /** Titre du modal */
  titre: string
  /** Mode : creation ou edition */
  mode?: 'creation' | 'edition'
  /** Chargement en cours */
  loading?: boolean
  /** Largeur du modal (classes daisyUI) */
  taille?: 'sm' | 'md' | 'lg' | 'xl'
}>()

const emit = defineEmits<{
  'update:visible': [value: boolean]
  soumettre: []
}>()

const tailleClasse = computed(() => {
  const map: Record<string, string> = {
    sm: 'max-w-sm',
    md: 'max-w-md',
    lg: 'max-w-2xl',
    xl: 'max-w-4xl',
  }
  return map[props.taille || 'lg']
})

const fermer = () => {
  if (!props.loading) {
    emit('update:visible', false)
  }
}
</script>

<template>
  <dialog class="modal" :class="{ 'modal-open': visible }">
    <div class="modal-box" :class="tailleClasse">
      <button
        class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
        :disabled="loading"
        @click="fermer"
      >
        <font-awesome-icon icon="xmark" />
      </button>

      <h3 class="font-bold text-lg mb-4">
        <font-awesome-icon
          :icon="mode === 'edition' ? 'pen-to-square' : 'plus'"
          class="mr-2"
        />
        {{ titre }}
      </h3>

      <form @submit.prevent="emit('soumettre')">
        <div class="space-y-4">
          <slot />
        </div>

        <div class="modal-action">
          <button
            type="button"
            class="btn btn-ghost"
            :disabled="loading"
            @click="fermer"
          >
            Annuler
          </button>
          <button
            type="submit"
            class="btn btn-primary"
            :class="{ loading }"
            :disabled="loading"
          >
            <font-awesome-icon
              v-if="!loading"
              :icon="mode === 'edition' ? 'floppy-disk' : 'plus'"
              class="mr-1"
            />
            {{ mode === 'edition' ? 'Enregistrer' : 'Creer' }}
          </button>
        </div>
      </form>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button @click="fermer">
        fermer
      </button>
    </form>
  </dialog>
</template>
