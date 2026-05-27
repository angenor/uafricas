<script setup lang="ts">
import type { RendezVousAPI } from '~/composables/useRendezVous'

const props = defineProps<{ rdv: RendezVousAPI }>()
const emit = defineEmits<{ (e: 'fermer'): void, (e: 'contre'): void }>()

const { contreProposer } = useRendezVous()

const DUREES = [15, 30, 45, 60] as const

const dateHeure = ref('')
const dureeMinutes = ref<number>(props.rdv.duree_minutes)
const erreur = ref('')
const envoiEnCours = ref(false)

const minDateHeure = computed(() => {
  const d = new Date()
  d.setMinutes(d.getMinutes() - d.getTimezoneOffset())
  return d.toISOString().slice(0, 16)
})

const soumettre = async () => {
  erreur.value = ''
  if (!dateHeure.value) {
    erreur.value = 'Veuillez choisir une date et une heure.'
    return
  }
  const iso = new Date(dateHeure.value).toISOString()
  if (new Date(iso).getTime() <= Date.now()) {
    erreur.value = 'La date du rendez-vous doit être dans le futur.'
    return
  }

  envoiEnCours.value = true
  try {
    await contreProposer(props.rdv.id, { date_heure: iso, duree_minutes: dureeMinutes.value })
    emit('contre')
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
    <div class="w-full max-w-sm bg-white rounded-2xl shadow-2xl overflow-hidden">
      <header class="flex items-center justify-between px-5 py-4 bg-linear-to-r from-custom-chocolat to-custom-green text-white">
        <h2 class="font-semibold flex items-center gap-2">
          <font-awesome-icon icon="fa-solid fa-calendar-days" />
          Proposer un autre créneau
        </h2>
        <button type="button" class="p-1 hover:bg-white/20 rounded-lg transition" aria-label="Fermer" @click="emit('fermer')">
          <font-awesome-icon icon="fa-solid fa-xmark" />
        </button>
      </header>

      <form class="p-5 space-y-4" @submit.prevent="soumettre">
        <p class="text-sm text-gray-500">
          Rendez-vous : <span class="font-semibold text-gray-700">{{ rdv.sujet }}</span>
        </p>

        <div>
          <label class="block text-sm font-semibold text-gray-700 mb-1" for="contre-date">Nouvelle date et heure <span class="text-red-500">*</span></label>
          <input
            id="contre-date"
            v-model="dateHeure"
            type="datetime-local"
            :min="minDateHeure"
            class="w-full px-3 py-2 border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-custom-chocolat/40"
          >
        </div>

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

        <p v-if="erreur" class="text-sm text-red-600 bg-red-50 border border-red-100 rounded-xl px-3 py-2">{{ erreur }}</p>

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
            Envoyer
          </button>
        </div>
      </form>
    </div>
  </div>
</template>
