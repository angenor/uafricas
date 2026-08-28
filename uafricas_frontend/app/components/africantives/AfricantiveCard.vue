<template>
  <NuxtLink
    :to="`/africantives/${africantive.id}`"
    class="group flex flex-col overflow-hidden rounded-[10px] border border-af-bordure bg-white transition hover:-translate-y-1 hover:border-af-chocolat"
  >
    <!-- Image container -->
    <div class="relative aspect-[16/10] shrink-0 overflow-hidden">
      <img
        v-if="africantive.image_couverture_url"
        :src="africantive.image_couverture_url"
        :alt="africantive.titre"
        class="w-full h-full object-cover transition-transform duration-500 group-hover:scale-110"
      />
  <!-- Pas de `<img src="…placeholder.jpg">` : ce fichier n'a jamais existé, si
       bien qu'une annonce sans photo affichait une image CASSÉE avec son texte
       de remplacement en travers. Un repli qui doit exister sur le disque est un
       repli qui peut manquer ; celui-ci est du balisage, il ne peut pas
       échouer. -->
      <div v-else class="grid h-full w-full place-items-center bg-af-fond">
        <font-awesome-icon icon="fa-solid fa-lightbulb" class="text-4xl text-af-atone-2" />
      </div>

      <!-- Badge domaine -->
      <span
        v-if="africantive.domaine"
        class="absolute top-3 left-3 rounded-full bg-af-chocolat px-3 py-1.5 text-xs font-bold text-white"
      >
        {{ africantive.domaine }}
      </span>

      <!-- Overlay gradient -->
      <div class="absolute inset-0 bg-gradient-to-t from-black/20 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
    </div>

    <!-- Contenu -->
    <div class="flex flex-1 flex-col p-4">
      <!-- Localisation -->
      <div class="flex items-center text-sm text-af-atone mb-2">
        <font-awesome-icon :icon="['fas', 'location-dot']" class="w-3 h-3 mr-1.5 text-af-vert" />
        <span>{{ africantive.pays || 'Afrique' }}</span>
        <span v-if="africantive.ville" class="text-af-bordure mx-1">&bull;</span>
        <span v-if="africantive.ville" class="text-af-atone-2">{{ africantive.ville }}</span>
      </div>

      <!-- Titre -->
      <h3 class="font-semibold text-af-encre line-clamp-2 mb-2 group-hover:text-af-chocolat transition-colors">
        {{ africantive.titre }}
      </h3>

      <!-- Description tronquee -->
      <p class="text-sm text-af-atone line-clamp-2 mb-3">
        {{ africantive.description }}
      </p>

      <!-- Auteur + date -->
      <div class="mt-auto flex items-center justify-between border-t border-af-bordure pt-3">
        <div class="flex items-center gap-2">
          <div class="w-7 h-7 bg-af-degrade rounded-full flex items-center justify-center text-white text-xs font-bold">
            {{ africantive.user.prenom.charAt(0) }}{{ africantive.user.nom.charAt(0) }}
          </div>
          <span class="text-xs text-af-corps">{{ africantive.user.prenom }} {{ africantive.user.nom }}</span>
        </div>
        <div class="flex items-center text-xs text-af-atone-2">
          <font-awesome-icon :icon="['fas', 'calendar-days']" class="w-3 h-3 mr-1.5" />
          {{ dateFormatee }}
        </div>
      </div>
    </div>
  </NuxtLink>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { formatDateCourte, type AfricantiveAPI } from '~/composables/useAfricantives'

const props = defineProps<{
  africantive: AfricantiveAPI
}>()

const dateFormatee = computed(() => {
  return formatDateCourte(props.africantive.created_at)
})
</script>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
