<template>
  <NuxtLayout name="africans">
    <template #fil-ariane>
      <AfricansFilAriane :segments="filAriane" />
    </template>

    <!-- Enfant DIRECT du gabarit : imbriqué dans la branche `v-else`,
         ce ne serait plus un slot. Il ne rend rien tant que l'élément
         n'est pas chargé. -->
    <template #bandeau>
      <OpportuniteAfriqueDetailHero
        v-if="recette"
        :titre="recette.titre"
        :sous-titre="recette.territoires_consommation"
        :image="images.length ? resoudreUrlImage(images[0]!) : null"
      >
        <template #badges>
          <span class="inline-flex items-center gap-1.5 rounded-full bg-af-chocolat px-3 py-1 text-sm font-bold text-white">
            <font-awesome-icon :icon="['fas', 'utensils']" class="w-3.5 h-3.5" />
            Recette culinaire
          </span>
        </template>
        <template #sous-titre-icon>
          <font-awesome-icon :icon="['fas', 'location-dot']" class="w-4 h-4 text-af-chocolat" />
        </template>
      </OpportuniteAfriqueDetailHero>
    </template>

    <!-- Panneaux secondaires dans le rail du gabarit : contacts, partage
         et navigation ne disputent plus leur largeur au texte. -->
    <template #rail>
      <template v-if="recette">
        <!-- Repères -->
        <section class="rounded-[10px] border border-af-bordure bg-white p-6">
          <h2 class="mb-4 text-xs font-semibold uppercase tracking-wide text-af-atone">En bref</h2>
          <dl class="space-y-3 text-sm">
            <div v-if="recette.territoires_consommation" class="flex items-start gap-3">
              <span class="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-af-chocolat/10 text-af-chocolat">
                <font-awesome-icon :icon="['fas', 'location-dot']" class="w-4 h-4" />
              </span>
              <div>
                <dt class="text-xs text-af-atone">Territoires de consommation</dt>
                <dd class="font-medium text-af-encre">{{ recette.territoires_consommation }}</dd>
              </div>
            </div>
            <div v-if="recette.ingredients && recette.ingredients.length" class="flex items-start gap-3">
              <span class="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-af-chocolat/10 text-af-chocolat">
                <font-awesome-icon :icon="['fas', 'basket-shopping']" class="w-4 h-4" />
              </span>
              <div>
                <dt class="text-xs text-af-atone">Ingrédients</dt>
                <dd class="font-medium text-af-encre">{{ recette.ingredients.length }}</dd>
              </div>
            </div>
            <div v-if="recette.etapes_preparation && recette.etapes_preparation.length" class="flex items-start gap-3">
              <span class="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-af-chocolat/10 text-af-chocolat">
                <font-awesome-icon :icon="['fas', 'list-ol']" class="w-4 h-4" />
              </span>
              <div>
                <dt class="text-xs text-af-atone">Étapes</dt>
                <dd class="font-medium text-af-encre">{{ recette.etapes_preparation.length }}</dd>
              </div>
            </div>
          </dl>
        </section>

        <!-- Réactions & partage -->
        <section class="rounded-[10px] border border-af-bordure bg-white p-6">
          <h2 class="mb-4 text-xs font-semibold uppercase tracking-wide text-af-atone">Cette recette vous met en appétit ?</h2>
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
            class="mt-3 flex w-full items-center justify-center gap-2 rounded-lg bg-af-chocolat px-4 py-2.5 text-sm font-medium text-white transition hover:opacity-90 cursor-pointer"
            @click="showPartage = true"
          >
            <font-awesome-icon :icon="['fas', 'share-nodes']" class="w-4 h-4" />
            Partager
          </button>
          <!--
            `RecetteCulinaireAPI` n'expose pas son `cree_par` : le masquage
            anti-auto-cadeau repose donc sur le serveur, qui refuse en 403.
          -->
          <div class="mt-3 flex justify-center">
            <EngagementOffrirCadeauBouton
              type-objet="recette_culinaire"
              :objet-id="recette.id"
              :destinataire="recette.titre"
              @offert="cadeauxRef?.rafraichir()"
            />
          </div>
        </section>

        <!-- Cadeaux reçus par cette recette -->
        <section class="rounded-[10px] border border-af-bordure bg-white p-6">
          <EngagementCadeauxRecus
            ref="cadeauxRef"
            type-objet="recette_culinaire"
            :objet-id="recette.id"
          />
        </section>

        <!-- Actions -->
        <section class="rounded-[10px] border border-af-bordure bg-white p-6">
          <NuxtLink
            :to="`/opportunite-afrique/${ficheId}`"
            class="flex w-full items-center justify-center gap-2 rounded-lg border border-af-bordure px-4 py-2.5 text-sm font-medium text-af-corps transition hover:bg-af-fond"
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
      </template>
    </template>

    <!-- Chargement -->
    <div v-if="chargement" class="flex items-center justify-center py-24">
      <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-af-chocolat"></div>
    </div>

    <!-- Introuvable -->
    <div v-else-if="!recette" class="flex flex-col items-center justify-center py-24 px-4 text-center">
      <font-awesome-icon :icon="['fas', 'utensils']" class="w-14 h-14 text-af-atone-2 mb-4" />
      <h1 class="text-2xl font-bold text-af-corps mb-2">Recette introuvable</h1>
      <p class="text-af-atone mb-4">Cette recette n'existe pas ou a été retirée.</p>
      <NuxtLink :to="`/opportunite-afrique/${ficheId}`" class="text-af-chocolat hover:underline">
        &#8592; Retour au territoire
      </NuxtLink>
    </div>

    <!-- Contenu -->
    <template v-else>

      <div class="flex flex-col gap-6">
        <!-- Bandeau de suspension -->
        <div
          v-if="recette.suspendu"
          class="flex items-start gap-2 rounded-[10px] border border-af-chocolat/20 bg-af-chocolat/5 px-4 py-3 text-sm text-af-chocolat"
        >
          <font-awesome-icon :icon="['fas', 'triangle-exclamation']" class="w-4 h-4 mt-0.5 shrink-0" />
          <span>Contribution suspendue : en cours de vérification par la modération.</span>
        </div>

        <!-- Galerie -->
        <section v-if="images.length" class="overflow-hidden rounded-[10px] border border-af-bordure bg-white">
          <div class="relative aspect-video bg-af-fond">
            <img :src="imagePrincipale" :alt="recette.titre" class="h-full w-full object-cover" />

            <template v-if="images.length > 1">
              <button
                type="button"
                class="absolute left-3 top-1/2 flex h-10 w-10 -translate-y-1/2 cursor-pointer items-center justify-center rounded-full bg-white/85 text-af-corps shadow hover:bg-white"
                @click="precedente"
              >
                <font-awesome-icon :icon="['fas', 'chevron-left']" class="w-4 h-4" />
              </button>
              <button
                type="button"
                class="absolute right-3 top-1/2 flex h-10 w-10 -translate-y-1/2 cursor-pointer items-center justify-center rounded-full bg-white/85 text-af-corps shadow hover:bg-white"
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
        <section v-if="recette.histoire" class="rounded-[10px] border border-af-bordure bg-white p-6 sm:p-8">
          <h2 class="mb-4 flex items-center gap-2 font-oswald text-xl font-bold text-af-encre">
            <span class="flex h-8 w-8 items-center justify-center rounded-lg bg-af-chocolat/10 text-af-chocolat">
              <font-awesome-icon :icon="['fas', 'book-open']" class="w-4 h-4" />
            </span>
            Histoire &amp; origines
          </h2>
          <p class="leading-relaxed text-af-corps">{{ recette.histoire }}</p>
        </section>

        <!-- Ingrédients + préparation -->
        <section
          v-if="(recette.ingredients && recette.ingredients.length) || (recette.etapes_preparation && recette.etapes_preparation.length)"
          class="rounded-[10px] border border-af-bordure bg-white p-6 sm:p-8"
        >
          <div class="grid grid-cols-1 gap-8 md:grid-cols-2">
            <div v-if="recette.ingredients && recette.ingredients.length">
              <h2 class="mb-3 flex items-center gap-2 text-base font-semibold text-af-encre">
                <font-awesome-icon :icon="['fas', 'basket-shopping']" class="w-4 h-4 text-af-chocolat" />
                Ingrédients
              </h2>
              <ul class="list-inside list-disc space-y-1.5 text-sm text-af-corps">
                <li v-for="(ing, i) in recette.ingredients" :key="i">{{ ing }}</li>
              </ul>
            </div>

            <div v-if="recette.etapes_preparation && recette.etapes_preparation.length">
              <h2 class="mb-3 flex items-center gap-2 text-base font-semibold text-af-encre">
                <font-awesome-icon :icon="['fas', 'list-ol']" class="w-4 h-4 text-af-chocolat" />
                Mode de préparation
              </h2>
              <ol class="space-y-2.5 text-sm text-af-corps">
                <li v-for="(etape, i) in recette.etapes_preparation" :key="i" class="flex gap-2">
                  <span class="shrink-0 font-semibold text-af-chocolat">{{ i + 1 }}.</span>
                  <span>{{ etape }}</span>
                </li>
              </ol>
            </div>
          </div>
        </section>
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
  </NuxtLayout>
</template>

<script setup lang="ts">

definePageMeta({ layout: false })
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

// Rechargé après qu'un cadeau vient d'être offert.
const cadeauxRef = ref<{ rafraichir: () => void } | null>(null)
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

const filAriane = computed(() => [
  { libelle: 'Opportunités en Afrique', vers: '/opportunite-afrique' },
  { libelle: 'Territoire', vers: `/opportunite-afrique/${ficheId}` },
  { libelle: recette.value?.titre || 'Recette' },
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
  const titre = `${recette.value.titre}, Recette | UAfricas`
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
