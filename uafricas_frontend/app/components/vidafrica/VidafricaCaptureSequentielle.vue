<script setup lang="ts">
import type { LecteurControl } from '~/composables/useVidafrica'
import { formaterTimestamp } from '~/mocks/vidafrica'

/**
 * Sous-titrage « au fil de la lecture ».
 *
 * Ne possède PAS son propre <video> : il pilote le lecteur unique déjà affiché
 * (VidafricaLecteur) via la poignée `lecteur`. Le contributeur laisse jouer la
 * vidéo puis, à un point de rupture, coupe (« Sous-titrer ici ») ; la vidéo se
 * met en pause pile sur la coupe, il saisit le texte du passage écoulé
 * [curseurMs → finMs], valide, et la lecture reprend exactement à la coupe.
 * Les segments sont donc strictement contigus et tous les timings proviennent
 * de la lecture — aucune saisie de millisecondes.
 */

const props = defineProps<{
  lecteur: LecteurControl | null
  pisteId: string | null
  pisteEditable: boolean
  // Repère de reprise (= max des fin_ms existants). Toujours frais côté parent.
  curseurInitial: number
  // État réel de lecture relayé (page → conteneur → ici).
  enLecture: boolean
  // La vidéo a atteint sa fin (permet de sous-titrer le tout dernier passage).
  videoTerminee?: boolean
}>()

const emit = defineEmits<{
  'segment-cree': []
  'erreur': [message: string]
}>()

const { creerSegment } = useVidafricaContribution()

// ── État ────────────────────────────────────────────────────
// curseurMs = début du prochain segment (= fin du dernier validé) → contiguïté.
const curseurMs = ref(props.curseurInitial)
// finMs = fin capturée ; non-null ⇒ on est en phase de saisie.
const finMs = ref<number | null>(null)
const texte = ref('')
const enregistrement = ref(false)
const champRef = ref<HTMLTextAreaElement | null>(null)

const enSaisie = computed(() => finMs.value !== null)
const dureeSegment = computed(() => (finMs.value ?? curseurMs.value) - curseurMs.value)

// Reste-t-il de la matière à sous-titrer jusqu'à la fin ? (fonction : relue à chaque rendu)
const resteFinASoustitrer = () => (props.lecteur?.dureeMs() ?? 0) > curseurMs.value

// ── Actions ─────────────────────────────────────────────────
const jouer = (e?: Event) => {
  ;(e?.currentTarget as HTMLElement | null)?.blur?.()
  props.lecteur?.lire()
}

const reecouter = (e?: Event) => {
  ;(e?.currentTarget as HTMLElement | null)?.blur?.()
  props.lecteur?.seek(curseurMs.value, true)
}

const ouvrirSaisie = (fin: number) => {
  props.lecteur?.pause()
  finMs.value = fin
  texte.value = ''
  nextTick(() => champRef.value?.focus())
}

// Couper : capture la position courante comme fin du segment.
const capturer = () => {
  if (!props.pisteEditable || !props.pisteId || finMs.value !== null) return
  const pos = props.lecteur?.positionMs() ?? 0
  if (pos <= curseurMs.value) {
    emit('erreur', 'Laissez la vidéo avancer un peu avant de couper le sous-titre.')
    return
  }
  ouvrirSaisie(pos)
}

// Sous-titrer le tout dernier passage jusqu'à la fin exacte de la vidéo.
const capturerFin = () => {
  if (!props.pisteEditable || !props.pisteId || finMs.value !== null) return
  const fin = props.lecteur?.dureeMs() ?? 0
  if (fin <= curseurMs.value) return
  ouvrirSaisie(fin)
}

// Valider le sous-titre du passage écoulé puis reprendre la lecture au repère.
const validerDirect = async () => {
  if (!props.pisteId || finMs.value === null || !texte.value.trim() || enregistrement.value) return
  enregistrement.value = true
  const fin = finMs.value
  try {
    await creerSegment(props.pisteId, {
      texte: texte.value.trim(),
      debut_ms: curseurMs.value,
      fin_ms: fin,
    })
    curseurMs.value = fin
    finMs.value = null
    texte.value = ''
    emit('segment-cree')
    // Reprendre EXACTEMENT au repère (robuste même si les contrôles natifs ont scrubé).
    props.lecteur?.seek(fin, true)
  } catch (e: any) {
    // Curseur et texte préservés pour réessayer sans rien perdre.
    emit('erreur', e?.data?.error || e?.message || 'Échec de l\'enregistrement — réessayez.')
  } finally {
    enregistrement.value = false
  }
}

