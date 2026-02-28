<script setup lang="ts">
definePageMeta({ layout: 'default' })

const userStore = useUserStore()
const { tableauDeBord, basculerTrouvable } = useRetrouvAmis()

const estConnecte = computed(() => userStore.isAuthenticated)

const dashboard = ref<{ avis_actifs: number; correspondances_en_attente: number; notifications_non_lues: number } | null>(null)
const estTrouvable = ref(false)
const chargementTrouvable = ref(false)

const chargerTableauDeBord = async () => {
  if (!estConnecte.value) return
  try {
    const res = await tableauDeBord()
    dashboard.value = res
    estTrouvable.value = res.est_trouvable ?? false
  } catch {
    // silencieux sur page publique
  }
}

const onCreerAvis = () => {
  navigateTo('/retrouve-amis/nouveau')
}

const onActiverTrouvable = async () => {
  chargementTrouvable.value = true
  try {
    const res = await basculerTrouvable()
    estTrouvable.value = res.est_trouvable
  } finally {
    chargementTrouvable.value = false
  }
}

const etapes = [
  {
    icone: 'fa-pen-to-square',
    titre: 'Deposez un avis',
    description: 'Decrivez la personne que vous recherchez : nom, lieu de derniere rencontre, epoque, details physiques ou anecdotes.'
  },
  {
    icone: 'fa-magnifying-glass',
    titre: 'Le systeme compare',
    description: 'Notre algorithme croise votre avis avec les profils et autres avis de recherche pour identifier des correspondances potentielles.'
  },
  {
    icone: 'fa-handshake',
    titre: 'Acceptez le contact',
    description: 'Quand une correspondance est trouvee, les deux parties doivent accepter avant que les coordonnees ne soient partagees.'
  }
]

onMounted(() => {
  chargerTableauDeBord()
})
</script>

