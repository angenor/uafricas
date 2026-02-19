<template>
  <div
    :class="open ? 'max-h-96 border-t-4 border-custom-chocolat' : 'max-h-0'"
    class="absolute top-full -left-4 z-50 w-48 whitespace-nowrap text-custom-chocolat overflow-hidden transition-all duration-300"
  >
    <NuxtLink
      v-for="(item, i) in items"
      :key="item.to"
      :to="item.to"
    >
      <div
        :class="open ? 'translate-x-0 opacity-100' : '-translate-x-full opacity-0'"
        :style="{ transitionDelay: open ? `${i * 50}ms` : `${(items.length - i) * 50}ms` }"
        class="px-3 py-2 bg-white/80 cursor-pointer transition-all duration-300 hover:border-l-4 hover:border-custom-green hover:pl-4 hover:text-custom-green hover:bg-white/70"
      >
        {{ item.label }}
      </div>
    </NuxtLink>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  open: boolean
  items: Array<{ label: string; to: string }>
}>()
</script>
