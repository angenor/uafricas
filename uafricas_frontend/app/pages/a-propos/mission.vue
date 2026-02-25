<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero Section -->
    <div class="relative bg-gradient-to-br from-custom-green via-emerald-600 to-teal-600 overflow-hidden">
      <div class="absolute inset-0 opacity-10">
        <div class="absolute inset-0" style="background-image: url('data:image/svg+xml,%3Csvg width=&quot;60&quot; height=&quot;60&quot; viewBox=&quot;0 0 60 60&quot; xmlns=&quot;http://www.w3.org/2000/svg&quot;%3E%3Cg fill=&quot;none&quot; fill-rule=&quot;evenodd&quot;%3E%3Cg fill=&quot;%23ffffff&quot; fill-opacity=&quot;0.4&quot;%3E%3Cpath d=&quot;M36 34v-4h-2v4h-4v2h4v4h2v-4h4v-2h-4zm0-30V0h-2v4h-4v2h4v4h2V6h4V4h-4zM6 34v-4H4v4H0v2h4v4h2v-4h4v-2H6zM6 4V0H4v4H0v2h4v4h2V6h4V4H6z&quot;/%3E%3C/g%3E%3C/g%3E%3C/svg%3E');" />
      </div>

      <div class="relative max-w-7xl mx-auto px-4 pt-24 pb-16 md:pt-32 md:pb-20">
        <div class="text-center text-white">
          <div class="inline-flex items-center gap-2 bg-white/20 backdrop-blur-xs px-4 py-2 rounded-full text-sm font-medium mb-6">
            <font-awesome-icon :icon="['fas', 'rocket']" class="w-4 h-4" />
            Notre engagement
          </div>
          <h1 class="text-3xl md:text-5xl font-bold mb-4 font-display">
            Notre Mission
          </h1>
          <p class="text-lg md:text-xl text-white/90 max-w-2xl mx-auto">
            Promouvoir les meilleures valeurs africaines et afro-descendantes pour un développement durable du continent.
          </p>
        </div>
      </div>

      <div class="absolute bottom-0 left-0 right-0">
        <svg class="w-full h-12 md:h-16" viewBox="0 0 1440 54" fill="none" xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="none">
          <path d="M0 22L60 16.7C120 11 240 1 360 0.7C480 1 600 11 720 16.7C840 22 960 22 1080 16.7C1200 11 1320 1 1380 0.7L1440 0V54H1380C1320 54 1200 54 1080 54C960 54 840 54 720 54C600 54 480 54 360 54C240 54 120 54 60 54H0V22Z" fill="#f9fafb"/>
        </svg>
      </div>
    </div>

    <!-- Breadcrumb -->
    <div class="bg-gray-50">
      <div class="max-w-7xl mx-auto px-4 py-4">
        <CommonBreadcrumbNav />
      </div>
    </div>

    <!-- Qui sommes-nous -->
    <section class="max-w-7xl mx-auto px-4 md:px-8 py-12">
      <div class="flex flex-col md:flex-row gap-10">
        <!-- Texte principal -->
        <div class="w-full md:w-8/12">
          <h2 class="text-custom-green text-2xl sm:text-3xl lg:text-4xl font-display mb-6">
            <span class="font-bold">Q</span>ui sommes-nous ?
          </h2>
          <div class="text-justify text-sm sm:text-base space-y-4 leading-relaxed text-gray-700">
            <p>
              Nous sommes un réseau international qui réunit tous les africain(e)s
              et afro-descendant(e)s convaincus par les valeurs positives
              africaines dans les domaines culturel, socio-économique et
              scientifique. Ce réseau engage les africains à restaurer leur
              confiance en une histoire africaine, des valeurs culturelles
              pertinentes et un développement durable africain. Il s'appuie sur la
              promotion de l'union des populations africaines, l'expertise de la
              diaspora et des bonnes pratiques et savoir-faire durables du
              continent.
            </p>
            <p>
              Africans comprend un Bureau international qui est l'équipe dirigeante
              à l'international. Le réseau comprend également dans chaque pays un
              Comité national relais dont la composition du bureau est désignée par
              vote et reconnue par l'équipe dirigeante d'Africans.
            </p>
            <p>
              Le Bureau international met en place des Centres culturels africains et
              Afro-descendants dans tous les pays en vue de valoriser les valeurs
              culturelles, socio-économiques et scientifiques de l'Afrique. Le Bureau
              s'appuie sur les Comités nationaux relais pour développer une
              programmation et faciliter la réalisation des activités liées au (i)
              programme d'échange sabbatique, (ii) promotion de la ZLECAF, (iii)
              développement du tourisme durable en Afrique, (iv) à la
              vulgarisation du Projet « Communautés unies d'Afrique au-delà des
              frontières territoriales ».
            </p>
            <p>
              Africans s'appuie par ailleurs sur des Groupes de réflexion ad hoc
              pour faire avancer des sujets d'intérêt africain, notamment : l'union
              africaine, l'éducation, la santé, l'approche de développement, valeurs
              culturelles et développement, diaspora et contribution au
              développement, intelligence artificielle et société, réseaux sociaux
              et sociétés modernes africaines, etc.
            </p>
          </div>
        </div>

        <!-- Responsable -->
        <div class="w-full md:w-4/12 flex flex-col items-center justify-start pt-4">
          <img
            class="h-36 w-36 sm:h-44 sm:w-44 lg:h-52 lg:w-52 object-cover rounded-full shadow-lg"
            :src="responsable.photo"
            :alt="responsable.nom"
            loading="lazy"
          />
          <p class="font-serif font-semibold mt-3 text-center">{{ responsable.nom }}</p>
          <p class="text-sm text-gray-600 text-center">{{ responsable.role }}</p>
        </div>
      </div>
    </section>

    <!-- Nos Objectifs -->
    <section class="bg-white py-12">
      <div class="max-w-7xl mx-auto px-4 md:px-8">
        <h2 class="text-custom-chocolat text-2xl sm:text-3xl font-display mb-8 text-center">
          <span class="font-bold">N</span>os Objectifs
        </h2>
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
          <div
            v-for="(objectif, index) in objectifs"
            :key="index"
            class="flex items-start gap-4 p-5 bg-gray-50 rounded-xl border border-gray-100"
          >
            <div class="w-10 h-10 rounded-full flex items-center justify-center flex-shrink-0"
              :class="index % 2 === 0 ? 'bg-custom-green/10 text-custom-green' : 'bg-custom-chocolat/10 text-custom-chocolat'"
            >
              <font-awesome-icon :icon="objectif.icone" class="w-5 h-5" />
            </div>
            <p class="text-sm text-gray-700 leading-relaxed">{{ objectif.texte }}</p>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { responsable } from '~/mocks/homepage'

useHead({
  title: 'Notre Mission - UAfricas',
  meta: [
    { name: 'description', content: 'Découvrez la mission et les objectifs de UAfricas - United Africa for Sustainable Development.' }
  ]
})

useAOS()

const objectifs = [
  {
    icone: 'fa-solid fa-masks-theater',
    texte: 'Promouvoir les valeurs culturelles africaines et Afro-descendantes pertinentes'
  },
  {
    icone: 'fa-solid fa-users',
    texte: "Renforcer l'union des populations d'origine africaine"
  },
  {
    icone: 'fa-solid fa-plane-arrival',
    texte: 'Favoriser le retour de la diaspora africaine et leur contribution au développement'
  },
  {
    icone: 'fa-solid fa-store',
    texte: "Soutenir le développement d'un marché africain et promouvoir la ZLECAF"
  },
  {
    icone: 'fa-solid fa-graduation-cap',
    texte: "Promouvoir l'expertise africaine à tous les niveaux"
  },
  {
    icone: 'fa-solid fa-earth-africa',
    texte: "Restaurer l'histoire de l'Afrique et ouvrir la voie à des trajectoires de développement originaux"
  }
]
</script>