<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section -->
    <div
      class="relative h-96 bg-cover bg-center"
      style="background-image: url('https://images.unsplash.com/photo-1529156069898-49953e39b3ac?ixlib=rb-4.0.3&auto=format&fit=crop&w=1900&q=80')"
    >
      <div class="absolute inset-0 bg-gradient-to-r from-custom-chocolat/90 to-black/70" />
      <div class="absolute inset-0 flex flex-col items-center justify-center mt-14">
        <h1 class="text-white text-4xl md:text-5xl font-bold mb-4 animate-title">
          Retrouve Amis
        </h1>
        <div class="h-1 w-24 bg-custom-green rounded animate-line" />
        <p class="text-white text-xl md:text-2xl mt-4 animate-subtitle">
          Retrouvez vos proches
        </p>
        <p class="text-white/80 text-sm md:text-base mt-3 max-w-3xl text-center px-4 animate-subtitle">
          Retrouvez vos amis, proches et connaissances perdus de vue grace a la communaute panafricaine.
        </p>
        <div class="flex flex-wrap justify-center gap-4 mt-8">
          <button
            v-if="estConnecte"
            class="px-6 py-3 bg-white text-custom-chocolat font-semibold rounded-lg hover:bg-amber-50 transition-colors cursor-pointer"
            @click="onCreerAvis"
          >
            <font-awesome-icon :icon="['fas', 'plus']" class="mr-2" />
            Creer un avis de recherche
          </button>
          <button
            v-if="estConnecte"
            class="px-6 py-3 border-2 border-white rounded-lg font-semibold transition-colors cursor-pointer"
            :class="estTrouvable ? 'bg-white/20 text-white' : 'bg-transparent text-white hover:bg-white/10'"
            :disabled="chargementTrouvable"
            @click="onActiverTrouvable"
          >
            <font-awesome-icon :icon="['fas', estTrouvable ? 'eye' : 'eye-slash']" class="mr-2" />
            {{ estTrouvable ? 'Vous etes trouvable' : 'Devenir trouvable' }}
          </button>
          <NuxtLink
            v-if="!estConnecte"
            to="/login"
            class="px-6 py-3 bg-white text-custom-chocolat font-semibold rounded-lg hover:bg-amber-50 transition-colors"
          >
            Se connecter pour commencer
          </NuxtLink>
        </div>
      </div>
    </div>

    <!-- Comment ca marche -->
    <section class="py-16 px-4">
      <div class="max-w-5xl mx-auto">
        <h2 class="text-3xl font-bold text-center text-gray-800 mb-12 font-[Oswald]">
          Comment ca marche ?
        </h2>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
          <div
            v-for="(etape, index) in etapes"
            :key="index"
            class="bg-white rounded-xl shadow-sm border border-gray-200 p-8 text-center hover:shadow-md transition-shadow"
          >
            <div class="w-16 h-16 mx-auto mb-5 bg-amber-100 text-amber-700 rounded-full flex items-center justify-center">
              <font-awesome-icon :icon="['fas', etape.icone]" class="text-2xl" />
            </div>
            <span class="inline-block w-8 h-8 bg-amber-700 text-white rounded-full text-sm font-bold leading-8 mb-3">
              {{ index + 1 }}
            </span>
            <h3 class="text-lg font-semibold text-gray-800 mb-2">{{ etape.titre }}</h3>
            <p class="text-gray-600 text-sm leading-relaxed">{{ etape.description }}</p>
          </div>
        </div>
      </div>
    </section>

    <!-- Tableau de bord (connecte) -->
    <section v-if="estConnecte && dashboard" class="py-12 px-4 bg-white">
      <div class="max-w-5xl mx-auto">
        <h2 class="text-2xl font-bold text-gray-800 mb-8 font-[Oswald]">
          Votre tableau de bord
        </h2>
        <div class="grid grid-cols-1 sm:grid-cols-3 gap-6">
          <NuxtLink
            to="/retrouve-amis/mes-recherches"
            class="bg-amber-50 border border-amber-200 rounded-xl p-6 text-center hover:shadow-md hover:border-amber-300 transition-all"
          >
            <p class="text-3xl font-bold text-amber-800">{{ dashboard.avis_actifs }}</p>
            <p class="text-sm text-amber-700 mt-1">Avis actifs</p>
            <p class="text-xs text-amber-500 mt-2">
              <font-awesome-icon :icon="['fas', 'arrow-right']" class="mr-1" />
              Voir mes recherches
            </p>
          </NuxtLink>
          <NuxtLink
            to="/retrouve-amis/correspondances"
            class="bg-green-50 border border-green-200 rounded-xl p-6 text-center hover:shadow-md hover:border-green-300 transition-all"
          >
            <p class="text-3xl font-bold text-green-800">{{ dashboard.correspondances_en_attente }}</p>
            <p class="text-sm text-green-700 mt-1">Correspondances en attente</p>
            <p class="text-xs text-green-500 mt-2">
              <font-awesome-icon :icon="['fas', 'arrow-right']" class="mr-1" />
              Voir les correspondances
            </p>
          </NuxtLink>
          <NuxtLink
            to="/retrouve-amis/correspondances"
            class="bg-blue-50 border border-blue-200 rounded-xl p-6 text-center hover:shadow-md hover:border-blue-300 transition-all"
          >
            <p class="text-3xl font-bold text-blue-800">{{ dashboard.notifications_non_lues }}</p>
            <p class="text-sm text-blue-700 mt-1">Notifications non lues</p>
            <p class="text-xs text-blue-500 mt-2">
              <font-awesome-icon :icon="['fas', 'arrow-right']" class="mr-1" />
              Consulter
            </p>
          </NuxtLink>
        </div>
      </div>
    </section>

    <!-- CTA inscription -->
    <section v-if="!estConnecte" class="py-16 px-4 bg-amber-50">
      <div class="max-w-3xl mx-auto text-center">
        <h2 class="text-2xl font-bold text-gray-800 mb-4 font-[Oswald]">
          Rejoignez la communaute
        </h2>
        <p class="text-gray-600 mb-8">
          Inscrivez-vous gratuitement pour deposer un avis de recherche et retrouver vos proches perdus de vue.
        </p>
        <NuxtLink
          to="/login?mode=inscription"
          class="inline-block px-8 py-3 bg-amber-700 text-white font-semibold rounded-lg hover:bg-amber-800 transition-colors"
        >
          Creer un compte gratuitement
        </NuxtLink>
      </div>
    </section>
  </div>
</template>
