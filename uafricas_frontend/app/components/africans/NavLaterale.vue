<template>
  <!-- Colonne de navigation : 312 px, 11 entrées, pas vertical de 44 px.
       En dessous de lg, elle devient une bande horizontale défilante, la
       maquette ne prévoit rien pour le mobile, c'est une décision d'intégration.
       Les accordéons n'existent QUE dans la disposition verticale : déplier une
       liste sous une bande qui défile horizontalement n'a pas de sens, et le
       tiroir de la barre supérieure porte déjà ces sous-entrées en mobile. -->
  <nav
    class="af-nav flex gap-1 overflow-x-auto scrollbar-none py-2 lg:flex-col lg:overflow-visible lg:py-0"
    aria-label="Navigation principale"
  >
    <!-- Un univers et ses applications forment UN bloc : même fond, même
         arrondi, et un filet vertical aligné sur l'icône de l'univers qui
         relie les applications à leur parent. Séparés, les deux niveaux se
         lisaient comme deux menus distincts. -->
    <div
      v-for="entree in NAV_AFRICANS"
      :key="entree.libelle"
      class="shrink-0 transition-colors duration-200 lg:w-full lg:rounded-xl"
      :class="entree.sousEntrees && estDeplie(entree) && 'lg:bg-af-chocolat/[0.05]'"
      @pointerenter="entrer(entree, $event)"
      @pointerleave="sortir(entree)"
      @focusin="focaliser(entree)"
      @focusout="defocaliser(entree, $event)"
    >
      <!-- Le fond d'état est porté par la LIGNE et non par le lien : posé sur
           le lien seul, il s'arrêtait avant le chevron et coupait l'en-tête
           de l'univers en deux pièces. -->
      <div
        class="flex items-center rounded-lg transition-colors"
        :class="estActive(entree)
          ? 'bg-af-chocolat/15 text-af-chocolat'
          : entree.vers
            ? 'text-af-encre hover:bg-af-chocolat/[0.07]'
            : 'text-af-atone-2'"
      >
        <component
          :is="entree.vers ? LienNuxt : 'span'"
          :to="entree.vers ?? undefined"
          class="flex flex-1 items-center gap-4 rounded-lg px-4 py-[11px] text-base font-bold whitespace-nowrap"
          :class="!entree.vers && 'cursor-not-allowed'"
          :title="entree.vers ? undefined : 'Route non encore rattachée'"
        >
          <font-awesome-icon :icon="entree.icone" class="size-6 shrink-0 text-lg" />
          <span>{{ entree.libelle }}</span>
        </component>

        <!-- Le chevron reste, le survol n'existant ni au toucher ni au
             clavier : il ÉPINGLE l'univers ouvert, ou le referme. C'est un
             bouton SÉPARÉ du lien : l'univers reste atteignable d'un clic. -->
        <button
          v-if="entree.sousEntrees"
          type="button"
          class="mr-1 hidden size-9 shrink-0 place-items-center rounded-lg transition hover:text-af-chocolat lg:grid"
          :class="estActive(entree) ? 'text-af-chocolat' : 'text-af-atone'"
          :aria-expanded="estDeplie(entree)"
          :aria-controls="`af-sous-nav-${identifiant(entree)}`"
          :aria-label="`${estDeplie(entree) ? 'Replier' : 'Déplier'} les applications de ${entree.libelle}`"
          @click="basculer(entree)"
        >
          <font-awesome-icon
            icon="fa-solid fa-chevron-down"
            class="text-xs transition-transform duration-200"
            :class="estDeplie(entree) && 'rotate-180'"
          />
        </button>
      </div>

      <!-- Déroulé animé par `grid-template-rows` 0fr → 1fr : la hauteur
           réelle n'a pas à être connue. Le contenu replié reste dans le DOM
           (la recherche du navigateur le trouve) mais `inert` le retire de la
           tabulation, sinon le clavier traverserait des liens invisibles. -->
      <div
        v-if="entree.sousEntrees"
        :id="`af-sous-nav-${identifiant(entree)}`"
        class="hidden transition-[grid-template-rows] duration-200 ease-out lg:grid"
        :class="estDeplie(entree) ? 'grid-rows-[1fr]' : 'grid-rows-[0fr]'"
        :inert="!estDeplie(entree) || undefined"
      >
        <div class="min-h-0 overflow-hidden">
          <!-- 28 px = 16 de marge interne du lien + la moitié de l'icône de
               24 px : le filet tombe pile sous le centre de l'icône. -->
          <ul class="mt-0.5 mb-2 ml-7 flex flex-col gap-0.5 border-l border-af-chocolat/20 pr-2">
            <li v-for="sous in entree.sousEntrees" :key="sous.to" class="relative">
              <NuxtLink
                :to="sous.to"
                class="ml-2 flex items-center gap-3 rounded-lg px-3 py-1.5 text-[14px]/[1.4] transition-colors"
                :class="sous.to === cibleLaPlusPrecise
                  ? 'bg-af-chocolat/[0.1] font-bold text-af-chocolat'
                  : 'text-af-corps hover:bg-af-chocolat/[0.07] hover:text-af-chocolat'"
                :title="sous.description"
              >
                <!-- Repère posé SUR le filet : l'application courante se lit
                     dans la continuité de l'arborescence. -->
                <span
                  v-if="sous.to === cibleLaPlusPrecise"
                  class="absolute top-1.5 bottom-1.5 -left-px w-0.5 rounded-full bg-af-chocolat"
                  aria-hidden="true"
                />
                <font-awesome-icon :icon="sous.icon" class="size-4 shrink-0" />
                <span class="truncate">{{ sous.label }}</span>
              </NuxtLink>
            </li>
          </ul>
        </div>
      </div>
    </div>

    <!-- Bouton d'action de la maquette. Il MÈNE à Codimoi : c'est le seul
         module de la plateforme où un membre publie librement, le fil, lui,
         agrège ce que les neuf sources produisent. -->
    <AfricansBouton
      vers="/codi-moi"
      icone="fa-solid fa-plus"
      class="mt-4 shrink-0 lg:w-full"
    >
      Publier
    </AfricansBouton>
  </nav>
