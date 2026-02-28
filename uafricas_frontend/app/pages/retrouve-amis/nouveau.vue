<script setup lang="ts">
definePageMeta({ layout: 'default' })

const userStore = useUserStore()
const route = useRoute()
const { creerAvis, detailAvis, modifierAvis } = useRetrouvAmis()

const chargement = ref(false)
const succes = ref(false)
const correspondancesTrouvees = ref(0)
const erreur = ref('')

const idModification = computed(() => route.query.id as string | undefined)
const estModification = computed(() => !!idModification.value)

const formulaire = ref<Record<string, any>>({})
const chargementInitial = ref(false)

const chargerAvis = async () => {
  if (!idModification.value) return
  chargementInitial.value = true
  try {
    const avis = await detailAvis(idModification.value)
    formulaire.value = avis
  } catch {
    erreur.value = 'Impossible de charger l\'avis de recherche.'
  } finally {
    chargementInitial.value = false
  }
}

const onSoumettre = async (data: Record<string, any>) => {
  chargement.value = true
  erreur.value = ''
  try {
    if (estModification.value && idModification.value) {
      await modifierAvis(idModification.value, data)
      navigateTo('/retrouve-amis/mes-recherches')
    } else {
      const res = await creerAvis(data)
      correspondancesTrouvees.value = res.correspondances_trouvees ?? 0
      succes.value = true
    }
  } catch (e: any) {
    erreur.value = e?.data?.message || 'Une erreur est survenue lors de la soumission.'
  } finally {
    chargement.value = false
  }
}

const onAnnuler = () => {
  navigateTo('/retrouve-amis')
}

const allerMesRecherches = () => {
  navigateTo('/retrouve-amis/mes-recherches')
}

onMounted(() => {
  if (!userStore.isAuthenticated) {
    navigateTo('/login')
    return
  }
  chargerAvis()
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
          {{ estModification ? 'Modifier l\'avis' : 'Nouvel avis' }}
        </h1>
        <div class="h-1 w-24 bg-custom-green rounded animate-line" />
        <p class="text-white text-xl md:text-2xl mt-4 animate-subtitle">
          Avis de recherche
        </p>
        <p class="text-white/80 text-sm md:text-base mt-3 max-w-3xl text-center px-4 animate-subtitle">
          {{ estModification ? 'Mettez a jour les informations de votre avis.' : 'Decrivez la personne que vous recherchez pour lancer la mise en relation.' }}
        </p>
      </div>
    </div>

    <div class="max-w-6xl mx-auto lg:flex lg:gap-8 px-4 py-8">
      <RetrouveAmisSideBar />
      <div class="flex-1 max-w-3xl">

          <!-- Message de succes -->
          <div v-if="succes" class="bg-white rounded-xl shadow-sm border border-green-200 p-8 text-center">
            <div class="w-16 h-16 mx-auto mb-4 bg-green-100 text-green-600 rounded-full flex items-center justify-center">
              <font-awesome-icon :icon="['fas', 'check']" class="text-3xl" />
            </div>
            <h2 class="text-2xl font-bold text-gray-800 mb-2 font-[Oswald]">
              Avis de recherche cree avec succes !
            </h2>
            <p v-if="correspondancesTrouvees > 0" class="text-green-700 font-medium mb-4">
              <font-awesome-icon :icon="['fas', 'sparkles']" class="mr-1" />
              {{ correspondancesTrouvees }} correspondance(s) potentielle(s) trouvee(s) !
            </p>
            <p v-else class="text-gray-600 mb-4">
              Nous vous notifierons des qu'une correspondance sera trouvee.
            </p>
            <button
              class="px-6 py-3 bg-amber-700 text-white font-semibold rounded-lg hover:bg-amber-800 transition-colors cursor-pointer"
              @click="allerMesRecherches"
            >
              Voir mes recherches
            </button>
          </div>

          <!-- Formulaire -->
          <div v-else>
            <!-- Erreur -->
            <div v-if="erreur" class="mb-6 p-4 bg-red-50 border border-red-200 rounded-lg text-red-700 text-sm">
              <font-awesome-icon :icon="['fas', 'circle-exclamation']" class="mr-2" />
              {{ erreur }}
            </div>

            <!-- Chargement initial -->
            <div v-if="chargementInitial" class="flex items-center justify-center py-20">
              <font-awesome-icon :icon="['fas', 'spinner']" class="text-3xl text-amber-600 animate-spin" />
            </div>

            <!-- Composant formulaire -->
            <div v-else class="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
              <RetrouveAmisAvisRechercheForm
                :avis-existant="estModification ? formulaire as any : undefined"
                :mode="estModification ? 'modification' : 'creation'"
                @submit="onSoumettre"
                @annuler="onAnnuler"
              />
            </div>
          </div>
      </div>
    </div>
  </div>
</template>
