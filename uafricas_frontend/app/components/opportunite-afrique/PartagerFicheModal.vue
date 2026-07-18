<script setup lang="ts">
interface Props {
  isOpen: boolean
  paysNom: string
  estConnecte?: boolean
}

const props = defineProps<Props>()
const emit = defineEmits<{
  (e: 'close'): void
  (e: 'submit', legende: string): void
}>()

const legende = ref('')
const enCours = ref(false)
const erreur = ref('')
const succes = ref(false)

const MAX = 500
const restant = computed(() => MAX - legende.value.length)

// ── Partage réseaux sociaux ─────────────────────────────────────
// URL de la page courante (résolue côté client à l'ouverture)
const urlPage = ref('')
const copieLienOk = ref(false)

const textePartage = computed(
  () => `Découvrez ${props.paysNom} sur AfricanS`,
)

const urlWhatsApp = computed(() =>
  `https://wa.me/?text=${encodeURIComponent(`${textePartage.value} ${urlPage.value}`)}`,
)
const urlFacebook = computed(() =>
  `https://www.facebook.com/sharer/sharer.php?u=${encodeURIComponent(urlPage.value)}`,
)
const urlTwitter = computed(() =>
  `https://twitter.com/intent/tweet?url=${encodeURIComponent(urlPage.value)}&text=${encodeURIComponent(textePartage.value)}`,
)
const urlLinkedIn = computed(() =>
  `https://www.linkedin.com/sharing/share-offsite/?url=${encodeURIComponent(urlPage.value)}`,
)

const reseaux = computed(() => [
  { nom: 'WhatsApp', url: urlWhatsApp.value, icon: ['fab', 'whatsapp'], couleur: 'bg-[#25D366] hover:bg-[#1da851]' },
  { nom: 'Facebook', url: urlFacebook.value, icon: ['fab', 'facebook'], couleur: 'bg-[#1877F2] hover:bg-[#0d65d9]' },
  { nom: 'X / Twitter', url: urlTwitter.value, icon: ['fab', 'twitter'], couleur: 'bg-black hover:bg-gray-800' },
  { nom: 'LinkedIn', url: urlLinkedIn.value, icon: ['fab', 'linkedin'], couleur: 'bg-[#0A66C2] hover:bg-[#084e96]' },
])

const partagerReseau = (url: string) => {
  window.open(url, '_blank', 'noopener,noreferrer,width=600,height=500')
}

const partageNatif = async () => {
  if (typeof navigator !== 'undefined' && navigator.share) {
    try {
      await navigator.share({ title: props.paysNom, text: textePartage.value, url: urlPage.value })
    }
    catch {
      // Partage annulé par l'utilisateur — sans effet
    }
  }
}

const supporteWebShare = ref(false)

const copierLien = async () => {
  try {
    await navigator.clipboard.writeText(urlPage.value)
    copieLienOk.value = true
    setTimeout(() => { copieLienOk.value = false }, 2000)
  }
  catch {
    // Clipboard indisponible — sans effet
  }
}

// Réinitialise à chaque ouverture
watch(
  () => props.isOpen,
  (ouvert) => {
    if (ouvert) {
      legende.value = ''
      erreur.value = ''
      succes.value = false
      enCours.value = false
      copieLienOk.value = false
      if (typeof window !== 'undefined') urlPage.value = window.location.href
    }
  },
)

const fermer = () => {
  if (enCours.value) return
  emit('close')
}

const soumettre = () => {
  if (enCours.value) return
  if (legende.value.length > MAX) {
    erreur.value = `La légende ne doit pas dépasser ${MAX} caractères.`
    return
  }
  erreur.value = ''
  emit('submit', legende.value.trim())
}

// API exposée au parent pour piloter l'état
const setLoading = (v: boolean) => {
  enCours.value = v
}
const setError = (msg: string) => {
  enCours.value = false
  erreur.value = msg
}
const setSuccess = () => {
  enCours.value = false
  succes.value = true
  setTimeout(() => emit('close'), 1200)
}
defineExpose({ setLoading, setError, setSuccess })

