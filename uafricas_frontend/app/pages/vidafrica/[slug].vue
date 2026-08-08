<script setup lang="ts">
import type { VideoAfrica, SousTitres, SegmentKaraoke, LecteurControl } from '~/composables/useVidafrica'
import { formaterDuree, formaterTimestamp, LANGUES_LABELS } from '~/mocks/vidafrica'

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

// Lecteur + synchronisation avec la transcription latérale
const lecteurRef = ref<LecteurControl | null>(null)
const positionActive = ref<number | null>(null)
// État réel de lecture (relayé par le lecteur) — pilote le mode « au fil de la lecture ».
const lectureEnCours = ref(false)
// La vidéo a atteint sa fin (permet de sous-titrer le dernier passage jusqu'au bout).
// Pilotée par l'événement `fin` du lecteur : vrai à la fin, faux dès qu'on relit
// ou qu'on déplace la tête ailleurs.
const videoTerminee = ref(false)

// Rechargement des cadeaux affichés après qu'un cadeau vient d'être offert :
// sans lui, l'offreur ne verrait le sien qu'au prochain chargement de page.
const cadeauxRef = ref<{ rafraichir: () => void } | null>(null)

const segments = computed<SegmentKaraoke[]>(() => {
  return sousTitres.value?.segments || []
})

