<template>
  <!-- Bandeau de module : 315 px, image bornée à la largeur du conteneur.
       Dans Figma l'image démarre à x=174 sur un viewport de 1920 mais s'arrête
       à 1617, laissant 303 px de vide à droite. On centre : le décentrage était
       un défaut de composition, pas une intention. -->
  <section class="relative isolate h-af-bandeau w-full overflow-hidden bg-af-bordure">
    <div class="mx-auto h-full max-w-af-conteneur px-6">
      <div class="relative h-full overflow-hidden">
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

        <div class="relative flex h-full flex-col items-center justify-center px-8 text-center">
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
