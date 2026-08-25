<script setup lang="ts">
import World from '@svg-maps/world'

/**
 * Silhouette du territoire, tracée puis remplie.
 *
 * La maquette Figma pose une carte à droite des informations générales. Aucun
 * découpage infranational n'existe côté base, mais le CONTOUR de chaque pays,
 * lui, est déjà dans le projet : `@svg-maps/world` le fournit pour les 55
 * territoires africains, et c'est lui qui dessine la carte de
 * `/opportunite-afrique`. Aucun asset à télécharger, aucune requête réseau.
 *
 * Le cadrage est calculé à partir du chemin lui-même (`cadrageTerritoire`),
 * sans passer par le DOM : le `viewBox` est donc juste dès le rendu SERVEUR,
 * et l'accordéon peut rester replié sans rien fausser. La version précédente
 * mesurait via `getBBox()`, ce qui imposait d'attendre l'affichage réel.
 */
const props = defineProps<{
  /** Code ISO2 du territoire (`shared.pays.code_iso2`). */
  code: string | null
  nom: string
}>()

const trace = computed(() => {
  if (!props.code) return null
  const cible = props.code.toLowerCase()
  return World.locations.find(l => l.id.toLowerCase() === cible) ?? null
})

const viewBox = computed(() => (trace.value ? cadrageTerritoire(trace.value.path) : null))
</script>

<template>
  <figure v-if="trace && viewBox" class="flex flex-col items-center gap-2">
    <svg
      :viewBox="viewBox"
      class="w-full max-w-[320px]"
      role="img"
      :aria-label="`Carte du territoire : ${nom}`"
      xmlns="http://www.w3.org/2000/svg"
    >
      <!-- `pathLength="1"` normalise la longueur du tracé : l'animation part
           de 1 et va à 0 quelle que soit la taille réelle du contour. Sans
           lui, il faudrait `getTotalLength()`, donc le DOM, donc attendre. -->
      <path :d="trace.path" pathLength="1" class="carte-territoire" />
    </svg>
    <figcaption class="text-[12px]/[1.4] text-af-atone">{{ nom }}</figcaption>
  </figure>
</template>

<style scoped>
/* Bleus de la carte du Figma. Ils ne sont PAS dans la palette `af-*` et n'y
   entrent pas : ce sont les couleurs d'une illustration, pas des jetons de
   marque. Les nommer ici les garde locaux à la carte. */
.carte-territoire {
  --carte-surface: #b9c5e2;
  --carte-contour: #3e5fa9;

  fill: var(--carte-surface);
  stroke: var(--carte-contour);
  stroke-width: 0.6;
  stroke-linejoin: round;
  /* Le tracé se dessine, puis la surface se remplit. */
  fill-opacity: 0;
  stroke-dasharray: 1;
  stroke-dashoffset: 1;
  animation:
    carte-tracer 1.6s ease-out forwards,
    carte-remplir 0.6s ease-out 1.2s forwards;
}

@keyframes carte-tracer {
  to { stroke-dashoffset: 0; }
}

@keyframes carte-remplir {
  to { fill-opacity: 1; }
}

/* Une animation de 2 s n'est pas négociable pour qui a demandé moins de
   mouvement : la carte s'affiche alors d'emblée, entière. */
@media (prefers-reduced-motion: reduce) {
  .carte-territoire {
    animation: none;
    fill-opacity: 1;
    stroke-dashoffset: 0;
  }
}
</style>
