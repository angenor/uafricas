<script setup lang="ts">
const route = useRoute()
const userStore = useUserStore()

const { tableauDeBord } = useRetrouvAmis()

const stats = ref<{ avis_actifs: number; correspondances_en_attente: number; notifications_non_lues: number } | null>(null)

const liens = [
  { to: '/retrouve-amis', label: 'Accueil', icon: 'fa-solid fa-house', exact: true },
  { to: '/retrouve-amis/mon-profil', label: 'Mon profil', icon: 'fa-solid fa-user-shield' },
  { to: '/retrouve-amis/nouveau', label: 'Nouvel avis', icon: 'fa-solid fa-plus' },
  { to: '/retrouve-amis/mes-recherches', label: 'Mes recherches', icon: 'fa-solid fa-magnifying-glass' },
  { to: '/retrouve-amis/correspondances', label: 'Correspondances', icon: 'fa-solid fa-handshake' },
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
  <!-- Navigation de section, servie dans le RAIL du gabarit : la colonne de
       gauche appartient déjà à la navigation des modules. Elle n'a donc plus
       son propre repli mobile ni son propre `sticky` — le gabarit s'en
       charge, et deux `sticky` imbriqués se neutralisent. -->
  <AfricansPanneau titre="Africonnect" icone="fa-solid fa-users">
    <nav class="flex flex-col gap-1">
      <NuxtLink
        v-for="lien in liens"
        :key="lien.to"
        :to="lien.to"
        class="flex items-center gap-2.5 rounded-md px-3 py-2.5 text-[14px]/[1.4] font-bold transition"
        :class="estActif(lien)
          ? 'bg-af-chocolat/10 text-af-chocolat'
          : 'text-af-corps hover:bg-af-fond'"
      >
        <font-awesome-icon :icon="lien.icon" class="w-4 shrink-0 text-center" />
        <span class="min-w-0 flex-1 truncate">{{ lien.label }}</span>
        <span
          v-if="badgePour(lien.to) > 0"
          class="grid h-5 min-w-5 shrink-0 place-items-center rounded-full bg-af-live px-1.5 text-[12px] font-bold text-white"
        >
          {{ badgePour(lien.to) }}
        </span>
      </NuxtLink>
    </nav>

    <dl v-if="stats" class="mt-4 flex flex-col gap-2 border-t border-af-bordure pt-3 text-[12px]/[1.4]">
      <div class="flex items-center justify-between gap-3">
        <dt class="text-af-atone">Avis actifs</dt>
        <dd class="font-bold text-af-corps">{{ stats.avis_actifs }}</dd>
      </div>
      <div class="flex items-center justify-between gap-3">
        <dt class="text-af-atone">Correspondances</dt>
        <dd class="font-bold text-af-corps">{{ stats.correspondances_en_attente }}</dd>
      </div>
      <div class="flex items-center justify-between gap-3">
        <dt class="text-af-atone">Notifications</dt>
        <dd class="font-bold text-af-corps">{{ stats.notifications_non_lues }}</dd>
      </div>
    </dl>
  </AfricansPanneau>
</template>
