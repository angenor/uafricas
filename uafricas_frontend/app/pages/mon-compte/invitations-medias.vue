<template>
  <NuxtLayout name="africans">
    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[{ libelle: 'Mon compte', vers: '/mon-compte/profil' }, { libelle: 'Invitations médias' }]"
      />
    </template>

    <div class="flex flex-col gap-6">
      <header>
        <h1 class="text-[24px]/[1.3] font-bold text-af-encre">Mes invitations médias</h1>
        <p class="mt-1 text-[14px]/[1.5] text-af-corps">Les propositions de co-détenir une chaîne ou une station qui vous ont été adressées.</p>
      </header>
      <div v-if="acceptationReussie" class="mb-6 rounded-lg bg-af-vert/5 border border-af-vert/30 px-4 py-3 text-sm text-af-vert">
        Invitation acceptée.
        <NuxtLink to="/mon-compte/mes-supports" class="font-semibold underline hover:no-underline">
          Voir mes supports
        </NuxtLink>
      </div>

      <div v-if="erreur" class="mb-6 rounded-lg bg-af-live/5 border border-af-live/30 px-4 py-3 text-sm text-af-live">
        {{ erreur }}
      </div>

      <div v-if="chargement" class="flex justify-center py-16">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-af-chocolat"></div>
      </div>

      <div v-else-if="invitations.length === 0" class="text-center py-16">
        <font-awesome-icon :icon="['fas', 'envelope-open']" class="w-12 h-12 text-af-atone-2 mb-4" />
        <p class="text-af-corps mb-1">Aucune invitation reçue.</p>
        <p class="text-sm text-af-atone-2">
          Un propriétaire de chaîne ou de station peut vous inviter à rejoindre son équipe.
        </p>
      </div>

      <ul v-else class="space-y-4">
        <li
          v-for="invitation in invitations"
          :key="invitation.id"
          class="bg-white rounded-lg shadow-sm border border-af-bordure p-5"
          :class="{ 'opacity-60': invitation.expiree }"
        >
          <div class="flex items-start justify-between gap-4 flex-wrap mb-2">
            <div class="min-w-0">
              <h2 class="font-semibold text-af-encre">
                {{ invitation.support_nom || LIBELLES_TYPE_SUPPORT[invitation.type_support] }}
              </h2>
              <p class="text-xs text-af-atone-2 mt-0.5">
                {{ LIBELLES_TYPE_SUPPORT[invitation.type_support] }} ·
                invitation du {{ dateFormatee(invitation.created_at) }}
              </p>
            </div>
            <span
              class="shrink-0 rounded-full px-3 py-1 text-xs font-bold uppercase tracking-wide"
              :class="styleStatut(invitation)"
            >
              {{ libelleStatut(invitation) }}
            </span>
          </div>

          <p class="text-sm text-af-corps">
            <span class="font-medium text-af-encre">{{ invitant(invitation) }}</span>
            vous propose le rôle de
            <span class="font-medium text-af-encre">{{ LIBELLES_ROLE_DETENTEUR[invitation.role] }}</span>.
          </p>
          <p class="mt-1 text-xs text-af-atone">
            {{ DESCRIPTIONS_ROLE_DETENTEUR[invitation.role] }}
          </p>

          <!-- Expirée : on l'annonce plutôt que de laisser des boutons que le
               serveur rejetterait de toute façon. -->
          <p v-if="invitation.expiree && invitation.statut === 'en_attente'" class="mt-3 text-sm text-af-atone">
            <font-awesome-icon :icon="['fas', 'clock']" class="w-3.5 h-3.5 mr-1" />
            Cette invitation a expiré le {{ dateFormatee(invitation.expire_at) }} ; il n’est plus possible d’y répondre.
          </p>

          <div v-else-if="estActionnable(invitation)" class="mt-4 flex flex-wrap items-center gap-3">
            <button
              type="button"
              class="cursor-pointer rounded-full bg-af-vert px-5 py-1.5 text-sm font-medium text-white transition-colors hover:bg-af-vert/90 disabled:cursor-not-allowed disabled:opacity-50"
              :disabled="enCours === invitation.id"
              @click="repondre(invitation, 'accepter')"
            >
              Accepter
            </button>
            <button
              type="button"
              class="cursor-pointer rounded-full border border-af-bordure bg-white px-5 py-1.5 text-sm font-medium text-af-corps transition-colors hover:bg-af-fond disabled:cursor-not-allowed disabled:opacity-50"
              :disabled="enCours === invitation.id"
              @click="repondre(invitation, 'refuser')"
            >
              Refuser
            </button>
            <span class="text-xs text-af-atone-2">
              À répondre avant le {{ dateFormatee(invitation.expire_at) }}
            </span>
          </div>

          <p v-else-if="invitation.statut === 'acceptee'" class="mt-3 text-sm">
            <NuxtLink to="/mon-compte/mes-supports" class="text-af-chocolat font-medium hover:underline">
              Retrouver ce support dans « Mes supports »
            </NuxtLink>
          </p>
        </li>
      </ul>
    </div>

    <template #rail>
      <ComptePanneauNavigation />
    </template>
  </NuxtLayout>
