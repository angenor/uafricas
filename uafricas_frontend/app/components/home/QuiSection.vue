<template>
  <section id="qui" class="py-12 md:py-16 lg:py-20 w-full bg-red-50">
    <div class="container mx-auto px-6 md:px-12 lg:px-20 flex flex-col md:flex-row gap-8 md:gap-12">

      <!-- Texte principal -->
      <div data-aos="fade-right" data-aos-duration="1000" class="w-full md:w-8/12">
        <h2 class="text-custom-green text-3xl sm:text-4xl lg:text-5xl text-center md:text-left">
          <span class="font-bold">Q</span>ui sommes-nous ?
        </h2>
        <div class="mt-4 text-justify text-sm sm:text-base space-y-3 leading-relaxed">
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
            afro-américains dans tous les pays en vue de valoriser les valeurs
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
      <div data-aos="fade-left" class="w-full md:w-4/12 flex flex-col items-center justify-start pt-4">
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
</template>

<script setup lang="ts">
import { responsable } from '~/mocks/homepage'

useAOS()
</script>
