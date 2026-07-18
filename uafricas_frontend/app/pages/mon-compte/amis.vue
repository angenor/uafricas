<script setup lang="ts">
import type {
  AmiAPI,
  BlocageAPI,
  DemandeEnvoyeeAPI,
  DemandeRecueAPI,
  MembreLightAPI,
  NotificationSocialeAPI,
} from '~/composables/useAmis'

definePageMeta({ middleware: 'auth' })

useHead({ title: 'Mes amis — AfricanS' })

const {
  listerDemandesRecues,
  accepterDemande,
  refuserDemande,
  listerNotifications,
  marquerNotificationLue,
  listerDemandesEnvoyees,
  annulerDemande,
  listerAmis,
  retirerAmi,
  bloquer,
  debloquer,
  listerBlocages,
} = useAmis()

const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string

type Onglet = 'amis' | 'recues' | 'envoyees' | 'bloques' | 'notifications'
const ongletActif = ref<Onglet>('amis')

const amis = ref<AmiAPI[]>([])
const demandes = ref<DemandeRecueAPI[]>([])
const envoyees = ref<DemandeEnvoyeeAPI[]>([])
const bloques = ref<BlocageAPI[]>([])
const notifications = ref<NotificationSocialeAPI[]>([])

const chargement = ref(false)
const traitement = ref<string | null>(null)
const rechercheAmis = ref('')

// Confirmation des actions destructives (retrait / blocage)
const confirmation = ref<{ action: 'retirer' | 'bloquer', membre: MembreLightAPI } | null>(null)

const onglets = computed(() => [
  { id: 'amis' as const, label: 'Amis', icon: 'fa-solid fa-user-check', badge: amis.value.length },
  { id: 'recues' as const, label: 'Demandes reçues', icon: 'fa-solid fa-user-clock', badge: demandes.value.length },
  { id: 'envoyees' as const, label: 'Demandes envoyées', icon: 'fa-solid fa-paper-plane', badge: envoyees.value.length },
  { id: 'bloques' as const, label: 'Bloqués', icon: 'fa-solid fa-ban', badge: bloques.value.length },
  { id: 'notifications' as const, label: 'Notifications', icon: 'fa-solid fa-bell', badge: notifications.value.filter(n => !n.lu).length },
])

// ── Helpers ──
const photoComplete = (url: string | null): string | null => {
  if (!url) return null
  return url.startsWith('http') ? url : `${apiBase}${url}`
}

const initiaux = (prenom: string, nom: string): string =>
  `${prenom?.charAt(0)?.toUpperCase() || ''}${nom?.charAt(0)?.toUpperCase() || ''}`

const dateFormatee = (iso: string): string =>
  new Date(iso).toLocaleDateString('fr-FR', { day: 'numeric', month: 'long', year: 'numeric' })

const libelleNotification = (n: NotificationSocialeAPI): string => {
  const acteur = n.acteur ? `${n.acteur.prenom} ${n.acteur.nom}` : 'Un membre'
  return n.type === 'demande_recue'
    ? `${acteur} vous a envoyé une demande d'amitié.`
    : `${acteur} a accepté votre demande d'amitié.`
}

// ── Chargement ──
const charger = async () => {
  chargement.value = true
  const [a, d, e, b, n] = await Promise.all([
    listerAmis(),
    listerDemandesRecues(),
    listerDemandesEnvoyees(),
    listerBlocages(),
    listerNotifications(),
  ])
  amis.value = a
  demandes.value = d
  envoyees.value = e
  bloques.value = b
  notifications.value = n
  chargement.value = false
}

// ── Recherche d'amis (filtre serveur) ──
let minuterieRecherche: ReturnType<typeof setTimeout> | null = null
watch(rechercheAmis, (v) => {
  if (minuterieRecherche) clearTimeout(minuterieRecherche)
  minuterieRecherche = setTimeout(async () => {
    amis.value = await listerAmis(v)
  }, 300)
})

// ── Actions US2 (demandes reçues) ──
const accepter = async (demandeId: string) => {
  traitement.value = demandeId
  const ok = await accepterDemande(demandeId)
  if (ok) {
    const acceptee = demandes.value.find(x => x.demande_id === demandeId)
    demandes.value = demandes.value.filter(x => x.demande_id !== demandeId)
    // Refléter l'ajout immédiat dans l'onglet Amis.
    if (acceptee) {
      amis.value = [{ utilisateur: acceptee.demandeur, ami_depuis: new Date().toISOString() }, ...amis.value]
    }
  }
  traitement.value = null
}

