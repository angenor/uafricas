<template>
  <!-- Carte d'un module ouvert au jeu. Un module qui n'a pas encore de quoi
       composer une partie n'est pas masqué : il s'annonce « bientôt disponible »
       et n'est pas cliquable (FR-005). -->
  <component
    :is="module.disponible ? LienNuxt : 'div'"
    :to="module.disponible ? `/activites/${module.code}` : undefined"
    class="flex items-center gap-4 rounded-[10px] border border-af-bordure bg-af-surface p-5 transition"
    :class="module.disponible
      ? 'hover:border-af-chocolat focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-af-chocolat'
      : 'opacity-60'"
  >
    <span class="grid size-12 shrink-0 place-items-center rounded-full bg-af-chocolat/10 text-af-chocolat">
      <font-awesome-icon :icon="`fa-solid fa-${module.icone || 'gamepad'}`" class="text-xl" />
    </span>
    <span class="min-w-0 flex-1">
      <span class="block truncate text-[17px]/[1.4] font-bold text-af-encre">{{ module.libelle }}</span>
      <span class="block text-[14px]/[1.4] text-af-atone">
        <template v-if="module.disponible">
          {{ module.epreuves_jouables }} épreuve{{ module.epreuves_jouables > 1 ? 's' : '' }}
        </template>
        <template v-else>Bientôt disponible</template>
      </span>
    </span>
    <font-awesome-icon
      v-if="module.disponible"
      icon="fa-solid fa-chevron-right"
      class="shrink-0 text-af-atone"
    />
  </component>
</template>

<script setup lang="ts">
import type { ModuleJeuAPI } from '~/composables/useJeu'

defineProps<{ module: ModuleJeuAPI }>()

// `<component :is="'NuxtLink'">` ne résout pas le composant : voir AfricansBouton.
const LienNuxt = resolveComponent('NuxtLink')
</script>
