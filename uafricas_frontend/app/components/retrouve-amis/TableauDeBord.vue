<script setup lang="ts">
interface DonneesDashboard {
  avis_actifs: number
  avis_clotures: number
  correspondances_en_attente: number
  correspondances_mutuelles: number
  notifications_non_lues: number
  est_trouvable: boolean
  nb_parcours: number
}

defineProps<{
  donnees: DonneesDashboard
}>()

defineEmits<{
  naviguer: [destination: string]
}>()

// ── Configuration des cartes statistiques ────────────────────
const cartesStats = computed(() => [
  {
    label: 'Avis actifs',
    cle: 'avis_actifs' as const,
    icone: 'search',
    bg: 'bg-af-chocolat/5',
    texteIcone: 'text-af-chocolat',
  },
  {
    label: 'Avis clotures',
    cle: 'avis_clotures' as const,
    icone: 'circle-check',
    bg: 'bg-af-fond',
    texteIcone: 'text-af-atone',
  },
  {
    label: 'Correspondances en attente',
    cle: 'correspondances_en_attente' as const,
    icone: 'clock',
    bg: 'bg-af-chocolat/5',
    texteIcone: 'text-af-chocolat',
  },
  {
    label: 'Correspondances mutuelles',
    cle: 'correspondances_mutuelles' as const,
    icone: 'handshake',
    bg: 'bg-af-vert/5',
    texteIcone: 'text-af-vert',
  },
])

const liensRapides = [
  { label: 'Mes recherches', destination: 'recherches', icone: 'search' },
  { label: 'Correspondances', destination: 'correspondances', icone: 'handshake' },
  { label: 'Mon profil', destination: 'profil', icone: 'user' },
]
</script>

<template>
  <div class="space-y-6">
    <!-- Grille principale des statistiques -->
    <div class="grid grid-cols-2 gap-4 md:grid-cols-4">
      <div
        v-for="carte in cartesStats"
        :key="carte.cle"
        class="rounded-lg border border-af-bordure p-4 shadow-sm"
        :class="carte.bg"
      >
        <div class="mb-2 flex items-center justify-between">
          <font-awesome-icon
            :icon="carte.icone"
            class="h-5 w-5"
            :class="carte.texteIcone"
          />
        </div>
        <p class="text-2xl font-bold text-af-encre">
          {{ donnees[carte.cle] }}
        </p>
        <p class="mt-0.5 text-xs text-af-atone">
          {{ carte.label }}
        </p>
      </div>
    </div>

    <!-- Deuxieme rangee : notifications, statut trouvable, parcours -->
    <div class="grid grid-cols-1 gap-4 sm:grid-cols-3">
      <!-- Notifications non lues -->
      <div class="flex items-center gap-3 rounded-lg border border-af-bordure bg-white p-4 shadow-sm">
        <div class="relative flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-af-live/5">
          <font-awesome-icon icon="bell" class="h-5 w-5 text-af-live" />
          <span
            v-if="donnees.notifications_non_lues > 0"
            class="absolute -right-1 -top-1 flex h-5 min-w-5 items-center justify-center rounded-full bg-af-live/50 px-1 text-xs font-bold text-white"
          >
            {{ donnees.notifications_non_lues }}
          </span>
        </div>
        <div>
          <p class="text-sm font-semibold text-af-encre">{{ donnees.notifications_non_lues }}</p>
          <p class="text-xs text-af-atone">Notifications non lues</p>
        </div>
      </div>

      <!-- Statut trouvable -->
      <div class="flex items-center gap-3 rounded-lg border border-af-bordure bg-white p-4 shadow-sm">
        <div
          class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full"
          :class="donnees.est_trouvable ? 'bg-af-vert/10' : 'bg-af-fond'"
        >
          <span
            class="h-3 w-3 rounded-full"
            :class="donnees.est_trouvable ? 'bg-af-vert' : 'bg-af-atone-2'"
          />
        </div>
        <div>
          <p class="text-sm font-semibold text-af-encre">
            {{ donnees.est_trouvable ? 'Trouvable' : 'Masque' }}
          </p>
          <p class="text-xs text-af-atone">Statut du profil</p>
        </div>
      </div>

      <!-- Parcours -->
      <div class="flex items-center gap-3 rounded-lg border border-af-bordure bg-white p-4 shadow-sm">
        <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-af-chocolat/5">
          <font-awesome-icon icon="route" class="h-5 w-5 text-af-chocolat" />
        </div>
        <div>
          <p class="text-sm font-semibold text-af-encre">{{ donnees.nb_parcours }}</p>
          <p class="text-xs text-af-atone">Parcours renseignes</p>
        </div>
      </div>
    </div>

    <!-- Liens rapides -->
    <div class="flex flex-wrap gap-3">
      <button
        v-for="lien in liensRapides"
        :key="lien.destination"
        type="button"
        class="inline-flex items-center gap-2 rounded-lg border border-af-bordure bg-white px-4 py-2.5 text-sm font-medium text-af-corps shadow-sm transition-colors hover:border-af-chocolat hover:text-af-chocolat"
        @click="$emit('naviguer', lien.destination)"
      >
        <font-awesome-icon :icon="lien.icone" class="h-3.5 w-3.5" />
        {{ lien.label }}
      </button>
    </div>
  </div>
</template>
