<script setup lang="ts">
/**
 * Mise en relation avec l'équipe d'un support média (US6, FR-046).
 *
 * La messagerie n'autorise normalement l'envoi qu'entre membres amis : seul
 * l'endpoint métier `POST /api/medias/{type}/{id}/contacter` peut ouvrir ce
 * canal. Le message part vers le propriétaire du support, ou, s'il n'y en a
 * plus, vers son co-détenteur actif le plus ancien.
 */
import type { TypeSupportMedia } from '~/composables/useMediaDetention'

const props = defineProps<{
  isOpen: boolean
  typeSupport: TypeSupportMedia
  supportId: string
  nomSupport: string
}>()

const emit = defineEmits<{ close: [], envoye: [conversationId: string] }>()

const { contacter, erreur } = useMediaDetention()
const userStore = useUserStore()
const { redirigerVersConnexion } = useAuth()

const message = ref('')
const envoi = ref(false)
const conversationId = ref<string | null>(null)

const MESSAGE_MAX = 2000
const messageValide = computed(() => {
  const t = message.value.trim()
  return t.length > 0 && t.length <= MESSAGE_MAX
})

const fermer = () => {
  if (envoi.value) return
  emit('close')
}

/** Réinitialiser à la réouverture : une modale rouverte doit repartir vierge. */
watch(() => props.isOpen, (ouvert) => {
  if (ouvert) {
    message.value = ''
    conversationId.value = null
  }
})

const envoyer = async () => {
  if (!messageValide.value || envoi.value) return
  envoi.value = true
  const resultat = await contacter(props.typeSupport, props.supportId, message.value.trim())
  envoi.value = false
  if (resultat) {
    conversationId.value = resultat.conversation_id
    emit('envoye', resultat.conversation_id)
  }
}
</script>

<template>
  <AfricansModale
    :model-value="isOpen"
    :titre="`Contacter « ${nomSupport} »`"
    sous-titre="Votre message ouvre une conversation privée avec l'équipe du support."
    icone="fa-solid fa-paper-plane"
    ton="chocolat"
    @update:model-value="fermer()"
  >
    <!-- Succès -->
    <div v-if="conversationId" class="flex flex-col items-center gap-3 py-6 text-center">
      <font-awesome-icon icon="fa-solid fa-circle-check" class="text-4xl text-af-vert" />
      <p class="text-base font-bold text-af-encre">Message envoyé</p>
      <p class="text-[14px]/[1.6] text-af-corps">
        La conversation est ouverte : la suite des échanges se poursuit dans votre messagerie.
      </p>
      <AfricansBouton vers="/messagerie" icone="fa-solid fa-comments" class="mt-2">
        Ouvrir la messagerie
      </AfricansBouton>
    </div>

    <!-- Formulaire -->
    <div v-else class="flex flex-col gap-4">
      <p
        v-if="!userStore.accessToken"
        class="rounded-lg border border-af-chocolat/20 bg-af-chocolat/5 px-4 py-3 text-[14px]/[1.4] text-af-corps"
      >
        Vous devez être connecté pour envoyer un message.
        <button type="button" class="font-bold text-af-chocolat underline" @click="redirigerVersConnexion()">
          Se connecter
        </button>
      </p>

      <div>
        <AfricansChamp
          v-model="message"
          libelle="Votre message"
          type="textarea"
          :lignes="6"
          :maxlength="MESSAGE_MAX"
          placeholder="Présentez votre projet, votre demande ou votre proposition de collaboration…"
        />
        <p class="mt-1 text-right text-[12px] text-af-atone-2">
          {{ message.trim().length }} / {{ MESSAGE_MAX }}
        </p>
      </div>

      <p v-if="erreur" class="rounded-lg border border-af-live/20 bg-af-live/5 px-4 py-3 text-[14px]/[1.4] text-af-live">
        {{ erreur }}
      </p>
    </div>

    <template v-if="!conversationId" #actions>
      <button
        type="button"
        class="text-base font-bold text-af-corps transition hover:opacity-70 disabled:opacity-50"
        :disabled="envoi"
        @click="fermer"
      >
        Annuler
      </button>
      <AfricansBouton
        :desactive="!messageValide || envoi || !userStore.accessToken"
        :tourne="envoi"
        :icone="envoi ? 'fa-solid fa-spinner' : 'fa-solid fa-paper-plane'"
        @click="envoyer"
      >
        Envoyer
      </AfricansBouton>
    </template>
  </AfricansModale>
</template>
