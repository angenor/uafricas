<template>
  <div class="group relative overflow-hidden bg-gradient-to-r from-emerald-600 via-teal-600 to-cyan-600">
    <div class="absolute inset-0 bg-black opacity-10" />
    <div class="absolute inset-0 bg-gradient-to-t from-black/30 to-transparent" />

    <!-- Motif geometrique moderne -->
    <div class="absolute inset-0 opacity-10">
      <div class="absolute -top-24 -right-24 w-96 h-96 bg-white rounded-full blur-3xl" />
      <div class="absolute -bottom-24 -left-24 w-96 h-96 bg-white rounded-full blur-3xl" />
    </div>

    <div class="relative max-w-7xl mx-auto px-4 pt-16 pb-6">
      <div class="text-center space-y-6">
        <!-- Titre ↔ sous-titre en crossfade au survol -->
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12 select-none">
          <h1 class="absolute inset-0 flex items-center justify-center text-2xl md:text-4xl font-extrabold text-white tracking-tight transition-opacity duration-300 group-hover:opacity-0">
            Mobiliser une expertise de pointe
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-sm md:text-base text-white/90 max-w-3xl mx-auto px-2 font-light opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Connectez-vous avec les meilleurs experts africains et afro-descendants
          </p>
        </div>
        <p class="text-white/70 text-sm md:text-base mt-3 max-w-3xl mx-auto text-center px-4">
          Valoriser en un seul endroit l'expertise africaine et afro-descendante et encourager les entreprises en Afrique à recourir aux ressources pertinentes sur le continent et au-delà.
        </p>

        <!-- Search Bar integree dans le hero -->
        <div class="max-w-4xl mx-auto mt-10">
          <div class="bg-white/10 backdrop-blur-md rounded-2xl p-2 shadow-2xl">
            <form class="flex flex-col md:flex-row gap-2" @submit.prevent="$emit('search')">
              <div class="relative flex-1">
                <input
                  v-model="localSearchTerm"
                  type="search"
                  class="w-full px-6 py-4 bg-white rounded-xl text-gray-900 placeholder-gray-500 focus:outline-hidden focus:ring-4 focus:ring-white/20 transition-all"
                  placeholder="Rechercher un expert, une competence..."
                />
                <svg
                  class="absolute right-4 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                  />
                </svg>
              </div>
              <select
                v-model="localCategorySelected"
                class="px-6 py-4 bg-white rounded-xl text-gray-900 focus:outline-hidden focus:ring-4 focus:ring-white/20 transition-all cursor-pointer"
              >
                <option v-for="category in categories" :key="category" :value="category">
                  {{ category }}
                </option>
              </select>
              <button
                type="submit"
                class="px-8 py-4 bg-gradient-to-r from-emerald-600 to-teal-600 text-white rounded-xl font-semibold hover:shadow-lg transform hover:scale-[1.02] transition-all"
              >
                Rechercher
              </button>
            </form>
          </div>
        </div>

        <!-- Stats rapides -->
        <div class="grid grid-cols-3 gap-4 max-w-2xl mx-auto mt-12">
          <div class="text-center">
            <div class="text-3xl font-bold text-white">{{ totalExperts }}+</div>
            <div class="text-sm text-white/70">Experts</div>
          </div>
          <div class="text-center border-x border-white/20">
            <div class="text-3xl font-bold text-white">15+</div>
            <div class="text-sm text-white/70">Domaines</div>
          </div>
          <div class="text-center">
            <div class="text-3xl font-bold text-white">25+</div>
            <div class="text-sm text-white/70">Territoires</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  totalExperts: number
  searchTerm: string
  categorySelected: string
  categories: string[]
}>()

const emit = defineEmits<{
  'update:searchTerm': [value: string]
  'update:categorySelected': [value: string]
  search: []
}>()

// Local refs with two-way binding
const localSearchTerm = computed({
  get: () => props.searchTerm,
  set: (value: string) => emit('update:searchTerm', value),
})

const localCategorySelected = computed({
  get: () => props.categorySelected,
  set: (value: string) => emit('update:categorySelected', value),
})
</script>
