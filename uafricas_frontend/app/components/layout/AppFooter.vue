<template>
  <footer class="bg-white text-gray-800">
    <div class="container mx-auto px-6 md:px-12 py-8">
      <div class="flex flex-col md:flex-row gap-8 md:gap-12">

        <!-- Logo -->
        <div class="flex-shrink-0 flex justify-center md:justify-start">
          <NuxtLink to="/">
            <img class="h-24 sm:h-28 lg:h-32" src="/logos/logo_uafracas.png" alt="UAfricas Logo" />
          </NuxtLink>
        </div>

        <!-- A propos -->
        <div class="flex-1 min-w-0">
          <h3 class="text-lg sm:text-xl text-custom-chocolat font-semibold">
            À propos de AfricanS
          </h3>
          <p class="mt-2 text-sm sm:text-base leading-relaxed">
            Africans est un réseau international visant la promotion des meilleures
            valeurs africaines et Afro-descendantes aux niveaux culturel,
            socio-économique et scientifique. Elle regroupe un bureau de
            coordination technique, des antennes nationales et des groupes de
            réflexion ad hoc. Africans poursuit un but non lucratif et met l'accent
            sur le volontariat des membres.
          </p>
          <ul class="list-disc ml-5 mt-2 text-sm space-y-1">
            <li>Promouvoir les valeurs culturelles africaines et Afro-descendantes pertinentes</li>
            <li>Renforcer l'union des populations d'origine africaine</li>
            <li>Favoriser le retour de la diaspora africaine et leur contribution au développement</li>
            <li>Soutenir le développement d'un marché africain et promouvoir la ZLECAF</li>
            <li>Promouvoir l'expertise africaine à tous les niveaux</li>
            <li>Restaurer l'histoire de l'Afrique et ouvrir la voie à des trajectoires de développement originaux</li>
          </ul>

          <div class="flex flex-col sm:flex-row gap-2 sm:gap-6 mt-4 text-sm">
            <a href="mailto:uafricas@gmail.com" class="flex items-center gap-1.5 hover:text-custom-green transition-colors">
              <font-awesome-icon icon="fa-solid fa-envelope" class="text-custom-chocolat" />
              uafricas@gmail.com
            </a>
            <span class="flex items-center gap-1.5">
              <font-awesome-icon icon="fa-solid fa-phone" class="text-custom-chocolat" />
              00 00 00 00 00
            </span>
          </div>
        </div>

        <!-- Contact -->
        <div class="w-full md:w-64 flex-shrink-0">
          <h3 class="text-lg text-custom-chocolat font-semibold">Contactez-nous</h3>
          <textarea
            class="mt-2 w-full border-2 border-custom-chocolat rounded p-2 text-sm resize-none"
            rows="3"
            placeholder="Écrire un commentaire"
          ></textarea>
          <button class="mt-2 w-full sm:w-auto px-4 py-1.5 bg-custom-chocolat text-white rounded text-sm hover:opacity-90 transition-opacity">
            ENVOYER
          </button>
        </div>
      </div>
    </div>

    <!-- Liens -->
    <div class="flex flex-wrap justify-center sm:justify-end gap-2 px-6 pb-2 text-sm sm:text-base font-semibold">
      <NuxtLink to="/" class="uppercase text-custom-green hover:underline">Innovation</NuxtLink>
      <span class="text-custom-chocolat">|</span>
      <NuxtLink to="/" class="uppercase text-custom-green hover:underline">Médiathèque</NuxtLink>
    </div>

    <!-- Copyright -->
    <div class="text-center text-white bg-custom-chocolat py-2 text-sm">
      &copy; Copyright 2025
    </div>
  </footer>
</template>
