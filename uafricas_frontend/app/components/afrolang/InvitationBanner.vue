<script setup lang="ts">
// Bannière invitation reçue — US5
// Tailwind v4 pur
import type { AdhesionSallePriveeAPI } from '~/composables/useAfrolang'

interface Props {
  invitation: AdhesionSallePriveeAPI
  titreSalle?: string
}

const props = defineProps<Props>()
const emit = defineEmits<{
  (e: 'decidee'): void
}>()

const { decisionAdhesion, chargement } = useAfrolang()
const erreurLocale = ref<string | null>(null)

const repondre = async (decision: 'acceptee' | 'refusee') => {
  erreurLocale.value = null
  const res = await decisionAdhesion(props.invitation.id, decision)
  if (res) emit('decidee')
  else erreurLocale.value = 'Échec de la décision'
}
</script>

<template>
  <div class="rounded-lg border border-amber-300 bg-amber-50 p-4 flex items-start gap-3">
    <div class="text-amber-600">
      <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
      </svg>
    </div>
    <div class="flex-1 space-y-2">
      <div>
        <p class="text-sm font-semibold text-amber-900">Invitation reçue</p>
        <p class="text-xs text-amber-800">
          Vous avez été invité à rejoindre
          <span v-if="titreSalle" class="font-medium">« {{ titreSalle }} »</span>
          <span v-else>une salle privée</span>.
        </p>
      </div>
      <div class="flex gap-2">
        <button
          type="button"
          :disabled="chargement"
          class="rounded-md bg-custom-green px-3 py-1.5 text-xs font-medium text-white hover:bg-green-700 disabled:opacity-50"
          @click="repondre('acceptee')"
        >
          Accepter
        </button>
        <button
          type="button"
          :disabled="chargement"
          class="rounded-md border border-amber-300 bg-white px-3 py-1.5 text-xs font-medium text-amber-900 hover:bg-amber-100"
          @click="repondre('refusee')"
        >
          Refuser
        </button>
      </div>
      <p v-if="erreurLocale" class="text-xs text-red-700">{{ erreurLocale }}</p>
    </div>
  </div>
</template>
