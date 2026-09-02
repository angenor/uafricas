<template>
  <Teleport to="body">
    <Transition
      enter-active-class="transition duration-150 ease-out"
      enter-from-class="opacity-0"
      leave-active-class="transition duration-100 ease-in"
      leave-to-class="opacity-0"
    >
      <div
        v-if="modelValue && groupeCourant"
        class="fixed inset-0 z-100 flex flex-col bg-black/95 font-af"
        role="dialog"
        aria-modal="true"
        :aria-label="`Africanités de ${groupeCourant.prenom} ${groupeCourant.nom}`"
      >
        <!-- Barres de progression : une par africanité de l'auteur courant.
             Elles disent où l'on en est ET combien il en reste, un compteur
             « 2/5 » ne montrerait pas la seconde information d'un coup d'œil. -->
        <div class="flex gap-1 p-3">
          <span
            v-for="(a, i) in groupeCourant.africanites"
            :key="a.id"
            class="h-1 flex-1 overflow-hidden rounded-full bg-white/30"
          >
            <span
              class="block h-full rounded-full bg-white transition-[width] duration-100 ease-linear"
              :style="{ width: i < indexAfricanite ? '100%' : i === indexAfricanite ? `${progression}%` : '0%' }"
            />
          </span>
        </div>

        <header class="flex items-center gap-3 px-4 pb-3">
          <AfricansAvatar
            :nom="`${groupeCourant.prenom} ${groupeCourant.nom}`"
            :src="resoudreMedia(groupeCourant.photo_url)"
            :taille="40"
          />
          <div class="min-w-0 flex-1">
            <p class="truncate text-[14px]/[1.4] font-bold text-white">
              {{ groupeCourant.est_moi ? 'Vous' : `${groupeCourant.prenom} ${groupeCourant.nom}` }}
            </p>
            <p class="text-[12px]/[1.4] text-white/70">{{ quand }}</p>
          </div>

          <!-- Le décompte n'apparaît que sur ses propres africanités : nul ne
               voit les lecteurs d'une africanité qui n'est pas la sienne. -->
          <span
            v-if="africaniteCourante?.nombre_vues !== undefined"
            class="flex items-center gap-2 text-[12px]/[1.4] text-white/70"
          >
            <font-awesome-icon icon="fa-solid fa-eye" />
            {{ africaniteCourante.nombre_vues }}
          </span>

          <button
            type="button"
            class="grid size-9 place-items-center rounded-full text-white transition hover:bg-white/15"
            aria-label="Fermer"
            @click="fermer"
          >
            <font-awesome-icon icon="fa-solid fa-xmark" class="text-xl" />
          </button>
        </header>

        <!-- Corps. Les deux moitiés de l'écran font reculer et avancer : c'est
             le geste attendu du format, et il évite d'avoir à viser une flèche. -->
        <div class="relative flex-1 overflow-hidden">
          <div class="absolute inset-0 grid place-items-center p-4">
            <img
              v-if="africaniteCourante?.forme === 'image'"
              :src="resoudreMedia(africaniteCourante.media_url) ?? ''"
              alt=""
              class="max-h-full max-w-full rounded-[10px] object-contain"
            />

            <video
              v-else-if="africaniteCourante?.forme === 'video'"
              ref="lecteur"
              :src="resoudreMedia(africaniteCourante.media_url) ?? ''"
              class="max-h-full max-w-full rounded-[10px]"
              autoplay
              playsinline
              @ended="suivant"
            />

            <div
              v-else-if="africaniteCourante"
              class="grid size-full max-w-lg place-items-center rounded-[10px] p-10 text-center"
              :style="{ backgroundColor: africaniteCourante.couleur_fond || '#A74916' }"
            >
              <p class="text-[24px]/[1.4] font-bold whitespace-pre-line text-white">
                {{ africaniteCourante.texte }}
              </p>
            </div>
          </div>

          <button
            type="button"
            class="absolute inset-y-0 left-0 w-1/3 cursor-default focus:outline-none"
            aria-label="Précédent"
            @click="precedent"
          />
          <button
            type="button"
            class="absolute inset-y-0 right-0 w-2/3 cursor-default focus:outline-none"
            aria-label="Suivant"
            @click="suivant"
          />
        </div>

        <p
          v-if="africaniteCourante?.legende"
          class="px-6 pb-8 text-center text-[16px]/[1.4] text-white"
        >
          {{ africaniteCourante.legende }}
        </p>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import type { AuteurAfricanitesAPI } from '~/composables/useAfricanite'

