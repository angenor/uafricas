<template>
  <div class="min-h-screen pb-10 bg-gray-50">
    <!-- Hero Section (compact, titre ↔ description au survol) -->
    <div
      class="group relative bg-cover bg-center z-0"
      style="background-image: url('https://images.unsplash.com/photo-1507842217343-583bb7270b66?ixlib=rb-1.2.1&auto=format&fit=crop&w=1900&q=80')"
    >
      <div class="absolute inset-0 bg-gradient-to-r from-custom-chocolat/90 to-black/70"></div>

      <div class="relative max-w-4xl mx-auto px-4 pt-16 pb-6 text-center select-none">
        <!-- Conteneur fixe : le titre et la description se superposent (crossfade au survol) -->
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12">
          <h1 class="absolute inset-0 flex items-center justify-center text-white text-2xl md:text-4xl font-bold transition-opacity duration-300 group-hover:opacity-0">
            Librafrica
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Explorez nos bibliothèques numériques et humaines africaines
          </p>
        </div>
      </div>
    </div>

    <!-- Liste des 2 catégories de bibliothèque -->
    <div class="max-w-6xl mx-auto px-4 relative pt-6">
      <div class="grid md:grid-cols-2 gap-8">
        <!-- Bibliothèque Numérique -->
        <NuxtLink to="/bibliotheque/numerique">
          <div class="group cursor-pointer transform transition-all duration-500 hover:scale-105">
            <div class="bg-white rounded-2xl shadow-lg overflow-hidden hover:shadow-2xl transition-all duration-300">
              <div class="relative overflow-hidden">
                <img
                  class="w-full h-64 object-cover transition-transform duration-500 group-hover:scale-110"
                  src="https://www.hailo-shop.fr/img/cms/BLOG/am%C3%A9nager%20une%20biblioth%C3%A8que/am%C3%A9nager-une-biblioth%C3%A8que.jpg"
                  alt="bibliotheque-numerique"
                />
                <div class="absolute inset-0 bg-gradient-to-t from-black/50 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                <div class="absolute top-4 right-4">
                  <div class="bg-custom-green text-white px-3 py-1 rounded-full text-sm font-medium">
                    Numérique
                  </div>
                </div>
              </div>
              <div class="p-6">
                <h3 class="text-2xl font-bold text-gray-800 mb-1 group-hover:text-custom-green transition-colors">
                  Bibliothèque Numérique
                </h3>
                <p class="text-gray-600 mb-2">
                  Accédez à des milliers de livres, articles et documents africains. Votre patrimoine culturel à portée de clic.
                </p>
                <div class="text-gray-600 mb-4 flex flex-wrap">
                  <div class="border border-custom-green px-2 m-1 text-custom-green rounded-full">E-books</div>
                  <div class="border border-custom-green px-2 m-1 text-custom-green rounded-full">Documents</div>
                </div>
                <div class="flex items-center justify-between">
                  <span class="text-custom-chocolat font-semibold">Explorer →</span>
                </div>
              </div>
            </div>
          </div>
        </NuxtLink>

        <!-- Bibliothèques Humaines -->
        <NuxtLink to="/bibliotheque/humaine">
          <div class="group cursor-pointer transform transition-all duration-500 hover:scale-105">
            <div class="bg-white rounded-2xl shadow-lg overflow-hidden hover:shadow-2xl transition-all duration-300">
              <div class="relative overflow-hidden">
                <img
                  class="w-full h-64 object-cover transition-transform duration-500 group-hover:scale-110"
                  src="https://miro.medium.com/v2/resize:fit:1000/0*iwIZ8kUVYXsghOSI.jpg"
                  alt="bibliotheque-humaine"
                />
                <div class="absolute inset-0 bg-gradient-to-t from-black/50 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                <div class="absolute top-4 right-4">
                  <div class="bg-custom-chocolat text-white px-3 py-1 rounded-full text-sm font-medium">
                    Humaine
                  </div>
                </div>
              </div>

              <div class="p-6">
                <h3 class="text-2xl font-bold text-gray-800 mb-1 group-hover:text-custom-green transition-colors">
                  Bibliothèques Humaines
                </h3>
                <p class="text-gray-600 mb-2">
                  Rencontrez des personnes-livres et partagez des histoires vivantes. L'expérience humaine au cœur du savoir.
                </p>
                <div class="text-gray-600 mb-4 flex flex-wrap">
                  <div class="border border-custom-green px-2 m-1 text-custom-green rounded-full">Témoignages</div>
                  <div class="border border-custom-green px-2 m-1 text-custom-green rounded-full">Partages</div>
                </div>
                <div class="flex items-center justify-between">
                  <span class="text-custom-chocolat font-semibold">Explorer →</span>
                </div>
              </div>
            </div>
          </div>
        </NuxtLink>
      </div>

      <!-- Section statistiques -->
      <div class="mt-16 bg-gradient-to-r from-custom-green to-custom-chocolat rounded-2xl p-8 text-white">
        <div class="grid grid-cols-2 md:grid-cols-4 gap-6 text-center">
          <div>
            <div class="text-3xl font-bold">10K+</div>
            <div class="text-sm opacity-90">Documents Disponibles</div>
          </div>
          <div>
            <div class="text-3xl font-bold">500+</div>
            <div class="text-sm opacity-90">Personnes-Livres</div>
          </div>
          <div>
            <div class="text-3xl font-bold">54</div>
            <div class="text-sm opacity-90">Territoires Représentés</div>
          </div>
          <div>
            <div class="text-3xl font-bold">100%</div>
            <div class="text-sm opacity-90">Contenu Africain</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
useHead({
  title: 'Librafrica - UAfricas',
  meta: [
    { name: 'description', content: 'Explorez nos bibliothèques numériques et humaines africaines' },
  ],
})

useAOS()
</script>
