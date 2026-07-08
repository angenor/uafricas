<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Chargement -->
    <div v-if="chargement" class="flex items-center justify-center h-screen">
      <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-amber-700"></div>
    </div>

    <!-- Introuvable -->
    <div v-else-if="!recette" class="flex flex-col items-center justify-center h-screen px-4 text-center">
      <font-awesome-icon :icon="['fas', 'utensils']" class="w-14 h-14 text-gray-300 mb-4" />
      <h1 class="text-2xl font-bold text-gray-700 mb-2">Recette introuvable</h1>
      <p class="text-gray-500 mb-4">Cette recette n'existe pas ou a été retirée.</p>
      <NuxtLink :to="`/opportunite-afrique/${ficheId}`" class="text-amber-700 hover:underline">
        &#8592; Retour au territoire
      </NuxtLink>
    </div>

    <!-- Contenu -->
    <template v-else>
      <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <CommonBreadcrumbNav class="mb-6" :custom-breadcrumbs="breadcrumbs" />

        <div class="bg-white rounded-2xl shadow-sm overflow-hidden">
          <!-- Galerie -->
          <div class="relative h-64 sm:h-96 bg-gradient-to-br from-amber-100 to-orange-200">
            <img
              v-if="imagePrincipale"
              :src="imagePrincipale"
              :alt="recette.titre"
              class="w-full h-full object-cover"
            />
            <div v-else class="w-full h-full flex items-center justify-center">
              <font-awesome-icon :icon="['fas', 'utensils']" class="w-16 h-16 text-amber-600" />
            </div>

            <template v-if="images.length > 1">
              <button
                type="button"
                class="absolute left-3 top-1/2 -translate-y-1/2 w-10 h-10 flex items-center justify-center rounded-full bg-white/85 text-gray-700 hover:bg-white shadow cursor-pointer"
                @click="precedente"
              >
                <font-awesome-icon :icon="['fas', 'chevron-left']" class="w-4 h-4" />
              </button>
              <button
                type="button"
                class="absolute right-3 top-1/2 -translate-y-1/2 w-10 h-10 flex items-center justify-center rounded-full bg-white/85 text-gray-700 hover:bg-white shadow cursor-pointer"
                @click="suivante"
              >
                <font-awesome-icon :icon="['fas', 'chevron-right']" class="w-4 h-4" />
              </button>
              <div class="absolute bottom-3 left-1/2 -translate-x-1/2 flex gap-1.5">
                <span
                  v-for="(img, i) in images"
                  :key="i"
                  class="w-2 h-2 rounded-full transition-colors"
                  :class="i === indexImage ? 'bg-white' : 'bg-white/50'"
                />
              </div>
            </template>
          </div>

          <!-- Corps -->
          <div class="p-6 sm:p-8">
            <h1 class="font-oswald text-3xl sm:text-4xl font-bold text-gray-900">{{ recette.titre }}</h1>

            <p
              v-if="recette.territoires_consommation"
              class="inline-flex items-center gap-1.5 text-sm text-gray-500 mt-2"
            >
              <font-awesome-icon :icon="['fas', 'location-dot']" class="w-3.5 h-3.5 text-amber-700" />
              {{ recette.territoires_consommation }}
            </p>

            <!-- Bandeau de suspension -->
            <div
              v-if="recette.suspendu"
              class="mt-4 flex items-start gap-2 rounded-md bg-orange-50 border border-orange-200 px-3 py-2 text-sm text-orange-800"
            >
              <font-awesome-icon :icon="['fas', 'triangle-exclamation']" class="w-4 h-4 mt-0.5 shrink-0" />
              <span>Contribution suspendue — en cours de vérification par la modération.</span>
            </div>

            <p v-if="recette.histoire" class="text-gray-600 leading-relaxed mt-5">
              {{ recette.histoire }}
            </p>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-8 mt-8">
              <div v-if="recette.ingredients && recette.ingredients.length">
                <h2 class="text-base font-semibold text-gray-800 mb-3 flex items-center gap-2">
                  <font-awesome-icon :icon="['fas', 'basket-shopping']" class="w-4 h-4 text-amber-700" />
                  Ingrédients
                </h2>
                <ul class="text-sm text-gray-600 space-y-1.5 list-disc list-inside">
                  <li v-for="(ing, i) in recette.ingredients" :key="i">{{ ing }}</li>
                </ul>
              </div>

              <div v-if="recette.etapes_preparation && recette.etapes_preparation.length">
                <h2 class="text-base font-semibold text-gray-800 mb-3 flex items-center gap-2">
                  <font-awesome-icon :icon="['fas', 'list-ol']" class="w-4 h-4 text-amber-700" />
                  Mode de préparation
                </h2>
                <ol class="text-sm text-gray-600 space-y-2.5">
                  <li v-for="(etape, i) in recette.etapes_preparation" :key="i" class="flex gap-2">
                    <span class="font-semibold text-amber-700 shrink-0">{{ i + 1 }}.</span>
                    <span>{{ etape }}</span>
                  </li>
                </ol>
              </div>
            </div>

            <!-- Actions -->
            <div class="flex flex-wrap items-center gap-4 mt-8 pt-5 border-t border-gray-100">
              <NuxtLink
                :to="`/opportunite-afrique/${ficheId}`"
                class="inline-flex items-center gap-1.5 text-sm font-medium text-gray-600 hover:text-amber-700"
              >
                <font-awesome-icon :icon="['fas', 'arrow-left']" class="w-3.5 h-3.5" />
                Retour au territoire
              </NuxtLink>
              <OpportuniteAfriqueContributionSignalerBouton
                type-objet="recette_culinaire"
                :objet-id="recette.id"
                :libelle="recette.titre"
                :a-signale="recette.a_signale"
                :est-authentifie="userStore.isAuthenticated"
                @require-login="navigateTo('/login')"
                @suspendu="recette.suspendu = true"
              />
            </div>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { useOpportuniteAfrique, type RecetteCulinaireAPI } from '~/composables/useOpportuniteAfrique'
