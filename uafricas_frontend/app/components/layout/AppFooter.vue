<template>
  <footer>
    <div class="flex justify-center space-x-14 py-8 bg-white rounded-3xl">
      <NuxtLink to="/">
        <img class="h-32 mx-auto my-1" src="/logos/logo_uafracas.png" alt="UAfricas Logo" />
      </NuxtLink>

      <div class="w-100">
        <div class="text-xl text-custom-chocolat text-justify">
          A propos de AfricanS
        </div>
        <div>
          Africans est un réseau international visant la promotion des meilleures
          valeurs africaines et afro-américaines aux niveaux culturel,
          socio-économique et scientifique. Elle regroupe un bureau de
          coordination technique, des antennes nationales et des groupes de
          réflexion ad hoc. Africans poursuit un but non lucratif et met l'accent
          sur le volontariat des membres. Africans poursuit les objectifs
          spécifiques suivants :
        </div>
        <ul class="list-disc ml-10">
          <li>
            Promouvoir les valeurs culturelles africaines et afro-américaines
            pertinentes
          </li>
          <li>Renforcer l'union des populations d'origine africaine</li>
          <li>
            Favoriser le retour de la diaspora africaine et leur contribution au
            développement
          </li>
          <li>
            Soutenir le développement d'un marché africain et promouvoir la ZLECAF
          </li>
          <li>
            Appuyer des projets de développement local grâce à des approches
            participatives et ingénieuses
          </li>
          <li>Promouvoir l'expertise africaine à tous les niveaux</li>
          <li>
            Restaurer l'histoire de l'Afrique et ouvrir la voie à la définition de
            trajectoires de développements originaux à tous les niveaux.
          </li>
        </ul>

        <div class="mt-5">
          <font-awesome-icon icon="fa-solid fa-envelope" class="text-custom-chocolat mr-1" />
          uafricas@gmail.com
        </div>
        <div>
          <font-awesome-icon icon="fa-solid fa-phone" class="text-custom-chocolat mr-1" />
          00 00 00 00 00
        </div>
      </div>

      <div>
        <div class="text-custom-chocolat">Contactez-nous</div>
        <textarea
          class="my-2 border-2 border-custom-chocolat"
          cols="30"
          rows="3"
          placeholder="Ecrire un commentaire"
        ></textarea>
        <br />
        <button class="px-2 bg-custom-chocolat text-white">ENVOYER</button>
      </div>
    </div>

    <div class="flex justify-end mr-5 text-custom-chocolat text-lg font-semibold bg-white">
      <div class="text-custom-green">
        <NuxtLink class="uppercase" to="/">Innovation</NuxtLink>
        <span class="text-custom-chocolat mx-2">|</span>
        <NuxtLink class="uppercase" to="/">Médiathèque</NuxtLink>
      </div>
    </div>

    <div class="text-center text-white bg-custom-chocolat py-1">
      &copy;Copyright 2025
    </div>
  </footer>
</template>

<script setup lang="ts">
// Footer component - pas de logique nécessaire
</script>
