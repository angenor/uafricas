<template>
  <div class="min-h-screen bg-gray-50">
    <div v-if="chargement" class="flex items-center justify-center h-screen">
      <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-custom-green"></div>
    </div>

    <div v-else-if="!secteur" class="flex flex-col items-center justify-center h-screen px-4 text-center">
      <font-awesome-icon :icon="['fas', 'briefcase']" class="w-14 h-14 text-gray-300 mb-4" />
      <h1 class="text-2xl font-bold text-gray-700 mb-2">Secteur introuvable</h1>
      <p class="text-gray-500 mb-4">Ce secteur d'opportunité n'existe pas ou a été retiré.</p>
      <NuxtLink :to="`/opportunite-afrique/${ficheId}`" class="text-custom-green hover:underline">
        &#8592; Retour au territoire
      </NuxtLink>
    </div>

    <template v-else>
      <!-- Hero immersif -->
      <OpportuniteAfriqueDetailHero
        :titre="secteur.nom"
        :sous-titre="secteur.localite"
        :image="secteur.image_url ? resoudreUrlImage(secteur.image_url) : null"
        :breadcrumbs="breadcrumbs"
        degrade-fond="bg-gradient-to-br from-custom-green to-emerald-800"
      >
        <template #badges>
          <span class="inline-flex items-center gap-1.5 rounded-full bg-custom-green px-3 py-1 text-sm font-medium text-white shadow-sm">
            <font-awesome-icon :icon="['fas', 'briefcase']" class="w-3.5 h-3.5" />
            Secteur d'opportunité
          </span>
        </template>
        <template #sous-titre-icon>
          <font-awesome-icon :icon="['fas', 'location-dot']" class="w-4 h-4 text-custom-green" />
        </template>
      </OpportuniteAfriqueDetailHero>

      <!-- Corps chevauchant le hero -->
      <div class="relative z-10 mx-auto -mt-10 max-w-5xl px-4 pb-16 sm:px-6 lg:px-8">
        <div class="grid grid-cols-1 gap-6 lg:grid-cols-3">
          <!-- Colonne principale -->
          <div class="space-y-6 lg:col-span-2">
            <!-- Bandeau de suspension -->
            <div
              v-if="secteur.suspendu"
              class="flex items-start gap-2 rounded-xl border border-orange-200 bg-orange-50 px-4 py-3 text-sm text-orange-800 shadow-sm"
            >
              <font-awesome-icon :icon="['fas', 'triangle-exclamation']" class="w-4 h-4 mt-0.5 shrink-0" />
              <span>Contribution suspendue — en cours de vérification par la modération.</span>
            </div>

            <!-- Description -->
            <section v-if="secteur.description" class="rounded-2xl border border-gray-100 bg-white p-6 shadow-sm sm:p-8">
              <h2 class="mb-4 flex items-center gap-2 font-oswald text-xl font-bold text-gray-900">
                <span class="flex h-8 w-8 items-center justify-center rounded-lg bg-custom-green/10 text-custom-green">
                  <font-awesome-icon :icon="['fas', 'circle-info']" class="w-4 h-4" />
                </span>
                Présentation du secteur
              </h2>
              <p class="whitespace-pre-line leading-relaxed text-gray-600">{{ secteur.description }}</p>
            </section>

            <!-- Références utiles -->
            <section v-if="secteur.references_utiles" class="rounded-2xl border border-gray-100 bg-white p-6 shadow-sm sm:p-8">
              <h2 class="mb-3 flex items-center gap-2 font-oswald text-xl font-bold text-gray-900">
                <span class="flex h-8 w-8 items-center justify-center rounded-lg bg-custom-chocolat/10 text-custom-chocolat">
                  <font-awesome-icon :icon="['fas', 'book']" class="w-4 h-4" />
                </span>
                Références utiles
              </h2>
              <p class="whitespace-pre-line leading-relaxed text-gray-600">{{ secteur.references_utiles }}</p>
            </section>
          </div>

          <!-- Sidebar -->
          <aside class="space-y-6">
            <!-- Contacts -->
            <section
              v-if="aContact"
              class="rounded-2xl border border-gray-100 bg-white p-6 shadow-sm"
            >
              <h2 class="mb-4 text-xs font-semibold uppercase tracking-wide text-gray-500">Contacts</h2>
              <div class="space-y-3 text-sm">
                <a v-if="secteur.contact_telephone" :href="`tel:${secteur.contact_telephone}`" class="flex items-center gap-3 text-gray-700 hover:text-custom-green">
                  <span class="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-custom-green/10 text-custom-green">
                    <font-awesome-icon :icon="['fas', 'phone']" class="w-4 h-4" />
                  </span>
                  {{ secteur.contact_telephone }}
                </a>
                <a v-if="secteur.contact_courriel" :href="`mailto:${secteur.contact_courriel}`" class="flex items-center gap-3 break-all text-gray-700 hover:text-custom-green">
                  <span class="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-custom-green/10 text-custom-green">
                    <font-awesome-icon :icon="['fas', 'envelope']" class="w-4 h-4" />
                  </span>
                  {{ secteur.contact_courriel }}
                </a>
                <div v-if="secteur.contact_adresse" class="flex items-center gap-3 text-gray-700">
                  <span class="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-custom-green/10 text-custom-green">
                    <font-awesome-icon :icon="['fas', 'location-dot']" class="w-4 h-4" />
                  </span>
                  {{ secteur.contact_adresse }}
                </div>
              </div>
              <a
                v-if="secteur.site_web_url"
                :href="secteur.site_web_url"
                target="_blank"
                rel="noopener noreferrer"
                class="mt-4 inline-flex w-full items-center justify-center gap-2 rounded-lg bg-custom-chocolat px-4 py-2.5 text-sm font-medium text-white transition hover:bg-custom-chocolat/90"
              >
                <font-awesome-icon :icon="['fas', 'arrow-up-right-from-square']" class="w-3.5 h-3.5" />
                Visiter le site web
              </a>
            </section>

            <!-- Réactions & partage -->
            <section class="rounded-2xl border border-gray-100 bg-white p-6 shadow-sm">
              <h2 class="mb-4 text-xs font-semibold uppercase tracking-wide text-gray-500">Ce secteur vous intéresse ?</h2>
              <OpportuniteAfriqueReactionsBar
                type-objet="secteur_developpement"
                :objet-id="secteur.id"
                :nombre-likes="secteur.nombre_likes"
                :nombre-dislikes="secteur.nombre_dislikes"
                :ma-reaction="secteur.ma_reaction"
                :est-authentifie="userStore.isAuthenticated"
                @require-login="redirigerVersConnexion()"
              />
              <button
                type="button"
                class="mt-3 flex w-full items-center justify-center gap-2 rounded-lg bg-custom-green px-4 py-2.5 text-sm font-medium text-white transition hover:bg-custom-green/90 cursor-pointer"
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
                  type-objet="secteur_developpement"
                  :objet-id="secteur.id"
                  :libelle="secteur.nom"
                  :a-signale="secteur.a_signale"
                  :est-authentifie="userStore.isAuthenticated"
                  @require-login="redirigerVersConnexion()"
                  @suspendu="secteur.suspendu = true"
                />
              </div>
            </section>
          </aside>
        </div>
      </div>

      <!-- Modal partage (réseaux sociaux + mur communautaire) -->
      <OpportuniteAfriquePartagerElementModal
        :is-open="showPartage"
        :titre="secteur.nom"
        type-label="le secteur"
        type-objet="secteur_developpement"
        :objet-id="secteur.id"
        :est-connecte="userStore.isAuthenticated"
        @close="showPartage = false"
      />
    </template>
  </div>
