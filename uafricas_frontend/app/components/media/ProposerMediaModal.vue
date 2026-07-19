<script setup lang="ts">
/**
 * Soumission d'un média par une partie prenante (US4).
 *
 * Remplace `AddProgramModal.vue`, maquette morte dont le `handleSubmit` se
 * contentait de simuler l'envoi.
 *
 * **Rien de ce qui est saisi ici n'atteint le public directement** : la
 * proposition part en `'en_attente'` et n'existe dans les tables métier
 * qu'après validation par un administrateur (FR-031). Le formulaire le dit
 * explicitement, pour ne pas laisser croire à une publication immédiate.
 */
import {
  useMediaProposition,
  ROLES_PARTIE_PRENANTE,
  LIBELLES_TYPE_OBJET,
  type TypeObjetPropose,
  type DonneesProposition,
} from '~/composables/useMediaProposition'

const props = withDefaults(defineProps<{
  isOpen: boolean
  /** Types offerts par la page appelante (la page Télé n'offre pas la radio). */
  typesOfferts?: TypeObjetPropose[]
  /** Support visé, quand la proposition part d'une page de détail. */
  targetId?: string | null
}>(), {
  typesOfferts: () => ['chaine_tv', 'station_radio', 'programme_tele', 'programme_radio'],
  targetId: null,
})

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'soumise'): void
}>()

const { soumettre, chargement, erreur: erreurApi } = useMediaProposition()
const userStore = useUserStore()

const typeObjet = ref<TypeObjetPropose>(props.typesOfferts[0] ?? 'chaine_tv')
const justification = ref('')
const donnees = ref<DonneesProposition>({})
const fichierMedia = ref<File | null>(null)
const fichierImage = ref<File | null>(null)
const lienExterne = ref('')
const erreur = ref('')
const succes = ref(false)

const estSupport = computed(() => ['chaine_tv', 'station_radio'].includes(typeObjet.value))
const estContenu = computed(() => typeObjet.value.startsWith('programme_'))
const estVideo = computed(() => typeObjet.value === 'programme_tele')

const reinitialiser = () => {
  typeObjet.value = props.typesOfferts[0] ?? 'chaine_tv'
  justification.value = ''
  donnees.value = {}
  fichierMedia.value = null
  fichierImage.value = null
  lienExterne.value = ''
  erreur.value = ''
  succes.value = false
}

watch(() => props.isOpen, (ouvert) => { if (ouvert) reinitialiser() })

// Changer de type invalide les champs propres à l'ancien.
watch(typeObjet, () => {
  donnees.value = {}
  fichierMedia.value = null
  lienExterne.value = ''
})

const surFichierMedia = (e: Event) => {
  fichierMedia.value = (e.target as HTMLInputElement).files?.[0] ?? null
  if (fichierMedia.value) lienExterne.value = ''
}
const surFichierImage = (e: Event) => {
  fichierImage.value = (e.target as HTMLInputElement).files?.[0] ?? null
}

const fermer = () => {
  if (chargement.value) return
  emit('close')
}

/**
 * Validation locale, doublée côté serveur ET par des CHECK SQL : ce qui suit
 * ne sert qu'à donner un message immédiat, jamais de garantie.
 */
const valider = (): string | null => {
  if (!donnees.value.nom?.trim()) return 'Le nom est requis.'
  if (!justification.value.trim()) {
    return 'Expliquez en quelques mots pourquoi vous proposez ce contenu.'
  }
  if (estSupport.value) {
    if (!donnees.value.role_partie_prenante) {
      return 'Indiquez à quel titre vous proposez ce média.'
    }
    if (
      donnees.value.role_partie_prenante === 'autre'
      && !donnees.value.role_partie_prenante_autre?.trim()
    ) {
      return 'Précisez le rôle choisi au titre de « Autre ».'
    }
  }
  if (estContenu.value && !donnees.value.theme_phare_autre?.trim() && !donnees.value.theme_phare_id) {
    return 'Indiquez le thème phare du contenu.'
  }
  return null
}

