<template>
  <div class="min-h-screen bg-gray-50 py-10">
    <div class="max-w-5xl mx-auto px-4 sm:px-6">
      <header class="mb-8 flex flex-col sm:flex-row sm:items-end sm:justify-between gap-4">
        <div>
          <NuxtLink
            to="/afrolang"
            class="inline-flex items-center gap-1 text-sm text-custom-chocolat hover:underline mb-3"
          >
            <font-awesome-icon icon="arrow-left" class="text-[10px]" />
            Retour à l'annuaire
          </NuxtLink>
          <h1 class="text-3xl sm:text-4xl font-bold text-gray-900">
            Mes propositions de salles
          </h1>
          <p class="text-gray-600 mt-2 max-w-2xl">
            Proposez à l'équipe Afrolang d'ouvrir une salle dédiée à un groupe ethnique
            absent de l'annuaire. Chaque demande est examinée sous 7 jours ouvrés.
          </p>
        </div>
        <button
          type="button"
          class="inline-flex items-center gap-2 px-5 py-3 rounded-xl bg-custom-chocolat text-white font-semibold shadow-md hover:bg-custom-chocolat/90 transition"
          @click="ouvrirModal()"
        >
          <font-awesome-icon icon="plus" />
          Nouvelle proposition
        </button>
      </header>

      <section>
        <div v-if="chargement" class="py-20 text-center text-gray-500">
          <font-awesome-icon icon="spinner" class="animate-spin text-3xl" />
          <p class="mt-3">Chargement…</p>
        </div>

        <div
          v-else-if="propositions.length === 0"
          class="py-20 text-center bg-white rounded-xl border border-dashed border-gray-300"
        >
          <font-awesome-icon icon="folder-open" class="text-4xl text-gray-400" />
          <p class="mt-4 text-gray-700 font-semibold">Vous n'avez pas encore soumis de proposition.</p>
          <p class="text-sm text-gray-500 mt-1">
            Cliquez sur « Nouvelle proposition » pour ouvrir le formulaire.
          </p>
        </div>

        <div v-else class="grid gap-4 sm:grid-cols-2">
          <PropositionCard
            v-for="p in propositions"
            :key="p.id"
            :proposition="p"
          />
        </div>
      </section>
    </div>

    <AfrolangProposerSalleModal
      :ouvert="modalOuvert"
      :nom-prerempli="nomPrerempli"
      @close="modalOuvert = false"
      @created="onCreated"
    />
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import type { PropositionSalleAPI } from '~/composables/useAfrolang'
import { useAfrolang } from '~/composables/useAfrolang'

definePageMeta({
  middleware: 'auth',
})

const { listerMesPropositions } = useAfrolang()

const propositions = ref<PropositionSalleAPI[]>([])
const chargement = ref(false)
const modalOuvert = ref(false)
const nomPrerempli = ref('')

const charger = async () => {
  chargement.value = true
  propositions.value = await listerMesPropositions()
  chargement.value = false
}

const ouvrirModal = (nom = '') => {
  nomPrerempli.value = nom
  modalOuvert.value = true
}

const onCreated = (p: PropositionSalleAPI) => {
  propositions.value = [p, ...propositions.value]
}

onMounted(() => {
  charger()
})
</script>
