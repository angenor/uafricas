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
            <NuxtLink to="/echanges-sabbatiques/proposer?type=interafricain">
              <button
                class="text-white whitespace-nowrap shadow-md hover:shadow-none h-10 rounded-full bg-custom-green px-4 hover:scale-105 transition-all"
              >
                Proposer un projet d'échange
                <font-awesome-icon icon="fa-solid fa-arrow-right" />
              </button>
            </NuxtLink>
            <NuxtLink to="/echanges-sabbatiques/programmes?type=interafricain">
              <button
                class="text-custom-chocolat whitespace-nowrap shadow-md hover:shadow-none h-10 rounded-full border border-custom-chocolat px-4 hover:scale-105 transition-all flex items-center gap-2"
              >
                Voir les programmes publiés
                <span class="bg-custom-chocolat rounded-full text-white px-3">
                  {{ nbInterafricain }}
                </span>
              </button>
            </NuxtLink>
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
            <NuxtLink to="/echanges-sabbatiques/proposer?type=hors_afrique">
              <button
                class="text-white whitespace-nowrap shadow-md hover:shadow-none h-10 rounded-full bg-custom-green px-4 hover:scale-105 transition-all"
              >
                Proposer un projet d'échange
                <font-awesome-icon icon="fa-solid fa-arrow-right" />
              </button>
            </NuxtLink>
            <NuxtLink to="/echanges-sabbatiques/programmes?type=hors_afrique">
              <button
                class="text-custom-chocolat whitespace-nowrap shadow-md hover:shadow-none h-10 rounded-full border border-custom-chocolat px-4 hover:scale-105 transition-all flex items-center gap-2"
              >
                Voir les programmes publiés
                <span class="bg-custom-chocolat rounded-full text-white px-3">
                  {{ nbHorsAfrique }}
                </span>
              </button>
            </NuxtLink>
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

  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import AOS from 'aos'
import { useSabbatiques } from '~/composables/useSabbatiques'

const { listerProgrammes } = useSabbatiques()

useHead({
  title: 'Échanges Sabbatiques - UAfricas',
  meta: [
    {
      name: 'description',
      content: 'Découvrez les programmes d\'échanges sabbatiques pour partager votre expertise en Afrique'
    }
  ]
})

const nbInterafricain = ref(0)
const nbHorsAfrique = ref(0)

onMounted(async () => {
  AOS.init({
    duration: 800,
    easing: 'ease-out-cubic',
    once: true
  })

  // Charger les compteurs depuis l'API
  const [interafricains, horsAfrique] = await Promise.all([
    listerProgrammes({ type: 'interafricain', par_page: 1 }),
    listerProgrammes({ type: 'hors_afrique', par_page: 1 }),
  ])
  if (interafricains) nbInterafricain.value = interafricains.total
  if (horsAfrique) nbHorsAfrique.value = horsAfrique.total
})
</script>
