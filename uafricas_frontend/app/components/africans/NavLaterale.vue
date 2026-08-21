<template>
  <!-- Colonne de navigation : 312 px, 11 entrées fixes, pas vertical de 44 px.
       En dessous de lg, elle devient une bande horizontale défilante — la
       maquette ne prévoit rien pour le mobile, c'est une décision d'intégration. -->
  <nav
    class="af-nav flex gap-1 overflow-x-auto scrollbar-none py-2 lg:flex-col lg:overflow-visible lg:py-0"
    aria-label="Navigation principale"
  >
    <component
      :is="entree.vers ? 'NuxtLink' : 'span'"
      v-for="entree in NAV_AFRICANS"
      :key="entree.libelle"
      :to="entree.vers ?? undefined"
      class="flex shrink-0 items-center gap-4 rounded-lg px-4 py-[11px] text-base font-bold whitespace-nowrap transition-colors"
      :class="[
        estActive(entree)
          ? 'bg-af-chocolat/15 text-af-chocolat'
          : entree.vers
            ? 'text-af-encre hover:bg-af-chocolat/[0.07]'
            : 'cursor-not-allowed text-af-atone-2',
      ]"
      :title="entree.vers ? undefined : 'Route non encore rattachée'"
    >
      <font-awesome-icon :icon="entree.icone" class="size-6 shrink-0 text-lg" />
      <span>{{ entree.libelle }}</span>
    </component>
  </nav>
</template>

<script setup lang="ts">
import { NAV_AFRICANS, type EntreeNav } from '~/utils/navigation-africans'

const route = useRoute()

/**
 * Une entrée est active si la route courante commence par sa cible. Le préfixe
 * est nécessaire pour que `/codi-moi/quelque-chose` allume bien « Africarise ».
 * Le cas `/` est exclu du préfixe, sinon toutes les entrées s'allumeraient.
 */
function estActive(entree: EntreeNav): boolean {
  if (!entree.vers) return false
  if (entree.vers === '/') return route.path === '/'
  return route.path === entree.vers || route.path.startsWith(`${entree.vers}/`)
}
</script>
