<script setup lang="ts">
import {
  useOpportuniteAfrique,
  formatDate,
  type FichePaysDetailAPI,
  type ContributeurAPI,
  type TypeObjetContribution,
  type SectionAfripulse,
} from '~/composables/useOpportuniteAfrique'
import { useUserStore } from '~/stores/user'

/**
 * Fiche territoire Afripulse : portée sur le gabarit de la refonte.
 *
 * La page était restée sur l'ancienne enveloppe (barre supérieure à
 * méga-menus, pas de navigation latérale) alors que `/opportunite-afrique`
 * était portée : cliquer une carte ou un drapeau faisait changer de squelette.
 *
 * Réagencée d'après le cadre Figma « afripulse-2 » :
 *   - le hero devient le bandeau de module, titre centré et badge de région en
 *     bas à droite. Le drapeau en quitte l'en-tête : il figure déjà dans les
 *     symboles nationaux, à droite ;
 *   - « Informations générales » passe en deux colonnes : les six champs à
 *     gauche, un visuel à droite ;
 *   - « Cultures et langues » liste les langues et les groupes ethniques
 *     NUMÉROTÉS sur deux colonnes, comme la maquette, au lieu de pastilles ;
 *   - les cinq sections enrichies deviennent des accordéons `AfricansAccordeon`
 *     (repliés), au lieu de bandes pleine largeur à fond gris ;
 *   - statistiques, symboles, réactions et actions passent dans le rail.
 *
 * UN ÉCART assumé à la maquette : elle dessine le DÉCOUPAGE PROVINCIAL du
 * pays, avec le nom et la superficie de chaque province. Aucune donnée
 * infranationale n'existe en base : la carte affiche donc le contour du
 * territoire, tracé à l'ouverture, et rien de plus. Inventer neuf provinces
 * ferait une jolie image et une fiche fausse.
 *
 * Les sept symboles nationaux de la maquette (devise, drapeau, armoiries,
 * hymne, fleur, animal, oiseau) sont désormais tous portés par la base, la
 * migration `11l` a ajouté les notices et les trois symboles qui manquaient.
 * Chacun n'apparaît que s'il est réellement renseigné.
 */
definePageMeta({ layout: false })

const route = useRoute()
const userStore = useUserStore()
const { redirigerVersConnexion } = useAuth()
const {
  obtenirFiche,
  soumettreContribution,
  soumettreContributionEnrichie,
  listerContributeurs,
  reagirFiche,
  partagerFiche,
  resoudreUrlImage,
} = useOpportuniteAfrique()

interface AfripulseContext {
  type_objet_contribution: TypeObjetContribution
  section_afripulse: SectionAfripulse
  type_contribution: 'ajout' | 'edition' | 'suppression'
  target_id?: string
  donnees_actuelles?: Record<string, unknown>
  libelle?: string
}

interface LegacyFieldContext {
  section: string
  label: string
  valeurActuelle?: string
}

const idFiche = route.params.id as string

// Chargement côté serveur (SSR) : indispensable pour que les balises Open Graph
// soient présentes dans le HTML lu par les robots des réseaux sociaux.
const { data: ficheChargee } = await useAsyncData(
  `fiche-pays-${idFiche}`,
  () => obtenirFiche(idFiche),
)
const pays = ref<FichePaysDetailAPI | null>(ficheChargee.value)

// ── SEO / Open Graph ───────────────────────────────────────────────────────

const requete = useRequestURL()
const origineSite = `${requete.protocol}//${requete.host}`
const urlCanonique = `${origineSite}/opportunite-afrique/${idFiche}`
const imageOg = computed(() => {
  const img = pays.value?.image_couverture || pays.value?.drapeau_url
  return img ? resoudreUrlImage(img) : ''
})
const descriptionOg = computed(() =>
  pays.value
    ? `Découvrez ${pays.value.nom} : capitale ${pays.value.capitale}, population ${pays.value.population}, culture, langues et opportunités sur AfricanS.`
    : 'Opportunités en Afrique | AfricanS',
)

