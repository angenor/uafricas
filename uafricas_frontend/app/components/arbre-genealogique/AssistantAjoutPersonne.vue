<script setup lang="ts">
import { ref, reactive, computed, watch, onMounted, onBeforeUnmount, nextTick, shallowRef } from 'vue'
import gsap from 'gsap'
import type { CreerPersonneForm, Genre } from '~/mocks/arbre-genealogique'

// ─── Props & Emits ───────────────────────────────────────────────────────

interface PersonneLiee {
  id: string
  nom: string
  prenoms?: string
}

type TypeLienWizard = 'pere' | 'mere' | 'parent' | 'conjoint' | 'enfant'
type TypeActionWizard = 'parent' | 'enfant' | 'conjoint'

interface Props {
  typeLien?: TypeLienWizard
  typeAction?: TypeActionWizard
  personneLiee?: PersonneLiee
  loading?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
})

const emit = defineEmits<{
  submit: [form: CreerPersonneForm, typeLien?: TypeLienWizard]
  annuler: []
  'formulaire-classique': []
}>()

// ─── État du wizard ──────────────────────────────────────────────────────

const NOMBRE_ETAPES = 7
const etapeCourante = ref(1)
const erreurChamp = ref('')
const erreurReseau = ref('')
const succes = ref(false)

// Type de lien choisi par l'utilisateur (en mode contextuel)
const typeLienChoisi = ref<TypeLienWizard | undefined>(props.typeLien)

// Options de type de lien selon l'action
const optionsTypeLien = computed(() => {
  if (!props.typeAction) return []

  if (props.typeAction === 'parent') {
    return [
      { valeur: 'pere' as TypeLienWizard, label: 'Père', icone: 'user' },
      { valeur: 'mere' as TypeLienWizard, label: 'Mère', icone: 'user' },
      { valeur: 'parent' as TypeLienWizard, label: 'Parent (non précisé)', icone: 'users' },
    ]
  }

  if (props.typeAction === 'enfant') {
    // Le typeLien décrit le rôle de la personne courante envers le nouvel enfant
    return [
      { valeur: 'pere' as TypeLienWizard, label: `${nomPersonneLiee.value} est son père`, icone: 'user' },
      { valeur: 'mere' as TypeLienWizard, label: `${nomPersonneLiee.value} est sa mère`, icone: 'user' },
      { valeur: 'parent' as TypeLienWizard, label: `${nomPersonneLiee.value} est son parent`, icone: 'users' },
    ]
  }

  return []
})

const aChoixTypeLien = computed(() => optionsTypeLien.value.length > 0)

const formulaire = reactive<{
  nom: string
  prenoms: string
  genre: Genre | ''
  est_decede: boolean
  naissanceAnnee: string
  naissance_lieu: string
}>({
  nom: '',
  prenoms: '',
  genre: '',
  est_decede: false,
  naissanceAnnee: '',
  naissance_lieu: '',
})

// ─── Accessibilité ───────────────────────────────────────────────────────

const prefereReducedMotion = ref(false)

// ─── GSAP Context ────────────────────────────────────────────────────────

const overlayRef = ref<HTMLElement | null>(null)
const contenuRef = ref<HTMLElement | null>(null)
const etapeRef = ref<HTMLElement | null>(null)
let gsapCtx: gsap.Context | null = null
const timelineCourante = shallowRef<gsap.core.Timeline | null>(null)

// ─── Textes dynamiques ──────────────────────────────────────────────────

const nomAffiche = computed(() => {
  if (formulaire.prenoms) return formulaire.prenoms
  if (formulaire.nom) return formulaire.nom
  return ''
})

const nomPersonneLiee = computed(() => {
  if (!props.personneLiee) return ''
  return props.personneLiee.prenoms
    ? `${props.personneLiee.prenoms} ${props.personneLiee.nom}`
    : props.personneLiee.nom
})

