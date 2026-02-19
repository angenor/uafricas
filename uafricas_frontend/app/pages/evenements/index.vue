<template>
  <div class="min-h-screen flex flex-col">
    <!-- Hero Section -->
    <div class="relative h-80 bg-font-baniere-ethnie bg-cover bg-center">
      <div class="absolute inset-0 bg-linear-to-r from-custom-chocolat/90 to-black/70"></div>

      <div class="absolute inset-0 flex flex-col items-center justify-center mt-5">
        <h1 class="text-white text-4xl md:text-5xl font-bold mb-4 animate-title">
          Forum
        </h1>
        <div class="h-1 w-24 bg-custom-green rounded animate-line"></div>
        <p class="text-white text-xl md:text-2xl mt-4 animate-subtitle">
          Événements & Rencontres
        </p>
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
            <NuxtLink to="/evenements/codi-moi">
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
  title: 'Forum - Événements | UAfricas'
})

const breadcrumbs = [
  { label: 'Centre Culturel', to: '/africa-culture' },
  { label: 'Promotion des Valeurs', to: '/promotion-valeur' },
  { label: 'Événements', to: null }
]
</script>

<style scoped>
@reference "~/assets/css/main.css";

@keyframes fadeIn {
  from { opacity: 0; transform: translateY(-20px); }
  to { opacity: 1; transform: translateY(0); }
}

@keyframes expandLine {
  from { width: 0; }
  to { width: 6rem; }
}

.animate-title {
  animation: fadeIn 1s ease-out forwards;
}

.animate-subtitle {
  animation: fadeIn 1s ease-out 0.3s forwards;
  opacity: 0;
}

.animate-line {
  animation: expandLine 1.2s ease-out 0.1s forwards;
  width: 0;
}
</style>
