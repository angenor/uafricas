<script setup lang="ts">
import type { DashboardActiviteItem } from '~/types/admin'

defineProps<{
  items: DashboardActiviteItem[]
}>()

const actionConfig: Record<string, { icon: string; color: string; label: string }> = {
  CREATE: { icon: 'plus', color: 'text-success', label: 'Creation' },
  UPDATE: { icon: 'pen', color: 'text-info', label: 'Modification' },
  DELETE: { icon: 'trash', color: 'text-error', label: 'Suppression' },
  LOGIN: { icon: 'right-to-bracket', color: 'text-neutral', label: 'Connexion' },
}

const getConfig = (action: string) => actionConfig[action] || { icon: 'circle', color: 'text-base-content/40', label: action }

const formatDate = (dateStr: string) => {
  const date = new Date(dateStr)
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffMin = Math.floor(diffMs / 60000)

  if (diffMin < 1) return "A l'instant"
  if (diffMin < 60) return `Il y a ${diffMin} min`
  const diffH = Math.floor(diffMin / 60)
  if (diffH < 24) return `Il y a ${diffH}h`
  const diffJ = Math.floor(diffH / 24)
  if (diffJ < 7) return `Il y a ${diffJ}j`
  return date.toLocaleDateString('fr-FR', { day: '2-digit', month: '2-digit' })
}

const tableLabel = (table: string) => table.replace(/_/g, ' ')
</script>

<template>
  <div class="card bg-base-100 border border-base-200 shadow-sm">
    <div class="card-body p-5">
      <h2 class="card-title text-base font-display mb-4">
        <font-awesome-icon icon="clock-rotate-left" class="text-custom-chocolat/60" />
        Activite recente
      </h2>

      <div v-if="items.length === 0" class="flex flex-col items-center justify-center py-8 text-base-content/30">
        <font-awesome-icon icon="clock-rotate-left" class="w-10 h-10 mb-3" />
        <p class="text-sm">Aucune activite recente</p>
      </div>

      <ul v-else class="space-y-1 max-h-96 overflow-y-auto">
        <li
          v-for="item in items"
          :key="item.id"
          class="flex items-start gap-3 py-2 px-2 rounded-lg hover:bg-base-200/50 transition-colors"
        >
          <div class="mt-0.5 flex-shrink-0">
            <font-awesome-icon
              :icon="getConfig(item.action).icon"
              class="w-3.5 h-3.5"
              :class="getConfig(item.action).color"
            />
          </div>
          <div class="flex-1 min-w-0">
            <p class="text-sm leading-tight">
              <span class="font-medium">{{ item.utilisateur_nom || 'Systeme' }}</span>
              <span class="text-base-content/60">
                {{ ' ' + getConfig(item.action).label.toLowerCase() }}
              </span>
              <span class="font-medium text-base-content/80">
                {{ ' ' + tableLabel(item.table_name) }}
              </span>
            </p>
            <p class="text-xs text-base-content/40 mt-0.5">
              <span class="badge badge-outline badge-xs mr-1">{{ item.schema_name }}</span>
              {{ formatDate(item.created_at) }}
            </p>
          </div>
        </li>
      </ul>
    </div>
  </div>
</template>
