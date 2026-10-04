<script setup lang="ts">
/**
 * Mes duels (feature 013). Chaque duel est résolu par le serveur avant d'être
 * listé : un forfait ou une expiration apparaît sans que personne l'ait
 * déclenché. Les signaux SSE `duel_*` font relire la liste.
 */
import { messageErreurJeu } from '~/composables/useJeu'
import type { DuelAPI, MesDuelsAPI } from '~/composables/useDuels'

definePageMeta({ layout: false, middleware: 'auth' })

useHead({ title: 'Mes duels | Activités | AfricanS' })

const { listerDuels, accepter, refuser, jouer, signal } = useDuels()
const userStore = useUserStore()

const duels = ref<MesDuelsAPI | null>(null)
const chargement = ref(true)
const occupe = ref(false)
const erreur = ref('')
const proposition = ref(false)
const message = ref('')

const charger = async () => {
  try {
    duels.value = await listerDuels()
  }
  catch (e) {
    erreur.value = messageErreurJeu(e, 'Impossible de charger vos duels.')
  }
  finally {
    chargement.value = false
  }
}

const agir = async (action: () => Promise<unknown>) => {
  erreur.value = ''
  occupe.value = true
  try {
    await action()
    await charger()
  }
  catch (e) {
    erreur.value = messageErreurJeu(e)
  }
  finally {
    occupe.value = false
  }
}

const lancer = async (duel: DuelAPI) => {
  occupe.value = true
  try {
    const res = await jouer(duel.id)
    if (res) await navigateTo(`/activites/partie/${res.partie_id}`)
  }
  catch (e) {
    erreur.value = messageErreurJeu(e)
    await charger()
  }
  finally {
    occupe.value = false
  }
}

const apresProposition = (duel: DuelAPI & { sera_compte: boolean }) => {
  message.value = duel.sera_compte
    ? 'Défi envoyé.'
    : 'Défi envoyé. Vous avez atteint le nombre de duels comptés du jour : celui-ci sera amical, sans gain.'
  charger()
}

watch(signal, charger)
onMounted(charger)
</script>

<template>
  <NuxtLayout name="africans">
    <template #fil-ariane>
      <AfricansFilAriane :segments="[{ libelle: 'Activités', vers: '/activites' }, { libelle: 'Mes duels' }]" />
    </template>

    <div class="flex flex-col gap-6 pb-24">
      <header class="flex flex-wrap items-end justify-between gap-4">
        <div>
          <h1 class="text-[24px]/[1.3] font-bold text-af-encre">Mes duels</h1>
          <p v-if="duels" class="mt-1 text-[14px]/[1.5] text-af-corps">
            {{ duels.quotas.comptes_aujourdhui }} duel{{ duels.quotas.comptes_aujourdhui > 1 ? 's' : '' }} compté{{ duels.quotas.comptes_aujourdhui > 1 ? 's' : '' }}
            aujourd'hui sur {{ duels.quotas.plafond }} ; au-delà, les duels sont amicaux.
          </p>
        </div>
        <AfricansBouton icone="fa-solid fa-hand-fist" @click="proposition = true">Défier un ami</AfricansBouton>
      </header>

      <p v-if="message" class="rounded-[10px] border border-af-vert/40 bg-af-vert/5 px-4 py-3 text-[14px]/[1.5] text-af-encre">
        {{ message }}
      </p>
      <p v-if="erreur" class="rounded-[10px] border border-af-live/30 bg-af-live/5 px-4 py-3 text-[14px]/[1.4] text-af-live" role="alert">
        {{ erreur }}
      </p>

      <div v-if="chargement" class="h-64 animate-pulse rounded-[10px] bg-af-bordure" />
      <JeuListeDuels
        v-else-if="duels"
        :duels="duels"
        :moi-id="userStore.user?.id ?? null"
        :occupe="occupe"
        @accepter="d => agir(() => accepter(d.id))"
        @refuser="d => agir(() => refuser(d.id))"
        @jouer="lancer"
      />
    </div>

    <JeuProposerDuelModal v-model="proposition" @propose="apresProposition" />
  </NuxtLayout>
</template>