// Passage sans dialogue : on avance le curseur sans créer de segment.
const passerBlanc = () => {
  if (finMs.value === null) return
  const fin = finMs.value
  curseurMs.value = fin
  finMs.value = null
  texte.value = ''
  props.lecteur?.seek(fin, true)
}

// Abandonner la capture en cours (la vidéo reste en pause sur la coupe).
const annulerCapture = () => {
  finMs.value = null
  texte.value = ''
}

// ── Raccourcis clavier ──────────────────────────────────────
const onKeydown = (e: KeyboardEvent) => {
  if (!props.pisteEditable || !props.pisteId) return
  if (e.code === 'Escape' && finMs.value !== null) {
    e.preventDefault()
    annulerCapture()
    return
  }
  // Espace coupe, SAUF si un vrai contrôle a le focus (on ne vole pas son action
  // native). La vidéo focalisée est admise : preventDefault bloque le play/pause
  // natif et déclenche la coupe à la place — sinon le raccourci serait inopérant
  // dès qu'on démarre via les contrôles natifs.
  if (e.code === 'Space' && props.enLecture && finMs.value === null) {
    const ae = document.activeElement as HTMLElement | null
    const tag = ae?.tagName
    if (ae?.isContentEditable || (tag && ['INPUT', 'TEXTAREA', 'SELECT', 'BUTTON', 'A'].includes(tag))) return
    e.preventDefault()
    capturer()
  }
}

// ── Repositionnement au repère de reprise ───────────────────
// `curseurInitial` reflète max(fin_ms) côté parent. Au sein d'UNE session (même
// piste — le composant est keyé par piste et remonté à tout changement), le
// repère ne fait qu'avancer. On ne repositionne donc que vers l'AVANT : cela
// neutralise à la fois le no-op post-validation (micro-rembobinage) et un
// éventuel retour en arrière dû à des réponses chargerSegments arrivées hors
// ordre. Jamais en pleine capture.
watch(() => props.curseurInitial, (v) => {
  if (finMs.value !== null || v <= curseurMs.value) return
  curseurMs.value = v
  props.lecteur?.seek(v, false)
})

// Changer de piste en pleine capture ⇒ on abandonne la capture en cours.
watch(() => props.pisteId, () => {
  finMs.value = null
  texte.value = ''
})

onMounted(() => {
  window.addEventListener('keydown', onKeydown)
  // Poser la tête de lecture au repère de reprise à l'ouverture de la bande.
  if (props.pisteId && props.pisteEditable) props.lecteur?.seek(curseurMs.value, false)
})
onUnmounted(() => window.removeEventListener('keydown', onKeydown))
</script>