function obtenirTexteAccroche(etape: number): string {
  const nom = nomAffiche.value
  const estDecede = formulaire.est_decede
  const aContexte = props.typeAction && props.personneLiee

  switch (etape) {
    case 1:
      if (aContexte) {
        if (props.typeAction === 'parent') {
          return `Ajoutons un parent à ${nomPersonneLiee.value}`
        }
        if (props.typeAction === 'enfant') {
          return `Ajoutons un enfant à ${nomPersonneLiee.value}`
        }
        if (props.typeAction === 'conjoint') {
          return `Qui est le/la conjoint(e) de ${nomPersonneLiee.value} ?`
        }
      }
      return 'Comment s\u2019appelle ce membre de votre famille ?'
    case 2:
      return nom
        ? `Magnifique ! Et quel est le prénom de ${nom} ?`
        : 'Et quel est son prénom ?'
    case 3:
      return nom
        ? `Pour mieux connaître ${nom}\u2026`
        : 'Pour mieux le/la connaître\u2026'
    case 4:
      return nom
        ? `${nom} est-il/elle toujours parmi nous ?`
        : 'Cette personne est-elle toujours parmi nous ?'
    case 5:
      if (estDecede) {
        return nom
          ? `Savez-vous quand ${nom} a vu le jour ?`
          : 'Savez-vous quand cette personne a vu le jour ?'
      }
      return nom
        ? `Savez-vous quand ${nom} est né(e) ?`
        : 'Savez-vous quand cette personne est née ?'
    case 6:
      return nom
        ? `Et où ${nom} a-t-il/elle vu le jour ?`
        : 'Et où a-t-il/elle vu le jour ?'
    case 7:
      return nom
        ? `Voici le portrait de ${nom} !`
        : 'Voici le récapitulatif !'
    default:
      return ''
  }
}

function obtenirTexteTransition(etape: number): string {
  if (formulaire.est_decede && etape > 4) {
    const transitions = ['Merci pour ce souvenir', 'Honorons sa mémoire', 'Continuons\u2026']
    return transitions[(etape - 5) % transitions.length] ?? 'Continuons\u2026'
  }
  const transitions = ['Parfait !', 'Très bien !', 'Continuons\u2026', 'Presque terminé !', 'Encore un peu\u2026', 'C\u2019est bientôt fini !']
  return transitions[(etape - 2) % transitions.length] ?? 'Continuons\u2026'
}

// Icônes par étape
function obtenirIcone(etape: number): string {
  if (etape === 4 && formulaire.est_decede) return 'dove'
  const icones: Record<number, string> = {
    1: 'user',
    2: 'signature',
    3: 'venus-mars',
    4: 'heart',
    5: 'calendar',
    6: 'map-marker-alt',
    7: 'circle-check',
  }
  return icones[etape] || 'circle'
}

// ─── Validation ─────────────────────────────────────────────────────────

function validerEtape(): boolean {
  erreurChamp.value = ''

  if (etapeCourante.value === 1) {
    if (!formulaire.nom.trim()) {
      erreurChamp.value = 'Le nom est obligatoire'
      return false
    }
  }

  if (etapeCourante.value === 5 && formulaire.naissanceAnnee) {
    const annee = parseInt(formulaire.naissanceAnnee)
    if (isNaN(annee) || annee < 1 || annee > new Date().getFullYear()) {
      erreurChamp.value = 'Année invalide'
      return false
    }
  }

  return true
}

// ─── Navigation ─────────────────────────────────────────────────────────

const texteTransition = ref('')
const afficherTransition = ref(false)
const enTransition = ref(false)

async function allerSuivant() {
  if (enTransition.value) return
  if (!validerEtape()) return

  if (etapeCourante.value < NOMBRE_ETAPES) {
    enTransition.value = true
    const prochaine = etapeCourante.value + 1
    texteTransition.value = obtenirTexteTransition(prochaine)
    afficherTransition.value = true

    if (!prefereReducedMotion.value) {
      await new Promise(resolve => setTimeout(resolve, 400))
    }
    afficherTransition.value = false
    etapeCourante.value++
    enTransition.value = false
  }
}

function allerPrecedent() {
  if (enTransition.value) return
  erreurChamp.value = ''
  if (etapeCourante.value > 1) {
    etapeCourante.value--
  }
}

