<script setup lang="ts">
import { LANGUES_LABELS, formaterTimestamp } from '~/mocks/vidafrica'
import type { PisteSousTitre, SegmentSousTitre, TimingMot } from '~/mocks/vidafrica'
import type { LecteurControl } from '~/composables/useVidafrica'

const props = defineProps<{
  videoId: string
  videoUrl: string
  // Poignée du lecteur unique (pour le mode « au fil de la lecture »).
  lecteur?: LecteurControl | null
  // État réel de lecture relayé par la page.
  enLecture?: boolean
  // La vidéo a atteint sa fin (pour sous-titrer le tout dernier passage).
  videoTerminee?: boolean
}>()

const emit = defineEmits<{
  'piste-modifiee': []
}>()

// Stratégie de saisie : « direct » (au fil de la lecture) ou « manuel » (timecodes).
const modeSaisie = ref<'direct' | 'manuel'>('direct')

const {
  mesPistes, creerPiste, supprimerPiste,
  chargerSegments, creerSegment, modifierSegment, supprimerSegment,
  enregistrerTimingsMot, supprimerTimingsMot,
} = useVidafricaContribution()

// ── État ────────────────────────────────────────────────────
const pistes = ref<PisteSousTitre[]>([])
const languesPrises = ref<string[]>([])
const pisteSelectionnee = ref<PisteSousTitre | null>(null)
const segments = ref<SegmentSousTitre[]>([])
const nouvelleLangue = ref('')
const erreur = ref('')
const chargement = ref(false)
const pisteLoading = ref(false)
const segmentLoading = ref(false)

// Ajout / édition de segment
const showAjoutSegment = ref(false)
const nouveauSegment = reactive({ texte: '', debut_ms: 0, fin_ms: 0 })
const segmentEdite = ref<string | null>(null)
const segmentEditeData = reactive({ texte: '', debut_ms: 0, fin_ms: 0 })

// Tap-to-mark
const showTapToMark = ref(false)
const tapToMarkSegment = ref<SegmentSousTitre | null>(null)

// ── Dérivés ─────────────────────────────────────────────────
const languesDisponibles = computed(() =>
  Object.entries(LANGUES_LABELS)
    .filter(([code]) => !languesPrises.value.includes(code))
    .map(([code, label]) => ({ code, label })),
)

// Repère de reprise du mode direct = fin du dernier segment existant (contiguïté).
// Dérivé des segments ⇒ toujours à jour après une capture directe OU une édition manuelle.
const curseurDepart = computed(() =>
  segments.value.reduce((m, s) => Math.max(m, s.fin_ms), 0),
)

// L'auteur peut éditer sa piste en brouillon ET après publication (modif en direct) ;
// seule une piste masquée par un admin est gelée.
const pisteEditable = computed(() => !!pisteSelectionnee.value && pisteSelectionnee.value.etat !== 'masque')

const labelEtat = (etat?: string): string => {
  if (etat === 'publie') return 'Publiée'
  if (etat === 'masque') return 'Masquée'
  return 'En attente de validation'
}

// ── Chargement ──────────────────────────────────────────────
const charger = async () => {
  chargement.value = true
  erreur.value = ''
  try {
    const res = await mesPistes(props.videoId)
    pistes.value = res.pistes
    languesPrises.value = res.langues_prises
    if (pisteSelectionnee.value) {
      const maj = pistes.value.find(p => p.id === pisteSelectionnee.value!.id)
      pisteSelectionnee.value = maj || null
      if (!maj) segments.value = []
    }
  } catch (e: any) {
    erreur.value = e?.data?.error || e?.message || 'Erreur de chargement'
  } finally {
    chargement.value = false
  }
}

