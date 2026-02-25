<template>
  <div
    class="mt-5 cursor-pointer transition-all duration-200 text-center"
    :class="active ? 'scale-110 text-custom-green drop-shadow-lg' : 'text-white hover:scale-110'"
    @click="$emit('toggle')"
  >
    <div
      class="mx-auto w-12 h-12 rounded-lg flex items-center justify-center transition-colors duration-200"
      :class="active ? 'bg-white/25' : 'bg-transparent'"
    >
      <font-awesome-icon v-if="item.icon" :icon="item.icon" class="text-2xl" />
      <img v-else-if="item.iconImg" class="w-8 h-8" :src="item.iconImg" :alt="item.label" />
    </div>
    <div class="text-[10px] mt-1 font-medium leading-tight">{{ item.label }}</div>
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
