<script setup lang="ts">
/** Choix des notifications d'un centre culturel. */
defineProps<{ isOpen: boolean }>()

const emit = defineEmits<{
  close: []
  submit: [options: { prioritaires: boolean, toutes: boolean }]
}>()

const prioritaires = ref(true)
const toutes = ref(true)

const handleSubmit = () => {
  emit('submit', { prioritaires: prioritaires.value, toutes: toutes.value })
}
</script>

<template>
  <AfricansModale
    :model-value="isOpen"
    titre="S'inscrire aux notifications"
    icone="fa-solid fa-bell"
    @update:model-value="!$event && emit('close')"
  >
    <div class="flex flex-col gap-2">
      <label class="flex cursor-pointer items-center gap-3 rounded-lg px-3 py-2.5 text-[14px]/[1.4] text-af-corps transition hover:bg-af-fond">
        <input v-model="prioritaires" type="checkbox" class="size-4 accent-af-chocolat" />
        Recevoir les notifications prioritaires
      </label>
      <label class="flex cursor-pointer items-center gap-3 rounded-lg px-3 py-2.5 text-[14px]/[1.4] text-af-corps transition hover:bg-af-fond">
        <input v-model="toutes" type="checkbox" class="size-4 accent-af-chocolat" />
        Recevoir toutes les notifications
      </label>
    </div>

    <template #actions>
      <!-- « Annuler » était un bouton ROUGE, à côté d'un bouton vert : deux
           actions de même poids visuel, dont l'une n'engage rien. -->
      <button
        type="button"
        class="text-base font-bold text-af-corps transition hover:opacity-70"
        @click="emit('close')"
      >
        Annuler
      </button>
      <AfricansBouton icone="fa-solid fa-bell" @click="handleSubmit">S'inscrire</AfricansBouton>
    </template>
  </AfricansModale>
</template>
