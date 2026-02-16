<template>
  <div class="bg-white rounded-xl shadow-md hover:shadow-lg transition-all duration-300 overflow-hidden border border-gray-100">
    <!-- Header avec etat -->
    <div
      class="px-4 py-3 flex items-center justify-between"
      :class="headerClass"
    >
      <div class="flex items-center gap-2 text-sm font-medium">
        <font-awesome-icon :icon="['fas', etatInfo.icone]" class="w-4 h-4" />
        {{ etatInfo.label }}
      </div>
      <span v-if="session.etat === 'en_cours'" class="flex items-center gap-1 text-xs animate-pulse">
        <font-awesome-icon :icon="['fas', 'circle']" class="w-2 h-2 text-red-500" />
        LIVE
      </span>
    </div>

    <!-- Contenu -->
    <div class="p-4">
      <h4 class="font-semibold text-gray-900 mb-2">
        {{ session.titre || 'Session sans titre' }}
      </h4>

      <!-- Infos -->
      <div class="space-y-1.5 text-sm text-gray-600 mb-4">
        <div v-if="session.date_debut_prevue" class="flex items-center gap-2">
          <font-awesome-icon :icon="['fas', 'calendar-days']" class="w-4 h-4 text-gray-400" />
          <span>{{ dateFormatee }}</span>
        </div>
        <div v-if="session.duree_secondes" class="flex items-center gap-2">
          <font-awesome-icon :icon="['far', 'clock']" class="w-4 h-4 text-gray-400" />
          <span>{{ dureeFormatee }}</span>
        </div>
        <div v-if="session.max_participants" class="flex items-center gap-2">
          <font-awesome-icon :icon="['fas', 'users']" class="w-4 h-4 text-gray-400" />
          <span>Max {{ session.max_participants }} participants</span>
        </div>
        <div v-if="session.nombre_participants_pic" class="flex items-center gap-2">
          <font-awesome-icon :icon="['fas', 'chart-line']" class="w-4 h-4 text-gray-400" />
          <span>Pic : {{ session.nombre_participants_pic }} participants</span>
        </div>
      </div>

      <!-- Actions -->
      <NuxtLink
        :to="`/afrolang/session/${session.id}`"
        class="block w-full text-center px-4 py-2 rounded-lg text-sm font-medium transition-all"
        :class="actionClass"
      >
        {{ actionLabel }}
      </NuxtLink>
    </div>
  </div>
</template>

<script setup lang="ts">
import { getEtatInfo, formatDuree, formatDateHeure, type SessionAPI } from '~/composables/useAfrolang'

const props = defineProps<{
  session: SessionAPI
}>()

const etatInfo = computed(() => getEtatInfo(props.session.etat))

const dateFormatee = computed(() => {
  if (!props.session.date_debut_prevue) return ''
  return formatDateHeure(props.session.date_debut_prevue)
})

const dureeFormatee = computed(() => formatDuree(props.session.duree_secondes))

const headerClass = computed(() => {
  switch (props.session.etat) {
    case 'en_cours': return 'bg-emerald-50 text-emerald-700'
    case 'planifiee': return 'bg-blue-50 text-blue-700'
    case 'terminee': return 'bg-gray-50 text-gray-600'
    case 'annulee': return 'bg-red-50 text-red-600'
    default: return 'bg-gray-50 text-gray-600'
  }
})

const actionClass = computed(() => {
  switch (props.session.etat) {
    case 'en_cours': return 'bg-gradient-to-r from-emerald-500 to-teal-500 text-white hover:shadow-lg'
    case 'planifiee': return 'bg-blue-100 text-blue-700 hover:bg-blue-200'
    default: return 'bg-gray-100 text-gray-600 hover:bg-gray-200'
  }
})

const actionLabel = computed(() => {
  switch (props.session.etat) {
    case 'en_cours': return 'Rejoindre la session'
    case 'planifiee': return 'Voir les détails'
    default: return 'Voir le résumé'
  }
})
</script>
