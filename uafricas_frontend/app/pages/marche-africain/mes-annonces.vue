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
            Mes annonces
          </h1>
          <p class="absolute inset-0 flex items-center justify-center text-white/95 text-sm md:text-base px-2 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
            Gérez vos annonces : modifier, marquer conclue ou supprimer.
          </p>
        </div>
      </div>
    </div>

    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-10">
      <!-- En-tête -->
      <CommonBreadcrumbNav
        class="mb-6"
        :custom-breadcrumbs="[
          { label: 'Marché Africain', to: '/marche-africain' },
          { label: 'Mes annonces', to: undefined },
        ]"
      />

      <div class="flex flex-wrap items-center justify-end gap-4 mb-8">
        <NuxtLink
          to="/marche-africain"
          class="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl bg-linear-to-r from-custom-chocolat to-amber-700 text-white font-semibold hover:from-amber-700 hover:to-custom-chocolat transition-all"
        >
          <font-awesome-icon :icon="['fas', 'plus']" class="w-4 h-4" />
          Publier une annonce
        </NuxtLink>
      </div>

      <!-- Chargement -->
      <div v-if="chargement" class="text-center py-16">
        <div class="animate-spin rounded-full h-12 w-12 border-4 border-emerald-500 border-t-transparent mx-auto mb-4"></div>
        <p class="text-gray-500">Chargement…</p>
      </div>

      <!-- Vide -->
      <div v-else-if="annonces.length === 0" class="text-center py-16 bg-white rounded-2xl shadow-xs">
        <font-awesome-icon :icon="['fas', 'box-open']" class="w-16 h-16 text-gray-300 mx-auto mb-4" />
        <h3 class="text-lg font-semibold text-gray-700 mb-2">Aucune annonce</h3>
        <p class="text-gray-500">Publiez votre première annonce sur le Marché Africain.</p>
      </div>

      <!-- Liste -->
      <div v-else class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
        <div
          v-for="a in annonces"
          :key="a.id"
          class="bg-white rounded-2xl shadow-sm overflow-hidden flex flex-col"
        >
          <div class="relative aspect-[16/10] bg-gray-100">
            <img v-if="a.photo_url" :src="a.photo_url" :alt="a.titre" class="w-full h-full object-cover" />
  <!-- Pas de `<img src="…placeholder.jpg">` : ce fichier n'a jamais existé, si
       bien qu'une annonce sans photo affichait une image CASSÉE avec son texte
       de remplacement en travers. Un repli qui doit exister sur le disque est un
       repli qui peut manquer ; celui-ci est du balisage, il ne peut pas
       échouer. -->
            <div v-else class="grid h-full w-full place-items-center">
              <font-awesome-icon icon="fa-solid fa-image" class="text-3xl text-af-atone-2" />
            </div>
            <span class="absolute top-3 left-3 px-3 py-1 rounded-full text-xs font-semibold" :class="badgeEtat(a.etat)">
              {{ libelleEtat(a.etat) }}
            </span>
          </div>
          <div class="p-4 flex-1 flex flex-col">
            <p class="text-xs text-gray-400 mb-1">{{ a.type_echange }} · {{ a.categorie }}</p>
            <h3 class="font-semibold text-gray-800 line-clamp-2 mb-2">{{ a.titre }}</h3>
            <p class="text-custom-chocolat font-bold mb-4">{{ formatPrix(a.prix, a.devise) }}</p>

            <div class="mt-auto flex flex-wrap gap-2">
              <NuxtLink
                :to="`/marche-africain/${a.id}`"
                class="px-3 py-1.5 rounded-lg text-sm border border-gray-200 text-gray-600 hover:bg-gray-50"
              >
                Voir
              </NuxtLink>
              <button
                class="px-3 py-1.5 rounded-lg text-sm border border-gray-200 text-gray-600 hover:bg-gray-50"
                @click="ouvrirEdition(a.id)"
              >
                Modifier
              </button>
              <button
                v-if="a.etat === 'publiee'"
                class="px-3 py-1.5 rounded-lg text-sm border border-emerald-200 text-emerald-700 hover:bg-emerald-50"
                @click="marquerConclue(a.id)"
              >
                Marquer conclue
              </button>
              <button
                class="px-3 py-1.5 rounded-lg text-sm border border-red-200 text-red-600 hover:bg-red-50"
                @click="confirmerSuppression(a.id)"
              >
                Supprimer
              </button>
            </div>
          </div>
        </div>
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

    <!-- Modal édition -->
    <Teleport to="body">
      <Transition
        enter-active-class="transition ease-out duration-300"
        enter-from-class="opacity-0"
        enter-to-class="opacity-100"
        leave-active-class="transition ease-in duration-200"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
      >
        <div v-if="showEdition && annonceEnEdition" class="fixed inset-0 z-50 flex items-center justify-center p-4">
          <div class="absolute inset-0 bg-black/50" @click="showEdition = false"></div>
          <div class="relative bg-white rounded-2xl shadow-xl max-w-2xl w-full max-h-[90vh] overflow-y-auto">
            <div class="sticky top-0 bg-white flex items-center justify-between px-6 py-4 border-b border-gray-100 z-10">
              <h2 class="text-xl font-bold text-gray-800">Modifier l'annonce</h2>
              <button class="p-2 text-gray-400 hover:text-gray-600" @click="showEdition = false">
                <font-awesome-icon :icon="['fas', 'xmark']" class="w-5 h-5" />
              </button>
            </div>
            <div class="p-6">
              <MarcheAnnonceForm
                mode="edition"
                :annonce="annonceEnEdition"
                @success="onEditionReussie"
                @cancel="showEdition = false"
              />
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import {
  useMarcheAfricain,
  formatPrix,
  type MesAnnonceItemAPI,
  type AnnonceDetailAPI,
} from '~/composables/useMarcheAfricain'