const selectionnerPiste = async (piste: PisteSousTitre) => {
  pisteSelectionnee.value = piste
  segmentEdite.value = null
  showAjoutSegment.value = false
  // Vider avant chargement : curseurDepart (computed) retombe à 0 le temps du
  // fetch, si bien que la bande directe (keyée par piste, remontée) part d'un
  // repère neuf et converge vers le max réel de LA piste choisie (jamais celui,
  // périmé, de la piste précédente).
  segments.value = []
  segmentLoading.value = true
  try {
    segments.value = await chargerSegments(piste.id)
  } catch (e: any) {
    erreur.value = e?.data?.error || e?.message || 'Erreur'
  } finally {
    segmentLoading.value = false
  }
}

// Segment créé par le mode direct : rafraîchir la liste (curseurDepart suit via computed).
const onSegmentDirectCree = async () => {
  if (!pisteSelectionnee.value) return
  try {
    segments.value = await chargerSegments(pisteSelectionnee.value.id)
    await charger()
    emit('piste-modifiee')
  } catch (e: any) {
    erreur.value = e?.data?.error || e?.message || 'Erreur de rafraîchissement'
  }
}

// ── Pistes ──────────────────────────────────────────────────
const ajouterPiste = async () => {
  if (!nouvelleLangue.value) return
  pisteLoading.value = true
  erreur.value = ''
  try {
    await creerPiste(props.videoId, nouvelleLangue.value)
    nouvelleLangue.value = ''
    await charger()
    emit('piste-modifiee')
  } catch (e: any) {
    erreur.value = e?.data?.error || e?.message || 'Erreur'
  } finally {
    pisteLoading.value = false
  }
}

const supprimerPisteAction = async (pisteId: string) => {
  if (!confirm('Supprimer cette piste de sous-titres ?')) return
  try {
    await supprimerPiste(pisteId)
    if (pisteSelectionnee.value?.id === pisteId) {
      pisteSelectionnee.value = null
      segments.value = []
    }
    await charger()
    emit('piste-modifiee')
  } catch (e: any) {
    erreur.value = e?.data?.error || e?.message || 'Erreur'
  }
}

// ── Segments ────────────────────────────────────────────────
const ajouterSegment = async () => {
  if (!pisteSelectionnee.value || !nouveauSegment.texte.trim()) return
  if (nouveauSegment.debut_ms >= nouveauSegment.fin_ms) {
    erreur.value = 'Le début doit être inférieur à la fin'
    return
  }
  if (chevaucheExistant(nouveauSegment.debut_ms, nouveauSegment.fin_ms)) {
    erreur.value = 'Ce segment chevauche un segment existant.'
    return
  }
  segmentLoading.value = true
  erreur.value = ''
  try {
    await creerSegment(pisteSelectionnee.value.id, {
      texte: nouveauSegment.texte.trim(),
      debut_ms: nouveauSegment.debut_ms,
      fin_ms: nouveauSegment.fin_ms,
    })
    nouveauSegment.texte = ''
    nouveauSegment.debut_ms = 0
    nouveauSegment.fin_ms = 0
    showAjoutSegment.value = false
    segments.value = await chargerSegments(pisteSelectionnee.value.id)
    await charger()
  } catch (e: any) {
    erreur.value = e?.data?.error || e?.message || 'Erreur'
  } finally {
    segmentLoading.value = false
  }
}

// Capture le temps courant de la vidéo (en millisecondes) dans un champ, pour
// éviter la saisie manuelle de ms (source d'erreurs d'unité : secondes tapées
// dans un champ ms → sous-titre d'une poignée de ms qui « flashe » et disparaît).
const capturerDansNouveau = (champ: 'debut_ms' | 'fin_ms') => {
  if (props.lecteur) nouveauSegment[champ] = props.lecteur.positionMs()
}
const capturerDansEdition = (champ: 'debut_ms' | 'fin_ms') => {
  if (props.lecteur) segmentEditeData[champ] = props.lecteur.positionMs()
}

// Chevauchement RÉEL avec un autre segment (les bornes qui se touchent sont OK,
// c'est le cas des segments contigus). Retour immédiat avant l'appel serveur.
const chevaucheExistant = (debut: number, fin: number, exclureId?: string) =>
  segments.value.some(s => s.id !== exclureId && debut < s.fin_ms && fin > s.debut_ms)

