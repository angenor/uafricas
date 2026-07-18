<script setup lang="ts">
import type { PermissionListeItem } from '~/types/admin'

const props = defineProps<{
  permissions: PermissionListeItem[]
  selectedIds: string[]
  disabled?: boolean
}>()

const emit = defineEmits<{
  'update:selectedIds': [ids: string[]]
}>()

// Grouper par type_ressource
const groupes = computed(() => {
  const map: Record<string, PermissionListeItem[]> = {}
  for (const perm of props.permissions) {
    const key = perm.type_ressource || 'autre'
    if (!map[key]) map[key] = []
    map[key].push(perm)
  }
  return Object.entries(map).sort(([a], [b]) => a.localeCompare(b))
})

const isSelected = (id: string) => props.selectedIds.includes(id)

const basculer = (id: string) => {
  if (props.disabled) return
  if (isSelected(id)) {
    emit('update:selectedIds', props.selectedIds.filter(s => s !== id))
  }
  else {
    emit('update:selectedIds', [...props.selectedIds, id])
  }
}

const toutSelectionnerGroupe = (perms: PermissionListeItem[]) => {
  const ids = perms.map(p => p.id)
  const allSelected = ids.every(id => isSelected(id))
  if (allSelected) {
    emit('update:selectedIds', props.selectedIds.filter(s => !ids.includes(s)))
  }
  else {
    const newIds = new Set([...props.selectedIds, ...ids])
    emit('update:selectedIds', [...newIds])
  }
}

const groupeToutSelectionne = (perms: PermissionListeItem[]) => {
  return perms.every(p => isSelected(p.id))
}
</script>

<template>
  <div class="space-y-4">
    <div v-for="[ressource, perms] in groupes" :key="ressource" class="card bg-base-200/50">
      <div class="card-body p-3">
        <div class="flex items-center gap-2 mb-2">
          <input
            type="checkbox"
            class="checkbox checkbox-sm checkbox-primary"
            :checked="groupeToutSelectionne(perms)"
            :disabled="disabled"
            @change="toutSelectionnerGroupe(perms)"
          >
          <h4 class="font-semibold text-sm" :title="ressource">{{ libelleRessource(ressource) }}</h4>
          <span class="badge badge-xs">{{ perms.length }}</span>
        </div>
        <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-2">
          <label
            v-for="perm in perms"
            :key="perm.id"
            class="flex items-center gap-2 cursor-pointer p-1.5 rounded hover:bg-base-300/50"
            :class="{ 'opacity-50': disabled }"
          >
            <input
              type="checkbox"
              class="checkbox checkbox-xs"
              :checked="isSelected(perm.id)"
              :disabled="disabled"
              @change="basculer(perm.id)"
            >
            <span class="text-sm" :title="perm.action">{{ libellePermissionAction(perm.action) }}</span>
          </label>
        </div>
      </div>
    </div>
    <div v-if="!groupes.length" class="text-center text-base-content/50 py-4">
      Aucune permission disponible
    </div>
  </div>
</template>
