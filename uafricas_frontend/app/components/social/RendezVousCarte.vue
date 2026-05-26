<script setup lang="ts">
import type { RendezVousAPI } from '~/composables/useRendezVous'

const props = defineProps<{ rdv: RendezVousAPI }>()
const emit = defineEmits<{
  (e: 'accepter' | 'refuser' | 'contre' | 'annuler' | 'rejoindre'): void
  (e: 'messagerie'): void
}>()

const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string

const nomComplet = computed(() => `${props.rdv.autre.prenom} ${props.rdv.autre.nom}`.trim())
const initiale = computed(() => (props.rdv.autre.prenom?.[0] || props.rdv.autre.nom?.[0] || '?').toUpperCase())
const photo = computed(() => {
  const url = props.rdv.autre.photoUrl
  if (!url) return null
  return url.startsWith('http') ? url : `${apiBase}${url}`
})

const dateFormatee = computed(() =>
  new Intl.DateTimeFormat('fr-FR', {
    weekday: 'short', day: 'numeric', month: 'short', hour: '2-digit', minute: '2-digit',
  }).format(new Date(props.rdv.date_heure)),
)

// Libellé + style du badge de statut (statut persisté ou état dérivé).
const badge = computed(() => {
  if (props.rdv.etat_derive === 'expire') return { texte: 'Expiré', classe: 'bg-gray-100 text-gray-500' }
  if (props.rdv.etat_derive === 'termine') return { texte: 'Terminé', classe: 'bg-gray-100 text-gray-500' }
  switch (props.rdv.statut) {
    case 'propose': return { texte: 'En attente', classe: 'bg-amber-100 text-amber-700' }
    case 'accepte': return { texte: 'Accepté', classe: 'bg-green-100 text-green-700' }
    case 'refuse': return { texte: 'Refusé', classe: 'bg-red-100 text-red-600' }
    case 'annule': return { texte: 'Annulé', classe: 'bg-gray-100 text-gray-500' }
    default: return { texte: props.rdv.statut, classe: 'bg-gray-100 text-gray-500' }
  }
})

// Actions disponibles selon l'état (et le tour).
const peutRepondre = computed(() => props.rdv.statut === 'propose' && props.rdv.mon_tour && !props.rdv.etat_derive)
const enAttenteAutre = computed(() => props.rdv.statut === 'propose' && !props.rdv.mon_tour && !props.rdv.etat_derive)
const peutAnnuler = computed(() =>
  (props.rdv.statut === 'propose' || props.rdv.statut === 'accepte') && !props.rdv.etat_derive,
)
</script>

<template>
  <article class="bg-white rounded-2xl border border-gray-100 shadow-sm p-4">
    <!-- En-tête : membre + statut -->
    <div class="flex items-start gap-3">
      <div class="shrink-0">
        <img v-if="photo" :src="photo" :alt="nomComplet" class="w-11 h-11 rounded-full object-cover">
        <div v-else class="w-11 h-11 rounded-full bg-linear-to-br from-custom-chocolat to-custom-green text-white flex items-center justify-center font-bold">
          {{ initiale }}
        </div>
      </div>
      <div class="min-w-0 flex-1">
        <div class="flex items-center justify-between gap-2">
          <p class="font-semibold text-gray-800 truncate">{{ nomComplet }}</p>
          <span class="shrink-0 text-xs font-semibold px-2 py-0.5 rounded-full" :class="badge.classe">{{ badge.texte }}</span>
        </div>
        <p v-if="rdv.autre.fonction" class="text-xs text-gray-500 truncate">{{ rdv.autre.fonction }}</p>
        <p v-if="rdv.autre.pays" class="text-xs text-gray-400 truncate">{{ rdv.autre.pays }}</p>
      </div>
    </div>

    <!-- Sujet & créneau -->
    <div class="mt-3">
      <p class="font-medium text-gray-800">{{ rdv.sujet }}</p>
      <p v-if="rdv.description" class="text-sm text-gray-500 mt-0.5 line-clamp-2">{{ rdv.description }}</p>
      <div class="flex items-center gap-3 text-xs text-gray-500 mt-2">
        <span class="inline-flex items-center gap-1">
          <font-awesome-icon icon="fa-solid fa-calendar" />
          {{ dateFormatee }}
        </span>
        <span class="inline-flex items-center gap-1">
          <font-awesome-icon icon="fa-solid fa-clock" />
          {{ rdv.duree_minutes }} min
        </span>
      </div>
    </div>

    <!-- Statut « en attente de l'autre » -->
    <p v-if="enAttenteAutre" class="text-xs text-gray-400 mt-3">
      En attente de la réponse de {{ rdv.autre.prenom }}.
    </p>

    <!-- Actions -->
    <div class="flex flex-wrap items-center gap-2 mt-3">
      <!-- Répondre (mon tour) -->
      <template v-if="peutRepondre">
        <button
          type="button"
          class="px-3 py-1.5 rounded-lg text-xs font-semibold text-white bg-custom-green hover:shadow-md transition"
          @click="emit('accepter')"
        >
          <font-awesome-icon icon="fa-solid fa-check" /> Accepter
        </button>
        <button
          type="button"
          class="px-3 py-1.5 rounded-lg text-xs font-semibold text-red-600 border border-red-200 hover:bg-red-50 transition"
          @click="emit('refuser')"
        >
          <font-awesome-icon icon="fa-solid fa-xmark" /> Refuser
        </button>
        <button
          type="button"
          class="px-3 py-1.5 rounded-lg text-xs font-semibold text-custom-chocolat border border-custom-chocolat/30 hover:bg-custom-chocolat/5 transition"
          @click="emit('contre')"
        >
          <font-awesome-icon icon="fa-solid fa-calendar-days" /> Contre-proposer
        </button>
      </template>

      <!-- Rejoindre (RDV accepté dans la fenêtre, US4) -->
      <button
        v-if="rdv.statut === 'accepte'"
        type="button"
        :disabled="!rdv.peut_rejoindre"
        class="px-3 py-1.5 rounded-lg text-xs font-semibold text-white bg-linear-to-r from-custom-chocolat to-custom-green hover:shadow-md transition disabled:opacity-50 disabled:cursor-not-allowed"
        :title="rdv.peut_rejoindre ? 'Rejoindre la visioconférence' : 'Disponible 5 min avant le rendez-vous'"
        @click="emit('rejoindre')"
      >
        <font-awesome-icon icon="fa-solid fa-video" /> Rejoindre
      </button>

      <!-- Lien messagerie privée -->
      <button
        type="button"
        class="px-3 py-1.5 rounded-lg text-xs font-semibold text-gray-500 hover:bg-gray-100 transition"
        @click="emit('messagerie')"
      >
        <font-awesome-icon icon="fa-solid fa-comments" /> Message
      </button>

      <!-- Annuler -->
      <button
        v-if="peutAnnuler"
        type="button"
        class="px-3 py-1.5 rounded-lg text-xs font-semibold text-gray-400 hover:text-red-600 hover:bg-red-50 transition ml-auto"
        @click="emit('annuler')"
      >
        <font-awesome-icon icon="fa-solid fa-ban" /> Annuler
      </button>
    </div>
  </article>
</template>
