<script setup lang="ts">
import type { MembreEquipeAPI } from '~/composables/useMediaEquipe'

/**
 * Rendu public d'une équipe éditoriale, feature 010.
 *
 * **Seuls les champs renseignés sont affichés** (FR-007) : une personne sans
 * territoire ni contact ne laisse aucun libellé creux derrière elle, et une
 * équipe vide ne produit aucun cadre.
 *
 * Le nom devient un lien vers le profil public **si et seulement si**
 * `utilisateur_id` est présent (FR-014). L'API ne le renseigne que lorsque le
 * compte existe et n'est pas supprimé : le gabarit n'a donc jamais à valider un
 * lien, et un compte fermé donne du texte simple plutôt qu'un lien mort.
 *
 * Le repli au-delà d'un seuil (FR-024) est **interne à ce composant** : il plie
 * des fiches, non du texte, `TexteRepliable` n'a rien à y faire.
 *
 * Tailwind v4 pur (Principe VI).
 */

const props = withDefaults(defineProps<{
  membres?: MembreEquipeAPI[]
  /** Titre du bloc. Vide = aucun en-tête (la page en pose un elle-même). */
  titre?: string
  /** Nombre de fiches visibles avant repli. `0` = tout afficher, sans commande. */
  seuil?: number
  /** Fond sombre : les vitrines médias sont en noir. */
  sombre?: boolean
  /** Disposition compacte, pour les blocs imbriqués (équipe d'un programme). */
  compact?: boolean
}>(), {
  membres: () => [],
  titre: 'Équipe éditoriale',
  seuil: 0,
  sombre: false,
  compact: false,
})

const deplie = ref(false)

const membresOrdonnes = computed(() =>
  [...props.membres].sort((a, b) => a.ordre - b.ordre),
)

const doitReplier = computed(
  () => props.seuil > 0 && membresOrdonnes.value.length > props.seuil,
)

const membresAffiches = computed(() =>
  doitReplier.value && !deplie.value
    ? membresOrdonnes.value.slice(0, props.seuil)
    : membresOrdonnes.value,
)

const restants = computed(() => membresOrdonnes.value.length - props.seuil)

const nomComplet = (membre: MembreEquipeAPI): string =>
  [membre.prenom, membre.nom].filter(Boolean).join(' ').trim()

/**
 * Le contact est une coordonnée SAISIE, jamais l'adresse du compte rattaché.
 * On la rend cliquable quand elle en a la forme, sans jamais la deviner.
 */
const lienContact = (contact: string): string | null => {
  const valeur = contact.trim()
  if (/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(valeur)) return `mailto:${valeur}`
  if (/^\+?[\d\s().-]{6,}$/.test(valeur)) return `tel:${valeur.replace(/[\s().-]/g, '')}`
  if (/^https?:\/\//i.test(valeur)) return valeur
  return null
}
</script>

<template>
  <section v-if="membresOrdonnes.length" :class="compact ? '' : 'mt-6'">
    <h3
      v-if="titre"
      class="mb-3 font-oswald uppercase tracking-wide"
      :class="[
        compact ? 'text-xs' : 'text-sm',
        sombre ? 'text-af-corps' : 'text-af-atone',
      ]"
    >
      {{ titre }}
    </h3>

    <ul
      class="grid gap-3"
      :class="compact ? 'sm:grid-cols-2 lg:grid-cols-3' : 'sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4'"
    >
      <li
        v-for="membre in membresAffiches"
        :key="membre.id"
        class="rounded-lg border px-3 py-2.5 transition-colors"
        :class="sombre
          ? 'border-af-bordure bg-af-fond hover:border-af-chocolat/50'
          : 'border-af-bordure bg-white hover:border-af-chocolat/50'"
      >
        <NuxtLink
          v-if="membre.utilisateur_id"
          :to="`/profil/${membre.utilisateur_id}`"
          class="block font-semibold underline-offset-2 transition-colors hover:underline"
          :class="sombre ? 'text-af-encre hover:text-af-chocolat' : 'text-af-encre hover:text-af-chocolat'"
        >
          {{ nomComplet(membre) }}
        </NuxtLink>
        <span
          v-else
          class="block font-semibold"
          :class="sombre ? 'text-af-encre' : 'text-af-encre'"
        >{{ nomComplet(membre) }}</span>

        <p class="mt-0.5 text-sm" :class="sombre ? 'text-af-chocolat' : 'text-af-chocolat'">
          {{ membre.fonction }}
        </p>

        <p
          v-if="membre.territoire"
          class="mt-0.5 text-xs"
          :class="sombre ? 'text-af-corps' : 'text-af-atone'"
        >
          {{ membre.territoire }}
        </p>

        <template v-if="membre.contact">
          <a
            v-if="lienContact(membre.contact)"
            :href="lienContact(membre.contact) as string"
            class="mt-1 block break-words text-xs underline underline-offset-2 transition-colors"
            :class="sombre ? 'text-af-corps hover:opacity-70' : 'text-af-atone hover:text-af-vert'"
          >{{ membre.contact }}</a>
          <p
            v-else
            class="mt-1 block break-words text-xs"
            :class="sombre ? 'text-af-corps' : 'text-af-atone'"
          >{{ membre.contact }}</p>
        </template>
      </li>
    </ul>

    <button
      v-if="doitReplier"
      type="button"
      class="mt-3 text-sm font-medium underline underline-offset-2 transition-colors cursor-pointer"
      :class="sombre ? 'text-af-chocolat hover:opacity-70' : 'text-af-chocolat hover:text-af-vert'"
      @click="deplie = !deplie"
    >
      {{ deplie ? 'voir moins' : `voir plus (${restants})` }}
    </button>
  </section>
</template>