definePageMeta({ middleware: 'auth' })

useHead({ title: 'Mes annonces - Marché Africain - AfricanS' })

const {
  chargement,
  mesAnnonces,
  obtenirAnnonce,
  conclureAnnonce,
  supprimerAnnonce,
} = useMarcheAfricain()

const annonces = ref<MesAnnonceItemAPI[]>([])
const page = ref(1)
const totalPages = ref(1)
const showEdition = ref(false)
const annonceEnEdition = ref<AnnonceDetailAPI | null>(null)

const PAR_PAGE = 12

const charger = async () => {
  const r = await mesAnnonces({ page: page.value, par_page: PAR_PAGE })
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

const libelleEtat = (etat: string): string => {
  const map: Record<string, string> = {
    publiee: 'Publiée',
    conclue: 'Conclue',
    suspendue: 'Suspendue',
    expiree: 'Expirée',
    brouillon: 'Brouillon',
    en_attente: 'En attente',
    supprimee: 'Supprimée',
  }
  return map[etat] || etat
}

const badgeEtat = (etat: string): string => {
  switch (etat) {
    case 'publiee': return 'bg-emerald-100 text-emerald-700'
    case 'conclue': return 'bg-gray-200 text-gray-600'
    case 'suspendue': return 'bg-red-100 text-red-700'
    default: return 'bg-amber-100 text-amber-700'
  }
}

const ouvrirEdition = async (id: string) => {
  const detail = await obtenirAnnonce(id)
  if (detail) {
    annonceEnEdition.value = detail
    showEdition.value = true
  }
}

const onEditionReussie = async () => {
  showEdition.value = false
  annonceEnEdition.value = null
  await charger()
}

const marquerConclue = async (id: string) => {
  if (!confirm('Marquer cette annonce comme conclue ? Elle ne sera plus visible publiquement.')) return
  const ok = await conclureAnnonce(id)
  if (ok) await charger()
}

const confirmerSuppression = async (id: string) => {
  if (!confirm('Supprimer définitivement cette annonce ?')) return
  const ok = await supprimerAnnonce(id)
  if (ok) await charger()
}

onMounted(charger)
</script>
