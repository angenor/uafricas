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
    <div v-for="entree in NAV_AFRICANS" :key="entree.libelle" class="shrink-0 lg:w-full">
      <div class="flex items-center">
        <component
          :is="entree.vers ? LienNuxt : 'span'"
          :to="entree.vers ?? undefined"
          class="flex flex-1 items-center gap-4 rounded-lg px-4 py-[11px] text-base font-bold whitespace-nowrap transition-colors"
          :class="[
            estActive(entree)
              ? 'bg-af-chocolat/15 text-af-chocolat'
              : entree.vers
                ? 'text-af-encre hover:bg-af-chocolat/[0.07]'
                : 'cursor-not-allowed text-af-atone-2']"
          :title="entree.vers ? undefined : 'Route non encore rattachée'"
        >
          <font-awesome-icon :icon="entree.icone" class="size-6 shrink-0 text-lg" />
          <span>{{ entree.libelle }}</span>
        </component>

        <!-- Le chevron est un bouton SÉPARÉ du lien : l'univers reste
             atteignable d'un clic, et dérouler ses applications ne force pas
             à le quitter. Les fondre en un seul élément obligerait à choisir
             entre naviguer et déplier. -->
        <button
          v-if="entree.sousEntrees"
          type="button"
          class="hidden size-9 shrink-0 place-items-center rounded-lg text-af-corps transition hover:bg-af-chocolat/[0.07] hover:text-af-chocolat lg:grid"
          :aria-expanded="estDeplie(entree)"
          :aria-controls="`af-sous-nav-${identifiant(entree)}`"
          :aria-label="`${estDeplie(entree) ? 'Replier' : 'Déplier'} les applications de ${entree.libelle}`"
          @click="basculer(entree)"
        >
          <font-awesome-icon
            icon="fa-solid fa-chevron-down"
            class="transition-transform"
            :class="estDeplie(entree) && 'rotate-180'"
          />
        </button>
      </div>

      <!-- v-show et non v-if : le contenu replié reste atteignable par la
           recherche du navigateur, comme pour l'accordéon des fiches pays. -->
      <ul
        v-if="entree.sousEntrees"
        :id="`af-sous-nav-${identifiant(entree)}`"
        v-show="estDeplie(entree)"
        class="hidden flex-col gap-0.5 py-1 pl-6 lg:flex"
      >
        <li v-for="sous in entree.sousEntrees" :key="sous.to">
          <NuxtLink
            :to="sous.to"
            class="flex items-center gap-3 rounded-lg px-4 py-2 text-[14px]/[1.4] transition-colors"
            :class="sous.to === cibleLaPlusPrecise
              ? 'bg-af-chocolat/[0.07] font-bold text-af-chocolat'
              : 'text-af-corps hover:bg-af-chocolat/[0.07]'"
            :title="sous.description"
          >
            <font-awesome-icon :icon="sous.icon" class="size-5 shrink-0" />
            <span class="truncate">{{ sous.label }}</span>
          </NuxtLink>
        </li>
      </ul>
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

/** Un univers est déplié d'office quand on se trouve dans l'une de ses
 *  applications : arriver sur `/afrolang` sans voir où l'on est dans
 *  l'arborescence obligerait à la reconstituer de tête. */
const deplies = ref<Set<string>>(new Set(
  NAV_AFRICANS.filter(e => e.sousEntrees?.some(s => s.to === cibleLaPlusPrecise.value))
    .map(e => e.libelle)))

const estDeplie = (entree: EntreeNav) => deplies.value.has(entree.libelle)

function basculer(entree: EntreeNav) {
  const suivant = new Set(deplies.value)
  if (suivant.has(entree.libelle)) suivant.delete(entree.libelle)
  else suivant.add(entree.libelle)
  deplies.value = suivant
}

// Naviguer vers une application déplie son univers, y compris quand la
// navigation vient d'ailleurs que de cette barre.
watch(cibleLaPlusPrecise, (gagnante) => {
  const univers = NAV_AFRICANS.find(e => e.sousEntrees?.some(s => s.to === gagnante))
  if (univers && !deplies.value.has(univers.libelle)) basculer(univers)
})
</script>
