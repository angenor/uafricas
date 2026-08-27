<template>
  <AfricansModale
    :model-value="open"
    titre="Je candidate à ce programme"
    icone="fa-solid fa-paper-plane"
    taille="large"
    @update:model-value="emit('close')"
  >
    <form id="form-candidature-sabbatique" class="flex flex-col gap-5" @submit.prevent="soumettre">
      <!-- Condition d'éligibilité : elle décide de la recevabilité, elle est
           donc énoncée AVANT le premier champ, jamais en pied. -->
      <p class="flex gap-3 rounded-lg border border-af-chocolat/20 bg-af-chocolat/5 px-4 py-3 text-[14px]/[1.6] text-af-corps">
        <font-awesome-icon icon="fa-solid fa-circle-info" class="mt-1 shrink-0 text-af-chocolat" />
        <span>
          Condition : vous devez être <strong class="font-bold">en emploi</strong> ou
          <strong class="font-bold">retraité(e)</strong>. Les personnes sans emploi ne sont
          pas éligibles.
        </span>
      </p>

      <!-- Statut d'emploi -->
      <fieldset>
        <legend class="mb-2 text-[14px]/[1.4] text-af-atone italic">
          Votre situation <span class="not-italic text-af-live">*</span>
        </legend>
        <div class="flex gap-3">
          <label v-for="statut in STATUTS_EMPLOI" :key="statut.value" class="flex-1 cursor-pointer">
            <input v-model="form.statutEmploi" type="radio" name="statut-emploi" :value="statut.value" class="sr-only peer" />
            <span
              class="block rounded-lg border-2 px-3 py-2.5 text-center text-[14px]/[1.4] font-bold transition peer-focus-visible:outline-2 peer-focus-visible:outline-offset-2 peer-focus-visible:outline-af-chocolat"
              :class="form.statutEmploi === statut.value
                ? 'border-af-vert bg-af-vert/10 text-af-vert'
                : 'border-af-bordure text-af-atone hover:border-af-chocolat'"
            >
              {{ statut.label }}
            </span>
          </label>
        </div>
      </fieldset>

      <AfricansChamp
        v-model="form.nomEtatCivil"
        libelle="Nom et prénoms à l'état civil"
        placeholder="Ex: Aminata Diallo"
        obligatoire
      />

      <div class="grid gap-5 md:grid-cols-2">
        <AfricansChamp
          v-model="form.fonctionActuelle"
          libelle="Fonction actuelle"
          placeholder="Ex: Ingénieure agronome"
          obligatoire
        />
        <AfricansChamp
          v-model="form.lieuResidence"
          libelle="Lieu de résidence ou de fonction"
          placeholder="Ex: Dakar, Sénégal"
          obligatoire
        />
      </div>

      <!-- Adéquation au profil -->
      <fieldset>
        <legend class="mb-2 text-[14px]/[1.4] text-af-atone italic">
          Répondez-vous entièrement au profil et à la spécialisation requis par le projet ?
          <span class="not-italic text-af-live">*</span>
        </legend>
        <div class="flex gap-3">
          <label class="flex-1 cursor-pointer">
            <input v-model="repondProfil" type="radio" name="repond-profil" :value="true" class="sr-only peer" />
            <span
              class="block rounded-lg border-2 px-3 py-2.5 text-center text-[14px]/[1.4] font-bold transition peer-focus-visible:outline-2 peer-focus-visible:outline-offset-2 peer-focus-visible:outline-af-chocolat"
              :class="repondProfil === true
                ? 'border-af-vert bg-af-vert/10 text-af-vert'
                : 'border-af-bordure text-af-atone hover:border-af-chocolat'"
            >
              Oui, entièrement
            </span>
          </label>
          <label class="flex-1 cursor-pointer">
            <input v-model="repondProfil" type="radio" name="repond-profil" :value="false" class="sr-only peer" />
            <span
              class="block rounded-lg border-2 px-3 py-2.5 text-center text-[14px]/[1.4] font-bold transition peer-focus-visible:outline-2 peer-focus-visible:outline-offset-2 peer-focus-visible:outline-af-chocolat"
              :class="repondProfil === false
                ? 'border-af-chocolat bg-af-chocolat/10 text-af-chocolat'
                : 'border-af-bordure text-af-atone hover:border-af-chocolat'"
            >
              Partiellement
            </span>
          </label>
        </div>
      </fieldset>

      <!-- Justificatif : l'UN des deux suffit, le cadre le dit. -->
      <div class="flex flex-col gap-4 rounded-lg border border-af-vert/30 bg-af-vert/5 p-4">
        <p class="text-[14px]/[1.4] font-bold text-af-vert">
          Justificatif <span class="text-af-live">*</span>
          <span class="font-normal text-af-atone">(CV ou lien vers votre compte expertise)</span>
        </p>

        <div class="flex flex-col gap-2">
          <label for="cand-cv" class="text-[12px]/[1.4] text-af-corps">CV (PDF)</label>
          <input
            id="cand-cv"
            type="file"
            accept=".pdf"
            class="w-full text-[14px]/[1.4] text-af-corps file:mr-3 file:rounded-md file:border-0 file:bg-af-vert file:px-3 file:py-1.5 file:text-[14px] file:text-white hover:file:opacity-90"
            @change="handleCvChange"
          />
        </div>

        <AfricansChamp
          v-model="form.lienExpertise"
          libelle="Ou lien vers votre compte expertise"
          type="url"
          placeholder="https://africans-world.org/profil/…"
        />
      </div>

      <AfricansChamp
        v-model="form.lettreMotivation"
        libelle="Lettre de motivation"
        type="textarea"
        :lignes="4"
        placeholder="Présentez votre motivation pour ce programme…"
        obligatoire
      />

      <p
        v-if="erreur"
        class="flex items-center gap-2 rounded-lg border border-af-live/20 bg-af-live/5 px-4 py-3 text-[14px]/[1.4] text-af-live"
      >
        <font-awesome-icon icon="fa-solid fa-circle-exclamation" />
        {{ erreur }}
      </p>
    </form>

    <template #actions>
      <button
        type="button"
        class="text-base font-bold text-af-corps transition hover:opacity-70"
        @click="emit('close')"
      >
        Annuler
      </button>
      <AfricansBouton
        type="submit"
        form="form-candidature-sabbatique"
        :desactive="!isValid || loading"
        :tourne="loading"
        :icone="loading ? 'fa-solid fa-spinner' : 'fa-solid fa-paper-plane'"
      >
        {{ loading ? 'Envoi en cours…' : 'Envoyer ma candidature' }}
      </AfricansBouton>
    </template>
  </AfricansModale>