<template>
  <div class="rounded-xl bg-custom-chocolat/5 ring-1 ring-custom-chocolat/15 p-3 mb-4">
    <!-- Repère d'intervalle -->
    <div class="flex items-center justify-between gap-2 mb-2.5">
      <span class="flex items-center gap-2 text-xs font-bold text-custom-chocolat font-['Oswald'] uppercase tracking-wide">
        <font-awesome-icon icon="wand-magic-sparkles" /> Au fil de la lecture
      </span>
      <span class="font-mono text-xs text-gray-500 tabular-nums">
        {{ formaterTimestamp(curseurMs) }}
        <span class="text-gray-400">→</span>
        {{ finMs !== null ? formaterTimestamp(finMs) : '…' }}
        <span v-if="dureeSegment > 0" class="text-gray-400">({{ (dureeSegment / 1000).toFixed(1) }}s)</span>
      </span>
    </div>

    <!-- Piste non prête -->
    <p v-if="!pisteId || !pisteEditable" class="text-sm text-gray-500 py-1">
      Sélectionnez ou créez une piste éditable ci-dessous pour démarrer le sous-titrage direct.
    </p>

    <!-- Phase LECTURE / PAUSE : un seul bouton primaire selon l'état réel -->
    <div v-else-if="!enSaisie" class="flex flex-wrap items-center gap-2">
      <!-- Fin de vidéo : proposer de sous-titrer le dernier passage jusqu'à la fin -->
      <button
        v-if="videoTerminee && resteFinASoustitrer()"
        class="inline-flex items-center gap-2 px-4 py-2 rounded-lg bg-custom-chocolat text-white text-sm font-semibold shadow-sm hover:bg-custom-chocolat/90 transition-colors"
        @click="capturerFin"
      >
        <font-awesome-icon icon="scissors" /> Sous-titrer jusqu'à la fin
      </button>
      <template v-else>
        <button
          v-if="enLecture"
          class="inline-flex items-center gap-2 px-4 py-2 rounded-lg bg-custom-chocolat text-white text-sm font-semibold shadow-sm hover:bg-custom-chocolat/90 transition-colors"
          @click="capturer"
        >
          <font-awesome-icon icon="scissors" /> Sous-titrer ici
        </button>
        <button
          v-else
          class="inline-flex items-center gap-2 px-4 py-2 rounded-lg bg-custom-chocolat text-white text-sm font-semibold shadow-sm hover:bg-custom-chocolat/90 transition-colors"
          @click="jouer"
        >
          <font-awesome-icon icon="play" /> Lire
        </button>
      </template>

      <button
        class="inline-flex items-center gap-2 px-3 py-2 rounded-lg bg-white text-gray-700 text-sm font-medium ring-1 ring-gray-300 hover:bg-gray-50 transition-colors"
        title="Replacer la lecture au dernier repère"
        @click="reecouter"
      >
        <font-awesome-icon icon="rotate-left" /> Réécouter
      </button>

      <span class="text-xs text-gray-400 ml-auto hidden sm:block">
        <kbd class="px-1.5 py-0.5 rounded border border-gray-300 bg-white text-[0.7rem] font-mono text-gray-600">Espace</kbd>
        pour couper
      </span>
    </div>

    <!-- Phase SAISIE : la vidéo est en pause pile sur la coupe -->
    <div v-else class="space-y-2">
      <textarea
        ref="champRef"
        v-model="texte"
        rows="2"
        placeholder="Tapez le sous-titre du passage écoulé…"
        class="w-full px-3 py-2 rounded-lg border border-gray-300 bg-white text-gray-900 text-sm placeholder:text-gray-400 focus:outline-none focus:ring-2 focus:ring-custom-chocolat/40 focus:border-custom-chocolat resize-none"
        @keydown.enter.exact.prevent="validerDirect"
      />
      <div class="flex flex-wrap items-center gap-2">
        <button
          class="inline-flex items-center gap-2 px-4 py-2 rounded-lg bg-custom-green text-white text-sm font-semibold shadow-sm hover:bg-custom-green/90 transition-colors disabled:opacity-50"
          :disabled="!texte.trim() || enregistrement"
          @click="validerDirect"
        >
          <font-awesome-icon :icon="enregistrement ? 'spinner' : 'check'" :spin="enregistrement" />
          Valider &amp; continuer
        </button>
        <button
          class="inline-flex items-center gap-2 px-3 py-2 rounded-lg bg-white text-gray-700 text-sm font-medium ring-1 ring-gray-300 hover:bg-gray-50 transition-colors"
          title="Aucun dialogue ici : avancer sans créer de sous-titre"
          @click="passerBlanc"
        >
          <font-awesome-icon icon="forward" /> Passer (blanc)
        </button>
        <button
          class="inline-flex items-center gap-2 px-3 py-2 rounded-lg text-gray-500 text-sm font-medium hover:bg-gray-100 transition-colors"
          @click="annulerCapture"
        >
          Annuler
        </button>
        <span class="text-xs text-gray-400 ml-auto hidden sm:block">
          <kbd class="px-1.5 py-0.5 rounded border border-gray-300 bg-white text-[0.7rem] font-mono text-gray-600">Entrée</kbd>
          valider ·
          <kbd class="px-1.5 py-0.5 rounded border border-gray-300 bg-white text-[0.7rem] font-mono text-gray-600">Échap</kbd>
          annuler
        </span>
      </div>
    </div>
  </div>
</template>
