<script setup lang="ts">
import type { MembreLightAPI } from '~/composables/useAmis'
import type { ProposerPayload } from '~/composables/useRendezVous'

const props = defineProps<{ membre: MembreLightAPI }>()
const emit = defineEmits<{ (e: 'fermer'): void, (e: 'propose'): void }>()

const { proposer } = useRendezVous()

const DUREES = [15, 30, 45, 60] as const

const sujet = ref('')
const description = ref('')
const dateHeure = ref('') // valeur d'un <input datetime-local> (heure locale)
const dureeMinutes = ref<number>(30)
const erreur = ref('')
const envoiEnCours = ref(false)

const nomComplet = computed(() => `${props.membre.prenom} ${props.membre.nom}`.trim())

// Borne minimale = maintenant (au format datetime-local : YYYY-MM-DDTHH:mm).
const minDateHeure = computed(() => {
  const d = new Date()
  d.setMinutes(d.getMinutes() - d.getTimezoneOffset())
  return d.toISOString().slice(0, 16)
})

const sujetValide = computed(() => {
  const s = sujet.value.trim()
  return s.length >= 1 && s.length <= 150
})

const soumettre = async () => {
  erreur.value = ''
  if (!sujetValide.value) {
    erreur.value = 'Le sujet est obligatoire (150 caractères maximum).'
    return
  }
  if (!dateHeure.value) {
    erreur.value = 'Veuillez choisir une date et une heure.'
    return
  }
  const iso = new Date(dateHeure.value).toISOString()
  if (new Date(iso).getTime() <= Date.now()) {
    erreur.value = 'La date du rendez-vous doit être dans le futur.'
    return
  }

  const payload: ProposerPayload = {
    destinataire_id: props.membre.id,
    sujet: sujet.value.trim(),
    description: description.value.trim() || undefined,
    date_heure: iso,
    duree_minutes: dureeMinutes.value,
  }

  envoiEnCours.value = true
  try {
    await proposer(payload)
    emit('propose')
    emit('fermer')
  }
  catch (e) {
    erreur.value = e instanceof Error ? e.message : 'Une erreur est survenue.'
  }
  finally {
    envoiEnCours.value = false
  }
}
</script>

<template>
  <div
    class="fixed inset-0 z-[60] flex items-center justify-center bg-black/50 p-4"
    @click.self="emit('fermer')"
  >
    <div class="w-full max-w-md bg-white rounded-2xl shadow-2xl overflow-hidden">
      <!-- En-tête -->
      <header class="flex items-center justify-between px-5 py-4 bg-linear-to-r from-custom-chocolat to-custom-green text-white">
        <h2 class="font-semibold flex items-center gap-2">
          <font-awesome-icon icon="fa-solid fa-video" />
          Proposer un rendez-vous
        </h2>
        <button type="button" class="p-1 hover:bg-white/20 rounded-lg transition" aria-label="Fermer" @click="emit('fermer')">
          <font-awesome-icon icon="fa-solid fa-xmark" />
        </button>
      </header>

      <form class="p-5 space-y-4" @submit.prevent="soumettre">
        <p class="text-sm text-gray-500">
          Avec <span class="font-semibold text-gray-700">{{ nomComplet }}</span>
        </p>

        <!-- Sujet -->
        <div>
          <label class="block text-sm font-semibold text-gray-700 mb-1" for="rdv-sujet">Sujet <span class="text-red-500">*</span></label>
          <input
            id="rdv-sujet"
            v-model="sujet"
            type="text"
            maxlength="150"
            placeholder="Ex. : Mentorat carrière"
            class="w-full px-3 py-2 border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-custom-chocolat/40"
          >
          <p class="text-xs text-gray-400 mt-1 text-right">{{ sujet.length }}/150</p>
        </div>

        <!-- Description -->
        <div>
          <label class="block text-sm font-semibold text-gray-700 mb-1" for="rdv-desc">Description <span class="text-gray-400 font-normal">(facultatif)</span></label>
          <textarea
            id="rdv-desc"
            v-model="description"
            rows="3"
            placeholder="Détails, objectifs, contexte…"
            class="w-full px-3 py-2 border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-custom-chocolat/40 resize-none"
          />
        </div>

        <!-- Date & heure -->
        <div>
          <label class="block text-sm font-semibold text-gray-700 mb-1" for="rdv-date">Date et heure <span class="text-red-500">*</span></label>
          <input
            id="rdv-date"
            v-model="dateHeure"
            type="datetime-local"
            :min="minDateHeure"
            class="w-full px-3 py-2 border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-custom-chocolat/40"
          >
        </div>

        <!-- Durée -->
        <div>
          <span class="block text-sm font-semibold text-gray-700 mb-2">Durée</span>
          <div class="grid grid-cols-4 gap-2">
            <button
              v-for="d in DUREES"
              :key="d"
              type="button"
              class="py-2 rounded-xl text-sm font-semibold border transition"
              :class="dureeMinutes === d
                ? 'bg-custom-chocolat text-white border-custom-chocolat'
                : 'bg-white text-gray-600 border-gray-200 hover:border-custom-chocolat/50'"
              @click="dureeMinutes = d"
            >
              {{ d }} min
            </button>
          </div>
        </div>

        <!-- Erreur -->
        <p v-if="erreur" class="text-sm text-red-600 bg-red-50 border border-red-100 rounded-xl px-3 py-2">{{ erreur }}</p>

        <!-- Actions -->
        <div class="flex justify-end gap-2 pt-1">
          <button
            type="button"
            class="px-4 py-2 rounded-xl text-sm font-semibold text-gray-500 hover:bg-gray-100 transition"
            @click="emit('fermer')"
          >
            Annuler
          </button>
          <button
            type="submit"
            :disabled="envoiEnCours"
            class="px-5 py-2 rounded-xl text-sm font-semibold text-white bg-linear-to-r from-custom-chocolat to-custom-green hover:shadow-lg transition disabled:opacity-60 disabled:cursor-not-allowed"
          >
            <font-awesome-icon v-if="envoiEnCours" icon="fa-solid fa-spinner" spin class="mr-1" />
            Proposer
          </button>
        </div>
      </form>
    </div>
  </div>
</template>
