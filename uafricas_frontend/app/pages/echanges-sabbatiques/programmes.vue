<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section (compact, titre ↔ description au survol) -->
    <div class="group relative bg-cover bg-center">
      <div class="absolute inset-0 bg-gradient-to-r from-custom-chocolat to-black/90"></div>

      <div class="relative max-w-4xl mx-auto px-4 pt-16 pb-6 text-center select-none">
        <!-- Conteneur fixe : le titre et la description se superposent (crossfade au survol) -->
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12">
          <h1 class="absolute inset-0 flex items-center justify-center text-white text-2xl md:text-4xl font-bold transition-opacity duration-300 group-hover:opacity-0">
            Offrir son expertise en volontariat et bénévolat
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Tous les programmes d'échanges d'expériences sabbatiques.
          </p>
        </div>
      </div>
    </div>

    <!-- Breadcrumb -->
    <CommonBreadcrumbNav
      class="mx-4 md:mx-16 lg:mx-64 pt-6"
      :custom-breadcrumbs="[
        { label: 'Échanges Sabbatiques', to: '/echanges-sabbatiques' },
        { label: 'Programmes' },
      ]"
    />

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

      <!-- Chargement -->
      <div
        v-if="chargement"
        class="text-center py-16"
      >
        <font-awesome-icon
          :icon="['fas', 'spinner']"
          class="h-8 text-custom-green animate-spin"
        />
        <p class="mt-4 text-gray-500">Chargement des programmes...</p>
      </div>

      <!-- Grille des programmes -->
      <div
        v-else-if="programmesFiltres.length > 0"
        class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6 justify-items-center"
      >
        <SabbatiqueCard
          v-for="programme in programmesFiltres"
          :key="programme.id"
          :programme="programme"
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
import { ref, watch, onMounted } from 'vue'
import AOS from 'aos'
import {
  useSabbatiques,
  type SabbatiqueAPI,
  type SabbatiqueFiltres,
  type TypeProgramme,
} from '~/composables/useSabbatiques'

const route = useRoute()
const { listerProgrammes, chargement } = useSabbatiques()

const typesValides: Array<'tous' | TypeProgramme> = ['tous', 'interafricain', 'hors_afrique']
const typeParam = route.query.type as string | undefined
const typeInitial = typesValides.includes(typeParam as any) ? (typeParam as 'tous' | TypeProgramme) : 'tous'

useHead({
  title: 'Tous les programmes d\'échanges - UAfricas',
  meta: [
    {
      name: 'description',
      content: 'Découvrez tous les programmes d\'échanges d\'expériences sabbatiques en Afrique'
    }
  ]
})

const filtres = ref<SabbatiqueFiltres>({
  type: typeInitial,
  pays: '',
  domaine: '',
  recherche: ''
})

const programmesFiltres = ref<SabbatiqueAPI[]>([])

const chargerProgrammes = async () => {
  const result = await listerProgrammes(filtres.value)
  if (result) {
    programmesFiltres.value = result.programmes
  }
}

const voirDetail = (programme: SabbatiqueAPI) => {
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

watch(filtres, () => {
  chargerProgrammes()
}, { deep: true })

onMounted(async () => {
  AOS.init({
    duration: 800,
    easing: 'ease-out-cubic',
    once: true
  })
  await chargerProgrammes()
})
</script>
