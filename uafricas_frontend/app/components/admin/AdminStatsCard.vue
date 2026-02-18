<script setup lang="ts">
import type { StatsCardData } from '~/types/admin'

defineProps<{
  stat: StatsCardData
}>()
</script>

<template>
  <div class="card bg-base-100 shadow-sm">
    <div class="card-body p-5">
      <div class="flex items-center justify-between">
        <div>
          <p class="text-sm text-base-content/60">
            {{ stat.label }}
          </p>
          <p class="text-2xl font-bold mt-1">
            {{ stat.value }}
          </p>
          <p
            v-if="stat.variation"
            class="text-xs mt-1"
            :class="{
              'text-success': stat.variation.type === 'hausse',
              'text-error': stat.variation.type === 'baisse',
              'text-base-content/50': stat.variation.type === 'stable',
            }"
          >
            <span v-if="stat.variation.type === 'hausse'">+</span>
            <span v-else-if="stat.variation.type === 'baisse'">-</span>
            {{ stat.variation.valeur }}%
          </p>
        </div>
        <div
          class="flex h-12 w-12 items-center justify-center rounded-lg"
          :class="stat.color || 'bg-primary/10 text-primary'"
        >
          <font-awesome-icon :icon="stat.icon" class="text-xl" />
        </div>
      </div>
    </div>
  </div>
</template>
