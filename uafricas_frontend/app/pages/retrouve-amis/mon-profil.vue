<script setup lang="ts">
import type { ParcoursTrouvable } from '~/composables/useRetrouvAmis'

definePageMeta({ layout: 'default' })

const userStore = useUserStore()
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
    navigateTo('/login')
    return
  }
  charger()
})
</script>

<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section -->
    <div
      class="relative h-80 bg-cover bg-center"
      style="background-image: url('https://images.unsplash.com/photo-1529156069898-49953e39b3ac?ixlib=rb-4.0.3&auto=format&fit=crop&w=1900&q=80')"
    >
      <div class="absolute inset-0 bg-gradient-to-r from-custom-chocolat/90 to-black/70" />
      <div class="absolute inset-0 flex flex-col items-center justify-center mt-14">
        <h1 class="text-white text-4xl md:text-5xl font-bold mb-4 animate-title">
          Mon profil
        </h1>
        <div class="h-1 w-24 bg-custom-green rounded animate-line" />
        <p class="text-white text-xl md:text-2xl mt-4 animate-subtitle">
          Visibilite et parcours
        </p>
        <p class="text-white/80 text-sm md:text-base mt-3 max-w-3xl text-center px-4 animate-subtitle">
          Gerez votre visibilite et renseignez votre parcours pour etre retrouve par vos proches.
        </p>
      </div>
    </div>

    <div class="max-w-6xl mx-auto lg:flex lg:gap-8 px-4 py-8">
      <RetrouveAmisSideBar />
      <div class="flex-1 max-w-3xl">
        <!-- Message succes -->
        <div
          v-if="messageSucces"
          class="mb-6 p-4 bg-green-50 border border-green-200 text-green-700 text-sm rounded-lg"
        >
          <font-awesome-icon :icon="['fas', 'check-circle']" class="mr-2" />
          {{ messageSucces }}
        </div>

        <!-- Message erreur -->
        <div
          v-if="erreur"
          class="mb-6 p-4 bg-red-50 border border-red-200 text-red-700 text-sm rounded-lg"
        >
          <font-awesome-icon :icon="['fas', 'exclamation-triangle']" class="mr-2" />
          {{ erreur }}
        </div>

        <!-- Chargement -->
        <div v-if="chargementPage" class="flex flex-col items-center justify-center py-24">
          <div class="w-12 h-12 border-4 border-amber-200 border-t-amber-700 rounded-full animate-spin mb-4" />
          <p class="text-gray-500 text-sm">Chargement du profil...</p>
        </div>

        <!-- Contenu -->
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
    </div>
  </div>
</template>