</template>

<script setup lang="ts">
import { reactive, ref, computed, watch } from 'vue'
import {
  useSabbatiques,
  STATUTS_EMPLOI,
  type CandidatureForm,
} from '~/composables/useSabbatiques'
import { useExperts } from '~/composables/useExperts'
import { useUserStore } from '~/stores/user'

const props = defineProps<{
  programmeId: string
  open: boolean
}>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'success'): void
}>()

const { candidater } = useSabbatiques()
const { obtenirMaCandidature } = useExperts()
const userStore = useUserStore()

const form = reactive<Omit<CandidatureForm, 'repondProfil'>>({
  nomEtatCivil: '',
  fonctionActuelle: '',
  lieuResidence: '',
  statutEmploi: '',
  lettreMotivation: '',
  lienExpertise: '',
})
const repondProfil = ref<boolean | null>(null)
const cvFile = ref<File | null>(null)
const loading = ref(false)
const erreur = ref<string | null>(null)

const handleCvChange = (event: Event) => {
  const target = event.target as HTMLInputElement
  cvFile.value = target.files && target.files[0] ? target.files[0] : null
}

// Pré-remplir le lien d'expertise si l'utilisateur connecté possède un
// compte d'expertise validé (lien vers son profil unifié).
const prechargerLienExpertise = async () => {
  if (!userStore.isAuthenticated || !userStore.user) return
  // Ne pas écraser une saisie manuelle existante
  if (form.lienExpertise?.trim()) return

  const candidature = await obtenirMaCandidature()
  if (candidature?.statut === 'valide') {
    const origine = import.meta.client ? window.location.origin : ''
    form.lienExpertise = `${origine}/profil/${userStore.user.id}`
  }
}

// Charger automatiquement à l'ouverture de la modale
watch(
  () => props.open,
  (ouvert) => {
    if (ouvert) prechargerLienExpertise()
  },
  { immediate: true },
)

const isValid = computed(() => {
  return !!form.statutEmploi &&
    form.nomEtatCivil.trim() &&
    form.fonctionActuelle.trim() &&
    form.lieuResidence.trim() &&
    !!form.lettreMotivation?.trim() &&
    repondProfil.value !== null &&
    (cvFile.value !== null || !!form.lienExpertise?.trim())
})

const soumettre = async () => {
  if (!isValid.value) return
  loading.value = true
  erreur.value = null

  const ok = await candidater(
    props.programmeId,
    { ...form, repondProfil: repondProfil.value === true },
    cvFile.value,
  )

  loading.value = false
  if (ok) {
    emit('success')
  } else {
    erreur.value = 'Impossible d\'envoyer votre candidature. Vérifiez les champs et réessayez.'
  }
}
</script>
