<script setup lang="ts">
import {
  useFormations,
  type FormationContenu,
  type ChapitrePublic,
  type LeconPublic,
} from '~/composables/useFormations'
import { youtubeEmbedUrl } from '~/composables/useEvenements'

const props = defineProps<{
  formationId: string
  /** Incrémenté par le parent après une inscription pour rafraîchir le contenu */
  refreshToken?: number
}>()

const emit = defineEmits<{
  (e: 'require-inscription'): void
}>()

const { obtenirContenuFormation, marquerLeconTerminee, annulerLeconTerminee } = useFormations()

const contenu = ref<FormationContenu | null>(null)
const chargement = ref(true)
const leconOuverte = ref<string | null>(null)
const leconOccupee = ref<string | null>(null)

const charger = async () => {
  chargement.value = true
  contenu.value = await obtenirContenuFormation(props.formationId)
  chargement.value = false
}

onMounted(charger)
watch(() => props.refreshToken, () => { charger() })

const aDuContenu = computed(() => (contenu.value?.chapitres.length ?? 0) > 0)
const accessible = computed(() => contenu.value?.accessible ?? false)

const formatDuree = (min: number | null): string => {
  if (!min || min <= 0) return ''
  if (min < 60) return `${min} min`
  const h = Math.floor(min / 60)
  const m = min % 60
  return m === 0 ? `${h} h` : `${h} h ${String(m).padStart(2, '0')}`
}

const dureeChapitre = (ch: ChapitrePublic): string => {
  const total = ch.lecons.reduce((acc, l) => acc + (l.duree_minutes || 0), 0)
  return formatDuree(total)
}

const basculerLecon = (lecon: LeconPublic) => {
  if (!lecon.accessible) {
    emit('require-inscription')
    return
  }
  leconOuverte.value = leconOuverte.value === lecon.id ? null : lecon.id
}

const basculerTerminee = async (lecon: LeconPublic) => {
  // Le suivi de progression suppose une inscription (le créateur en aperçu n'en a pas).
  if (!lecon.accessible || !contenu.value?.est_inscrit || leconOccupee.value) return
  leconOccupee.value = lecon.id
  const resultat = lecon.terminee
    ? await annulerLeconTerminee(props.formationId, lecon.id)
    : await marquerLeconTerminee(props.formationId, lecon.id)
  if (resultat && contenu.value) {
    lecon.terminee = resultat.terminee
    contenu.value.progression = resultat.progression
    contenu.value.nombre_lecons = resultat.nombre_lecons
    contenu.value.nombre_lecons_terminees = resultat.nombre_lecons_terminees
  }
  leconOccupee.value = null
}
</script>