const debutEdition = (seg: SegmentSousTitre) => {
  segmentEdite.value = seg.id
  segmentEditeData.texte = seg.texte
  segmentEditeData.debut_ms = seg.debut_ms
  segmentEditeData.fin_ms = seg.fin_ms
}

const sauvegarderSegment = async () => {
  if (!segmentEdite.value || !pisteSelectionnee.value) return
  if (segmentEditeData.debut_ms >= segmentEditeData.fin_ms) {
    erreur.value = 'Le début doit être inférieur à la fin'
    return
  }
  if (chevaucheExistant(segmentEditeData.debut_ms, segmentEditeData.fin_ms, segmentEdite.value)) {
    erreur.value = 'Ce segment chevauche un autre segment.'
    return
  }
  erreur.value = ''
  segmentLoading.value = true
  try {
    await modifierSegment(segmentEdite.value, {
      texte: segmentEditeData.texte.trim(),
      debut_ms: segmentEditeData.debut_ms,
      fin_ms: segmentEditeData.fin_ms,
    })
    segmentEdite.value = null
    segments.value = await chargerSegments(pisteSelectionnee.value.id)
  } catch (e: any) {
    erreur.value = e?.data?.error || e?.message || 'Erreur'
  } finally {
    segmentLoading.value = false
  }
}

const supprimerSegmentAction = async (segId: string) => {
  if (!confirm('Supprimer ce segment ?') || !pisteSelectionnee.value) return
  await supprimerSegment(segId)
  segments.value = await chargerSegments(pisteSelectionnee.value.id)
  await charger()
}

// ── Tap-to-mark ─────────────────────────────────────────────
const ouvrirTapToMark = (seg: SegmentSousTitre) => {
  tapToMarkSegment.value = seg
  showTapToMark.value = true
}

const onTimingsEnregistres = async (timings: TimingMot[]) => {
  if (!tapToMarkSegment.value || !pisteSelectionnee.value) return
  await enregistrerTimingsMot(tapToMarkSegment.value.id, timings)
  showTapToMark.value = false
  tapToMarkSegment.value = null
  segments.value = await chargerSegments(pisteSelectionnee.value.id)
  await charger()
}

const supprimerTimingsAction = async (segId: string) => {
  if (!confirm('Supprimer les timings mot de ce segment ?') || !pisteSelectionnee.value) return
  await supprimerTimingsMot(segId)
  segments.value = await chargerSegments(pisteSelectionnee.value.id)
  await charger()
}

onMounted(() => charger())
</script>

