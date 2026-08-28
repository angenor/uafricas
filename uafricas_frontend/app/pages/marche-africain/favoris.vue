<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <!-- L'image était hotlinkée sur Unsplash ; celle du module existait
           déjà dans le dépôt. -->
      <AfricansBandeauModule
        titre="Mes favoris"
        sous-titre="Les annonces d'Afromarket que vous avez sauvegardées."
        image="/images/marche-afrique.png"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[
          { libelle: 'Opafrica', vers: '/marche-africain' },
          { libelle: 'Afromarket', vers: '/marche-africain' },
          { libelle: 'Mes favoris' },
        ]"
      >
        <template #action>
          <AfricansBouton vers="/marche-africain/mes-annonces" variante="secondaire" icone="fa-solid fa-sliders">
            Mes annonces
          </AfricansBouton>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="flex flex-col gap-6">

      <!-- Chargement -->
      <div v-if="chargement" class="text-center py-16">
        <div class="animate-spin rounded-full h-12 w-12 border-4 border-af-vert border-t-transparent mx-auto mb-4"></div>
        <p class="text-af-atone">Chargement…</p>
      </div>

      <!-- Vide -->
      <div v-else-if="annonces.length === 0" class="text-center py-16 bg-white rounded-2xl shadow-xs">
        <font-awesome-icon :icon="['far', 'heart']" class="w-16 h-16 text-af-atone-2 mx-auto mb-4" />
        <h3 class="text-lg font-semibold text-af-corps mb-2">Aucun favori</h3>
        <p class="text-af-atone mb-4">Parcourez le marché et ajoutez des annonces à vos favoris.</p>
        <NuxtLink
          to="/marche-africain"
          class="inline-flex items-center gap-2 px-5 py-2.5 rounded-lg bg-af-vert text-white font-medium hover:bg-af-vert transition-colors"
        >
          Explorer le marché
        </NuxtLink>
      </div>

      <!-- Grille -->
      <div v-else class="grid gap-5 sm:grid-cols-2">
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
          class="p-2 rounded-lg border border-af-bordure text-af-corps hover:bg-af-fond disabled:opacity-50"
          @click="changerPage(page - 1)"
        >
          <font-awesome-icon :icon="['fas', 'chevron-left']" class="w-4 h-4" />
        </button>
        <span class="px-4 py-2 text-sm text-af-corps">Page {{ page }} / {{ totalPages }}</span>
        <button
          :disabled="page === totalPages"
          class="p-2 rounded-lg border border-af-bordure text-af-corps hover:bg-af-fond disabled:opacity-50"
          @click="changerPage(page + 1)"
        >
          <font-awesome-icon :icon="['fas', 'chevron-right']" class="w-4 h-4" />
        </button>
      </div>
    </div>
  </NuxtLayout>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useMarcheAfricain, type AnnonceAPI } from '~/composables/useMarcheAfricain'

definePageMeta({ middleware: 'auth', layout: false })

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
