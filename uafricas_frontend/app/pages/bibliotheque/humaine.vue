<template>
  <div class="min-h-screen pb-10 bg-gray-50">
    <!-- Hero Section -->
    <div
      class="relative h-100 bg-cover bg-center pt-10"
      style="background-image: url('/images/culturel_danse.jpg')"
    >
      <div class="absolute inset-0 bg-gradient-to-r from-custom-chocolat/80 to-black/70"></div>

      <div class="absolute inset-0 flex flex-col items-center justify-center">
        <h1 class="text-white text-4xl md:text-5xl font-bold mb-4 animate-title">
          Bibliothèques Humaines - Humaintech
        </h1>
        <div class="h-1 w-24 bg-custom-green rounded animate-line"></div>
        <p class="text-white text-xl md:text-2xl mt-4 animate-subtitle max-w-3xl text-center px-4">
          Découvrez les expériences et connaissances précieuses de nos bibliothèques humaines
        </p>
      </div>
    </div>

    <div class="max-w-7xl mx-auto bg-white min-h-screen rounded-lg shadow-xl relative -mt-10">
      <!-- Bouton d'inscription -->
      <div class="flex justify-center pt-8 font-bold">
        <button
          @click="showRegisterPopup = true"
          class="flex items-center px-6 py-3 shadow-lg mx-auto text-white rounded-full bg-gradient-to-r from-custom-chocolat to-amber-700 transform hover:scale-105 active:scale-95 transition-all duration-300"
        >
          <img
            class="h-14 mr-3 animate-bounce-slow"
            src="https://cdn-icons-png.flaticon.com/512/225/225932.png"
            alt=""
          />
          <div class="text-lg">Devenir une Bibliothèque Humaine</div>
        </button>
      </div>

      <!-- Barre de recherche -->
      <div class="max-w-4xl mx-auto mt-8 px-4">
        <div class="bg-white rounded-xl shadow-xl p-5 transform transition-all hover:shadow-2xl">
          <div class="flex flex-col md:flex-row gap-3">
            <div class="flex-1">
              <input
                v-model="searchQuery"
                type="text"
                class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:outline-hidden focus:ring-2 focus:ring-custom-green transition-all"
                placeholder="Rechercher une bibliothèque humaine..."
              />
            </div>
            <button
              class="bg-gradient-to-r from-custom-green to-green-600 hover:from-green-600 hover:to-custom-green text-white px-6 py-3 rounded-lg transition-all duration-300 transform hover:scale-105 focus:outline-hidden focus:ring-2 focus:ring-custom-green flex items-center justify-center"
            >
              <font-awesome-icon icon="fa-solid fa-search" class="mr-2" />
              Recherche
            </button>
          </div>

          <!-- Filtres -->
          <div class="flex flex-wrap mt-3 gap-2">
            <label
              v-for="type in filterTypes"
              :key="type"
              class="filter-option"
            >
              <input
                type="radio"
                name="filter"
                v-model="selectedFilter"
                :value="type"
                class="hidden"
              />
              <div
                class="px-4 py-2 rounded-full text-sm cursor-pointer transition-all duration-200"
                :class="[
                  selectedFilter === type
                    ? 'bg-custom-chocolat text-white'
                    : 'bg-gray-100 text-gray-600 hover:bg-gray-200',
                ]"
              >
                {{ type }}
              </div>
            </label>
          </div>
        </div>
      </div>

      <!-- Titre de section -->
      <div class="max-w-7xl mx-auto px-4 mt-12 mb-6">
        <div class="flex items-center">
          <div class="h-1 w-12 bg-custom-green rounded-full"></div>
          <h2 class="text-2xl font-bold text-gray-800 mx-4">
            Nos Bibliothèques Humaines
          </h2>
          <div class="h-1 flex-grow bg-custom-green rounded-full"></div>
        </div>
        <p class="text-gray-600 mt-2">
          Explorez notre collection de savoirs et d'expériences humaines
        </p>
      </div>

      <!-- Grille des bibliothèques humaines -->
      <div class="px-6 sm:px-10 pb-10">
        <TransitionGroup
          name="list"
          tag="div"
          class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6"
        >
          <NuxtLink
            v-for="(biblio, index) in filteredBiblios"
            :key="biblio.id"
            :to="'/profil/' + biblio.user_id"
            class="group relative overflow-hidden rounded-xl shadow-lg hover:shadow-2xl transition-all duration-500 transform hover:-translate-y-2"
          >
            <!-- Background image -->
            <div
              :class="[
                index % 3 === 0 ? 'bg-mini-1' : index % 3 === 1 ? 'bg-mini-2' : 'bg-mini-3',
              ]"
              class="h-40 w-full bg-cover bg-center relative"
            >
              <div class="absolute inset-0 bg-gradient-to-t from-black/70 to-black/20 group-hover:opacity-60 transition-opacity duration-500"></div>
            </div>

            <!-- Profile image -->
            <div class="absolute top-4 left-4 z-30">
              <img
                class="w-24 h-24 rounded-full shadow-md border-4 border-white object-cover group-hover:scale-110 transition-all duration-500"
                :src="biblio.photo_url"
                :alt="biblio.prenom + ' ' + biblio.nom"
              />
            </div>

            <!-- Badge de spécialité -->
            <div class="absolute top-4 right-4 z-20">
              <span class="bg-custom-green text-white text-xs px-2 py-1 rounded-full opacity-90 shadow-md">
                {{ biblio.specialite || 'Bibliothèque Humaine' }}
              </span>
            </div>

            <!-- Content -->
            <div class="relative z-20 -mt-8 pt-16 pb-6 px-4 bg-gradient-to-b from-gray-900/95 to-gray-800/95 text-white rounded-b-xl">
              <div class="line-clamp-1 text-xl mb-1">
                <span class="font-bold">{{ biblio.prenom }} </span>
                <span class="font-bold uppercase">{{ biblio.nom }}</span>
              </div>
              <div class="flex items-center line-clamp-1 mb-1">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-green-400 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 13.255A23.931 23.931 0 0112 15c-3.183 0-6.22-.62-9-1.745M16 6V4a2 2 0 00-2-2h-4a2 2 0 00-2 2v2m4 6h.01M5 20h14a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
                </svg>
                <span class="font-medium">{{ biblio.fonction || 'Expert' }}</span>
              </div>
              <div class="flex items-center line-clamp-1 mb-3">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-red-400 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
                </svg>
                <span class="font-medium">{{ biblio.pays || 'International' }}</span>
              </div>

              <!-- Biographie avec animation -->
              <div class="line-clamp-2 text-sm text-gray-300 italic opacity-0 group-hover:opacity-100 transition-all duration-500 transform translate-y-2 group-hover:translate-y-0">
                "{{ biblio.biographie ? biblio.biographie.substring(0, 100) + '...' : 'Découvrez mon profil pour en savoir plus...' }}"
              </div>

              <!-- Bouton Voir plus -->
              <div class="mt-4 opacity-0 group-hover:opacity-100 transition-all duration-500">
                <div class="bg-white/20 text-white text-center py-1 rounded-md backdrop-blur-xs hover:bg-white/30 transition-all cursor-pointer">
                  Voir le profil
                </div>
              </div>
            </div>
          </NuxtLink>
        </TransitionGroup>

        <!-- État vide -->
        <div v-if="filteredBiblios.length === 0" class="flex flex-col items-center justify-center py-16 text-center">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-20 w-20 text-gray-300 mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
          </svg>
          <p class="text-gray-600 text-xl font-medium mb-2">
            Aucune bibliothèque humaine trouvée
          </p>
          <p class="text-gray-500 max-w-md">
            Soyez le premier à partager vos connaissances et votre expérience.
          </p>
          <button
            @click="showRegisterPopup = true"
            class="mt-6 px-6 py-3 bg-gradient-to-r from-custom-green to-green-600 text-white rounded-lg hover:from-green-600 hover:to-custom-green transition-all duration-300 transform hover:scale-105"
          >
            S'inscrire maintenant
          </button>
        </div>
      </div>
    </div>
  </div>

  <!-- Popup Inscription (simplifié pour mock) -->
  <Transition name="modal-fade">
    <div v-if="showRegisterPopup" class="z-50 fixed inset-0 flex items-center justify-center">
      <div @click="showRegisterPopup = false" class="absolute inset-0 bg-black/60 backdrop-blur-xs"></div>
      <div class="relative w-full max-w-md mx-4 bg-white rounded-xl shadow-2xl p-6">
        <div class="flex justify-center mb-4">
          <img
            class="h-16 w-16 object-contain"
            src="https://cdn-icons-png.flaticon.com/512/225/225932.png"
            alt=""
          />
        </div>
        <h2 class="text-xl font-bold text-center mb-4">Devenir une Bibliothèque Humaine</h2>
        <p class="text-gray-600 text-center mb-6">
          Cette fonctionnalité nécessite une connexion au backend.
          Pour l'instant, vous pouvez explorer les profils de démonstration.
        </p>
        <button
          @click="showRegisterPopup = false"
          class="w-full py-2 bg-custom-green text-white rounded-lg hover:bg-green-700 transition-colors"
        >
          Compris
        </button>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { biblioHumaines, filterTypes } from '~/mocks/bibliotheques'