const onKeydown = (e: KeyboardEvent) => {
  if (e.key === 'Escape' && props.isOpen) fermer()
}
onMounted(() => {
  window.addEventListener('keydown', onKeydown)
  supporteWebShare.value = typeof navigator !== 'undefined' && !!navigator.share
})
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
        <div class="absolute inset-0 bg-black/50 backdrop-blur-sm"></div>

        <div class="relative bg-white rounded-2xl shadow-2xl w-full max-w-lg overflow-hidden">
          <!-- En-tête -->
          <div class="flex items-center justify-between px-6 py-4 border-b border-gray-100">
            <h3 class="font-oswald text-xl font-bold text-gray-900">
              Partager ce territoire
            </h3>
            <button
              type="button"
              class="w-9 h-9 flex items-center justify-center rounded-full text-gray-400 hover:bg-gray-100 hover:text-gray-700 transition-colors cursor-pointer"
              :disabled="enCours"
              @click="fermer"
            >
              <font-awesome-icon :icon="['fas', 'xmark']" class="w-5 h-5" />
            </button>
          </div>

          <!-- Corps -->
          <div class="px-6 py-5">
            <div
              v-if="succes"
              class="flex flex-col items-center justify-center py-8 text-center"
            >
              <div class="w-14 h-14 rounded-full bg-custom-green/10 flex items-center justify-center mb-4">
                <font-awesome-icon :icon="['fas', 'check']" class="w-7 h-7 text-custom-green" />
              </div>
              <p class="font-medium text-gray-900">Partagé dans la communauté !</p>
              <p class="text-sm text-gray-500 mt-1">Votre partage apparaît désormais dans /publications.</p>
            </div>

            <template v-else>
              <!-- Partage réseaux sociaux -->
              <div class="mb-5">
                <h4 class="text-xs font-bold uppercase tracking-widest text-gray-400 mb-3">
                  Partager sur les réseaux sociaux
                </h4>
                <div class="flex flex-wrap gap-2.5">
                  <button
                    v-for="reseau in reseaux"
                    :key="reseau.nom"
                    type="button"
                    class="flex items-center gap-2 rounded-xl px-3.5 py-2.5 text-sm font-medium text-white shadow-sm transition-all hover:shadow-md active:scale-95 cursor-pointer"
                    :class="reseau.couleur"
                    :title="`Partager sur ${reseau.nom}`"
                    @click="partagerReseau(reseau.url)"
                  >
                    <font-awesome-icon :icon="reseau.icon" />
                    <span class="hidden sm:inline">{{ reseau.nom }}</span>
                  </button>

                  <!-- Partage natif (mobile) -->
                  <button
                    v-if="supporteWebShare"
                    type="button"
                    class="flex items-center gap-2 rounded-xl px-3.5 py-2.5 text-sm font-medium text-white bg-custom-chocolat hover:bg-custom-chocolat/90 shadow-sm transition-all active:scale-95 cursor-pointer"
                    title="Plus d'options de partage"
                    @click="partageNatif"
                  >
                    <font-awesome-icon :icon="['fas', 'share-nodes']" />
                    <span class="hidden sm:inline">Plus…</span>
                  </button>

                  <!-- Copier le lien -->
                  <button
                    type="button"
                    class="flex items-center gap-2 rounded-xl px-3.5 py-2.5 text-sm font-medium shadow-sm transition-all active:scale-95 cursor-pointer"
                    :class="copieLienOk
                      ? 'bg-green-50 text-green-700 ring-1 ring-green-300'
                      : 'bg-gray-50 text-gray-700 ring-1 ring-gray-200 hover:bg-gray-100'"
                    @click="copierLien"
                  >
                    <font-awesome-icon :icon="copieLienOk ? ['fas', 'check'] : ['fas', 'link']" />
                    <span class="hidden sm:inline">{{ copieLienOk ? 'Copié !' : 'Copier le lien' }}</span>
                  </button>
                </div>
              </div>

              <div class="relative my-5">
                <div class="absolute inset-0 flex items-center">
                  <div class="w-full border-t border-gray-200"></div>
                </div>
                <div class="relative flex justify-center">
                  <span class="bg-white px-3 text-xs font-medium text-gray-400">ou sur le mur AfricanS</span>
                </div>
              </div>

              <!-- Partage communautaire (réservé aux membres connectés) -->
              <template v-if="estConnecte">
                <p class="text-sm text-gray-600 mb-4">
                  Vous partagez le territoire
                  <span class="font-semibold text-gray-900">{{ paysNom }}</span>
                  sur le mur communautaire. Ajoutez un mot (facultatif) pour accompagner votre partage.
                </p>

                <label class="block text-sm font-medium text-gray-700 mb-1.5">
                  Légende <span class="text-gray-400 font-normal">(facultatif)</span>
                </label>
                <textarea
                  v-model="legende"
                  rows="4"
                  :maxlength="MAX"
                  placeholder="Ex. : À découvrir absolument, paysages magnifiques et accueil chaleureux !"
                  class="w-full px-3.5 py-2.5 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-custom-green focus:border-transparent resize-none"
                  :disabled="enCours"
                ></textarea>
                <div class="flex items-center justify-between mt-1.5">
                  <p v-if="erreur" class="text-sm text-red-600">{{ erreur }}</p>
                  <span v-else></span>
                  <span class="text-xs" :class="restant < 0 ? 'text-red-600' : 'text-gray-400'">
                    {{ restant }}
                  </span>
                </div>
              </template>

              <p v-else class="text-sm text-gray-500 text-center">
                <NuxtLink to="/login" class="text-custom-green font-medium hover:underline">
                  Connectez-vous
                </NuxtLink>
                pour partager aussi ce territoire sur le mur communautaire AfricanS.
              </p>
            </template>
          </div>

          <!-- Pied -->
          <div
            v-if="!succes && estConnecte"
            class="flex items-center justify-end gap-3 px-6 py-4 border-t border-gray-100 bg-gray-50"
          >
            <button
              type="button"
              class="px-4 py-2 text-sm font-medium text-gray-700 rounded-lg hover:bg-gray-200 transition-colors cursor-pointer disabled:opacity-50"
              :disabled="enCours"
              @click="fermer"
            >
              Annuler
            </button>
            <button
              type="button"
              class="px-5 py-2 text-sm font-medium text-white bg-custom-green rounded-lg hover:bg-custom-green/90 transition-colors cursor-pointer disabled:opacity-60 inline-flex items-center gap-2"
              :disabled="enCours"
              @click="soumettre"
            >
              <font-awesome-icon
                v-if="enCours"
                :icon="['fas', 'spinner']"
                class="w-4 h-4 animate-spin"
              />
              <font-awesome-icon v-else :icon="['fas', 'share-nodes']" class="w-4 h-4" />
              Partager
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
