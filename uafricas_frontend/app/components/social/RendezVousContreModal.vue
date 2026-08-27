<script setup lang="ts">
import type { RendezVousAPI } from '~/composables/useRendezVous'

/** Contre-proposition de créneau pour un rendez-vous en visioconférence. */
const props = defineProps<{ rdv: RendezVousAPI }>()
const emit = defineEmits<{ (e: 'fermer'): void, (e: 'contre'): void }>()

const { contreProposer } = useRendezVous()

const DUREES = [15, 30, 45, 60] as const

const dateHeure = ref('')
const dureeMinutes = ref<number>(props.rdv.duree_minutes)
const erreur = ref('')
const envoiEnCours = ref(false)

/**
 * Borne basse du sélecteur, exprimée en heure LOCALE : `toISOString` rend de
 * l'UTC, et un `min` en UTC décalerait la borne de tout le fuseau.
 */
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
  // Le `min` du champ est une aide de saisie, pas une garantie : il se
  // contourne au clavier.
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
  <AfricansModale
    :model-value="true"
    titre="Proposer un autre créneau"
    icone="fa-solid fa-calendar-days"
    @update:model-value="!$event && emit('fermer')"
  >
    <form class="flex flex-col gap-5" @submit.prevent="soumettre">
      <p class="text-[14px]/[1.5] text-af-corps">
        Rendez-vous : <strong class="font-bold text-af-encre">{{ rdv.sujet }}</strong>
      </p>

      <label class="flex flex-col gap-2">
        <span class="text-[14px]/[1.4] text-af-atone italic">Nouvelle date et heure *</span>
        <input
          v-model="dateHeure"
          type="datetime-local"
          :min="minDateHeure"
          class="h-11 rounded-md border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:border-af-chocolat focus:outline-none"
        />
      </label>

      <div class="flex flex-col gap-2">
        <span class="text-[14px]/[1.4] text-af-atone italic">Durée</span>
        <div class="grid grid-cols-4 gap-2">
          <button
            v-for="d in DUREES"
            :key="d"
            type="button"
            class="rounded-lg border py-2 text-[14px]/[1.4] font-bold transition"
            :class="dureeMinutes === d
              ? 'border-af-chocolat bg-af-chocolat text-white'
              : 'border-af-bordure text-af-corps hover:border-af-chocolat'"
            @click="dureeMinutes = d"
          >
            {{ d }} min
          </button>
        </div>
      </div>

      <p v-if="erreur" role="alert" class="flex items-start gap-2 text-[12px]/[1.4] text-af-live">
        <font-awesome-icon icon="fa-solid fa-circle-exclamation" class="mt-0.5 shrink-0" />
        {{ erreur }}
      </p>
    </form>

    <template #actions>
      <button
        type="button"
        class="text-base font-bold text-af-corps transition hover:opacity-70"
        @click="emit('fermer')"
      >
        Annuler
      </button>
      <AfricansBouton
        :desactive="envoiEnCours"
        :tourne="envoiEnCours"
        :icone="envoiEnCours ? 'fa-solid fa-spinner' : 'fa-solid fa-paper-plane'"
        @click="soumettre"
      >
        Envoyer
      </AfricansBouton>
    </template>
  </AfricansModale>
</template>
