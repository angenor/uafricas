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
        v-if="secteur"
        :titre="secteur.nom"
        :sous-titre="secteur.localite"
        :image="secteur.image_url ? resoudreUrlImage(secteur.image_url) : null"
      >
        <template #badges>
          <span class="inline-flex items-center gap-1.5 rounded-full bg-af-vert px-3 py-1 text-sm font-bold text-white">
            <font-awesome-icon :icon="['fas', 'briefcase']" class="w-3.5 h-3.5" />
            Secteur d'opportunité
          </span>
        </template>
        <template #sous-titre-icon>
          <font-awesome-icon :icon="['fas', 'location-dot']" class="w-4 h-4 text-af-vert" />
        </template>
      </OpportuniteAfriqueDetailHero>
    </template>

    <!-- Panneaux secondaires dans le rail du gabarit : contacts, partage
         et navigation ne disputent plus leur largeur au texte. -->
    <template #rail>
      <template v-if="secteur">
        <!-- Contacts -->
        <section
          v-if="aContact"
          class="rounded-[10px] border border-af-bordure bg-white p-6"
        >
          <h2 class="mb-4 text-xs font-semibold uppercase tracking-wide text-af-atone">Contacts</h2>
          <div class="space-y-3 text-sm">
            <a v-if="secteur.contact_telephone" :href="`tel:${secteur.contact_telephone}`" class="flex items-center gap-3 text-af-corps hover:text-af-vert">
              <span class="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-af-vert/10 text-af-vert">
                <font-awesome-icon :icon="['fas', 'phone']" class="w-4 h-4" />
              </span>
              {{ secteur.contact_telephone }}
            </a>
            <a v-if="secteur.contact_courriel" :href="`mailto:${secteur.contact_courriel}`" class="flex items-center gap-3 break-all text-af-corps hover:text-af-vert">
              <span class="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-af-vert/10 text-af-vert">
                <font-awesome-icon :icon="['fas', 'envelope']" class="w-4 h-4" />
              </span>
              {{ secteur.contact_courriel }}
            </a>
            <div v-if="secteur.contact_adresse" class="flex items-center gap-3 text-af-corps">
              <span class="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-af-vert/10 text-af-vert">
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
            class="mt-4 inline-flex w-full items-center justify-center gap-2 rounded-lg bg-af-chocolat px-4 py-2.5 text-sm font-medium text-white transition hover:bg-af-chocolat/90"
          >
            <font-awesome-icon :icon="['fas', 'arrow-up-right-from-square']" class="w-3.5 h-3.5" />
            Visiter le site web
          </a>
        </section>

        <!-- Réactions & partage -->
        <section class="rounded-[10px] border border-af-bordure bg-white p-6">
          <h2 class="mb-4 text-xs font-semibold uppercase tracking-wide text-af-atone">Ce secteur vous intéresse ?</h2>
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
            class="mt-3 flex w-full items-center justify-center gap-2 rounded-lg bg-af-vert px-4 py-2.5 text-sm font-medium text-white transition hover:bg-af-vert/90 cursor-pointer"
            @click="showPartage = true"
          >
            <font-awesome-icon :icon="['fas', 'share-nodes']" class="w-4 h-4" />
            Partager
          </button>
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
      </template>
    </template>

    <div v-if="chargement" class="flex items-center justify-center py-24">
      <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-af-vert"></div>
    </div>

    <div v-else-if="!secteur" class="flex flex-col items-center justify-center py-24 px-4 text-center">
      <font-awesome-icon :icon="['fas', 'briefcase']" class="w-14 h-14 text-af-atone-2 mb-4" />
      <h1 class="text-2xl font-bold text-af-corps mb-2">Secteur introuvable</h1>
      <p class="text-af-atone mb-4">Ce secteur d'opportunité n'existe pas ou a été retiré.</p>
      <NuxtLink :to="`/opportunite-afrique/${ficheId}`" class="text-af-vert hover:underline">
        &#8592; Retour au territoire
      </NuxtLink>
    </div>

    <template v-else>

      <div class="flex flex-col gap-6">
        <!-- Bandeau de suspension -->
        <div
          v-if="secteur.suspendu"
          class="flex items-start gap-2 rounded-[10px] border border-af-chocolat/20 bg-af-chocolat/5 px-4 py-3 text-sm text-af-chocolat"
        >
          <font-awesome-icon :icon="['fas', 'triangle-exclamation']" class="w-4 h-4 mt-0.5 shrink-0" />
          <span>Contribution suspendue : en cours de vérification par la modération.</span>
        </div>

        <!-- Description -->
        <section v-if="secteur.description" class="rounded-[10px] border border-af-bordure bg-white p-6 sm:p-8">
          <h2 class="mb-4 flex items-center gap-2 font-oswald text-xl font-bold text-af-encre">
            <span class="flex h-8 w-8 items-center justify-center rounded-lg bg-af-vert/10 text-af-vert">
              <font-awesome-icon :icon="['fas', 'circle-info']" class="w-4 h-4" />
            </span>
            Présentation du secteur
          </h2>
          <p class="whitespace-pre-line leading-relaxed text-af-corps">{{ secteur.description }}</p>
        </section>

        <!-- Références utiles -->
        <section v-if="secteur.references_utiles" class="rounded-[10px] border border-af-bordure bg-white p-6 sm:p-8">
          <h2 class="mb-3 flex items-center gap-2 font-oswald text-xl font-bold text-af-encre">
            <span class="flex h-8 w-8 items-center justify-center rounded-lg bg-af-chocolat/10 text-af-chocolat">
              <font-awesome-icon :icon="['fas', 'book']" class="w-4 h-4" />
            </span>
            Références utiles
          </h2>
          <p class="whitespace-pre-line leading-relaxed text-af-corps">{{ secteur.references_utiles }}</p>
        </section>
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
  </NuxtLayout>
</template>

<script setup lang="ts">

definePageMeta({ layout: false })
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

const filAriane = computed(() => [
  { libelle: 'Opportunités en Afrique', vers: '/opportunite-afrique' },
  { libelle: 'Territoire', vers: `/opportunite-afrique/${ficheId}` },
  { libelle: secteur.value?.nom || 'Secteur' },
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
  const titre = `${secteur.value.nom}, Secteur d'opportunité | UAfricas`
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
