<template>
  <section class="relative overflow-hidden bg-white">
    <!-- Bandeau image de couverture -->
    <div class="relative h-48 sm:h-56 md:h-64 overflow-hidden">
      <div class="absolute inset-0 bg-font-baniere-histoire bg-cover bg-center" />
      <div class="absolute inset-0 bg-linear-to-b from-black/30 via-transparent to-white" />
      <div class="absolute inset-0 flex items-center justify-center" data-aos="zoom-in" data-aos-duration="800">
        <div class="text-center">
          <div class="inline-flex items-center gap-3 bg-white/20 backdrop-blur-md rounded-full px-6 py-2.5 border border-white/30">
            <font-awesome-icon icon="fa-solid fa-table-cells-large" class="text-white text-lg" />
            <span class="text-white font-display text-sm sm:text-base uppercase tracking-widest font-semibold">Ecosysteme digital</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Contenu principal -->
    <div class="relative -mt-10 z-10 pb-16 md:pb-24">
      <div class="container mx-auto px-6 md:px-12 lg:px-20">
        <!-- Titre -->
        <div class="text-center mb-10 md:mb-14" data-aos="fade-up" data-aos-duration="800">
          <h2 class="text-custom-chocolat text-3xl sm:text-4xl lg:text-5xl font-display">
            <span class="font-bold">N</span>os <span class="font-bold">A</span>pplications
          </h2>
          <div class="w-20 h-1 bg-custom-green mx-auto mt-4 rounded-full" />
          <p class="text-gray-500 mt-4 text-lg max-w-2xl mx-auto">
            Explorez notre ecosysteme d'applications pour connecter, developper et valoriser l'Afrique.
          </p>
        </div>

        <!-- Grille d'applis -->
        <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4 md:gap-5">
          <NuxtLink
            v-for="(appli, index) in applis"
            :key="appli.label"
            :to="appli.to"
            class="group block"
            data-aos="fade-up"
            :data-aos-delay="index * 50"
            data-aos-duration="500"
          >
            <div class="relative bg-white rounded-2xl border border-gray-100 p-5 text-center transition-all duration-300 hover:shadow-lg hover:shadow-gray-200/60 hover:-translate-y-1.5 hover:border-custom-green/30">
              <!-- Barre de couleur en haut -->
              <div
                class="absolute top-0 left-1/2 -translate-x-1/2 h-1 w-0 group-hover:w-full rounded-t-2xl transition-all duration-300"
                :class="appli.barClass"
              />

              <!-- Icone -->
              <div
                class="w-12 h-12 mx-auto rounded-xl flex items-center justify-center mb-3 transition-all duration-300 group-hover:scale-110 group-hover:rounded-2xl"
                :class="appli.bgClass"
              >
                <font-awesome-icon :icon="appli.icon" class="text-xl text-white" />
              </div>

              <!-- Nom -->
              <h3 class="font-display text-sm md:text-base font-bold text-gray-800 mb-1 transition-colors duration-300 group-hover:text-custom-chocolat">
                {{ appli.label }}
              </h3>

              <!-- Description -->
              <p class="text-gray-400 text-xs leading-tight hidden sm:block">
                {{ appli.description }}
              </p>

              <!-- Fleche hover -->
              <div class="mt-2 opacity-0 group-hover:opacity-100 transition-all duration-300">
                <font-awesome-icon icon="fa-solid fa-arrow-right" class="text-custom-green text-xs" />
              </div>
            </div>
          </NuxtLink>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
useAOS()

interface Appli {
  label: string
  to: string
  icon: string
  bgClass: string
  barClass: string
  description: string
}

const applis: Appli[] = [
  {
    label: 'Codimoi',
    to: '/evenements/codi-moi',
    icon: 'fa-solid fa-comments',
    bgClass: 'bg-blue-500',
    barClass: 'bg-blue-500',
    description: 'Echanges culturels',
  },
  {
    label: 'Afrolang',
    to: '/afrolang',
    icon: 'fa-solid fa-language',
    bgClass: 'bg-purple-500',
    barClass: 'bg-purple-500',
    description: 'Langues africaines',
  },
  {
    label: 'Africalive',
    to: '/evenements/liste',
    icon: 'fa-solid fa-calendar-days',
    bgClass: 'bg-rose-500',
    barClass: 'bg-rose-500',
    description: 'Evenements en direct',
  },
  {
    label: 'Afroculture',
    to: '/africain-afro-americain',
    icon: 'fa-solid fa-earth-africa',
    bgClass: 'bg-amber-500',
    barClass: 'bg-amber-500',
    description: 'Culture afro',
  },
  {
    label: 'Afromarket',
    to: '/marche-africain',
    icon: 'fa-solid fa-store',
    bgClass: 'bg-emerald-500',
    barClass: 'bg-emerald-500',
    description: 'Marche africain',
  },
  {
    label: 'Diapertise',
    to: '/experts',
    icon: 'fa-solid fa-user-tie',
    bgClass: 'bg-indigo-500',
    barClass: 'bg-indigo-500',
    description: 'Reseau d\'experts',
  },
  {
    label: 'Afripulse',
    to: '/opportunite-afrique',
    icon: 'fa-solid fa-bolt',
    bgClass: 'bg-orange-500',
    barClass: 'bg-orange-500',
    description: 'Opportunites',
  },
  {
    label: 'Sabbafrica',
    to: '/echanges-sabbatiques',
    icon: 'fa-solid fa-plane',
    bgClass: 'bg-cyan-500',
    barClass: 'bg-cyan-500',
    description: 'Echanges sabbatiques',
  },
  {
    label: 'Librafrica',
    to: '/bibliotheque/numerique',
    icon: 'fa-solid fa-book-open',
    bgClass: 'bg-teal-500',
    barClass: 'bg-teal-500',
    description: 'Bibliotheque numerique',
  },
  {
    label: 'Mindshiftlab',
    to: '/universite/inuda',
    icon: 'fa-solid fa-graduation-cap',
    bgClass: 'bg-violet-500',
    barClass: 'bg-violet-500',
    description: 'Universite INUDA',
  },
  {
    label: 'Novagouv',
    to: '/universite/gouvernance',
    icon: 'fa-solid fa-landmark',
    bgClass: 'bg-sky-500',
    barClass: 'bg-sky-500',
    description: 'Gouvernance',
  },
  {
    label: 'Africamood',
    to: '/medias',
    icon: 'fa-solid fa-tv',
    bgClass: 'bg-red-500',
    barClass: 'bg-red-500',
    description: 'Medias africains',
  },
  {
    label: 'Africantives',
    to: '/africantives',
    icon: 'fa-solid fa-rocket',
    bgClass: 'bg-lime-600',
    barClass: 'bg-lime-600',
    description: 'Innovations africaines',
  },
  {
    label: 'Africonnect',
    to: '/retrouve-amis',
    icon: 'fa-solid fa-users',
    bgClass: 'bg-amber-700',
    barClass: 'bg-amber-700',
    description: 'Retrouvez vos proches',
  },
  {
    label: 'Rootstree',
    to: '/arbre-genealogique',
    icon: 'fa-solid fa-tree',
    bgClass: 'bg-[var(--color-custom-green)]',
    barClass: 'bg-[var(--color-custom-green)]',
    description: 'Arbre généalogique',
  },
]
</script>
