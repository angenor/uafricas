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
  <!-- Montée par `v-if` chez l'appelant : le `model-value` est donc toujours
       vrai, et c'est la fermeture de la coque qui remonte `fermer`. -->
  <AfricansModale
    :model-value="true"
    titre="Proposer un rendez-vous"
    :sous-titre="`Avec ${nomComplet}`"
    icone="fa-solid fa-video"
    @update:model-value="emit('fermer')"
  >
    <form id="form-rdv" class="flex flex-col gap-5" @submit.prevent="soumettre">
      <div>
        <AfricansChamp
          v-model="sujet"
          libelle="Sujet"
          placeholder="Ex. : Mentorat carrière"
          :maxlength="150"
          obligatoire
        />
        <p class="mt-1 text-right text-[12px] text-af-atone-2">{{ sujet.length }}/150</p>
      </div>

      <AfricansChamp
        v-model="description"
        libelle="Description"
        type="textarea"
        placeholder="Détails, objectifs, contexte…"
        aide="Facultatif"
      />

      <!-- `datetime-local` n'est pas un type d'AfricansChamp, qui n'émet que
           des chaînes de saisie libre : le champ natif est conservé, habillé
           aux mêmes jetons. -->
      <div>
        <label class="mb-2 block text-base font-bold text-af-encre" for="rdv-date">
          Date et heure <span class="text-af-live">*</span>
        </label>
        <input
          id="rdv-date"
          v-model="dateHeure"
          type="datetime-local"
          :min="minDateHeure"
          class="w-full rounded-lg border border-af-bordure bg-white px-4 py-3 text-base text-af-encre transition focus:border-af-chocolat focus:outline-none"
        >
      </div>

      <div>
        <span class="mb-2 block text-base font-bold text-af-encre">Durée</span>
        <div class="grid grid-cols-4 gap-2">
          <button
            v-for="d in DUREES"
            :key="d"
            type="button"
            class="rounded-lg border py-2.5 text-[14px]/[1.4] font-bold transition"
            :class="dureeMinutes === d
              ? 'border-af-chocolat bg-af-chocolat text-white'
              : 'border-af-bordure bg-white text-af-corps hover:border-af-chocolat'"
            @click="dureeMinutes = d"
          >
            {{ d }} min
          </button>
        </div>
      </div>

      <p v-if="erreur" class="rounded-lg border border-af-live/20 bg-af-live/5 px-4 py-3 text-[14px]/[1.4] text-af-live">
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
        type="submit"
        form="form-rdv"
        :desactive="envoiEnCours"
        :tourne="envoiEnCours"
        icone="fa-solid fa-video"
      >
        Proposer
      </AfricansBouton>
    </template>
  </AfricansModale>
</template>
