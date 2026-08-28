<script setup lang="ts">
import type { TypeReponsePublique } from '~/composables/useRetrouvAmis'

const props = defineProps<{
  slug: string
  auteurId?: string
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
      class="bg-af-vert/5 border border-af-vert/30 rounded-lg p-4 flex items-start gap-3"
    >
      <font-awesome-icon :icon="['fas', 'circle-check']" class="text-af-vert text-lg mt-0.5 shrink-0" />
      <div>
        <p class="text-af-vert text-sm font-medium">{{ messageSucces }}</p>
        <p class="text-af-vert text-xs mt-1">
          Une correspondance a été créée. L'auteur de l'avis pourra vous contacter.
        </p>
      </div>
    </div>

    <!-- Erreur -->
    <div
      v-if="erreur"
      class="bg-af-live/5 border border-af-live/30 rounded-lg p-4 flex items-start gap-3"
    >
      <font-awesome-icon :icon="['fas', 'circle-exclamation']" class="text-af-live text-lg mt-0.5 shrink-0" />
      <p class="text-af-live text-sm">{{ erreur }}</p>
    </div>

    <!-- Bouton pour ouvrir le formulaire (masque si deja repondu ou si auteur) -->
    <div v-if="!reponseEnvoyee && !estAuteur && !formulaireOuvert">
      <button
        type="button"
        class="w-full sm:w-auto flex items-center justify-center gap-2 px-6 py-3 bg-af-vert text-white font-medium rounded-lg hover:bg-af-vert transition-colors text-sm"
        @click="ouvrirFormulaire"
      >
        <font-awesome-icon :icon="['fas', 'paper-plane']" />
        Repondre a cet avis
      </button>
    </div>

    <!-- Formulaire de reponse -->
    <div
      v-if="formulaireOuvert"
      class="bg-white border border-af-bordure rounded-lg shadow-sm p-6 space-y-5"
    >
      <div class="flex items-center justify-between">
        <h3 class="text-lg font-semibold text-af-encre">
          Repondre a cet avis de recherche
        </h3>
        <button type="button" class="text-af-atone-2 hover:text-af-corps" @click="fermer">
          <font-awesome-icon :icon="['fas', 'xmark']" class="text-xl" />
        </button>
      </div>

      <div class="bg-af-chocolat/5 border border-af-chocolat/20 rounded-lg p-3">
        <p class="text-af-chocolat text-sm">
          <font-awesome-icon :icon="['fas', 'circle-info']" class="mr-1" />
          Votre réponse créera une correspondance avec l'auteur de l'avis. Il sera notifié et pourra entrer en contact avec vous.
        </p>
      </div>

      <!-- Type de reponse -->
      <div>
        <label class="block text-sm font-medium text-af-corps mb-2">
          Votre relation avec la personne recherchée <span class="text-af-live">*</span>
        </label>
        <div class="space-y-2">
          <label
            v-for="type in typesReponse"
            :key="type.value"
            class="flex items-start gap-3 p-3 rounded-lg border cursor-pointer transition-colors"
            :class="typeReponse === type.value
              ? 'border-af-chocolat bg-af-chocolat/5'
              : 'border-af-bordure hover:border-af-bordure'"
          >
            <input
              v-model="typeReponse"
              type="radio"
              :value="type.value"
              class="mt-0.5 accent-af-chocolat"
            />
            <div>
              <span class="text-sm font-medium text-af-encre">{{ type.label }}</span>
              <p class="text-xs text-af-atone mt-0.5">{{ type.description }}</p>
            </div>
          </label>
        </div>
      </div>

      <!-- Message -->
      <div>
        <label for="message-reponse" class="block text-sm font-medium text-af-corps mb-1">
          Votre message <span class="text-af-live">*</span>
        </label>
        <textarea
          id="message-reponse"
          v-model="message"
          rows="4"
          class="w-full rounded-lg border border-af-bordure px-4 py-3 text-sm text-af-encre placeholder:text-af-atone-2 focus:border-af-chocolat focus:ring-2 focus:border-af-chocolat/20 focus:outline-none transition-colors resize-none"
          placeholder="Decrivez comment vous connaissez cette personne, les details qui pourraient aider a l'identifier..."
        />
      </div>

      <!-- Boutons -->
      <div class="flex justify-end gap-3">
        <button
          type="button"
          class="px-4 py-2 text-sm text-af-corps hover:text-af-encre transition-colors"
          @click="fermer"
        >
          Annuler
        </button>
        <button
          type="button"
          class="px-5 py-2 bg-af-vert text-white text-sm font-medium rounded-lg hover:bg-af-vert transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
          :disabled="!message.trim() || chargement"
          @click="soumettre"
        >
          <font-awesome-icon v-if="chargement" :icon="['fas', 'spinner']" class="animate-spin" />
          <font-awesome-icon v-else :icon="['fas', 'paper-plane']" />
          Envoyer ma réponse
        </button>
      </div>
    </div>
  </div>
</template>
