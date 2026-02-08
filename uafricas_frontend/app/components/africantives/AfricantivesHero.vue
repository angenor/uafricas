<template>
  <div class="relative bg-gradient-to-r from-amber-600 via-orange-600 to-red-600 py-16 lg:py-24">
    <!-- Pattern overlay -->
    <div class="absolute inset-0 opacity-10">
      <div class="absolute inset-0" style="background-image: url('data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 80 80%22><path d=%22M40 0L80 40L40 80L0 40Z%22 fill=%22none%22 stroke=%22white%22 stroke-width=%221%22/></svg>'); background-size: 60px 60px;"></div>
    </div>

    <div class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <!-- Titre -->
      <div class="text-center mb-10">
        <h1 class="text-4xl md:text-5xl lg:text-6xl font-bold text-white mb-4">
          African<span class="text-yellow-300">tives</span>
        </h1>
        <p class="text-lg md:text-xl text-orange-100 max-w-2xl mx-auto">
          Découvrez les initiatives africaines qui transforment le continent. Partagez la vôtre !
        </p>
      </div>

      <!-- Barre de recherche -->
      <div class="max-w-4xl mx-auto">
        <div class="bg-white rounded-2xl shadow-2xl p-2 flex flex-col md:flex-row gap-2">
          <!-- Dropdown domaine -->
          <div class="relative">
            <button
              @click="showDomaines = !showDomaines"
              class="flex items-center justify-between w-full md:w-48 px-4 py-3 bg-gray-50 rounded-xl hover:bg-gray-100 transition-colors"
            >
              <span class="text-gray-700 font-medium truncate">{{ selectedDomaineLabel }}</span>
              <font-awesome-icon
                :icon="['fas', 'chevron-down']"
                class="w-3 h-3 text-gray-400 ml-2 transition-transform"
                :class="{ 'rotate-180': showDomaines }"
              />
            </button>

            <!-- Menu deroulant domaines -->
            <Transition
              enter-active-class="transition ease-out duration-200"
              enter-from-class="opacity-0 translate-y-1"
              enter-to-class="opacity-100 translate-y-0"
              leave-active-class="transition ease-in duration-150"
              leave-from-class="opacity-100 translate-y-0"
              leave-to-class="opacity-0 translate-y-1"
            >
              <div
                v-if="showDomaines"
                class="absolute z-20 left-0 mt-2 w-56 bg-white rounded-xl shadow-lg py-2 border border-gray-100 max-h-64 overflow-y-auto"
              >
                <button
                  v-for="dom in domaines"
                  :key="dom.value"
                  @click="selectDomaine(dom.value)"
                  class="w-full px-4 py-2.5 text-left text-gray-700 hover:bg-orange-50 hover:text-orange-700 transition-colors flex items-center justify-between"
                  :class="{ 'bg-orange-50 text-orange-700': modelDomaine === dom.value }"
                >
                  <span>{{ dom.label }}</span>
                  <font-awesome-icon
                    v-if="modelDomaine === dom.value"
                    :icon="['fas', 'check']"
                    class="w-4 h-4 text-orange-600"
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
              placeholder="Rechercher une initiative..."
              class="w-full pl-11 pr-4 py-3 rounded-xl border-0 focus:ring-2 focus:ring-orange-500 bg-gray-50 placeholder-gray-400"
            />
          </div>

          <!-- Bouton publier -->
          <button
            @click="$emit('publish')"
            class="flex items-center justify-center gap-2 px-6 py-3 bg-gradient-to-r from-custom-chocolat to-amber-700 text-white font-semibold rounded-xl hover:from-custom-chocolat/90 hover:to-amber-800 transition-all shadow-lg hover:shadow-xl"
          >
            <font-awesome-icon :icon="['fas', 'plus']" class="w-4 h-4" />
            <span class="whitespace-nowrap">Publier une initiative</span>
          </button>
        </div>
      </div>

      <!-- Stats -->
      <div class="mt-10 flex flex-wrap justify-center gap-8 text-center">
        <div class="text-white">
          <div class="text-3xl font-bold">{{ totalAfricantives }}</div>
          <div class="text-orange-200 text-sm">Initiatives</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { DOMAINES_AFRICANTIVES } from '~/composables/useAfricantives'

const props = defineProps<{
  modelDomaine: string
  modelRecherche: string
  totalAfricantives: number
}>()

const emit = defineEmits<{
  'update:modelDomaine': [value: string]
  'update:modelRecherche': [value: string]
  'search': []
  'publish': []
}>()

const showDomaines = ref(false)
const domaines = DOMAINES_AFRICANTIVES

const selectedDomaineLabel = computed(() => {
  const dom = domaines.find(d => d.value === props.modelDomaine)
  return dom?.label || 'Tous les domaines'
})

const selectDomaine = (value: string) => {
  emit('update:modelDomaine', value)
  showDomaines.value = false
}
</script>