</template>

<script setup lang="ts">
import { NAV_AFRICANS, type EntreeNav } from '~/utils/navigation-africans'

/**
 * `<component :is="'NuxtLink'">` ne résout PAS le composant : la chaîne est
 * rendue telle quelle, et le navigateur reçoit une balise `<NuxtLink>` inerte
 * un lien qui n'en est pas un. `resolveComponent` le résout pour de bon.
 */
const LienNuxt = resolveComponent('NuxtLink')

const route = useRoute()

const identifiant = (entree: EntreeNav) => entree.libelle.toLowerCase().replace(/[^a-z0-9]+/g, '-')

/**
 * Le préfixe est nécessaire : `/codi-moi/quelque-chose` doit allumer
 * « Africarise ». Le cas `/` en est exclu, sinon toutes les entrées
 * s'allumeraient.
 */
function correspond(cible: string, chemin: string): boolean {
  if (cible === '/') return chemin === '/'
  return chemin === cible || chemin.startsWith(`${cible}/`)
}

/** Toutes les destinations déclarées, univers et applications confondus. */
const CIBLES: string[] = NAV_AFRICANS.flatMap(e => [
  ...(e.vers ? [e.vers] : []), ...(e.sousEntrees?.map(s => s.to) ?? [])])

/**
 * La cible la plus SPÉCIFIQUE qui corresponde à la route, la plus longue.
 *
 * Un simple préfixe ne suffit pas, parce que deux univers se chevauchent :
 * Novagouv vit sous `/universite/gouvernance`, qui est à l'intérieur du
 * `/universite` de Mindshiftlab. Aller sur Novagouv allumait donc Novagouv,
 * Mindshiftlab ET son application Muniversa, trois entrées surlignées pour
 * une seule page, et le membre ne savait plus où il se trouvait.
 *
 * Arbitrer par la longueur règle le cas sans déplacer aucune route :
 * `/universite/gouvernance` bat `/universite`, et sur `/universite` seul,
 * Mindshiftlab reste bien le gagnant.
 */
const cibleLaPlusPrecise = computed(() => {
  let gagnante = ''
  for (const cible of CIBLES) {
    if (correspond(cible, route.path) && cible.length > gagnante.length) gagnante = cible
  }
  return gagnante
})

/**
 * Une entrée d'univers s'allume aussi quand la cible retenue est celle d'une de
 * ses applications : `/afrolang` appartient à Africarise, même si l'univers
 * pointe ailleurs. Sans cela, un membre dans Afrolang ne verrait rien de
 * surligné.
 */
