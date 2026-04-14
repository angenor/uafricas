<template>
  <section
    v-if="visible"
    class="bg-white rounded-2xl shadow-lg border border-custom-chocolat/20 p-5"
  >
    <header class="flex items-center justify-between mb-4">
      <div class="flex items-center gap-2">
        <span class="inline-flex w-8 h-8 items-center justify-center bg-custom-chocolat/10 text-custom-chocolat rounded-full">
          <font-awesome-icon icon="shield-halved" />
        </span>
        <h3 class="text-lg font-bold text-gray-900">Modération de session</h3>
      </div>
      <span class="text-xs text-gray-500">
        Vous modérez actuellement cette session.
      </span>
    </header>

    <p class="text-sm text-gray-600 mb-4">
      Transférez le rôle de modérateur à un autre participant actif de la session.
      L'utilisateur ciblé sera notifié.
    </p>

    <div v-if="participantsActifs.length === 0" class="text-sm text-gray-500 italic py-4 text-center">
      Aucun autre participant actif disponible pour l'instant.
    </div>

    <ul v-else class="space-y-2 max-h-72 overflow-y-auto pr-1">
      <li
        v-for="p in participantsActifs"
        :key="p.id"
        class="flex items-center justify-between gap-3 p-2 rounded-lg hover:bg-gray-50 transition"
      >
        <div class="flex items-center gap-3 min-w-0">
          <div
            class="w-9 h-9 rounded-full bg-custom-chocolat text-white text-xs font-bold flex items-center justify-center flex-shrink-0"
          >
            {{ initiales(p) }}
          </div>
          <div class="min-w-0">
            <p class="text-sm font-semibold text-gray-900 truncate">
              {{ p.prenom }} {{ p.nom }}
            </p>
            <p class="text-xs text-gray-500">
              Arrivé le {{ formatDateHeure(p.rejoint_at) }}
            </p>
          </div>
        </div>
        <button
          type="button"
          class="px-3 py-1.5 rounded-lg bg-custom-chocolat text-white text-xs font-semibold hover:bg-custom-chocolat/90 disabled:opacity-50 transition"
          :disabled="enTransfert === p.utilisateur_id"
          @click="confirmerTransfert(p)"
        >
          <font-awesome-icon
            v-if="enTransfert === p.utilisateur_id"
            icon="spinner"
            class="animate-spin mr-1"
          />
          Transférer
        </button>
      </li>
    </ul>

    <div v-if="messageRetour" class="mt-3 text-sm" :class="messageRetourClasse">
      {{ messageRetour }}
    </div>
  </section>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import type { ParticipantAPI } from '~/composables/useAfrolang'
import { formatDateHeure, useAfrolang } from '~/composables/useAfrolang'

interface Props {
  sessionId: string
  utilisateurCourantId: string | null
  moderateurActifId: string | null
  participants: ParticipantAPI[]
}

const props = defineProps<Props>()

const emit = defineEmits<{
  (e: 'transferred', destinataireId: string): void
}>()

const { transfererModerationSession } = useAfrolang()

const enTransfert = ref<string | null>(null)
const messageRetour = ref<string | null>(null)
const messageRetourClasse = ref('text-green-600')

const visible = computed(
  () =>
    !!props.utilisateurCourantId &&
    props.moderateurActifId === props.utilisateurCourantId,
)

const participantsActifs = computed(() =>
  props.participants.filter(
    (p) =>
      !p.quitte_at &&
      p.utilisateur_id !== props.utilisateurCourantId,
  ),
)

const initiales = (p: ParticipantAPI) => {
  const a = (p.prenom?.charAt(0) || '').toUpperCase()
  const b = (p.nom?.charAt(0) || '').toUpperCase()
  return (a + b) || '?'
}

const confirmerTransfert = async (p: ParticipantAPI) => {
  if (!confirm(`Transférer la modération à ${p.prenom} ${p.nom} ?`)) return
  messageRetour.value = null
  enTransfert.value = p.utilisateur_id

  const ok = await transfererModerationSession(props.sessionId, p.utilisateur_id)

  enTransfert.value = null
  if (ok) {
    messageRetourClasse.value = 'text-green-600'
    messageRetour.value = 'Modération transférée.'
    emit('transferred', p.utilisateur_id)
  } else {
    messageRetourClasse.value = 'text-red-600'
    messageRetour.value = 'Échec du transfert.'
  }
}
</script>