const soumettreFormulaire = async () => {
  const probleme = valider()
  if (probleme) {
    erreur.value = probleme
    return
  }
  erreur.value = ''

  // Un lien externe et un fichier ne coexistent pas pour le même média.
  const payload: DonneesProposition = { ...donnees.value }
  if (lienExterne.value.trim() && !fichierMedia.value) {
    if (estVideo.value) payload.video_url = lienExterne.value.trim()
    else if (estContenu.value) payload.audio_url = lienExterne.value.trim()
    else payload.stream_url = lienExterne.value.trim()
  }

  const res = await soumettre({
    type_objet: typeObjet.value,
    target_id: props.targetId,
    justification: justification.value.trim(),
    donnees: payload,
    media: fichierMedia.value,
    image: fichierImage.value,
  })

  if (res) {
    succes.value = true
    emit('soumise')
    setTimeout(() => emit('close'), 2200)
  }
  else {
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

        <div class="relative bg-white rounded-2xl shadow-2xl w-full max-w-2xl max-h-[90vh] overflow-hidden flex flex-col">
          <!-- En-tête -->
          <div class="flex items-center justify-between px-6 py-4 border-b border-gray-100 shrink-0">
            <h3 class="font-oswald text-xl font-bold text-gray-900">Proposer un contenu</h3>
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
                <font-awesome-icon :icon="['fas', 'check']" class="w-7 h-7 text-custom-green" />
              </div>
              <p class="font-medium text-gray-900">Proposition envoyée !</p>
              <p class="text-sm text-gray-500 mt-2 max-w-sm">
                Elle sera examinée par un administrateur avant publication. Vous pouvez
                en suivre l’avancement depuis
                <NuxtLink to="/mon-compte/propositions-medias" class="text-custom-green hover:underline">
                  vos propositions
                </NuxtLink>.
              </p>
            </div>

            <!-- Invitation à se connecter -->
            <div v-else-if="!userStore.accessToken" class="py-10 text-center">
              <font-awesome-icon :icon="['fas', 'lock']" class="w-10 h-10 text-gray-300 mb-4" />
              <p class="text-gray-600 mb-2">Proposer un contenu demande un compte.</p>
              <NuxtLink to="/login" class="text-custom-green font-medium hover:underline">
                Se connecter
              </NuxtLink>
            </div>

            <form v-else class="space-y-5" @submit.prevent="soumettreFormulaire">
              <!-- Rien de non validé n'est public : le dire d'emblée. -->
              <p class="flex gap-3 rounded-lg bg-amber-50 border border-amber-200 px-4 py-3 text-sm text-amber-900">
                <font-awesome-icon :icon="['fas', 'circle-info']" class="w-4 h-4 mt-0.5 shrink-0" />
                <span>
                  Votre proposition est examinée par un administrateur avant toute
                  publication. Elle n’apparaîtra sur le site qu’une fois validée.
                </span>
              </p>

              <!-- Type -->
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1.5">
                  Que proposez-vous ? <span class="text-red-500">*</span>
                </label>
                <select
                  v-model="typeObjet"
                  class="w-full px-3.5 py-2.5 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-custom-green focus:border-transparent"
                >
                  <option v-for="t in typesOfferts" :key="t" :value="t">
                    {{ LIBELLES_TYPE_OBJET[t] }}
                  </option>
                </select>
              </div>

              <!-- Nom -->
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1.5">
                  Nom <span class="text-red-500">*</span>
                </label>
                <input
                  v-model="donnees.nom"
                  type="text"
                  maxlength="350"
                  class="w-full px-3.5 py-2.5 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-custom-green focus:border-transparent"
                  placeholder="Nom de la chaîne, de la station ou de l’émission"
                >
              </div>

              <!-- Description -->
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1.5">Description</label>
                <textarea
                  v-model="donnees.description"
                  rows="3"
                  class="w-full px-3.5 py-2.5 border border-gray-300 rounded-lg text-sm resize-none focus:ring-2 focus:ring-custom-green focus:border-transparent"
                  placeholder="Présentez le contenu en quelques phrases…"
                ></textarea>
              </div>

              <!-- Rôle de partie prenante (supports) — FR-029 -->
              <div v-if="estSupport">
                <label class="block text-sm font-medium text-gray-700 mb-1.5">
                  À quel titre proposez-vous ce média ? <span class="text-red-500">*</span>
                </label>
                <select
                  v-model="donnees.role_partie_prenante"
                  class="w-full px-3.5 py-2.5 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-custom-green focus:border-transparent"
                >
                  <option value="">Choisir…</option>
                  <option v-for="r in ROLES_PARTIE_PRENANTE" :key="r.valeur" :value="r.valeur">
                    {{ r.libelle }}
                  </option>
                </select>
                <input
                  v-if="donnees.role_partie_prenante === 'autre'"
                  v-model="donnees.role_partie_prenante_autre"
                  type="text"
                  maxlength="200"
                  class="mt-2 w-full px-3.5 py-2.5 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-custom-green focus:border-transparent"
                  placeholder="Précisez votre rôle"
                >
              </div>

              <!-- Thème phare (contenus) — FR-030 -->
              <div v-if="estContenu">
                <label class="block text-sm font-medium text-gray-700 mb-1.5">
                  Thème phare <span class="text-red-500">*</span>
                </label>
                <input
                  v-model="donnees.theme_phare_autre"
                  type="text"
                  maxlength="200"
                  class="w-full px-3.5 py-2.5 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-custom-green focus:border-transparent"
                  placeholder="Ex. : Afrique Société, Innovations simples chez nous…"
                >
              </div>

              <!-- Média : fichier OU lien, jamais les deux (FR-056) -->
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1.5">
                  {{ estVideo ? 'Vidéo' : estContenu ? 'Audio' : 'Flux de diffusion' }}
                </label>
                <input
                  v-if="estContenu"
                  type="file"
                  :accept="estVideo ? 'video/*' : 'audio/*'"
                  class="w-full text-sm text-gray-600 file:mr-3 file:rounded-lg file:border-0 file:bg-gray-100 file:px-4 file:py-2 file:text-sm file:font-medium hover:file:bg-gray-200"
                  @change="surFichierMedia"
                >
                <input
                  v-model="lienExterne"
                  type="url"
                  :disabled="!!fichierMedia"
                  class="mt-2 w-full px-3.5 py-2.5 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-custom-green focus:border-transparent disabled:bg-gray-100"
                  placeholder="…ou collez un lien (YouTube, flux en ligne…)"
                >
                <p v-if="fichierMedia" class="mt-1 text-xs text-gray-500">
                  Fichier choisi : {{ fichierMedia.name }} — le champ de lien est désactivé.
                </p>
              </div>

              <!-- Image de couverture -->
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1.5">Image de couverture</label>
                <input
                  type="file"
                  accept="image/*"
                  class="w-full text-sm text-gray-600 file:mr-3 file:rounded-lg file:border-0 file:bg-gray-100 file:px-4 file:py-2 file:text-sm file:font-medium hover:file:bg-gray-200"
                  @change="surFichierImage"
                >
              </div>

              <!-- Source et auteur : aucune décharge de droits n'est recueillie
                   (H-012), l'administrateur se prononce seul sur la licéité. -->
              <div class="grid sm:grid-cols-2 gap-4">
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-1.5">Source du média</label>
                  <input
                    v-model="donnees.source_declaree"
                    type="text"
                    maxlength="300"
                    class="w-full px-3.5 py-2.5 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-custom-green focus:border-transparent"
                    placeholder="D’où provient ce contenu ?"
                  >
                </div>
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-1.5">Auteur du contenu</label>
                  <input
                    v-model="donnees.auteur_declare"
                    type="text"
                    maxlength="300"
                    class="w-full px-3.5 py-2.5 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-custom-green focus:border-transparent"
                    placeholder="Qui l’a réalisé ?"
                  >
                </div>
              </div>

              <!-- Justification -->
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1.5">
                  Pourquoi proposez-vous ce contenu ? <span class="text-red-500">*</span>
                </label>
                <textarea
                  v-model="justification"
                  rows="3"
                  class="w-full px-3.5 py-2.5 border border-gray-300 rounded-lg text-sm resize-none focus:ring-2 focus:ring-custom-green focus:border-transparent"
                  placeholder="Ce mot accompagne votre proposition auprès de l’administrateur."
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
              :disabled="chargement"
              @click="soumettreFormulaire"
            >
              <font-awesome-icon v-if="chargement" :icon="['fas', 'spinner']" class="w-4 h-4 animate-spin" />
              <font-awesome-icon v-else :icon="['fas', 'paper-plane']" class="w-4 h-4" />
              Soumettre
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
