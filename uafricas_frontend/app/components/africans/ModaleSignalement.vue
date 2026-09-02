<script setup lang="ts">
/**
 * Modale de signalement — la coquille commune des quatre de la plateforme.
 *
 * Profil, contribution Afripulse, contenu média et session Afrolang portaient
 * chacune la même mécanique : une liste de motifs, une description bornée à
 * 1 000 signes, un état d'envoi piloté par la page, une confirmation qui se
 * referme d'elle-même. Seuls les MOTIFS et la phrase d'accroche diffèrent.
 *
 * Le contrat des appelantes est CONSERVÉ : `isOpen` / `close` /
 * `submit({ motif, description })`, et les trois méthodes exposées.
 */
const props = defineProps<{
  isOpen: boolean
  titre: string
  motifs: { value: string, label: string }[]
}>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'submit', payload: { motif: string, description: string }): void
}>()

const MAX = 1000

const motif = ref('')
const description = ref('')
const enCours = ref(false)
const erreur = ref('')
const succes = ref(false)
const messageSucces = ref('')

const restant = computed(() => MAX - description.value.length)

watch(() => props.isOpen, (ouvert) => {
  if (!ouvert) return
  motif.value = ''
  description.value = ''
  erreur.value = ''
  succes.value = false
  messageSucces.value = ''
  enCours.value = false
})

const fermer = () => {
  if (enCours.value) return
  emit('close')
}

const soumettre = () => {
  if (enCours.value) return
  // Le motif est la seule chose exigée : c'est lui qui oriente la modération,
  // la description ne fait que l'éclairer.
  if (!motif.value) {
    erreur.value = 'Veuillez sélectionner un motif.'
    return
  }
  erreur.value = ''
  emit('submit', { motif: motif.value, description: description.value.trim() })
}

defineExpose({
  setLoading: (v: boolean) => { enCours.value = v },
  setError: (msg: string) => { enCours.value = false; erreur.value = msg },
  setSuccess: (message: string) => {
    enCours.value = false
    succes.value = true
    messageSucces.value = message
    // La modale se referme d'elle-même : le membre n'a rien à confirmer de
    // plus, et rester devant un message fixe suggère qu'il reste une étape.
    setTimeout(() => emit('close'), 1800)
  },
})
</script>

<template>
  <AfricansModale
    :model-value="isOpen"
    :titre="titre"
    icone="fa-solid fa-flag"
    @update:model-value="!$event && fermer()"
  >
    <div v-if="succes" class="flex flex-col items-center gap-3 py-8 text-center">
      <font-awesome-icon icon="fa-solid fa-circle-check" class="text-4xl text-af-vert" />
      <p class="max-w-sm text-[14px]/[1.4] text-af-corps">{{ messageSucces }}</p>
    </div>

    <div v-else class="flex flex-col gap-5">
      <p class="text-[14px]/[1.5] text-af-corps"><slot /></p>

      <fieldset class="flex flex-col gap-2">
        <legend class="mb-2 text-[14px]/[1.4] text-af-atone italic">Motif du signalement *</legend>
        <label
          v-for="m in motifs"
          :key="m.value"
          class="flex cursor-pointer items-center gap-3 rounded-lg border px-4 py-2.5 text-[14px]/[1.4] transition"
          :class="motif === m.value ? 'border-af-chocolat bg-af-chocolat/[0.07] font-bold text-af-chocolat' : 'border-af-bordure text-af-corps hover:border-af-chocolat'"
        >
          <input v-model="motif" type="radio" :value="m.value" :disabled="enCours" class="accent-af-chocolat" />
          {{ m.label }}
        </label>
      </fieldset>

      <div class="flex flex-col gap-2">
        <label for="description-signalement" class="text-[14px]/[1.4] text-af-atone italic">
          Précisions (facultatives)
        </label>
        <textarea
          id="description-signalement"
          v-model="description"
          rows="4"
          :maxlength="MAX"
          :disabled="enCours"
          placeholder="Ce qui vous a alerté, et où le constater."
          class="rounded-md border border-af-bordure bg-white px-4 py-3 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:border-af-chocolat focus:outline-none disabled:opacity-50"
        />
        <div class="flex items-start justify-between gap-3">
          <p v-if="erreur" role="alert" class="text-[12px]/[1.4] text-af-live">{{ erreur }}</p>
          <span v-else />
          <span class="shrink-0 text-[12px]/[1.4]" :class="restant < 0 ? 'text-af-live' : 'text-af-atone'">
            {{ restant }}
          </span>
        </div>
      </div>
    </div>

    <template v-if="!succes" #actions>
      <button
        type="button"
        :disabled="enCours"
        class="text-base font-bold text-af-corps transition hover:opacity-70 disabled:opacity-50"
        @click="fermer"
      >
        Annuler
      </button>
      <!-- Bouton rouge : signaler engage, et la couleur le dit. -->
      <button
        type="button"
        :disabled="enCours"
        class="inline-flex h-10 items-center gap-2 rounded-lg bg-af-live px-6 text-base font-bold text-white transition hover:opacity-90 disabled:opacity-50"
        @click="soumettre"
      >
        <font-awesome-icon :icon="enCours ? 'fa-solid fa-spinner' : 'fa-solid fa-flag'" :class="enCours && 'animate-spin'" />
        {{ enCours ? 'Envoi…' : 'Signaler' }}
      </button>
    </template>
  </AfricansModale>
</template>