function passerEtape() {
  if (enTransition.value) return
  erreurChamp.value = ''
  if (etapeCourante.value < NOMBRE_ETAPES) {
    etapeCourante.value++
  }
}

function allerAEtape(etape: number) {
  if (etape >= 1 && etape <= NOMBRE_ETAPES) {
    erreurChamp.value = ''
    etapeCourante.value = etape
  }
}

// ─── Soumission ─────────────────────────────────────────────────────────

async function soumettre() {
  erreurReseau.value = ''

  const annee = formulaire.naissanceAnnee ? parseInt(formulaire.naissanceAnnee) : undefined

  const form: CreerPersonneForm = {
    nom: formulaire.nom.trim(),
    prenoms: formulaire.prenoms.trim() || undefined,
    genre: (formulaire.genre as Genre) || undefined,
    naissance: annee ? { annee } : undefined,
    naissance_lieu: formulaire.naissance_lieu.trim() || undefined,
    est_decede: formulaire.est_decede,
  }

  emit('submit', form, typeLienChoisi.value)
}

// Gestion de l'état de succès (piloté par le parent via loading)
watch(() => props.loading, (nouveau, ancien) => {
  if (ancien && !nouveau && etapeCourante.value === 7) {
    succes.value = true
    lancerCelebration()
  }
})

// ─── Étape obligatoire ──────────────────────────────────────────────────

function etapeEstObligatoire(etape: number): boolean {
  return etape === 1
}

// ─── Indicateur de progression (6 dots, excluant récapitulatif) ─────────

const dotsRef = ref<HTMLElement | null>(null)

function animerProgression(nouvelleEtape: number, ancienneEtape: number) {
  if (!dotsRef.value || prefereReducedMotion.value) return

  const dots = dotsRef.value.querySelectorAll('.dot-indicateur')
  const segments = dotsRef.value.querySelectorAll('.segment-indicateur')

  dots.forEach((dot, i) => {
    const etapeIndex = i + 1
    if (etapeIndex === Math.min(nouvelleEtape, 6)) {
      gsap.to(dot, { scale: 1.3, duration: 0.3, ease: 'back.out(1.7)' })
    } else {
      gsap.to(dot, { scale: 1, duration: 0.2 })
    }
  })

  segments.forEach((seg, i) => {
    const etapeIndex = i + 1
    if (etapeIndex < Math.min(nouvelleEtape, 7)) {
      gsap.to(seg, { scaleX: 1, duration: 0.3, transformOrigin: ancienneEtape < nouvelleEtape ? 'left' : 'right' })
    } else {
      gsap.to(seg, { scaleX: 0, duration: 0.3, transformOrigin: ancienneEtape < nouvelleEtape ? 'right' : 'left' })
    }
  })
}

// ─── Animations GSAP ────────────────────────────────────────────────────

function animerEntreeOverlay() {
  if (prefereReducedMotion.value || !overlayRef.value || !contenuRef.value) return

  gsap.fromTo(overlayRef.value, { opacity: 0 }, { opacity: 1, duration: 0.4 })
  gsap.fromTo(contenuRef.value, { scale: 0.9, opacity: 0 }, { scale: 1, opacity: 1, duration: 0.4, ease: 'back.out(1.4)' })
}

function animerTransitionEtape(direction: 'avant' | 'arriere') {
  if (prefereReducedMotion.value || !etapeRef.value) return

  if (timelineCourante.value) {
    timelineCourante.value.kill()
  }

  const xDepart = direction === 'avant' ? 80 : -80

  const tl = gsap.timeline()
  tl.fromTo(
    etapeRef.value,
    { x: xDepart, opacity: 0 },
    { x: 0, opacity: 1, duration: 0.4, ease: 'power2.out' },
  )

  timelineCourante.value = tl
}

