<script setup lang="ts">
import type { MembreAPI } from '~/composables/useMembres'
import type { EtatRelation } from '~/composables/useAmis'
import { useUserStore } from '~/stores/user'

/**
 * Annuaire des membres : porté sur le gabarit de la refonte.
 *
 * Les données et les filtres sont inchangés : même endpoint, même pagination
 * serveur, mêmes trois familles (tous / experts / bibliothèques). Trois
 * corrections de présentation :
 *   - le bandeau dégradé de la carte portait `relative`. Un élément positionné
 *     se peint APRÈS le contenu statique qui le suit : l'avatar, remonté de
 *     40 px pour chevaucher ce bandeau, passait DESSOUS et se retrouvait
 *     tranché. La carte n'a plus de bandeau du tout ;
 *   - recherche et filtres passent dans le rail ;
 *   - la recherche s'applique à la frappe (500 ms) au lieu d'attendre Entrée :
 *     dans le rail, il n'y a plus de bouton « Rechercher » à côté du champ.
 *
 * La pagination numérotée est remplacée par un défilement continu. La
 * pagination SERVEUR est conservée telle quelle : c'est le même endpoint, la
 * même taille de page : seul le rendu change, les pages s'empilent au lieu de
 * se remplacer.
 */
definePageMeta({ layout: false })

useHead({
  title: 'Annuaire des membres | AfricanS',
  meta: [
    {
      name: 'description',
      content: 'Découvrez tous les membres inscrits sur la plateforme AfricanS.',
    },
  ],
})

const { listerMembres } = useMembres()
const { obtenirEtatsRelationLot } = useAmis()
const userStore = useUserStore()
const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string

const estConnecte = computed(() => userStore.isAuthenticated)

// ─── État ─────────────────────────────────────────────────────────────────

const membres = ref<MembreAPI[]>([])
const etats = ref<Record<string, EtatRelation>>({})
const total = ref(0)
const totalPages = ref(1)
const page = ref(1)

/**
 * Deux états de chargement, et non un seul : `chargement` (celui du composable)
 * passe à vrai pour TOUTE requête. S'en servir pour la grille remplacerait la
 * liste déjà lue par des squelettes à chaque page suivante, l'écran se viderait
 * sous les yeux du visiteur au moment précis où il défile.
 */
const chargementInitial = ref(true)
const chargementSuite = ref(false)

const aSuite = computed(() => page.value < totalPages.value)
const recherche = ref('')
const rechercheActive = ref('')
const typeActif = ref('')
const parPage = 12

const typesMembres = [
  { value: '', label: 'Tous les membres', icone: 'fa-solid fa-users' },
  { value: 'expert', label: 'Experts', icone: 'fa-solid fa-briefcase' },
  { value: 'biblio', label: 'Bibliothèques humaines', icone: 'fa-solid fa-book-open' }]

// ─── Helpers ──────────────────────────────────────────────────────────────

const photoComplete = (url: string | null): string | null => {
  if (!url) return null
  return url.startsWith('http') ? url : `${apiBase}${url}`
}

const nomComplet = (membre: MembreAPI) => `${membre.prenom ?? ''} ${membre.nom ?? ''}`.trim()

const aucunFiltreActif = computed(() => !rechercheActive.value && !typeActif.value)

// ─── Chargement ───────────────────────────────────────────────────────────

/**
 * `reprendre` distingue la première page d'une suivante. Sans ce drapeau il
 * faudrait deviner d'après `page.value === 1`, ce qui est faux au retour d'un
 * filtre : la page repasse à 1 alors qu'il s'agit bien d'un rechargement
 * complet.
 */
/**
 * Jeton de série. Changer de filtre pendant qu'une page suivante est en vol
 * laisserait sa réponse s'empiler sur la liste déjà vidée : on verrait des
 * membres qui ne correspondent plus au filtre choisi. La réponse d'une série
 * périmée est ignorée.
 */
let serie = 0

const charger = async (reprendre = false) => {
  const maSerie = ++serie
  if (reprendre) chargementSuite.value = true
  else chargementInitial.value = true

  const result = await listerMembres({
    recherche: rechercheActive.value || undefined,
    type: typeActif.value || undefined,
    page: page.value,
    par_page: parPage,
  })

  if (maSerie !== serie) return

  if (result) {
    // Un même membre ne doit pas apparaître deux fois : entre deux requêtes,
    // une inscription décale toute la pagination serveur d'un rang, et la
    // dernière carte d'une page redescend en tête de la suivante.
    const dejaLa = new Set(membres.value.map(m => m.id))
    const nouveaux = result.membres.filter(m => !dejaLa.has(m.id))

    membres.value = reprendre ? [...membres.value, ...nouveaux] : result.membres
    total.value = result.total
    totalPages.value = result.total_pages
    await chargerEtats(reprendre ? nouveaux : membres.value)
  }

  chargementInitial.value = false
  chargementSuite.value = false
}

