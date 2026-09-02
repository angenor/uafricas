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
        v-if="personnalite"
        :titre="personnalite.nom_complet"
        :sous-titre="periodeVie"
        :portrait="personnalite.portrait_url ? resoudreUrlImage(personnalite.portrait_url) : null"
      >
        <template #badges>
          <span class="inline-flex items-center gap-1.5 rounded-full bg-af-vert px-3 py-1 text-sm font-bold text-white">
            <font-awesome-icon :icon="['fas', 'user']" class="w-3.5 h-3.5" />
            {{ labelDomaine(personnalite.domaine) }}
          </span>
        </template>
        <template #sous-titre-icon>
          <font-awesome-icon :icon="['fas', 'calendar']" class="w-4 h-4 text-white/60" />
        </template>
      </OpportuniteAfriqueDetailHero>
    </template>

    <!-- Panneaux secondaires dans le rail du gabarit : contacts, partage
         et navigation ne disputent plus leur largeur au texte. -->
    <template #rail>
      <template v-if="personnalite">
        <!-- Fiche d'identité -->
        <section class="rounded-[10px] border border-af-bordure bg-white p-6">
          <!-- Le `-mt-16` d'origine faisait déborder le médaillon sur le hero,
               qu'il surplombait. Dans le rail, il le faisait dépasser du haut
               de sa propre carte, où il se coupait. Il porte aussi le portrait
               en mobile, où celui du bandeau est masqué. -->
          <div class="mx-auto size-28 overflow-hidden rounded-[10px] border border-af-bordure bg-af-fond">
            <img
              v-if="personnalite.portrait_url"
              :src="resoudreUrlImage(personnalite.portrait_url)"
              :alt="personnalite.nom_complet"
              class="h-full w-full object-cover"
            />
            <div
              v-else
              class="flex h-full w-full items-center justify-center bg-gradient-to-br from-af-chocolat/20 to-af-vert/20 font-oswald text-4xl font-bold text-af-chocolat"
            >
              {{ initiales }}
            </div>
          </div>

          <dl class="mt-4 space-y-3 text-sm">
            <div class="flex items-center justify-between border-b border-af-bordure pb-3">
              <dt class="text-af-atone">Domaine</dt>
              <dd class="font-medium text-af-encre">{{ labelDomaine(personnalite.domaine) }}</dd>
            </div>
            <div v-if="personnalite.annee_naissance" class="flex items-center justify-between border-b border-af-bordure pb-3">
              <dt class="text-af-atone">Naissance</dt>
              <dd class="font-medium text-af-encre">{{ personnalite.annee_naissance }}</dd>
            </div>
            <div v-if="personnalite.annee_deces" class="flex items-center justify-between">
              <dt class="text-af-atone">Décès</dt>
              <dd class="font-medium text-af-encre">{{ personnalite.annee_deces }}</dd>
            </div>
          </dl>
        </section>

        <!-- Réactions & partage -->
        <section class="rounded-[10px] border border-af-bordure bg-white p-6">
          <h2 class="mb-4 text-xs font-semibold uppercase tracking-wide text-af-atone">Cette personnalité vous inspire ?</h2>
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
            class="mt-3 flex w-full items-center justify-center gap-2 rounded-lg bg-af-vert px-4 py-2.5 text-sm font-medium text-white transition hover:bg-af-vert/90 cursor-pointer"
            @click="showPartage = true"
          >
            <font-awesome-icon :icon="['fas', 'share-nodes']" class="w-4 h-4" />
            Partager
          </button>
          <div class="mt-3 flex justify-center">
            <EngagementOffrirCadeauBouton
              type-objet="personnalite_connue"
              :objet-id="personnalite.id"
              :auteur-id="personnalite.cree_par"
              :destinataire="personnalite.nom_complet"
              @offert="cadeauxRef?.rafraichir()"
            />
          </div>
        </section>

        <!-- Cadeaux reçus par cette personnalité -->
        <section class="rounded-[10px] border border-af-bordure bg-white p-6">
          <EngagementCadeauxRecus
            ref="cadeauxRef"
            type-objet="personnalite_connue"
            :objet-id="personnalite.id"
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
      </template>
    </template>

    <div v-if="chargement" class="flex items-center justify-center py-24">
      <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-af-chocolat"></div>
    </div>

    <div v-else-if="!personnalite" class="flex flex-col items-center justify-center py-24 px-4 text-center">
      <font-awesome-icon :icon="['fas', 'user']" class="w-14 h-14 text-af-atone-2 mb-4" />
      <h1 class="text-2xl font-bold text-af-corps mb-2">Personnalité introuvable</h1>
      <p class="text-af-atone mb-4">Cette personnalité n'existe pas ou a été retirée.</p>
      <NuxtLink :to="`/opportunite-afrique/${ficheId}`" class="text-af-chocolat hover:underline">
        &#8592; Retour au territoire
      </NuxtLink>
    </div>

    <template v-else>

      <div class="flex flex-col gap-6">
        <!-- Bandeau de suspension -->
        <div
          v-if="personnalite.suspendu"
          class="flex items-start gap-2 rounded-[10px] border border-af-chocolat/20 bg-af-chocolat/5 px-4 py-3 text-sm text-af-chocolat"
        >
          <font-awesome-icon :icon="['fas', 'triangle-exclamation']" class="w-4 h-4 mt-0.5 shrink-0" />
          <span>Contribution suspendue : en cours de vérification par la modération.</span>
        </div>

        <!-- Biographie -->
        <section class="rounded-[10px] border border-af-bordure bg-white p-6 sm:p-8">
          <h2 class="mb-4 flex items-center gap-2 font-oswald text-xl font-bold text-af-encre">
            <span class="flex h-8 w-8 items-center justify-center rounded-lg bg-af-chocolat/10 text-af-chocolat">
              <font-awesome-icon :icon="['fas', 'book-open']" class="w-4 h-4" />
            </span>
            Biographie
          </h2>
          <p class="whitespace-pre-line leading-relaxed text-af-corps">{{ personnalite.biographie_courte }}</p>

          <a
            v-if="personnalite.lien_reference"
            :href="personnalite.lien_reference"
            target="_blank"
            rel="noopener noreferrer"
            class="mt-6 inline-flex items-center gap-2 text-sm font-medium text-af-chocolat hover:underline"
          >
            <font-awesome-icon :icon="['fas', 'arrow-up-right-from-square']" class="w-3.5 h-3.5" />
            En savoir plus
          </a>
        </section>
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
  </NuxtLayout>
