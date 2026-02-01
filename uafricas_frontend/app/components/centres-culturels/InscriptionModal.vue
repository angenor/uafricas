<script setup lang="ts">
defineProps<{
  isOpen: boolean
}>()

const emit = defineEmits<{
  close: []
  submit: [options: { prioritaires: boolean; toutes: boolean }]
}>()

const prioritaires = ref(true)
const toutes = ref(true)

const handleSubmit = () => {
  emit('submit', { prioritaires: prioritaires.value, toutes: toutes.value })
}
</script>

<template>
  <Teleport to="body">
    <div
      v-if="isOpen"
      class="z-50 bg-opacity-40 min-h-screen backdrop-blur-sm w-screen bg-black fixed inset-0 flex items-start justify-center pt-14"
      @click.self="emit('close')"
    >
      <div
        class="bg-white px-6 py-5 w-full max-w-md overflow-hidden rounded-md border-t-8 border-green-700"
        data-aos="zoom-in"
        data-aos-duration="300"
      >
        <h3 class="text-xl font-bold text-gray-800 mb-4">
          S'inscrire aux notifications
        </h3>

        <div class="space-y-3">
          <label class="flex items-center cursor-pointer hover:bg-gray-50 p-2 rounded">
            <input
              v-model="prioritaires"
              class="accent-green-600 w-5 h-5"
              type="checkbox"
            />
            <span class="ml-3 text-gray-700">Recevoir les notifications prioritaires</span>
          </label>

          <label class="flex items-center cursor-pointer hover:bg-gray-50 p-2 rounded">
            <input
              v-model="toutes"
              class="accent-green-600 w-5 h-5"
              type="checkbox"
            />
            <span class="ml-3 text-gray-700">Recevoir toutes les notifications</span>
          </label>
        </div>

        <div class="flex space-x-4 justify-center mt-8">
          <button
            @click="emit('close')"
            class="px-4 py-2 bg-red-600 text-white rounded-md hover:bg-red-700 transition-colors"
          >
            Annuler
          </button>
          <button
            @click="handleSubmit"
            class="px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 transition-colors"
          >
            Soumettre
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>
