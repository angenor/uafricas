<template>
  <div class="min-h-screen flex flex-col">
    <!-- Hero Section (compact, titre ↔ description au survol) -->
    <div class="group relative">
      <div class="absolute inset-0 bg-linear-to-r from-custom-chocolat to-black/90"></div>

      <div class="relative max-w-4xl mx-auto px-4 pt-16 pb-6 text-center select-none">
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12">
          <h1 class="absolute inset-0 flex items-center justify-center text-white text-2xl md:text-4xl font-bold transition-opacity duration-300 group-hover:opacity-0">
            Forum
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Événements & Rencontres
          </p>
        </div>
      </div>
    </div>

    <!-- Breadcrumb -->
    <div class="backdrop-blur-xs">
      <div class="mx-auto px-4 py-3">
        <CommonBreadcrumbNav :custom-breadcrumbs="breadcrumbs" />
      </div>
    </div>

    <!-- Contenu principal - 3 sections -->
    <div class="flex-1 flex flex-col lg:flex-row">
      <!-- Section 1: Codi-Moi -->
      <div class="flex-1 p-4 border-b lg:border-b-0 lg:border-r border-gray-200 overflow-hidden flex flex-col">
        <div class="text-2xl font-bold uppercase text-custom-chocolat mb-2">
          Codi-Moi
        </div>
        <div class="flex flex-wrap gap-1 mb-2">
          <span class="inline-flex items-center text-xs text-black font-bold">
            <span class="w-2 h-2 bg-black rounded-full mr-1"></span>Education
          </span>
          <span class="inline-flex items-center text-xs text-black font-bold">
            <span class="w-2 h-2 bg-black rounded-full mr-1"></span>Santé
          </span>
          <span class="inline-flex items-center text-xs text-black font-bold">
            <span class="w-2 h-2 bg-black rounded-full mr-1"></span>Agriculture
          </span>
        </div>
        <div class="flex-1 relative overflow-hidden rounded-md min-h-64">
          <img
            class="absolute inset-0 w-full h-full object-cover"
            src="https://www.agoraafricaine.info/wp-content/uploads/2022/05/valeurs-africaines.jpg"
            alt="Codification"
          />
          <div class="absolute inset-0 bg-black/30 flex items-end p-3">
            <NuxtLink to="/codi-moi">
              <button class="text-white text-sm whitespace-nowrap rounded-full bg-custom-green px-3 py-1 hover:scale-105 transition-all">
                Découvrir
                <font-awesome-icon icon="fa-solid fa-arrow-right" class="ml-1" />
              </button>
            </NuxtLink>
          </div>
        </div>
      </div>

      <!-- Section 2: Afrolang -->
      <div class="flex-1 p-4 border-b lg:border-b-0 lg:border-r border-gray-200 overflow-hidden flex flex-col">
        <div class="text-2xl font-bold uppercase text-custom-chocolat mb-2">
          Afrolang
        </div>
        <div class="text-sm mb-2">
          Découvrez les langues et cultures africaines à travers notre programme linguistique innovant.
        </div>
        <div class="flex-1 relative overflow-hidden rounded-md min-h-64">
          <img
            class="absolute inset-0 w-full h-full object-cover"
            src="https://www.fratmat.info/media/k2/items/cache/c9e4f768b9037f2637ff258302bd739d_XL.jpg"
            alt="Afrolang"
          />
          <div class="absolute inset-0 bg-black/30 flex items-end p-3">
            <NuxtLink to="/afrolang">
              <button class="text-white text-sm whitespace-nowrap rounded-full bg-custom-green px-3 py-1 hover:scale-105 transition-all">
                Découvrir
                <font-awesome-icon icon="fa-solid fa-arrow-right" class="ml-1" />
              </button>
            </NuxtLink>
          </div>
        </div>
      </div>

      <!-- Section 3: Événements & Ateliers -->
      <div class="flex-1 p-4 overflow-hidden flex flex-col">
        <div class="text-2xl font-bold uppercase text-custom-chocolat mb-2">
          Événements & Ateliers
        </div>
        <div class="text-sm mb-2">
          Participez à nos événements culturels, ateliers et rencontres pour célébrer la richesse africaine.
        </div>
        <div class="flex-1 relative overflow-hidden rounded-md min-h-64">
          <img
            class="absolute inset-0 w-full h-full object-cover"
            src="https://www.learnthings.fr/wp-content/uploads/2024/06/atelier-en-formation-768x578.jpg.webp"
            alt="Événements"
          />
          <div class="absolute inset-0 bg-black/30 flex items-end p-3">
            <NuxtLink to="/evenements/liste">
              <button class="text-white text-sm whitespace-nowrap rounded-full bg-custom-green px-3 py-1 hover:scale-105 transition-all">
                Découvrir
                <font-awesome-icon icon="fa-solid fa-arrow-right" class="ml-1" />
              </button>
            </NuxtLink>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
useHead({
  title: 'Forum - Événements | AfricanS'
})

const breadcrumbs = [
  { label: 'Événements', to: undefined }
]
</script>
