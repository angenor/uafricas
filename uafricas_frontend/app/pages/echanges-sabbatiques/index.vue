<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section -->
    <SabbatiqueHero />

    <!-- Breadcrumb -->
    <CommonBreadcrumbNav class="mx-4 md:mx-16 lg:mx-64 pt-6" />

    <!-- Section Interafricain -->
    <section
      id="interafricain"
      class="w-full shadow-xl bg-white"
      data-aos="fade-up"
      data-aos-duration="500"
    >
      <div class="flex flex-col lg:flex-row p-8 lg:p-20">
        <div class="flex-1">
          <h2 class="text-3xl lg:text-5xl font-extrabold uppercase text-custom-chocolat">
            Programme d'échange Interafricain
          </h2>
          <p class="py-5 text-gray-700">
            Les Africain(e)s sur le continent ou de la diaspora ainsi que les
            afro-américains peuvent ici mettre leur expertise à contribution en
            faveur du développement local grâce à des projets de mobilité entre
            3 et 12 mois.
          </p>
          <div class="flex flex-wrap gap-4 mt-4">
            <button
              class="text-white whitespace-nowrap shadow-md hover:shadow-none h-10 rounded-full bg-custom-green px-4 hover:scale-105 transition-all"
            >
              Proposer un projet d'échange
              <font-awesome-icon icon="fa-solid fa-arrow-right" />
            </button>
            <a href="#sabbatique">
              <button
                class="text-custom-chocolat whitespace-nowrap shadow-md hover:shadow-none h-10 rounded-full border border-custom-chocolat px-4 hover:scale-105 transition-all flex items-center gap-2"
              >
                Voir les programmes publiés
                <span class="bg-custom-chocolat rounded-full text-white px-3">
                  {{ nbInterafricain }}
                </span>
              </button>
            </a>
          </div>
          <div class="text-center mt-10 lg:mt-20">
            <a href="#hors-afrique">
              <button class="border border-black p-4 rounded-full animate-bounce">
                <font-awesome-icon :icon="['fas', 'arrow-down']" />
              </button>
            </a>
          </div>
        </div>
        <div class="flex-shrink-0 mt-8 lg:mt-0 lg:ml-8">
          <img
            class="max-h-80 lg:max-h-120 rounded-md"
            src="/images/carte-afrique.jpg"
            alt="Carte d'Afrique"
          />
        </div>
      </div>
    </section>

    <!-- Section Hors Afrique -->
    <section
      id="hors-afrique"
      class="w-full bg-people-bg-2 py-16 lg:py-32"
      data-aos="fade-right"
      data-aos-duration="700"
    >
      <div class="flex flex-col lg:flex-row p-8 lg:p-20">
        <div class="flex-1">
          <h2 class="text-3xl lg:text-5xl font-extrabold uppercase text-custom-chocolat">
            Programme d'échange Hors Afrique vers Afrique
          </h2>
          <p class="py-5 text-gray-700">
            Ce programme permet aux experts et professionnels du monde entier de
            partager leurs compétences avec les communautés africaines. Une
            opportunité unique de contribuer au développement du continent tout
            en vivant une expérience culturelle enrichissante.
          </p>
          <div class="flex flex-wrap gap-4 mt-4">
            <button
              class="text-white whitespace-nowrap shadow-md hover:shadow-none h-10 rounded-full bg-custom-green px-4 hover:scale-105 transition-all"
            >
              Proposer un projet d'échange
              <font-awesome-icon icon="fa-solid fa-arrow-right" />
            </button>
            <a href="#sabbatique">
              <button
                class="text-custom-chocolat whitespace-nowrap shadow-md hover:shadow-none h-10 rounded-full border border-custom-chocolat px-4 hover:scale-105 transition-all flex items-center gap-2"
              >
                Voir les programmes publiés
                <span class="bg-custom-chocolat rounded-full text-white px-3">
                  {{ nbHorsAfrique }}
                </span>
              </button>
            </a>
          </div>
        </div>
        <div class="flex-shrink-0 mt-8 lg:mt-0 lg:ml-8">
          <img
            class="rounded-md max-h-80"
            src="/images/zone_libre_echange.jpg"
            alt="Zone libre échange"
          />
        </div>
      </div>
    </section>

    <!-- Liste des programmes -->
    <section id="sabbatique" class="px-4 md:px-16 lg:px-64 py-12">
      <h2
        class="text-center text-2xl lg:text-3xl font-extrabold uppercase mb-8"
        data-aos="fade-up"
      >
        Tous les programmes d'échanges d'expériences (sabbatique)
      </h2>

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
  sabbatiquesMock,
  rechercherSabbatiques,
  getSabbatiquesInterafricains,
  getSabbatiquesHorsAfrique,
  type FiltresSabbatique,
  type ProgrammeSabbatique
} from '~/mocks/sabbatiques'

useHead({
  title: 'Échanges Sabbatiques - UAfricas',
  meta: [
    {
      name: 'description',
      content: 'Découvrez les programmes d\'échanges sabbatiques pour partager votre expertise en Afrique'
    }
  ]
})

const filtres = ref<FiltresSabbatique>({
  type: 'tous',
  pays: '',
  domaine: '',
  recherche: ''
})

const programmes = ref(sabbatiquesMock)

const programmesFiltres = computed(() => {
  return rechercherSabbatiques(filtres.value)
})

const nbInterafricain = computed(() => getSabbatiquesInterafricains().length)
const nbHorsAfrique = computed(() => getSabbatiquesHorsAfrique().length)

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
