<template>
  <div>
    <!-- En-tête du Dashboard -->
    <div class="mb-8">
      <h1 class="text-2xl font-display font-bold text-base-content">Dashboard</h1>
      <p class="text-sm text-base-content/60 mt-1">Vue d'ensemble de la plateforme UAfricas</p>
    </div>

    <!-- KPI Cards placeholder -->
    <div class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-4 gap-4 mb-8">
      <div v-for="kpi in kpis" :key="kpi.label" class="stat bg-base-100 border border-base-200 rounded-xl shadow-sm">
        <div class="stat-figure text-custom-chocolat/30">
          <font-awesome-icon :icon="['fas', kpi.icon]" class="w-7 h-7" />
        </div>
        <div class="stat-title text-xs">{{ kpi.label }}</div>
        <div class="stat-value text-2xl font-display">{{ kpi.value }}</div>
        <div class="stat-desc text-xs">{{ kpi.desc }}</div>
      </div>
    </div>

    <!-- Placeholder contenu -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- Activité récente -->
      <div class="card bg-base-100 border border-base-200 shadow-sm">
        <div class="card-body">
          <h2 class="card-title text-base font-display">Activité récente</h2>
          <div class="flex flex-col items-center justify-center py-12 text-base-content/30">
            <font-awesome-icon :icon="['fas', 'clock-rotate-left']" class="w-12 h-12 mb-4" />
            <p class="text-sm font-medium">Sera implémenté prochainement</p>
            <p class="text-xs mt-1">Le journal d'activité apparaîtra ici</p>
          </div>
        </div>
      </div>

      <!-- Statistiques -->
      <div class="card bg-base-100 border border-base-200 shadow-sm">
        <div class="card-body">
          <h2 class="card-title text-base font-display">Statistiques</h2>
          <div class="flex flex-col items-center justify-center py-12 text-base-content/30">
            <font-awesome-icon :icon="['fas', 'chart-line']" class="w-12 h-12 mb-4" />
            <p class="text-sm font-medium">Sera implémenté prochainement</p>
            <p class="text-xs mt-1">Les graphiques et tendances apparaîtront ici</p>
          </div>
        </div>
      </div>

      <!-- Modération en attente -->
      <div class="card bg-base-100 border border-base-200 shadow-sm">
        <div class="card-body">
          <h2 class="card-title text-base font-display">Modération en attente</h2>
          <div class="flex flex-col items-center justify-center py-12 text-base-content/30">
            <font-awesome-icon :icon="['fas', 'shield-alt']" class="w-12 h-12 mb-4" />
            <p class="text-sm font-medium">Sera implémenté prochainement</p>
            <p class="text-xs mt-1">Les éléments à modérer apparaîtront ici</p>
          </div>
        </div>
      </div>

      <!-- Raccourcis rapides -->
      <div class="card bg-base-100 border border-base-200 shadow-sm">
        <div class="card-body">
          <h2 class="card-title text-base font-display">Raccourcis rapides</h2>
          <div class="flex flex-col items-center justify-center py-12 text-base-content/30">
            <font-awesome-icon :icon="['fas', 'rocket']" class="w-12 h-12 mb-4" />
            <p class="text-sm font-medium">Sera implémenté prochainement</p>
            <p class="text-xs mt-1">Les actions rapides apparaîtront ici</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'admin',
})

const kpis = [
  { label: 'Utilisateurs', value: '—', icon: 'users', desc: 'Total inscrits' },
  { label: 'Annonces', value: '—', icon: 'bullhorn', desc: 'Publiées' },
  { label: 'Événements', value: '—', icon: 'calendar', desc: 'À venir' },
  { label: 'Innovations', value: '—', icon: 'rocket', desc: 'Soumises' },
]
</script>