useHead(() => {
  if (!pays.value) return {}
  const titre = `${pays.value.nom} : Afripulse | AfricanS`
  return {
    title: titre,
    meta: [
      { name: 'description', content: descriptionOg.value },
      { property: 'og:type', content: 'article' },
      { property: 'og:title', content: titre },
      { property: 'og:description', content: descriptionOg.value },
      { property: 'og:url', content: urlCanonique },
      { property: 'og:site_name', content: 'AfricanS' }, ...(imageOg.value ? [{ property: 'og:image', content: imageOg.value }] : []),
      { name: 'twitter:card', content: imageOg.value ? 'summary_large_image' : 'summary' },
      { name: 'twitter:title', content: titre },
      { name: 'twitter:description', content: descriptionOg.value }, ...(imageOg.value ? [{ name: 'twitter:image', content: imageOg.value }] : [])],
    link: [{ rel: 'canonical', href: urlCanonique }],
  }
})

// ── Informations générales ─────────────────────────────────────────────────

/** Les six champs de la maquette, dans son ordre. Un champ vide est omis. */
const CHAMPS = computed(() => {
  const p = pays.value
  if (!p) return []
  return [
    { icone: 'fa-solid fa-building', libelle: 'Capitale', valeur: p.capitale },
    { icone: 'fa-solid fa-boxes-stacked', libelle: 'Superficie', valeur: p.superficie },
    { icone: 'fa-solid fa-location-dot', libelle: 'Région', valeur: p.region },
    { icone: 'fa-solid fa-users', libelle: 'Population', valeur: p.population },
    { icone: 'fa-solid fa-coins', libelle: 'Monnaie', valeur: p.monnaie },
    { icone: 'fa-solid fa-flag', libelle: 'Devise', valeur: p.devise }].filter(c => Boolean(c.valeur))
})

/**
 * Symboles nationaux, dans l'ordre de la maquette. Un symbole n'apparaît que
 * s'il porte une image OU un texte : une fiche mal renseignée montre moins de
 * lignes, jamais une ligne vide, et jamais une valeur inventée.
 */
const SYMBOLES = computed(() => {
  const p = pays.value
  if (!p) return []
  return [
    { cle: 'devise', libelle: 'Devise nationale', texte: p.devise, image: null, italique: true },
    {
      cle: 'drapeau',
      libelle: 'Drapeau',
      texte: p.drapeau_description,
      image: p.drapeau_url ? resoudreUrlImage(p.drapeau_url) : null,
      hauteur: 'h-12',
    },
    {
      cle: 'armoiries',
      libelle: 'Armoiries',
      texte: p.embleme_description,
      image: p.embleme_url ? resoudreUrlImage(p.embleme_url) : null,
      hauteur: 'h-20',
    },
    // Le titre de l'hymne et sa notice sont deux colonnes : on les joint sur
    // une ligne pour ne pas faire deux entrées d'un seul symbole.
    {
      cle: 'hymne',
      libelle: 'Hymne national',
      texte: [p.hymne_national, p.hymne_description].filter(Boolean).join(' - ') || null,
      image: null,
    },
    { cle: 'fleur', libelle: 'Fleur nationale', texte: [p.fleur_nationale, p.fleur_description].filter(Boolean).join(' - ') || null, image: null },
    { cle: 'animal', libelle: 'Animal national', texte: [p.animal_national, p.animal_description].filter(Boolean).join(' - ') || null, image: null },
    { cle: 'oiseau', libelle: 'Oiseau national', texte: [p.oiseau_national, p.oiseau_description].filter(Boolean).join(' - ') || null, image: null }].filter(sym => sym.texte || sym.image)
})

// ── Recherche du rail : elle renvoie à la liste des territoires ────────────

const rechercheTerritoire = ref('')
const lancerRecherche = () => {
  const q = rechercheTerritoire.value.trim()
  navigateTo(q ? `/opportunite-afrique?recherche=${encodeURIComponent(q)}` : '/opportunite-afrique')
}

// ── Contribution ───────────────────────────────────────────────────────────

const contributeurs = ref<ContributeurAPI[]>([])
const showContributionModal = ref(false)
const afripulseContext = ref<AfripulseContext | null>(null)
const legacyFieldContext = ref<LegacyFieldContext | null>(null)
const contributionModalRef = ref<{ setLoading: (v: boolean) => void, setError: (m: string) => void, setSuccess: () => void } | null>(null)

