<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero -->
    <SabbatiqueHero />

    <!-- Breadcrumb -->
    <CommonBreadcrumbNav class="mx-4 md:mx-16 lg:mx-64 pt-6" />

    <!-- Liste des programmes -->
    <section class="px-4 md:px-16 lg:px-64 py-12">
      <h1
        class="text-center text-2xl lg:text-3xl font-extrabold uppercase mb-8"
        data-aos="fade-up"
      >
        Tous les programmes d'échanges d'expériences (sabbatique)
      </h1>

      <!-- Filtres -->
      <SabbatiqueFilters v-model="filtres" />

      <!-- Grille des programmes -->
      <div
        v-if="programmesFiltres.length > 0"
        class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6 justify-items-center"
      >
        <SabbatiqueCard
          v-for="programme in programmesFiltres"
          :key="programme.id"
          :programme="programme"
          data-aos="fade-up"
          data-aos-duration="400"
          @click="voirDetail"
        />
      </div>

      <!-- État vide -->
      <div
        v-else
        class="text-center py-16 text-gray-500"
      >
        <font-awesome-icon
          :icon="['fas', 'search']"
          class="h-12 mb-4 text-gray-300"
        />
        <p class="text-lg">Aucun programme ne correspond à vos critères de recherche.</p>
        <button
          class="mt-4 text-custom-green underline hover:no-underline"
          @click="reinitialiserFiltres"
        >
          Réinitialiser les filtres
        </button>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import AOS from 'aos'
import {
  rechercherSabbatiques,
  type FiltresSabbatique,
  type ProgrammeSabbatique
} from '~/mocks/sabbatiques'

useHead({
  title: 'Tous les programmes d\'échanges - UAfricas',
  meta: [
    {
      name: 'description',
      content: 'Découvrez tous les programmes d\'échanges d\'expériences sabbatiques en Afrique'
    }
  ]
})

const filtres = ref<FiltresSabbatique>({
  type: 'tous',
  pays: '',
  domaine: '',
  recherche: ''
})

const programmesFiltres = computed(() => {
  return rechercherSabbatiques(filtres.value)
})

const voirDetail = (programme: ProgrammeSabbatique) => {
  navigateTo(`/echanges-sabbatiques/${programme.id}`)
}

const reinitialiserFiltres = () => {
  filtres.value = {
    type: 'tous',
    pays: '',
    domaine: '',
    recherche: ''
  }
}

onMounted(() => {
  AOS.init({
    duration: 800,
    easing: 'ease-out-cubic',
    once: true
  })
})
</script>