const refuser = async (demandeId: string) => {
  traitement.value = demandeId
  const ok = await refuserDemande(demandeId)
  if (ok) demandes.value = demandes.value.filter(x => x.demande_id !== demandeId)
  traitement.value = null
}

// ── Actions US4 ──
const annuler = async (demandeId: string) => {
  traitement.value = demandeId
  const ok = await annulerDemande(demandeId)
  if (ok) envoyees.value = envoyees.value.filter(x => x.demande_id !== demandeId)
  traitement.value = null
}

const debloquerMembre = async (utilisateurId: string) => {
  traitement.value = utilisateurId
  const ok = await debloquer(utilisateurId)
  if (ok) bloques.value = bloques.value.filter(x => x.utilisateur.id !== utilisateurId)
  traitement.value = null
}

// Confirmation puis exécution (retrait / blocage)
const demanderConfirmation = (action: 'retirer' | 'bloquer', membre: MembreLightAPI) => {
  confirmation.value = { action, membre }
}

const confirmerAction = async () => {
  if (!confirmation.value) return
  const { action, membre } = confirmation.value
  traitement.value = membre.id
  if (action === 'retirer') {
    const ok = await retirerAmi(membre.id)
    if (ok) amis.value = amis.value.filter(a => a.utilisateur.id !== membre.id)
  }
  else {
    const ok = await bloquer(membre.id)
    if (ok) {
      // Le membre quitte l'onglet Amis et rejoint les Bloqués.
      amis.value = amis.value.filter(a => a.utilisateur.id !== membre.id)
      bloques.value = [{ utilisateur: membre, depuis: new Date().toISOString() }, ...bloques.value]
    }
  }
  traitement.value = null
  confirmation.value = null
}

const marquerLue = async (n: NotificationSocialeAPI) => {
  if (n.lu) return
  const ok = await marquerNotificationLue(n.id)
  if (ok) n.lu = true
}

onMounted(charger)
</script>

