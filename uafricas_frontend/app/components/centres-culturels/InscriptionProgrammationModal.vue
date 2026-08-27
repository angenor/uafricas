<script setup lang="ts">
import type { InscriptionProgPayload } from '~/composables/useCentresCulturels'

const props = defineProps<{
  isOpen: boolean
  loading?: boolean
  titreProgrammation?: string
  defautNom?: string
  defautPrenom?: string
}>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'submit', payload: InscriptionProgPayload): void
}>()

const { listerPays } = useCentresCulturels()

const form = reactive<InscriptionProgPayload>({
  nom: '',
  prenom: '',
  pays: '',
  lieu_residence: '',
  titre: '',
})

const pays = ref<{ id: string, nom: string }[]>([])
const erreurLocale = ref<string | null>(null)

// Pré-remplir nom/prénom depuis le compte + charger la liste des pays à l'ouverture
watch(() => props.isOpen, async (ouvert) => {
  if (ouvert) {
    erreurLocale.value = null
    form.nom = form.nom || props.defautNom || ''
    form.prenom = form.prenom || props.defautPrenom || ''
    if (pays.value.length === 0) pays.value = await listerPays()
  }
})

const soumettre = () => {
  erreurLocale.value = null
  if (!form.nom.trim() || !form.prenom.trim()) {
    erreurLocale.value = 'Le nom et le prénom sont requis.'
    return
  }
  if (!form.pays.trim() || !form.lieu_residence.trim() || !form.titre.trim()) {
    erreurLocale.value = 'Veuillez renseigner le pays, le lieu de résidence et le titre.'
    return
  }
  emit('submit', {
    nom: form.nom.trim(),
    prenom: form.prenom.trim(),
    pays: form.pays.trim(),
    lieu_residence: form.lieu_residence.trim(),
    titre: form.titre.trim(),
  })
}
</script>

<template>
  <AfricansModale
    :model-value="isOpen"
    titre="S'inscrire à la programmation"
    :sous-titre="titreProgrammation"
    icone="fa-solid fa-user-plus"
    @update:model-value="emit('close')"
  >
    <form id="form-inscription-prog" class="flex flex-col gap-5" @submit.prevent="soumettre">
      <div class="grid gap-5 sm:grid-cols-2">
        <AfricansChamp v-model="form.nom" libelle="Nom" obligatoire />
        <AfricansChamp v-model="form.prenom" libelle="Prénom" obligatoire />
      </div>

      <AfricansChamp v-model="form.pays" libelle="Pays" type="select" obligatoire>
        <option value="" disabled>Sélectionnez un pays</option>
        <option v-for="p in pays" :key="p.id" :value="p.nom">{{ p.nom }}</option>
      </AfricansChamp>

      <AfricansChamp
        v-model="form.lieu_residence"
        libelle="Lieu de résidence"
        placeholder="Ville, quartier…"
        obligatoire
      />

      <AfricansChamp
        v-model="form.titre"
        libelle="Titre"
        placeholder="Fonction, profession, titre…"
        obligatoire
      />

      <p v-if="erreurLocale" class="rounded-lg border border-af-live/20 bg-af-live/5 px-4 py-3 text-[14px]/[1.4] text-af-live">
        {{ erreurLocale }}
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
        form="form-inscription-prog"
        :desactive="loading"
        :tourne="loading"
        :icone="loading ? 'fa-solid fa-spinner' : 'fa-solid fa-user-plus'"
      >
        Confirmer l'inscription
      </AfricansBouton>
    </template>
  </AfricansModale>
</template>
