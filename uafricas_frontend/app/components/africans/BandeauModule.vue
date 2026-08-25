<template>
  <!-- Bandeau de module : 315 px, image PLEINE LARGEUR.
       Dans Figma l'image est bornée au conteneur (elle démarre à x=174 sur un
       viewport de 1920 et s'arrête à 1617). Reproduit tel quel, cela laissait
       une bande grise à gauche et à droite dès que l'écran dépassait la largeur
       du conteneur : le fond de la section transparaissait. L'image est donc
       étendue au viewport, tandis que le TEXTE reste, lui, aligné sur le
       conteneur : c'est la seule chose que l'alignement de la maquette servait
       vraiment. -->
  <section class="relative isolate h-af-bandeau w-full overflow-hidden bg-af-bordure">
    <img
      v-if="image"
      :src="image"
      :alt="''"
      class="absolute inset-0 size-full object-cover"
    />
    <!-- Voile : dégradé de marque quand il n'y a pas d'image, voile noir
         par-dessus l'image pour garantir la lisibilité du titre blanc. -->
    <div
      class="absolute inset-0"
      :class="image ? 'bg-black/40' : 'bg-af-degrade'"
    />

    <!-- Le conteneur ne borne plus l'image, seulement son contenu. Il reste
         `relative` pour que le lien d'aide se cale sur SON bord droit et non
         sur celui de la fenêtre. -->
    <div class="relative mx-auto h-full max-w-af-conteneur px-6">
      <div class="flex h-full flex-col items-center justify-center px-8 text-center">
        <h1 class="text-[48px]/[1.4] font-bold text-white">{{ titre }}</h1>
        <p v-if="sousTitre" class="mt-1 max-w-3xl text-base text-white/90">
          {{ sousTitre }}
        </p>
      </div>

      <!-- Lien d'explication, en bas à droite. -->
      <button
        v-if="aide"
        type="button"
        class="absolute right-6 bottom-5 flex items-center gap-2 text-base text-white underline underline-offset-4 transition hover:opacity-80"
        @click="$emit('aide')"
      >
        <font-awesome-icon icon="fa-solid fa-circle-question" class="size-6" />
        {{ aide }}
      </button>

      <!-- Emplacement libre : badge de région, bouton d'inscription… -->
      <div v-if="$slots.action" class="absolute right-6 bottom-5">
        <slot name="action" />
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
defineProps<{
  titre: string
  sousTitre?: string
  image?: string | null
  /** Libellé du lien d'aide, ex. « C'est quoi Codimoi ? ». */
  aide?: string
}>()

defineEmits<{ aide: [] }>()
</script>
