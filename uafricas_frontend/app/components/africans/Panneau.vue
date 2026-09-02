<template>
  <!-- Panneau du rail droit. UN composant pour les neuf contenus relevés :
       Filtres, Statistiques, Tendances, Créateurs à suivre, Communautés,
       Mes ami(e)s, Posts populaires, Langues populaires, Partager cet avis. -->
  <section class="rounded-[10px] border border-af-bordure bg-white">
    <header v-if="titre" class="flex items-center gap-3 px-5 pt-5 pb-4">
      <font-awesome-icon v-if="icone" :icon="icone" class="size-6 text-af-chocolat" />
      <h2 class="text-[17px]/[1.4] font-bold">{{ titre }}</h2>
      <button
        v-if="actionLibelle"
        type="button"
        class="ml-auto text-sm font-bold text-af-chocolat transition hover:opacity-70"
        @click="$emit('action')"
      >
        {{ actionLibelle }}
      </button>
    </header>
    <div class="px-5 pb-5" :class="!titre && 'pt-5'">
      <slot />
    </div>
  </section>
</template>

<script setup lang="ts">
defineProps<{
  titre?: string
  icone?: string
  /** Lien discret à droite du titre, ex. « Réinitialiser ». */
  actionLibelle?: string
}>()

defineEmits<{ action: [] }>()
</script>