// Clic sur un sous-titre enregistré → déplace la lecture
const allerAuSegment = (seg: SegmentKaraoke) => {
  lecteurRef.value?.seek(seg.debut_ms)
}

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
    const premiereLangue = video.value.languesDisponibles[0]
    if (premiereLangue) {
      langueActive.value = premiereLangue
      sousTitres.value = await chargerSousTitres(video.value.id, premiereLangue)
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

// Une piste a été modifiée dans l'atelier : rafraîchir le panneau droit (publiées).
const onPisteModifiee = async () => {
  if (!video.value || !langueActive.value) return
  sousTitres.value = await chargerSousTitres(video.value.id, langueActive.value)
}

onMounted(() => charger())
</script>

<template>
  <div class="bg-gray-100 min-h-screen lg:h-screen lg:overflow-hidden lg:flex lg:flex-col">
    <!-- Barre d'outils (dégage la NavBar absolue) -->
    <header class="shrink-0 bg-linear-to-r from-custom-chocolat to-custom-chocolat/85 text-white shadow-md">
      <div class="px-4 pt-20 pb-3 flex items-center justify-between gap-4">
        <div class="min-w-0">
          <NuxtLink to="/vidafrica" class="inline-flex items-center gap-1 text-white/75 hover:text-white text-xs transition-colors">
            <font-awesome-icon icon="arrow-left" /> Catalogue Vidafrica
          </NuxtLink>
          <h1 class="text-lg md:text-2xl font-bold font-['Oswald'] leading-tight truncate">
            {{ video?.titre || 'Vidafrica' }}
          </h1>
        </div>

        <!-- Actions atelier -->
        <div v-if="video" class="flex items-center gap-2 shrink-0">
          <button
            v-if="estConnecte && video.fichierVideoUrl"
            class="px-3 py-1.5 rounded-lg text-xs md:text-sm font-medium transition-colors"
            :class="showContribution
              ? 'bg-white text-custom-chocolat'
              : 'bg-white/15 text-white hover:bg-white/25 ring-1 ring-white/30'"
            @click="showContribution = !showContribution"
          >
            <font-awesome-icon icon="closed-captioning" class="mr-1" />
            <span class="hidden lg:inline">{{ showContribution ? 'Fermer' : 'Connaissez-vous la langue de cette vidéo ? Proposez un sous-titre' }}</span>
            <span class="hidden sm:inline lg:hidden">{{ showContribution ? 'Fermer' : 'Proposer un sous-titre' }}</span>
          </button>
          <button
            v-if="estConnecte"
            class="px-3 py-1.5 rounded-lg text-xs md:text-sm font-medium bg-white/15 text-white hover:bg-white/25 ring-1 ring-white/30 transition-colors"
            @click="showProposer = true"
          >
            <font-awesome-icon icon="plus" class="mr-1" /><span class="hidden sm:inline">Proposer une vidéo</span>
          </button>
        </div>
      </div>
    </header>

    <!-- Chargement -->
    <div v-if="chargement" class="flex-1 flex justify-center items-center py-32">
      <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-custom-chocolat" />
    </div>

    <!-- Erreur -->
    <div v-else-if="erreur" class="flex-1 flex flex-col justify-center items-center px-4 py-32 text-center">
      <p class="text-xl text-gray-500">{{ erreur }}</p>
      <NuxtLink to="/vidafrica" class="mt-4 inline-block text-custom-chocolat hover:underline">
        ← Retour au catalogue
      </NuxtLink>
    </div>

    <!-- ESPACE DE TRAVAIL (3 panneaux) -->
    <div v-else-if="video" class="lg:flex-1 lg:min-h-0 p-3">
      <div class="grid gap-3 lg:grid-cols-[19rem_minmax(0,1fr)_22rem] lg:h-full">
        <!-- ── PANNEAU GAUCHE : options & infos ─────────────────── -->
        <aside class="order-2 lg:order-1 lg:min-h-0 lg:overflow-y-auto flex flex-col gap-3">
          <!-- Langues de sous-titrage -->
          <div class="rounded-xl bg-white shadow-sm p-4">
            <h2 class="flex items-center gap-2 text-sm font-bold text-gray-900 font-['Oswald'] uppercase tracking-wide mb-3">
              <font-awesome-icon icon="language" class="text-custom-chocolat" /> Sous-titres
            </h2>
            <div v-if="video.languesDisponibles.length" class="flex flex-col gap-1.5">
              <button
                v-for="langue in video.languesDisponibles" :key="langue"
                class="flex items-center justify-between px-3 py-2 rounded-lg text-sm font-medium transition-all text-left"
                :class="langue === langueActive
                  ? 'bg-custom-chocolat text-white shadow-sm'
                  : 'bg-gray-50 text-gray-700 hover:bg-gray-100'"
                @click="changerLangue(langue)"
              >
                <span>{{ LANGUES_LABELS[langue] || langue }}</span>
                <font-awesome-icon v-if="langue === langueActive" icon="check" class="text-xs" />
              </button>
            </div>
            <p v-else class="text-sm text-gray-400">Aucun sous-titre disponible pour le moment.</p>
          </div>

          <!-- Réactions -->
          <div class="rounded-xl bg-white shadow-sm p-4">
            <VidafricaReactionsBar :video="video" :peut-interagir="estConnecte" />
            <div class="mt-3">
              <EngagementOffrirCadeauBouton
                type-objet="video"
                :objet-id="video.id"
                :destinataire="video.titre"
                taille="sm"
                @offert="cadeauxRef?.rafraichir()"
              />
            </div>
          </div>

          <!-- Cadeaux reçus par la vidéo -->
          <div class="rounded-xl bg-white shadow-sm p-4">
            <EngagementCadeauxRecus ref="cadeauxRef" type-objet="video" :objet-id="video.id" />
          </div>

          <!-- Infos vidéo -->
          <div class="rounded-xl bg-white shadow-sm p-4">
            <h2 class="flex items-center gap-2 text-sm font-bold text-gray-900 font-['Oswald'] uppercase tracking-wide mb-3">
              <font-awesome-icon icon="circle-info" class="text-custom-chocolat" /> Informations
            </h2>

            <div class="flex items-center gap-4 text-xs text-gray-500 mb-3">
              <span v-if="video.dureeSecondes">
                <font-awesome-icon icon="clock" class="mr-1" />{{ formaterDuree(video.dureeSecondes) }}
              </span>
              <span v-if="video.languesDisponibles.length">
                <font-awesome-icon icon="language" class="mr-1" />{{ video.languesDisponibles.length }} langue{{ video.languesDisponibles.length > 1 ? 's' : '' }}
              </span>
            </div>

            <p v-if="video.description" class="text-sm text-gray-700 leading-relaxed mb-3">
              {{ video.description }}
            </p>

            <dl class="space-y-3 text-sm border-t border-gray-100 pt-3">
              <div v-if="video.territoires.length">
                <dt class="flex items-center gap-1.5 text-gray-500 font-medium mb-1">
                  <font-awesome-icon icon="location-dot" class="text-gray-400" /> Territoires
                </dt>
                <dd class="flex flex-wrap gap-1.5">
                  <span
                    v-for="t in video.territoires" :key="t"
                    class="inline-block px-2 py-0.5 rounded-full bg-custom-chocolat/10 text-custom-chocolat text-xs font-medium"
                  >{{ t }}</span>
                </dd>
              </div>

              <div v-if="video.langueOriginale">
                <dt class="flex items-center gap-1.5 text-gray-500 font-medium mb-0.5">
                  <font-awesome-icon icon="language" class="text-gray-400" /> Langue de la vidéo
                </dt>
                <dd class="font-medium text-gray-800">{{ video.langueOriginale }}</dd>
              </div>

              <div v-if="video.auteurReel">
                <dt class="flex items-center gap-1.5 text-gray-500 font-medium mb-0.5">
                  <font-awesome-icon icon="user" class="text-gray-400" /> Auteur
                </dt>
                <dd class="font-medium text-gray-800">{{ video.auteurReel }}</dd>
              </div>

              <div>
                <dt class="flex items-center gap-1.5 text-gray-500 font-medium mb-0.5">
                  <font-awesome-icon icon="calendar-days" class="text-gray-400" /> Ajoutée le
                </dt>
                <dd class="text-gray-700">
                  {{ new Date(video.createdAt).toLocaleDateString('fr-FR', { year: 'numeric', month: 'long', day: 'numeric' }) }}
                </dd>
              </div>
            </dl>

            <div class="mt-3 pt-3 border-t border-gray-100 space-y-1">
              <p class="text-xs italic text-gray-400">
                Le contributeur déclare ne pas être l'auteur de cette œuvre et ne revendiquer aucun droit à ce sujet.
              </p>
              <p class="text-xs italic text-gray-400">
                Le sous-titrage réalisé est une courtoisie et n'est aucunement professionnel.
              </p>
            </div>

            <p v-if="!estConnecte" class="mt-3 pt-3 border-t border-gray-100 text-xs text-gray-500">
              <NuxtLink to="/login" class="text-custom-chocolat hover:underline">Connectez-vous</NuxtLink>
              pour proposer une vidéo ou contribuer des sous-titres.
            </p>
          </div>
        </aside>

        <!-- ── CANVAS CENTRAL : lecteur vidéo (pleine taille) ────── -->
        <main class="order-1 lg:order-2 lg:min-h-0 flex flex-col rounded-xl bg-gray-900 shadow-sm overflow-hidden">
          <div class="lg:flex-1 min-h-0 flex items-center justify-center p-3 lg:p-4">
            <VidafricaLecteur
              v-if="video.fichierVideoUrl"
              ref="lecteurRef"
              class="w-full"
              :video-url="video.fichierVideoUrl!"
              :segments="segments"
              @segment-change="positionActive = $event"
              @lecture="lectureEnCours = $event"
              @fin="videoTerminee = $event"
            />
            <p v-else class="text-gray-400 text-sm">Vidéo indisponible.</p>
          </div>

          <!-- Mention auteur -->
          <p v-if="sousTitres?.auteur" class="shrink-0 px-4 pb-3 text-xs text-gray-400 text-center">
            <font-awesome-icon icon="closed-captioning" class="mr-1" />
            Sous-titres<template v-if="langueActive"> en {{ LANGUES_LABELS[langueActive] || langueActive }}</template>
            proposés par <span class="font-medium text-gray-200">{{ sousTitres.auteur }}</span>.
            La plateforme n'est pas responsable de l'exactitude de la traduction.
          </p>
        </main>

        <!-- ── PANNEAU DROIT : sous-titres enregistrés (+ atelier en overlay) ── -->
        <aside class="order-3 lg:min-h-0 relative flex flex-col rounded-xl bg-white shadow-sm overflow-hidden">
          <header class="shrink-0 flex items-center justify-between gap-2 px-4 py-3 border-b border-gray-100">
            <h2 class="flex items-center gap-2 text-sm font-bold text-gray-900 font-['Oswald'] uppercase tracking-wide">
              <font-awesome-icon icon="closed-captioning" class="text-custom-chocolat" /> Sous-titres enregistrés
            </h2>
            <span
              v-if="langueActive"
              class="px-2 py-0.5 rounded-full bg-gray-100 text-gray-600 text-xs font-medium shrink-0"
            >{{ LANGUES_LABELS[langueActive] || langueActive }}</span>
          </header>

          <div class="lg:flex-1 lg:min-h-0 lg:overflow-y-auto p-2">
            <p v-if="!segments.length" class="text-sm text-gray-400 text-center py-8 px-3">
              Aucun sous-titre enregistré dans cette langue.
            </p>
            <ul v-else class="space-y-1">
              <li
                v-for="seg in segments" :key="seg.position"
                class="rounded-lg p-2.5 cursor-pointer transition-colors border"
                :class="seg.position === positionActive
                  ? 'bg-custom-chocolat/10 border-custom-chocolat/40'
                  : 'border-transparent hover:bg-gray-50'"
                @click="allerAuSegment(seg)"
              >
                <div class="flex items-center gap-2 mb-0.5">
                  <span
                    class="inline-flex items-center gap-1 text-xs font-mono"
                    :class="seg.position === positionActive ? 'text-custom-chocolat' : 'text-gray-400'"
                  >
                    <font-awesome-icon icon="play" class="text-[0.6rem]" />
                    {{ formaterTimestamp(seg.debut_ms) }}
                  </span>
                </div>
                <p
                  class="text-sm leading-snug"
                  :class="seg.position === positionActive ? 'text-gray-900 font-medium' : 'text-gray-700'"
                >{{ seg.texte }}</p>
              </li>
            </ul>
          </div>

          <!-- Atelier de sous-titrage : superposé au widget (desktop) / empilé (mobile) -->
          <Transition name="atelier">
            <section
              v-if="estConnecte && showContribution && video.fichierVideoUrl"
              class="static lg:absolute lg:inset-0 lg:z-20 flex flex-col overflow-hidden rounded-xl bg-linear-to-br from-custom-chocolat via-custom-chocolat/90 to-amber-900/90 lg:shadow-2xl"
            >
              <header class="shrink-0 flex items-center justify-between gap-2 px-4 py-2.5 text-white border-b border-white/15">
                <span class="flex items-center gap-2 text-sm font-bold font-['Oswald'] uppercase tracking-wide">
                  <font-awesome-icon icon="closed-captioning" /> Proposer un sous-titre
                </span>
                <button
                  class="w-7 h-7 inline-flex items-center justify-center rounded-lg bg-white/15 hover:bg-white/30 transition-colors"
                  title="Fermer l'atelier"
                  @click="showContribution = false"
                >
                  <font-awesome-icon icon="xmark" />
                </button>
              </header>
              <div class="lg:flex-1 lg:min-h-0 overflow-y-auto p-2">
                <VidafricaContribuerSousTitres
                  :video-id="video.id"
                  :video-url="video.fichierVideoUrl!"
                  :lecteur="lecteurRef"
                  :en-lecture="lectureEnCours"
                  :video-terminee="videoTerminee"
                  @piste-modifiee="onPisteModifiee"
                />
              </div>
            </section>
          </Transition>
        </aside>
      </div>
    </div>

    <!-- Modale : proposer une vidéo -->
    <VidafricaProposerVideoModal v-model="showProposer" />
  </div>
</template>

<style scoped>
/* Apparition / disparition de l'atelier docké */
.atelier-enter-active,
.atelier-leave-active {
  transition: opacity 0.25s ease, transform 0.25s ease;
}
.atelier-enter-from,
.atelier-leave-to {
  opacity: 0;
  transform: translateY(16px);
}
</style>
