<template>
  <!-- Bande pêche au motif tressé : le motif est un actif extrait du Figma au
       lot 1, employé en TEXTURE de fond : d'où le voile pêche par-dessus, sans
       lequel le texte chocolat ne se détacherait pas du dessin. -->
  <section class="relative isolate w-full overflow-hidden bg-af-chocolat/15 py-16 font-af md:py-24">
    <img
      src="/images/africans/motifs/motif-tresse.jpg"
      alt=""
      class="absolute inset-0 -z-10 size-full object-cover opacity-20"
    />

    <div class="mx-auto flex max-w-af-conteneur flex-col items-center gap-10 px-6 lg:flex-row lg:gap-16">
      <div class="flex flex-col items-center gap-5 text-center lg:items-start lg:text-left">
        <h2 class="text-[28px]/[1.4] font-bold text-af-chocolat sm:text-[36px]/[1.4] lg:text-[40px]/[1.4]">
          Mobiliser une expertise
        </h2>
        <p class="max-w-xl text-[16px]/[1.4] text-af-corps sm:text-[20px]/[1.4]">
          Vous êtes ressortissant africain ou africain d'origine ? Retrouvez des milliers
          d'experts prêts à s'engager pour l'Afrique et pour vous.
        </p>
        <AfricansBouton variante="vert" vers="/experts" icone="fa-solid fa-arrow-right">
          Trouver des experts
        </AfricansBouton>
      </div>

      <!-- Les domaines (agriculture, informatique, mécanique, élevage…) sont
           DANS l'illustration : la liste animée qui les répétait à côté disait
           deux fois la même chose. -->
      <img
        src="/images/africans/illustrations/mobiliser-expertise.svg"
        alt="Domaines d'expertise : agriculture, informatique, mécanique, élevage et bien d'autres"
        class="w-full max-w-xl shrink-0 lg:w-[650px]"
        loading="lazy"
      />
    </div>
  </section>
</template>
