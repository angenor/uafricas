<template>
  <div class="grid grid-cols-2 md:grid-cols-5 gap-4" data-aos="fade-up">
    <div class="bg-white rounded-xl shadow-md p-4 text-center">
      <font-awesome-icon :icon="['fas', 'door-open']" class="w-6 h-6 text-blue-500 mb-2" />
      <div class="text-2xl font-bold text-gray-900">{{ stats.total_salles }}</div>
      <div class="text-xs text-gray-500">Salles</div>
    </div>
    <div class="bg-white rounded-xl shadow-md p-4 text-center">
      <font-awesome-icon :icon="['fas', 'lock']" class="w-6 h-6 text-cyan-500 mb-2" />
      <div class="text-2xl font-bold text-gray-900">{{ stats.total_salles_privees }}</div>
      <div class="text-xs text-gray-500">Canal privé</div>
    </div>
    <div class="bg-white rounded-xl shadow-md p-4 text-center">
      <font-awesome-icon :icon="['fas', 'video']" class="w-6 h-6 text-emerald-500 mb-2" />
      <div class="text-2xl font-bold text-gray-900">{{ stats.sessions_en_cours }}</div>
      <div class="text-xs text-gray-500">En direct</div>
    </div>
    <div class="bg-white rounded-xl shadow-md p-4 text-center">
      <font-awesome-icon :icon="['fas', 'circle-check']" class="w-6 h-6 text-gray-400 mb-2" />
      <div class="text-2xl font-bold text-gray-900">{{ stats.sessions_terminees }}</div>
      <div class="text-xs text-gray-500">Terminées</div>
    </div>
    <div class="bg-white rounded-xl shadow-md p-4 text-center col-span-2 md:col-span-1">
      <font-awesome-icon :icon="['fas', 'users']" class="w-6 h-6 text-blue-600 mb-2" />
      <div class="text-2xl font-bold text-gray-900">{{ stats.total_participants_uniques }}</div>
      <div class="text-xs text-gray-500">Participants</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { AfrolangStats } from '~/composables/useAfrolang'

defineProps<{
  stats: AfrolangStats
}>()
</script>
