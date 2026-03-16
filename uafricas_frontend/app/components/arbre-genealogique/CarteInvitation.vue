<script setup lang="ts">
import { libellePermission } from '~/mocks/collaboration'
import type { Invitation } from '~/mocks/collaboration'

defineProps<{
  invitation: Invitation
}>()

const emit = defineEmits<{
  accepter: [id: string]
  refuser: [id: string]
}>()
</script>

<template>
  <div class="flex items-center justify-between rounded-xl border border-[var(--color-custom-green)]/30 bg-[var(--color-custom-green)]/5 p-4">
    <div>
      <p class="text-sm font-semibold text-stone-800">
        {{ invitation.proprietaire_nom }} vous invite à collaborer
      </p>
      <p class="text-xs text-stone-500">
        Permission : {{ libellePermission(invitation.permission) }}
      </p>
    </div>
    <div class="flex gap-2">
      <button
        class="rounded-lg bg-[var(--color-custom-green)] px-4 py-1.5 text-sm font-semibold text-white transition-colors hover:bg-[var(--color-custom-green)]/90"
        @click="emit('accepter', invitation.id)"
      >
        Accepter
      </button>
      <button
        class="rounded-lg border border-stone-300 px-4 py-1.5 text-sm text-stone-600 transition-colors hover:bg-stone-50"
        @click="emit('refuser', invitation.id)"
      >
        Refuser
      </button>
    </div>
  </div>
</template>
