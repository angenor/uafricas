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
      <!-- Hero immersif -->
      <OpportuniteAfriqueDetailHero
        :titre="recette.titre"
        :sous-titre="recette.territoires_consommation"
        :image="images.length ? resoudreUrlImage(images[0]!) : null"
        :breadcrumbs="breadcrumbs"
        degrade-fond="bg-gradient-to-br from-amber-600 to-orange-800"
      >
        <template #badges>
          <span class="inline-flex items-center gap-1.5 rounded-full bg-amber-600 px-3 py-1 text-sm font-medium text-white shadow-sm">
            <font-awesome-icon :icon="['fas', 'utensils']" class="w-3.5 h-3.5" />
            Recette culinaire
          </span>
        </template>
        <template #sous-titre-icon>
          <font-awesome-icon :icon="['fas', 'location-dot']" class="w-4 h-4 text-amber-400" />
        </template>
      </OpportuniteAfriqueDetailHero>

      <!-- Corps chevauchant le hero -->
      <div class="relative z-10 mx-auto -mt-10 max-w-5xl px-4 pb-16 sm:px-6 lg:px-8">
        <div class="grid grid-cols-1 gap-6 lg:grid-cols-3">
          <!-- Colonne principale -->
          <div class="space-y-6 lg:col-span-2">
            <!-- Bandeau de suspension -->
            <div
              v-if="recette.suspendu"
              class="flex items-start gap-2 rounded-xl border border-orange-200 bg-orange-50 px-4 py-3 text-sm text-orange-800 shadow-sm"
            >
              <font-awesome-icon :icon="['fas', 'triangle-exclamation']" class="w-4 h-4 mt-0.5 shrink-0" />
              <span>Contribution suspendue — en cours de vérification par la modération.</span>
            </div>

            <!-- Galerie -->
            <section v-if="images.length" class="overflow-hidden rounded-2xl border border-gray-100 bg-white shadow-sm">
              <div class="relative aspect-video bg-gradient-to-br from-amber-100 to-orange-200">
                <img :src="imagePrincipale" :alt="recette.titre" class="h-full w-full object-cover" />

                <template v-if="images.length > 1">
                  <button
                    type="button"
                    class="absolute left-3 top-1/2 flex h-10 w-10 -translate-y-1/2 cursor-pointer items-center justify-center rounded-full bg-white/85 text-gray-700 shadow hover:bg-white"
                    @click="precedente"
                  >
                    <font-awesome-icon :icon="['fas', 'chevron-left']" class="w-4 h-4" />
                  </button>
                  <button
                    type="button"
                    class="absolute right-3 top-1/2 flex h-10 w-10 -translate-y-1/2 cursor-pointer items-center justify-center rounded-full bg-white/85 text-gray-700 shadow hover:bg-white"
                    @click="suivante"
                  >
                    <font-awesome-icon :icon="['fas', 'chevron-right']" class="w-4 h-4" />
                  </button>
                  <div class="absolute bottom-3 left-1/2 flex -translate-x-1/2 gap-1.5">
                    <span
                      v-for="(img, i) in images"
                      :key="i"
                      class="h-2 w-2 rounded-full transition-colors"
                      :class="i === indexImage ? 'bg-white' : 'bg-white/50'"
                    />
                  </div>
                </template>
              </div>
            </section>

            <!-- Histoire -->
            <section v-if="recette.histoire" class="rounded-2xl border border-gray-100 bg-white p-6 shadow-sm sm:p-8">
              <h2 class="mb-4 flex items-center gap-2 font-oswald text-xl font-bold text-gray-900">
                <span class="flex h-8 w-8 items-center justify-center rounded-lg bg-amber-100 text-amber-700">
                  <font-awesome-icon :icon="['fas', 'book-open']" class="w-4 h-4" />
                </span>
                Histoire &amp; origines
              </h2>
              <p class="leading-relaxed text-gray-600">{{ recette.histoire }}</p>
            </section>

            <!-- Ingrédients + préparation -->
            <section
              v-if="(recette.ingredients && recette.ingredients.length) || (recette.etapes_preparation && recette.etapes_preparation.length)"
              class="rounded-2xl border border-gray-100 bg-white p-6 shadow-sm sm:p-8"
            >
              <div class="grid grid-cols-1 gap-8 md:grid-cols-2">
                <div v-if="recette.ingredients && recette.ingredients.length">
                  <h2 class="mb-3 flex items-center gap-2 text-base font-semibold text-gray-800">
                    <font-awesome-icon :icon="['fas', 'basket-shopping']" class="w-4 h-4 text-amber-700" />
                    Ingrédients
                  </h2>
                  <ul class="list-inside list-disc space-y-1.5 text-sm text-gray-600">
                    <li v-for="(ing, i) in recette.ingredients" :key="i">{{ ing }}</li>
                  </ul>
                </div>

                <div v-if="recette.etapes_preparation && recette.etapes_preparation.length">
                  <h2 class="mb-3 flex items-center gap-2 text-base font-semibold text-gray-800">
                    <font-awesome-icon :icon="['fas', 'list-ol']" class="w-4 h-4 text-amber-700" />
                    Mode de préparation
                  </h2>
                  <ol class="space-y-2.5 text-sm text-gray-600">
                    <li v-for="(etape, i) in recette.etapes_preparation" :key="i" class="flex gap-2">
                      <span class="shrink-0 font-semibold text-amber-700">{{ i + 1 }}.</span>
                      <span>{{ etape }}</span>
                    </li>
                  </ol>
                </div>
              </div>
            </section>
          </div>

          <!-- Sidebar -->
          <aside class="space-y-6">
            <!-- Repères -->
            <section class="rounded-2xl border border-gray-100 bg-white p-6 shadow-sm">
              <h2 class="mb-4 text-xs font-semibold uppercase tracking-wide text-gray-500">En bref</h2>
              <dl class="space-y-3 text-sm">
                <div v-if="recette.territoires_consommation" class="flex items-start gap-3">
                  <span class="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-amber-100 text-amber-700">
                    <font-awesome-icon :icon="['fas', 'location-dot']" class="w-4 h-4" />
                  </span>
                  <div>
                    <dt class="text-xs text-gray-500">Territoires de consommation</dt>
                    <dd class="font-medium text-gray-900">{{ recette.territoires_consommation }}</dd>
                  </div>
                </div>
                <div v-if="recette.ingredients && recette.ingredients.length" class="flex items-start gap-3">
                  <span class="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-amber-100 text-amber-700">
                    <font-awesome-icon :icon="['fas', 'basket-shopping']" class="w-4 h-4" />
                  </span>
                  <div>
                    <dt class="text-xs text-gray-500">Ingrédients</dt>
                    <dd class="font-medium text-gray-900">{{ recette.ingredients.length }}</dd>
                  </div>
                </div>
                <div v-if="recette.etapes_preparation && recette.etapes_preparation.length" class="flex items-start gap-3">
                  <span class="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-amber-100 text-amber-700">
                    <font-awesome-icon :icon="['fas', 'list-ol']" class="w-4 h-4" />
                  </span>
                  <div>
                    <dt class="text-xs text-gray-500">Étapes</dt>
                    <dd class="font-medium text-gray-900">{{ recette.etapes_preparation.length }}</dd>
                  </div>
                </div>
              </dl>
            </section>

            <!-- Réactions & partage -->
            <section class="rounded-2xl border border-gray-100 bg-white p-6 shadow-sm">
              <h2 class="mb-4 text-xs font-semibold uppercase tracking-wide text-gray-500">Cette recette vous met en appétit ?</h2>
              <OpportuniteAfriqueReactionsBar
                type-objet="recette_culinaire"
                :objet-id="recette.id"
                :nombre-likes="recette.nombre_likes"
                :nombre-dislikes="recette.nombre_dislikes"
                :ma-reaction="recette.ma_reaction"
                :est-authentifie="userStore.isAuthenticated"
                @require-login="redirigerVersConnexion()"
              />
              <button
                type="button"
                class="mt-3 flex w-full items-center justify-center gap-2 rounded-lg bg-amber-600 px-4 py-2.5 text-sm font-medium text-white transition hover:bg-amber-700 cursor-pointer"
                @click="showPartage = true"
              >
                <font-awesome-icon :icon="['fas', 'share-nodes']" class="w-4 h-4" />
                Partager
              </button>
            </section>

            <!-- Actions -->
            <section class="rounded-2xl border border-gray-100 bg-white p-6 shadow-sm">
              <NuxtLink
                :to="`/opportunite-afrique/${ficheId}`"
                class="flex w-full items-center justify-center gap-2 rounded-lg border border-gray-300 px-4 py-2.5 text-sm font-medium text-gray-700 transition hover:bg-gray-50"
              >
                <font-awesome-icon :icon="['fas', 'arrow-left']" class="w-3.5 h-3.5" />
                Retour au territoire
              </NuxtLink>
              <div class="mt-3 flex justify-center">
                <OpportuniteAfriqueContributionSignalerBouton
                  type-objet="recette_culinaire"
                  :objet-id="recette.id"
                  :libelle="recette.titre"
                  :a-signale="recette.a_signale"
                  :est-authentifie="userStore.isAuthenticated"
                  @require-login="redirigerVersConnexion()"
                  @suspendu="recette.suspendu = true"
                />
              </div>
            </section>
          </aside>
        </div>
      </div>

      <!-- Modal partage (réseaux sociaux + mur communautaire) -->
      <OpportuniteAfriquePartagerElementModal
        :is-open="showPartage"
        :titre="recette.titre"
        type-label="la recette"
        type-objet="recette_culinaire"
        :objet-id="recette.id"
        :est-connecte="userStore.isAuthenticated"
        @close="showPartage = false"
      />
    </template>
  </div>