<template>
  <div class="min-h-screen bg-linear-to-br from-gray-50 via-white to-gray-50 pt-28 pb-16">
    <div class="max-w-3xl mx-auto px-4">

      <!-- En-tête -->
      <div class="mb-8">
        <h1 class="text-3xl font-bold text-gray-800 font-display">Mes amis</h1>
        <p class="text-gray-500 mt-2 text-sm">
          Gérez vos amitiés, vos demandes et les membres que vous avez bloqués.
        </p>
      </div>

      <!-- Onglets -->
      <div class="bg-white rounded-2xl shadow-lg overflow-hidden">
        <div class="flex border-b border-gray-200 overflow-x-auto">
          <button
            v-for="tab in onglets"
            :key="tab.id"
            class="flex items-center justify-center gap-2 px-5 py-4 text-sm font-medium transition-all relative whitespace-nowrap"
            :class="ongletActif === tab.id ? 'text-custom-chocolat' : 'text-gray-500 hover:text-gray-700 hover:bg-gray-50'"
            @click="ongletActif = tab.id"
          >
            <font-awesome-icon :icon="tab.icon" />
            {{ tab.label }}
            <span
              v-if="tab.badge > 0"
              class="ml-1 bg-custom-chocolat text-white text-[10px] font-semibold px-1.5 py-0.5 rounded-full"
            >{{ tab.badge }}</span>
            <div v-if="ongletActif === tab.id" class="absolute bottom-0 left-0 right-0 h-0.5 bg-custom-chocolat"></div>
          </button>
        </div>

        <div class="p-6">
          <!-- Chargement -->
          <div v-if="chargement" class="flex justify-center py-16">
            <font-awesome-icon icon="fa-solid fa-spinner" class="text-3xl text-custom-chocolat animate-spin" />
          </div>

          <!-- Onglet Amis -->
          <template v-else-if="ongletActif === 'amis'">
            <div class="mb-4">
              <div class="relative">
                <font-awesome-icon icon="fa-solid fa-magnifying-glass" class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 text-sm" />
                <input
                  v-model="rechercheAmis"
                  type="search"
                  placeholder="Rechercher un ami…"
                  class="w-full rounded-xl border border-gray-200 pl-9 pr-3 py-2.5 text-sm bg-gray-50 focus:bg-white focus:ring-2 focus:ring-custom-green focus:border-transparent transition"
                />
              </div>
            </div>

            <div v-if="amis.length === 0" class="text-center py-12">
              <div class="w-16 h-16 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-4">
                <font-awesome-icon icon="fa-solid fa-users" class="text-2xl text-gray-400" />
              </div>
              <p class="text-gray-500 text-sm">Vous n'avez pas encore d'amis. Faites-vous des amis depuis l'annuaire !</p>
              <NuxtLink to="/profil" class="inline-block mt-3 text-sm font-semibold text-custom-chocolat hover:underline">
                Parcourir les membres
              </NuxtLink>
            </div>

            <ul v-else class="divide-y divide-gray-100">
              <li v-for="a in amis" :key="a.utilisateur.id" class="flex items-center gap-4 py-4">
                <NuxtLink :to="`/profil/${a.utilisateur.id}`" class="shrink-0">
                  <img
                    v-if="photoComplete(a.utilisateur.photoUrl)"
                    :src="photoComplete(a.utilisateur.photoUrl)!"
                    :alt="`${a.utilisateur.prenom} ${a.utilisateur.nom}`"
                    class="w-12 h-12 rounded-full object-cover border border-gray-200"
                  />
                  <div v-else class="w-12 h-12 rounded-full bg-custom-chocolat text-white flex items-center justify-center font-bold">
                    {{ initiaux(a.utilisateur.prenom, a.utilisateur.nom) }}
                  </div>
                </NuxtLink>
                <div class="flex-1 min-w-0">
                  <NuxtLink :to="`/profil/${a.utilisateur.id}`" class="font-semibold text-gray-800 hover:text-custom-chocolat transition truncate block">
                    {{ a.utilisateur.prenom }} {{ a.utilisateur.nom }}
                  </NuxtLink>
                  <p v-if="a.utilisateur.fonction" class="text-xs text-gray-500 truncate">{{ a.utilisateur.fonction }}</p>
                  <p class="text-xs text-gray-400 mt-0.5">Amis depuis le {{ dateFormatee(a.ami_depuis) }}</p>
                </div>
                <div class="flex items-center gap-2 shrink-0">
                  <button
                    type="button"
                    :disabled="traitement === a.utilisateur.id"
                    class="inline-flex items-center gap-1.5 px-3 py-2 text-xs font-semibold text-gray-600 bg-gray-100 rounded-lg hover:bg-gray-200 transition disabled:opacity-60"
                    @click="demanderConfirmation('retirer', a.utilisateur)"
                  >
                    <font-awesome-icon icon="fa-solid fa-user-xmark" />
                    Retirer
                  </button>
                  <button
                    type="button"
                    :disabled="traitement === a.utilisateur.id"
                    class="inline-flex items-center gap-1.5 px-3 py-2 text-xs font-semibold text-red-600 bg-red-50 rounded-lg hover:bg-red-100 transition disabled:opacity-60"
                    @click="demanderConfirmation('bloquer', a.utilisateur)"
                  >
                    <font-awesome-icon icon="fa-solid fa-ban" />
                    Bloquer
                  </button>
                </div>
              </li>
            </ul>
          </template>

          <!-- Onglet Demandes reçues -->
          <template v-else-if="ongletActif === 'recues'">
            <div v-if="demandes.length === 0" class="text-center py-12">
              <div class="w-16 h-16 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-4">
                <font-awesome-icon icon="fa-solid fa-inbox" class="text-2xl text-gray-400" />
              </div>
              <p class="text-gray-500 text-sm">Aucune demande d'amitié en attente.</p>
            </div>

            <ul v-else class="divide-y divide-gray-100">
              <li v-for="d in demandes" :key="d.demande_id" class="flex items-center gap-4 py-4">
                <NuxtLink :to="`/profil/${d.demandeur.id}`" class="shrink-0">
                  <img
                    v-if="photoComplete(d.demandeur.photoUrl)"
                    :src="photoComplete(d.demandeur.photoUrl)!"
                    :alt="`${d.demandeur.prenom} ${d.demandeur.nom}`"
                    class="w-12 h-12 rounded-full object-cover border border-gray-200"
                  />
                  <div v-else class="w-12 h-12 rounded-full bg-custom-chocolat text-white flex items-center justify-center font-bold">
                    {{ initiaux(d.demandeur.prenom, d.demandeur.nom) }}
                  </div>
                </NuxtLink>
                <div class="flex-1 min-w-0">
                  <NuxtLink :to="`/profil/${d.demandeur.id}`" class="font-semibold text-gray-800 hover:text-custom-chocolat transition truncate block">
                    {{ d.demandeur.prenom }} {{ d.demandeur.nom }}
                  </NuxtLink>
                  <p v-if="d.demandeur.fonction" class="text-xs text-gray-500 truncate">{{ d.demandeur.fonction }}</p>
                  <p class="text-xs text-gray-400 mt-0.5">{{ dateFormatee(d.created_at) }}</p>
                </div>
                <div class="flex items-center gap-2 shrink-0">
                  <button
                    type="button"
                    :disabled="traitement === d.demande_id"
                    class="inline-flex items-center gap-1.5 px-3 py-2 text-xs font-semibold text-white bg-custom-green rounded-lg hover:shadow-md transition disabled:opacity-60"
                    @click="accepter(d.demande_id)"
                  >
                    <font-awesome-icon icon="fa-solid fa-user-check" />
                    Accepter
                  </button>
                  <button
                    type="button"
                    :disabled="traitement === d.demande_id"
                    class="inline-flex items-center gap-1.5 px-3 py-2 text-xs font-semibold text-gray-600 bg-gray-100 rounded-lg hover:bg-gray-200 transition disabled:opacity-60"
                    @click="refuser(d.demande_id)"
                  >
                    <font-awesome-icon icon="fa-solid fa-user-xmark" />
                    Refuser
                  </button>
                </div>
              </li>
            </ul>
          </template>

          <!-- Onglet Demandes envoyées -->
          <template v-else-if="ongletActif === 'envoyees'">
            <div v-if="envoyees.length === 0" class="text-center py-12">
              <div class="w-16 h-16 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-4">
                <font-awesome-icon icon="fa-solid fa-paper-plane" class="text-2xl text-gray-400" />
              </div>
              <p class="text-gray-500 text-sm">Aucune demande envoyée en attente.</p>
            </div>

            <ul v-else class="divide-y divide-gray-100">
              <li v-for="d in envoyees" :key="d.demande_id" class="flex items-center gap-4 py-4">
                <NuxtLink :to="`/profil/${d.destinataire.id}`" class="shrink-0">
                  <img
                    v-if="photoComplete(d.destinataire.photoUrl)"
                    :src="photoComplete(d.destinataire.photoUrl)!"
                    :alt="`${d.destinataire.prenom} ${d.destinataire.nom}`"
                    class="w-12 h-12 rounded-full object-cover border border-gray-200"
                  />
                  <div v-else class="w-12 h-12 rounded-full bg-custom-chocolat text-white flex items-center justify-center font-bold">
                    {{ initiaux(d.destinataire.prenom, d.destinataire.nom) }}
                  </div>
                </NuxtLink>
                <div class="flex-1 min-w-0">
                  <NuxtLink :to="`/profil/${d.destinataire.id}`" class="font-semibold text-gray-800 hover:text-custom-chocolat transition truncate block">
                    {{ d.destinataire.prenom }} {{ d.destinataire.nom }}
                  </NuxtLink>
                  <p v-if="d.destinataire.fonction" class="text-xs text-gray-500 truncate">{{ d.destinataire.fonction }}</p>
                  <p class="text-xs text-gray-400 mt-0.5">Envoyée le {{ dateFormatee(d.created_at) }}</p>
                </div>
                <button
                  type="button"
                  :disabled="traitement === d.demande_id"
                  class="inline-flex items-center gap-1.5 px-3 py-2 text-xs font-semibold text-gray-600 bg-gray-100 rounded-lg hover:bg-gray-200 transition disabled:opacity-60 shrink-0"
                  @click="annuler(d.demande_id)"
                >
                  <font-awesome-icon icon="fa-solid fa-xmark" />
                  Annuler
                </button>
              </li>
            </ul>
          </template>

          <!-- Onglet Bloqués -->
          <template v-else-if="ongletActif === 'bloques'">
            <div v-if="bloques.length === 0" class="text-center py-12">
              <div class="w-16 h-16 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-4">
                <font-awesome-icon icon="fa-solid fa-ban" class="text-2xl text-gray-400" />
              </div>
              <p class="text-gray-500 text-sm">Vous n'avez bloqué aucun membre.</p>
            </div>

            <ul v-else class="divide-y divide-gray-100">
              <li v-for="b in bloques" :key="b.utilisateur.id" class="flex items-center gap-4 py-4">
                <div class="shrink-0">
                  <img
                    v-if="photoComplete(b.utilisateur.photoUrl)"
                    :src="photoComplete(b.utilisateur.photoUrl)!"
                    :alt="`${b.utilisateur.prenom} ${b.utilisateur.nom}`"
                    class="w-12 h-12 rounded-full object-cover border border-gray-200 grayscale"
                  />
                  <div v-else class="w-12 h-12 rounded-full bg-gray-400 text-white flex items-center justify-center font-bold">
                    {{ initiaux(b.utilisateur.prenom, b.utilisateur.nom) }}
                  </div>
                </div>
                <div class="flex-1 min-w-0">
                  <p class="font-semibold text-gray-700 truncate">{{ b.utilisateur.prenom }} {{ b.utilisateur.nom }}</p>
                  <p class="text-xs text-gray-400 mt-0.5">Bloqué le {{ dateFormatee(b.depuis) }}</p>
                </div>
                <button
                  type="button"
                  :disabled="traitement === b.utilisateur.id"
                  class="inline-flex items-center gap-1.5 px-3 py-2 text-xs font-semibold text-custom-green bg-custom-green/10 rounded-lg hover:bg-custom-green/20 transition disabled:opacity-60 shrink-0"
                  @click="debloquerMembre(b.utilisateur.id)"
                >
                  <font-awesome-icon icon="fa-solid fa-user-check" />
                  Débloquer
                </button>
              </li>
            </ul>
          </template>

          <!-- Onglet Notifications -->
          <template v-else-if="ongletActif === 'notifications'">
            <div v-if="notifications.length === 0" class="text-center py-12">
              <div class="w-16 h-16 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-4">
                <font-awesome-icon icon="fa-solid fa-bell" class="text-2xl text-gray-400" />
              </div>
              <p class="text-gray-500 text-sm">Aucune notification.</p>
            </div>

            <ul v-else class="divide-y divide-gray-100">
              <li
                v-for="n in notifications"
                :key="n.id"
                class="flex items-center gap-3 py-3.5 cursor-pointer"
                :class="!n.lu ? 'bg-custom-chocolat/5 -mx-6 px-6' : ''"
                @click="marquerLue(n)"
              >
                <div
                  class="w-9 h-9 rounded-full flex items-center justify-center shrink-0"
                  :class="n.type === 'demande_acceptee' ? 'bg-custom-green/10 text-custom-green' : 'bg-custom-chocolat/10 text-custom-chocolat'"
                >
                  <font-awesome-icon :icon="n.type === 'demande_acceptee' ? 'fa-solid fa-user-check' : 'fa-solid fa-user-plus'" />
                </div>
                <div class="flex-1 min-w-0">
                  <p class="text-sm text-gray-700">{{ libelleNotification(n) }}</p>
                  <p class="text-xs text-gray-400 mt-0.5">{{ dateFormatee(n.created_at) }}</p>
                </div>
                <span v-if="!n.lu" class="w-2 h-2 rounded-full bg-custom-chocolat shrink-0"></span>
              </li>
            </ul>
          </template>
        </div>
      </div>
    </div>

    <!-- Modale de confirmation (retrait / blocage) -->
    <Teleport to="body">
      <div
        v-if="confirmation"
        class="fixed inset-0 z-[60] flex items-center justify-center bg-black/40 px-4"
        @click.self="confirmation = null"
      >
        <div class="bg-white rounded-2xl shadow-xl max-w-sm w-full p-6">
          <div
            class="w-12 h-12 rounded-full flex items-center justify-center mx-auto mb-4"
            :class="confirmation.action === 'bloquer' ? 'bg-red-50 text-red-600' : 'bg-gray-100 text-gray-500'"
          >
            <font-awesome-icon :icon="confirmation.action === 'bloquer' ? 'fa-solid fa-ban' : 'fa-solid fa-user-xmark'" class="text-xl" />
          </div>
          <h3 class="text-center text-lg font-bold text-gray-800">
            {{ confirmation.action === 'bloquer' ? 'Bloquer ce membre ?' : 'Retirer cet ami ?' }}
          </h3>
          <p class="text-center text-sm text-gray-500 mt-2">
            <template v-if="confirmation.action === 'bloquer'">
              {{ confirmation.membre.prenom }} {{ confirmation.membre.nom }} ne pourra plus vous contacter ni vous envoyer de demande. Votre amitié et vos demandes en cours seront supprimées.
            </template>
            <template v-else>
              Vous ne serez plus amis avec {{ confirmation.membre.prenom }} {{ confirmation.membre.nom }}. Votre conversation sera conservée mais verrouillée.
            </template>
          </p>
          <div class="flex gap-3 mt-6">
            <button
              type="button"
              class="flex-1 px-4 py-2.5 text-sm font-semibold text-gray-600 bg-gray-100 rounded-xl hover:bg-gray-200 transition"
              @click="confirmation = null"
            >
              Annuler
            </button>
            <button
              type="button"
              :disabled="traitement === confirmation.membre.id"
              class="flex-1 px-4 py-2.5 text-sm font-semibold text-white rounded-xl transition disabled:opacity-60"
              :class="confirmation.action === 'bloquer' ? 'bg-red-600 hover:bg-red-700' : 'bg-custom-chocolat hover:shadow-md'"
              @click="confirmerAction"
            >
              {{ confirmation.action === 'bloquer' ? 'Bloquer' : 'Retirer' }}
            </button>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>
