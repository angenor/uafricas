<template>
  <Transition
    enter-active-class="transition-all duration-200 ease-out"
    enter-from-class="opacity-0 translate-y-1"
    enter-to-class="opacity-100 translate-y-0"
    leave-active-class="transition-all duration-150 ease-in"
    leave-from-class="opacity-100 translate-y-0"
    leave-to-class="opacity-0 translate-y-1"
  >
    <div v-if="open" class="absolute top-full left-1/2 -translate-x-1/2 z-50 pt-2">
      <div class="w-96 flex bg-white rounded-xl shadow-xl border border-gray-100 overflow-hidden">
        <!-- Zone image à gauche -->
        <div class="relative w-28 shrink-0 overflow-hidden" :class="gradient">
          <img
            v-if="image"
            :src="image"
            :alt="menuLabel"
            class="absolute inset-0 w-full h-full object-cover opacity-40"
          />
          <div class="absolute inset-0 bg-black/20" />
          <div class="relative h-full flex flex-col justify-between p-3.5 z-10">
            <p class="text-white font-bold text-sm leading-tight drop-shadow-sm">{{ menuLabel }}</p>
            <NuxtLink
              :to="menuTo"
              class="inline-flex items-center gap-1.5 self-start text-white text-[11px] font-semibold bg-white/20 hover:bg-white/30 backdrop-blur-sm px-3 py-1.5 rounded-full transition-colors"
            >
              Découvrir
              <font-awesome-icon icon="fa-solid fa-arrow-right" class="text-[9px]" />
            </NuxtLink>
          </div>
        </div>

        <!-- Liens à droite -->
        <div class="flex-1 min-w-0">
          <div v-if="description" class="px-4 py-2 border-b border-gray-100">
            <p class="text-[11px] text-gray-500 leading-relaxed">{{ description }}</p>
          </div>
          <div class="p-1.5">
            <NuxtLink
              v-for="item in items"
              :key="item.to"
              :to="item.to"
              class="group flex items-start gap-3 px-3 py-2.5 rounded-lg hover:bg-gray-50 transition-all duration-150"
            >
              <div class="shrink-0 w-8 h-8 rounded-lg bg-orange-50 text-custom-chocolat flex items-center justify-center mt-0.5 group-hover:bg-green-50 group-hover:text-custom-green transition-colors duration-150">
                <font-awesome-icon :icon="item.icon" class="text-sm" />
              </div>
              <div class="min-w-0">
                <p class="text-sm font-medium text-gray-800 group-hover:text-custom-green transition-colors duration-150">
                  {{ item.label }}
                </p>
                <p class="text-[11px] text-gray-400 mt-0.5 leading-snug">
                  {{ item.description }}
                </p>
              </div>
            </NuxtLink>
          </div>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
defineProps<{
  open: boolean
  description?: string
  image?: string
  menuLabel: string
  menuTo: string
  gradient: string
  items: Array<{ label: string; to: string; description: string; icon: string }>
}>()
</script>