</template>

<script setup lang="ts">
import { useOpportuniteAfrique, type SecteurOpportuniteAPI } from '~/composables/useOpportuniteAfrique'
import { useUserStore } from '~/stores/user'

const route = useRoute()
const userStore = useUserStore()
const { redirigerVersConnexion } = useAuth()
const { obtenirSecteurOpportunite, resoudreUrlImage } = useOpportuniteAfrique()

const showPartage = ref(false)

const ficheId = route.params.id as string
const secteurId = route.params.secteurId as string

const { data: secteurCharge, pending: chargement } = await useAsyncData(
  `secteur-${ficheId}-${secteurId}`,
  () => obtenirSecteurOpportunite(ficheId, secteurId),
)
const secteur = ref<SecteurOpportuniteAPI | null>(secteurCharge.value)

const aContact = computed(() =>
  !!(secteur.value?.contact_telephone || secteur.value?.contact_courriel || secteur.value?.contact_adresse || secteur.value?.site_web_url),
)

const breadcrumbs = computed(() => [
  { label: 'Opportunités en Afrique', to: '/opportunite-afrique' },
  { label: 'Territoire', to: `/opportunite-afrique/${ficheId}` },
  { label: secteur.value?.nom || 'Secteur', to: undefined },
])

// ── SEO / Open Graph (aperçu lors du partage sur les réseaux sociaux) ──
const requete = useRequestURL()
const urlCanonique = `${requete.protocol}//${requete.host}/opportunite-afrique/${ficheId}/secteurs/${secteurId}`
const imageOg = computed(() => (secteur.value?.image_url ? resoudreUrlImage(secteur.value.image_url) : ''))
const descriptionOg = computed(() =>
  (secteur.value?.description || `Secteur d'opportunité ${secteur.value?.nom ?? ''}`).slice(0, 200),
)

useHead(() => {
  if (!secteur.value) return {}
  const titre = `${secteur.value.nom} — Secteur d'opportunité — UAfricas`
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
