<script setup lang="ts">
// Espace commentaires temps réel de session (US6), persistance REST +
// diffusion instantanée par DataPacket LiveKit (`type: 'chat'`).
//
// La diffusion est faite PAR LE SERVEUR après l'INSERT, pas par le client
// émetteur : le token de session refuse `can_publish_data` à tout participant
// ordinaire (canal data réservé au tableau blanc), si bien qu'une diffusion
// côté client ne servirait le direct qu'aux modérateurs. Le client se contente
// donc d'écouter : et d'afficher son propre message, qu'il connaît déjà.
//
// Le composant reste MONTÉ même quand le panneau est replié (`v-show` côté
// parent) : sans cela il manquerait les paquets reçus pendant la fermeture, et
// le compteur de non-lus serait faux.
//
// Tailwind v4 pur (thème sombre, cohérent avec la salle).
import type { Room } from 'livekit-client'
import type { MessageSessionAPI } from '~/composables/useAfrolang'

interface Props {
  sessionId: string
  /** Room LiveKit connectée : support de la diffusion temps réel. */
  room?: Room | null
  /** Le panneau est-il visible ? Sert à remettre le compteur de non-lus à zéro. */
  visible?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  room: null,
  visible: true,
})

const emit = defineEmits<{
  /** Nombre de messages reçus alors que le panneau était replié. */
  'non-lus': [nombre: number]
  fermer: []
}>()

const { listerMessagesSession, envoyerMessageSession } = useAfrolang()
const userStore = useUserStore()

const messages = ref<MessageSessionAPI[]>([])
const nouveauMessage = ref('')
const envoyerEnCours = ref(false)
const chargementHistorique = ref(false)
const erreurEnvoi = ref<string | null>(null)
const zoneFlux = ref<HTMLElement | null>(null)
const champSaisie = ref<HTMLTextAreaElement | null>(null)
/** L'utilisateur a-t-il fait défiler vers le haut ? (on n'auto-défile plus alors) */
const colleEnBas = ref(true)
const nouveauxEnBas = ref(0)
const nonLus = ref(0)

/** Ids déjà affichés : le même message peut arriver par POST, par DataPacket
 *  et par la reprise d'historique `since`. */
const idsConnus = new Set<string>()

const decodeur = new TextDecoder()

const initiales = (auteur: MessageSessionAPI): string => {
  const p = (auteur.auteur_prenom ?? '')[0] ?? ''
  const n = (auteur.auteur_nom ?? '')[0] ?? ''
  return (p + n).toUpperCase() || '?'
}

const nomAuteur = (m: MessageSessionAPI): string =>
  [m.auteur_prenom, m.auteur_nom].filter(Boolean).join(' ') || 'Participant'

const estMoi = (auteurId: string): boolean => userStore.user?.id === auteurId

const heure = (iso: string): string => {
  const d = new Date(iso)
  return Number.isNaN(d.getTime())
    ? ''
    : d.toLocaleTimeString('fr-FR', { hour: '2-digit', minute: '2-digit' })
}

/** Vrai quand le flux est (quasi) en bas, tolérance de 60 px. */
const estEnBas = (): boolean => {
  const el = zoneFlux.value
  if (!el) return true
  return el.scrollHeight - el.scrollTop - el.clientHeight < 60
}

const defiler = async (force = false) => {
  if (!force && !colleEnBas.value) return
  await nextTick()
  const el = zoneFlux.value
  if (!el) return
  el.scrollTop = el.scrollHeight
  nouveauxEnBas.value = 0
}

const surDefilement = () => {
  colleEnBas.value = estEnBas()
  if (colleEnBas.value) nouveauxEnBas.value = 0
}

/** Insère un message s'il est inédit, en maintenant l'ordre chronologique.
 *  Retourne vrai si le message a bien été ajouté. */
const ajouterMessage = (m: MessageSessionAPI): boolean => {
  if (!m?.id || idsConnus.has(m.id)) return false
  idsConnus.add(m.id)
  const liste = [...messages.value, m]
  liste.sort((a, b) => a.created_at.localeCompare(b.created_at))
  messages.value = liste
  return true
}

const derniereDate = (): string | undefined =>
  messages.value.length > 0 ? messages.value[messages.value.length - 1]?.created_at : undefined