/** États de relation des cartes nouvellement arrivées, en un seul appel (anti N+1). */
const chargerEtats = async (cibles: MembreAPI[]) => {
  if (!estConnecte.value) {
    etats.value = {}
    return
  }
  const ids = cibles.map(m => m.id).filter(id => id !== userStore.user?.id)
  if (ids.length === 0) return
  // Fusion et non remplacement : les états déjà lus concernent des cartes
  // toujours à l'écran.
  etats.value = { ...etats.value, ...(await obtenirEtatsRelationLot(ids)) }
}

/**
 * Sentinelle de fin de liste. `uneSeuleFois: false` est le point important :
 * l'observateur doit se redéclencher à CHAQUE fois qu'elle revient dans le
 * cadre, sinon une seule page supplémentaire serait chargée.
 */
const sentinelle = ref<HTMLElement | null>(null)
const { estVisible } = useObservateurVisibilite(sentinelle, {
  marge: '400px',
  uneSeuleFois: false,
})

/** Page suivante : demandée par la sentinelle ou par le bouton de repli. */
const chargerSuite = async () => {
  if (!aSuite.value || chargementInitial.value || chargementSuite.value) return
  page.value += 1
  await charger(true)

  /*
   * Relance tant que la sentinelle n'a pas quitté le cadre. Sur un grand écran,
   * douze cartes ne remplissent pas la hauteur : la sentinelle reste visible
   * après la page 2, `estVisible` ne CHANGE donc pas, et un observateur seul
   * n'émettrait plus rien : le défilement s'arrêterait net sur une liste
   * incomplète. C'est le piège classique du défilement infini.
   */
  await nextTick()
  if (estVisible.value && aSuite.value) chargerSuite()
}

watch(estVisible, (visible) => {
  if (visible) chargerSuite()
})

let minuterie: ReturnType<typeof setTimeout> | null = null
watch(recherche, () => {
  if (minuterie) clearTimeout(minuterie)
  minuterie = setTimeout(() => {
    rechercheActive.value = recherche.value.trim()
  }, 500)
})

const reinitialiser = () => {
  recherche.value = ''
  rechercheActive.value = ''
  typeActif.value = ''
  page.value = 1
}

watch([rechercheActive, typeActif], () => {
  page.value = 1
  membres.value = []
  etats.value = {}
  charger()
})

