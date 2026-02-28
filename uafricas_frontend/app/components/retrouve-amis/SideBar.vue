<script setup lang="ts">
const route = useRoute()
const userStore = useUserStore()

const { tableauDeBord } = useRetrouvAmis()

const stats = ref<{ avis_actifs: number; correspondances_en_attente: number; notifications_non_lues: number } | null>(null)

const liens = [
  { to: '/retrouve-amis', label: 'Accueil', icon: 'home', exact: true },
  { to: '/retrouve-amis/nouveau', label: 'Nouvel avis', icon: 'plus' },
  { to: '/retrouve-amis/mes-recherches', label: 'Mes recherches', icon: 'magnifying-glass' },
  { to: '/retrouve-amis/correspondances', label: 'Correspondances', icon: 'handshake' },
]

const estActif = (lien: typeof liens[0]) => {
  if (lien.exact) return route.path === lien.to
  return route.path.startsWith(lien.to)
}

const badgePour = (to: string): number => {
  if (!stats.value) return 0
  if (to === '/retrouve-amis/correspondances') return stats.value.correspondances_en_attente
  return 0
}

onMounted(async () => {
  if (!userStore.isAuthenticated) return
  try {
    const res = await tableauDeBord()
    stats.value = res
  } catch {
    // silencieux
  }
})
</script>

<template>
  <div class="lg:contents">
    <!-- Mobile : barre horizontale -->
    <nav class="lg:hidden flex items-center gap-1 bg-white rounded-xl shadow-sm border border-gray-200 p-1.5 mb-6 overflow-x-auto">
      <NuxtLink
        v-for="lien in liens"
        :key="lien.to"
        :to="lien.to"
        class="flex items-center gap-1.5 px-3 py-2 text-sm font-medium rounded-lg whitespace-nowrap transition-colors"
        :class="estActif(lien)
          ? 'bg-amber-700 text-white'
          : 'text-gray-600 hover:bg-gray-100'"
      >
        <font-awesome-icon :icon="['fas', lien.icon]" class="text-xs" />
        {{ lien.label }}
        <span
          v-if="badgePour(lien.to) > 0"
          class="ml-1 min-w-5 h-5 px-1.5 flex items-center justify-center text-xs font-bold rounded-full"
          :class="estActif(lien) ? 'bg-white text-amber-700' : 'bg-red-500 text-white'"
        >
          {{ badgePour(lien.to) }}
        </span>
      </NuxtLink>
    </nav>

    <!-- Desktop : sidebar verticale -->
    <aside class="hidden lg:block w-56 shrink-0">
      <div class="sticky top-24 bg-white rounded-xl shadow-sm border border-gray-200 p-3">
        <h3 class="text-xs font-semibold text-gray-400 uppercase tracking-wider px-3 mb-2">
          Retrouve Amis
        </h3>
        <nav class="space-y-0.5">
          <NuxtLink
            v-for="lien in liens"
            :key="lien.to"
            :to="lien.to"
            class="flex items-center gap-2.5 px-3 py-2.5 text-sm font-medium rounded-lg transition-colors"
            :class="estActif(lien)
              ? 'bg-amber-50 text-amber-800 border-l-3 border-amber-700'
              : 'text-gray-600 hover:bg-gray-50 hover:text-gray-800'"
          >
            <font-awesome-icon :icon="['fas', lien.icon]" class="w-4 text-center" />
            <span class="flex-1">{{ lien.label }}</span>
            <span
              v-if="badgePour(lien.to) > 0"
              class="min-w-5 h-5 px-1.5 flex items-center justify-center text-xs font-bold rounded-full bg-red-500 text-white"
            >
              {{ badgePour(lien.to) }}
            </span>
          </NuxtLink>
        </nav>

        <!-- Mini stats -->
        <div v-if="stats" class="mt-4 pt-3 border-t border-gray-100 px-3 space-y-2">
          <div class="flex items-center justify-between text-xs">
            <span class="text-gray-500">Avis actifs</span>
            <span class="font-semibold text-gray-700">{{ stats.avis_actifs }}</span>
          </div>
          <div class="flex items-center justify-between text-xs">
            <span class="text-gray-500">Correspondances</span>
            <span class="font-semibold text-gray-700">{{ stats.correspondances_en_attente }}</span>
          </div>
          <div class="flex items-center justify-between text-xs">
            <span class="text-gray-500">Notifications</span>
            <span class="font-semibold text-gray-700">{{ stats.notifications_non_lues }}</span>
          </div>
        </div>
      </div>
    </aside>
  </div>
</template>
