<template>
  <div class="relative bg-white rounded-2xl overflow-hidden shadow-lg hover:shadow-2xl transition-all duration-300 group">
    <!-- Header avec gradient -->
    <div class="relative h-32 bg-gradient-to-r from-blue-500 via-cyan-500 to-teal-500 p-4">
      <!-- Badge protection -->
      <div class="absolute top-4 right-4">
        <span
          v-if="sallePrivee.est_protegee"
          class="bg-amber-100 text-amber-700 px-3 py-1 rounded-full text-xs font-medium flex items-center gap-1.5"
        >
          <font-awesome-icon :icon="['fas', 'lock']" class="w-3 h-3" />
          Protégée
        </span>
        <span
          v-else
          class="bg-green-100 text-green-700 px-3 py-1 rounded-full text-xs font-medium flex items-center gap-1.5"
        >
          <font-awesome-icon :icon="['fas', 'door-open']" class="w-3 h-3" />
          Ouverte
        </span>
      </div>

      <!-- Indicateur en direct -->
      <div v-if="sallePrivee.session_en_cours" class="absolute top-4 left-4">
        <span class="bg-red-500 text-white px-3 py-1 rounded-full text-xs font-medium flex items-center gap-1.5 animate-pulse">
          <font-awesome-icon :icon="['fas', 'circle']" class="w-2 h-2" />
          En direct
        </span>
      </div>

      <!-- Titre -->
      <div class="absolute bottom-0 left-0 right-0 p-4 text-white">
        <h3 class="text-lg font-bold line-clamp-1">{{ sallePrivee.titre }}</h3>
        <p v-if="sallePrivee.salle_langue" class="text-white/80 text-sm flex items-center gap-1.5">
          <font-awesome-icon :icon="['fas', 'language']" class="w-3 h-3" />
          {{ sallePrivee.salle_langue }}
        </p>
      </div>
    </div>

    <!-- Contenu -->
    <div class="p-5">
      <!-- Description -->
      <p class="text-gray-600 text-sm line-clamp-2 mb-4">
        {{ sallePrivee.description || 'Aucune description' }}
      </p>

      <!-- Infos -->
      <div class="space-y-2 mb-4">
        <div v-if="sallePrivee.max_participants" class="flex items-center gap-2 text-sm text-gray-600">
          <font-awesome-icon :icon="['fas', 'users']" class="w-4 h-4 text-gray-400" />
          <span>Max {{ sallePrivee.max_participants }} participants</span>
        </div>
      </div>

      <!-- Footer -->
      <div class="flex items-center justify-between pt-4 border-t border-gray-100">
        <div class="flex items-center gap-2">
          <div
            v-if="sallePrivee.createur.photo_url"
            class="w-8 h-8 rounded-full overflow-hidden"
          >
            <img
              :src="sallePrivee.createur.photo_url"
              :alt="sallePrivee.createur.prenom || ''"
              class="w-full h-full object-cover"
            />
          </div>
          <div
            v-else
            class="w-8 h-8 rounded-full bg-blue-500 text-white flex items-center justify-center text-xs font-semibold"
          >
            {{ initiales }}
          </div>
          <span class="text-xs text-gray-500">
            {{ sallePrivee.createur.prenom }} {{ sallePrivee.createur.nom }}
          </span>
        </div>
        <NuxtLink
          :to="`/afrolang/salle-privee/${sallePrivee.id}`"
          class="px-4 py-2 bg-gradient-to-r from-blue-500 to-cyan-500 text-white text-sm rounded-lg font-medium hover:shadow-lg transform hover:scale-[1.02] transition-all"
          @click.stop
        >
          Voir les sessions
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { getInitiales, type SallePriveeAPI } from '~/composables/useAfrolang'

const props = defineProps<{
  sallePrivee: SallePriveeAPI
}>()

const initiales = computed(() => {
  return getInitiales(props.sallePrivee.createur.nom, props.sallePrivee.createur.prenom)
})
</script>

<style scoped>
.line-clamp-1 {
  display: -webkit-box;
  -webkit-line-clamp: 1;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
