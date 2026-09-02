<template>
  <div class="flex min-h-svh bg-af-fond font-af">
    <!-- Colonne illustrée, à partir de lg seulement. En dessous, la photo
         disparaît au lieu de passer en fond : un formulaire posé sur une image
         est plus difficile à lire qu'un formulaire sur fond uni, et c'est lui
         qu'on vient remplir. -->
    <div class="relative hidden flex-1 lg:block">
      <img :src="image" alt="" class="absolute inset-0 size-full object-cover" />
      <div class="absolute inset-0 bg-black/30" />
    </div>

    <div class="flex w-full items-center justify-center px-6 py-12 lg:w-[580px] lg:shrink-0">
      <div class="w-full max-w-md overflow-hidden rounded-[10px] border border-af-bordure bg-white">
        <!-- Filet de marque de 17 px, puis en-tête BLANC : c'est l'idiome de
             `Modale`, et il est ici une nécessité. Le logo est chocolat et vert
             sur transparent : il n'existe aucune variante claire, et posé sur
             le dégradé, son mot-symbole se fondait dans le fond. Sur blanc, il
             a le contraste qu'il a partout ailleurs sur le site. -->
        <div class="h-[17px] bg-af-degrade" />

        <header class="flex flex-col items-center gap-2 px-8 pt-8 text-center">
          <!-- Le logo ramène à l'accueil : c'est la seule sortie de ces pages,
               qui n'ont ni navigation ni pied. -->
          <NuxtLink to="/" class="transition hover:opacity-80">
            <img src="/logos/logo_uafracas.png" alt="AfricanS" class="h-14 w-auto" />
          </NuxtLink>
          <h1 class="text-[20px]/[1.4] font-bold text-af-encre">{{ titre }}</h1>
          <p v-if="sousTitre" class="text-[14px]/[1.4] text-af-atone">{{ sousTitre }}</p>
        </header>

        <div class="flex flex-col gap-6 p-8">
          <slot />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
/**
 * Cadre commun aux pages d'authentification.
 *
 * La maquette ne dessine AUCUN écran de connexion ni d'inscription, seul le
 * bouton « Se Connecter » de la barre supérieure y figure. Ce cadre est donc
 * construit avec les éléments du système : jetons `af-*`, dégradé de marque,
 * rayon de 10 px, bordure `af-bordure`, police Inter.
 *
 * Il vit dans un composant et non dans `layouts/auth.vue` : ce layout sert
 * aussi les deux pages de vérification d'e-mail, qui n'ont pas de formulaire
 * et n'ont rien à faire dans un écran scindé.
 */
withDefaults(defineProps<{
  titre: string
  sousTitre?: string
  image?: string
}>(), { image: '/images/font_login.jpg' })
</script>
