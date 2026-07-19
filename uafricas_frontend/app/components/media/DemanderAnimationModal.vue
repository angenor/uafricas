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
  /** Nature du support visé — sert uniquement à formuler les libellés. */
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

const onKeydown = (e: KeyboardEvent) => {
  if (e.key === 'Escape' && props.isOpen) fermer()
}
onMounted(() => window.addEventListener('keydown', onKeydown))
onBeforeUnmount(() => window.removeEventListener('keydown', onKeydown))
</script>

<template>
  <Teleport to="body">
    <Transition name="modal-fade">
      <div
        v-if="isOpen"
        class="fixed inset-0 z-[90] flex items-center justify-center p-4"
        @click.self="fermer"
      >
        <div class="absolute inset-0 bg-black/60 backdrop-blur-sm"></div>

        <div class="relative bg-white rounded-2xl shadow-2xl w-full max-w-xl max-h-[90vh] overflow-hidden flex flex-col">
          <!-- En-tête -->
          <div class="flex items-center justify-between px-6 py-4 border-b border-gray-100 shrink-0">
            <h3 class="font-oswald text-xl font-bold text-gray-900">Demander à animer un programme</h3>
            <button
              type="button"
              class="w-9 h-9 flex items-center justify-center rounded-full text-gray-400 hover:bg-gray-100 hover:text-gray-700 transition-colors cursor-pointer"
              :disabled="chargement"
              @click="fermer"
            >
              <font-awesome-icon :icon="['fas', 'xmark']" class="w-5 h-5" />
            </button>
          </div>

          <div class="px-6 py-5 overflow-y-auto">
            <!-- Confirmation -->
            <div v-if="succes" class="flex flex-col items-center justify-center py-10 text-center">
              <div class="w-14 h-14 rounded-full bg-custom-green/10 flex items-center justify-center mb-4">
                <font-awesome-icon :icon="['fas', 'microphone']" class="w-7 h-7 text-custom-green" />
              </div>
              <p class="font-medium text-gray-900">Demande envoyée !</p>
              <p class="text-sm text-gray-500 mt-2 max-w-sm">
                Elle est <span class="font-medium text-gray-700">en attente de décision</span>
                de l’équipe de {{ nomSupport }}. Rien n’est publié tant que la décision
                n’est pas prise. Suivez son avancement depuis
                <NuxtLink to="/mon-compte/propositions-medias" class="text-custom-green hover:underline">
                  vos propositions
                </NuxtLink>.
              </p>
            </div>

            <!-- Invitation à se connecter -->
            <div v-else-if="!userStore.accessToken" class="py-10 text-center">
              <font-awesome-icon :icon="['fas', 'lock']" class="w-10 h-10 text-gray-300 mb-4" />
              <p class="text-gray-600 mb-2">Demander à animer un programme demande un compte.</p>
              <NuxtLink to="/login" class="text-custom-green font-medium hover:underline">
                Se connecter
              </NuxtLink>
            </div>

            <form v-else class="space-y-5" @submit.prevent="soumettreFormulaire">
              <!-- FR-045 : conséquence majeure d'une acceptation, affichée en évidence. -->
              <div class="rounded-xl border border-red-200 bg-linear-to-r from-red-50 to-amber-50 px-4 py-3.5">
                <div class="flex gap-3">
                  <font-awesome-icon
                    :icon="['fas', 'triangle-exclamation']"
                    class="w-5 h-5 mt-0.5 shrink-0 text-red-500"
                  />
                  <div class="text-sm text-red-900">
                    <p class="font-semibold mb-1">Cette demande engage plus qu’une émission.</p>
                    <p>
                      Si elle est acceptée, vous devenez
                      <span class="font-semibold">co-détenteur de {{ nomSupport }}</span> :
                      vous pourrez gérer les contenus de {{ libelleSupport }} et sa grille de
                      programmes, au même titre que son équipe actuelle.
                    </p>
                  </div>
                </div>
              </div>

              <!-- Nom du programme souhaité -->
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1.5">
                  Nom du programme souhaité <span class="text-red-500">*</span>
                </label>
                <input
                  v-model="nomProgramme"
                  type="text"
                  maxlength="350"
                  class="w-full px-3.5 py-2.5 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-custom-green focus:border-transparent"
                  placeholder="Ex. : Paroles d’artisans"
                >
              </div>

              <!-- Présentation du projet -->
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1.5">
                  Présentation du projet <span class="text-red-500">*</span>
                </label>
                <textarea
                  v-model="presentation"
                  rows="5"
                  class="w-full px-3.5 py-2.5 border border-gray-300 rounded-lg text-sm resize-none focus:ring-2 focus:ring-custom-green focus:border-transparent"
                  placeholder="Format, durée, rythme de diffusion, public visé, ton éditorial…"
                ></textarea>
              </div>

              <!-- Expérience / rôle du demandeur → donnees.info_animateur -->
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1.5">
                  Votre expérience et votre rôle <span class="text-red-500">*</span>
                </label>
                <textarea
                  v-model="experience"
                  rows="4"
                  class="w-full px-3.5 py-2.5 border border-gray-300 rounded-lg text-sm resize-none focus:ring-2 focus:ring-custom-green focus:border-transparent"
                  placeholder="Qui êtes-vous ? Quelle expérience d’animation ou de production avez-vous ?"
                ></textarea>
              </div>

              <!-- Justification -->
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1.5">
                  Pourquoi cette demande ? <span class="text-red-500">*</span>
                </label>
                <textarea
                  v-model="justification"
                  rows="3"
                  class="w-full px-3.5 py-2.5 border border-gray-300 rounded-lg text-sm resize-none focus:ring-2 focus:ring-custom-green focus:border-transparent"
                  placeholder="Ce mot accompagne votre demande auprès de l’équipe du support."
                ></textarea>
              </div>

              <p v-if="erreur" class="text-sm text-red-600">{{ erreur }}</p>
            </form>
          </div>

          <!-- Pied -->
          <div
            v-if="!succes && userStore.accessToken"
            class="flex items-center justify-end gap-3 px-6 py-4 border-t border-gray-100 bg-gray-50 shrink-0"
          >
            <button
              type="button"
              class="px-4 py-2 text-sm font-medium text-gray-700 rounded-lg hover:bg-gray-200 transition-colors cursor-pointer disabled:opacity-50"
              :disabled="chargement"
              @click="fermer"
            >
              Annuler
            </button>
            <button
              type="button"
              class="px-5 py-2 text-sm font-medium text-white bg-custom-green rounded-lg hover:bg-custom-green/90 transition-colors cursor-pointer disabled:opacity-60 inline-flex items-center gap-2"
              :disabled="chargement || !formulaireComplet"
              @click="soumettreFormulaire"
            >
              <font-awesome-icon v-if="chargement" :icon="['fas', 'spinner']" class="w-4 h-4 animate-spin" />
              <font-awesome-icon v-else :icon="['fas', 'microphone']" class="w-4 h-4" />
              Envoyer la demande
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: opacity 0.2s ease;
}
.modal-fade-enter-from,
.modal-fade-leave-to {
  opacity: 0;
}
</style>