</template>

<script setup lang="ts">

definePageMeta({ layout: false })
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

// Rechargé après qu'un cadeau vient d'être offert.
const cadeauxRef = ref<{ rafraichir: () => void } | null>(null)

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
  return `${p.annee_naissance ?? '?'} - ${p.annee_deces ?? 'présent'}`
})

const initiales = computed(() => {
  const nom = personnalite.value?.nom_complet ?? ''
  const parts = nom.trim().split(/\s+/)
  const first = parts[0]?.charAt(0) ?? ''
  const last = parts.length > 1 ? (parts[parts.length - 1]?.charAt(0) ?? '') : ''
  return `${first}${last}`.toUpperCase()
})

const filAriane = computed(() => [
  { libelle: 'Opportunités en Afrique', vers: '/opportunite-afrique' },
  { libelle: 'Territoire', vers: `/opportunite-afrique/${ficheId}` },
  { libelle: personnalite.value?.nom_complet || 'Personnalité' },
])

// ── SEO / Open Graph (aperçu lors du partage sur les réseaux sociaux) ──
const requete = useRequestURL()
const urlCanonique = `${requete.protocol}//${requete.host}/opportunite-afrique/${ficheId}/personnalites/${personnaliteId}`
const imageOg = computed(() => (personnalite.value?.portrait_url ? resoudreUrlImage(personnalite.value.portrait_url) : ''))
const descriptionOg = computed(() => (personnalite.value?.biographie_courte ?? '').slice(0, 200))

useHead(() => {
  if (!personnalite.value) return {}
  const titre = `${personnalite.value.nom_complet}, Personnalité | UAfricas`
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
