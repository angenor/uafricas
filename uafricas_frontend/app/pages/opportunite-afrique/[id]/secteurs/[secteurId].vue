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
      <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <CommonBreadcrumbNav class="mb-6" :custom-breadcrumbs="breadcrumbs" />

        <div class="bg-white rounded-2xl shadow-sm overflow-hidden border-l-4 border-custom-green">
          <img
            v-if="secteur.image_url"
            :src="resoudreUrlImage(secteur.image_url)"
            :alt="secteur.nom"
            class="w-full h-64 sm:h-80 object-cover"
          />

          <div class="p-6 sm:p-8">
            <h1 class="font-oswald text-3xl sm:text-4xl font-bold text-gray-900">{{ secteur.nom }}</h1>

            <p v-if="secteur.localite" class="inline-flex items-center gap-1.5 text-sm text-gray-500 mt-2">
              <font-awesome-icon :icon="['fas', 'location-dot']" class="w-3.5 h-3.5 text-custom-chocolat" />
              {{ secteur.localite }}
            </p>

            <!-- Bandeau de suspension -->
            <div
              v-if="secteur.suspendu"
              class="mt-4 flex items-start gap-2 rounded-md bg-orange-50 border border-orange-200 px-3 py-2 text-sm text-orange-800"
            >
              <font-awesome-icon :icon="['fas', 'triangle-exclamation']" class="w-4 h-4 mt-0.5 shrink-0" />
              <span>Contribution suspendue — en cours de vérification par la modération.</span>
            </div>

            <p v-if="secteur.description" class="text-gray-600 leading-relaxed mt-5 whitespace-pre-line">
              {{ secteur.description }}
            </p>

            <!-- Références -->
            <div v-if="secteur.references_utiles" class="mt-5">
              <h2 class="text-sm font-semibold text-gray-700 mb-1">Références utiles</h2>
              <p class="text-sm text-gray-600 whitespace-pre-line">{{ secteur.references_utiles }}</p>
            </div>

            <!-- Contacts + site web -->
            <div
              v-if="secteur.contact_telephone || secteur.contact_courriel || secteur.contact_adresse || secteur.site_web_url"
              class="mt-6 flex flex-col gap-2 text-sm rounded-md bg-gray-50 px-4 py-4"
            >
              <p class="mb-1 text-xs font-medium uppercase tracking-wide text-gray-500">Contacts</p>
              <a v-if="secteur.contact_telephone" :href="`tel:${secteur.contact_telephone}`" class="inline-flex items-center gap-2 text-gray-700 hover:text-custom-green">
                <font-awesome-icon :icon="['fas', 'phone']" class="w-3.5 h-3.5 text-custom-green" />
                {{ secteur.contact_telephone }}
              </a>
              <a v-if="secteur.contact_courriel" :href="`mailto:${secteur.contact_courriel}`" class="inline-flex items-center gap-2 text-gray-700 hover:text-custom-green break-all">
                <font-awesome-icon :icon="['fas', 'envelope']" class="w-3.5 h-3.5 text-custom-green" />
                {{ secteur.contact_courriel }}
              </a>
              <span v-if="secteur.contact_adresse" class="inline-flex items-center gap-2 text-gray-700">
                <font-awesome-icon :icon="['fas', 'location-dot']" class="w-3.5 h-3.5 text-custom-green" />
                {{ secteur.contact_adresse }}
              </span>
              <a
                v-if="secteur.site_web_url"
                :href="secteur.site_web_url"
                target="_blank"
                rel="noopener noreferrer"
                class="inline-flex items-center gap-2 font-medium text-custom-chocolat hover:underline"
              >
                <font-awesome-icon :icon="['fas', 'arrow-up-right-from-square']" class="w-3.5 h-3.5" />
                Visiter le site web
              </a>
            </div>

            <!-- Actions -->
            <div class="flex flex-wrap items-center gap-4 mt-8 pt-5 border-t border-gray-100">
              <NuxtLink
                :to="`/opportunite-afrique/${ficheId}`"
                class="inline-flex items-center gap-1.5 text-sm font-medium text-gray-600 hover:text-custom-green"
              >
                <font-awesome-icon :icon="['fas', 'arrow-left']" class="w-3.5 h-3.5" />
                Retour au territoire
              </NuxtLink>
              <OpportuniteAfriqueContributionSignalerBouton
                type-objet="secteur_developpement"
                :objet-id="secteur.id"
                :libelle="secteur.nom"
                :a-signale="secteur.a_signale"
                :est-authentifie="userStore.isAuthenticated"
                @require-login="navigateTo('/login')"
                @suspendu="secteur.suspendu = true"
              />
            </div>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { useOpportuniteAfrique, type SecteurOpportuniteAPI } from '~/composables/useOpportuniteAfrique'
import { useUserStore } from '~/stores/user'

const route = useRoute()
const userStore = useUserStore()
const { obtenirSecteurOpportunite, resoudreUrlImage } = useOpportuniteAfrique()

const ficheId = route.params.id as string
const secteurId = route.params.secteurId as string

const { data: secteurCharge, pending: chargement } = await useAsyncData(
  `secteur-${ficheId}-${secteurId}`,
  () => obtenirSecteurOpportunite(ficheId, secteurId),
)
const secteur = ref<SecteurOpportuniteAPI | null>(secteurCharge.value)

const breadcrumbs = computed(() => [
  { label: 'Opportunités en Afrique', to: '/opportunite-afrique' },
  { label: 'Territoire', to: `/opportunite-afrique/${ficheId}` },
  { label: secteur.value?.nom || 'Secteur', to: undefined },
])

useHead(() => ({
  title: secteur.value ? `${secteur.value.nom} — Secteur d'opportunité — UAfricas` : "Secteur d'opportunité — UAfricas",
  meta: secteur.value
    ? [{ name: 'description', content: (secteur.value.description || `Secteur ${secteur.value.nom}`).slice(0, 160) }]
    : [],
}))
</script>