const chargerHistorique = async () => {
  chargementHistorique.value = true
  const liste = await listerMessagesSession(props.sessionId, { limit: 200 })
  messages.value = []
  idsConnus.clear()
  for (const m of liste) ajouterMessage(m)
  chargementHistorique.value = false
  await defiler(true)
}

/** Rattrapage après une coupure réseau : ne récupère que ce qui a suivi le
 *  dernier message connu (les DataPackets émis pendant la déconnexion sont
 *  définitivement perdus, la base est la seule source de vérité). */
const rattraper = async () => {
  const since = derniereDate()
  const liste = await listerMessagesSession(props.sessionId, since ? { since } : { limit: 200 })
  let ajoutes = 0
  for (const m of liste) {
    if (ajouterMessage(m)) ajoutes++
  }
  if (ajoutes > 0) signalerNouveaux(ajoutes)
}

/** Comptabilise des messages entrants (non-lus si replié, indicateur de bas de
 *  flux si l'utilisateur lit plus haut). */
const signalerNouveaux = (nombre: number) => {
  if (!props.visible) {
    nonLus.value += nombre
    emit('non-lus', nonLus.value)
  }
  if (colleEnBas.value) {
    void defiler()
  }
  else {
    nouveauxEnBas.value += nombre
  }
}

const envoyer = async () => {
  const contenu = nouveauMessage.value.trim()
  if (!contenu || envoyerEnCours.value) return
  envoyerEnCours.value = true
  erreurEnvoi.value = null

  const message = await envoyerMessageSession(props.sessionId, contenu)
  envoyerEnCours.value = false

  if (!message) {
    erreurEnvoi.value = 'Envoi impossible, vérifiez votre connexion et réessayez.'
    return
  }

  nouveauMessage.value = ''
  colleEnBas.value = true
  ajouterMessage(message)
  await defiler(true)
}

const surDataPacket = (payload: Uint8Array) => {
  try {
    const data = JSON.parse(decodeur.decode(payload)) as {
      type?: string
      message?: MessageSessionAPI
    }
    if (data?.type !== 'chat' || !data.message?.id) return
    if (ajouterMessage(data.message)) signalerNouveaux(1)
  }
  catch {
    /* paquet ignoré */
  }
}

/** Filet de rattrapage périodique. Le `send_data` serveur est best-effort (son
 *  échec est journalisé, jamais propagé, pour ne pas faire échouer un message
 *  pourtant persisté) : ce sondage léger, une requête `since`, le plus souvent
 *  vide : garantit la convergence, LiveKit restant le chemin instantané. */
const INTERVALLE_RATTRAPAGE_MS = 30_000
let minuteurRattrapage: ReturnType<typeof setInterval> | null = null

let detacher: (() => void) | null = null

const attacher = (room: Room | null | undefined) => {
  detacher?.()
  detacher = null
  if (!room) return
  room.on('dataReceived', surDataPacket)
  room.on('reconnected', rattraper)
  detacher = () => {
    room.off('dataReceived', surDataPacket)
    room.off('reconnected', rattraper)
  }
}

watch(() => props.room, room => attacher(room), { immediate: true })

watch(() => props.visible, async (visible) => {
  if (!visible) return
  nonLus.value = 0
  emit('non-lus', 0)
  await defiler(true)
  champSaisie.value?.focus()
})

watch(() => props.sessionId, () => {
  chargerHistorique()
})

onMounted(() => {
  chargerHistorique()
  minuteurRattrapage = setInterval(() => {
    if (document.hidden || chargementHistorique.value) return
    void rattraper()
  }, INTERVALLE_RATTRAPAGE_MS)
})

onBeforeUnmount(() => {
  detacher?.()
  detacher = null
  if (minuteurRattrapage) {
    clearInterval(minuteurRattrapage)
    minuteurRattrapage = null
  }
})
</script>

