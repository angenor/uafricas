<script setup lang="ts">
/**
 * Demande d'animation d'un programme sur une chaîne ou une station
 * (US6, FR-045).
 *
 * Cette demande n'est pas anodine : une acceptation fait du demandeur un
 * **co-détenteur du support**, avec pouvoir sur ses contenus et sa grille.
 * L'avertissement est affiché en évidence, jamais replié.
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

const nomProgramme = ref('')
const presentation = ref('')
const experience = ref('')
const justification = ref('')
const erreur = ref('')
const succes = ref(false)

const libelleSupport = computed(() =>
  props.typeSupport === 'chaine_tv' ? 'cette chaîne' : 'cette station',
)

/** Les quatre champs sont obligatoires : la décision engage la gestion du support. */
const formulaireComplet = computed(() =>
  !!nomProgramme.value.trim()
  && !!presentation.value.trim()
  && !!experience.value.trim()
  && !!justification.value.trim(),
)

const reinitialiser = () => {
  nomProgramme.value = ''
  presentation.value = ''
  experience.value = ''
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
    erreur.value = 'Tous les champs marqués d’un astérisque sont requis.'
    return
  }
  erreur.value = ''

  const donnees: DonneesProposition = {
    nom: nomProgramme.value.trim(),
    description: presentation.value.trim(),
    info_animateur: experience.value.trim(),
  }

  const res = await soumettre({
    type_objet: 'animation_programme',
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
    titre="Demander à animer un programme"
    :sous-titre="`Sur ${nomSupport}`"
    icone="fa-solid fa-microphone"
    @update:model-value="fermer()"
  >
    <!-- Confirmation -->
    <div v-if="succes" class="flex flex-col items-center gap-3 py-6 text-center">
      <span class="grid size-14 place-items-center rounded-full bg-af-vert/10">
        <font-awesome-icon icon="fa-solid fa-microphone" class="text-2xl text-af-vert" />
      </span>
      <p class="text-base font-bold text-af-encre">Demande envoyée !</p>
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
      <p class="text-[14px]/[1.4] text-af-corps">Demander à animer un programme demande un compte.</p>
      <AfricansBouton vers="/login" variante="secondaire">Se connecter</AfricansBouton>
    </div>

    <form v-else id="form-animation-media" class="flex flex-col gap-5" @submit.prevent="soumettreFormulaire">
      <!-- FR-045 : conséquence majeure d'une acceptation, jamais repliée. -->
      <div class="flex gap-3 rounded-lg border border-af-live/30 bg-af-live/5 px-4 py-3.5 text-[14px]/[1.6] text-af-encre">
        <font-awesome-icon icon="fa-solid fa-triangle-exclamation" class="mt-0.5 shrink-0 text-af-live" />
        <div>
          <p class="font-bold">Cette demande engage plus qu'une émission.</p>
          <p class="mt-1">
            Si elle est acceptée, vous devenez
            <span class="font-bold">co-détenteur de {{ nomSupport }}</span> : vous pourrez
            gérer les contenus de {{ libelleSupport }} et sa grille de programmes, au même
            titre que son équipe actuelle.
          </p>
        </div>
      </div>

      <AfricansChamp
        v-model="nomProgramme"
        libelle="Nom du programme souhaité"
        :maxlength="350"
        placeholder="Ex. : Paroles d'artisans"
        obligatoire
      />

      <AfricansChamp
        v-model="presentation"
        libelle="Présentation du projet"
        type="textarea"
        :lignes="5"
        placeholder="Format, durée, rythme de diffusion, public visé, ton éditorial…"
        obligatoire
      />

      <AfricansChamp
        v-model="experience"
        libelle="Votre expérience et votre rôle"
        type="textarea"
        :lignes="4"
        placeholder="Qui êtes-vous ? Quelle expérience d'animation ou de production avez-vous ?"
        obligatoire
      />

      <AfricansChamp
        v-model="justification"
        libelle="Pourquoi cette demande ?"
        type="textarea"
        :lignes="3"
        placeholder="Ce mot accompagne votre demande auprès de l'équipe du support."
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
        form="form-animation-media"
        :desactive="chargement || !formulaireComplet"
        :tourne="chargement"
        :icone="chargement ? 'fa-solid fa-spinner' : 'fa-solid fa-microphone'"
      >
        Envoyer la demande
      </AfricansBouton>
    </template>
  </AfricansModale>
</template>
