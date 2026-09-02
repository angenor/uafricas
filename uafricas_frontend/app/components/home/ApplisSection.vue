<template>
  <section class="w-full bg-white py-16 font-af md:py-24">
    <div class="mx-auto max-w-af-conteneur px-6">
      <header class="flex flex-col items-center gap-3 text-center">
        <p class="text-[16px]/[1.4] text-af-corps">Écosystème digital</p>
        <h2 class="text-[28px]/[1.4] font-bold text-af-chocolat sm:text-[36px]/[1.4] lg:text-[40px]/[1.4]">
          Nos applications
        </h2>
        <p class="max-w-3xl text-[16px]/[1.4] text-af-corps sm:text-[20px]/[1.4]">
          Explorez notre écosystème d'applications pour connecter, développer et valoriser l'Afrique.
        </p>
      </header>

      <!-- Cinq colonnes, comme la maquette : les quinze applications tiennent
           en trois rangées pleines, sans orpheline en bout de grille. -->
      <div class="mt-12 grid grid-cols-2 gap-5 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5">
        <NuxtLink
          v-for="appli in APPLIS"
          :key="appli.label"
          :to="appli.to"
          class="group flex flex-col items-center gap-1 rounded-[10px] border border-af-bordure bg-white p-5 text-center transition hover:border-af-chocolat"
        >
          <font-awesome-icon
            :icon="appli.icone"
            class="mb-2 size-8 text-af-chocolat transition group-hover:scale-110"
          />
          <h3 class="text-[16px]/[1.4] font-bold text-af-encre transition group-hover:text-af-chocolat">
            {{ appli.label }}
          </h3>
          <p class="text-[12px]/[1.4] text-af-atone">{{ appli.description }}</p>
        </NuxtLink>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
/**
 * Les quinze applications de la plateforme. Les couleurs par appli de la
 * version précédente ont disparu : quinze pastilles de quinze teintes ne
 * hiérarchisaient rien, et la maquette n'en montre aucune. Le nom, l'icône et
 * la promesse suffisent à distinguer une carte de sa voisine.
 */
interface Appli {
  label: string
  to: string
  icone: string
  description: string
}

const APPLIS: Appli[] = [
  { label: 'Codimoi', to: '/codi-moi', icone: 'fa-solid fa-comments', description: 'Échanges culturels' },
  { label: 'Afrolang', to: '/afrolang', icone: 'fa-solid fa-language', description: 'Langues africaines' },
  { label: 'Africalive', to: '/evenements/liste', icone: 'fa-solid fa-calendar-days', description: 'Événements en direct' },
  { label: 'Afroculture', to: '/centres', icone: 'fa-solid fa-earth-africa', description: 'Culture afro' },
  { label: 'Afromarket', to: '/marche-africain', icone: 'fa-solid fa-store', description: 'Marché africain' },
  { label: 'Diapertise', to: '/experts', icone: 'fa-solid fa-user-tie', description: "Réseau d'experts" },
  { label: 'Afripulse', to: '/opportunite-afrique', icone: 'fa-solid fa-bolt', description: 'Opportunités' },
  { label: 'Sabbafrica', to: '/echanges-sabbatiques', icone: 'fa-solid fa-plane', description: 'Échanges sabbatiques' },
  { label: 'Librafrica', to: '/bibliotheque/numerique', icone: 'fa-solid fa-book-open', description: 'Bibliothèque numérique' },
  { label: 'Mindshiftlab', to: '/mindshiftlab', icone: 'fa-solid fa-graduation-cap', description: 'Formation & savoir' },
  { label: 'Novagouv', to: '/universite/gouvernance', icone: 'fa-solid fa-landmark', description: 'Gouvernance' },
  { label: 'Africamood', to: '/medias', icone: 'fa-solid fa-tv', description: 'Médias africains' },
  { label: 'Africantives', to: '/africantives', icone: 'fa-solid fa-rocket', description: 'Innovations africaines' },
  { label: 'Africonnect', to: '/retrouve-amis', icone: 'fa-solid fa-users', description: 'Retrouvez vos proches' },
  { label: 'Rootstree', to: '/arbre-genealogique', icone: 'fa-solid fa-tree', description: 'Arbre généalogique' },
]
</script>
