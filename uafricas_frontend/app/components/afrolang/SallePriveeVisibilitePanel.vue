<script setup lang="ts">
// Panneau réservé au créateur de la salle privée (US5)
// Tailwind v4 pur — aucune classe daisyUI
import type { SallePriveeDetailAPI, VisibiliteSallePrivee } from '~/composables/useAfrolang'

interface Props {
  salle: SallePriveeDetailAPI
}

const props = defineProps<Props>()
const emit = defineEmits<{
  (e: 'maj'): void
}>()

const { changerVisibiliteSallePrivee, modifierMaxParticipantsSallePrivee, inviterMembre } =
  useAfrolang()

const visibiliteActuelle = ref<VisibiliteSallePrivee>(
  (props.salle.visibilite as VisibiliteSallePrivee) ?? 'fermee',
)
const maxActuel = ref<number | null>(props.salle.max_participants ?? null)
const inviteUid = ref('')
const enCours = ref(false)
const message = ref<{ texte: string; type: 'succes' | 'erreur' } | null>(null)

watch(
  () => props.salle,
  (s) => {
    visibiliteActuelle.value = (s.visibilite as VisibiliteSallePrivee) ?? 'fermee'
    maxActuel.value = s.max_participants ?? null
  },
)

const afficherMessage = (texte: string, type: 'succes' | 'erreur') => {
  message.value = { texte, type }
  setTimeout(() => {
    message.value = null
  }, 4000)
}

const basculerVisibilite = async () => {
  enCours.value = true
  const cible: VisibiliteSallePrivee = visibiliteActuelle.value === 'fermee' ? 'visible' : 'fermee'
  const ok = await changerVisibiliteSallePrivee(props.salle.id, cible)
  enCours.value = false
  if (ok) {
    visibiliteActuelle.value = cible
    afficherMessage(`Visibilité : ${cible}`, 'succes')
    emit('maj')
  }
  else {
    afficherMessage('Échec du changement de visibilité', 'erreur')
  }
}

const mettreAJourMax = async () => {
  if (!maxActuel.value || maxActuel.value < 1) {
    afficherMessage('La limite doit être supérieure ou égale à 1', 'erreur')
    return
  }
  enCours.value = true
  const ok = await modifierMaxParticipantsSallePrivee(props.salle.id, maxActuel.value)
  enCours.value = false
  if (ok) {
    afficherMessage('Limite mise à jour', 'succes')
    emit('maj')
  }
  else {
    afficherMessage('Échec de la mise à jour', 'erreur')
  }
}

const envoyerInvitation = async () => {
  if (!inviteUid.value.trim()) return
  enCours.value = true
  const res = await inviterMembre(props.salle.id, inviteUid.value.trim())
  enCours.value = false
  if (res) {
    afficherMessage('Invitation envoyée', 'succes')
    inviteUid.value = ''
    emit('maj')
  }
  else {
    afficherMessage("Échec de l'invitation", 'erreur')
  }
}
</script>

<template>
  <section class="rounded-lg border border-gray-200 bg-white p-6 space-y-5">
    <header class="space-y-1">
      <h3 class="text-lg font-semibold text-gray-900">Gestion de la salle privée</h3>
      <p class="text-sm text-gray-500">Réservé au créateur</p>
    </header>

    <!-- Visibilité -->
    <div class="flex items-center justify-between gap-4">
      <div>
        <p class="text-sm font-medium text-gray-900">Visibilité</p>
        <p class="text-xs text-gray-500">
          {{ visibiliteActuelle === 'fermee'
            ? 'Fermée — invitations uniquement'
            : 'Visible — les membres peuvent demander à rejoindre' }}
        </p>
      </div>
      <button
        type="button"
        :disabled="enCours"
        class="rounded-md bg-custom-chocolat px-4 py-2 text-sm font-medium text-white hover:bg-amber-800 disabled:opacity-50"
        @click="basculerVisibilite"
      >
        {{ visibiliteActuelle === 'fermee' ? 'Rendre visible' : 'Fermer' }}
      </button>
    </div>

    <!-- Limite participants -->
    <div class="space-y-2">
      <p class="text-sm font-medium text-gray-900">Limite de participants</p>
      <div class="flex items-center gap-2">
        <input
          v-model.number="maxActuel"
          type="number"
          min="1"
          placeholder="Illimité"
          class="w-32 rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-custom-chocolat focus:outline-none"
        />
        <button
          type="button"
          :disabled="enCours"
          class="rounded-md bg-custom-green px-3 py-2 text-sm font-medium text-white hover:bg-green-700 disabled:opacity-50"
          @click="mettreAJourMax"
        >
          Enregistrer
        </button>
      </div>
    </div>

    <!-- Invitation -->
    <div class="space-y-2">
      <p class="text-sm font-medium text-gray-900">Inviter un membre</p>
      <div class="flex items-center gap-2">
        <input
          v-model="inviteUid"
          type="text"
          placeholder="Identifiant utilisateur (UUID)"
          class="flex-1 rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-custom-chocolat focus:outline-none"
        />
        <button
          type="button"
          :disabled="enCours || !inviteUid.trim()"
          class="rounded-md bg-gray-900 px-3 py-2 text-sm font-medium text-white hover:bg-gray-700 disabled:opacity-50"
          @click="envoyerInvitation"
        >
          Inviter
        </button>
      </div>
    </div>

    <div
      v-if="message"
      :class="[
        'rounded-md px-3 py-2 text-sm',
        message.type === 'succes' ? 'bg-green-50 text-green-800' : 'bg-red-50 text-red-800',
      ]"
    >
      {{ message.texte }}
    </div>
  </section>
</template>
