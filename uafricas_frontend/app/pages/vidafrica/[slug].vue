<script setup lang="ts">
import type { VideoAfrica, SousTitres, SegmentKaraoke } from '~/composables/useVidafrica'
import { formaterDuree } from '~/mocks/vidafrica'

const route = useRoute()
const slug = route.params.slug as string

const { chargerVideo, chargerSousTitres } = useVidafrica()
const userStore = useUserStore()
const estConnecte = computed(() => userStore.isAuthenticated)

const video = ref<VideoAfrica | null>(null)
const sousTitres = ref<SousTitres | null>(null)
const langueActive = ref('')
const chargement = ref(true)
const erreur = ref('')

// Contribution membre
const showProposer = ref(false)
const showContribution = ref(false)

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
    <!-- Hero compact (dégage la NavBar fixe + fil d'Ariane) -->
    <section class="bg-gradient-to-r from-custom-chocolat to-custom-chocolat/80 text-white">
      <div class="max-w-5xl mx-auto px-4 pt-20 pb-6">
        <NuxtLink to="/vidafrica" class="inline-flex items-center gap-1 text-white/80 hover:text-white text-sm transition-colors mb-2">
          <font-awesome-icon icon="arrow-left" /> Catalogue Vidafrica
        </NuxtLink>
        <h1 class="text-2xl md:text-3xl font-bold font-['Oswald'] leading-tight">
          {{ video?.titre || 'Vidafrica' }}
        </h1>
      </div>
    </section>

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
        <div class="flex items-center gap-4 text-sm text-gray-500">
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

        <!-- Actions contributeur (utilisateur connecté) -->
        <div v-if="estConnecte" class="mt-5 pt-4 border-t border-gray-100 flex flex-wrap gap-2">
          <button
            class="px-4 py-2 rounded-lg text-sm font-medium border border-custom-chocolat text-custom-chocolat hover:bg-custom-chocolat/5 transition-colors"
            @click="showContribution = !showContribution"
          >
            <font-awesome-icon icon="closed-captioning" class="mr-1" />
            {{ showContribution ? 'Masquer le sous-titrage' : 'Contribuer des sous-titres' }}
          </button>
          <button
            class="px-4 py-2 rounded-lg text-sm font-medium bg-custom-chocolat text-white hover:bg-custom-chocolat/90 transition-colors"
            @click="showProposer = true"
          >
            <font-awesome-icon icon="plus" class="mr-1" /> Proposer une vidéo
          </button>
        </div>
        <p v-else class="mt-5 pt-4 border-t border-gray-100 text-sm text-gray-500">
          <NuxtLink to="/login" class="text-custom-chocolat hover:underline">Connectez-vous</NuxtLink>
          pour proposer une vidéo ou contribuer des sous-titres.
        </p>
      </div>

      <!-- Atelier de sous-titrage -->
      <div v-if="estConnecte && showContribution && video.fichierVideoUrl" class="mt-6">
        <VidafricaContribuerSousTitres
          :video-id="video.id"
          :video-url="video.fichierVideoUrl!"
        />
      </div>

      <!-- Retour -->
      <div class="mt-6">
        <NuxtLink to="/vidafrica" class="text-custom-chocolat hover:underline text-sm">
          ← Retour au catalogue Vidafrica
        </NuxtLink>
      </div>
    </div>

    <!-- Modale : proposer une vidéo -->
    <VidafricaProposerVideoModal v-model="showProposer" />
  </div>
</template>
