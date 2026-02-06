<template>
  <div class="relative bg-gradient-to-r from-emerald-600 via-green-600 to-teal-600 py-16 lg:py-24">
    <!-- Pattern overlay -->
    <div class="absolute inset-0 opacity-10">
      <div class="absolute inset-0" style="background-image: url('data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 80 80%22><circle cx=%2240%22 cy=%2240%22 r=%222%22 fill=%22white%22/></svg>'); background-size: 40px 40px;"></div>
    </div>

    <div class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <!-- Titre -->
      <div class="text-center mb-10">
        <h1 class="text-4xl md:text-5xl lg:text-6xl font-bold text-white mb-4">
          Marché <span class="text-yellow-300">Africain</span>
        </h1>
        <p class="text-lg md:text-xl text-emerald-100 max-w-2xl mx-auto">
          Découvrez des trésors à travers toute l'Afrique. Achetez, vendez, échangez.
        </p>
      </div>

      <!-- Barre de recherche -->
      <div class="max-w-4xl mx-auto">
        <div class="bg-white rounded-2xl shadow-2xl p-2 flex flex-col md:flex-row gap-2">
          <!-- Dropdown catégorie -->
          <div class="relative group">
            <button
              @click="showCategories = !showCategories"
              class="flex items-center justify-between w-full md:w-48 px-4 py-3 bg-gray-50 rounded-xl hover:bg-gray-100 transition-colors"
            >
              <span class="text-gray-700 font-medium truncate">{{ selectedCategoryLabel }}</span>
              <font-awesome-icon
                :icon="['fas', 'chevron-down']"
                class="w-3 h-3 text-gray-400 ml-2 transition-transform"
                :class="{ 'rotate-180': showCategories }"
              />
            </button>

            <!-- Menu déroulant -->
            <Transition
              enter-active-class="transition ease-out duration-200"
              enter-from-class="opacity-0 translate-y-1"
              enter-to-class="opacity-100 translate-y-0"
              leave-active-class="transition ease-in duration-150"
              leave-from-class="opacity-100 translate-y-0"
              leave-to-class="opacity-0 translate-y-1"
            >
              <div
                v-if="showCategories"
                class="absolute z-20 left-0 mt-2 w-56 bg-white rounded-xl shadow-lg py-2 border border-gray-100"
              >
                <button
                  v-for="cat in categories"
                  :key="cat.key"
                  @click="selectCategory(cat.key)"
                  class="w-full px-4 py-2.5 text-left text-gray-700 hover:bg-emerald-50 hover:text-emerald-700 transition-colors flex items-center justify-between"
                  :class="{ 'bg-emerald-50 text-emerald-700': modelCategorie === cat.key }"
                >
                  <span>{{ cat.label }}</span>
                  <font-awesome-icon
                    v-if="modelCategorie === cat.key"
                    :icon="['fas', 'check']"
                    class="w-4 h-4 text-emerald-600"
                  />
                </button>
              </div>
            </Transition>
          </div>

          <!-- Input recherche -->
          <div class="flex-1 relative">
            <font-awesome-icon
              :icon="['fas', 'magnifying-glass']"
              class="absolute left-4 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400"
            />
            <input
              type="text"
              :value="modelRecherche"
              @input="$emit('update:modelRecherche', ($event.target as HTMLInputElement).value)"
              @keyup.enter="$emit('search')"
              placeholder="Rechercher une annonce..."
              class="w-full pl-11 pr-4 py-3 rounded-xl border-0 focus:ring-2 focus:ring-emerald-500 bg-gray-50 placeholder-gray-400"
            />
          </div>

          <!-- Bouton publier -->
          <button
            @click="$emit('publish')"
            class="flex items-center justify-center gap-2 px-6 py-3 bg-gradient-to-r from-orange-500 to-amber-500 text-white font-semibold rounded-xl hover:from-orange-600 hover:to-amber-600 transition-all shadow-lg hover:shadow-xl"
          >
            <font-awesome-icon :icon="['fas', 'plus']" class="w-4 h-4" />
            <span class="whitespace-nowrap">Publier une annonce</span>
          </button>
        </div>
      </div>

      <!-- Stats -->
      <div class="mt-10 flex flex-wrap justify-center gap-8 text-center">
        <div class="text-white">
          <div class="text-3xl font-bold">{{ totalAnnonces }}</div>
          <div class="text-emerald-200 text-sm">Annonces</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { CATEGORIES, type Categorie } from '~/composables/useMarcheAfricain'

const props = defineProps<{
  modelCategorie: Categorie | 'Tout'
  modelRecherche: string
  totalAnnonces: number
}>()

const emit = defineEmits<{
  'update:modelCategorie': [value: Categorie | 'Tout']
  'update:modelRecherche': [value: string]
  'search': []
  'publish': []
}>()

const showCategories = ref(false)
const categories = CATEGORIES

const selectedCategoryLabel = computed(() => {
  const cat = categories.find(c => c.key === props.modelCategorie)
  return cat?.label || 'Toutes les catégories'
})

const selectCategory = (key: Categorie | 'Tout') => {
  emit('update:modelCategorie', key)
  showCategories.value = false
}
</script>
