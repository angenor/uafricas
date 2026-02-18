<script setup lang="ts">
import type { TableColumn, PaginationState } from '~/types/admin'

const props = defineProps<{
  /** Definition des colonnes */
  colonnes: TableColumn[]
  /** Donnees du tableau */
  donnees: any[]
  /** Etat de pagination */
  pagination: PaginationState
  /** Colonne de tri actuelle */
triColonne?: string
  /** Direction de tri actuelle */
  triDirection?: 'asc' | 'desc'
  /** Chargement en cours */
  loading?: boolean
  /** Afficher les cases a cocher de selection */
  selectable?: boolean
  /** Elements selectionnes (ids) */
  selection?: string[]
  /** Cle unique de chaque ligne (defaut: 'id') */
  cleUnique?: string
}>()

const emit = defineEmits<{
  trier: [colonne: string]
  'aller-page': [page: number]
  'update:selection': [ids: string[]]
  'action-ligne': [action: string, item: any]
}>()

defineSlots<{
  /** Slot pour personnaliser le rendu d'une cellule */
  [key: `cell-${string}`]: (props: { item: any, value: any }) => any
  /** Slot pour les actions de ligne */
  actions?: (props: { item: any }) => any
}>()

const cleId = computed(() => props.cleUnique || 'id')

// Selection
const toutSelectionne = computed(() => {
  if (!props.donnees.length) return false
  return props.donnees.every(d => props.selection?.includes(d[cleId.value]))
})

const basculerTout = () => {
  if (toutSelectionne.value) {
    emit('update:selection', [])
  }
  else {
    emit('update:selection', props.donnees.map(d => d[cleId.value]))
  }
}

const basculerLigne = (id: string) => {
  const current = props.selection || []
  if (current.includes(id)) {
    emit('update:selection', current.filter(s => s !== id))
  }
  else {
    emit('update:selection', [...current, id])
  }
}

const getCellValue = (item: any, col: TableColumn) => {
  const value = item[col.key]
  return col.format ? col.format(value, item) : value
}

// Pagination
const pages = computed(() => {
  const total = props.pagination.totalPages
  const current = props.pagination.page
  const result: (number | '...')[] = []

  if (total <= 7) {
    for (let i = 1; i <= total; i++) result.push(i)
    return result
  }

  result.push(1)
  if (current > 3) result.push('...')

  const start = Math.max(2, current - 1)
  const end = Math.min(total - 1, current + 1)
  for (let i = start; i <= end; i++) result.push(i)

  if (current < total - 2) result.push('...')
  result.push(total)

  return result
})
</script>

<template>
  <div class="card bg-base-100 shadow-sm">
    <!-- Table -->
    <div class="overflow-x-auto">
      <table class="table table-sm">
        <thead>
          <tr>
            <th v-if="selectable" class="w-10">
              <input
                type="checkbox"
                class="checkbox checkbox-sm"
                :checked="toutSelectionne"
                @change="basculerTout"
              >
            </th>
            <th
              v-for="col in colonnes"
              :key="col.key"
              :class="[
                col.width,
                col.align === 'center' && 'text-center',
                col.align === 'right' && 'text-right',
                col.sortable && 'cursor-pointer select-none hover:bg-base-200',
              ]"
              @click="col.sortable && emit('trier', col.key)"
            >
              <span class="inline-flex items-center gap-1">
                {{ col.label }}
                <span v-if="col.sortable && triColonne === col.key" class="text-primary">
                  <font-awesome-icon
                    :icon="triDirection === 'asc' ? 'arrow-up' : 'arrow-down'"
                    class="text-xs"
                  />
                </span>
                <span v-else-if="col.sortable" class="text-base-content/20">
                  <font-awesome-icon icon="sort" class="text-xs" />
                </span>
              </span>
            </th>
            <th v-if="$slots.actions" class="w-20 text-center">
              Actions
            </th>
          </tr>
        </thead>

        <tbody>
          <!-- Chargement -->
          <tr v-if="loading">
            <td
              :colspan="colonnes.length + (selectable ? 1 : 0) + ($slots.actions ? 1 : 0)"
              class="text-center py-12"
            >
              <span class="loading loading-spinner loading-md" />
              <p class="text-sm text-base-content/50 mt-2">
                Chargement...
              </p>
            </td>
          </tr>

          <!-- Aucune donnee -->
          <tr v-else-if="!donnees.length">
            <td
              :colspan="colonnes.length + (selectable ? 1 : 0) + ($slots.actions ? 1 : 0)"
              class="text-center py-12"
            >
              <font-awesome-icon icon="inbox" class="text-3xl text-base-content/20 mb-2" />
              <p class="text-sm text-base-content/50">
                Aucun element trouve
              </p>
            </td>
          </tr>

          <!-- Donnees -->
          <tr
            v-for="item in donnees"
            v-else
            :key="item[cleId]"
            class="hover:bg-base-200/50"
          >
            <td v-if="selectable">
              <input
                type="checkbox"
                class="checkbox checkbox-sm"
                :checked="selection?.includes(item[cleId])"
                @change="basculerLigne(item[cleId])"
              >
            </td>
            <td
              v-for="col in colonnes"
              :key="col.key"
              :class="[
                col.align === 'center' && 'text-center',
                col.align === 'right' && 'text-right',
              ]"
            >
              <slot :name="`cell-${col.key}`" :item="item" :value="getCellValue(item, col)">
                {{ getCellValue(item, col) ?? '—' }}
              </slot>
            </td>
            <td v-if="$slots.actions" class="text-center">
              <slot name="actions" :item="item" />
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Pagination -->
    <div
      v-if="pagination.totalPages > 1"
      class="flex items-center justify-between px-4 py-3 border-t border-base-200"
    >
      <p class="text-sm text-base-content/60">
        {{ pagination.total }} element{{ pagination.total > 1 ? 's' : '' }}
        — Page {{ pagination.page }} / {{ pagination.totalPages }}
      </p>
      <div class="join">
        <button
          class="join-item btn btn-sm"
          :disabled="pagination.page <= 1"
          @click="emit('aller-page', pagination.page - 1)"
        >
          <font-awesome-icon icon="chevron-left" />
        </button>
        <template v-for="p in pages" :key="p">
          <button
            v-if="p === '...'"
            class="join-item btn btn-sm btn-disabled"
          >
            ...
          </button>
          <button
            v-else
            class="join-item btn btn-sm"
            :class="{ 'btn-active': p === pagination.page }"
            @click="emit('aller-page', p as number)"
          >
            {{ p }}
          </button>
        </template>
        <button
          class="join-item btn btn-sm"
          :disabled="pagination.page >= pagination.totalPages"
          @click="emit('aller-page', pagination.page + 1)"
        >
          <font-awesome-icon icon="chevron-right" />
        </button>
      </div>
    </div>
  </div>
</template>
