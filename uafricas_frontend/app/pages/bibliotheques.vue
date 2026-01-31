<template>
  <div class="min-h-screen pb-10 bg-gray-50">
    <!-- Hero Section -->
    <div
      class="relative h-80 bg-cover bg-center z-0"
      style="background-image: url('https://images.unsplash.com/photo-1507842217343-583bb7270b66?ixlib=rb-1.2.1&auto=format&fit=crop&w=1900&q=80')"
    >
      <div class="absolute inset-0 bg-gradient-to-r from-custom-chocolat/90 to-black/70"></div>

      <!-- Texte avec animation d'entrée -->
      <div class="absolute inset-0 flex flex-col items-center justify-center mt-5">
        <h1 class="text-white text-4xl md:text-5xl font-bold mb-4 animate-title">
          Bibliothèques
        </h1>
        <div class="h-1 w-24 bg-custom-green rounded animate-line"></div>
        <p class="text-white text-xl md:text-2xl mt-4 animate-subtitle">
          Explorez nos bibliothèques numériques et humaines africaines
        </p>
      </div>
    </div>

    <!-- Liste des 2 catégories de bibliothèque -->
    <div class="max-w-6xl mx-auto px-4 relative -top-10">
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
            <div class="text-sm opacity-90">Pays Représentés</div>
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
  title: 'Bibliothèques - UAfricas',
  meta: [
    { name: 'description', content: 'Explorez nos bibliothèques numériques et humaines africaines' },
  ],
})

useAOS()
</script>

<style scoped>
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
