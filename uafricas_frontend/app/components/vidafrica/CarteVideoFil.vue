<template>
  <AfricansCartePublication
    :auteur="auteur"
    :titre="video.titre"
    :texte="video.description || undefined"
    :etiquettes="langues"
    :likes="video.nombreLikes"
    :partages="video.nombrePartages"
    :actions="['jaime', 'partager']"
    :quand="`Partagé ${dateRelative}`"
    :jaime="video.maReaction === 'like'"
    @jaime="$emit('jaime')"
    @partager="$emit('partager')"
  >
    <template #media>
      <!-- La vignette MÈNE à la page vidéo, elle ne lit pas sur place : la
           lecture y est solidaire du sélecteur de langue et du surlignage des
           sous-titres, qu'un lecteur posé dans le fil n'aurait pas. -->
      <NuxtLink :to="`/vidafrica/${video.slug}`" class="group relative block aspect-video overflow-hidden bg-af-bordure">
        <img
          v-if="video.vignetteUrl"
          :src="video.vignetteUrl"
          :alt="''"
          class="size-full object-cover transition duration-300 group-hover:scale-105"
        />
        <span v-else class="grid size-full place-items-center">
          <font-awesome-icon icon="fa-solid fa-video" class="text-4xl text-af-atone-2" />
        </span>

        <span class="absolute inset-0 grid place-items-center">
          <span class="grid size-14 place-items-center rounded-full bg-black/50 text-white transition group-hover:bg-af-chocolat">
            <font-awesome-icon icon="fa-solid fa-play" class="ml-0.5 text-lg" />
          </span>
        </span>

        <span
          v-if="video.dureeSecondes"
          class="absolute right-3 bottom-3 rounded bg-black/75 px-2 py-1 text-[12px]/[1.4] text-white"
        >{{ formaterDuree(video.dureeSecondes) }}</span>
      </NuxtLink>
    </template>

    <template #actions>
      <button
        type="button"
        class="flex items-center gap-2 transition hover:text-af-chocolat"
        :class="video.maReaction === 'dislike' && 'text-af-live'"
        :aria-pressed="video.maReaction === 'dislike'"
        @click="$emit('jaime-pas')"
      >
        <font-awesome-icon icon="fa-solid fa-thumbs-down" />
        {{ video.nombreDislikes }}
      </button>
    </template>
  </AfricansCartePublication>
</template>

<script setup lang="ts">
import type { VideoAfrica } from '~/composables/useVidafrica'
import { LANGUES_LABELS, formaterDuree } from '~/mocks/vidafrica'

/**
 * Vidéo Vidafrica dans le fil de la refonte. Vidafrica ne porte AUCUN
 * commentaire : le bouton est retiré de la barre plutôt que laissé à zéro,
 * un bouton qui ne mène nulle part valant moins que pas de bouton.
 */
const props = defineProps<{ video: VideoAfrica }>()

defineEmits<{ jaime: [], 'jaime-pas': [], partager: [] }>()

/** Les langues de sous-titrage sont l'information distinctive de Vidafrica :
 *  elles montent en étiquettes, comme sur la maquette. */
const langues = computed(() =>
  props.video.languesDisponibles.map(l => LANGUES_LABELS[l] || l),
)

/** `auteurReel` est un texte libre déclaré à la proposition, pas un compte :
 *  aucun lien de profil, et rien du tout quand il n'est pas renseigné. */
const auteur = computed(() =>
  props.video.auteurReel ? { nom: props.video.auteurReel } : undefined,
)

const dateRelative = computed(() => {
  const ms = Date.now() - new Date(props.video.createdAt).getTime()
  const heures = Math.floor(ms / 3_600_000)
  if (heures < 1) return "à l'instant"
  if (heures < 24) return `il y a ${heures} h`
  const jours = Math.floor(heures / 24)
  if (jours < 31) return `il y a ${jours} j`
  return new Date(props.video.createdAt).toLocaleDateString('fr-FR')
})
</script>
