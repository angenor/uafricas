<template>
  <AfricansPanneau titre="Mon compte" icone="fa-solid fa-gear">
    <nav aria-label="Pages de mon compte" class="flex flex-col gap-1">
      <NuxtLink
        v-for="entree in NAV_COMPTE"
        :key="entree.vers"
        :to="entree.vers"
        class="flex items-center gap-3 rounded-lg px-3 py-2.5 text-[14px]/[1.4] transition"
        :class="route.path === entree.vers
          ? 'bg-af-chocolat/15 font-bold text-af-chocolat'
          : 'text-af-corps hover:bg-af-chocolat/[0.07]'"
        :aria-current="route.path === entree.vers ? 'page' : undefined"
      >
        <font-awesome-icon :icon="entree.icone" class="w-4 shrink-0 text-center" />
        <span class="min-w-0 truncate">{{ entree.libelle }}</span>
      </NuxtLink>
    </nav>
  </AfricansPanneau>
</template>

<script setup lang="ts">
/**
 * Rail commun aux huit pages de « Mon compte ».
 *
 * Comparaison EXACTE de la route et non par préfixe : les huit cibles sont
 * sœurs sous `/mon-compte/`, aucune n'en contient une autre, et un préfixe
 * n'apporterait ici qu'un risque de double surlignage.
 */
import { NAV_COMPTE } from '~/utils/navigation-compte'

const route = useRoute()
</script>
