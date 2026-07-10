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
      <!-- Hero immersif -->
      <OpportuniteAfriqueDetailHero
        :titre="personnalite.nom_complet"
        :sous-titre="periodeVie"
        :image="personnalite.portrait_url ? resoudreUrlImage(personnalite.portrait_url) : null"
        :breadcrumbs="breadcrumbs"
        position-image="object-top"
        degrade-fond="bg-gradient-to-br from-custom-chocolat to-amber-900"
      >
        <template #badges>
          <span class="inline-flex items-center gap-1.5 rounded-full bg-custom-green px-3 py-1 text-sm font-medium text-white shadow-sm">
            <font-awesome-icon :icon="['fas', 'user']" class="w-3.5 h-3.5" />
            {{ labelDomaine(personnalite.domaine) }}
          </span>
        </template>
        <template #sous-titre-icon>
          <font-awesome-icon :icon="['fas', 'calendar']" class="w-4 h-4 text-white/60" />
        </template>
      </OpportuniteAfriqueDetailHero>

      <!-- Corps chevauchant le hero -->
      <div class="relative z-10 mx-auto -mt-10 max-w-5xl px-4 pb-16 sm:px-6 lg:px-8">
        <div class="grid grid-cols-1 gap-6 lg:grid-cols-3">
          <!-- Colonne principale -->
          <div class="space-y-6 lg:col-span-2">
            <!-- Bandeau de suspension -->
            <div
              v-if="personnalite.suspendu"
              class="flex items-start gap-2 rounded-xl border border-orange-200 bg-orange-50 px-4 py-3 text-sm text-orange-800 shadow-sm"
            >
              <font-awesome-icon :icon="['fas', 'triangle-exclamation']" class="w-4 h-4 mt-0.5 shrink-0" />
              <span>Contribution suspendue — en cours de vérification par la modération.</span>
            </div>

            <!-- Biographie -->
            <section class="rounded-2xl border border-gray-100 bg-white p-6 shadow-sm sm:p-8">
              <h2 class="mb-4 flex items-center gap-2 font-oswald text-xl font-bold text-gray-900">
                <span class="flex h-8 w-8 items-center justify-center rounded-lg bg-custom-chocolat/10 text-custom-chocolat">
                  <font-awesome-icon :icon="['fas', 'book-open']" class="w-4 h-4" />
                </span>
                Biographie
              </h2>
              <p class="whitespace-pre-line leading-relaxed text-gray-600">{{ personnalite.biographie_courte }}</p>

              <a
                v-if="personnalite.lien_reference"
                :href="personnalite.lien_reference"
                target="_blank"
                rel="noopener noreferrer"
                class="mt-6 inline-flex items-center gap-2 text-sm font-medium text-custom-chocolat hover:underline"
              >
                <font-awesome-icon :icon="['fas', 'arrow-up-right-from-square']" class="w-3.5 h-3.5" />
                En savoir plus
              </a>
            </section>
          </div>

          <!-- Sidebar -->
          <aside class="space-y-6">
            <!-- Fiche d'identité -->
            <section class="rounded-2xl border border-gray-100 bg-white p-6 shadow-sm">
              <div class="mx-auto -mt-16 h-28 w-28 overflow-hidden rounded-2xl border-4 border-white bg-gray-100 shadow-md">
                <img
                  v-if="personnalite.portrait_url"
                  :src="resoudreUrlImage(personnalite.portrait_url)"
                  :alt="personnalite.nom_complet"
                  class="h-full w-full object-cover"
                />
                <div
                  v-else
                  class="flex h-full w-full items-center justify-center bg-gradient-to-br from-custom-chocolat/20 to-custom-green/20 font-oswald text-4xl font-bold text-custom-chocolat"
                >
                  {{ initiales }}
                </div>
              </div>

              <dl class="mt-4 space-y-3 text-sm">
                <div class="flex items-center justify-between border-b border-gray-100 pb-3">
                  <dt class="text-gray-500">Domaine</dt>
                  <dd class="font-medium text-gray-900">{{ labelDomaine(personnalite.domaine) }}</dd>
                </div>
                <div v-if="personnalite.annee_naissance" class="flex items-center justify-between border-b border-gray-100 pb-3">
                  <dt class="text-gray-500">Naissance</dt>
                  <dd class="font-medium text-gray-900">{{ personnalite.annee_naissance }}</dd>
                </div>
                <div v-if="personnalite.annee_deces" class="flex items-center justify-between">
                  <dt class="text-gray-500">Décès</dt>
                  <dd class="font-medium text-gray-900">{{ personnalite.annee_deces }}</dd>
                </div>
              </dl>
            </section>

            <!-- Réactions & partage -->
            <section class="rounded-2xl border border-gray-100 bg-white p-6 shadow-sm">
              <h2 class="mb-4 text-xs font-semibold uppercase tracking-wide text-gray-500">Cette personnalité vous inspire ?</h2>
              <OpportuniteAfriqueReactionsBar
                type-objet="personnalite_connue"
                :objet-id="personnalite.id"
                :nombre-likes="personnalite.nombre_likes"
                :nombre-dislikes="personnalite.nombre_dislikes"
                :ma-reaction="personnalite.ma_reaction"
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
                  type-objet="personnalite_connue"
                  :objet-id="personnalite.id"
                  :libelle="personnalite.nom_complet"
                  :a-signale="personnalite.a_signale"
                  :est-authentifie="userStore.isAuthenticated"
                  @require-login="redirigerVersConnexion()"
                  @suspendu="personnalite.suspendu = true"
                />
              </div>
            </section>
          </aside>
        </div>
      </div>

      <!-- Modal partage (réseaux sociaux + mur communautaire) -->
      <OpportuniteAfriquePartagerElementModal
        :is-open="showPartage"
        :titre="personnalite.nom_complet"
        type-label="la personnalité"
        type-objet="personnalite_connue"
        :objet-id="personnalite.id"
        :est-connecte="userStore.isAuthenticated"
        @close="showPartage = false"
      />
    </template>
  </div>
</template>

<script setup lang="ts">
import { useOpportuniteAfrique, type PersonnaliteConnueAPI, type DomainePersonnalite } from '~/composables/useOpportuniteAfrique'
import { useUserStore } from '~/stores/user'

const route = useRoute()
const userStore = useUserStore()
const { redirigerVersConnexion } = useAuth()
const { obtenirPersonnalite, resoudreUrlImage } = useOpportuniteAfrique()

const showPartage = ref(false)

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

const periodeVie = computed(() => {
  const p = personnalite.value
  if (!p) return null
  if (!p.annee_naissance && !p.annee_deces) return null
  return `${p.annee_naissance ?? '?'} — ${p.annee_deces ?? 'présent'}`
})

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

// ── SEO / Open Graph (aperçu lors du partage sur les réseaux sociaux) ──
const requete = useRequestURL()
const urlCanonique = `${requete.protocol}//${requete.host}/opportunite-afrique/${ficheId}/personnalites/${personnaliteId}`
const imageOg = computed(() => (personnalite.value?.portrait_url ? resoudreUrlImage(personnalite.value.portrait_url) : ''))
const descriptionOg = computed(() => (personnalite.value?.biographie_courte ?? '').slice(0, 200))

useHead(() => {
  if (!personnalite.value) return {}
  const titre = `${personnalite.value.nom_complet} — Personnalité — UAfricas`
  return {
    title: titre,
    meta: [
      { name: 'description', content: descriptionOg.value },
      { property: 'og:type', content: 'profile' },
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
