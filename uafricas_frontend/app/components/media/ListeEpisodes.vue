<script setup lang="ts">
/**
 * Épisodes d'un programme, **paginés** (feature 009, US1, SC-009).
 *
 * La pagination n'est pas un confort : c'est ce qui rend un programme de
 * 500 épisodes navigable. Charger la série entière d'un bloc ferait exactement
 * ce que la refonte cherche à éviter.
 *
 * Tailwind v4 pur : page publique, aucune classe daisyUI (principe VI).
 */
import type { EpisodeAPI, TypeSupportMedia } from '~/composables/useMediaEmissions'

const props = withDefaults(defineProps<{
  typeSupport: TypeSupportMedia
  emissionId: string
  /** Épisodes déjà chargés par la page : évite un aller-retour au montage. */
  initiaux?: EpisodeAPI[]
  total?: number
  taille?: number
}>(), {
  initiaux: () => [],
  total: 0,
  taille: 24,
})

const { listerEpisodes, lienEpisode, chargement } = useMediaEmissions()

const episodes = ref<EpisodeAPI[]>([...props.initiaux])
const page = ref(1)
const total = ref(props.total)

/** `null` tant que rien n'a été chargé : l'appelant peut ne pas connaître le total. */
const resteAcharger = computed(() => episodes.value.length < total.value)

const chargerPage = async (numero: number) => {
  const res = await listerEpisodes(props.typeSupport, props.emissionId, numero, props.taille)
  if (!res) return
  total.value = res.pagination.total
  // Chargement progressif plutôt que remplacement : le visiteur ne perd pas sa
  // position dans la liste en demandant la suite.
  episodes.value = numero === 1 ? res.episodes : [...episodes.value, ...res.episodes]
  page.value = numero
}

onMounted(() => {
  if (!episodes.value.length) chargerPage(1)
})

const dureeLisible = (minutes: number | null) =>
  minutes ? (minutes >= 60 ? `${Math.floor(minutes / 60)} h ${minutes % 60}` : `${minutes} min`) : null
</script>

<template>
  <section>
    <div class="flex items-baseline justify-between gap-4 mb-4">
      <h2 class="font-oswald text-xl font-bold text-af-encre">
        Épisodes
      </h2>
      <span v-if="total" class="text-sm text-af-corps">
        {{ total }} au total
      </span>
    </div>

    <ul v-if="episodes.length" class="divide-y divide-white/10">
      <li v-for="episode in episodes" :key="episode.id">
        <NuxtLink
          :to="lienEpisode(episode)"
          class="group flex gap-4 py-4 hover:bg-af-fond transition-colors rounded-lg px-2 -mx-2"
        >
          <div class="w-32 sm:w-44 shrink-0 aspect-video rounded-lg overflow-hidden bg-af-fond">
            <img
              v-if="episode.image_couverture_url"
              :src="episode.image_couverture_url"
              :alt="episode.titre"
              loading="lazy"
              class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300"
            >
            <span v-else class="w-full h-full flex items-center justify-center">
              <font-awesome-icon :icon="['fas', 'play']" class="text-af-atone-2" />
            </span>
          </div>

          <div class="min-w-0 flex-1">
            <p class="text-af-encre font-semibold group-hover:text-af-chocolat transition-colors">
              <span v-if="episode.numero_episode" class="text-af-atone font-normal">
                {{ episode.numero_episode }}.
              </span>
              {{ episode.titre }}
            </p>
            <p v-if="episode.description" class="text-af-corps text-sm mt-1 line-clamp-2">
              {{ episode.description }}
            </p>
            <p class="text-xs text-af-atone mt-1 flex flex-wrap gap-x-3">
              <span v-if="dureeLisible(episode.duree_minutes)">
                {{ dureeLisible(episode.duree_minutes) }}
              </span>
              <span v-if="episode.interactions?.nombre_commentaires">
                {{ episode.interactions.nombre_commentaires }} commentaire(s)
              </span>
            </p>
          </div>
        </NuxtLink>
      </li>
    </ul>

    <p v-else-if="!chargement" class="text-af-atone text-sm">
      Ce programme n'a pas encore d'épisode publié.
    </p>

    <div v-if="resteAcharger" class="mt-6 text-center">
      <button
        type="button"
        :disabled="chargement"
        class="inline-flex items-center gap-2 rounded-full border border-af-bordure text-af-corps px-6 py-2 text-sm hover:border-af-chocolat hover:text-af-chocolat transition-colors disabled:opacity-50"
        @click="chargerPage(page + 1)"
      >
        <font-awesome-icon v-if="chargement" :icon="['fas', 'spinner']" spin />
        Voir plus d'épisodes
      </button>
    </div>
  </section>
</template>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
