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
const ogImageDefault = 'https://www.africans-world.org/images/og-retrouve-amis.png'
const ogImageUrl = computed(() => {
  if (estActif.value) {
    const a = avis.value as AvisPublicDetail
    if (a.photo_url) {
      if (a.photo_url.startsWith('http')) return a.photo_url
      return `https://www.africans-world.org${a.photo_url}`
    }
  }
  return ogImageDefault
})

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
  ogImage: () => ogImageUrl.value,
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
  twitterImage: () => ogImageUrl.value,
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
      class="relative h-56 bg-cover bg-center"
      style="background-image: url('https://images.unsplash.com/photo-1529156069898-49953e39b3ac?ixlib=rb-4.0.3&auto=format&fit=crop&w=1900&q=80')"
    >
      <div class="absolute inset-0 bg-gradient-to-r from-custom-chocolat/90 to-black/70" />
      <div class="absolute inset-0 flex flex-col items-center justify-center pt-14">
        <h1 class="text-white text-2xl md:text-3xl font-bold mb-2">
          Avis de recherche
        </h1>
        <div class="h-1 w-16 bg-custom-green rounded" />
      </div>
    </div>

    <div class="max-w-4xl mx-auto px-4 -mt-6 relative z-10 pb-12">
      <!-- Erreur / non disponible -->
      <div v-if="nonDisponible" class="rounded-2xl bg-white p-14 text-center shadow-lg ring-1 ring-gray-200/60">
        <div class="mx-auto mb-6 flex h-24 w-24 items-center justify-center rounded-full bg-gray-50">
          <font-awesome-icon :icon="['fas', 'circle-xmark']" class="text-4xl text-gray-300" />
        </div>
        <h2 class="text-xl font-semibold text-gray-700 mb-2">
          Avis non disponible
        </h2>
        <p class="text-gray-400 text-sm mb-8">
          Cet avis de recherche n'existe pas ou n'est plus disponible.
        </p>
        <NuxtLink
          to="/retrouve-amis"
          class="inline-flex items-center gap-2 rounded-xl bg-custom-chocolat px-6 py-3 text-sm font-semibold text-white shadow-sm transition-all hover:bg-amber-800 hover:shadow-md"
        >
          <font-awesome-icon :icon="['fas', 'arrow-left']" />
          Retour a Retrouv'Amis
        </NuxtLink>
      </div>

      <!-- Avis non actif (cloture / suspendu) -->
      <div v-else-if="estNonActif" class="rounded-2xl bg-white p-14 text-center shadow-lg ring-1 ring-gray-200/60">
        <div
          class="mx-auto mb-6 flex h-24 w-24 items-center justify-center rounded-full"
          :class="(avis as AvisPublicEtat).etat === 'cloture'
            ? 'bg-green-50'
            : 'bg-orange-50'"
        >
          <font-awesome-icon
            :icon="(avis as AvisPublicEtat).etat === 'cloture'
              ? ['fas', 'heart']
              : ['fas', 'shield-halved']"
            class="text-4xl"
            :class="(avis as AvisPublicEtat).etat === 'cloture' ? 'text-custom-green' : 'text-orange-400'"
          />
        </div>
        <h2 class="text-xl font-semibold text-gray-700 mb-2">
          {{ (avis as AvisPublicEtat).message }}
        </h2>
        <p v-if="(avis as AvisPublicEtat).etat === 'cloture'" class="text-gray-400 text-sm mb-8">
          L'auteur de cet avis a retrouve la personne qu'il recherchait.
        </p>
        <p v-else class="text-gray-400 text-sm mb-8">
          Un examen de cet avis est en cours. Veuillez reessayer plus tard.
        </p>
        <NuxtLink
          to="/retrouve-amis"
          class="inline-flex items-center gap-2 rounded-xl bg-custom-chocolat px-6 py-3 text-sm font-semibold text-white shadow-sm transition-all hover:bg-amber-800 hover:shadow-md"
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
          class="mt-6 rounded-2xl bg-linear-to-br from-amber-50 to-orange-50 p-8 text-center ring-1 ring-amber-200/60"
        >
          <div class="mx-auto mb-4 flex h-14 w-14 items-center justify-center rounded-full bg-amber-100">
            <font-awesome-icon :icon="['fas', 'user-lock']" class="text-xl text-amber-600" />
          </div>
          <p class="text-lg font-semibold text-gray-800 mb-1">
            Vous connaissez cette personne ?
          </p>
          <p class="text-gray-500 text-sm mb-6 max-w-md mx-auto">
            Connectez-vous pour contacter l'auteur de cet avis et l'aider a retrouver la personne recherchee.
          </p>
          <NuxtLink
            :to="`/login?redirect=${encodeURIComponent(`/retrouve-amis/public/${slug}`)}`"
            class="inline-flex items-center gap-2 rounded-xl bg-custom-chocolat px-6 py-3 text-sm font-semibold text-white shadow-sm transition-all hover:bg-amber-800 hover:shadow-md"
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
