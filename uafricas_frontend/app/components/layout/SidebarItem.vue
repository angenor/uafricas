<template>
  <div
    class="text-white mt-5 cursor-pointer hover:scale-110 transition-transform text-center"
    @click="$emit('toggle')"
  >
    <font-awesome-icon v-if="item.icon" :icon="item.icon" class="text-3xl" />
    <img v-else-if="item.iconImg" class="w-10 h-10 mx-auto" :src="item.iconImg" :alt="item.label" />
    <div class="text-xs mt-0.5">{{ item.label }}</div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  item: {
    id: string
    icon?: string
    iconImg?: string
    label: string
  }
  active: boolean
}>()

defineEmits<{
  toggle: []
}>()
</script>