const proposerModification = () => {
  if (!userStore.isAuthenticated) return redirigerVersConnexion()
  afripulseContext.value = null
  legacyFieldContext.value = null
  showContributionModal.value = true
}

const onOpenContribution = (ctx: AfripulseContext) => {
  legacyFieldContext.value = null
  afripulseContext.value = ctx
  showContributionModal.value = true
}

const onOpenChampVoyage = (ctx: LegacyFieldContext) => {
  afripulseContext.value = null
  legacyFieldContext.value = ctx
  showContributionModal.value = true
}

const onRequireLogin = () => redirigerVersConnexion()

const fermerContributionModal = () => {
  showContributionModal.value = false
  afripulseContext.value = null
  legacyFieldContext.value = null
}

// ── Réactions ──────────────────────────────────────────────────────────────

const reactionEnCours = ref(false)

const basculerReaction = async (type: 'like' | 'dislike') => {
  if (!pays.value) return
  if (!userStore.isAuthenticated) return redirigerVersConnexion()
  if (reactionEnCours.value) return
  reactionEnCours.value = true
  const etat = await reagirFiche(pays.value.id, type)
  if (etat) {
    pays.value.nombre_likes = etat.nombre_likes
    pays.value.nombre_dislikes = etat.nombre_dislikes
    pays.value.ma_reaction = etat.ma_reaction
  }
  reactionEnCours.value = false
}

// ── Partage ────────────────────────────────────────────────────────────────

const showPartageModal = ref(false)
const partageModalRef = ref<{ setLoading: (v: boolean) => void, setError: (m: string) => void, setSuccess: () => void } | null>(null)
const cadeauxRef = ref<{ rafraichir: () => void } | null>(null)

// Le partage réseaux sociaux est ouvert à tous ; seule la publication sur le
// mur communautaire requiert une connexion (gérée dans la modale).
const ouvrirPartage = () => { showPartageModal.value = true }

const handlePartageSubmit = async (legende: string) => {
  if (!pays.value) return
  partageModalRef.value?.setLoading(true)
  const res = await partagerFiche(pays.value.id, legende || undefined)
  if (res) partageModalRef.value?.setSuccess()
  else partageModalRef.value?.setError('Erreur lors du partage. Veuillez réessayer.')
}

type SubmitLegacy = {
  mode: 'legacy'
  section: string
  type_contribution: string
  nouvelle_valeur: string
  justification: string
}

type SubmitAfripulse = {
  mode: 'afripulse'
  type_objet_contribution: TypeObjetContribution
  section_afripulse: SectionAfripulse
  type_contribution: 'ajout' | 'edition' | 'suppression'
  target_id?: string
  nouvelle_valeur_jsonb: Record<string, unknown>
  justification: string
}

const handleContributionSubmit = async (data: SubmitLegacy | SubmitAfripulse) => {
  if (!pays.value) return
  contributionModalRef.value?.setLoading(true)

  let ok = false
  if (data.mode === 'afripulse') {
    const res = await soumettreContributionEnrichie(pays.value.id, {
      type_objet_contribution: data.type_objet_contribution,
      section_afripulse: data.section_afripulse,
      type_contribution: data.type_contribution,
      target_id: data.target_id,
      nouvelle_valeur_jsonb: data.nouvelle_valeur_jsonb,
      justification: data.justification || undefined,
    })
    ok = Boolean(res)
  }
  else {
    const res = await soumettreContribution(pays.value.id, {
      section: data.section,
      type_contribution: data.type_contribution,
      nouvelle_valeur: data.nouvelle_valeur,
      justification: data.justification || undefined,
    })
    ok = Boolean(res)
  }

  if (ok) {
    contributionModalRef.value?.setSuccess()
    contributeurs.value = await listerContributeurs(pays.value.id)
  }
  else {
    contributionModalRef.value?.setError('Erreur lors de la soumission de votre contribution. Veuillez réessayer.')
  }
}

