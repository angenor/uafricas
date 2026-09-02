<template>
  <!-- 312 x 48 dans Figma : exactement les dimensions de l'item de navigation.
       Ce n'est pas une coïncidence, c'est le module du rail. -->
  <label class="relative block">
    <span class="sr-only">{{ placeholder }}</span>
    <font-awesome-icon
      icon="fa-solid fa-magnifying-glass"
      class="pointer-events-none absolute top-1/2 left-5 size-6 -translate-y-1/2 text-af-atone-2"
    />
    <input
      type="search"
      :value="modelValue"
      :placeholder="placeholder"
      class="h-12 w-full rounded-lg border border-af-bordure bg-white pr-4 pl-14 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:border-af-chocolat focus:outline-none"
      @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
    />
  </label>
</template>

<script setup lang="ts">
withDefaults(defineProps<{
  modelValue?: string
  placeholder?: string
}>(), { placeholder: 'Rechercher…' })

defineEmits<{ 'update:modelValue': [string] }>()
</script>
