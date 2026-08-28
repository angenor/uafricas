<script setup lang="ts">
import type {
  AmiAPI,
  BlocageAPI,
  DemandeEnvoyeeAPI,
  DemandeRecueAPI,
  MembreLightAPI,
  NotificationSocialeAPI,
} from '~/composables/useAmis'

definePageMeta({ layout: false, middleware: 'auth' })

useHead({ title: 'Mes amis | AfricanS' })

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

/** L'onglet retenu, nommé dans la barre de contexte : les onglets ayant quitté
 *  la colonne principale, rien d'autre ne dirait lequel est ouvert. */
const ongletCourant = computed(() => onglets.value.find(o => o.id === ongletActif.value) ?? onglets.value[0]!)

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
  <NuxtLayout name="africans">
    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[{ libelle: 'Mon compte', vers: '/mon-compte/profil' }, { libelle: 'Mes ami(e)s' }]"
      >
        <template #centre>
          <p class="text-base font-bold text-af-encre">{{ ongletCourant.label }}</p>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="flex flex-col gap-6">
      <header>
        <h1 class="text-[24px]/[1.3] font-bold text-af-encre">Mes ami(e)s</h1>
        <p class="mt-1 text-[14px]/[1.5] text-af-corps">
          Gérez vos amitiés, vos demandes et les membres que vous avez bloqués.
        </p>
      </header>

      <!-- Les cinq onglets sont passés dans le rail. En barre horizontale ils
           ne tenaient pas : le cinquième, « Notifications », était coupé au
           bord du cadre, et son `overflow-x-auto` ne l'annonçait par aucune
           ombre ni flèche — on ne pouvait le trouver qu'en tirant au hasard. -->
      <div class="rounded-[10px] border border-af-bordure bg-white p-6">
          <!-- Chargement -->
          <div v-if="chargement" class="flex justify-center py-16">
            <font-awesome-icon icon="fa-solid fa-spinner" class="text-3xl text-af-chocolat animate-spin" />
          </div>

          <!-- Onglet Amis -->
          <template v-else-if="ongletActif === 'amis'">
            <div class="mb-4">
              <div class="relative">
                <font-awesome-icon icon="fa-solid fa-magnifying-glass" class="absolute left-3 top-1/2 -translate-y-1/2 text-af-atone-2 text-sm" />
                <input
                  v-model="rechercheAmis"
                  type="search"
                  placeholder="Rechercher un ami…"
                  class="w-full rounded-lg border border-af-bordure pl-9 pr-3 py-2.5 text-sm bg-af-fond focus:bg-white focus:ring-2 focus:ring-af-vert focus:border-transparent transition"
                />
              </div>
            </div>

            <div v-if="amis.length === 0" class="text-center py-12">
              <div class="w-16 h-16 bg-af-fond rounded-full flex items-center justify-center mx-auto mb-4">
                <font-awesome-icon icon="fa-solid fa-users" class="text-2xl text-af-atone-2" />
              </div>
              <p class="text-af-atone text-sm">Vous n'avez pas encore d'amis. Faites-vous des amis depuis l'annuaire !</p>
              <NuxtLink to="/profil" class="inline-block mt-3 text-sm font-semibold text-af-chocolat hover:underline">
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
                    class="w-12 h-12 rounded-full object-cover border border-af-bordure"
                  />
                  <div v-else class="w-12 h-12 rounded-full bg-af-chocolat text-white flex items-center justify-center font-bold">
                    {{ initiaux(a.utilisateur.prenom, a.utilisateur.nom) }}
                  </div>
                </NuxtLink>
                <div class="flex-1 min-w-0">
                  <NuxtLink :to="`/profil/${a.utilisateur.id}`" class="font-semibold text-af-encre hover:text-af-chocolat transition truncate block">
                    {{ a.utilisateur.prenom }} {{ a.utilisateur.nom }}
                  </NuxtLink>
                  <p v-if="a.utilisateur.fonction" class="text-xs text-af-atone truncate">{{ a.utilisateur.fonction }}</p>
                  <p class="text-xs text-af-atone-2 mt-0.5">Amis depuis le {{ dateFormatee(a.ami_depuis) }}</p>
                </div>
                <div class="flex items-center gap-2 shrink-0">
                  <button
                    type="button"
                    :disabled="traitement === a.utilisateur.id"
                    class="inline-flex items-center gap-1.5 px-3 py-2 text-xs font-semibold text-af-corps bg-af-fond rounded-lg hover:bg-af-bordure transition disabled:opacity-60"
                    @click="demanderConfirmation('retirer', a.utilisateur)"
                  >
                    <font-awesome-icon icon="fa-solid fa-user-xmark" />
                    Retirer
                  </button>
                  <button
                    type="button"
                    :disabled="traitement === a.utilisateur.id"
                    class="inline-flex items-center gap-1.5 px-3 py-2 text-xs font-semibold text-af-live bg-af-live/5 rounded-lg hover:bg-af-live/10 transition disabled:opacity-60"
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
              <div class="w-16 h-16 bg-af-fond rounded-full flex items-center justify-center mx-auto mb-4">
                <font-awesome-icon icon="fa-solid fa-inbox" class="text-2xl text-af-atone-2" />
              </div>
              <p class="text-af-atone text-sm">Aucune demande d'amitié en attente.</p>
            </div>

            <ul v-else class="divide-y divide-gray-100">
              <li v-for="d in demandes" :key="d.demande_id" class="flex items-center gap-4 py-4">
                <NuxtLink :to="`/profil/${d.demandeur.id}`" class="shrink-0">
                  <img
                    v-if="photoComplete(d.demandeur.photoUrl)"
                    :src="photoComplete(d.demandeur.photoUrl)!"
                    :alt="`${d.demandeur.prenom} ${d.demandeur.nom}`"
                    class="w-12 h-12 rounded-full object-cover border border-af-bordure"
                  />
                  <div v-else class="w-12 h-12 rounded-full bg-af-chocolat text-white flex items-center justify-center font-bold">
                    {{ initiaux(d.demandeur.prenom, d.demandeur.nom) }}
                  </div>
                </NuxtLink>
                <div class="flex-1 min-w-0">
                  <NuxtLink :to="`/profil/${d.demandeur.id}`" class="font-semibold text-af-encre hover:text-af-chocolat transition truncate block">
                    {{ d.demandeur.prenom }} {{ d.demandeur.nom }}
                  </NuxtLink>
                  <p v-if="d.demandeur.fonction" class="text-xs text-af-atone truncate">{{ d.demandeur.fonction }}</p>
                  <p class="text-xs text-af-atone-2 mt-0.5">{{ dateFormatee(d.created_at) }}</p>
                </div>
                <div class="flex items-center gap-2 shrink-0">
                  <button
                    type="button"
                    :disabled="traitement === d.demande_id"
                    class="inline-flex items-center gap-1.5 px-3 py-2 text-xs font-semibold text-white bg-af-vert rounded-lg hover:shadow-md transition disabled:opacity-60"
                    @click="accepter(d.demande_id)"
                  >
                    <font-awesome-icon icon="fa-solid fa-user-check" />
                    Accepter
                  </button>
                  <button
                    type="button"
                    :disabled="traitement === d.demande_id"
                    class="inline-flex items-center gap-1.5 px-3 py-2 text-xs font-semibold text-af-corps bg-af-fond rounded-lg hover:bg-af-bordure transition disabled:opacity-60"
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
              <div class="w-16 h-16 bg-af-fond rounded-full flex items-center justify-center mx-auto mb-4">
                <font-awesome-icon icon="fa-solid fa-paper-plane" class="text-2xl text-af-atone-2" />
              </div>
              <p class="text-af-atone text-sm">Aucune demande envoyée en attente.</p>
            </div>

            <ul v-else class="divide-y divide-gray-100">
              <li v-for="d in envoyees" :key="d.demande_id" class="flex items-center gap-4 py-4">
                <NuxtLink :to="`/profil/${d.destinataire.id}`" class="shrink-0">
                  <img
                    v-if="photoComplete(d.destinataire.photoUrl)"
                    :src="photoComplete(d.destinataire.photoUrl)!"
                    :alt="`${d.destinataire.prenom} ${d.destinataire.nom}`"
                    class="w-12 h-12 rounded-full object-cover border border-af-bordure"
                  />
                  <div v-else class="w-12 h-12 rounded-full bg-af-chocolat text-white flex items-center justify-center font-bold">
                    {{ initiaux(d.destinataire.prenom, d.destinataire.nom) }}
                  </div>
                </NuxtLink>
                <div class="flex-1 min-w-0">
                  <NuxtLink :to="`/profil/${d.destinataire.id}`" class="font-semibold text-af-encre hover:text-af-chocolat transition truncate block">
                    {{ d.destinataire.prenom }} {{ d.destinataire.nom }}
                  </NuxtLink>
                  <p v-if="d.destinataire.fonction" class="text-xs text-af-atone truncate">{{ d.destinataire.fonction }}</p>
                  <p class="text-xs text-af-atone-2 mt-0.5">Envoyée le {{ dateFormatee(d.created_at) }}</p>
                </div>
                <button
                  type="button"
                  :disabled="traitement === d.demande_id"
                  class="inline-flex items-center gap-1.5 px-3 py-2 text-xs font-semibold text-af-corps bg-af-fond rounded-lg hover:bg-af-bordure transition disabled:opacity-60 shrink-0"
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
              <div class="w-16 h-16 bg-af-fond rounded-full flex items-center justify-center mx-auto mb-4">
                <font-awesome-icon icon="fa-solid fa-ban" class="text-2xl text-af-atone-2" />
              </div>
              <p class="text-af-atone text-sm">Vous n'avez bloqué aucun membre.</p>
            </div>

            <ul v-else class="divide-y divide-gray-100">
              <li v-for="b in bloques" :key="b.utilisateur.id" class="flex items-center gap-4 py-4">
                <div class="shrink-0">
                  <img
                    v-if="photoComplete(b.utilisateur.photoUrl)"
                    :src="photoComplete(b.utilisateur.photoUrl)!"
                    :alt="`${b.utilisateur.prenom} ${b.utilisateur.nom}`"
                    class="w-12 h-12 rounded-full object-cover border border-af-bordure grayscale"
                  />
                  <div v-else class="w-12 h-12 rounded-full bg-af-atone-2 text-white flex items-center justify-center font-bold">
                    {{ initiaux(b.utilisateur.prenom, b.utilisateur.nom) }}
                  </div>
                </div>
                <div class="flex-1 min-w-0">
                  <p class="font-semibold text-af-corps truncate">{{ b.utilisateur.prenom }} {{ b.utilisateur.nom }}</p>
                  <p class="text-xs text-af-atone-2 mt-0.5">Bloqué le {{ dateFormatee(b.depuis) }}</p>
                </div>
                <button
                  type="button"
                  :disabled="traitement === b.utilisateur.id"
                  class="inline-flex items-center gap-1.5 px-3 py-2 text-xs font-semibold text-af-vert bg-af-vert/10 rounded-lg hover:bg-af-vert/20 transition disabled:opacity-60 shrink-0"
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
              <div class="w-16 h-16 bg-af-fond rounded-full flex items-center justify-center mx-auto mb-4">
                <font-awesome-icon icon="fa-solid fa-bell" class="text-2xl text-af-atone-2" />
              </div>
              <p class="text-af-atone text-sm">Aucune notification.</p>
            </div>

            <ul v-else class="divide-y divide-gray-100">
              <li
                v-for="n in notifications"
                :key="n.id"
                class="flex items-center gap-3 py-3.5 cursor-pointer"
                :class="!n.lu ? 'bg-af-chocolat/5 -mx-6 px-6' : ''"
                @click="marquerLue(n)"
              >
                <div
                  class="w-9 h-9 rounded-full flex items-center justify-center shrink-0"
                  :class="n.type === 'demande_acceptee' ? 'bg-af-vert/10 text-af-vert' : 'bg-af-chocolat/10 text-af-chocolat'"
                >
                  <font-awesome-icon :icon="n.type === 'demande_acceptee' ? 'fa-solid fa-user-check' : 'fa-solid fa-user-plus'" />
                </div>
                <div class="flex-1 min-w-0">
                  <p class="text-sm text-af-corps">{{ libelleNotification(n) }}</p>
                  <p class="text-xs text-af-atone-2 mt-0.5">{{ dateFormatee(n.created_at) }}</p>
                </div>
                <span v-if="!n.lu" class="w-2 h-2 rounded-full bg-af-chocolat shrink-0"></span>
              </li>
            </ul>
          </template>
      </div>
    </div>

    <template #rail>
      <AfricansPanneau titre="Mes relations" icone="fa-solid fa-user-group">
        <nav class="flex flex-col gap-1" aria-label="Sections de mes ami(e)s">
          <button
            v-for="tab in onglets"
            :key="tab.id"
            type="button"
            class="flex w-full items-center gap-3 rounded-lg px-3 py-2.5 text-left text-[14px]/[1.4] transition"
            :class="ongletActif === tab.id
              ? 'bg-af-chocolat/15 font-bold text-af-chocolat'
              : 'text-af-corps hover:bg-af-chocolat/[0.07]'"
            :aria-current="ongletActif === tab.id ? 'page' : undefined"
            @click="ongletActif = tab.id"
          >
            <font-awesome-icon :icon="tab.icon" class="w-4 shrink-0 text-center" />
            <span class="min-w-0 flex-1 truncate">{{ tab.label }}</span>
            <span
              v-if="tab.badge > 0"
              class="inline-flex h-5 min-w-5 shrink-0 items-center justify-center rounded-full px-1.5 text-[11px] font-bold"
              :class="ongletActif === tab.id ? 'bg-af-chocolat text-white' : 'bg-af-fond text-af-corps'"
            >{{ tab.badge }}</span>
          </button>
        </nav>
      </AfricansPanneau>

      <ComptePanneauNavigation />
    </template>

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
            :class="confirmation.action === 'bloquer' ? 'bg-af-live/5 text-af-live' : 'bg-af-fond text-af-atone'"
          >
            <font-awesome-icon :icon="confirmation.action === 'bloquer' ? 'fa-solid fa-ban' : 'fa-solid fa-user-xmark'" class="text-xl" />
          </div>
          <h3 class="text-center text-lg font-bold text-af-encre">
            {{ confirmation.action === 'bloquer' ? 'Bloquer ce membre ?' : 'Retirer cet ami ?' }}
          </h3>
          <p class="text-center text-sm text-af-atone mt-2">
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
              class="flex-1 px-4 py-2.5 text-sm font-semibold text-af-corps bg-af-fond rounded-lg hover:bg-af-bordure transition"
              @click="confirmation = null"
            >
              Annuler
            </button>
            <button
              type="button"
              :disabled="traitement === confirmation.membre.id"
              class="flex-1 px-4 py-2.5 text-sm font-semibold text-white rounded-lg transition disabled:opacity-60"
              :class="confirmation.action === 'bloquer' ? 'bg-af-live hover:opacity-90' : 'bg-af-chocolat hover:shadow-md'"
              @click="confirmerAction"
            >
              {{ confirmation.action === 'bloquer' ? 'Bloquer' : 'Retirer' }}
            </button>
          </div>
        </div>
      </div>
    </Teleport>
  </NuxtLayout>
</template>
