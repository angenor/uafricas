<template>
  <div
    class="fixed right-6 bottom-24 z-[75] w-[20rem] max-w-[calc(100vw-3rem)] overflow-hidden rounded-[10px] border border-af-bordure bg-af-surface shadow-2xl"
    role="alertdialog"
    aria-labelledby="titre-invitation-duel"
  >
    <div class="flex items-center gap-2 bg-af-degrade px-4 py-2.5 text-white">
      <font-awesome-icon icon="fa-solid fa-bolt" />
      <span id="titre-invitation-duel" class="text-[14px] font-bold">Duel en direct</span>
      <span class="ml-auto text-[13px] tabular-nums opacity-90">{{ reste }}</span>
    </div>
    <div class="flex flex-col items-center gap-3 px-4 py-4 text-center">
      <AfricansAvatar :nom="nom" :src="urlMedia(invitation.proposant.photoUrl) ?? undefined" :taille="56" />
      <p class="text-[15px]/[1.5] text-af-encre">
        <strong>{{ nom }}</strong> vous défie, maintenant, sur la même épreuve.
      </p>
      <p v-if="erreur" class="text-[13px]/[1.4] text-af-live" role="alert">{{ erreur }}</p>
      <div class="flex gap-3">
        <AfricansBouton variante="secondaire" :desactive="occupe" @click="refuserInvitation">Refuser</AfricansBouton>
        <AfricansBouton icone="fa-solid fa-hand-fist" :desactive="occupe" @click="accepterInvitation">Accepter</AfricansBouton>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { dureeRestante, messageErreurJeu } from '~/composables/useJeu'
import type { InvitationDuel } from '~/composables/useDuels'

/**
 * Invitation à un duel DIRECT, surgie par le flux temps réel où que le membre
 * se trouve sur le site. Pas de sonnerie : une invitation à jouer n'est pas un
 * appel. Le délai de réponse est court : au-delà, le duel expire.
 */
const props = defineProps<{ invitation: InvitationDuel }>()

const { accepter, refuser, invitation: etatInvitation } = useDuels()
const occupe = ref(false)
const erreur = ref('')

const nom = computed(() => `${props.invitation.proposant.prenom} ${props.invitation.proposant.nom}`.trim())

// Compte à rebours de l'invitation ; à zéro, elle disparaît d'elle-même.
const maintenant = ref(Date.now())
let minuteur: ReturnType<typeof setInterval> | null = null
onMounted(() => { minuteur = setInterval(() => { maintenant.value = Date.now() }, 1000) })
onBeforeUnmount(() => { if (minuteur) clearInterval(minuteur) })
const reste = computed(() => dureeRestante(props.invitation.expire_a, maintenant.value))
watch(maintenant, () => {
  if (new Date(props.invitation.expire_a).getTime() <= maintenant.value) etatInvitation.value = null
})

const accepterInvitation = async () => {
  occupe.value = true
  erreur.value = ''
  try {
    await accepter(props.invitation.duel_id)
    const id = props.invitation.duel_id
    etatInvitation.value = null
    await navigateTo(`/activites/duels/${id}`)
  }
  catch (e) {
    erreur.value = messageErreurJeu(e, 'Ce duel ne peut plus être accepté.')
  }
  finally {
    occupe.value = false
  }
}

const refuserInvitation = async () => {
  occupe.value = true
  try {
    await refuser(props.invitation.duel_id)
  }
  catch {
    // Déjà expiré ou annulé : rien à refuser.
  }
  finally {
    etatInvitation.value = null
    occupe.value = false
  }
}
</script>