useHead({
  title: 'Bibliothèques Humaines - UAfricas',
  meta: [
    { name: 'description', content: 'Découvrez les personnes-livres et partagez des histoires vivantes' },
  ],
})

useAOS()

const searchQuery = ref('')
const selectedFilter = ref('Tous')
const showRegisterPopup = ref(false)

const filteredBiblios = computed(() => {
  let biblios = biblioHumaines

  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    biblios = biblios.filter(b =>
      b.prenom.toLowerCase().includes(query) ||
      b.nom.toLowerCase().includes(query) ||
      b.specialite?.toLowerCase().includes(query) ||
      b.fonction?.toLowerCase().includes(query)
    )
  }

  if (selectedFilter.value && selectedFilter.value !== 'Tous') {
    biblios = biblios.filter(b => b.specialite === selectedFilter.value)
  }

  return biblios
})
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

@keyframes bounce-slow {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-6px); }
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

.animate-bounce-slow {
  animation: bounce-slow 2s infinite;
}

.list-enter-active,
.list-leave-active {
  transition: all 0.5s ease;
}

.list-enter-from,
.list-leave-to {
  opacity: 0;
  transform: translateY(30px);
}

.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: opacity 0.3s ease;
}

.modal-fade-enter-from,
.modal-fade-leave-to {
  opacity: 0;
}

/* Background images */
.bg-mini-1 {
  background-image: url('/images/bg-mini-1.jpg');
}

.bg-mini-2 {
  background-image: url('/images/bg-mini-2.jpg');
}

.bg-mini-3 {
  background-image: url('/images/bg-mini-3.jpg');
}
</style>
