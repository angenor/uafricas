<template>
  <!-- Bandeau d'accueil du site public. Le titre est CENTRÉ, comme sur la
       maquette : il était calé à droite, ce qui le faisait chevaucher les
       visages de la photographie selon la largeur de l'écran. -->
  <section class="relative isolate flex h-svh w-full items-center justify-center overflow-hidden bg-af-encre font-af">
    <img
      src="/images/africans/heros/hero-accueil.jpg"
      alt=""
      class="absolute inset-0 -z-10 size-full object-cover"
    />
    <!-- Voile : le titre est blanc, et la moitié basse de la photographie est
         claire. Sans lui, deux lignes sur trois deviennent illisibles. -->
    <div class="absolute inset-0 -z-10 bg-linear-to-b from-black/30 via-black/45 to-black/70" />

    <div class="mx-auto flex max-w-af-conteneur flex-col items-center gap-6 px-6 text-center">
      <!-- Cette page PRÉSENTE la plateforme ; le bouton y fait entrer. Le fil
           d'actualité est la racine : c'est là que la plateforme démarre. -->
      <AfricansBouton vers="/" icone="fa-solid fa-arrow-right">
        Entrer dans la communauté
      </AfricansBouton>

      <h1 class="text-[32px]/[1.4] font-bold text-white sm:text-[40px]/[1.4] lg:text-[48px]/[1.4]">
        Africains, afrodescendants,<br />
        Nation et développement soutenable
        <span class="mt-1 block text-[40px]/[1.2] text-af-chocolat sm:text-[56px]/[1.2] lg:text-[64px]/[1.2]">
          AFRICANS
        </span>
      </h1>
    </div>

    <a
      href="#histoire"
      aria-label="Défiler vers la suite"
      class="absolute bottom-8 left-1/2 grid size-12 -translate-x-1/2 animate-bounce place-items-center rounded-full bg-white/80 text-af-chocolat transition hover:bg-white"
    >
      <font-awesome-icon icon="fa-solid fa-arrow-down" class="text-xl" />
    </a>
  </section>
</template>
