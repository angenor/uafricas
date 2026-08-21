<template>
  <!--
    Gabarit de la refonte. Mesuré dans Figma sur un artboard de 1920 :
    conteneur 1443, colonnes 312 / 733 / 312.

    Deux écarts de la maquette sont corrigés ici plutôt que reproduits :
      - le conteneur y est décentré (174 px à gauche, 303 à droite) → centré ;
      - les gouttières y valent 73 et 26 px → une seule valeur de 40 px, qui
        redonne 739 px à la colonne principale, à 6 px du 733 d'origine.
  -->
  <div class="flex min-h-screen flex-col bg-af-fond font-af text-af-encre antialiased">
    <AfricansBarreSuperieure />

    <slot name="bandeau" />

    <div class="mx-auto w-full max-w-af-conteneur flex-1 px-6">
      <slot name="fil-ariane" />

      <div class="af-grille py-8">
        <AfricansNavLaterale class="af-zone-nav" />
        <main class="af-zone-principale min-w-0"><slot /></main>
        <aside v-if="$slots.rail" class="af-zone-rail min-w-0">
          <div class="flex flex-col gap-6"><slot name="rail" /></div>
        </aside>
      </div>
    </div>

    <!-- Services globaux repris du layout par défaut : montés hors du <slot/>,
         c'est ce placement qui fait survivre l'écoute et la messagerie à la
         navigation. Les retirer ici casserait les deux. -->
    <ClientOnly>
      <SocialMessagerieFlottante v-if="estConnecte" />
    </ClientOnly>
    <ClientOnly>
      <MediaBarreLecturePersistante />
    </ClientOnly>
  </div>
</template>

<script setup lang="ts">
import { useUserStore } from '~/stores/user'

const userStore = useUserStore()
const estConnecte = computed(() => userStore.isAuthenticated)
</script>

<style>
/* Grille en zones nommées : le rail passe sous le contenu avant de disparaître,
   et la navigation devient une bande horizontale en mobile sans changer d'ordre
   dans le DOM — l'ordre de lecture reste celui de la maquette. */
.af-grille {
  display: grid;
  gap: 2.5rem;
  grid-template-areas:
    'nav'
    'principale';
  grid-template-columns: minmax(0, 1fr);
}
.af-zone-nav { grid-area: nav; }
.af-zone-principale { grid-area: principale; }
.af-zone-rail { grid-area: rail; }

/* Le rail n'apparaît qu'à partir de 1024 px : en dessous, ses panneaux sont
   secondaires (filtres, statistiques) et écraseraient la colonne principale. */
@media (min-width: 64rem) {
  .af-grille {
    grid-template-areas:
      'nav principale'
      'nav rail';
    grid-template-columns: var(--spacing-af-colonne) minmax(0, 1fr);
    align-content: start;
  }
}

@media (min-width: 80rem) {
  .af-grille {
    grid-template-areas: 'nav principale rail';
    grid-template-columns:
      var(--spacing-af-colonne)
      minmax(0, 1fr)
      var(--spacing-af-colonne);
  }
}
</style>
