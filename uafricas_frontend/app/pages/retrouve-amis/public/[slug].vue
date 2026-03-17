<script setup lang="ts">
import type { AvisPublicDetail, AvisPublicEtat } from '~/composables/useRetrouvAmis'

definePageMeta({ layout: 'default' })

const route = useRoute()
const config = useRuntimeConfig()
const slug = route.params.slug as string
const userStore = useUserStore()

// En SSR, utiliser l'URL interne du backend (Docker) si configuree, sinon l'URL publique
// En client, utiliser l'URL publique (relative ou absolue selon l'env)
const apiBase = import.meta.server
  ? ((config as any).ssrApiBaseUrl || config.public.apiBaseUrl || '')
  : (config.public.apiBaseUrl as string)

// Charger les donnees cote serveur via useFetch (SSR) — endpoint public sans auth
const { data, error: fetchError } = await useFetch<{ success: boolean; data: AvisPublicDetail | AvisPublicEtat | null; error: string | null }>(
  `/api/retrouve-amis/public/${slug}`,
  { baseURL: apiBase },
)

const avis = computed(() => data.value?.data ?? null)
const estActif = computed(() => avis.value && 'auteur_anonyme' in avis.value)
const estNonActif = computed(() => avis.value && 'message' in avis.value && !('auteur_anonyme' in avis.value))
const nonDisponible = computed(() => fetchError.value || !data.value?.success)

// SEO — balises completes (Open Graph + Twitter Card pour apercu riche)
const ogImageUrl = 'https://www.africans-world.org/images/og-retrouve-amis.png'

useSeoMeta({
  title: () => {
    if (estActif.value) {
      const a = avis.value as AvisPublicDetail
      return `Recherche : ${a.nom_recherche}${a.prenom_recherche ? ' ' + a.prenom_recherche : ''} — UAfricas`
    }
    return 'Avis de recherche — UAfricas'
  },
  description: () => {
    if (estActif.value) {
      const a = avis.value as AvisPublicDetail
      const parties = []
      if (a.ville) parties.push(a.ville)
      if (a.pays) parties.push(a.pays.nom)
      if (a.ecole) parties.push(a.ecole)
      return `Aidez ${a.auteur_anonyme} a retrouver ${a.nom_recherche}${a.prenom_recherche ? ' ' + a.prenom_recherche : ''}${parties.length ? ' (' + parties.join(', ') + ')' : ''}. Partagez cet avis de recherche.`
    }
    return 'Avis de recherche sur UAfricas — Retrouver des amis perdus de vue.'
  },
  ogTitle: () => {
    if (estActif.value) {
      const a = avis.value as AvisPublicDetail
      return `Recherche : ${a.nom_recherche}${a.prenom_recherche ? ' ' + a.prenom_recherche : ''}`
    }
    return 'Avis de recherche — UAfricas'
  },
  ogDescription: () => {
    if (estActif.value) {
      const a = avis.value as AvisPublicDetail
      return `Aidez a retrouver ${a.nom_recherche}${a.prenom_recherche ? ' ' + a.prenom_recherche : ''} sur UAfricas.`
    }
    return 'Retrouvez des amis perdus de vue sur UAfricas.'
  },
  ogType: 'article',
  ogUrl: () => `https://www.africans-world.org/retrouve-amis/public/${slug}`,
  ogImage: ogImageUrl,
  twitterCard: 'summary_large_image',
  twitterTitle: () => {
    if (estActif.value) {
      const a = avis.value as AvisPublicDetail
      return `Recherche : ${a.nom_recherche}${a.prenom_recherche ? ' ' + a.prenom_recherche : ''}`
    }
    return 'Avis de recherche — UAfricas'
  },
  twitterDescription: () => {
    if (estActif.value) {
      const a = avis.value as AvisPublicDetail
      return `Aidez a retrouver ${a.nom_recherche}${a.prenom_recherche ? ' ' + a.prenom_recherche : ''} sur UAfricas.`
    }
    return 'Retrouvez des amis perdus de vue sur UAfricas.'
  },
  twitterImage: ogImageUrl,
})

// noindex/nofollow pour les pages non-actives
useHead({
  meta: () => {
    if (!estActif.value) {
      return [{ name: 'robots', content: 'noindex, nofollow' }]
    }
    return []
  },
})
</script>

