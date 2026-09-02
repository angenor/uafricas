<script setup lang="ts">
import type { AvisPublicDetail, AvisPublicEtat } from '~/composables/useRetrouvAmis'

definePageMeta({ layout: false })

const route = useRoute()
const config = useRuntimeConfig()
const slug = route.params.slug as string
const userStore = useUserStore()

// En SSR, utiliser l'URL interne du backend (Docker) si configuree, sinon l'URL publique
// En client, utiliser l'URL publique (relative ou absolue selon l'env)
const apiBase = import.meta.server
  ? ((config as any).ssrApiBaseUrl || config.public.apiBaseUrl || '')
  : (config.public.apiBaseUrl as string)

// Charger les donnees cote serveur via useFetch (SSR), endpoint public sans auth
const { data, error: fetchError } = await useFetch<{ success: boolean; data: AvisPublicDetail | AvisPublicEtat | null; error: string | null }>(
  `/api/retrouve-amis/public/${slug}`,
  { baseURL: apiBase },
)

const avis = computed(() => data.value?.data ?? null)
const estActif = computed(() => avis.value && 'auteur_anonyme' in avis.value)
const estNonActif = computed(() => avis.value && 'message' in avis.value && !('auteur_anonyme' in avis.value))
const nonDisponible = computed(() => fetchError.value || !data.value?.success)

