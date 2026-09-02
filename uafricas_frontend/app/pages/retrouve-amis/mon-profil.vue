<script setup lang="ts">
import type { ParcoursTrouvable } from '~/composables/useRetrouvAmis'

definePageMeta({ layout: false })

const userStore = useUserStore()
const { redirigerVersConnexion } = useAuth()
const { basculerTrouvable, listerParcours, ajouterParcours, modifierParcours, supprimerParcours, tableauDeBord, chargement, erreur } = useRetrouvAmis()

const estTrouvable = ref(false)
const parcours = ref<ParcoursTrouvable[]>([])
const messageSucces = ref('')
const chargementPage = ref(true)

// Charger les donnees
const charger = async () => {
  chargementPage.value = true
  try {
    const [dashboard, listeParcours] = await Promise.all([
      tableauDeBord(),
      listerParcours(),
    ])
    if (dashboard) {
      estTrouvable.value = dashboard.est_trouvable
    }
    if (listeParcours) {
      parcours.value = listeParcours
    }
  }
  catch {
    // erreur geree par le composable
  }
  finally {
    chargementPage.value = false
  }
}

// Basculer le statut trouvable
const onBasculerTrouvable = async () => {
  const res = await basculerTrouvable(!estTrouvable.value)
  if (res) {
    estTrouvable.value = res.est_trouvable
    if (res.correspondances_trouvees > 0) {
      messageSucces.value = `Profil active ! ${res.correspondances_trouvees} correspondance(s) potentielle(s) trouvee(s).`
    }
    else {
      messageSucces.value = res.est_trouvable
        ? 'Votre profil est maintenant visible.'
        : 'Votre profil est maintenant masque.'
    }
    setTimeout(() => { messageSucces.value = '' }, 4000)
  }
}

// Ajouter un parcours
const onAjouterParcours = async (data: any) => {
  const res = await ajouterParcours(data)
  if (res) {
    messageSucces.value = 'Parcours ajoute avec succes.'
    setTimeout(() => { messageSucces.value = '' }, 3000)
    const liste = await listerParcours()
    if (liste) parcours.value = liste
  }
}

// Modifier un parcours
const onModifierParcours = async (id: string, data: any) => {
  const res = await modifierParcours(id, data)
  if (res) {
    messageSucces.value = 'Parcours modifie avec succes.'
    setTimeout(() => { messageSucces.value = '' }, 3000)
    const liste = await listerParcours()
    if (liste) parcours.value = liste
  }
}

// Supprimer un parcours
const onSupprimerParcours = async (id: string) => {
  const confirmer = window.confirm('Etes-vous sur de vouloir supprimer ce parcours ?')
  if (!confirmer) return

  const res = await supprimerParcours(id)
  if (res) {
    messageSucces.value = 'Parcours supprime.'
    setTimeout(() => { messageSucces.value = '' }, 3000)
    parcours.value = parcours.value.filter(p => p.id !== id)
  }
}

onMounted(() => {
  if (!userStore.isAuthenticated) {
    redirigerVersConnexion()
    return
  }
  charger()
})
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Mon profil Africonnect"
        sous-titre="Gérez votre visibilité et renseignez votre parcours pour être retrouvé par vos proches."
        image="/images/africans/heros/hero-africonnect.jpg"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[
          { libelle: 'Opafrica', vers: '/retrouve-amis' },
          { libelle: 'Africonnect', vers: '/retrouve-amis' },
          { libelle: 'Mon profil' },
        ]"
      />
    </template>

    <div class="flex flex-col gap-6">
      <p
        v-if="messageSucces"
        class="flex items-center gap-2 rounded-[10px] border border-af-vert/30 bg-af-vert/5 px-4 py-3 text-[14px]/[1.4] text-af-vert"
      >
        <font-awesome-icon icon="fa-solid fa-circle-check" class="shrink-0" />
        {{ messageSucces }}
      </p>

      <p
        v-if="erreur"
        class="flex items-center gap-2 rounded-[10px] border border-af-live/30 bg-af-live/5 px-4 py-3 text-[14px]/[1.4] text-af-live"
      >
        <font-awesome-icon icon="fa-solid fa-triangle-exclamation" class="shrink-0" />
        {{ erreur }}
      </p>

      <div v-if="chargementPage" class="flex flex-col items-center gap-4 py-24">
        <font-awesome-icon icon="fa-solid fa-spinner" class="animate-spin text-3xl text-af-chocolat" />
        <p class="text-[14px]/[1.4] text-af-atone">Chargement du profil…</p>
      </div>

      <RetrouveAmisProfilTrouvableForm
        v-else
        :est-trouvable="estTrouvable"
        :parcours="parcours"
        :chargement="chargement"
        @basculer-trouvable="onBasculerTrouvable"
        @ajouter-parcours="onAjouterParcours"
        @modifier-parcours="onModifierParcours"
        @supprimer-parcours="onSupprimerParcours"
      />
    </div>

    <template #rail>
      <RetrouveAmisSideBar />
    </template>
  </NuxtLayout>
</template>