<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Hero compact -->
    <div
      class="relative h-48 bg-cover bg-center"
      style="background-image: url('https://images.unsplash.com/photo-1529156069898-49953e39b3ac?ixlib=rb-4.0.3&auto=format&fit=crop&w=1900&q=80')"
    >
      <div class="absolute inset-0 bg-gradient-to-r from-custom-chocolat/90 to-black/70" />
      <div class="absolute inset-0 flex flex-col items-center justify-center mt-10">
        <h1 class="text-white text-2xl md:text-3xl font-bold mb-2">
          Avis de recherche
        </h1>
        <div class="h-1 w-16 bg-custom-green rounded" />
      </div>
    </div>

    <div class="max-w-4xl mx-auto px-4 py-8">
      <!-- Erreur / non disponible -->
      <div v-if="nonDisponible" class="bg-white rounded-xl shadow-sm border border-gray-200 p-12 text-center">
        <div class="w-20 h-20 mx-auto mb-6 bg-gray-100 text-gray-400 rounded-full flex items-center justify-center">
          <font-awesome-icon :icon="['fas', 'circle-xmark']" class="text-3xl" />
        </div>
        <h2 class="text-xl font-semibold text-gray-700 mb-2">
          Avis non disponible
        </h2>
        <p class="text-gray-500 mb-6">
          Cet avis de recherche n'existe pas ou n'est plus disponible.
        </p>
        <NuxtLink
          to="/retrouve-amis"
          class="inline-flex items-center gap-2 px-6 py-3 bg-amber-700 text-white font-semibold rounded-lg hover:bg-amber-800 transition-colors"
        >
          <font-awesome-icon :icon="['fas', 'arrow-left']" />
          Retour a Retrouv'Amis
        </NuxtLink>
      </div>

      <!-- Avis non actif (cloture / suspendu) -->
      <div v-else-if="estNonActif" class="bg-white rounded-xl shadow-sm border border-gray-200 p-12 text-center">
        <div class="w-20 h-20 mx-auto mb-6 rounded-full flex items-center justify-center"
          :class="(avis as AvisPublicEtat).etat === 'cloture'
            ? 'bg-green-50 text-custom-green'
            : 'bg-orange-50 text-orange-500'"
        >
          <font-awesome-icon
            :icon="(avis as AvisPublicEtat).etat === 'cloture'
              ? ['fas', 'heart']
              : ['fas', 'shield-halved']"
            class="text-3xl"
          />
        </div>
        <h2 class="text-xl font-semibold text-gray-700 mb-2">
          {{ (avis as AvisPublicEtat).message }}
        </h2>
        <p v-if="(avis as AvisPublicEtat).etat === 'cloture'" class="text-gray-500 mb-6">
          L'auteur de cet avis a retrouve la personne qu'il recherchait.
        </p>
        <p v-else class="text-gray-500 mb-6">
          Un examen de cet avis est en cours. Veuillez reessayer plus tard.
        </p>
        <NuxtLink
          to="/retrouve-amis"
          class="inline-flex items-center gap-2 px-6 py-3 bg-amber-700 text-white font-semibold rounded-lg hover:bg-amber-800 transition-colors"
        >
          <font-awesome-icon :icon="['fas', 'arrow-left']" />
          Retour a Retrouv'Amis
        </NuxtLink>
      </div>

      <!-- Avis actif : contenu complet -->
      <template v-else-if="estActif">
        <RetrouveAmisPagePublique :avis="(avis as AvisPublicDetail)" />
        <RetrouveAmisBoutonsPartage
          :slug="slug"
          :compteur-partages="(avis as AvisPublicDetail).compteur_partages"
          :nom-recherche="(avis as AvisPublicDetail).nom_recherche"
          :prenom-recherche="(avis as AvisPublicDetail).prenom_recherche"
        />
        <!-- CTA connexion pour visiteurs non connectes -->
        <div
          v-if="!userStore.isAuthenticated"
          class="mt-6 bg-amber-50 border border-amber-200 rounded-xl p-6 text-center"
        >
          <font-awesome-icon :icon="['fas', 'user-lock']" class="text-amber-600 text-2xl mb-3" />
          <p class="text-gray-800 font-medium mb-2">
            Vous connaissez cette personne ?
          </p>
          <p class="text-gray-600 text-sm mb-4">
            Connectez-vous pour contacter l'auteur de cet avis et l'aider a retrouver la personne recherchee.
          </p>
          <NuxtLink
            :to="`/login?redirect=${encodeURIComponent(`/retrouve-amis/public/${slug}`)}`"
            class="inline-flex items-center gap-2 px-6 py-3 bg-amber-700 text-white font-semibold rounded-lg hover:bg-amber-800 transition-colors"
          >
            <font-awesome-icon :icon="['fas', 'right-to-bracket']" />
            Se connecter pour repondre
          </NuxtLink>
        </div>

        <RetrouveAmisFormulaireReponse
          v-else-if="userStore.user?.id !== (avis as AvisPublicDetail).auteur_id"
          :slug="slug"
          :auteur-id="(avis as AvisPublicDetail).auteur_id"
        />
        <RetrouveAmisDemandeRetrait
          v-if="userStore.isAuthenticated && userStore.user?.id !== (avis as AvisPublicDetail).auteur_id"
          :slug="slug"
          @suspendu="$router.go(0)"
        />
      </template>
    </div>
  </div>
</template>
