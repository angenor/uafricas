<script setup lang="ts">
const props = defineProps<{
  ancienEtat: Record<string, any> | null
  nouvelEtat: Record<string, any> | null
}>()

type DiffLine = {
  key: string
  ancien: string | null
  nouveau: string | null
  type: 'added' | 'removed' | 'changed' | 'unchanged'
}

const diffLines = computed<DiffLine[]>(() => {
  const ancien = props.ancienEtat || {}
  const nouveau = props.nouvelEtat || {}
  const allKeys = new Set([...Object.keys(ancien), ...Object.keys(nouveau)])
  const lines: DiffLine[] = []

  for (const key of allKeys) {
    const aVal = ancien[key]
    const nVal = nouveau[key]
    const aStr = aVal !== undefined ? JSON.stringify(aVal) : null
    const nStr = nVal !== undefined ? JSON.stringify(nVal) : null

    if (aStr === null && nStr !== null) {
      lines.push({ key, ancien: null, nouveau: nStr, type: 'added' })
    } else if (aStr !== null && nStr === null) {
      lines.push({ key, ancien: aStr, nouveau: null, type: 'removed' })
    } else if (aStr !== nStr) {
      lines.push({ key, ancien: aStr, nouveau: nStr, type: 'changed' })
    } else {
      lines.push({ key, ancien: aStr, nouveau: nStr, type: 'unchanged' })
    }
  }

  // Changed/added/removed first, unchanged last
  return lines.sort((a, b) => {
    const order = { changed: 0, added: 1, removed: 2, unchanged: 3 }
    return order[a.type] - order[b.type]
  })
})

const hasDiff = computed(() => diffLines.value.some(l => l.type !== 'unchanged'))

const typeClass = (type: DiffLine['type']) => {
  const map: Record<string, string> = {
    added: 'bg-success/10 border-l-4 border-success',
    removed: 'bg-error/10 border-l-4 border-error',
    changed: 'bg-info/10 border-l-4 border-info',
    unchanged: '',
  }
  return map[type]
}

const typeIcon = (type: DiffLine['type']) => {
  const map: Record<string, string> = {
    added: '+',
    removed: '-',
    changed: '~',
    unchanged: ' ',
  }
  return map[type]
}
</script>

<template>
  <div>
    <div v-if="!ancienEtat && !nouvelEtat" class="text-center text-base-content/50 py-8">
      Aucune donnee before/after disponible
    </div>

    <div v-else-if="!hasDiff" class="text-center text-base-content/50 py-4">
      Aucune difference detectee
    </div>

    <div v-else class="overflow-x-auto">
      <table class="table table-sm table-fixed w-full">
        <thead>
          <tr>
            <th class="w-8"></th>
            <th class="w-40">Champ</th>
            <th>Avant</th>
            <th>Apres</th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="line in diffLines.filter(l => l.type !== 'unchanged')"
            :key="line.key"
            :class="typeClass(line.type)"
          >
            <td class="text-center font-mono font-bold text-sm">{{ typeIcon(line.type) }}</td>
            <td class="font-semibold text-sm">{{ line.key }}</td>
            <td class="font-mono text-xs break-all">
              <span v-if="line.ancien !== null" class="text-error">{{ line.ancien }}</span>
              <span v-else class="text-base-content/30">-</span>
            </td>
            <td class="font-mono text-xs break-all">
              <span v-if="line.nouveau !== null" class="text-success">{{ line.nouveau }}</span>
              <span v-else class="text-base-content/30">-</span>
            </td>
          </tr>
        </tbody>
      </table>

      <!-- Unchanged fields (collapsed) -->
      <details v-if="diffLines.some(l => l.type === 'unchanged')" class="mt-2">
        <summary class="cursor-pointer text-sm text-base-content/50 hover:text-base-content/70">
          {{ diffLines.filter(l => l.type === 'unchanged').length }} champs inchanges
        </summary>
        <table class="table table-sm table-fixed w-full mt-1">
          <tbody>
            <tr v-for="line in diffLines.filter(l => l.type === 'unchanged')" :key="line.key" class="opacity-50">
              <td class="w-8 text-center font-mono text-sm">&nbsp;</td>
              <td class="w-40 font-semibold text-sm">{{ line.key }}</td>
              <td class="font-mono text-xs break-all" colspan="2">{{ line.ancien }}</td>
            </tr>
          </tbody>
        </table>
      </details>
    </div>
  </div>
</template>
