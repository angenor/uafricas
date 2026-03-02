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
    bg: 'bg-custom-chocolat/5',
    texteIcone: 'text-custom-chocolat',
  },
  {
    label: 'Avis clotures',
    cle: 'avis_clotures' as const,
    icone: 'circle-check',
    bg: 'bg-gray-50',
    texteIcone: 'text-gray-500',
  },
  {
    label: 'Correspondances en attente',
    cle: 'correspondances_en_attente' as const,
    icone: 'clock',
    bg: 'bg-amber-50',
    texteIcone: 'text-amber-600',
  },
  {
    label: 'Correspondances mutuelles',
    cle: 'correspondances_mutuelles' as const,
    icone: 'handshake',
    bg: 'bg-custom-green/5',
    texteIcone: 'text-custom-green',
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
        class="rounded-xl border border-gray-100 p-4 shadow-sm"
        :class="carte.bg"
      >
        <div class="mb-2 flex items-center justify-between">
          <font-awesome-icon
            :icon="carte.icone"
            class="h-5 w-5"
            :class="carte.texteIcone"
          />
        </div>
        <p class="text-2xl font-bold text-gray-900">
          {{ donnees[carte.cle] }}
        </p>
        <p class="mt-0.5 text-xs text-gray-500">
          {{ carte.label }}
        </p>
      </div>
    </div>

    <!-- Deuxieme rangee : notifications, statut trouvable, parcours -->
    <div class="grid grid-cols-1 gap-4 sm:grid-cols-3">
      <!-- Notifications non lues -->
      <div class="flex items-center gap-3 rounded-xl border border-gray-100 bg-white p-4 shadow-sm">
        <div class="relative flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-red-50">
          <font-awesome-icon icon="bell" class="h-5 w-5 text-red-500" />
          <span
            v-if="donnees.notifications_non_lues > 0"
            class="absolute -right-1 -top-1 flex h-5 min-w-5 items-center justify-center rounded-full bg-red-500 px-1 text-xs font-bold text-white"
          >
            {{ donnees.notifications_non_lues }}
          </span>
        </div>
        <div>
          <p class="text-sm font-semibold text-gray-900">{{ donnees.notifications_non_lues }}</p>
          <p class="text-xs text-gray-500">Notifications non lues</p>
        </div>
      </div>

      <!-- Statut trouvable -->
      <div class="flex items-center gap-3 rounded-xl border border-gray-100 bg-white p-4 shadow-sm">
        <div
          class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full"
          :class="donnees.est_trouvable ? 'bg-custom-green/10' : 'bg-gray-100'"
        >
          <span
            class="h-3 w-3 rounded-full"
            :class="donnees.est_trouvable ? 'bg-custom-green' : 'bg-gray-400'"
          />
        </div>
        <div>
          <p class="text-sm font-semibold text-gray-900">
            {{ donnees.est_trouvable ? 'Trouvable' : 'Masque' }}
          </p>
          <p class="text-xs text-gray-500">Statut du profil</p>
        </div>
      </div>

      <!-- Parcours -->
      <div class="flex items-center gap-3 rounded-xl border border-gray-100 bg-white p-4 shadow-sm">
        <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-indigo-50">
          <font-awesome-icon icon="route" class="h-5 w-5 text-indigo-600" />
        </div>
        <div>
          <p class="text-sm font-semibold text-gray-900">{{ donnees.nb_parcours }}</p>
          <p class="text-xs text-gray-500">Parcours renseignes</p>
        </div>
      </div>
    </div>

    <!-- Liens rapides -->
    <div class="flex flex-wrap gap-3">
      <button
        v-for="lien in liensRapides"
        :key="lien.destination"
        type="button"
        class="inline-flex items-center gap-2 rounded-lg border border-gray-200 bg-white px-4 py-2.5 text-sm font-medium text-gray-700 shadow-sm transition-colors hover:border-custom-chocolat hover:text-custom-chocolat"
        @click="$emit('naviguer', lien.destination)"
      >
        <font-awesome-icon :icon="lien.icone" class="h-3.5 w-3.5" />
        {{ lien.label }}
      </button>
    </div>
  </div>
</template>