onMounted(() => charger())
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Communauté"
        sous-titre="Experts, bibliothèques humaines et passionnés du développement durable africain"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane :segments="[{ libelle: 'Communauté' }]">
        <template #centre>
          <p v-if="total > 0" class="text-base font-bold text-af-encre">
            {{ total }} membre{{ total > 1 ? 's' : '' }} inscrit{{ total > 1 ? 's' : '' }}
          </p>
        </template>
        <template #action>
          <AfricansBouton v-if="estConnecte" variante="secondaire" icone="fa-solid fa-user-pen" vers="/mon-compte/profil">
            Mon profil
          </AfricansBouton>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="flex flex-col gap-6">
      <div v-if="chargementInitial" class="grid gap-5 sm:grid-cols-2 xl:grid-cols-3">
        <div v-for="n in 6" :key="n" class="h-64 animate-pulse rounded-[10px] bg-af-bordure" />
      </div>

      <template v-else-if="membres.length > 0">
        <div class="grid gap-5 sm:grid-cols-2 xl:grid-cols-3">
          <!-- La carte n'a plus de bandeau : l'avatar posé à cheval dessus
               était tranché, et le dégradé orange→vert n'appartient pas à la
               palette de la refonte. -->
          <article
            v-for="membre in membres"
            :key="membre.id"
            class="flex flex-col items-center rounded-[10px] border border-af-bordure bg-white p-5 text-center transition hover:border-af-chocolat"
          >
            <NuxtLink :to="`/profil/${membre.id}`" class="flex flex-col items-center gap-3">
              <AfricansAvatar :nom="nomComplet(membre)" :src="photoComplete(membre.photoUrl)" :taille="80" />
              <div>
                <h3 class="text-[17px]/[1.4] font-bold text-af-encre transition hover:text-af-chocolat">
                  {{ nomComplet(membre) }}
                </h3>
                <p v-if="membre.fonction" class="mt-0.5 text-[14px]/[1.4] text-af-corps">
                  {{ membre.fonction }}
                </p>
              </div>
            </NuxtLink>

            <div v-if="membre.estExpert || membre.estBiblio" class="mt-3 flex flex-wrap justify-center gap-2">
              <AfricansEtiquette v-if="membre.estExpert" ton="vert">Expert</AfricansEtiquette>
              <AfricansEtiquette v-if="membre.estBiblio">Bibliothèque humaine</AfricansEtiquette>
            </div>

            <p
              v-if="membre.ville || membre.pays"
              class="mt-3 flex items-center gap-1.5 text-[12px]/[1.4] text-af-atone"
            >
              <font-awesome-icon icon="fa-solid fa-location-dot" />
              {{ [membre.ville, membre.pays].filter(Boolean).join(', ') }}
            </p>

            <!-- `mt-auto` cale le bouton en bas quelle que soit la hauteur des
                 lignes précédentes : sans lui, les cartes d'une même rangée
                 alignent leurs boutons à des hauteurs différentes. -->
            <div v-if="estConnecte && membre.id !== userStore.user?.id" class="mt-auto pt-4">
              <SocialBoutonAmitie
                :utilisateur-id="membre.id"
                :etat="etats[membre.id] || 'aucune'"
                taille="sm"
                @update="(e: EtatRelation) => etats[membre.id] = e"
              />
            </div>
          </article>
        </div>

        <!-- Sentinelle : elle entre dans le cadre 400 px avant le bas de la
             liste, ce qui laisse le temps à la requête d'aboutir avant que le
             visiteur n'atteigne le vide.

             Le bouton n'est pas décoratif. L'observateur ne se déclenche qu'au
             défilement à la souris ou au doigt : sans lui, une navigation au
             clavier ou un navigateur sans IntersectionObserver s'arrêterait à
             la première page. -->
        <div v-if="aSuite" ref="sentinelle" class="flex flex-col gap-5">
          <div v-if="chargementSuite" class="grid gap-5 sm:grid-cols-2 xl:grid-cols-3" aria-hidden="true">
            <div v-for="n in 3" :key="n" class="h-64 animate-pulse rounded-[10px] bg-af-bordure" />
          </div>
          <AfricansBouton
            v-else
            class="self-center"
            variante="secondaire"
            icone="fa-solid fa-arrow-down"
            @click="chargerSuite"
          >
            Afficher plus de membres
          </AfricansBouton>
        </div>

        <p v-else-if="membres.length > parPage" class="text-center text-[14px]/[1.4] text-af-atone">
          Vous avez vu les {{ total }} membres.
        </p>

        <!-- Politesse envers les lecteurs d'écran : la liste s'allonge sans
             qu'aucun élément ne reçoive le focus, rien ne le signalerait. -->
        <p class="sr-only" role="status" aria-live="polite">
          {{ membres.length }} membre{{ membres.length > 1 ? 's' : '' }} affiché{{ membres.length > 1 ? 's' : '' }} sur {{ total }}.
        </p>
      </template>

      <!-- Deux vides distincts : « rien ne correspond » n'est pas « personne
           n'est inscrit », et la sortie proposée n'est pas la même. -->
      <div v-else class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
        <font-awesome-icon icon="fa-solid fa-users-slash" class="text-4xl text-af-atone-2" />
        <p class="mt-4 text-[16px]/[1.4] font-bold">
          {{ aucunFiltreActif ? 'Aucun membre inscrit pour le moment' : 'Aucun membre ne correspond à vos critères' }}
        </p>
        <AfricansBouton
          v-if="!aucunFiltreActif"
          class="mt-6"
          variante="secondaire"
          icone="fa-solid fa-rotate-left"
          @click="reinitialiser"
        >
          Réinitialiser
        </AfricansBouton>
      </div>
    </div>

    <template #rail>
      <AfricansRecherche v-model="recherche" placeholder="Nom, fonction, ville…" />

      <AfricansPanneau titre="Filtres" icone="fa-solid fa-sliders" action-libelle="Réinitialiser" @action="reinitialiser">
        <ul class="flex flex-col gap-1">
          <li v-for="t in typesMembres" :key="t.value">
            <button
              type="button"
              class="flex w-full items-center gap-3 rounded-lg px-3 py-2.5 text-left text-[14px]/[1.4] font-bold transition"
              :class="typeActif === t.value
                ? 'bg-af-chocolat/15 text-af-chocolat'
                : 'text-af-corps hover:bg-af-chocolat/[0.07]'"
              :aria-pressed="typeActif === t.value"
              @click="typeActif = t.value"
            >
              <font-awesome-icon :icon="t.icone" class="size-5 shrink-0" />
              {{ t.label }}
            </button>
          </li>
        </ul>
      </AfricansPanneau>

      <AfricansPanneau titre="Statistiques" icone="fa-solid fa-chart-line">
        <dl class="flex flex-col">
          <div class="flex items-baseline justify-between gap-4 pb-3">
            <dt class="text-[14px]/[1.4] font-bold">Membres</dt>
            <dd class="text-[14px]/[1.4] font-bold text-af-chocolat">{{ total }}</dd>
          </div>
          <div class="flex items-baseline justify-between gap-4 border-t border-af-bordure pt-3">
            <dt class="text-[14px]/[1.4] font-bold">Affichés</dt>
            <dd class="text-[14px]/[1.4] font-bold text-af-chocolat">{{ membres.length }}</dd>
          </div>
        </dl>
      </AfricansPanneau>
    </template>
  </NuxtLayout>
</template>