function lancerCelebration() {
  if (prefereReducedMotion.value) return

  const checkmark = document.querySelector('.checkmark-svg') as SVGPathElement
  if (checkmark) {
    const longueur = checkmark.getTotalLength()
    gsap.fromTo(
      checkmark,
      { strokeDasharray: longueur, strokeDashoffset: longueur },
      { strokeDashoffset: 0, duration: 0.6, ease: 'power2.out' },
    )
  }

  const checkContainer = document.querySelector('.checkmark-conteneur')
  if (checkContainer) {
    gsap.fromTo(
      checkContainer,
      { scale: 0 },
      { scale: 1, duration: 0.5, ease: 'back.out(1.7)' },
    )
  }

  const conteneurConfettis = document.querySelector('.confettis-zone')
  if (conteneurConfettis) {
    const couleurs = ['#A54A1C', '#228B22', '#FFD700', '#FF6347', '#4169E1', '#FF69B4']
    const fragments: HTMLDivElement[] = []

    for (let i = 0; i < 40; i++) {
      const div = document.createElement('div')
      div.style.position = 'absolute'
      div.style.width = `${gsap.utils.random(6, 12)}px`
      div.style.height = `${gsap.utils.random(6, 12)}px`
      div.style.backgroundColor = couleurs[Math.floor(Math.random() * couleurs.length)] ?? '#A54A1C'
      div.style.borderRadius = Math.random() > 0.5 ? '50%' : '2px'
      div.style.left = '50%'
      div.style.top = '50%'
      conteneurConfettis.appendChild(div)
      fragments.push(div)
    }

    gsap.to(fragments, {
      x: () => gsap.utils.random(-200, 200),
      y: () => gsap.utils.random(-250, 100),
      rotation: () => gsap.utils.random(-360, 360),
      scale: 0,
      opacity: 0,
      duration: 1.2,
      stagger: 0.02,
      ease: 'power2.out',
      onComplete: () => {
        fragments.forEach(f => f.remove())
      },
    })
  }
}

// ─── Watch transitions ──────────────────────────────────────────────────

watch(etapeCourante, async (nouvelle, ancienne) => {
  await nextTick()
  const direction = nouvelle > ancienne ? 'avant' : 'arriere'
  animerTransitionEtape(direction)
  animerProgression(nouvelle, ancienne)
})

// ─── Lifecycle ──────────────────────────────────────────────────────────

onMounted(() => {
  prefereReducedMotion.value = window.matchMedia('(prefers-reduced-motion: reduce)').matches

  gsapCtx = gsap.context(() => {}, overlayRef.value!)
  animerEntreeOverlay()
})

onBeforeUnmount(() => {
  if (timelineCourante.value) {
    timelineCourante.value.kill()
  }
  gsapCtx?.revert()
})

// ─── Récapitulatif ──────────────────────────────────────────────────────

const lignesRecapitulatif = computed(() => {
  const lignes: { label: string; valeur: string; etape: number }[] = []

  lignes.push({ label: 'Nom', valeur: formulaire.nom || '—', etape: 1 })
  lignes.push({ label: 'Prénoms', valeur: formulaire.prenoms || 'Non renseigné', etape: 2 })

  const genreLibelle: Record<string, string> = {
    masculin: 'Masculin',
    feminin: 'Féminin',
    autre: 'Autre',
    non_precise: 'Non précisé',
  }
  lignes.push({ label: 'Genre', valeur: formulaire.genre ? (genreLibelle[formulaire.genre] ?? 'Non renseigné') : 'Non renseigné', etape: 3 })

  lignes.push({ label: 'Statut', valeur: formulaire.est_decede ? 'Décédé(e)' : 'En vie', etape: 4 })

  lignes.push({ label: 'Année de naissance', valeur: formulaire.naissanceAnnee || 'Non renseignée', etape: 5 })

  lignes.push({ label: 'Lieu de naissance', valeur: formulaire.naissance_lieu || 'Non renseigné', etape: 6 })

  return lignes
})
</script>

