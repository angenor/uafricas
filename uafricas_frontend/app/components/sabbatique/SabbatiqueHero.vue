<template>
  <!-- Hero compact : le contenu s'adapte au type de programme -->
  <div class="relative w-full bg-font-baniere-ethnie bg-cover bg-center">
    <div class="absolute inset-0 bg-af-chocolat/50"></div>

    <div class="relative max-w-4xl mx-auto px-4 pt-16 pb-8 text-center select-none">
      <div class="flex items-center justify-center min-h-10 md:min-h-12">
        <h1 class="text-white font-semibold text-2xl md:text-3xl">
          Programme d'échanges d'expériences (sabbatique)
        </h1>
      </div>

      <!-- Badge + accroche variables selon le type -->
      <Transition name="fade" mode="out-in">
        <div :key="type || 'defaut'" class="mt-3">
          <span
            v-if="contenu.badge"
            class="inline-flex items-center gap-2 rounded-full bg-white/15 px-3 py-1 text-xs md:text-sm font-medium text-white backdrop-blur-sm"
          >
            <font-awesome-icon :icon="['fas', contenu.icon]" />
            {{ contenu.badge }}
          </span>
          <p class="mt-3 text-sm md:text-base text-white/90 max-w-2xl mx-auto">
            {{ contenu.accroche }}
          </p>
        </div>
      </Transition>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { TypeProgramme } from '~/composables/useSabbatiques'

const props = defineProps<{
  /** Type de programme affiché ; vide = accroche générique */
  type?: TypeProgramme | ''
}>()

const CONTENUS: Record<TypeProgramme | 'defaut', { badge: string; accroche: string; icon: string }> = {
  interafricain: {
    badge: 'Interafricain',
    accroche:
      "Partagez votre expertise d'un pays d'Afrique à un autre et renforcez les compétences du continent.",
    icon: 'earth-africa',
  },
  hors_afrique: {
    badge: 'Hors Afrique vers Afrique',
    accroche:
      "Apportez votre expertise depuis l'international au service du développement en Afrique.",
    icon: 'plane-arrival',
  },
  defaut: {
    badge: '',
    accroche:
      "Mettez votre expérience professionnelle au service du développement durable en Afrique.",
    icon: 'earth-africa',
  },
}

const contenu = computed(() => CONTENUS[props.type || 'defaut'])
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
