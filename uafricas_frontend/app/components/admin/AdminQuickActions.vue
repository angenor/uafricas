<script setup lang="ts">
import type { DashboardStats } from '~/types/admin'

const props = defineProps<{
  stats: DashboardStats | null
}>()

const alertes = computed(() => {
  if (!props.stats) return []
  const items: { label: string; count: number; lien: string; icon: string; couleur: string }[] = []

  if (props.stats.programmes.candidatures_en_attente > 0) {
    items.push({
      label: 'Candidatures en attente',
      count: props.stats.programmes.candidatures_en_attente,
      lien: '/admin/candidatures',
      icon: 'user-graduate',
      couleur: 'text-warning',
    })
  }
  if (props.stats.fiches_pays.contributions_en_attente > 0) {
    items.push({
      label: 'Contributions pays en attente',
      count: props.stats.fiches_pays.contributions_en_attente,
      lien: '/admin/profils-pays',
      icon: 'earth-africa',
      couleur: 'text-info',
    })
  }
  if (props.stats.annonces.en_attente > 0) {
    items.push({
      label: 'Annonces en attente',
      count: props.stats.annonces.en_attente,
      lien: '/admin/annonces',
      icon: 'bullhorn',
      couleur: 'text-error',
    })
  }
  if (props.stats.projets.en_revue > 0) {
    items.push({
      label: 'Projets en revue',
      count: props.stats.projets.en_revue,
      lien: '/admin/projets',
      icon: 'diagram-project',
      couleur: 'text-accent',
    })
  }
  if (props.stats.utilisateurs.en_attente > 0) {
    items.push({
      label: 'Utilisateurs en attente',
      count: props.stats.utilisateurs.en_attente,
      lien: '/admin/utilisateurs',
      icon: 'user-clock',
      couleur: 'text-warning',
    })
  }
  return items
})
</script>

<template>
  <div class="card bg-base-100 border border-base-200 shadow-sm">
    <div class="card-body p-5">
      <h2 class="card-title text-base font-display mb-4">
        <font-awesome-icon icon="bolt" class="text-custom-chocolat/60" />
        Alertes & Actions rapides
      </h2>

      <div v-if="alertes.length === 0" class="flex flex-col items-center justify-center py-8 text-base-content/30">
        <font-awesome-icon icon="circle-check" class="w-10 h-10 mb-3" />
        <p class="text-sm">Tout est a jour</p>
      </div>

      <ul v-else class="space-y-2">
        <li v-for="alerte in alertes" :key="alerte.lien">
          <NuxtLink
            :to="alerte.lien"
            class="flex items-center gap-3 p-3 rounded-lg border border-base-200 hover:bg-base-200/50 transition-colors group"
          >
            <div class="flex-shrink-0">
              <font-awesome-icon :icon="alerte.icon" class="w-4 h-4" :class="alerte.couleur" />
            </div>
            <div class="flex-1">
              <span class="text-sm font-medium">{{ alerte.label }}</span>
            </div>
            <span class="badge badge-sm badge-warning">{{ alerte.count }}</span>
            <font-awesome-icon icon="chevron-right" class="w-3 h-3 text-base-content/30 group-hover:text-base-content/60 transition-colors" />
          </NuxtLink>
        </li>
      </ul>
    </div>
  </div>
</template>