<template>
  <div
    ref="overlayRef"
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4"
    @click.self="emit('annuler')"
  >
    <div
      ref="contenuRef"
      class="relative w-full max-w-lg rounded-2xl bg-white shadow-2xl overflow-hidden"
    >
      <!-- Zone confettis -->
      <div class="confettis-zone pointer-events-none absolute inset-0 overflow-hidden" />

      <!-- Indicateur de progression (6 dots) -->
      <div
        v-if="!succes"
        ref="dotsRef"
        class="flex items-center justify-center gap-0 px-8 pt-6 pb-2"
      >
        <template v-for="i in 6" :key="i">
          <div
            class="dot-indicateur h-3 w-3 rounded-full transition-colors duration-300"
            :class="[
              i < Math.min(etapeCourante, 7) ? 'bg-[var(--color-custom-green)]' :
              i === Math.min(etapeCourante, 7) ? 'bg-[var(--color-custom-chocolat)]' :
              'bg-stone-300'
            ]"
          />
          <div
            v-if="i < 6"
            class="segment-indicateur mx-1 h-0.5 w-6 origin-left bg-[var(--color-custom-green)]"
            :style="{ transform: i < Math.min(etapeCourante, 7) ? 'scaleX(1)' : 'scaleX(0)' }"
          />
        </template>
      </div>

      <!-- Bouton fermer -->
      <button
        class="absolute right-4 top-4 z-10 flex h-8 w-8 items-center justify-center rounded-full text-stone-400 transition-colors hover:bg-stone-100 hover:text-stone-600"
        @click="emit('annuler')"
      >
        <font-awesome-icon :icon="['fas', 'xmark']" />
      </button>

      <!-- Contenu principal -->
      <div class="px-8 pb-8 pt-4">
        <!-- Texte de transition -->
        <div
          v-if="afficherTransition"
          class="py-8 text-center text-lg font-semibold text-[var(--color-custom-green)]"
        >
          {{ texteTransition }}
        </div>

        <!-- Écran de succès (célébration) -->
        <div v-else-if="succes" class="py-12 text-center">
          <div class="checkmark-conteneur mx-auto mb-6 flex h-20 w-20 items-center justify-center rounded-full bg-[var(--color-custom-green)]/10">
            <svg class="h-12 w-12" viewBox="0 0 52 52" fill="none">
              <path
                class="checkmark-svg"
                d="M14 27 L22 35 L38 17"
                stroke="var(--color-custom-green)"
                stroke-width="4"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
            </svg>
          </div>
          <h2 class="mb-2 text-xl font-bold text-stone-800">Personne ajoutée !</h2>
          <p class="text-stone-500">
            {{ nomAffiche }} a été ajouté(e) à votre arbre généalogique.
          </p>
        </div>

        <!-- Étapes du wizard -->
        <div v-else ref="etapeRef">
          <!-- Icône + Accroche -->
          <div class="mb-6 text-center">
            <div class="mx-auto mb-3 flex h-14 w-14 items-center justify-center rounded-full bg-[var(--color-custom-chocolat)]/10">
              <font-awesome-icon
                :icon="['fas', obtenirIcone(etapeCourante)]"
                class="text-xl text-[var(--color-custom-chocolat)]"
              />
            </div>
            <h2 class="text-lg font-bold text-stone-800">
              {{ obtenirTexteAccroche(etapeCourante) }}
            </h2>
            <p class="mt-1 text-xs text-stone-400">
              Étape {{ Math.min(etapeCourante, 6) }} / 6
              <span v-if="etapeCourante <= 6 && !etapeEstObligatoire(etapeCourante)" class="ml-1">(optionnel)</span>
              <span v-if="etapeEstObligatoire(etapeCourante)" class="ml-1 text-red-400">*</span>
            </p>
          </div>

          <!-- Étape 1 : Type de lien (si contextuel) + Nom -->
          <div v-if="etapeCourante === 1" class="space-y-4">
            <!-- Sélecteur type de lien (mode contextuel) -->
            <div v-if="aChoixTypeLien" class="space-y-2">
              <p class="text-center text-sm font-medium text-stone-500">Quel lien ?</p>
              <div class="flex flex-wrap justify-center gap-2">
                <button
                  v-for="option in optionsTypeLien"
                  :key="option.valeur"
                  type="button"
                  class="rounded-lg border px-3 py-1.5 text-sm transition-all"
                  :class="typeLienChoisi === option.valeur
                    ? 'border-[var(--color-custom-chocolat)] bg-[var(--color-custom-chocolat)]/10 font-semibold text-[var(--color-custom-chocolat)]'
                    : 'border-stone-300 text-stone-600 hover:border-[var(--color-custom-chocolat)] hover:bg-[var(--color-custom-chocolat)]/5'"
                  @click="typeLienChoisi = option.valeur"
                >
                  {{ option.label }}
                </button>
              </div>
            </div>

            <input
              v-model="formulaire.nom"
              type="text"
              placeholder="Nom de famille"
              autofocus
              class="w-full rounded-xl border px-4 py-3 text-center text-lg text-stone-800 placeholder-stone-400 outline-none transition-shadow focus:ring-2 focus:ring-[var(--color-custom-chocolat)]/50"
              :class="erreurChamp ? 'border-red-400' : 'border-stone-300'"
              @keyup.enter="allerSuivant"
            />
            <p v-if="erreurChamp" class="text-center text-sm text-red-500">{{ erreurChamp }}</p>
          </div>

          <!-- Étape 2 : Prénoms -->
          <div v-if="etapeCourante === 2" class="space-y-4">
            <input
              v-model="formulaire.prenoms"
              type="text"
              placeholder="Prénoms (optionnel)"
              autofocus
              class="w-full rounded-xl border border-stone-300 px-4 py-3 text-center text-lg text-stone-800 placeholder-stone-400 outline-none transition-shadow focus:ring-2 focus:ring-[var(--color-custom-chocolat)]/50"
              @keyup.enter="allerSuivant"
            />
          </div>

          <!-- Étape 3 : Genre -->
          <div v-if="etapeCourante === 3" class="space-y-3">
            <button
              v-for="option in [
                { valeur: 'masculin', label: 'Masculin' },
                { valeur: 'feminin', label: 'Féminin' },
                { valeur: 'autre', label: 'Autre' },
                { valeur: 'non_precise', label: 'Ne souhaite pas préciser' },
              ]"
              :key="option.valeur"
              type="button"
              class="w-full rounded-xl border px-4 py-3 text-left text-stone-700 transition-all hover:border-[var(--color-custom-chocolat)] hover:bg-[var(--color-custom-chocolat)]/5"
              :class="formulaire.genre === option.valeur
                ? 'border-[var(--color-custom-chocolat)] bg-[var(--color-custom-chocolat)]/10 font-semibold'
                : 'border-stone-300'"
              @click="formulaire.genre = option.valeur as Genre; allerSuivant()"
            >
              {{ option.label }}
            </button>
          </div>

          <!-- Étape 4 : Statut vital -->
          <div v-if="etapeCourante === 4" class="space-y-3">
            <button
              v-for="option in [
                { valeur: false, label: 'En vie', icone: 'heart' },
                { valeur: true, label: 'Décédé(e)', icone: 'dove' },
              ]"
              :key="String(option.valeur)"
              type="button"
              class="flex w-full items-center gap-3 rounded-xl border px-4 py-3 text-left text-stone-700 transition-all hover:border-[var(--color-custom-chocolat)] hover:bg-[var(--color-custom-chocolat)]/5"
              :class="formulaire.est_decede === option.valeur
                ? 'border-[var(--color-custom-chocolat)] bg-[var(--color-custom-chocolat)]/10 font-semibold'
                : 'border-stone-300'"
              @click="formulaire.est_decede = option.valeur; allerSuivant()"
            >
              <font-awesome-icon :icon="['fas', option.icone]" class="text-stone-500" />
              {{ option.label }}
            </button>
          </div>

          <!-- Étape 5 : Année de naissance -->
          <div v-if="etapeCourante === 5" class="space-y-4">
            <input
              v-model="formulaire.naissanceAnnee"
              type="number"
              placeholder="ex : 1920"
              min="1"
              :max="new Date().getFullYear()"
              autofocus
              class="w-full rounded-xl border px-4 py-3 text-center text-lg text-stone-800 placeholder-stone-400 outline-none transition-shadow focus:ring-2 focus:ring-[var(--color-custom-chocolat)]/50"
              :class="erreurChamp ? 'border-red-400' : 'border-stone-300'"
              @keyup.enter="allerSuivant"
            />
            <p v-if="erreurChamp" class="text-center text-sm text-red-500">{{ erreurChamp }}</p>
          </div>

          <!-- Étape 6 : Lieu de naissance -->
          <div v-if="etapeCourante === 6" class="space-y-4">
            <input
              v-model="formulaire.naissance_lieu"
              type="text"
              placeholder="ex : Dakar, Sénégal"
              autofocus
              class="w-full rounded-xl border border-stone-300 px-4 py-3 text-center text-lg text-stone-800 placeholder-stone-400 outline-none transition-shadow focus:ring-2 focus:ring-[var(--color-custom-chocolat)]/50"
              @keyup.enter="allerSuivant"
            />
          </div>

          <!-- Étape 7 : Récapitulatif -->
          <div v-if="etapeCourante === 7" class="space-y-4">
            <div class="space-y-2 rounded-xl bg-stone-50 p-4">
              <div
                v-for="ligne in lignesRecapitulatif"
                :key="ligne.label"
                class="flex items-center justify-between border-b border-stone-200 py-2 last:border-0"
              >
                <span class="text-sm font-medium text-stone-500">{{ ligne.label }}</span>
                <div class="flex items-center gap-2">
                  <span class="text-sm text-stone-800">{{ ligne.valeur }}</span>
                  <button
                    type="button"
                    class="text-xs text-[var(--color-custom-chocolat)] underline hover:no-underline"
                    @click="allerAEtape(ligne.etape)"
                  >
                    Modifier
                  </button>
                </div>
              </div>
            </div>

            <!-- Erreur réseau -->
            <div v-if="erreurReseau" class="rounded-lg bg-red-50 p-3 text-center text-sm text-red-600">
              {{ erreurReseau }}
              <button
                type="button"
                class="ml-2 font-semibold underline"
                @click="soumettre"
              >
                Réessayer
              </button>
            </div>
          </div>

          <!-- Boutons de navigation -->
          <div class="mt-8 flex items-center gap-3">
            <!-- Bouton Retour -->
            <button
              v-if="etapeCourante > 1"
              type="button"
              class="flex items-center gap-2 rounded-xl border border-stone-300 px-4 py-2.5 text-sm font-semibold text-stone-600 transition-colors hover:bg-stone-50"
              @click="allerPrecedent"
            >
              <font-awesome-icon :icon="['fas', 'chevron-left']" class="text-xs" />
              Retour
            </button>

            <div class="flex-1" />

            <!-- Bouton Passer (si étape optionnelle et pas récap) -->
            <button
              v-if="etapeCourante < 7 && !etapeEstObligatoire(etapeCourante)"
              type="button"
              class="text-sm text-stone-400 transition-colors hover:text-stone-600"
              @click="passerEtape"
            >
              Passer
            </button>

            <!-- Bouton Suivant / Valider -->
            <button
              v-if="etapeCourante < 7"
              type="button"
              class="flex items-center gap-2 rounded-xl bg-[var(--color-custom-chocolat)] px-6 py-2.5 text-sm font-semibold text-white transition-opacity hover:opacity-90"
              @click="allerSuivant"
            >
              Suivant
              <font-awesome-icon :icon="['fas', 'chevron-right']" class="text-xs" />
            </button>

            <button
              v-if="etapeCourante === 7"
              type="button"
              :disabled="loading"
              class="flex items-center gap-2 rounded-xl bg-[var(--color-custom-green)] px-6 py-2.5 text-sm font-semibold text-white transition-opacity hover:opacity-90 disabled:cursor-not-allowed disabled:opacity-50"
              @click="soumettre"
            >
              <font-awesome-icon v-if="loading" :icon="['fas', 'spinner']" class="animate-spin" />
              <font-awesome-icon v-else :icon="['fas', 'check']" />
              {{ loading ? 'Enregistrement…' : 'Valider' }}
            </button>
          </div>

          <!-- Lien formulaire classique -->
          <div class="mt-6 text-center">
            <p class="text-xs text-stone-400">
              Vous préférez un formulaire classique ?
              <button
                type="button"
                class="text-[var(--color-custom-gray)] underline transition-colors hover:no-underline"
                @click="emit('formulaire-classique')"
              >
                Formulaire rapide
              </button>
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