</template>

<script setup lang="ts">
/**
 * Les invitations à co-détenir un support média, US5.
 *
 * Un propriétaire de chaîne ou de station invite par courriel ; l'invité
 * accepte ou refuse ici. Une acceptation le fait entrer dans l'équipe du
 * support, visible depuis « Mes supports ».
 */
import {
  useMediaDetention,
  LIBELLES_ROLE_DETENTEUR,
  DESCRIPTIONS_ROLE_DETENTEUR,
  LIBELLES_STATUT_INVITATION,
  type InvitationDetenteurAPI,
} from '~/composables/useMediaDetention'

definePageMeta({ middleware: 'auth', layout: false })

useHead({ title: 'Mes invitations médias | UAfricas' })

const { mesInvitations, repondreInvitation, chargement, erreur } = useMediaDetention()

const invitations = ref<InvitationDetenteurAPI[]>([])

/** Invitation en cours de traitement : évite le double clic sur Accepter/Refuser. */
const enCours = ref<string | null>(null)

/** Affiché après une acceptation, pour orienter vers le support fraîchement rejoint. */
const acceptationReussie = ref(false)

const charger = async () => {
  invitations.value = await mesInvitations()
}

onMounted(charger)

/**
 * Une invitation expirée n'est jamais basculée en base : aucune tâche de fond
 * ne périme les invitations, c'est la lecture qui tranche via le champ
 * `expiree` calculé par le serveur. Le statut peut donc valoir encore
 * `en_attente` alors que l'invitation n'est plus actionnable, `expiree` prime.
 */
const estActionnable = (invitation: InvitationDetenteurAPI): boolean =>
  invitation.statut === 'en_attente' && !invitation.expiree

const libelleStatut = (invitation: InvitationDetenteurAPI): string =>
  invitation.expiree && invitation.statut === 'en_attente'
    ? LIBELLES_STATUT_INVITATION.expiree
    : LIBELLES_STATUT_INVITATION[invitation.statut]

const STYLES_STATUT: Record<string, string> = {
  en_attente: 'bg-af-chocolat/10 text-af-chocolat',
  acceptee: 'bg-af-vert/10 text-af-vert',
  refusee: 'bg-af-live/10 text-af-live',
  expiree: 'bg-af-fond text-af-corps',
}

const styleStatut = (invitation: InvitationDetenteurAPI): string =>
  invitation.expiree && invitation.statut === 'en_attente'
    ? STYLES_STATUT.expiree!
    : STYLES_STATUT[invitation.statut]!

const LIBELLES_TYPE_SUPPORT: Record<InvitationDetenteurAPI['type_support'], string> = {
  chaine_tv: 'Chaîne de télévision',
  station_radio: 'Station de radio',
}

const invitant = (invitation: InvitationDetenteurAPI): string => {
  const nom = [invitation.invite_par_prenom, invitation.invite_par_nom].filter(Boolean).join(' ')
  return nom || 'Un détenteur du support'
}

const repondre = async (invitation: InvitationDetenteurAPI, reponse: 'accepter' | 'refuser') => {
  enCours.value = invitation.id
  const ok = await repondreInvitation(invitation.id, reponse)
  enCours.value = null
  if (!ok) return
  if (reponse === 'accepter') acceptationReussie.value = true
  // La liste est rechargée : le serveur est seul juge du statut résultant.
  await charger()
}

const dateFormatee = (iso: string) =>
  new Intl.DateTimeFormat('fr-FR', { day: 'numeric', month: 'long', year: 'numeric' })
    .format(new Date(iso))

</script>