// SEO : balises completes (Open Graph + Twitter Card pour apercu riche)
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
      return `Recherche : ${a.nom_recherche}${a.prenom_recherche ? ' ' + a.prenom_recherche : ''} | AfricanS`
    }
    return 'Avis de recherche | AfricanS'
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
    return 'Avis de recherche sur AfricanS, Retrouver des amis perdus de vue.'
  },
  ogTitle: () => {
    if (estActif.value) {
      const a = avis.value as AvisPublicDetail
      return `Recherche : ${a.nom_recherche}${a.prenom_recherche ? ' ' + a.prenom_recherche : ''}`
    }
    return 'Avis de recherche | AfricanS'
  },
  ogDescription: () => {
    if (estActif.value) {
      const a = avis.value as AvisPublicDetail
      return `Aidez a retrouver ${a.nom_recherche}${a.prenom_recherche ? ' ' + a.prenom_recherche : ''} sur AfricanS.`
    }
    return 'Retrouvez des amis perdus de vue sur AfricanS.'
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
    return 'Avis de recherche | AfricanS'
  },
  twitterDescription: () => {
    if (estActif.value) {
      const a = avis.value as AvisPublicDetail
      return `Aidez a retrouver ${a.nom_recherche}${a.prenom_recherche ? ' ' + a.prenom_recherche : ''} sur AfricanS.`
    }
    return 'Retrouvez des amis perdus de vue sur AfricanS.'
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
  <NuxtLayout name="africans">
    <template #bandeau>
      <!-- L'image était hotlinkée sur Unsplash ; le hero local du module
           existait déjà. -->
      <AfricansBandeauModule
        titre="Avis de recherche"
        sous-titre="Aidez à retrouver une personne perdue de vue."
        image="/images/africans/heros/hero-africonnect.jpg"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[
          { libelle: 'Opafrica', vers: '/retrouve-amis' },
          { libelle: 'Africonnect', vers: '/retrouve-amis' },
          { libelle: 'Avis de recherche' },
        ]"
      />
    </template>

    <!-- Page PUBLIQUE : pas de rail de section. Un visiteur non connecté n'a
         ni avis, ni correspondances, ni statistiques à y voir. -->
    <div class="min-w-0">
      <!-- Erreur / non disponible -->
      <div v-if="nonDisponible" class="rounded-[10px] bg-white p-14 text-center border border-af-bordure">
        <div class="mx-auto mb-6 flex h-24 w-24 items-center justify-center rounded-full bg-af-fond">
          <font-awesome-icon :icon="['fas', 'circle-xmark']" class="text-4xl text-af-atone-2" />
        </div>
        <h2 class="text-xl font-semibold text-af-corps mb-2">
          Avis non disponible
        </h2>
        <p class="text-af-atone-2 text-sm mb-8">
          Cet avis de recherche n'existe pas ou n'est plus disponible.
        </p>
        <NuxtLink
          to="/retrouve-amis"
          class="inline-flex items-center gap-2 rounded-lg bg-af-chocolat px-6 py-3 text-sm font-semibold text-white shadow-sm transition-all hover:opacity-90 hover:shadow-md"
        >
          <font-awesome-icon :icon="['fas', 'arrow-left']" />
          Retour a Retrouv'Amis
        </NuxtLink>
      </div>

      <!-- Avis non actif (cloture / suspendu) -->
      <div v-else-if="estNonActif" class="rounded-[10px] bg-white p-14 text-center border border-af-bordure">
        <div
          class="mx-auto mb-6 flex h-24 w-24 items-center justify-center rounded-full"
          :class="(avis as AvisPublicEtat).etat === 'cloture'
            ? 'bg-af-vert/5'
            : 'bg-af-chocolat/5'"
        >
          <font-awesome-icon
            :icon="(avis as AvisPublicEtat).etat === 'cloture'
              ? ['fas', 'heart']
              : ['fas', 'shield-halved']"
            class="text-4xl"
            :class="(avis as AvisPublicEtat).etat === 'cloture' ? 'text-af-vert' : 'text-af-chocolat'"
          />
        </div>
        <h2 class="text-xl font-semibold text-af-corps mb-2">
          {{ (avis as AvisPublicEtat).message }}
        </h2>
        <p v-if="(avis as AvisPublicEtat).etat === 'cloture'" class="text-af-atone-2 text-sm mb-8">
          L'auteur de cet avis a retrouve la personne qu'il recherchait.
        </p>
        <p v-else class="text-af-atone-2 text-sm mb-8">
          Un examen de cet avis est en cours. Veuillez reessayer plus tard.
        </p>
        <NuxtLink
          to="/retrouve-amis"
          class="inline-flex items-center gap-2 rounded-lg bg-af-chocolat px-6 py-3 text-sm font-semibold text-white shadow-sm transition-all hover:opacity-90 hover:shadow-md"
        >
          <font-awesome-icon :icon="['fas', 'arrow-left']" />
          Retour a Retrouv'Amis
        </NuxtLink>
      </div>

      <!-- Avis actif : contenu complet -->
      <template v-else-if="estActif">
        <!-- L'avis d'abord, la réponse ensuite : on ne répond pas à ce qu'on
             n'a pas encore lu, et l'avis lui-même annonce « Répondez à cet
             avis CI-DESSOUS » — promesse que l'ordre inverse démentait. -->
        <RetrouveAmisPagePublique
          :avis="(avis as AvisPublicDetail)"
          :peut-repondre="userStore.user?.id !== (avis as AvisPublicDetail).auteur_id"
        />

        <!-- CTA connexion pour visiteurs non connectes -->
        <div
          v-if="!userStore.isAuthenticated"
          class="mt-6 rounded-[10px] border border-af-chocolat/20 bg-af-chocolat/5 p-8 text-center"
        >
          <div class="mx-auto mb-4 flex h-14 w-14 items-center justify-center rounded-full bg-af-chocolat/10">
            <font-awesome-icon :icon="['fas', 'user-lock']" class="text-xl text-af-chocolat" />
          </div>
          <p class="text-lg font-semibold text-af-encre mb-1">
            Vous connaissez cette personne ?
          </p>
          <p class="text-af-atone text-sm mb-6 max-w-md mx-auto">
            Connectez-vous pour contacter l'auteur de cet avis et l'aider à retrouver la personne recherchée.
          </p>
          <NuxtLink
            :to="`/login?redirect=${encodeURIComponent(`/retrouve-amis/public/${slug}`)}`"
            class="inline-flex items-center gap-2 rounded-lg bg-af-chocolat px-6 py-3 text-sm font-semibold text-white shadow-sm transition-all hover:opacity-90 hover:shadow-md"
          >
            <font-awesome-icon :icon="['fas', 'right-to-bracket']" />
            Se connecter pour répondre
          </NuxtLink>
        </div>

        <RetrouveAmisFormulaireReponse
          v-else-if="userStore.user?.id !== (avis as AvisPublicDetail).auteur_id"
          class="mt-6"
          :slug="slug"
          :auteur-id="(avis as AvisPublicDetail).auteur_id"
        />

        <!-- L'auteur ne voit ni invitation ni formulaire, à juste titre. Sans
             ce repère il ne saurait pas OÙ les réponses lui parviennent : la
             page publique est souvent le seul écran du module qu'il rouvre. -->
        <p
          v-if="userStore.isAuthenticated && userStore.user?.id === (avis as AvisPublicDetail).auteur_id"
          class="mt-6 flex flex-wrap items-center justify-center gap-2 rounded-[10px] border border-af-bordure bg-af-fond px-5 py-4 text-[14px]/[1.4] text-af-corps"
        >
          <font-awesome-icon icon="fa-solid fa-handshake" class="text-af-chocolat" />
          Vous êtes l'auteur de cet avis. Les réponses vous parviennent dans
          <NuxtLink to="/retrouve-amis/correspondances" class="font-bold text-af-chocolat underline">
            vos correspondances
          </NuxtLink>.
        </p>

        <RetrouveAmisBoutonsPartage
          :slug="slug"
          :avis-id="(avis as AvisPublicDetail).id"
          :compteur-partages="(avis as AvisPublicDetail).compteur_partages"
          :nom-recherche="(avis as AvisPublicDetail).nom_recherche"
          :prenom-recherche="(avis as AvisPublicDetail).prenom_recherche"
        />

        <RetrouveAmisDemandeRetrait
          v-if="userStore.isAuthenticated && userStore.user?.id !== (avis as AvisPublicDetail).auteur_id"
          :slug="slug"
          @suspendu="$router.go(0)"
        />
      </template>
    </div>
  </NuxtLayout>
</template>