function estActive(entree: EntreeNav): boolean {
  const gagnante = cibleLaPlusPrecise.value
  if (!gagnante) return false
  if (entree.vers === gagnante) return true
  return entree.sousEntrees?.some(s => s.to === gagnante) ?? false
}

/**
 * Trois raisons d'afficher les applications d'un univers, cumulables :
 *  - `epingles` : ouvert par le chevron, ou d'office parce qu'on se trouve dans
 *    l'une de ses applications (arriver sur `/afrolang` sans voir où l'on est
 *    obligerait à reconstituer l'arborescence de tête) ;
 *  - `survole` : la souris est sur le bloc ;
 *  - `focalise` : le focus clavier est dans le bloc.
 */
const epingles = ref<Set<string>>(new Set(
  NAV_AFRICANS.filter(e => e.sousEntrees?.some(s => s.to === cibleLaPlusPrecise.value))
    .map(e => e.libelle)))
const survole = ref<string | null>(null)
const focalise = ref<string | null>(null)

/**
 * Univers refermé au chevron PENDANT qu'il était survolé : sans cette
 * exception, le survol le rouvrirait aussitôt et le clic semblerait sans
 * effet. L'exception tombe dès que la souris quitte le bloc.
 */
const survolIgnore = ref<string | null>(null)

const estDeplie = (entree: EntreeNav) =>
  epingles.value.has(entree.libelle)
  || (survole.value === entree.libelle && survolIgnore.value !== entree.libelle)
  || focalise.value === entree.libelle

/**
 * Intention de survol : l'ouverture attend un instant, sinon descendre la
 * colonne d'un geste déroulerait chaque univers traversé et ferait sauter
 * toutes les entrées suivantes sous le curseur. La fermeture attend aussi,
 * pour qu'un écart de quelques pixels hors du bloc ne le replie pas.
 */
const DELAI_OUVERTURE = 150
const DELAI_FERMETURE = 200
let minuterie: ReturnType<typeof setTimeout> | undefined

function entrer(entree: EntreeNav, evenement: PointerEvent) {
  // Au toucher, `pointerenter` précède le clic : ouvrir ici doublerait le
  // chevron et le refermerait aussitôt.
  if (!entree.sousEntrees || evenement.pointerType !== 'mouse') return
  clearTimeout(minuterie)
  // Passer directement d'un univers ouvert à un autre ne doit pas attendre.
  const delai = survole.value ? 0 : DELAI_OUVERTURE
  minuterie = setTimeout(() => { survole.value = entree.libelle }, delai)
}

function sortir(entree: EntreeNav) {
  if (!entree.sousEntrees) return
  clearTimeout(minuterie)
  if (survolIgnore.value === entree.libelle) survolIgnore.value = null
  minuterie = setTimeout(() => {
    if (survole.value === entree.libelle) survole.value = null
  }, DELAI_FERMETURE)
}

function focaliser(entree: EntreeNav) {
  if (entree.sousEntrees) focalise.value = entree.libelle
}

function defocaliser(entree: EntreeNav, evenement: FocusEvent) {
  const bloc = evenement.currentTarget as HTMLElement
  // Le focus passe du lien à l'une de ses applications : on reste dans le bloc.
  if (bloc.contains(evenement.relatedTarget as Node | null)) return
  if (focalise.value === entree.libelle) focalise.value = null
}

onBeforeUnmount(() => clearTimeout(minuterie))

function basculer(entree: EntreeNav) {
  const suivant = new Set(epingles.value)
  if (estDeplie(entree)) {
    suivant.delete(entree.libelle)
    if (survole.value === entree.libelle) survolIgnore.value = entree.libelle
    if (focalise.value === entree.libelle) focalise.value = null
  }
  else {
    suivant.add(entree.libelle)
  }
  epingles.value = suivant
}

// Naviguer vers une application déplie son univers, y compris quand la
// navigation vient d'ailleurs que de cette barre.
watch(cibleLaPlusPrecise, (gagnante) => {
  const univers = NAV_AFRICANS.find(e => e.sousEntrees?.some(s => s.to === gagnante))
  if (univers && !epingles.value.has(univers.libelle))
    epingles.value = new Set(epingles.value).add(univers.libelle)
})
</script>
