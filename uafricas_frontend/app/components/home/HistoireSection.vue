<template>
  <section id="histoire" class="w-full bg-af-fond py-16 font-af md:py-24">
    <!-- Le tiret chocolat de la maquette : 39 x 4 px, centré. C'est le seul
         séparateur de section du dessin, il ne porte aucun texte. -->
    <div class="mx-auto mb-12 h-1 w-10 rounded-full bg-af-chocolat" />

    <div class="mx-auto flex max-w-af-conteneur flex-col items-center gap-10 px-6 lg:flex-row lg:gap-16">
      <div class="flex flex-col items-center gap-5 text-center lg:items-start lg:text-left">
        <h2 class="text-[28px]/[1.4] font-bold text-af-chocolat sm:text-[36px]/[1.4] lg:text-[40px]/[1.4]">
          Connaissez-vous l'histoire de l'Afrique ?
        </h2>

        <p class="max-w-2xl text-[16px]/[1.4] text-af-corps sm:text-[20px]/[1.4]">
          Découvrez toute la richesse culturelle de l'Afrique, les territoires, cultures,
          les ethnies, les contes et histoires ainsi que le lien entre chaque culture.
        </p>

        <ul class="flex flex-wrap justify-center gap-x-8 gap-y-3 lg:justify-start">
          <li
            v-for="volet in VOLETS"
            :key="volet"
            class="flex items-center gap-2 text-[16px]/[1.4] font-bold text-af-encre"
          >
            <font-awesome-icon icon="fa-solid fa-seedling" class="text-af-vert" />
            {{ volet }}
          </li>
        </ul>

        <AfricansBouton variante="vert" vers="/africa-culture" icone="fa-solid fa-arrow-right" class="mt-2">
          Découvrir
        </AfricansBouton>
      </div>

      <!-- Illustration extraite du Figma. Elle remplace `flutte_inde.png`, dont
           le NOM DE FICHIER disait l'Inde sur une section consacrée à l'Afrique
           (constat #3 de l'audit). -->
      <img
        src="/images/africans/illustrations/histoire-afrique.svg"
        alt=""
        class="w-full max-w-lg shrink-0 lg:w-[596px]"
        loading="lazy"
      />
    </div>
  </section>
</template>

<script setup lang="ts">
const VOLETS = ['Histoires', 'Ethnies', 'Coutumes', 'Alliances']
</script>
