<script setup lang="ts">
import type { VideoAfrica, SousTitres, SegmentKaraoke } from '~/composables/useVidafrica'
import { formaterDuree } from '~/mocks/vidafrica'

const route = useRoute()
const slug = route.params.slug as string

const { chargerVideo, chargerSousTitres } = useVidafrica()

const video = ref<VideoAfrica | null>(null)
const sousTitres = ref<SousTitres | null>(null)
const langueActive = ref('')
const chargement = ref(true)
const erreur = ref('')

const segments = computed<SegmentKaraoke[]>(() => {
  return sousTitres.value?.segments || []
})

const charger = async () => {
  chargement.value = true
  erreur.value = ''
  try {
    video.value = await chargerVideo(slug)
    if (!video.value) {
      erreur.value = 'Vidéo non trouvée'
      return
    }
    // Charger les sous-titres dans la première langue disponible
    if (video.value.languesDisponibles.length > 0) {
      langueActive.value = video.value.languesDisponibles[0]
      sousTitres.value = await chargerSousTitres(video.value.id, langueActive.value)
    }
  } catch {
    erreur.value = 'Erreur lors du chargement de la vidéo'
  } finally {
    chargement.value = false
  }
}

const changerLangue = async (langue: string) => {
  if (!video.value || langue === langueActive.value) return
  langueActive.value = langue
  sousTitres.value = await chargerSousTitres(video.value.id, langue)
}

onMounted(() => charger())
</script>

<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Chargement -->
    <div v-if="chargement" class="flex justify-center items-center py-32">
      <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-custom-chocolat" />
    </div>

    <!-- Erreur -->
    <div v-else-if="erreur" class="max-w-2xl mx-auto px-4 py-32 text-center">
      <p class="text-xl text-gray-500">{{ erreur }}</p>
      <NuxtLink to="/vidafrica" class="mt-4 inline-block text-custom-chocolat hover:underline">
        ← Retour au catalogue
      </NuxtLink>
    </div>

    <!-- Contenu -->
    <div v-else-if="video" class="max-w-5xl mx-auto px-4 py-8">
      <!-- Lecteur vidéo avec karaoké -->
      <div v-if="video.fichierVideoUrl" class="mb-6">
        <VidafricaLecteur
          :video-url="video.fichierVideoUrl!"
          :segments="segments"
        />
      </div>

      <!-- Sélecteur de langue -->
      <div v-if="video.languesDisponibles.length > 1" class="mb-6">
        <VidafricaSelecteurLangue
          :langues="video.languesDisponibles"
          :langue-active="langueActive"
          @changer-langue="changerLangue"
        />
      </div>

      <!-- Infos vidéo -->
      <div class="bg-white rounded-xl p-6 shadow-sm">
        <h1 class="text-2xl font-bold text-gray-900 font-['Oswald']">{{ video.titre }}</h1>

        <div class="flex items-center gap-4 mt-2 text-sm text-gray-500">
          <span v-if="video.dureeSecondes">
            <font-awesome-icon icon="clock" class="mr-1" />
            {{ formaterDuree(video.dureeSecondes) }}
          </span>
          <span v-if="video.languesDisponibles.length > 0">
            <font-awesome-icon icon="language" class="mr-1" />
            {{ video.languesDisponibles.length }} langue{{ video.languesDisponibles.length > 1 ? 's' : '' }}
          </span>
          <span>
            {{ new Date(video.createdAt).toLocaleDateString('fr-FR', { year: 'numeric', month: 'long', day: 'numeric' }) }}
          </span>
        </div>

        <p v-if="video.description" class="mt-4 text-gray-700 leading-relaxed">
          {{ video.description }}
        </p>
      </div>

      <!-- Retour -->
      <div class="mt-6">
        <NuxtLink to="/vidafrica" class="text-custom-chocolat hover:underline text-sm">
          ← Retour au catalogue Vidafrica
        </NuxtLink>
      </div>
    </div>
  </div>
</template>
