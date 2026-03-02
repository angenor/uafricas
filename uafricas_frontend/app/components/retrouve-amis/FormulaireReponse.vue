<script setup lang="ts">
import type { TypeReponsePublique } from '~/composables/useRetrouvAmis'

const props = defineProps<{
  slug: string
  auteurId: string
}>()

const userStore = useUserStore()
const router = useRouter()
const { repondreAvisPublic, chargement, erreur } = useRetrouvAmis()

// ── Etat local ────────────────────────────────────────────
const formulaireOuvert = ref(false)
const typeReponse = ref<TypeReponsePublique>('je_la_connais')
const message = ref('')
const messageSucces = ref('')
const reponseEnvoyee = ref(false)

const typesReponse: { value: TypeReponsePublique; label: string; description: string }[] = [
  {
    value: 'je_suis_cette_personne',
    label: 'Je suis cette personne',
    description: 'Vous pensez etre la personne recherchee dans cet avis.',
  },
  {
    value: 'je_la_connais',
    label: 'Je la connais',
    description: 'Vous connaissez ou avez connu la personne recherchee.',
  },
  {
    value: 'jai_des_informations',
    label: 'J\'ai des informations',
    description: 'Vous avez des informations qui pourraient aider.',
  },
]

// ── Verifier si l'utilisateur est l'auteur ─────────────────
const estAuteur = computed(() => {
  return userStore.isAuthenticated && userStore.user?.id === props.auteurId
})

// ── Ouvrir le formulaire ───────────────────────────────────
const ouvrirFormulaire = () => {
  if (!userStore.isAuthenticated) {
    router.push(`/login?redirect=${encodeURIComponent(`/retrouve-amis/public/${props.slug}`)}`)
    return
  }
  formulaireOuvert.value = true
}

// ── Soumettre la reponse ───────────────────────────────────
const soumettre = async () => {
  if (!message.value.trim()) return

  const resultat = await repondreAvisPublic(props.slug, {
    type_reponse: typeReponse.value,
    message: message.value.trim(),
  })

  if (resultat) {
    messageSucces.value = resultat.message
    reponseEnvoyee.value = true
    formulaireOuvert.value = false
  }
}

const fermer = () => {
  formulaireOuvert.value = false
}
</script>

<template>
  <div class="mt-6">
    <!-- Message de succes -->
    <div
      v-if="messageSucces"
      class="bg-green-50 border border-green-200 rounded-lg p-4 flex items-start gap-3"
    >
      <font-awesome-icon :icon="['fas', 'circle-check']" class="text-custom-green text-lg mt-0.5 shrink-0" />
      <div>
        <p class="text-green-800 text-sm font-medium">{{ messageSucces }}</p>
        <p class="text-green-700 text-xs mt-1">
          Une correspondance a ete creee. L'auteur de l'avis pourra vous contacter.
        </p>
      </div>
    </div>

    <!-- Erreur -->
    <div
      v-if="erreur"
      class="bg-red-50 border border-red-200 rounded-lg p-4 flex items-start gap-3"
    >
      <font-awesome-icon :icon="['fas', 'circle-exclamation']" class="text-red-500 text-lg mt-0.5 shrink-0" />
      <p class="text-red-700 text-sm">{{ erreur }}</p>
    </div>

    <!-- Bouton pour ouvrir le formulaire (masque si deja repondu ou si auteur) -->
    <div v-if="!reponseEnvoyee && !estAuteur && !formulaireOuvert">
      <button
        type="button"
        class="w-full sm:w-auto flex items-center justify-center gap-2 px-6 py-3 bg-custom-green text-white font-medium rounded-lg hover:bg-green-700 transition-colors text-sm"
        @click="ouvrirFormulaire"
      >
        <font-awesome-icon :icon="['fas', 'paper-plane']" />
        Repondre a cet avis
      </button>
    </div>

    <!-- Formulaire de reponse -->
    <div
      v-if="formulaireOuvert"
      class="bg-white border border-gray-200 rounded-xl shadow-sm p-6 space-y-5"
    >
      <div class="flex items-center justify-between">
        <h3 class="text-lg font-semibold text-gray-800">
          Repondre a cet avis de recherche
        </h3>
        <button type="button" class="text-gray-400 hover:text-gray-600" @click="fermer">
          <font-awesome-icon :icon="['fas', 'xmark']" class="text-xl" />
        </button>
      </div>

      <div class="bg-blue-50 border border-blue-200 rounded-lg p-3">
        <p class="text-blue-800 text-sm">
          <font-awesome-icon :icon="['fas', 'circle-info']" class="mr-1" />
          Votre reponse creera une correspondance avec l'auteur de l'avis. Il sera notifie et pourra entrer en contact avec vous.
        </p>
      </div>

      <!-- Type de reponse -->
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-2">
          Votre relation avec la personne recherchee <span class="text-red-500">*</span>
        </label>
        <div class="space-y-2">
          <label
            v-for="type in typesReponse"
            :key="type.value"
            class="flex items-start gap-3 p-3 rounded-lg border cursor-pointer transition-colors"
            :class="typeReponse === type.value
              ? 'border-amber-500 bg-amber-50'
              : 'border-gray-200 hover:border-gray-300'"
          >
            <input
              v-model="typeReponse"
              type="radio"
              :value="type.value"
              class="mt-0.5 accent-amber-700"
            />
            <div>
              <span class="text-sm font-medium text-gray-800">{{ type.label }}</span>
              <p class="text-xs text-gray-500 mt-0.5">{{ type.description }}</p>
            </div>
          </label>
        </div>
      </div>

      <!-- Message -->
      <div>
        <label for="message-reponse" class="block text-sm font-medium text-gray-700 mb-1">
          Votre message <span class="text-red-500">*</span>
        </label>
        <textarea
          id="message-reponse"
          v-model="message"
          rows="4"
          class="w-full rounded-lg border border-gray-300 px-4 py-3 text-sm text-gray-800 placeholder:text-gray-400 focus:border-amber-500 focus:ring-2 focus:ring-amber-500/20 focus:outline-none transition-colors resize-none"
          placeholder="Decrivez comment vous connaissez cette personne, les details qui pourraient aider a l'identifier..."
        />
      </div>

      <!-- Boutons -->
      <div class="flex justify-end gap-3">
        <button
          type="button"
          class="px-4 py-2 text-sm text-gray-600 hover:text-gray-800 transition-colors"
          @click="fermer"
        >
          Annuler
        </button>
        <button
          type="button"
          class="px-5 py-2 bg-custom-green text-white text-sm font-medium rounded-lg hover:bg-green-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
          :disabled="!message.trim() || chargement"
          @click="soumettre"
        >
          <font-awesome-icon v-if="chargement" :icon="['fas', 'spinner']" class="animate-spin" />
          <font-awesome-icon v-else :icon="['fas', 'paper-plane']" />
          Envoyer ma reponse
        </button>
      </div>
    </div>
  </div>
</template>
