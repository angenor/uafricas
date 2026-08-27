<script setup lang="ts">
/**
 * Proposition d'une idée de contenu à une chaîne ou une station (US6, FR-044).
 *
 * Une idée n'est **pas** un contenu : elle est adressée à l'équipe du support
 * visé, qui reste seule juge de la retenir. Même retenue, elle ne crée aucune
 * émission automatiquement : le formulaire le dit, pour ne pas laisser croire
 * à une publication.
 *
 * `target_id` est obligatoire : le CHECK `ck_prop_media_cible_requise` refuse
 * en SQL toute proposition de ce type sans support visé.
 */
import {
  useMediaProposition,
  type DonneesProposition,
} from '~/composables/useMediaProposition'

const props = defineProps<{
  isOpen: boolean
  /** Nature du support visé : sert uniquement à formuler les libellés. */
  typeSupport: 'chaine_tv' | 'station_radio'
  /** Identifiant du support destinataire (obligatoire côté serveur). */
  supportId: string
  /** Nom lisible du support, affiché à l'utilisateur. */
  nomSupport: string
}>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'soumis', proposition: unknown): void
}>()

const { soumettre, chargement, erreur: erreurApi } = useMediaProposition()
const userStore = useUserStore()

const titre = ref('')
const description = ref('')
const justification = ref('')
const erreur = ref('')
const succes = ref(false)

const libelleSupport = computed(() =>
  props.typeSupport === 'chaine_tv' ? 'la chaîne' : 'la station',
)

/** Le bouton d'envoi n'est actif qu'avec les trois champs obligatoires remplis. */
const formulaireComplet = computed(() =>
  !!titre.value.trim() && !!description.value.trim() && !!justification.value.trim(),
)

const reinitialiser = () => {
  titre.value = ''
  description.value = ''
  justification.value = ''
  erreur.value = ''
  succes.value = false
}

watch(() => props.isOpen, (ouvert) => { if (ouvert) reinitialiser() })

const fermer = () => {
  if (chargement.value) return
  emit('close')
}

const soumettreFormulaire = async () => {
  if (!formulaireComplet.value) {
    erreur.value = 'Le titre, la description et la justification sont requis.'
    return
  }
  erreur.value = ''

  const donnees: DonneesProposition = {
    nom: titre.value.trim(),
    description: description.value.trim(),
  }

  const res = await soumettre({
    type_objet: 'idee_contenu',
    target_id: props.supportId,
    justification: justification.value.trim(),
    donnees,
  })

  if (res) {
    succes.value = true
    emit('soumis', res)
  }
  else {
    // Le message du serveur est repris tel quel : il porte les motifs métier.
    erreur.value = erreurApi.value || 'Erreur lors de l’envoi. Veuillez réessayer.'
  }
}
</script>

<template>
  <AfricansModale
    :model-value="isOpen"
    titre="Proposer une idée de contenu"
    :sous-titre="`À l'équipe de ${nomSupport}`"
    icone="fa-solid fa-lightbulb"
    @update:model-value="fermer()"
  >
    <!-- Confirmation -->
    <div v-if="succes" class="flex flex-col items-center gap-3 py-6 text-center">
      <span class="grid size-14 place-items-center rounded-full bg-af-vert/10">
        <font-awesome-icon icon="fa-solid fa-lightbulb" class="text-2xl text-af-vert" />
      </span>
      <p class="text-base font-bold text-af-encre">Idée transmise !</p>
      <p class="max-w-sm text-[14px]/[1.6] text-af-corps">
        Elle est <span class="font-bold">en attente de décision</span> de l'équipe de
        {{ nomSupport }}. Rien n'est publié tant que la décision n'est pas prise.
        Suivez son avancement depuis
        <NuxtLink to="/mon-compte/propositions-medias" class="font-bold text-af-chocolat underline">
          vos propositions
        </NuxtLink>.
      </p>
    </div>

    <!-- Invitation à se connecter -->
    <div v-else-if="!userStore.accessToken" class="flex flex-col items-center gap-3 py-8 text-center">
      <font-awesome-icon icon="fa-solid fa-lock" class="text-3xl text-af-atone-2" />
      <p class="text-[14px]/[1.4] text-af-corps">Proposer une idée demande un compte.</p>
      <AfricansBouton vers="/login" variante="secondaire">Se connecter</AfricansBouton>
    </div>

    <form v-else id="form-idee-media" class="flex flex-col gap-5" @submit.prevent="soumettreFormulaire">
      <!-- Ce que fait, et ne fait pas, une idée retenue. -->
      <p class="flex gap-3 rounded-lg border border-af-chocolat/20 bg-af-chocolat/5 px-4 py-3 text-[14px]/[1.6] text-af-corps">
        <font-awesome-icon icon="fa-solid fa-circle-info" class="mt-1 shrink-0 text-af-chocolat" />
        <span>
          Votre idée est adressée à l'équipe de
          <span class="font-bold">{{ nomSupport }}</span>, qui décide seule de la retenir.
          Une idée retenue ne crée aucun contenu automatiquement : elle nourrit la
          réflexion éditoriale de {{ libelleSupport }}.
        </span>
      </p>

      <AfricansChamp
        v-model="titre"
        libelle="Titre de l'idée"
        :maxlength="350"
        placeholder="Ex. : Une chronique sur les innovations simples chez nous"
        obligatoire
      />

      <AfricansChamp
        v-model="description"
        libelle="Description détaillée"
        type="textarea"
        :lignes="5"
        placeholder="Décrivez le sujet, le format envisagé, le public visé…"
        obligatoire
      />

      <AfricansChamp
        v-model="justification"
        libelle="Pourquoi cette idée ?"
        type="textarea"
        :lignes="3"
        placeholder="Ce mot accompagne votre idée auprès de l'équipe éditoriale."
        obligatoire
      />

      <p v-if="erreur" class="rounded-lg border border-af-live/20 bg-af-live/5 px-4 py-3 text-[14px]/[1.4] text-af-live">
        {{ erreur }}
      </p>
    </form>

    <template v-if="!succes && userStore.accessToken" #actions>
      <button
        type="button"
        class="text-base font-bold text-af-corps transition hover:opacity-70 disabled:opacity-50"
        :disabled="chargement"
        @click="fermer"
      >
        Annuler
      </button>
      <AfricansBouton
        type="submit"
        form="form-idee-media"
        :desactive="chargement || !formulaireComplet"
        :tourne="chargement"
        :icone="chargement ? 'fa-solid fa-spinner' : 'fa-solid fa-lightbulb'"
      >
        Envoyer l'idée
      </AfricansBouton>
    </template>
  </AfricansModale>
</template>
