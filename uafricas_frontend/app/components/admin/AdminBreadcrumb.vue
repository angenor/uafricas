<script setup lang="ts">
interface BreadcrumbItem {
  label: string
  to?: string
}

defineProps<{
  items: BreadcrumbItem[]
}>()
</script>

<template>
  <div class="breadcrumbs text-sm mb-4">
    <ul>
      <li>
        <NuxtLink to="/admin" class="text-base-content/60 hover:text-primary">
          <font-awesome-icon icon="gauge-high" class="mr-1" />
          Admin
        </NuxtLink>
      </li>
      <li v-for="(item, index) in items" :key="index">
        <NuxtLink
          v-if="item.to && index < items.length - 1"
          :to="item.to"
          class="text-base-content/60 hover:text-primary"
        >
          {{ item.label }}
        </NuxtLink>
        <span v-else class="font-medium">{{ item.label }}</span>
      </li>
    </ul>
  </div>
</template>
