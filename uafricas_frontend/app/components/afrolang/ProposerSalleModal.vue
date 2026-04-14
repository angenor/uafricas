<template>
  <Teleport to="body">
    <div
      v-if="ouvert"
      class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
      @click.self="fermer"
    >
      <div
        class="relative w-full max-w-lg bg-white rounded-2xl shadow-2xl p-6 sm:p-8 max-h-[90vh] overflow-y-auto"
      >
        <button
          type="button"
          class="absolute top-3 right-3 w-9 h-9 flex items-center justify-center rounded-full text-gray-500 hover:text-custom-chocolat hover:bg-gray-100 transition"
          aria-label="Fermer la fenêtre"
          @click="fermer"
        >
          <font-awesome-icon icon="xmark" />
        </button>

        <h2 class="text-2xl font-bold text-custom-chocolat mb-1">
          Proposer une salle Afrolang
        </h2>
        <p class="text-sm text-gray-600 mb-5">
          Votre proposition sera examinée par l'équipe d'administration sous 7 jours ouvrés.
        </p>

        <!-- État : doublon détecté -->
        <div
          v-if="etat === 'doublon'"
          class="mb-5 border border-amber-300 bg-amber-50 rounded-xl p-4 text-sm"
        >
          <p class="font-semibold text-amber-800 mb-1">Une demande équivalente existe déjà</p>
          <p class="text-amber-900">
            {{ messageDoublon }}
          </p>
          <div class="mt-3 flex flex-wrap gap-2">
            <NuxtLink
              v-if="salleExistanteId"
              :to="`/afrolang/${salleExistanteId}`"
              class="inline-flex items-center gap-1 px-3 py-1.5 rounded-lg bg-custom-chocolat text-white text-xs font-medium hover:bg-custom-chocolat/90"
              @click="fermer"
            >
              Accéder à la salle existante
              <font-awesome-icon icon="arrow-right" class="text-[10px]" />
            </NuxtLink>
            <NuxtLink
              v-else-if="propositionDoublonId"
              to="/afrolang/proposer"
              class="inline-flex items-center gap-1 px-3 py-1.5 rounded-lg bg-custom-chocolat text-white text-xs font-medium hover:bg-custom-chocolat/90"
              @click="fermer"
            >
              Voir les propositions en cours
              <font-awesome-icon icon="arrow-right" class="text-[10px]" />
            </NuxtLink>
          </div>
        </div>

        <form class="space-y-4" @submit.prevent="soumettre">
          <div>
            <label for="nom-groupe" class="block text-sm font-semibold text-gray-800 mb-1.5">
              Nom du groupe ethnique <span class="text-red-500">*</span>
            </label>
            <input
              id="nom-groupe"
              v-model="form.nom_groupe_ethnique"
              type="text"
              required
              maxlength="250"
              placeholder="Ex : Zulu, Haoussa, Wolof…"
              class="w-full px-3 py-2 rounded-lg border border-gray-300 focus:border-custom-chocolat focus:ring-2 focus:ring-custom-chocolat/20 outline-none transition"
            >
          </div>

          <div>
            <label for="langue-cible" class="block text-sm font-semibold text-gray-800 mb-1.5">
              Langue enseignée
            </label>
            <input
              id="langue-cible"
              v-model="form.langue_cible"
              type="text"
              maxlength="100"
              placeholder="Ex : isiZulu, haoussa, wolof…"
              class="w-full px-3 py-2 rounded-lg border border-gray-300 focus:border-custom-chocolat focus:ring-2 focus:ring-custom-chocolat/20 outline-none transition"
            >
          </div>

          <div>
            <label for="description" class="block text-sm font-semibold text-gray-800 mb-1.5">
              Description
            </label>
            <textarea
              id="description"
              v-model="form.description"
              rows="3"
              maxlength="2000"
              placeholder="Pourquoi cette salle est-elle utile ? Qui pourrait en bénéficier ?"
              class="w-full px-3 py-2 rounded-lg border border-gray-300 focus:border-custom-chocolat focus:ring-2 focus:ring-custom-chocolat/20 outline-none transition resize-none"
            />
          </div>

          <div v-if="erreurLocale" class="text-sm text-red-600 bg-red-50 border border-red-200 rounded-lg p-2.5">
            {{ erreurLocale }}
          </div>

          <div class="flex items-center justify-end gap-2 pt-2">
            <button
              type="button"
              class="px-4 py-2 rounded-lg text-gray-700 hover:bg-gray-100 transition"
              @click="fermer"
            >
              Annuler
            </button>
            <button
              type="submit"
              :disabled="chargement || !peutSoumettre"
              class="px-5 py-2 rounded-lg bg-custom-chocolat text-white font-semibold hover:bg-custom-chocolat/90 disabled:opacity-50 disabled:cursor-not-allowed transition"
            >
              <font-awesome-icon v-if="chargement" icon="spinner" class="animate-spin mr-2" />
              <font-awesome-icon v-else icon="paper-plane" class="mr-2" />
              Soumettre la proposition
            </button>
          </div>
        </form>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { computed, reactive, ref, watch } from 'vue'
import type { PropositionSalleAPI, ProposerSalleForm } from '~/composables/useAfrolang'
import { useAfrolang } from '~/composables/useAfrolang'

interface Props {
  ouvert: boolean
  nomPrerempli?: string
}

const props = withDefaults(defineProps<Props>(), {
  nomPrerempli: '',
})

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'created', proposition: PropositionSalleAPI): void
}>()

const { soumettrePropositionSalle, chargement } = useAfrolang()

const form = reactive<ProposerSalleForm>({
  nom_groupe_ethnique: '',
  pays_id: null,
  groupe_ethnique_id: null,
  langue_cible: '',
  description: '',
})

const etat = ref<'idle' | 'doublon'>('idle')
const salleExistanteId = ref<string | null>(null)
const propositionDoublonId = ref<string | null>(null)
const messageDoublon = ref<string>('')
const erreurLocale = ref<string | null>(null)

const peutSoumettre = computed(
  () => form.nom_groupe_ethnique.trim().length > 0 && !chargement.value,
)

watch(
  () => props.ouvert,
  (val) => {
    if (val) {
      form.nom_groupe_ethnique = props.nomPrerempli
      form.langue_cible = ''
      form.description = ''
      etat.value = 'idle'
      salleExistanteId.value = null
      propositionDoublonId.value = null
      messageDoublon.value = ''
      erreurLocale.value = null
    }
  },
  { immediate: true },
)

const fermer = () => {
  emit('close')
}

const soumettre = async () => {
  erreurLocale.value = null
  etat.value = 'idle'

  const reponse = await soumettrePropositionSalle({ ...form })

  if (!reponse) {
    erreurLocale.value = 'Impossible de soumettre la proposition.'
    return
  }

  if ('erreur' in reponse && reponse.erreur === 'doublon') {
    etat.value = 'doublon'
    salleExistanteId.value = reponse.salle_id ?? null
    propositionDoublonId.value = reponse.proposition_id ?? null
    messageDoublon.value = reponse.salle_id
      ? 'Une salle publique existe déjà pour ce groupe ethnique.'
      : 'Une proposition pour ce groupe ethnique est déjà en cours d\'examen.'
    return
  }

  emit('created', reponse as PropositionSalleAPI)
  emit('close')
}
</script>