import { useUserStore } from '~/stores/user'

const route = useRoute()
const userStore = useUserStore()
const { obtenirRecetteCulinaire, resoudreUrlImage } = useOpportuniteAfrique()

const ficheId = route.params.id as string
const recetteId = route.params.recetteId as string

const { data: recetteChargee, pending: chargement } = await useAsyncData(
  `recette-${ficheId}-${recetteId}`,
  () => obtenirRecetteCulinaire(ficheId, recetteId),
)
const recette = ref<RecetteCulinaireAPI | null>(recetteChargee.value)

// Galerie
const indexImage = ref(0)
const images = computed(() => recette.value?.images ?? [])
const imagePrincipale = computed(() =>
  images.value.length ? resoudreUrlImage(images.value[indexImage.value]) : '',
)
const precedente = () => {
  if (!images.value.length) return
  indexImage.value = (indexImage.value - 1 + images.value.length) % images.value.length
}
const suivante = () => {
  if (!images.value.length) return
  indexImage.value = (indexImage.value + 1) % images.value.length
}

const onKeydown = (e: KeyboardEvent) => {
  if (e.key === 'ArrowLeft') precedente()
  else if (e.key === 'ArrowRight') suivante()
}
onMounted(() => window.addEventListener('keydown', onKeydown))
onBeforeUnmount(() => window.removeEventListener('keydown', onKeydown))

const breadcrumbs = computed(() => [
  { label: 'Opportunités en Afrique', to: '/opportunite-afrique' },
  { label: 'Territoire', to: `/opportunite-afrique/${ficheId}` },
  { label: recette.value?.titre || 'Recette', to: undefined },
])

useHead(() => ({
  title: recette.value ? `${recette.value.titre} — Recette — UAfricas` : 'Recette — UAfricas',
  meta: recette.value
    ? [{ name: 'description', content: recette.value.histoire?.slice(0, 160) || `Recette ${recette.value.titre}` }]
    : [],
}))
</script>