<template>
  <div class="bg-white rounded-xl p-6 shadow-sm">
    <h2 class="text-lg font-bold text-gray-900 font-['Oswald'] mb-1">Contribuer des sous-titres</h2>
    <p class="text-sm text-gray-500 mb-3">
      Votre piste sera publiée après validation par un administrateur ; une fois publiée,
      vous pouvez toujours la corriger — vos modifications sont prises en compte immédiatement.
    </p>

    <!-- Stratégie de sous-titrage -->
    <div class="mb-1 inline-flex rounded-lg bg-gray-100 p-1">
      <button
        class="px-3 py-1.5 rounded-md text-sm font-medium transition-colors"
        :class="modeSaisie === 'direct' ? 'bg-white text-custom-chocolat shadow-sm' : 'text-gray-500 hover:text-gray-700'"
        @click="modeSaisie = 'direct'"
      >
        <font-awesome-icon icon="wand-magic-sparkles" class="mr-1" /> Au fil de la lecture
      </button>
      <button
        class="px-3 py-1.5 rounded-md text-sm font-medium transition-colors"
        :class="modeSaisie === 'manuel' ? 'bg-white text-custom-chocolat shadow-sm' : 'text-gray-500 hover:text-gray-700'"
        @click="modeSaisie = 'manuel'"
      >
        <font-awesome-icon icon="stopwatch" class="mr-1" /> Saisie manuelle
      </button>
    </div>
    <p class="text-xs text-gray-400 mb-4">
      {{ modeSaisie === 'direct'
        ? 'Lancez la vidéo, laissez-la jouer puis coupez pour sous-titrer chaque passage — les temps sont capturés automatiquement.'
        : 'Saisissez chaque segment avec ses temps de début/fin, puis marquez le timing mot par mot.' }}
    </p>

    <div v-if="erreur" class="rounded-lg bg-red-50 border border-red-200 px-3 py-2 text-sm text-red-700 mb-4">
      {{ erreur }}
      <button class="ml-2 underline" @click="erreur = ''">Fermer</button>
    </div>

    <!-- Mes pistes -->
    <div v-if="chargement" class="flex justify-center py-6">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-custom-chocolat" />
    </div>

    <div v-else>
      <div v-if="pistes.length === 0" class="text-sm text-gray-500 py-2">
        Vous n'avez pas encore contribué de piste pour cette vidéo.
      </div>

      <div v-else class="space-y-2">
        <div
          v-for="piste in pistes" :key="piste.id"
          class="flex items-center gap-3 p-3 rounded-lg cursor-pointer border transition-colors"
          :class="pisteSelectionnee?.id === piste.id
            ? 'border-custom-chocolat bg-custom-chocolat/5'
            : 'border-gray-200 hover:bg-gray-50'"
          @click="selectionnerPiste(piste)"
        >
          <span class="font-medium text-gray-900">{{ LANGUES_LABELS[piste.langue] || piste.langue }}</span>
          <span
            class="px-2 py-0.5 rounded-full text-xs font-medium"
            :class="{
              'bg-amber-100 text-amber-700': piste.etat === 'brouillon',
              'bg-custom-green/10 text-custom-green': piste.etat === 'publie',
              'bg-gray-200 text-gray-600': piste.etat === 'masque',
            }"
          >
            {{ labelEtat(piste.etat) }}
          </span>
          <span class="text-xs text-gray-400">{{ piste.nombre_segments }} segment{{ piste.nombre_segments > 1 ? 's' : '' }}</span>
          <button
            v-if="piste.etat !== 'masque'"
            class="ml-auto text-gray-400 hover:text-red-500 transition-colors"
            title="Supprimer la piste"
            @click.stop="supprimerPisteAction(piste.id)"
          >
            <font-awesome-icon icon="trash" />
          </button>
        </div>
      </div>

      <!-- Ajouter une langue -->
      <div v-if="languesDisponibles.length > 0" class="flex items-end gap-2 mt-4 pt-4 border-t border-gray-100">
        <div class="flex-1">
          <label class="block text-sm font-medium text-gray-700 mb-1">Ajouter une langue</label>
          <select
            v-model="nouvelleLangue"
            class="w-full px-3 py-2 rounded-lg border border-gray-300 text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-custom-chocolat/40 focus:border-custom-chocolat"
          >
            <option value="">Choisir une langue…</option>
            <option v-for="l in languesDisponibles" :key="l.code" :value="l.code">{{ l.label }}</option>
          </select>
        </div>
        <button
          class="px-4 py-2 rounded-lg bg-custom-chocolat text-white text-sm font-medium hover:bg-custom-chocolat/90 transition-colors disabled:opacity-60"
          :disabled="!nouvelleLangue || pisteLoading"
          @click="ajouterPiste"
        >
          <font-awesome-icon icon="plus" class="mr-1" /> Ajouter
        </button>
      </div>
    </div>

    <!-- Segments de la piste sélectionnée -->
    <div v-if="pisteSelectionnee" class="mt-6 pt-6 border-t border-gray-100">
      <div class="flex items-center justify-between mb-3">
        <h3 class="font-semibold text-gray-900">
          Segments — {{ LANGUES_LABELS[pisteSelectionnee.langue] || pisteSelectionnee.langue }}
        </h3>
        <button
          v-if="pisteEditable && modeSaisie === 'manuel'"
          class="px-3 py-1.5 rounded-lg bg-custom-chocolat text-white text-sm font-medium hover:bg-custom-chocolat/90 transition-colors"
          @click="showAjoutSegment = !showAjoutSegment"
        >
          <font-awesome-icon icon="plus" class="mr-1" /> Nouveau segment
        </button>
      </div>

      <!-- Mode « au fil de la lecture » : bande de capture pilotant le lecteur -->
      <VidafricaCaptureSequentielle
        v-if="modeSaisie === 'direct'"
        :key="pisteSelectionnee.id"
        :lecteur="lecteur"
        :piste-id="pisteSelectionnee.id"
        :piste-editable="pisteEditable"
        :curseur-initial="curseurDepart"
        :en-lecture="!!enLecture"
        :video-terminee="!!videoTerminee"
        @segment-cree="onSegmentDirectCree"
        @erreur="erreur = $event"
      />

      <p v-if="modeSaisie === 'direct' && pisteEditable" class="text-xs text-amber-600 mb-3 -mt-1">
        <font-awesome-icon icon="circle-info" /> Le mode direct crée des sous-titres au niveau du segment.
        Pour l'effet karaoké mot par mot, affinez ensuite via « Saisie manuelle ».
      </p>

      <p v-if="!pisteEditable" class="text-sm text-gray-500 rounded-lg bg-gray-50 px-3 py-2">
        Cette piste a été {{ labelEtat(pisteSelectionnee.etat).toLowerCase() }} : elle n'est plus modifiable.
      </p>

      <!-- Formulaire ajout segment (mode manuel) -->
      <div v-if="showAjoutSegment && pisteEditable && modeSaisie === 'manuel'" class="bg-gray-50 rounded-lg p-4 mb-3">
        <label class="block text-sm font-medium text-gray-700 mb-1">Texte du segment</label>
        <textarea
          v-model="nouveauSegment.texte"
          rows="2"
          placeholder="Ex : Bienvenue dans ce documentaire"
          class="w-full px-3 py-2 rounded-lg border border-gray-300 text-sm focus:outline-none focus:ring-2 focus:ring-custom-chocolat/40 focus:border-custom-chocolat mb-2"
        />
        <p v-if="lecteur" class="text-xs text-gray-500 mb-2">
          <font-awesome-icon icon="circle-info" class="text-custom-chocolat" />
          Astuce : mettez la vidéo à la bonne position puis cliquez <font-awesome-icon icon="stopwatch" /> pour capturer le temps (en millisecondes).
        </p>
        <div class="flex gap-3">
          <div class="flex-1">
            <label class="block text-xs font-medium text-gray-600 mb-1">Début (ms)</label>
            <div class="flex items-center gap-1">
              <input v-model.number="nouveauSegment.debut_ms" type="number" min="0" class="w-full px-2 py-1.5 rounded-lg border border-gray-300 text-sm focus:outline-none focus:ring-2 focus:ring-custom-chocolat/40">
              <button v-if="lecteur" type="button" class="shrink-0 w-8 h-8 inline-flex items-center justify-center rounded-lg bg-white border border-gray-300 text-gray-600 hover:bg-gray-100" title="Capturer la position actuelle de la vidéo" @click="capturerDansNouveau('debut_ms')">
                <font-awesome-icon icon="stopwatch" />
              </button>
            </div>
            <p class="text-[0.7rem] text-gray-400 mt-0.5 font-mono">= {{ formaterTimestamp(nouveauSegment.debut_ms || 0) }}</p>
          </div>
          <div class="flex-1">
            <label class="block text-xs font-medium text-gray-600 mb-1">Fin (ms)</label>
            <div class="flex items-center gap-1">
              <input v-model.number="nouveauSegment.fin_ms" type="number" min="0" class="w-full px-2 py-1.5 rounded-lg border border-gray-300 text-sm focus:outline-none focus:ring-2 focus:ring-custom-chocolat/40">
              <button v-if="lecteur" type="button" class="shrink-0 w-8 h-8 inline-flex items-center justify-center rounded-lg bg-white border border-gray-300 text-gray-600 hover:bg-gray-100" title="Capturer la position actuelle de la vidéo" @click="capturerDansNouveau('fin_ms')">
                <font-awesome-icon icon="stopwatch" />
              </button>
            </div>
            <p class="text-[0.7rem] text-gray-400 mt-0.5 font-mono">= {{ formaterTimestamp(nouveauSegment.fin_ms || 0) }}</p>
          </div>
        </div>
        <div class="flex justify-end gap-2 mt-3">
          <button class="px-3 py-1.5 rounded-lg text-sm text-gray-600 hover:bg-gray-100" @click="showAjoutSegment = false">Annuler</button>
          <button class="px-3 py-1.5 rounded-lg bg-custom-chocolat text-white text-sm font-medium hover:bg-custom-chocolat/90 disabled:opacity-60" :disabled="segmentLoading" @click="ajouterSegment">Ajouter</button>
        </div>
      </div>

      <!-- Liste des segments -->
      <div v-if="segmentLoading" class="flex justify-center py-4">
        <div class="animate-spin rounded-full h-6 w-6 border-b-2 border-custom-chocolat" />
      </div>

      <div v-else-if="segments.length === 0" class="text-sm text-gray-400 py-4 text-center">
        Aucun segment pour le moment.
      </div>

      <div v-else class="space-y-2">
        <div v-for="seg in segments" :key="seg.id" class="border border-gray-200 rounded-lg p-3">
          <!-- Affichage -->
          <div v-if="segmentEdite !== seg.id">
            <div class="flex items-start justify-between gap-2">
              <div>
                <span class="inline-block px-1.5 py-0.5 rounded border border-gray-300 text-xs text-gray-500 mr-2">#{{ seg.position }}</span>
                <span class="text-sm font-medium text-gray-800">{{ seg.texte }}</span>
              </div>
              <div v-if="pisteEditable" class="flex gap-1 shrink-0">
                <button class="p-1.5 rounded text-gray-500 hover:bg-gray-100" title="Marquer les timings" @click="ouvrirTapToMark(seg)">
                  <font-awesome-icon icon="stopwatch" />
                </button>
                <button class="p-1.5 rounded text-gray-500 hover:bg-gray-100" title="Modifier" @click="debutEdition(seg)">
                  <font-awesome-icon icon="pen" />
                </button>
                <button class="p-1.5 rounded text-gray-500 hover:text-red-500 hover:bg-gray-100" title="Supprimer" @click="supprimerSegmentAction(seg.id)">
                  <font-awesome-icon icon="trash" />
                </button>
              </div>
            </div>
            <div class="text-xs text-gray-400 mt-1">
              {{ formaterTimestamp(seg.debut_ms) }} → {{ formaterTimestamp(seg.fin_ms) }}
            </div>

            <!-- Timings mot -->
            <div v-if="seg.timings_mot.length > 0" class="mt-2 flex flex-wrap items-center gap-1">
              <span
                v-for="t in seg.timings_mot" :key="t.position"
                class="px-1.5 py-0.5 rounded border border-gray-200 text-xs text-gray-600"
                :title="`${formaterTimestamp(t.debut_ms)} → ${formaterTimestamp(t.fin_ms)}`"
              >
                {{ t.mot }}
              </span>
              <button v-if="pisteEditable" class="p-1 rounded text-amber-500 hover:bg-gray-100" title="Remarquer" @click="ouvrirTapToMark(seg)">
                <font-awesome-icon icon="redo" />
              </button>
              <button v-if="pisteEditable" class="p-1 rounded text-red-400 hover:bg-gray-100" title="Supprimer les timings" @click="supprimerTimingsAction(seg.id)">
                <font-awesome-icon icon="xmark" />
              </button>
            </div>
            <div v-else-if="pisteEditable" class="text-xs text-amber-600 mt-1">
              <font-awesome-icon icon="exclamation-triangle" /> Pas de timings mot — cliquez sur
              <font-awesome-icon icon="stopwatch" /> pour les marquer
            </div>
          </div>

          <!-- Édition -->
          <div v-else>
            <textarea v-model="segmentEditeData.texte" rows="2" class="w-full px-3 py-2 rounded-lg border border-gray-300 text-sm mb-2 focus:outline-none focus:ring-2 focus:ring-custom-chocolat/40" />
            <div class="flex gap-3 mb-1">
              <div class="flex-1">
                <div class="flex items-center gap-1">
                  <input v-model.number="segmentEditeData.debut_ms" type="number" min="0" placeholder="Début (ms)" class="w-full px-2 py-1.5 rounded-lg border border-gray-300 text-sm focus:outline-none focus:ring-2 focus:ring-custom-chocolat/40">
                  <button v-if="lecteur" type="button" class="shrink-0 w-8 h-8 inline-flex items-center justify-center rounded-lg bg-white border border-gray-300 text-gray-600 hover:bg-gray-100" title="Capturer la position actuelle de la vidéo" @click="capturerDansEdition('debut_ms')">
                    <font-awesome-icon icon="stopwatch" />
                  </button>
                </div>
                <p class="text-[0.7rem] text-gray-400 mt-0.5 font-mono">= {{ formaterTimestamp(segmentEditeData.debut_ms || 0) }}</p>
              </div>
              <div class="flex-1">
                <div class="flex items-center gap-1">
                  <input v-model.number="segmentEditeData.fin_ms" type="number" min="0" placeholder="Fin (ms)" class="w-full px-2 py-1.5 rounded-lg border border-gray-300 text-sm focus:outline-none focus:ring-2 focus:ring-custom-chocolat/40">
                  <button v-if="lecteur" type="button" class="shrink-0 w-8 h-8 inline-flex items-center justify-center rounded-lg bg-white border border-gray-300 text-gray-600 hover:bg-gray-100" title="Capturer la position actuelle de la vidéo" @click="capturerDansEdition('fin_ms')">
                    <font-awesome-icon icon="stopwatch" />
                  </button>
                </div>
                <p class="text-[0.7rem] text-gray-400 mt-0.5 font-mono">= {{ formaterTimestamp(segmentEditeData.fin_ms || 0) }}</p>
              </div>
            </div>
            <div class="flex justify-end gap-2">
              <button class="px-3 py-1.5 rounded-lg text-sm text-gray-600 hover:bg-gray-100" @click="segmentEdite = null">Annuler</button>
              <button class="px-3 py-1.5 rounded-lg bg-custom-chocolat text-white text-sm font-medium hover:bg-custom-chocolat/90" @click="sauvegarderSegment">Enregistrer</button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Modale Tap-to-mark -->
    <Teleport to="body">
      <div v-if="showTapToMark && tapToMarkSegment" class="fixed inset-0 z-50 flex items-center justify-center p-4">
        <div class="absolute inset-0 bg-black/50" @click="showTapToMark = false" />
        <div class="relative w-full max-w-3xl bg-white rounded-2xl shadow-xl max-h-[90vh] overflow-y-auto">
          <div class="flex items-center justify-between px-6 py-4 border-b border-gray-100">
            <h3 class="text-lg font-bold text-gray-900 font-['Oswald']">Marquer les timings mot par mot</h3>
            <button class="text-gray-400 hover:text-gray-600" @click="showTapToMark = false">
              <font-awesome-icon icon="xmark" class="text-xl" />
            </button>
          </div>
          <div class="px-6 py-5">
            <VidafricaTapToMarkPublic
              :video-url="props.videoUrl"
              :mots="tapToMarkSegment.texte.split(/\s+/).filter(Boolean)"
              :debut-ms="tapToMarkSegment.debut_ms"
              :fin-ms="tapToMarkSegment.fin_ms"
              @timings-enregistres="onTimingsEnregistres"
              @annuler="showTapToMark = false"
            />
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>
