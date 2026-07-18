<script setup lang="ts">
import type { ParcoursTrouvable } from '~/composables/useRetrouvAmis'

definePageMeta({ layout: 'default' })

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
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section (compact, titre ↔ description au survol) -->
    <div
      class="group relative bg-cover bg-center"
      style="background-image: url('https://images.unsplash.com/photo-1529156069898-49953e39b3ac?ixlib=rb-4.0.3&auto=format&fit=crop&w=1900&q=80')"
    >
      <div class="absolute inset-0 bg-gradient-to-r from-custom-chocolat/90 to-black/70" />
      <div class="relative max-w-4xl mx-auto px-4 pt-16 pb-6 text-center select-none">
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12">
          <h1 class="absolute inset-0 flex items-center justify-center text-white text-2xl md:text-4xl font-bold transition-opacity duration-300 group-hover:opacity-0">
            Mon profil
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Gerez votre visibilite et renseignez votre parcours pour etre retrouve par vos proches.
          </p>
        </div>
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
