<script setup lang="ts">
// Carte d'une demande/invitation d'adhésion (US5)
// Tailwind v4 pur
import type { AdhesionSallePriveeAPI, EtatAdhesion } from '~/composables/useAfrolang'

interface Props {
  adhesion: AdhesionSallePriveeAPI
  peutDecider?: boolean
}

const props = defineProps<Props>()
const emit = defineEmits<{
  (e: 'decidee', etat: EtatAdhesion): void
  (e: 'retiree'): void
}>()

const { decisionAdhesion, retirerAbonne, chargement } = useAfrolang()

const erreurLocale = ref<string | null>(null)

const libelleEtat = computed(() => {
  switch (props.adhesion.etat) {
    case 'en_attente': return 'En attente'
    case 'acceptee': return 'Acceptée'
    case 'refusee': return 'Refusée'
    case 'groupe_complet': return 'Groupe complet'
    default: return props.adhesion.etat
  }
})

const classEtat = computed(() => {
  switch (props.adhesion.etat) {
    case 'en_attente': return 'bg-amber-100 text-amber-800'
    case 'acceptee': return 'bg-green-100 text-green-800'
    case 'refusee': return 'bg-red-100 text-red-800'
    case 'groupe_complet': return 'bg-gray-200 text-gray-700'
    default: return 'bg-gray-100 text-gray-700'
  }
})

const libelleType = computed(() => {
  switch (props.adhesion.type_adhesion) {
    case 'demande': return 'Demande'
    case 'invitation': return 'Invitation'
    case 'abonne': return 'Abonné'
    default: return props.adhesion.type_adhesion
  }
})

const decider = async (decision: 'acceptee' | 'refusee') => {
  erreurLocale.value = null
  const res = await decisionAdhesion(props.adhesion.id, decision)
  if (res) emit('decidee', res.etat)
  else erreurLocale.value = 'Échec de la décision'
}

const retirer = async () => {
  erreurLocale.value = null
  const ok = await retirerAbonne(props.adhesion.id)
  if (ok) emit('retiree')
  else erreurLocale.value = 'Échec du retrait'
}
</script>

<template>
  <article class="rounded-lg border border-gray-200 bg-white p-4 space-y-3">
    <header class="flex items-start justify-between gap-3">
      <div class="flex items-center gap-3">
        <div
          v-if="adhesion.utilisateur_photo"
          class="h-10 w-10 overflow-hidden rounded-full"
        >
          <img :src="adhesion.utilisateur_photo" :alt="adhesion.utilisateur_nom ?? ''" class="h-full w-full object-cover" />
        </div>
        <div
          v-else
          class="flex h-10 w-10 items-center justify-center rounded-full bg-custom-chocolat text-white font-semibold"
        >
          {{ (adhesion.utilisateur_prenom?.[0] ?? '') + (adhesion.utilisateur_nom?.[0] ?? '') }}
        </div>
        <div>
          <p class="text-sm font-medium text-gray-900">
            {{ adhesion.utilisateur_prenom }} {{ adhesion.utilisateur_nom }}
          </p>
          <p class="text-xs text-gray-500">{{ libelleType }}</p>
        </div>
      </div>
      <span :class="['rounded-full px-2.5 py-0.5 text-xs font-medium', classEtat]">
        {{ libelleEtat }}
      </span>
    </header>

    <div v-if="peutDecider && adhesion.etat === 'en_attente'" class="flex gap-2">
      <button
        type="button"
        :disabled="chargement"
        class="flex-1 rounded-md bg-custom-green px-3 py-2 text-sm font-medium text-white hover:bg-green-700 disabled:opacity-50"
        @click="decider('acceptee')"
      >
        Accepter
      </button>
      <button
        type="button"
        :disabled="chargement"
        class="flex-1 rounded-md border border-red-300 bg-white px-3 py-2 text-sm font-medium text-red-700 hover:bg-red-50 disabled:opacity-50"
        @click="decider('refusee')"
      >
        Refuser
      </button>
    </div>

    <div v-else-if="peutDecider && adhesion.etat === 'acceptee' && adhesion.type_adhesion === 'abonne'" class="flex justify-end">
      <button
        type="button"
        :disabled="chargement"
        class="rounded-md border border-gray-300 bg-white px-3 py-1.5 text-xs font-medium text-gray-700 hover:bg-gray-50"
        @click="retirer"
      >
        Retirer l'abonné
      </button>
    </div>

    <p v-if="erreurLocale" class="text-xs text-red-600">{{ erreurLocale }}</p>
  </article>
</template>
