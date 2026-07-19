<script setup lang="ts">
/**
 * Proposition d'une idée de contenu à une chaîne ou une station (US6, FR-044).
 *
 * Une idée n'est **pas** un contenu : elle est adressée à l'équipe du support
 * visé, qui reste seule juge de la retenir. Même retenue, elle ne crée aucune
 * émission automatiquement — le formulaire le dit, pour ne pas laisser croire
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
            <h3 class="font-oswald text-xl font-bold text-gray-900">Proposer une idée de contenu</h3>
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
                <font-awesome-icon :icon="['fas', 'lightbulb']" class="w-7 h-7 text-custom-green" />
              </div>
              <p class="font-medium text-gray-900">Idée transmise !</p>
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
              <p class="text-gray-600 mb-2">Proposer une idée demande un compte.</p>
              <NuxtLink to="/login" class="text-custom-green font-medium hover:underline">
                Se connecter
              </NuxtLink>
            </div>

            <form v-else class="space-y-5" @submit.prevent="soumettreFormulaire">
              <!-- Ce que fait — et ne fait pas — une idée retenue. -->
              <p class="flex gap-3 rounded-lg bg-amber-50 border border-amber-200 px-4 py-3 text-sm text-amber-900">
                <font-awesome-icon :icon="['fas', 'circle-info']" class="w-4 h-4 mt-0.5 shrink-0" />
                <span>
                  Votre idée est adressée à l’équipe de
                  <span class="font-semibold">{{ nomSupport }}</span>, qui décide seule
                  de la retenir. Une idée retenue ne crée aucun contenu automatiquement :
                  elle nourrit la réflexion éditoriale de {{ libelleSupport }}.
                </span>
              </p>

              <!-- Titre de l'idée -->
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1.5">
                  Titre de l’idée <span class="text-red-500">*</span>
                </label>
                <input
                  v-model="titre"
                  type="text"
                  maxlength="350"
                  class="w-full px-3.5 py-2.5 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-custom-green focus:border-transparent"
                  placeholder="Ex. : Une chronique sur les innovations simples chez nous"
                >
              </div>

              <!-- Description détaillée -->
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1.5">
                  Description détaillée <span class="text-red-500">*</span>
                </label>
                <textarea
                  v-model="description"
                  rows="5"
                  class="w-full px-3.5 py-2.5 border border-gray-300 rounded-lg text-sm resize-none focus:ring-2 focus:ring-custom-green focus:border-transparent"
                  placeholder="Décrivez le sujet, le format envisagé, le public visé…"
                ></textarea>
              </div>

              <!-- Justification -->
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1.5">
                  Pourquoi cette idée ? <span class="text-red-500">*</span>
                </label>
                <textarea
                  v-model="justification"
                  rows="3"
                  class="w-full px-3.5 py-2.5 border border-gray-300 rounded-lg text-sm resize-none focus:ring-2 focus:ring-custom-green focus:border-transparent"
                  placeholder="Ce mot accompagne votre idée auprès de l’équipe éditoriale."
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
              <font-awesome-icon v-else :icon="['fas', 'lightbulb']" class="w-4 h-4" />
              Envoyer l’idée
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