</template>

<script setup lang="ts">
import { useOpportuniteAfrique, type RecetteCulinaireAPI } from '~/composables/useOpportuniteAfrique'
import { useUserStore } from '~/stores/user'

const route = useRoute()
const userStore = useUserStore()
const { redirigerVersConnexion } = useAuth()
const { obtenirRecetteCulinaire, resoudreUrlImage } = useOpportuniteAfrique()

const showPartage = ref(false)

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
  images.value.length ? resoudreUrlImage(images.value[indexImage.value]!) : '',
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

// ── SEO / Open Graph (aperçu lors du partage sur les réseaux sociaux) ──
const requete = useRequestURL()
const urlCanonique = `${requete.protocol}//${requete.host}/opportunite-afrique/${ficheId}/recettes/${recetteId}`
const imageOg = computed(() => (images.value.length ? resoudreUrlImage(images.value[0]!) : ''))
const descriptionOg = computed(() =>
  (recette.value?.histoire || `Recette ${recette.value?.titre ?? ''}`).slice(0, 200),
)

useHead(() => {
  if (!recette.value) return {}
  const titre = `${recette.value.titre} — Recette — UAfricas`
  return {
    title: titre,
    meta: [
      { name: 'description', content: descriptionOg.value },
      { property: 'og:type', content: 'article' },
      { property: 'og:title', content: titre },
      { property: 'og:description', content: descriptionOg.value },
      { property: 'og:url', content: urlCanonique },
      { property: 'og:site_name', content: 'UAfricas' },
      ...(imageOg.value ? [{ property: 'og:image', content: imageOg.value }] : []),
      { name: 'twitter:card', content: imageOg.value ? 'summary_large_image' : 'summary' },
      { name: 'twitter:title', content: titre },
      { name: 'twitter:description', content: descriptionOg.value },
      ...(imageOg.value ? [{ name: 'twitter:image', content: imageOg.value }] : []),
    ],
    link: [{ rel: 'canonical', href: urlCanonique }],
  }
})
</script>
