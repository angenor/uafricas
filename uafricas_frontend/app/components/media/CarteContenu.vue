<script setup lang="ts">
/**
 * Carte d'un contenu dans une rangée horizontale.
 * Sert indifféremment un programme télé et une émission radio : seuls
 * l'illustration, le titre et l'action de lecture changent d'un cas à l'autre.
 */
withDefaults(defineProps<{
  titre: string
  image: string
  description?: string | null
  /** Marque le contenu mis en avant sur sa chaîne ou sa station. */
  aLaUne?: boolean
  /** Vrai quand ce contenu est celui qui joue en ce moment. */
  enLecture?: boolean
  lien?: string | null
  /** Rend la carte moins large : les rangées radio en affichent davantage. */
  compacte?: boolean
}>(), {
  description: null,
  aLaUne: false,
  enLecture: false,
  lien: null,
  compacte: false,
})

const emit = defineEmits<{ lire: [] }>()
</script>

<template>
  <article
    role="listitem"
    class="shrink-0 snap-start group/carte"
    :class="compacte ? 'w-48' : 'w-64'"
  >
    <button
      type="button"
      class="relative w-full aspect-video rounded-lg overflow-hidden bg-gray-800 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-yellow-400"
      :class="enLecture ? 'ring-2 ring-yellow-400' : ''"
      :aria-label="`Lire ${titre}`"
      @click="emit('lire')"
    >
      <img
        v-if="image"
        :src="image"
        :alt="titre"
        loading="lazy"
        class="w-full h-full object-cover transition-transform duration-300 group-hover/carte:scale-105"
      >
      <!-- Faute de couverture, un aplat sobre plutôt qu'une image cassée. -->
      <span v-else class="w-full h-full flex items-center justify-center bg-neutral-800">
        <font-awesome-icon :icon="['fas', 'image']" class="text-2xl text-neutral-600" />
      </span>

      <span
        class="absolute inset-0 bg-black/40 opacity-0 group-hover/carte:opacity-100 focus-visible:opacity-100 transition-opacity flex items-center justify-center"
        :class="enLecture ? 'opacity-100' : ''"
      >
        <span class="h-12 w-12 rounded-full bg-white/90 text-black flex items-center justify-center">
          <font-awesome-icon :icon="['fas', enLecture ? 'volume-high' : 'play']" />
        </span>
      </span>

      <span
        v-if="aLaUne"
        class="absolute top-2 left-2 rounded bg-yellow-400 text-black text-[10px] font-bold px-1.5 py-0.5 uppercase"
      >
        À la une
      </span>
    </button>

    <div class="mt-2 px-0.5">
      <!-- `component :is` avec resolveComponent() échoue à l'exécution pour
           NuxtLink : le composant doit être résolu au rendu, pas dans une
           expression de template. D'où les deux branches explicites. -->
      <NuxtLink
        v-if="lien"
        :to="lien"
        class="block text-white text-sm font-semibold truncate hover:text-yellow-400 transition-colors"
        :title="titre"
      >
        {{ titre }}
      </NuxtLink>
      <p v-else class="block text-white text-sm font-semibold truncate" :title="titre">
        {{ titre }}
      </p>
      <p v-if="description" class="text-gray-400 text-xs line-clamp-2 mt-0.5">{{ description }}</p>
    </div>
  </article>
</template>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
