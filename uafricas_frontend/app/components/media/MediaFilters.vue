<script setup lang="ts">
interface Props {
  types: string[]
  countries: string[]
  genres: string[]
  selectedType: string
  selectedCountry: string
  selectedGenre: string
}

const props = defineProps<Props>()

const emit = defineEmits<{
  (e: 'selectType', type: string): void
  (e: 'selectCountry', country: string): void
  (e: 'selectGenre', genre: string): void
  (e: 'reset'): void
}>()

const activeFilter = ref<'type' | 'country' | 'genre'>('type')
</script>

<template>
  <div class="max-w-6xl mx-auto mt-10">
    <!-- Onglets des filtres -->
    <div class="flex">
      <button
        @click="activeFilter = 'type'"
        :class="[
          'px-6 py-3 text-sm font-medium rounded-t-xl transition-all duration-200 focus:outline-hidden',
          activeFilter === 'type'
            ? 'bg-black/60 text-yellow-400 border-t-2 border-yellow-400'
            : 'bg-gray-900/60 text-gray-400 hover:text-white'
        ]"
      >
        <div class="flex items-center">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M3 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd" />
          </svg>
          Type
        </div>
      </button>
      <button
        @click="activeFilter = 'country'"
        :class="[
          'px-6 py-3 text-sm font-medium rounded-t-xl transition-all duration-200 focus:outline-hidden',
          activeFilter === 'country'
            ? 'bg-black/60 text-yellow-400 border-t-2 border-yellow-400'
            : 'bg-gray-900/60 text-gray-400 hover:text-white'
        ]"
      >
        <div class="flex items-center">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M3 6a3 3 0 013-3h10a1 1 0 01.8 1.6L14.25 8l2.55 3.4A1 1 0 0116 13H6a1 1 0 00-1 1v3a1 1 0 11-2 0V6z" clip-rule="evenodd" />
          </svg>
          Territoire
        </div>
      </button>
      <button
        @click="activeFilter = 'genre'"
        :class="[
          'px-6 py-3 text-sm font-medium rounded-t-xl transition-all duration-200 focus:outline-hidden',
          activeFilter === 'genre'
            ? 'bg-black/60 text-yellow-400 border-t-2 border-yellow-400'
            : 'bg-gray-900/60 text-gray-400 hover:text-white'
        ]"
      >
        <div class="flex items-center">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
            <path d="M18 3a1 1 0 00-1.196-.98l-10 2A1 1 0 006 5v9.114A4.369 4.369 0 005 14c-1.657 0-3 .895-3 2s1.343 2 3 2 3-.895 3-2V7.82l8-1.6v5.894A4.37 4.37 0 0015 12c-1.657 0-3 .895-3 2s1.343 2 3 2 3-.895 3-2V3z" />
          </svg>
          Genre
        </div>
      </button>
    </div>

    <!-- Contenu des filtres -->
    <div class="bg-black/60 backdrop-blur-md rounded-b-xl rounded-tr-xl p-4 shadow-lg border border-gray-800">
      <!-- Filtre par type de programme -->
      <div v-if="activeFilter === 'type'">
        <h2 class="text-xl font-bold text-yellow-400 mb-4 flex items-center">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M3 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd" />
          </svg>
          Filtrer par type de programme
        </h2>

        <div class="flex flex-wrap gap-2">
          <button
            v-for="type in types"
            :key="type"
            @click="emit('selectType', type)"
            :class="[
              'px-4 py-2 rounded-full text-sm font-medium transition-all duration-200',
              selectedType === type
                ? 'bg-yellow-500 text-black shadow-lg'
                : 'bg-gray-800 text-gray-300 hover:bg-gray-700'
            ]"
          >
            {{ type }}
          </button>
        </div>
      </div>

      <!-- Filtre par pays -->
      <div v-if="activeFilter === 'country'">
        <h2 class="text-xl font-bold text-yellow-400 mb-4 flex items-center">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M3 3a1 1 0 011-1h12a1 1 0 011 1v3a1 1 0 01-.293.707L12 11.414V15a1 1 0 01-.293.707l-2 2A1 1 0 018 17v-5.586L3.293 6.707A1 1 0 013 6V3z" clip-rule="evenodd" />
          </svg>
          Filtrer par territoire
        </h2>

        <div class="flex flex-wrap gap-2">
          <button
            @click="emit('selectCountry', 'Tous les territoires')"
            :class="[
              'px-4 py-2 rounded-full text-sm font-medium transition-all duration-200',
              selectedCountry === 'Tous les territoires'
                ? 'bg-yellow-500 text-black shadow-lg'
                : 'bg-gray-800 text-gray-300 hover:bg-gray-700'
            ]"
          >
            Tous les territoires
          </button>
          <button
            v-for="country in countries"
            :key="country"
            @click="emit('selectCountry', country)"
            :class="[
              'px-4 py-2 rounded-full text-sm font-medium transition-all duration-200',
              selectedCountry === country
                ? 'bg-yellow-500 text-black shadow-lg'
                : 'bg-gray-800 text-gray-300 hover:bg-gray-700'
            ]"
          >
            {{ country }}
          </button>
        </div>
      </div>

      <!-- Filtre par genre -->
      <div v-if="activeFilter === 'genre'">
        <h2 class="text-xl font-bold text-yellow-400 mb-4 flex items-center">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
            <path d="M18 3a1 1 0 00-1.196-.98l-10 2A1 1 0 006 5v9.114A4.369 4.369 0 005 14c-1.657 0-3 .895-3 2s1.343 2 3 2 3-.895 3-2V7.82l8-1.6v5.894A4.37 4.37 0 0015 12c-1.657 0-3 .895-3 2s1.343 2 3 2 3-.895 3-2V3z" />
          </svg>
          Filtrer par genre
        </h2>

        <div class="flex flex-wrap gap-2">
          <button
            @click="emit('selectGenre', 'Tous les genres')"
            :class="[
              'px-4 py-2 rounded-full text-sm font-medium transition-all duration-200',
              selectedGenre === 'Tous les genres'
                ? 'bg-yellow-500 text-black shadow-lg'
                : 'bg-gray-800 text-gray-300 hover:bg-gray-700'
            ]"
          >
            Tous les genres
          </button>
          <button
            v-for="genre in genres"
            :key="genre"
            @click="emit('selectGenre', genre)"
            :class="[
              'px-4 py-2 rounded-full text-sm font-medium transition-all duration-200',
              selectedGenre === genre
                ? 'bg-yellow-500 text-black shadow-lg'
                : 'bg-gray-800 text-gray-300 hover:bg-gray-700'
            ]"
          >
            {{ genre }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