onMounted(async () => {
  // La fiche est déjà chargée côté serveur ; on complète avec les contributeurs.
  if (pays.value) contributeurs.value = await listerContributeurs(pays.value.id)
})
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        :titre="pays?.nom ?? 'Territoire'"
        :sous-titre="pays?.slogan ?? undefined"
        :image="pays?.image_couverture ? resoudreUrlImage(pays.image_couverture) : null"
      >
        <template v-if="pays?.region" #action>
          <span class="rounded-lg bg-af-vert px-4 py-2 text-[14px]/[1.4] font-bold text-white">
            {{ pays.region }}
          </span>
        </template>
      </AfricansBandeauModule>
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[
          { libelle: 'Africarise', vers: '/africa-culture' },
          { libelle: 'Afripulse', vers: '/opportunite-afrique' },
          { libelle: pays?.nom ?? 'Territoire' }]"
      />
    </template>

    <div v-if="!pays" class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
      <font-awesome-icon icon="fa-solid fa-earth-africa" class="text-4xl text-af-atone-2" />
      <p class="mt-4 text-[16px]/[1.4] font-bold">Territoire introuvable</p>
      <p class="mt-2 text-[14px]/[1.4] text-af-corps">
        Cette fiche n'existe pas ou n'est plus publiée.
      </p>
      <AfricansBouton class="mt-6" variante="secondaire" icone="fa-solid fa-arrow-left" vers="/opportunite-afrique">
        Retour aux territoires
      </AfricansBouton>
    </div>

    <div v-else class="flex flex-col gap-5">
      <!-- Informations générales : champs à gauche, visuel à droite. -->
      <AfricansAccordeon titre="Informations générales" icone="fa-solid fa-circle-info" fond="blanc" par-defaut-ouvert>
        <div class="grid gap-6 md:grid-cols-2 md:items-center">
          <dl class="flex flex-col gap-5">
            <div v-for="champ in CHAMPS" :key="champ.libelle" class="flex items-start gap-3">
              <font-awesome-icon :icon="champ.icone" class="mt-0.5 size-5 shrink-0 text-af-chocolat" />
              <div class="min-w-0">
                <dt class="text-[14px]/[1.4] text-af-corps">{{ champ.libelle }}</dt>
                <dd class="text-[14px]/[1.4] font-bold text-af-encre">{{ champ.valeur }}</dd>
              </div>
            </div>
          </dl>

          <!-- La maquette pose une carte à cette place. Le contour du pays
               vient de `@svg-maps/world`, déjà utilisé par la carte d'Afrique
               de la liste : les 55 territoires sont couverts, sans asset ni
               requête. Repli sur la photo pour une fiche sans code ISO2. -->
          <OpportuniteAfriqueCarteTerritoire
            v-if="pays.code"
            :code="pays.code"
            :nom="pays.nom"
          />
          <img
            v-else-if="pays.image_couverture"
            :src="resoudreUrlImage(pays.image_couverture)"
            :alt="pays.nom"
            class="w-full rounded-[10px] object-cover"
          />
        </div>
      </AfricansAccordeon>

      <!-- Cultures et langues -->
      <AfricansAccordeon
        v-if="pays.langues?.length || pays.ethnies?.length"
        titre="Cultures et langues"
        icone="fa-solid fa-earth-africa"
      >
        <div class="grid gap-6 md:grid-cols-[1fr_auto] md:items-center">
          <div class="flex flex-col gap-8">
          <div v-if="pays.langues?.length" class="flex flex-col gap-3">
            <h4 class="text-[17px]/[1.4] font-bold text-af-encre">Langues parlées</h4>
            <ol class="grid gap-x-8 gap-y-1 sm:grid-cols-2">
              <li
                v-for="(langue, i) in pays.langues"
                :key="langue"
                class="text-[14px]/[1.4] text-af-encre"
              >
                {{ i + 1 }}-{{ langue }}
              </li>
            </ol>
          </div>

          <div v-if="pays.ethnies?.length" class="flex flex-col gap-3">
            <h4 class="text-[17px]/[1.4] font-bold text-af-encre">Principaux groupes ethniques</h4>
            <ol class="grid gap-x-8 gap-y-1 sm:grid-cols-2">
              <li
                v-for="(ethnie, i) in pays.ethnies"
                :key="ethnie"
                class="text-[14px]/[1.4] text-af-encre"
              >
                {{ i + 1 }}-{{ ethnie }}
              </li>
            </ol>
          </div>
          </div>

          <!-- L'illustration du Figma, exportée depuis la maquette. Son
               `width`/`height` en dur a été retiré à l'enregistrement : le
               `viewBox` suffit à donner le rapport d'aspect, et c'est la mise
               en page qui décide de la taille.
               `alt` vide : elle est décorative, la relire à voix haute
               n'apprendrait rien de plus que les listes qu'elle accompagne. -->
          <img
            src="/images/africans/illustrations/cultures-langues.svg"
            alt=""
            class="mx-auto w-full max-w-[260px] md:mx-0"
          />
        </div>
      </AfricansAccordeon>

      <!-- Sections enrichies (US1), toutes repliées. -->
      <OpportuniteAfriqueSecteursOpportunitesSection
        :fiche-id="pays.id"
        :est-authentifie="userStore.isAuthenticated"
        @open-contribution="onOpenContribution"
        @require-login="onRequireLogin"
      />
      <OpportuniteAfriqueRecettesCulinairesSection
        :fiche-id="pays.id"
        :est-authentifie="userStore.isAuthenticated"
        @open-contribution="onOpenContribution"
        @require-login="onRequireLogin"
      />
      <OpportuniteAfriqueSitesTouristiquesSection
        :fiche-id="pays.id"
        :est-authentifie="userStore.isAuthenticated"
        @open-contribution="onOpenContribution"
        @require-login="onRequireLogin"
      />
      <OpportuniteAfriquePersonnalitesSection
        :fiche-id="pays.id"
        :est-authentifie="userStore.isAuthenticated"
        @open-contribution="onOpenContribution"
        @require-login="onRequireLogin"
      />
      <OpportuniteAfriqueSavoirAvantVoyagerSection
        :fiche-id="pays.id"
        :fiche="pays"
        :est-authentifie="userStore.isAuthenticated"
        @open-contribution="onOpenContribution"
        @open-champ-voyage="onOpenChampVoyage"
        @require-login="onRequireLogin"
      />
    </div>

    <template #rail>
      <form @submit.prevent="lancerRecherche">
        <AfricansRecherche v-model="rechercheTerritoire" placeholder="Territoire, région, pays…" />
      </form>

      <template v-if="pays">
        <AfricansPanneau titre="Statistiques" icone="fa-solid fa-sliders">
          <dl class="flex flex-col">
            <div class="flex items-baseline justify-between gap-4 pb-3">
              <dt class="text-[14px]/[1.4] font-bold">Contributions</dt>
              <dd class="text-[14px]/[1.4] font-bold text-af-chocolat">{{ pays.nombre_contributions }}</dd>
            </div>
            <div class="flex items-baseline justify-between gap-4 border-t border-af-bordure py-3">
              <dt class="text-[14px]/[1.4] font-bold">J'aime</dt>
              <dd class="text-[14px]/[1.4] font-bold text-af-chocolat">{{ pays.nombre_likes }}</dd>
            </div>
            <div class="flex items-baseline justify-between gap-4 border-t border-af-bordure pt-3">
              <dt class="text-[14px]/[1.4] font-bold">Dernière mise à jour</dt>
              <dd class="text-[14px]/[1.4] text-af-corps">{{ formatDate(pays.updated_at) }}</dd>
            </div>
          </dl>
        </AfricansPanneau>

        <AfricansPanneau v-if="SYMBOLES.length" titre="Symboles nationaux" icone="fa-solid fa-star">
          <ul class="flex flex-col gap-5">
            <li v-for="sym in SYMBOLES" :key="sym.cle" class="flex flex-col gap-2">
              <p class="text-[12px]/[1.4] font-bold text-af-atone uppercase">{{ sym.libelle }}</p>
              <!-- `self-start` n'est PAS cosmétique : le <li> est un conteneur
                   flex en colonne, dont l'`align-items` vaut `stretch`. Une
                   image en `w-auto` s'y étirait donc sur toute la largeur du
                   rail tout en gardant la hauteur imposée, le drapeau
                   paraissait écrasé, six fois trop large. -->
              <img
                v-if="sym.image"
                :src="sym.image"
                :alt="`${sym.libelle} - ${pays.nom}`"
                class="w-auto self-start"
                :class="sym.hauteur"
              />
              <p
                v-if="sym.texte"
                class="text-[14px]/[1.4]"
                :class="sym.italique ? 'text-af-encre italic' : 'text-af-corps'"
              >
                <template v-if="sym.italique">« {{ sym.texte }} »</template>
                <template v-else>{{ sym.texte }}</template>
              </p>
            </li>
          </ul>
        </AfricansPanneau>

        <AfricansPanneau titre="Cette fiche vous plaît ?" icone="fa-solid fa-thumbs-up">
          <div class="flex items-center gap-3">
            <button
              type="button"
              :disabled="reactionEnCours"
              class="flex flex-1 items-center justify-center gap-2 rounded-lg border py-2.5 text-[14px]/[1.4] font-bold transition disabled:opacity-60"
              :class="pays.ma_reaction === 'like'
                ? 'border-af-vert bg-af-vert text-white'
                : 'border-af-bordure bg-white text-af-corps hover:border-af-vert'"
              @click="basculerReaction('like')"
            >
              <font-awesome-icon icon="fa-solid fa-thumbs-up" />
              {{ pays.nombre_likes }}
            </button>
            <button
              type="button"
              :disabled="reactionEnCours"
              class="flex flex-1 items-center justify-center gap-2 rounded-lg border py-2.5 text-[14px]/[1.4] font-bold transition disabled:opacity-60"
              :class="pays.ma_reaction === 'dislike'
                ? 'border-af-live bg-af-live text-white'
                : 'border-af-bordure bg-white text-af-corps hover:border-af-live'"
              @click="basculerReaction('dislike')"
            >
              <font-awesome-icon icon="fa-solid fa-thumbs-down" />
              {{ pays.nombre_dislikes }}
            </button>
          </div>
        </AfricansPanneau>

        <AfricansPanneau titre="Cadeaux" icone="fa-solid fa-gift">
          <div class="flex flex-col gap-4">
            <EngagementOffrirCadeauBouton
              type-objet="fiche_pays"
              :objet-id="pays.id"
              :destinataire="pays.nom"
              @offert="cadeauxRef?.rafraichir()"
            />
            <EngagementCadeauxRecus ref="cadeauxRef" type-objet="fiche_pays" :objet-id="pays.id" />
          </div>
        </AfricansPanneau>

        <OpportuniteAfriqueContributeursSection :contributeurs="contributeurs" />

        <AfricansPanneau titre="Actions" icone="fa-solid fa-pen-to-square">
          <div class="flex flex-col gap-3">
            <AfricansBouton icone="fa-solid fa-pen-to-square" @click="proposerModification">
              Proposer une modification
            </AfricansBouton>
            <AfricansBouton variante="secondaire" icone="fa-solid fa-share-nodes" @click="ouvrirPartage">
              Partager
            </AfricansBouton>
            <AfricansBouton variante="secondaire" icone="fa-solid fa-arrow-left" vers="/opportunite-afrique">
              Retour aux territoires
            </AfricansBouton>
          </div>
        </AfricansPanneau>
      </template>
    </template>

    <OpportuniteAfriqueContributionModal
      ref="contributionModalRef"
      :is-open="showContributionModal"
      :fiche-id="pays?.id || ''"
      :pays-nom="pays?.nom || ''"
      :afripulse-context="afripulseContext"
      :legacy-context="legacyFieldContext"
      @close="fermerContributionModal"
      @submit="handleContributionSubmit"
    />

    <OpportuniteAfriquePartagerFicheModal
      ref="partageModalRef"
      :is-open="showPartageModal"
      :pays-nom="pays?.nom || ''"
      :fiche-id="pays?.id"
      :est-connecte="userStore.isAuthenticated"
      @close="showPartageModal = false"
      @submit="handlePartageSubmit"
    />
  </NuxtLayout>
</template>
