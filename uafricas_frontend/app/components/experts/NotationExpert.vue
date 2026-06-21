<script setup lang="ts">
import { useExperts, type NoteExpertAPI } from '~/composables/useExperts'

const props = defineProps<{
  /** utilisateur_id de l'expert noté */
  utilisateurId: string
  /** Note moyenne actuelle (0–5) */
  rating: number
  /** Nombre de notes reçues */
  nombreNotes: number
  /** Note du membre connecté (1–5) ou null */
  maNote: number | null
  /** Le membre connecté peut-il noter (connecté + hors son profil) */
  peutNoter: boolean
}>()

const emit = defineEmits<{ (e: 'note', payload: NoteExpertAPI): void }>()

const { noterExpert } = useExperts()

const survol = ref(0)
const enCours = ref(false)
const erreur = ref<string | null>(null)

/** Niveau d'étoiles à peindre : survol prioritaire, sinon la note du membre. */
const niveauAffiche = computed(() => survol.value || props.maNote || 0)

const soumettre = async (note: number) => {
  if (!props.peutNoter || enCours.value || note === props.maNote) return
  enCours.value = true
  erreur.value = null
  try {
    const res = await noterExpert(props.utilisateurId, note)
    if (res) emit('note', res)
  }
  catch (e: any) {
    erreur.value = e?.message || 'Impossible d\'enregistrer la note'
  }
  finally {
    enCours.value = false
  }
}
</script>

<template>
  <div class="bg-gray-50 rounded-xl p-4">
    <p class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-1">Note</p>

    <!-- Moyenne + nombre d'avis -->
    <p class="text-sm font-semibold text-gray-800 flex items-center gap-1.5">
      <font-awesome-icon icon="fa-solid fa-star" class="text-amber-500" />
      {{ rating.toFixed(1) }}/5
      <span class="text-xs font-normal text-gray-400">
        ({{ nombreNotes }} avis)
      </span>
    </p>

    <!-- Étoiles interactives -->
    <div v-if="peutNoter" class="mt-3">
      <p class="text-xs text-gray-500 mb-1">{{ maNote ? 'Votre note' : 'Notez cet expert' }}</p>
      <div class="flex items-center gap-1" @mouseleave="survol = 0">
        <button
          v-for="n in 5"
          :key="n"
          type="button"
          :disabled="enCours"
          class="text-xl leading-none transition-transform hover:scale-110 disabled:opacity-50 disabled:cursor-not-allowed"
          :aria-label="`Donner la note de ${n} sur 5`"
          @mouseenter="survol = n"
          @click="soumettre(n)"
        >
          <font-awesome-icon
            icon="fa-solid fa-star"
            :class="n <= niveauAffiche ? 'text-amber-500' : 'text-gray-300'"
          />
        </button>
        <font-awesome-icon
          v-if="enCours"
          icon="fa-solid fa-spinner"
          class="text-gray-400 animate-spin ml-1.5 text-sm"
        />
      </div>
      <p v-if="erreur" class="text-xs text-red-500 mt-1">{{ erreur }}</p>
    </div>

    <!-- Invitation à se connecter -->
    <p v-else-if="!peutNoter" class="text-xs text-gray-400 mt-2">
      Connectez-vous pour noter cet expert.
    </p>
  </div>
</template>
