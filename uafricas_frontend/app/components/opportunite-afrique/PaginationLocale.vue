<script setup lang="ts">
interface Props {
  /** Page courante (v-model). */
  page: number
  /** Nombre total de pages. */
  totalPages: number
  /** Couleur d'accent de la section (classes Tailwind de fond pour la page active). */
  accentClass?: string
}

const props = withDefaults(defineProps<Props>(), {
  accentClass: 'bg-custom-green border-custom-green text-white',
})

const emit = defineEmits<{ (e: 'update:page', value: number): void }>()

// Suite « intelligente » de numéros : 1 … (cur-1) cur (cur+1) … total
const pages = computed<(number | '…')[]>(() => {
  const total = props.totalPages
  const cur = props.page
  if (total <= 7) return Array.from({ length: total }, (_, i) => i + 1)

  const res: (number | '…')[] = [1]
  const debut = Math.max(2, cur - 1)
  const fin = Math.min(total - 1, cur + 1)
  if (debut > 2) res.push('…')
  for (let i = debut; i <= fin; i++) res.push(i)
  if (fin < total - 1) res.push('…')
  res.push(total)
  return res
})

const aller = (p: number) => {
  if (p >= 1 && p <= props.totalPages && p !== props.page) emit('update:page', p)
}
</script>

<template>
  <nav
    v-if="totalPages > 1"
    class="mt-8 flex items-center justify-center gap-1.5"
    aria-label="Pagination"
  >
    <button
      type="button"
      :disabled="page <= 1"
      class="inline-flex items-center justify-center w-9 h-9 rounded-md border border-gray-300 bg-white text-gray-600 hover:bg-gray-50 disabled:opacity-40 disabled:hover:bg-white transition-colors cursor-pointer disabled:cursor-default"
      aria-label="Page précédente"
      @click="aller(page - 1)"
    >
      <font-awesome-icon :icon="['fas', 'chevron-left']" class="w-3.5 h-3.5" />
    </button>

    <template v-for="(p, i) in pages" :key="i">
      <span
        v-if="p === '…'"
        class="inline-flex items-center justify-center w-9 h-9 text-gray-400 select-none"
      >…</span>
      <button
        v-else
        type="button"
        class="inline-flex items-center justify-center w-9 h-9 rounded-md border text-sm font-medium transition-colors cursor-pointer"
        :class="p === page
          ? accentClass
          : 'border-gray-300 bg-white text-gray-700 hover:bg-gray-50'"
        :aria-current="p === page ? 'page' : undefined"
        @click="aller(p)"
      >
        {{ p }}
      </button>
    </template>

    <button
      type="button"
      :disabled="page >= totalPages"
      class="inline-flex items-center justify-center w-9 h-9 rounded-md border border-gray-300 bg-white text-gray-600 hover:bg-gray-50 disabled:opacity-40 disabled:hover:bg-white transition-colors cursor-pointer disabled:cursor-default"
      aria-label="Page suivante"
      @click="aller(page + 1)"
    >
      <font-awesome-icon :icon="['fas', 'chevron-right']" class="w-3.5 h-3.5" />
    </button>
  </nav>
</template>
