<script setup lang="ts">
// Bannière pour une recommandation d'accompagnateur reçue
// (feature 001-ressources-fermeture-session, US1). Tailwind v4 pur.
import { ref } from 'vue'
import type { RecommandationRecueAPI } from '~/composables/useAfrolangAccompagnateur'
import { useAfrolangAccompagnateur } from '~/composables/useAfrolangAccompagnateur'

const props = defineProps<{
  recommandation: RecommandationRecueAPI
}>()

const emit = defineEmits<{
  (e: 'actualisee'): void
}>()

const { accepter, refuser, retirerConsentement, chargement } = useAfrolangAccompagnateur()

const modalRefusOuvert = ref(false)
const motifRefus = ref('')

const dateFormatee = (iso: string) =>
  new Date(iso).toLocaleDateString('fr-FR', { day: '2-digit', month: 'short', year: 'numeric' })

const onAccepter = async () => {
  const res = await accepter(props.recommandation.id)
  if (res) emit('actualisee')
}

const onRefuserConfirmer = async () => {
  const res = await refuser(props.recommandation.id, motifRefus.value.trim() || null)
  if (res) {
    modalRefusOuvert.value = false
    motifRefus.value = ''
    emit('actualisee')
  }
}

const onRetirer = async () => {
  if (!confirm('Retirer votre consentement comme accompagnateur ? Vous ne serez plus affiché publiquement.')) return
  const res = await retirerConsentement(props.recommandation.id)
  if (res) emit('actualisee')
}

const couleursStatut: Record<string, string> = {
  en_attente: 'bg-amber-50 border-amber-200 text-amber-900',
  acceptee: 'bg-green-50 border-green-200 text-green-900',
  refusee: 'bg-gray-50 border-gray-200 text-gray-700',
  retiree: 'bg-gray-50 border-gray-200 text-gray-700',
}

const libelleStatut: Record<string, string> = {
  en_attente: 'En attente de votre réponse',
  acceptee: 'Accepté — visible publiquement',
  refusee: 'Refusé',
  retiree: 'Consentement retiré',
}
</script>

<template>
  <article class="border rounded-xl p-4 mb-3" :class="couleursStatut[recommandation.statut_accompagnateur]">
    <header class="flex items-start justify-between gap-3 mb-3">
      <div class="flex-1 min-w-0">
        <div class="text-xs font-semibold uppercase tracking-wide mb-1">
          {{ libelleStatut[recommandation.statut_accompagnateur] }}
        </div>
        <h4 class="text-base font-semibold text-gray-900">
          Salle « {{ recommandation.salle.titre }} »
        </h4>
        <p v-if="recommandation.salle.groupe_ethnique_nom" class="text-xs text-gray-600 mt-0.5">
          Groupe ethnique : {{ recommandation.salle.groupe_ethnique_nom }}
        </p>
      </div>
      <time class="text-xs text-gray-500 shrink-0">{{ dateFormatee(recommandation.created_at) }}</time>
    </header>

    <div class="mb-3">
      <p class="text-xs text-gray-700 mb-1">
        Recommandé(e) par
        <span class="font-medium">{{ recommandation.auteur.prenom }} {{ recommandation.auteur.nom }}</span> :
      </p>
      <blockquote class="text-sm text-gray-800 italic border-l-2 border-gray-300 pl-3 py-1">
        « {{ recommandation.motif_recommandation }} »
      </blockquote>
    </div>

    <!-- Actions selon statut -->
    <div v-if="recommandation.statut_accompagnateur === 'en_attente'" class="flex gap-2">
      <button type="button"
              :disabled="chargement"
              class="flex-1 inline-flex items-center justify-center gap-2 px-4 py-2 bg-custom-chocolat text-white text-sm rounded-md hover:bg-custom-chocolat/90 transition-colors disabled:opacity-50"
              @click="onAccepter">
        <font-awesome-icon icon="fa-solid fa-circle-check" />
        Accepter
      </button>
      <button type="button"
              :disabled="chargement"
              class="flex-1 inline-flex items-center justify-center gap-2 px-4 py-2 bg-white text-gray-700 border border-gray-300 text-sm rounded-md hover:bg-gray-50 transition-colors disabled:opacity-50"
              @click="modalRefusOuvert = true">
        <font-awesome-icon icon="fa-solid fa-circle-xmark" />
        Refuser
      </button>
    </div>
    <div v-else-if="recommandation.statut_accompagnateur === 'acceptee'" class="flex gap-2">
      <button type="button"
              :disabled="chargement"
              class="inline-flex items-center gap-2 px-3 py-1.5 text-xs text-gray-700 border border-gray-300 rounded-md hover:bg-white transition-colors disabled:opacity-50"
              @click="onRetirer">
        <font-awesome-icon icon="fa-solid fa-arrow-rotate-left" class="text-xs" />
        Retirer mon consentement
      </button>
    </div>

    <!-- Modal refus -->
    <Teleport to="body">
      <div v-if="modalRefusOuvert"
           class="fixed inset-0 z-50 bg-black/50 flex items-center justify-center p-4"
           @click.self="modalRefusOuvert = false">
        <div class="w-full max-w-md bg-white rounded-xl shadow-2xl p-5">
          <h3 class="text-base font-semibold text-gray-900 mb-3">Refuser cette recommandation</h3>
          <p class="text-xs text-gray-600 mb-3">
            Le motif est facultatif et ne sera <strong>pas transmis</strong> à la personne qui vous a recommandé(e).
          </p>
          <textarea v-model="motifRefus"
                    rows="3"
                    maxlength="500"
                    placeholder="Motif interne (optionnel, ≤ 500 caractères)…"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-custom-chocolat focus:border-transparent" />
          <div class="flex justify-end gap-2 mt-3">
            <button type="button" class="px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 rounded-md" @click="modalRefusOuvert = false">
              Annuler
            </button>
            <button type="button"
                    :disabled="chargement"
                    class="px-4 py-2 text-sm font-medium text-white bg-red-600 hover:bg-red-700 rounded-md disabled:opacity-50"
                    @click="onRefuserConfirmer">
              Confirmer le refus
            </button>
          </div>
        </div>
      </div>
    </Teleport>
  </article>
</template>