<template>
  <section class="flex h-full min-h-0 flex-col bg-gray-800 text-gray-100">
    <header class="flex items-center justify-between border-b border-gray-700 px-4 py-2">
      <span class="flex items-center gap-2 text-sm font-semibold">
        <font-awesome-icon :icon="['fas', 'comments']" class="h-4 w-4 text-sky-400" />
        Commentaires
      </span>
      <button
        type="button"
        class="text-gray-400 transition-colors hover:text-white"
        aria-label="Fermer les commentaires"
        @click="emit('fermer')"
      >
        <font-awesome-icon :icon="['fas', 'xmark']" class="h-4 w-4" />
      </button>
    </header>

    <div class="relative min-h-0 flex-1">
      <div
        ref="zoneFlux"
        class="h-full overflow-y-auto px-4 py-3 space-y-3"
        @scroll="surDefilement"
      >
        <p v-if="chargementHistorique" class="text-center text-sm text-gray-400">
          Chargement des commentaires...
        </p>
        <p v-else-if="messages.length === 0" class="text-center text-sm text-gray-400">
          Aucun commentaire : lancez la discussion.
        </p>

        <div
          v-for="message in messages"
          :key="message.id"
          class="flex gap-2"
          :class="estMoi(message.auteur_id) ? 'justify-end' : 'justify-start'"
        >
          <div
            v-if="!estMoi(message.auteur_id)"
            class="mt-0.5 h-8 w-8 shrink-0 overflow-hidden rounded-full bg-custom-chocolat text-xs font-semibold text-white"
          >
            <img
              v-if="message.auteur_photo"
              :src="message.auteur_photo"
              :alt="nomAuteur(message)"
              class="h-full w-full object-cover"
            >
            <span v-else class="flex h-full w-full items-center justify-center">
              {{ initiales(message) }}
            </span>
          </div>

          <div
            class="max-w-[80%] rounded-2xl px-3 py-2 text-sm"
            :class="estMoi(message.auteur_id)
              ? 'bg-sky-600 text-white rounded-br-sm'
              : 'bg-gray-700 text-gray-100 rounded-bl-sm'"
          >
            <p
              v-if="!estMoi(message.auteur_id)"
              class="text-xs font-medium text-sky-300"
            >
              {{ nomAuteur(message) }}
            </p>
            <p class="wrap-break-word whitespace-pre-wrap">{{ message.contenu }}</p>
            <p class="mt-0.5 text-right text-[10px] opacity-60">{{ heure(message.created_at) }}</p>
          </div>
        </div>
      </div>

      <!-- Rappel de bas de flux : l'utilisateur lit plus haut et rate l'arrivée -->
      <button
        v-if="nouveauxEnBas > 0"
        type="button"
        class="absolute bottom-3 left-1/2 flex -translate-x-1/2 items-center gap-2 rounded-full bg-sky-600 px-3 py-1.5 text-xs font-medium text-white shadow-lg transition-colors hover:bg-sky-500"
        @click="colleEnBas = true; defiler(true)"
      >
        <font-awesome-icon :icon="['fas', 'arrow-down']" class="h-3 w-3" />
        {{ nouveauxEnBas }} nouveau{{ nouveauxEnBas > 1 ? 'x' : '' }} message{{ nouveauxEnBas > 1 ? 's' : '' }}
      </button>
    </div>

    <form class="border-t border-gray-700 p-3" @submit.prevent="envoyer">
      <p v-if="erreurEnvoi" class="mb-2 text-xs text-red-300">{{ erreurEnvoi }}</p>
      <div class="flex items-end gap-2">
        <textarea
          ref="champSaisie"
          v-model="nouveauMessage"
          rows="1"
          maxlength="4000"
          placeholder="Écrire un commentaire..."
          class="max-h-32 flex-1 resize-none rounded-xl border border-gray-600 bg-gray-900 px-3 py-2 text-sm text-gray-100 placeholder-gray-500 focus:border-sky-500 focus:outline-none"
          @keydown.enter.exact.prevent="envoyer"
        />
        <button
          type="submit"
          :disabled="envoyerEnCours || !nouveauMessage.trim()"
          class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-sky-600 text-white transition-colors hover:bg-sky-500 disabled:cursor-not-allowed disabled:opacity-40"
          aria-label="Envoyer le commentaire"
          title="Envoyer (Entrée) : Maj+Entrée pour aller à la ligne"
        >
          <font-awesome-icon
            :icon="['fas', envoyerEnCours ? 'spinner' : 'paper-plane']"
            class="h-4 w-4"
            :class="envoyerEnCours ? 'animate-spin' : ''"
          />
        </button>
      </div>
    </form>
  </section>
</template>