<template>
  <div class="bg-white rounded-lg shadow-md p-6">
    <div class="flex items-center justify-between mb-4 gap-3">
      <h2 class="text-2xl font-bold">Programme de la formation</h2>
      <span v-if="aDuContenu" class="text-sm text-gray-500 whitespace-nowrap">
        {{ contenu?.chapitres.length }} chapitre(s) · {{ contenu?.nombre_lecons }} leçon(s)
      </span>
    </div>

    <!-- Chargement -->
    <div v-if="chargement" class="space-y-3">
      <div v-for="i in 3" :key="i" class="h-12 bg-gray-100 rounded-lg animate-pulse"></div>
    </div>

    <!-- Vide -->
    <p v-else-if="!aDuContenu" class="text-gray-500 text-sm py-4">
      Le programme détaillé de cette formation sera bientôt disponible.
    </p>

    <template v-else>
      <!-- Barre de progression (inscrit) -->
      <div v-if="contenu?.est_inscrit" class="mb-5">
        <div class="flex items-center justify-between text-sm mb-1">
          <span class="font-medium text-gray-700">Ma progression</span>
          <span class="text-gray-500">
            {{ contenu.nombre_lecons_terminees }}/{{ contenu.nombre_lecons }} leçons · {{ Math.round(contenu.progression) }}%
          </span>
        </div>
        <div class="w-full h-2.5 bg-gray-200 rounded-full overflow-hidden">
          <div class="h-full bg-custom-green rounded-full transition-all duration-500"
               :style="{ width: `${contenu.progression}%` }"></div>
        </div>
      </div>

      <!-- Accès verrouillé (visiteur non inscrit, non créateur) -->
      <div v-else-if="!accessible"
           class="mb-5 flex items-start gap-3 rounded-lg bg-amber-50 border border-amber-200 px-4 py-3">
        <svg class="w-5 h-5 text-amber-500 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
          <path fill-rule="evenodd" d="M5 9V7a5 5 0 0110 0v2a2 2 0 012 2v5a2 2 0 01-2 2H5a2 2 0 01-2-2v-5a2 2 0 012-2zm8-2v2H7V7a3 3 0 016 0z" clip-rule="evenodd"></path>
        </svg>
        <div class="text-sm text-amber-800">
          <p class="font-medium">Contenu réservé aux inscrits</p>
          <p>Inscrivez-vous gratuitement pour accéder aux vidéos, supports et ressources de chaque leçon.</p>
          <button class="mt-2 text-amber-900 font-semibold underline underline-offset-2 hover:text-amber-700"
                  @click="emit('require-inscription')">
            S'inscrire pour accéder →
          </button>
        </div>
      </div>

      <!-- Chapitres -->
      <div class="space-y-4">
        <div v-for="(chapitre, idx) in contenu!.chapitres" :key="chapitre.id"
             class="border border-gray-200 rounded-lg overflow-hidden">
          <!-- En-tête de chapitre -->
          <div class="bg-gray-50 px-4 py-3 border-b border-gray-200">
            <div class="flex items-center justify-between gap-3">
              <h3 class="font-semibold text-gray-800">
                <span class="text-custom-green">Chapitre {{ idx + 1 }}.</span> {{ chapitre.titre }}
              </h3>
              <span class="text-xs text-gray-500 whitespace-nowrap">
                {{ chapitre.lecons.length }} leçon(s)<template v-if="dureeChapitre(chapitre)"> · {{ dureeChapitre(chapitre) }}</template>
              </span>
            </div>
            <p v-if="chapitre.description" class="text-sm text-gray-500 mt-1 whitespace-pre-line">
              {{ chapitre.description }}
            </p>
          </div>

          <!-- Leçons -->
          <ul class="divide-y divide-gray-100">
            <li v-for="(lecon, li) in chapitre.lecons" :key="lecon.id">
              <!-- Ligne de leçon -->
              <button type="button"
                      class="w-full flex items-center gap-3 px-4 py-3 text-left transition hover:bg-gray-50"
                      :class="{ 'cursor-default': !lecon.accessible }"
                      @click="basculerLecon(lecon)">
                <!-- Indicateur -->
                <span class="flex-shrink-0 w-7 h-7 rounded-full flex items-center justify-center text-xs font-semibold"
                      :class="lecon.terminee
                        ? 'bg-custom-green text-white'
                        : (lecon.accessible ? 'bg-blue-100 text-blue-700' : 'bg-gray-100 text-gray-400')">
                  <svg v-if="lecon.terminee" class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"></path>
                  </svg>
                  <svg v-else-if="!lecon.accessible" class="w-3.5 h-3.5" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M5 9V7a5 5 0 0110 0v2a2 2 0 012 2v5a2 2 0 01-2 2H5a2 2 0 01-2-2v-5a2 2 0 012-2zm8-2v2H7V7a3 3 0 016 0z" clip-rule="evenodd"></path>
                  </svg>
                  <span v-else>{{ li + 1 }}</span>
                </span>

                <!-- Titre + méta -->
                <span class="flex-1 min-w-0">
                  <span class="block font-medium text-gray-800 truncate">{{ lecon.titre }}</span>
                  <span class="flex items-center gap-2 text-xs text-gray-400 mt-0.5">
                    <span v-if="lecon.a_video" class="inline-flex items-center gap-1">
                      <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20"><path d="M2 6a2 2 0 012-2h6a2 2 0 012 2v8a2 2 0 01-2 2H4a2 2 0 01-2-2V6zM14.553 7.106A1 1 0 0014 8v4a1 1 0 00.553.894l2 1A1 1 0 0018 13V7a1 1 0 00-1.447-.894l-2 1z"/></svg>
                      Vidéo
                    </span>
                    <span v-if="lecon.a_document" class="inline-flex items-center gap-1">
                      <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M4 4a2 2 0 012-2h4.586A2 2 0 0112 2.586L15.414 6A2 2 0 0116 7.414V16a2 2 0 01-2 2H6a2 2 0 01-2-2V4z" clip-rule="evenodd"/></svg>
                      Document
                    </span>
                    <span v-if="formatDuree(lecon.duree_minutes)">{{ formatDuree(lecon.duree_minutes) }}</span>
                  </span>
                </span>

                <!-- Chevron (accessible) ou cadenas -->
                <svg v-if="lecon.accessible" class="w-4 h-4 text-gray-400 flex-shrink-0 transition-transform"
                     :class="{ 'rotate-180': leconOuverte === lecon.id }"
                     fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                </svg>
              </button>

              <!-- Contenu déplié (accessible uniquement) -->
              <div v-if="lecon.accessible && leconOuverte === lecon.id" class="px-4 pb-4 pt-1 bg-gray-50/60 space-y-4">
                <!-- Vidéo YouTube -->
                <div v-if="lecon.video_url && youtubeEmbedUrl(lecon.video_url)"
                     class="aspect-video w-full rounded-lg overflow-hidden bg-black">
                  <iframe class="w-full h-full" :src="youtubeEmbedUrl(lecon.video_url)!"
                          title="Vidéo de la leçon" frameborder="0"
                          allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                          allowfullscreen></iframe>
                </div>
                <!-- Vidéo fichier / lien direct -->
                <video v-else-if="lecon.video_url" :src="lecon.video_url" controls preload="metadata"
                       class="w-full max-h-[28rem] rounded-lg bg-black"></video>

                <!-- Contenu texte -->
                <p v-if="lecon.contenu" class="text-gray-700 leading-relaxed whitespace-pre-line text-sm">
                  {{ lecon.contenu }}
                </p>

                <!-- Document -->
                <a v-if="lecon.document_url" :href="lecon.document_url" target="_blank" rel="noopener noreferrer"
                   class="inline-flex items-center gap-2 text-sm font-medium text-blue-600 hover:underline">
                  <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z" clip-rule="evenodd"/></svg>
                  Télécharger le document
                </a>

                <!-- Marquer terminée (inscrits uniquement ; le créateur n'a qu'un aperçu) -->
                <div v-if="contenu?.est_inscrit" class="pt-1">
                  <button type="button"
                          class="inline-flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium transition disabled:opacity-60"
                          :class="lecon.terminee
                            ? 'bg-green-50 text-custom-green border border-custom-green/40 hover:bg-green-100'
                            : 'bg-custom-green text-white hover:bg-green-700'"
                          :disabled="leconOccupee === lecon.id"
                          @click="basculerTerminee(lecon)">
                    <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"></path>
                    </svg>
                    {{ lecon.terminee ? 'Leçon terminée' : 'Marquer comme terminée' }}
                  </button>
                </div>
              </div>
            </li>
          </ul>
        </div>
      </div>
    </template>
  </div>
</template>
