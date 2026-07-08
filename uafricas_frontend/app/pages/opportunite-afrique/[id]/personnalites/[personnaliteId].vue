<template>
  <div class="min-h-screen bg-gray-50">
    <div v-if="chargement" class="flex items-center justify-center h-screen">
      <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-custom-chocolat"></div>
    </div>

    <div v-else-if="!personnalite" class="flex flex-col items-center justify-center h-screen px-4 text-center">
      <font-awesome-icon :icon="['fas', 'user']" class="w-14 h-14 text-gray-300 mb-4" />
      <h1 class="text-2xl font-bold text-gray-700 mb-2">Personnalité introuvable</h1>
      <p class="text-gray-500 mb-4">Cette personnalité n'existe pas ou a été retirée.</p>
      <NuxtLink :to="`/opportunite-afrique/${ficheId}`" class="text-custom-chocolat hover:underline">
        &#8592; Retour au territoire
      </NuxtLink>
    </div>

    <template v-else>
      <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <CommonBreadcrumbNav class="mb-6" :custom-breadcrumbs="breadcrumbs" />

        <div class="bg-white rounded-2xl shadow-sm overflow-hidden">
          <div class="sm:flex">
            <!-- Portrait -->
            <div class="sm:w-72 shrink-0 aspect-square sm:aspect-auto bg-gray-100 relative overflow-hidden">
              <img
                v-if="personnalite.portrait_url"
                :src="resoudreUrlImage(personnalite.portrait_url)"
                :alt="personnalite.nom_complet"
                class="w-full h-full object-cover"
              />
              <div
                v-else
                class="w-full h-full min-h-64 flex items-center justify-center bg-gradient-to-br from-custom-chocolat/20 to-custom-green/20 font-oswald text-6xl font-bold text-custom-chocolat"
              >
                {{ initiales }}
              </div>
            </div>

            <!-- Infos -->
            <div class="p-6 sm:p-8 flex-1">
              <h1 class="font-oswald text-3xl sm:text-4xl font-bold text-gray-900">{{ personnalite.nom_complet }}</h1>

              <span class="inline-block mt-2 px-2.5 py-0.5 text-sm font-medium bg-custom-green/10 text-custom-green rounded-full">
                {{ labelDomaine(personnalite.domaine) }}
              </span>

              <p v-if="personnalite.annee_naissance || personnalite.annee_deces" class="text-sm text-gray-500 mt-3">
                {{ personnalite.annee_naissance ?? '?' }} — {{ personnalite.annee_deces ?? '' }}
              </p>

              <!-- Bandeau de suspension -->
              <div
                v-if="personnalite.suspendu"
                class="mt-4 flex items-start gap-2 rounded-md bg-orange-50 border border-orange-200 px-3 py-2 text-sm text-orange-800"
              >
                <font-awesome-icon :icon="['fas', 'triangle-exclamation']" class="w-4 h-4 mt-0.5 shrink-0" />
                <span>Contribution suspendue — en cours de vérification par la modération.</span>
              </div>

              <p class="text-gray-600 leading-relaxed mt-5 whitespace-pre-line">
                {{ personnalite.biographie_courte }}
              </p>

              <a
                v-if="personnalite.lien_reference"
                :href="personnalite.lien_reference"
                target="_blank"
                rel="noopener noreferrer"
                class="inline-flex items-center gap-2 mt-5 text-sm font-medium text-custom-chocolat hover:underline"
              >
                <font-awesome-icon :icon="['fas', 'arrow-up-right-from-square']" class="w-3.5 h-3.5" />
                En savoir plus
              </a>

              <!-- Actions -->
              <div class="flex flex-wrap items-center gap-4 mt-8 pt-5 border-t border-gray-100">
                <NuxtLink
                  :to="`/opportunite-afrique/${ficheId}`"
                  class="inline-flex items-center gap-1.5 text-sm font-medium text-gray-600 hover:text-custom-chocolat"
                >
                  <font-awesome-icon :icon="['fas', 'arrow-left']" class="w-3.5 h-3.5" />
                  Retour au territoire
                </NuxtLink>
                <OpportuniteAfriqueContributionSignalerBouton
                  type-objet="personnalite_connue"
                  :objet-id="personnalite.id"
                  :libelle="personnalite.nom_complet"
                  :a-signale="personnalite.a_signale"
                  :est-authentifie="userStore.isAuthenticated"
                  @require-login="navigateTo('/login')"
                  @suspendu="personnalite.suspendu = true"
                />
              </div>
            </div>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { useOpportuniteAfrique, type PersonnaliteConnueAPI, type DomainePersonnalite } from '~/composables/useOpportuniteAfrique'
import { useUserStore } from '~/stores/user'

const route = useRoute()
const userStore = useUserStore()
const { obtenirPersonnalite, resoudreUrlImage } = useOpportuniteAfrique()

const ficheId = route.params.id as string
const personnaliteId = route.params.personnaliteId as string

const { data: persoCharge, pending: chargement } = await useAsyncData(
  `personnalite-${ficheId}-${personnaliteId}`,
  () => obtenirPersonnalite(ficheId, personnaliteId),
)
const personnalite = ref<PersonnaliteConnueAPI | null>(persoCharge.value)

const LABELS_DOMAINE: Record<DomainePersonnalite, string> = {
  politique: 'Politique',
  artiste_musicien: 'Artiste musicien',
  artiste_autre: 'Artiste (autre)',
  sportif: 'Sportif',
  entrepreneur: 'Entrepreneur',
  scientifique: 'Scientifique',
  militaire_historique: 'Militaire / Historique',
  autre: 'Autre',
}
const labelDomaine = (d: DomainePersonnalite): string => LABELS_DOMAINE[d] ?? d

const initiales = computed(() => {
  const nom = personnalite.value?.nom_complet ?? ''
  const parts = nom.trim().split(/\s+/)
  const first = parts[0]?.charAt(0) ?? ''
  const last = parts.length > 1 ? (parts[parts.length - 1]?.charAt(0) ?? '') : ''
  return `${first}${last}`.toUpperCase()
})

const breadcrumbs = computed(() => [
  { label: 'Opportunités en Afrique', to: '/opportunite-afrique' },
  { label: 'Territoire', to: `/opportunite-afrique/${ficheId}` },
  { label: personnalite.value?.nom_complet || 'Personnalité', to: undefined },
])

useHead(() => ({
  title: personnalite.value ? `${personnalite.value.nom_complet} — Personnalité — UAfricas` : 'Personnalité — UAfricas',
  meta: personnalite.value
    ? [{ name: 'description', content: personnalite.value.biographie_courte.slice(0, 160) }]
    : [],
}))
</script>
