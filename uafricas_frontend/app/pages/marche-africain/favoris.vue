<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero (compact, titre ↔ description au survol) -->
    <div
      class="group relative bg-cover bg-center"
      style="background-image: url('https://images.unsplash.com/photo-1555217851-6141535bd771?ixlib=rb-1.2.1&auto=format&fit=crop&w=1900&q=80')"
    >
      <div class="absolute inset-0 bg-linear-to-r from-custom-chocolat/90 to-black/70"></div>
      <div class="relative max-w-4xl mx-auto px-4 pt-16 pb-6 text-center select-none">
        <div class="relative flex items-center justify-center min-h-10 md:min-h-12">
          <h1 class="absolute inset-0 flex items-center justify-center text-white text-2xl md:text-4xl font-bold transition-opacity duration-300 group-hover:opacity-0">
            Mes favoris
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Les annonces du Marché Africain que vous avez sauvegardées.
          </p>
        </div>
      </div>
    </div>

    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-10">
      <CommonBreadcrumbNav
        class="mb-6"
        :custom-breadcrumbs="[
          { label: 'Marché Africain', to: '/marche-africain' },
          { label: 'Mes favoris', to: undefined },
        ]"
      />

      <div class="flex flex-wrap items-center justify-end gap-4 mb-8">
        <NuxtLink
          to="/marche-africain/mes-annonces"
          class="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl border border-gray-200 text-gray-600 hover:bg-white transition-all"
        >
          <font-awesome-icon :icon="['fas', 'sliders']" class="w-4 h-4" />
          Mes annonces
        </NuxtLink>
      </div>

      <!-- Chargement -->
      <div v-if="chargement" class="text-center py-16">
        <div class="animate-spin rounded-full h-12 w-12 border-4 border-emerald-500 border-t-transparent mx-auto mb-4"></div>
        <p class="text-gray-500">Chargement…</p>
      </div>

      <!-- Vide -->
      <div v-else-if="annonces.length === 0" class="text-center py-16 bg-white rounded-2xl shadow-xs">
        <font-awesome-icon :icon="['far', 'heart']" class="w-16 h-16 text-gray-300 mx-auto mb-4" />
        <h3 class="text-lg font-semibold text-gray-700 mb-2">Aucun favori</h3>
        <p class="text-gray-500 mb-4">Parcourez le marché et ajoutez des annonces à vos favoris.</p>
        <NuxtLink
          to="/marche-africain"
          class="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl bg-emerald-500 text-white font-medium hover:bg-emerald-600 transition-colors"
        >
          Explorer le marché
        </NuxtLink>
      </div>

      <!-- Grille -->
      <div v-else class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 gap-6">
        <MarcheAnnonceCard
          v-for="annonce in annonces"
          :key="annonce.id"
          :annonce="annonce"
          :est-favori="true"
        />
      </div>

      <!-- Pagination -->
      <div v-if="totalPages > 1" class="mt-10 flex items-center justify-center gap-2">
        <button
          :disabled="page === 1"
          class="p-2 rounded-lg border border-gray-200 text-gray-600 hover:bg-gray-50 disabled:opacity-50"
          @click="changerPage(page - 1)"
        >
          <font-awesome-icon :icon="['fas', 'chevron-left']" class="w-4 h-4" />
        </button>
        <span class="px-4 py-2 text-sm text-gray-600">Page {{ page }} / {{ totalPages }}</span>
        <button
          :disabled="page === totalPages"
          class="p-2 rounded-lg border border-gray-200 text-gray-600 hover:bg-gray-50 disabled:opacity-50"
          @click="changerPage(page + 1)"
        >
          <font-awesome-icon :icon="['fas', 'chevron-right']" class="w-4 h-4" />
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useMarcheAfricain, type AnnonceAPI } from '~/composables/useMarcheAfricain'

definePageMeta({ middleware: 'auth' })

useHead({ title: 'Mes favoris - Marché Africain - AfricanS' })

const { chargement, listerFavoris } = useMarcheAfricain()

const annonces = ref<AnnonceAPI[]>([])
const page = ref(1)
const totalPages = ref(1)
const PAR_PAGE = 12

const charger = async () => {
  const r = await listerFavoris({ page: page.value, par_page: PAR_PAGE })
  if (r) {
    annonces.value = r.annonces
    totalPages.value = r.total_pages
  }
}

const changerPage = (p: number) => {
  if (p >= 1 && p <= totalPages.value) {
    page.value = p
    charger()
    window.scrollTo({ top: 0, behavior: 'smooth' })
  }
}

onMounted(charger)
</script>
