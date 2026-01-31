<template>
  <div id="qui" class="py-10 w-[80] flex space-x-10 px-32 bg-red-50">
    <div data-aos="fade-right" data-aos-duration="1000" class="w-8/12">
      <div class="text-custom-green text-5xl text-center">
        <span class="font-bold">Q</span>ui Somme Nous ?
      </div>
      <div class="mt-2 px-5 text-justify">
        <div>
          Nous sommes un réseau international qui réunit toutes les africain ( e
          )s et afro-africain ( e)s convaincus par les valeurs positives
          africaines dans les domaines culturel, socio- économique et
          scientifique. Ce réseau engage les africains à restaurer leur
          confiance en une histoire africaine, des valeurs culturelles
          pertinentes et une développement durable africain. Il s'appuie sur la
          promotion de l'union des populations africaines, l'expertise de la
          diaspora et des bonnes pratiques et savoir-faire durables du
          continent.
        </div>
        <div class="mt-3">
          Africans comprends un Bureau international qui est l'équipe dirigeante
          à l'international. Le réseau comprend également dans chaque pays un
          Comité national relais dont la composition du bureau est désignée par
          vote et reconnu par l'équipe dirigeante d'Africans.
        </div>
        <div class="mt-3">
          Le Bureau international met en place des Centres culturels Africain et
          afro-américain dans tous les pays en vue de valoriser les valeurs
          culturel, socio-économique et scientifique de l'Afrique. Le Bureau
          s'appui sur les Comités nationaux relais pour développer une
          programmation et faciliter la réalisation des activités liées au (i)
          programme d'échange sabbatique, (ii) promotion de la ZLECAF, (iii)
          développement du tourisme du tourisme durable en Afrique, (iv) à la
          vulgarisation du Projet ' Communautés unies d'Afrique au-delà des
          frontières territoriales'.
        </div>
        <div class="mt-3">
          Africans s'appuie par ailleurs sur des Groupes de réflexion Ad'hoc
          pour faire avancer des sujets d'intérêt africain, notamment : l'union
          africaine, l'éducation, la santé, l'approche de développement, valeurs
          culturelles et développement, diaspora et contribution au
          développement, intelligence artificielle et société, réseaux sociaux
          et sociétés modernes africaines, etc.
        </div>
      </div>
    </div>

    <div data-aos="fade-left" class="w-52 text-center">
      <img
        class="h-52 w-52 object-cover rounded-full"
        :src="responsable.photo"
        :alt="responsable.nom"
      />
      <div class="font-serif">{{ responsable.nom }}</div>
      <div class="text-sm">
        {{ responsable.role }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { responsable } from '~/mocks/homepage'

useAOS()
</script>
