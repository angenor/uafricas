<template>
  <div class="relative">
    <div class="relative">
      <font-awesome-icon
        :icon="['fas', 'search']"
        class="absolute left-4 top-1/2 transform -translate-y-1/2 text-gray-400 w-5 h-5"
      />
      <input
        v-model="searchQuery"
        type="text"
        placeholder="Rechercher un projet..."
        class="w-full pl-12 pr-4 py-3 bg-white border border-gray-200 rounded-xl focus:outline-hidden focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all shadow-xs"
        @input="onInput"
      />
      <button
        v-if="searchQuery"
        class="absolute right-4 top-1/2 transform -translate-y-1/2 text-gray-400 hover:text-gray-600"
        @click="clearSearch"
      >
        <font-awesome-icon :icon="['fas', 'times']" class="w-4 h-4" />
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
const searchQuery = defineModel<string>({ default: '' })

const emit = defineEmits<{
  search: []
}>()

let debounceTimer: ReturnType<typeof setTimeout> | null = null

const onInput = () => {
  if (debounceTimer) {
    clearTimeout(debounceTimer)
  }
  debounceTimer = setTimeout(() => {
    emit('search')
  }, 500)
}

const clearSearch = () => {
  searchQuery.value = ''
  emit('search')
}

onUnmounted(() => {
  if (debounceTimer) {
    clearTimeout(debounceTimer)
  }
})
</script>
