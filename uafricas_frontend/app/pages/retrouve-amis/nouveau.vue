<script setup lang="ts">
definePageMeta({ layout: 'default' })

const userStore = useUserStore()
const { creerAvis } = useRetrouvAmis()

const chargement = ref(false)
const succes = ref(false)
const slugCree = ref('')
const correspondancesTrouvees = ref(0)
const erreur = ref('')

const onSoumettre = async (formData: FormData) => {
  chargement.value = true
  erreur.value = ''
  try {
    const res = await creerAvis(formData)
    if (res) {
      correspondancesTrouvees.value = res.correspondances_trouvees ?? 0
      slugCree.value = res.slug ?? ''
      succes.value = true
    }
  } catch (e: any) {
    erreur.value = e?.data?.error || e?.message || 'Une erreur est survenue lors de la soumission.'
  } finally {
    chargement.value = false
  }
}

const onAnnuler = () => {
  navigateTo('/retrouve-amis')
}

onMounted(() => {
  if (!userStore.isAuthenticated) {
    navigateTo('/login')
  }
})
</script>

<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section -->
    <div
      class="relative h-80 bg-cover bg-center"
      style="background-image: url('https://images.unsplash.com/photo-1529156069898-49953e39b3ac?ixlib=rb-4.0.3&auto=format&fit=crop&w=1900&q=80')"
    >
      <div class="absolute inset-0 bg-linear-to-r from-custom-chocolat/90 to-black/70" />
      <div class="absolute inset-0 flex flex-col items-center justify-center mt-14">
        <h1 class="text-white text-4xl md:text-5xl font-bold mb-4 animate-title">
          Nouvel avis
        </h1>
        <div class="h-1 w-24 bg-custom-green rounded animate-line" />
        <p class="text-white text-xl md:text-2xl mt-4 animate-subtitle">
          Avis de recherche
        </p>
        <p class="text-white/80 text-sm md:text-base mt-3 max-w-3xl text-center px-4 animate-subtitle">
          Decrivez la personne que vous recherchez pour lancer la mise en relation.
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
            Avis de recherche publie !
          </h2>
          <p class="text-gray-600 mb-2">
            Votre avis est maintenant visible par tous les visiteurs.
          </p>
          <p v-if="correspondancesTrouvees > 0" class="text-green-700 font-medium mb-4">
            {{ correspondancesTrouvees }} correspondance(s) potentielle(s) trouvee(s) !
          </p>
          <p v-else class="text-gray-500 text-sm mb-6">
            Nous vous notifierons des qu'une correspondance sera trouvee.
          </p>
          <div class="flex flex-wrap justify-center gap-3">
            <NuxtLink
              v-if="slugCree"
              :to="`/retrouve-amis/public/${slugCree}`"
              class="px-6 py-3 bg-amber-700 text-white font-semibold rounded-lg hover:bg-amber-800 transition-colors"
            >
              <font-awesome-icon :icon="['fas', 'eye']" class="mr-2" />
              Voir l'avis public
            </NuxtLink>
            <NuxtLink
              to="/retrouve-amis/mes-recherches"
              class="px-6 py-3 border border-gray-300 text-gray-700 font-semibold rounded-lg hover:bg-gray-100 transition-colors"
            >
              Mes recherches
            </NuxtLink>
          </div>
        </div>

        <!-- Formulaire -->
        <div v-else>
          <!-- Erreur -->
          <div v-if="erreur" class="mb-6 p-4 bg-red-50 border border-red-200 rounded-lg text-red-700 text-sm">
            <font-awesome-icon :icon="['fas', 'circle-exclamation']" class="mr-2" />
            {{ erreur }}
          </div>

          <RetrouveAmisAvisRechercheForm
            mode="creation"
            :chargement-soumission="chargement"
            @submit="onSoumettre"
            @annuler="onAnnuler"
          />
        </div>
      </div>
    </div>
  </div>
</template>