/**
 * Visionneuse plein écran. Elle enchaîne les africanités d'un auteur, puis
 * passe à l'auteur suivant, et se referme après le dernier (FR-009, FR-010).
 *
 * L'avance est chronométrée pour les formes fixes, image et texte. Une vidéo,
 * elle, avance à SA fin : lui imposer une durée arbitraire la couperait au
 * milieu ou laisserait un écran figé après la dernière image.
 */
const props = defineProps<{
  modelValue: boolean
  groupes: AuteurAfricanitesAPI[]
  /** Auteur par lequel commencer. */
  auteurInitial: string | null
}>()

const emit = defineEmits<{ 'update:modelValue': [boolean], vue: [africaniteId: string] }>()

const { resoudreMedia } = useAfricanite()

/** Durée d'affichage d'une forme fixe, en millisecondes. */
const DUREE_MS = 5000
const PAS_MS = 50

const indexGroupe = ref(0)
const indexAfricanite = ref(0)
const progression = ref(0)
let minuteur: ReturnType<typeof setInterval> | null = null

const groupeCourant = computed(() => props.groupes[indexGroupe.value])
const africaniteCourante = computed(() => groupeCourant.value?.africanites[indexAfricanite.value])

const quand = computed(() => {
  const a = africaniteCourante.value
  if (!a) return ''
  const ms = Date.now() - new Date(a.created_at).getTime()
  const heures = Math.floor(ms / 3_600_000)
  if (heures < 1) return `il y a ${Math.max(1, Math.floor(ms / 60_000))} min`
  return `il y a ${heures} h`
})

function arreterMinuteur() {
  if (minuteur) clearInterval(minuteur)
  minuteur = null
}

/** La vidéo n'est PAS chronométrée : elle avance sur son propre `ended`. */
function demarrerMinuteur() {
  arreterMinuteur()
  progression.value = 0
  if (africaniteCourante.value?.forme === 'video') return
  minuteur = setInterval(() => {
    progression.value += (PAS_MS / DUREE_MS) * 100
    if (progression.value >= 100) suivant()
  }, PAS_MS)
}

function fermer() {
  arreterMinuteur()
  emit('update:modelValue', false)
}

function suivant() {
  const groupe = groupeCourant.value
  if (!groupe) return fermer()
  if (indexAfricanite.value < groupe.africanites.length - 1) {
    indexAfricanite.value += 1
  }
  else if (indexGroupe.value < props.groupes.length - 1) {
    indexGroupe.value += 1
    indexAfricanite.value = 0
  }
  else {
    // Dernière africanité du dernier auteur : on rend le fil (FR-009).
    return fermer()
  }
}

function precedent() {
  if (indexAfricanite.value > 0) {
    indexAfricanite.value -= 1
  }
  else if (indexGroupe.value > 0) {
    indexGroupe.value -= 1
    indexAfricanite.value = Math.max(0, (groupeCourant.value?.africanites.length ?? 1) - 1)
  }
}

function surTouche(e: KeyboardEvent) {
  if (e.key === 'Escape') fermer()
  else if (e.key === 'ArrowRight') suivant()
  else if (e.key === 'ArrowLeft') precedent()
}

// Marquer comme vue à l'AFFICHAGE et non à la fermeture : fermer au milieu
// d'un enchaînement laisserait comme non vues des africanités déjà regardées.
watch(africaniteCourante, (a) => {
  if (!a) return
  demarrerMinuteur()
  if (!a.vue) emit('vue', a.id)
}, { immediate: true })

watch(() => props.modelValue, (ouvert) => {
  if (import.meta.server) return
  if (ouvert) {
    const depart = props.groupes.findIndex(g => g.auteur_id === props.auteurInitial)
    indexGroupe.value = depart >= 0 ? depart : 0
    indexAfricanite.value = 0
    document.addEventListener('keydown', surTouche)
    document.body.style.overflow = 'hidden'
    demarrerMinuteur()
  }
  else {
    document.removeEventListener('keydown', surTouche)
    document.body.style.overflow = ''
    arreterMinuteur()
  }
})

onUnmounted(() => {
  if (import.meta.server) return
  document.removeEventListener('keydown', surTouche)
  document.body.style.overflow = ''
  arreterMinuteur()
})
</script>
